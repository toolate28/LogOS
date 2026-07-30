//! Agda Polarity logic mirror for KernelWitness alignment checks.

use super::kernel_witness::{KernelWitness, LiftOk, OpalMode};
use crate::core::srac_strategies::DivergenceReason;

pub const PREDICTION_ERROR_SCALE: i32 = 1000;
pub const THRESHOLD_SCALED: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityResult {
    Aligned,
    Flipped,
}

pub trait Polarity {
    fn positive(&self) -> bool;
    fn negative(&self) -> bool;

    fn aligned(&self) -> bool {
        self.positive() && !self.negative()
    }
}

impl Polarity for KernelWitness {
    fn positive(&self) -> bool {
        self.prediction_error_scaled() <= THRESHOLD_SCALED
            && self.lift_ok.betti_proxy_below_threshold
            && self.lift_ok.tomczak_preserved
    }

    fn negative(&self) -> bool {
        self.out_surge
            || self.prediction_error_scaled() > THRESHOLD_SCALED
            || matches!(self.active_mode, OpalMode::Unstabilized)
    }
}

pub fn check_polarity_and_proceed(
    witness: &KernelWitness,
) -> Result<KernelWitness, DivergenceReason> {
    if witness.polarity() == PolarityResult::Aligned {
        Ok(witness.clone())
    } else {
        let reason = if witness.out_surge {
            DivergenceReason::SurgeDetected
        } else if witness.prediction_error() > PREDICTION_ERROR_THRESHOLD as f32 {
            DivergenceReason::HighPredictionError {
                actual: witness.prediction_error(),
                threshold: PREDICTION_ERROR_THRESHOLD as f32,
            }
        } else {
            DivergenceReason::LiftOkFailed {
                betti_above_threshold: !witness.lift_ok.betti_proxy_below_threshold,
                tomczak_preserved: witness.lift_ok.tomczak_preserved,
            }
        };
        Err(reason)
    }
}

impl KernelWitness {
    pub fn polarity(&self) -> PolarityResult {
        if self.positive() && !self.negative() {
            PolarityResult::Aligned
        } else {
            PolarityResult::Flipped
        }
    }

    pub fn prediction_error_scaled(&self) -> i32 {
        (self.prediction_error() * PREDICTION_ERROR_SCALE as f32) as i32
    }
}

use super::kernel_witness::PREDICTION_ERROR_THRESHOLD;