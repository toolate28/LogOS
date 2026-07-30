//! Live telemetry wiring for CutileHarness (v0.4.1 extension)
//! Logs leech_density alongside prediction_error for SRAC analysis.
//! Mono, idempotent, append-only. Ready for coherence-mcp ingestion.

use crate::harness::{KernelWitness, PREDICTION_ERROR_THRESHOLD};
use std::collections::VecDeque;

/// Extended witness with Leech density prior telemetry.
#[derive(Debug, Clone)]
pub struct TelemetryKernelWitness {
    pub base: KernelWitness,
    pub leech_density: Option<f32>,      // From LeechDensityConfig (0.0–1.0 normalized)
    pub density_weight: Option<f32>,     // Config weight used in tie-breaker
    pub burst_rate_delta: Option<f32>,   // Measured reduction in SRAC bursts (simulation)
}

impl TelemetryKernelWitness {
    pub fn new(base: KernelWitness, leech_density: Option<f32>, density_weight: Option<f32>) -> Self {
        Self {
            base,
            leech_density,
            density_weight,
            burst_rate_delta: None, // populated post-run in live harness
        }
    }

    /// Core logging point: call after every ffi_roundtrip_sm100 or diagnostic tick.
    pub fn log_telemetry(&self) {
        let pred_err = self.base.prediction_error();
        let density = self.leech_density.unwrap_or(-1.0);
        let weight = self.density_weight.unwrap_or(0.0);

        println!(
            "[cutile-telemetry] prediction_error={:.4} (threshold {:.2}) | leech_density={:.4} | weight={:.2} | lift_ok={} | surge={}",
            pred_err,
            PREDICTION_ERROR_THRESHOLD,
            density,
            weight,
            self.base.lift_ok.is_ok(),
            self.base.out_surge
        );

        // In live deployment: emit to coherence-mcp OTLP / ATOM trail
        // e.g. coherence_mcp::emit_metric("triweavon.leech_density", density);
        // e.g. coherence_mcp::emit_metric("triweavon.prediction_error", pred_err);
    }

    /// Update burst rate delta after SRAC correction burst (idempotent accumulation).
    pub fn record_burst_delta(&mut self, previous_burst_rate: f32, current_burst_rate: f32) {
        self.burst_rate_delta = Some(previous_burst_rate - current_burst_rate);
    }
}

/// Example integration hook inside CutileHarness::ffi_roundtrip_sm100 (or tick).
/// Call this after witness extraction when Leech prior is active.
pub fn wire_leech_telemetry(
    witness: KernelWitness,
    leech_density: Option<f32>,
    density_weight: Option<f32>,
) -> TelemetryKernelWitness {
    let tw = TelemetryKernelWitness::new(witness, leech_density, density_weight);
    tw.log_telemetry();
    tw
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::LiftOk;

    #[test]
    fn telemetry_logs_without_panic() {
        let base = KernelWitness {
            out_stretch: 1.75,
            out_betti_proxy: 12,
            out_surge: false,
            lift_ok: LiftOk::from_kernel(12, false),
            active_mode: crate::skills::discrete_bkm_check::OpalMode::PhaseStabilized { tau: 0.05, stretch: 1.75 },
        };
        let tw = wire_leech_telemetry(base, Some(0.87), Some(0.65));
        assert!(tw.leech_density.is_some());
    }
}
