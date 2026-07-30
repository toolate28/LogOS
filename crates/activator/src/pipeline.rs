//! Pipeline Execution Engine
//!
//! Executes composed capability chains with WAVE scoring at each transition.
//! Implements the Fibonacci-weighted progression model from the Pedagogical
//! Notebook Pipeline: each step must maintain coherence above threshold
//! before the next step is unlocked.

use crate::{ActivationError, PipelineTemplate};
use serde::{Deserialize, Serialize};

// ─── Execution State ────────────────────────────────────────────────

/// The execution state of a pipeline run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineExecution {
    pub pipeline_id: String,
    pub steps: Vec<StepExecution>,
    pub status: PipelineStatus,
    /// Fibonacci-weighted progression thresholds per step
    pub phi_weights: Vec<f64>,
    /// Hash chain — each step's output hash feeds the next step's input
    pub hash_chain: Vec<String>,
}

/// Status of a pipeline execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineStatus {
    /// Not yet started
    Pending,
    /// Currently executing step N
    Running { current_step: usize },
    /// Completed successfully — all steps passed coherence
    Completed,
    /// Failed at step N with reason
    Failed { step: usize, reason: String },
    /// Phasonic flip — agent sent back to an earlier step
    PhasonicFlip { from_step: usize, to_step: usize },
}

/// Execution state for a single pipeline step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecution {
    pub step_id: String,
    pub capability_id: String,
    pub status: StepStatus,
    /// WAVE score at this transition (None if not yet scored)
    pub wave_score: Option<f64>,
    /// Fibonacci weight for this step's threshold
    pub phi_weight: f64,
    /// Input hash (from previous step or initial seed)
    pub hash_in: Option<String>,
    /// Output hash (computed after execution)
    pub hash_out: Option<String>,
}

/// Status of a single step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Locked,
    Unlocked,
    Running,
    Passed,
    Failed,
}

// ─── Fibonacci Weights ──────────────────────────────────────────────

/// Generate Fibonacci-weighted progression thresholds for N steps.
/// Maps to the Pedagogical Notebook Pipeline's Φ progression:
/// Step 1: 0.20, Step 2: 0.35, Step 3: 0.60, Step 4: 0.85
pub fn fibonacci_weights(n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0.85]; // Single step gets full threshold
    }

    // Generate fibonacci sequence
    let mut fib = vec![1u64, 1];
    for i in 2..n {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    let total: f64 = fib.iter().sum::<u64>() as f64;

    // Normalize to [0.20, 0.85] range — earlier steps are easier
    fib.iter()
        .map(|&f| {
            let normalized = f as f64 / total;
            0.20 + normalized * 0.65 // Map [0,1] → [0.20, 0.85]
        })
        .collect()
}

// ─── Pipeline Preparation ───────────────────────────────────────────

/// Prepare a pipeline for execution from a template.
pub fn prepare_execution(
    template: &PipelineTemplate,
    initial_hash: &str,
) -> PipelineExecution {
    let phi_weights = fibonacci_weights(template.steps.len());

    let steps: Vec<StepExecution> = template
        .steps
        .iter()
        .enumerate()
        .map(|(i, step)| StepExecution {
            step_id: step.id.clone(),
            capability_id: step.capability_id.clone(),
            status: if i == 0 {
                StepStatus::Unlocked
            } else {
                StepStatus::Locked
            },
            wave_score: None,
            phi_weight: phi_weights.get(i).copied().unwrap_or(0.85),
            hash_in: if i == 0 {
                Some(initial_hash.to_string())
            } else {
                None
            },
            hash_out: None,
        })
        .collect();

    PipelineExecution {
        pipeline_id: template.id.clone(),
        steps,
        status: PipelineStatus::Pending,
        phi_weights,
        hash_chain: vec![initial_hash.to_string()],
    }
}

/// Advance a pipeline execution by recording a step result.
/// Returns Ok(true) if pipeline is complete, Ok(false) if more steps remain.
pub fn advance_step(
    execution: &mut PipelineExecution,
    step_index: usize,
    wave_score: f64,
    output_hash: &str,
) -> Result<bool, ActivationError> {
    if step_index >= execution.steps.len() {
        return Err(ActivationError::IncompatibleStep {
            step: format!("step_{}", step_index),
        });
    }

    let step = &mut execution.steps[step_index];
    let threshold = step.phi_weight;

    // Record the score
    step.wave_score = Some(wave_score);
    step.hash_out = Some(output_hash.to_string());

    if wave_score >= threshold {
        // Step passed — advance
        step.status = StepStatus::Passed;
        execution.hash_chain.push(output_hash.to_string());

        // Unlock next step if exists
        if step_index + 1 < execution.steps.len() {
            execution.steps[step_index + 1].status = StepStatus::Unlocked;
            execution.steps[step_index + 1].hash_in = Some(output_hash.to_string());
            execution.status = PipelineStatus::Running {
                current_step: step_index + 1,
            };
            Ok(false)
        } else {
            // All steps complete
            execution.status = PipelineStatus::Completed;
            Ok(true)
        }
    } else {
        // Step failed — coherence too low
        step.status = StepStatus::Failed;
        execution.status = PipelineStatus::Failed {
            step: step_index,
            reason: format!(
                "WAVE score {:.4} below Fibonacci threshold {:.4}",
                wave_score, threshold
            ),
        };
        Err(ActivationError::CoherenceDrop {
            step: format!("step_{}", step_index),
            score: wave_score,
        })
    }
}

/// Trigger a Phasonic Flip — send the agent back to an earlier step.
pub fn phasonic_flip(
    execution: &mut PipelineExecution,
    from_step: usize,
    to_step: usize,
) -> Result<(), ActivationError> {
    if to_step >= from_step || from_step >= execution.steps.len() {
        return Err(ActivationError::IncompatibleStep {
            step: format!("flip_{}→{}", from_step, to_step),
        });
    }

    // Reset steps from to_step onward
    for i in to_step..=from_step {
        execution.steps[i].status = if i == to_step {
            StepStatus::Unlocked
        } else {
            StepStatus::Locked
        };
        execution.steps[i].wave_score = None;
        execution.steps[i].hash_out = None;
    }

    // Restore hash_in for the target step from the chain
    if to_step < execution.hash_chain.len() {
        execution.steps[to_step].hash_in = Some(execution.hash_chain[to_step].clone());
    }

    // Truncate hash chain
    execution.hash_chain.truncate(to_step + 1);

    execution.status = PipelineStatus::PhasonicFlip {
        from_step,
        to_step,
    };

    Ok(())
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PipelineStep;

    fn test_template() -> PipelineTemplate {
        PipelineTemplate {
            id: "test-pipeline".to_string(),
            name: "Test Pipeline".to_string(),
            trigger_phrases: vec![],
            steps: vec![
                PipelineStep {
                    id: "step_0".to_string(),
                    capability_id: "wave_check".to_string(),
                    action: "execute".to_string(),
                    input_refs: vec![],
                    params: serde_json::Value::Null,
                },
                PipelineStep {
                    id: "step_1".to_string(),
                    capability_id: "data_viz".to_string(),
                    action: "execute".to_string(),
                    input_refs: vec!["$step_0.output".to_string()],
                    params: serde_json::Value::Null,
                },
                PipelineStep {
                    id: "step_2".to_string(),
                    capability_id: "ollama_local".to_string(),
                    action: "execute".to_string(),
                    input_refs: vec!["$step_1.output".to_string()],
                    params: serde_json::Value::Null,
                },
            ],
            coherence_threshold: 0.85,
        }
    }

    #[test]
    fn test_fibonacci_weights() {
        let w = fibonacci_weights(4);
        assert_eq!(w.len(), 4);
        // Weights should be increasing (Fibonacci progression)
        for i in 1..w.len() {
            assert!(w[i] >= w[i - 1], "Weights should be non-decreasing");
        }
        // All within [0.20, 0.85]
        for w in &w {
            assert!(*w >= 0.20 && *w <= 0.86);
        }
    }

    #[test]
    fn test_prepare_execution() {
        let template = test_template();
        let exec = prepare_execution(&template, "0xABCD");
        assert_eq!(exec.steps.len(), 3);
        assert_eq!(exec.steps[0].status, StepStatus::Unlocked);
        assert_eq!(exec.steps[1].status, StepStatus::Locked);
        assert_eq!(exec.steps[2].status, StepStatus::Locked);
        assert_eq!(exec.hash_chain.len(), 1);
    }

    #[test]
    fn test_advance_step_success() {
        let template = test_template();
        let mut exec = prepare_execution(&template, "0xABCD");

        // Pass step 0 with high score
        let complete = advance_step(&mut exec, 0, 0.95, "0xBEEF").unwrap();
        assert!(!complete);
        assert_eq!(exec.steps[0].status, StepStatus::Passed);
        assert_eq!(exec.steps[1].status, StepStatus::Unlocked);
    }

    #[test]
    fn test_full_pipeline_completion() {
        let template = test_template();
        let mut exec = prepare_execution(&template, "0xABCD");

        let _ = advance_step(&mut exec, 0, 0.95, "0x1111").unwrap();
        let _ = advance_step(&mut exec, 1, 0.95, "0x2222").unwrap();
        let complete = advance_step(&mut exec, 2, 0.95, "0x3333").unwrap();

        assert!(complete);
        assert_eq!(exec.status, PipelineStatus::Completed);
        assert_eq!(exec.hash_chain.len(), 4); // initial + 3 steps
    }

    #[test]
    fn test_phasonic_flip() {
        let template = test_template();
        let mut exec = prepare_execution(&template, "0xABCD");

        // Pass first two steps
        let _ = advance_step(&mut exec, 0, 0.95, "0x1111").unwrap();
        let _ = advance_step(&mut exec, 1, 0.95, "0x2222").unwrap();

        // Flip from step 2 back to step 1
        phasonic_flip(&mut exec, 2, 1).unwrap();
        assert_eq!(exec.steps[1].status, StepStatus::Unlocked);
        assert_eq!(exec.steps[2].status, StepStatus::Locked);
        assert!(exec.steps[1].wave_score.is_none());
    }
}
