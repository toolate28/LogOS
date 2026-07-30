//! KernelWitness + sm_100 harness scaffold for OPAL/discreteBKM integration.

use crate::existence_cert::{ExistenceCertificate, TomczakGateWitness};
use crate::VERSION;

impl From<&LiftOk> for TomczakGateWitness {
    fn from(lift: &LiftOk) -> Self {
        Self {
            betti_proxy_below_threshold: lift.betti_proxy_below_threshold,
            tomczak_preserved: lift.tomczak_preserved,
        }
    }
}

pub const STRETCH_FACTOR: f32 = 1.8;
pub const RETENTION_FACTOR: f32 = 1.18;
pub const SURGE_THRESHOLD: f32 = 0.15;
pub const PREDICTION_ERROR_THRESHOLD: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpalMode {
    PhaseStabilized { tau: f32, stretch: f32 },
    Unstabilized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiftOk {
    pub betti_proxy_below_threshold: bool,
    pub tomczak_preserved: bool,
}

impl LiftOk {
    pub fn from_kernel(betti_proxy: u64, surge: bool) -> Self {
        Self {
            betti_proxy_below_threshold: betti_proxy < 128,
            tomczak_preserved: !surge,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.betti_proxy_below_threshold && self.tomczak_preserved
    }
}

#[derive(Debug, Clone)]
pub struct KernelWitness {
    pub out_stretch: f32,
    pub out_betti_proxy: u64,
    pub out_surge: bool,
    pub lift_ok: LiftOk,
    pub active_mode: OpalMode,
    prediction_error: f32,
}

impl KernelWitness {
    pub fn prediction_error(&self) -> f32 {
        self.prediction_error
    }

    pub fn with_prediction_error(mut self, err: f32) -> Self {
        self.prediction_error = err;
        self
    }

    /// Coherence / entropy path certificate (Tomczak gate already evaluated in witness).
    pub fn emit_existence_certificate(
        &self,
        wave: f64,
        coherence_delta: f64,
        atom_trail_id: impl Into<String>,
    ) -> ExistenceCertificate {
        ExistenceCertificate::from_lift_and_mehler(
            &TomczakGateWitness::from(&self.lift_ok),
            f64::from(self.prediction_error),
            self.lift_ok.is_ok(),
            wave,
            15.0,
            coherence_delta,
            atom_trail_id,
            None,
            None,
            None,
        )
    }
}

pub struct CutileHarness;

impl CutileHarness {
    pub fn new_sm100() -> Self {
        Self
    }

    pub fn launch_entropy_reduction(&self, total_dof: u32) -> KernelWitness {
        let stretch = STRETCH_FACTOR * RETENTION_FACTOR;
        let surge = stretch > SURGE_THRESHOLD * 10.0;
        KernelWitness {
            out_stretch: stretch,
            out_betti_proxy: (total_dof / 1024).max(1) as u64,
            out_surge: surge,
            lift_ok: LiftOk::from_kernel((total_dof / 1024).max(1) as u64, surge),
            active_mode: OpalMode::PhaseStabilized {
                tau: 0.05,
                stretch,
            },
            prediction_error: 0.04,
        }
    }
}