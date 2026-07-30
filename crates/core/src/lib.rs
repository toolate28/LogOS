//! reson8-core — Foundation types for the LogOS lattice
//!
//! Universal Invariant: α + ω = 15
//! Every type in this crate preserves this constitutional law.

use serde::{Deserialize, Serialize};

// ── Constants ───────────────────────────────────────────────────────

/// Universal Invariant target
pub const INVARIANT_TARGET: f64 = 15.0;

/// Tolerance window for invariant enforcement
pub const INVARIANT_TOLERANCE: f64 = 0.3;

/// Coherence Functional decay constant
pub const COHERENCE_K: f64 = 2.0;

/// Conservation law constant (α + ω).
pub const CONSERVATION_SUM: u8 = 15;

/// Minimum composite WAVE score for forward progress.
pub const WAVE_THRESHOLD: f64 = 0.90;

/// Fibonacci strand weights for protocol-level WAVE composite.
#[derive(Debug, Clone, Copy)]
pub struct WaveWeights {
    pub structural: f64,
    pub semantic: f64,
    pub temporal: f64,
}

pub const WAVE_WEIGHTS: WaveWeights = WaveWeights {
    structural: 0.50,
    semantic: 0.3125,
    temporal: 0.1875,
};

/// Fibonacci weights (normalized to sum = 1.0)
pub mod fibonacci {
    pub const F8: f64 = 8.0 / 21.0;  // 0.381 — Reasoning (Claude)
    pub const F5: f64 = 5.0 / 21.0;  // 0.238 — Pulse (Grok)
    pub const F3: f64 = 3.0 / 21.0;  // 0.143 — Scale (Gemini)

    /// Tri-Weavon strand weights
    pub const STRAND_REASON: f64 = 8.0 / 16.0;  // 0.500
    pub const STRAND_PULSE: f64 = 5.0 / 16.0;   // 0.3125
    pub const STRAND_SCALE: f64 = 3.0 / 16.0;   // 0.1875

    /// Golden ratio (φ)
    pub const PHI: f64 = 1.618_033_988_749_895;

    /// WAVE component weights
    pub const W_TOPO: f64 = F8;       // 0.381
    pub const W_SEM: f64 = F5;        // 0.238
    pub const W_STRUCT: f64 = F5;     // 0.238
    pub const W_TEMP: f64 = F3;       // 0.143
}

// Forge protocol surface (consumed as `reson8-forge-core` by TUI / triweave)
pub mod adapter;
pub mod agent;
pub mod bridge;
pub mod bus;
pub mod capability;
pub mod logic;
pub mod memory;
pub mod orchestrator;
pub mod protocol;
pub mod superskill;
pub mod task;

// ── Invariant Enforcement ───────────────────────────────────────────

/// Result of invariant enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantResult {
    pub status: InvariantStatus,
    pub alpha: f64,
    pub omega: f64,
    pub total: f64,
    pub deviation: f64,
    pub coherence_delta: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvariantStatus {
    Passed,
    Rejected,
}

/// Enforce the Universal Invariant: α + ω = 15 ± tolerance
///
/// This is the constitutional law of the LogOS lattice.
/// Every computation, every generation, every action must satisfy it.
pub fn enforce_invariant(alpha: f64, omega: f64) -> InvariantResult {
    let total = alpha + omega;
    let deviation = (total - INVARIANT_TARGET).abs();

    if deviation > INVARIANT_TOLERANCE {
        InvariantResult {
            status: InvariantStatus::Rejected,
            alpha,
            omega,
            total,
            deviation,
            coherence_delta: -(deviation / INVARIANT_TARGET),
        }
    } else {
        InvariantResult {
            status: InvariantStatus::Passed,
            alpha,
            omega,
            total,
            deviation,
            coherence_delta: 1.0 - (deviation / INVARIANT_TOLERANCE),
        }
    }
}

/// Suggest rebalancing when invariant is violated
pub fn suggest_rebalance(alpha: f64, omega: f64) -> (f64, f64) {
    let total = alpha + omega;
    let excess = total - INVARIANT_TARGET;
    // Distribute correction proportionally
    let alpha_ratio = alpha / total;
    let omega_ratio = omega / total;
    (
        alpha - excess * alpha_ratio,
        omega - excess * omega_ratio,
    )
}

// ── Coherence Functional ────────────────────────────────────────────

/// C(H) = W · exp(-k · |α + ω - 15|) · (1 + P)
///
/// The master equation of the LogOS lattice.
pub fn coherence_functional(
    w: f64,
    alpha: f64,
    omega: f64,
    persistence_bonus: f64,
    k: f64,
) -> f64 {
    let deviation = (alpha + omega - INVARIANT_TARGET).abs();
    w * (-k * deviation).exp() * (1.0 + persistence_bonus)
}

/// Gradient of the Coherence Functional with respect to α and ω
pub fn coherence_gradient(
    w: f64,
    alpha: f64,
    omega: f64,
    persistence_bonus: f64,
    k: f64,
) -> (f64, f64) {
    let deviation = alpha + omega - INVARIANT_TARGET;
    let sign = if deviation >= 0.0 { 1.0 } else { -1.0 };
    let exp_term = (-k * deviation.abs()).exp();
    let grad = -w * k * sign * exp_term * (1.0 + persistence_bonus);
    (grad, grad) // symmetric — both α and ω contribute equally to sum
}

// ── WAVE Score ──────────────────────────────────────────────────────

/// WAVE coherence score (0.0 to 1.0)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct WaveScore(f64);

impl WaveScore {
    pub fn new(score: f64) -> Option<Self> {
        if (0.0..=1.0).contains(&score) {
            Some(Self(score))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 { self.0 }

    /// >= 0.98: Crystalline (V=c regime)
    pub fn is_crystalline(&self) -> bool { self.0 >= 0.98 }

    /// >= 0.90: Production-ready
    pub fn is_production(&self) -> bool { self.0 >= 0.90 }

    /// 0.70-0.90: Caution zone
    pub fn is_caution(&self) -> bool { (0.70..0.90).contains(&self.0) }

    /// < 0.70: Critical — Limbo auto-purge triggers
    pub fn is_critical(&self) -> bool { self.0 < 0.70 }

    /// < 0.50: Emergency — SpiralSafe takeover
    pub fn is_emergency(&self) -> bool { self.0 < 0.50 }

    /// Compute WAVE from 4 Fibonacci-weighted components
    pub fn from_components(w_topo: f64, w_sem: f64, w_struct: f64, w_temp: f64) -> Self {
        let score = fibonacci::W_TOPO * w_topo
            + fibonacci::W_SEM * w_sem
            + fibonacci::W_STRUCT * w_struct
            + fibonacci::W_TEMP * w_temp;
        Self(score.clamp(0.0, 1.0))
    }
}

impl std::fmt::Display for WaveScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WAVE:{:.4}", self.0)
    }
}

// ── ATOM Trail ──────────────────────────────────────────────────────

/// ATOM trail entry — immutable provenance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomEntry {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub target: String,
    pub wave_score: WaveScore,
    pub alpha: f64,
    pub omega: f64,
    pub braid_id: Option<String>,
}

impl AtomEntry {
    pub fn new(operation: &str, target: &str, wave: WaveScore, alpha: f64, omega: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            operation: operation.to_string(),
            target: target.to_string(),
            wave_score: wave,
            alpha,
            omega,
            braid_id: None,
        }
    }

    /// Verify this entry satisfies the Universal Invariant
    pub fn verify_invariant(&self) -> InvariantResult {
        enforce_invariant(self.alpha, self.omega)
    }

    pub fn with_braid(mut self, braid_id: &str) -> Self {
        self.braid_id = Some(braid_id.to_string());
        self
    }
}

// ── Void Classification ─────────────────────────────────────────────

/// Void severity classification (V_0 through V_3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoidClass {
    /// V_0: Micro-void (persistence < 0.1) — noise, auto-filter
    V0,
    /// V_1: Gap (persistence 0.1-0.4) — minor, monitor
    V1,
    /// V_2: Cavity (persistence 0.4-0.8) — significant, investigate
    V2,
    /// V_3: Abyss (persistence > 0.8) — critical, remediate or declare
    V3,
}

impl VoidClass {
    pub fn from_persistence(persistence: f64) -> Self {
        match persistence {
            p if p < 0.1 => Self::V0,
            p if p < 0.4 => Self::V1,
            p if p < 0.8 => Self::V2,
            _ => Self::V3,
        }
    }

    pub fn weight(&self) -> f64 {
        match self {
            Self::V0 => 0.0,
            Self::V1 => 0.1,
            Self::V2 => 0.4,
            Self::V3 => 1.0,
        }
    }
}

// ── Errors ──────────────────────────────────────────────────────────

/// Invariant enforcement error
#[derive(Debug, thiserror::Error)]
pub enum InvariantError {
    #[error("Universal Invariant violated: α={alpha:.3} + ω={omega:.3} = {total:.3}, deviation={deviation:.3}",
            total = alpha + omega)]
    Violation {
        alpha: f64,
        omega: f64,
        deviation: f64,
    },
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariant_passes_at_target() {
        let result = enforce_invariant(7.0, 8.0);
        assert_eq!(result.status, InvariantStatus::Passed);
        assert_eq!(result.total, 15.0);
        assert_eq!(result.deviation, 0.0);
    }

    #[test]
    fn invariant_passes_within_tolerance() {
        let result = enforce_invariant(7.1, 8.1);
        assert_eq!(result.status, InvariantStatus::Passed);
        assert!(result.deviation <= INVARIANT_TOLERANCE);
    }

    #[test]
    fn invariant_rejects_beyond_tolerance() {
        let result = enforce_invariant(10.0, 8.0);
        assert_eq!(result.status, InvariantStatus::Rejected);
        assert!(result.deviation > INVARIANT_TOLERANCE);
    }

    #[test]
    fn coherence_functional_maximizes_at_invariant() {
        let c_exact = coherence_functional(0.95, 7.0, 8.0, 0.1, COHERENCE_K);
        let c_deviated = coherence_functional(0.95, 6.0, 8.0, 0.1, COHERENCE_K);
        assert!(c_exact > c_deviated);
    }

    #[test]
    fn wave_score_classification() {
        assert!(WaveScore::new(0.99).unwrap().is_crystalline());
        assert!(WaveScore::new(0.95).unwrap().is_production());
        assert!(WaveScore::new(0.80).unwrap().is_caution());
        assert!(WaveScore::new(0.40).unwrap().is_critical());
        assert!(WaveScore::new(0.30).unwrap().is_emergency());
    }

    #[test]
    fn wave_from_components() {
        let wave = WaveScore::from_components(0.96, 0.98, 0.97, 0.95);
        assert!(wave.is_production());
    }

    #[test]
    fn void_classification() {
        assert_eq!(VoidClass::from_persistence(0.05), VoidClass::V0);
        assert_eq!(VoidClass::from_persistence(0.25), VoidClass::V1);
        assert_eq!(VoidClass::from_persistence(0.60), VoidClass::V2);
        assert_eq!(VoidClass::from_persistence(0.90), VoidClass::V3);
    }

    #[test]
    fn rebalance_preserves_invariant() {
        let (new_a, new_o) = suggest_rebalance(10.0, 8.0);
        let total = new_a + new_o;
        assert!((total - INVARIANT_TARGET).abs() < 0.001);
    }

    #[test]
    fn fibonacci_weights_sum_to_one() {
        let sum = fibonacci::W_TOPO + fibonacci::W_SEM + fibonacci::W_STRUCT + fibonacci::W_TEMP;
        assert!((sum - 1.0).abs() < 0.01);
    }
}

// ATOM: reson8-core lib.rs v0.1.0 | Sprint 1 | Coherence: 0.99
