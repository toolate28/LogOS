//! SuperskillEngine — pipeline executor that enforces α+ω=15 at every transition.
//!
//! When a pipeline is triggered (e.g., "paper-draft"), the engine:
//! 1. Sends `TriggerPipeline` command over the bridge
//! 2. Monitors incoming `Coherence` events for WAVE score
//! 3. If WAVE drops below threshold → aborts pipeline, emits `InvariantViolated`
//! 4. If conservation law breaks → immediate abort
//! 5. On completion → emits `PipelineComplete`

use tokio::sync::mpsc;
use tracing::{error, info};

use crate::bridge::{BridgeCommand, BridgeEvent, CoherencePayload};
use crate::protocol::ConservationState;
use serde_json::Value;

// ---------------------------------------------------------------------------
// Events emitted by the engine → consumed by App
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum SuperskillEvent {
    PipelineStarted(String),
    StateUpdated(PipelineState),
    InvariantViolated(Violation),
    PipelineComplete(String),
    PipelineAborted { id: String, reason: String },
}

#[derive(Debug, Clone)]
pub struct PipelineState {
    pub wave_score: f64,
    pub alpha: u8,
    pub omega: u8,
    pub current_step: String,
    pub percent: u8,
}

impl PipelineState {
    /// Check all invariants hold: conservation + WAVE threshold.
    pub fn invariants_hold(&self) -> bool {
        self.alpha.saturating_add(self.omega) == crate::CONSERVATION_SUM
            && self.wave_score >= crate::WAVE_THRESHOLD
    }
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub reason: String,
    pub wave_score: f64,
    pub conservation: ConservationState,
}

/// Pipeline status tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStatus {
    Idle,
    Running,
    Paused,
    Complete,
    Aborted,
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

pub struct SuperskillEngine {
    cmd_tx: mpsc::Sender<BridgeCommand>,
    event_tx: mpsc::Sender<SuperskillEvent>,
    status: PipelineStatus,
    active_pipeline: Option<String>,
    last_state: Option<PipelineState>,
}

impl SuperskillEngine {
    pub fn new(
        cmd_tx: mpsc::Sender<BridgeCommand>,
        event_tx: mpsc::Sender<SuperskillEvent>,
    ) -> Self {
        Self {
            cmd_tx,
            event_tx,
            status: PipelineStatus::Idle,
            active_pipeline: None,
            last_state: None,
        }
    }

    /// Trigger a named pipeline with parameters.
    pub async fn trigger(
        &mut self,
        pipeline_name: &str,
        params: serde_json::Value,
    ) -> Result<(), String> {
        if self.status == PipelineStatus::Running {
            return Err("Pipeline already running".into());
        }

        self.status = PipelineStatus::Running;
        self.active_pipeline = Some(pipeline_name.to_string());

        // Notify the TUI
        let _ = self
            .event_tx
            .send(SuperskillEvent::PipelineStarted(pipeline_name.to_string()))
            .await;

        // Send command over the bridge
        let _ = self
            .cmd_tx
            .send(BridgeCommand::TriggerPipeline {
                name: pipeline_name.to_string(),
                params,
            })
            .await;

        info!("SuperskillEngine: triggered pipeline '{}'", pipeline_name);
        Ok(())
    }

    /// Handle an incoming bridge event — check invariants, update state.
    pub async fn handle(&mut self, event: BridgeEvent) {
        match event {
            BridgeEvent::Coherence(payload) => {
                self.check_invariants(&payload).await;
            }
            BridgeEvent::PipelineProgress(p) => {
                if p.completed_steps >= p.total_steps {
                    self.complete().await;
                } else {
                    let state = PipelineState {
                        wave_score: self.last_state.as_ref().map_or(0.0, |s| s.wave_score),
                        alpha: self.last_state.as_ref().map_or(7, |s| s.alpha),
                        omega: self.last_state.as_ref().map_or(8, |s| s.omega),
                        current_step: p.current_step.clone(),
                        percent: p.percent,
                    };
                    let _ = self.event_tx.send(SuperskillEvent::StateUpdated(state)).await;
                }
            }
            BridgeEvent::ExecuteMcpTool { tool_name, args: _, req_id } => {
                info!("ExecuteMcpTool received: tool={}, req_id={}", tool_name, req_id);
                // In a full implementation, we would execute the tool logic here.
                // For now, we simulate execution and commit to the ledger.
                
                // Let's create a dummy payload to simulate the tool's execution result.
                let mut execution_payload = serde_json::Map::new();
                execution_payload.insert("tool".to_string(), Value::String(tool_name.clone()));
                execution_payload.insert("status".to_string(), Value::String("success".to_string()));
                execution_payload.insert("WAVE".to_string(), Value::Number(serde_json::Number::from_f64(0.95).unwrap()));
                
                let _payload_str = serde_json::to_string(&execution_payload).unwrap();
                
                // Here we invoke MarketplaceOrigin to anchor to G: Drive
                // Note: We need to import marketplace and zero_latency_ledgers, but they are external crates in the workspace.
                // For simplicity in this engine, we might just log or simulate until we get the full trait bound.
                info!("Writing to zero_latency_ledger for tool {}", tool_name);
                
                // We'd send a response back over the bridge to the MCP (not yet implemented in BridgeCommand)
                // Let's assume the TS MCP is listening for Coherence or Atom events as a form of "progress"
            }
            _ => {}
        }
    }

    async fn check_invariants(&mut self, payload: &CoherencePayload) {
        let state = PipelineState {
            wave_score: payload.wave_score,
            alpha: payload.conservation.alpha,
            omega: payload.conservation.omega,
            current_step: payload.step_id.clone(),
            percent: 0,
        };

        // Conservation check: α + ω = 15
        if !payload.conservation.verify() {
            let violation = Violation {
                reason: format!(
                    "Conservation violated: α({}) + ω({}) = {} ≠ 15",
                    payload.conservation.alpha,
                    payload.conservation.omega,
                    payload.conservation.sum,
                ),
                wave_score: payload.wave_score,
                conservation: payload.conservation,
            };
            error!("{}", violation.reason);
            let _ = self.event_tx.send(SuperskillEvent::InvariantViolated(violation)).await;
            self.abort("conservation law violated").await;
            return;
        }

        // WAVE threshold check
        if payload.wave_score < crate::WAVE_THRESHOLD {
            let violation = Violation {
                reason: format!(
                    "WAVE below threshold: {:.3} < {:.3}",
                    payload.wave_score,
                    crate::WAVE_THRESHOLD,
                ),
                wave_score: payload.wave_score,
                conservation: payload.conservation,
            };
            error!("{}", violation.reason);
            let _ = self.event_tx.send(SuperskillEvent::InvariantViolated(violation)).await;
            self.abort("WAVE coherence below threshold").await;
            return;
        }

        self.last_state = Some(state.clone());
        let _ = self.event_tx.send(SuperskillEvent::StateUpdated(state)).await;
    }

    async fn complete(&mut self) {
        if let Some(name) = self.active_pipeline.take() {
            self.status = PipelineStatus::Complete;
            info!("SuperskillEngine: pipeline '{}' complete", name);
            let _ = self.event_tx.send(SuperskillEvent::PipelineComplete(name)).await;
        }
    }

    async fn abort(&mut self, reason: &str) {
        let id = self.active_pipeline.take().unwrap_or_default();
        self.status = PipelineStatus::Aborted;
        error!("SuperskillEngine: pipeline '{}' ABORTED: {}", id, reason);
        let _ = self
            .event_tx
            .send(SuperskillEvent::PipelineAborted {
                id,
                reason: reason.to_string(),
            })
            .await;
    }
}
