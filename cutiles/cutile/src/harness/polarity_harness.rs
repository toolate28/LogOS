#![cfg(kani)]

use super::kernel_witness::KernelWitness;
use super::polarity::{check_polarity_and_proceed, Polarity, THRESHOLD_SCALED};

#[kani::proof]
fn check_polarity_prediction_error_threshold() {
    let w: KernelWitness = kani::any();
    let scaled = w.prediction_error_scaled();
    if scaled <= THRESHOLD_SCALED {
        kani::assert(w.positive() || !w.negative());
    } else {
        kani::assert(!w.positive() || w.negative());
    }
}

#[kani::proof]
fn check_polarity_and_proceed_soundness() {
    let w: KernelWitness = kani::any();
    match check_polarity_and_proceed(&w) {
        Ok(_) => kani::assert(w.polarity() == super::polarity::PolarityResult::Aligned),
        Err(_) => kani::assert(w.polarity() == super::polarity::PolarityResult::Flipped),
    }
}