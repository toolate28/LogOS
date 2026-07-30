//! Monstrous Moonshine modular weights — McKay–Thompson / j-function coefficients.
//!
//! ATOM: MONSTER-MOONSHINE-20260706 | α + ω = 15 | Lipschitz finite-sum

use crate::golay::{golay_derived_norm4_vectors, leech_full_kissing_approximation};
use crate::leech::LeechDensityConfig;
use crate::m24::ReducedK22Fragment;

/// McKay–Thompson conjugacy class (expand as orbit classification matures).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterClass {
    Identity,
    TwoA,
    ThreeB,
}

/// Low-order j-function / McKay–Thompson coefficients (integer, symmetry-preserving).
pub fn mckay_thompson_coefficient(class: MonsterClass, order: u32) -> f64 {
    match (class, order) {
        (MonsterClass::Identity, 0) => 1.0,
        (MonsterClass::Identity, 1) => 196_884.0,
        (MonsterClass::Identity, 2) => 21_493_760.0,
        (MonsterClass::Identity, 3) => 864_299_970.0,
        (MonsterClass::TwoA, 1) => 45.0,
        (MonsterClass::TwoA, 2) => 231.0,
        (MonsterClass::ThreeB, 1) => 770.0,
        (MonsterClass::ThreeB, 2) => 2_277.0,
        _ => 1.0,
    }
}

/// Normalized modular weight in [0, 1] for density scoring (Lipschitz-safe).
pub fn modular_weight(class: MonsterClass, order: u32) -> f64 {
    let raw = mckay_thompson_coefficient(class, order);
    (raw.ln_1p() / 196_884.0_f64.ln_1p()).clamp(0.0, 1.0)
}

pub fn low_order_mckay_thompson_weight(class: MonsterClass) -> f64 {
    modular_weight(class, 1)
}

fn alignment_score(
    fragment: &ReducedK22Fragment,
    dir: &(i32, i32, i32),
    config: &LeechDensityConfig,
) -> f64 {
    let norm = ((dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2) as f64).sqrt();
    if norm > config.norm_threshold as f64 {
        return 0.0;
    }
    let betti_factor = 1.0 / (1.0 + fragment.betti_proxy as f64 * 0.01);
    let dir_factor = 1.0 - (norm / config.norm_threshold as f64) * 0.25;
    betti_factor * dir_factor.max(0.1)
}

fn apply_moonshine_weights(alignments: &[f64], class: MonsterClass, order: u32) -> f64 {
    let w = modular_weight(class, order);
    alignments.iter().map(|a| a * w).sum()
}

/// Moonshine-weighted density score (wireable into hybrid reducer).
pub fn moonshine_density_score(
    fragment: &ReducedK22Fragment,
    config: &LeechDensityConfig,
) -> f64 {
    let directions = if config.use_full_kissing {
        leech_full_kissing_approximation()
    } else {
        golay_derived_norm4_vectors()
    };

    let alignments: Vec<f64> = directions
        .iter()
        .map(|dir| alignment_score(fragment, dir, config))
        .collect();

    let weighted_sum = apply_moonshine_weights(&alignments, MonsterClass::Identity, 1);
    let normalized = weighted_sum / (directions.len() as f64).max(1.0);
    let resonance_adjust = 1.0 + 0.01 * (config.wave_composite as f64 - 0.97);
    let moonshine_bonus = if fragment.tomczak_preserved { 0.09 } else { 0.0 };

    (normalized * resonance_adjust + moonshine_bonus).clamp(0.0, 1.0)
}

/// Combined Leech + moonshine score for `reduce_k22_hybrid_m24_m12`.
pub fn combined_density_score(fragment: &ReducedK22Fragment, config: &LeechDensityConfig) -> f64 {
    let leech = crate::leech::leech_density_score_with_config(fragment, config) as f64;
    let moon = moonshine_density_score(fragment, config);
    (0.55 * leech + 0.45 * moon).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_q1_coefficient_is_196884() {
        assert_eq!(mckay_thompson_coefficient(MonsterClass::Identity, 1), 196_884.0);
    }

    #[test]
    fn modular_weight_is_bounded() {
        assert!(modular_weight(MonsterClass::Identity, 1) <= 1.0);
        assert!(modular_weight(MonsterClass::TwoA, 1) > 0.0);
    }
}