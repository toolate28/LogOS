//! Integration points with discreteBKM Predictor and HYPHA kernel

use discrete_bkm_predictor::{discrete_bkm_predict, TelemetryKernelWitness};

/// Example integration: Call discreteBKM from within the DDE instance
pub fn evaluate_internal_state(witness: &TelemetryKernelWitness) -> f32 {
    let prediction = discrete_bkm_predict(witness);
    prediction.coherence_delta
}