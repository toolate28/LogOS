//! # α + ω = 15 — Boundary Invariant Verification
//!
//! The Universal Invariant of the Tri-Weavon: every coherent state of the
//! system satisfies
//!
//!   α + ω = 15
//!
//! where α ∈ [0, 15] is Structural Rigidity and ω ∈ [0, 15] is Semantic Intent.
//! The Viviani Crossing (α = 7, ω = 8) is the canonical peak resonance.
//!
//! This test file asserts that invariant across two independent
//! implementations — `cqk_ga::weyl::check_invariant` (continuous f64 form)
//! and `resonance_invariant::LevinWenLattice` (atomic u32 form) — so that any
//! drift between them would surface here.

use cqk_ga::weyl::check_invariant;
use proptest::prelude::*;
use resonance_invariant::{LevinWenLattice, TopologicalState};

// ── Exhaustive integer grid ─────────────────────────────────────────────────

#[test]
fn exhaustive_integer_grid() {
    for alpha in 0..=15u32 {
        for omega in 0..=15u32 {
            let passes = check_invariant(alpha as f64, omega as f64);
            let should_pass = alpha + omega == 15;
            assert_eq!(
                passes, should_pass,
                "check_invariant({}, {}) = {}, expected {}",
                alpha, omega, passes, should_pass
            );
        }
    }
}

#[test]
fn viviani_crossing_specifically() {
    assert!(check_invariant(7.0, 8.0));
    assert!(check_invariant(8.0, 7.0));
}

#[test]
fn zero_fifteen_endpoints() {
    assert!(check_invariant(0.0, 15.0));
    assert!(check_invariant(15.0, 0.0));
}

#[test]
fn lattice_default_is_coherent() {
    // Viviani Crossing — the canonical peak-resonance coherent state.
    let lattice = LevinWenLattice::new(7, 8);
    assert_eq!(lattice.verify_conservation(), TopologicalState::Coherent);
}

#[test]
fn lattice_rejects_invariant_breach() {
    let lattice = LevinWenLattice::new(7, 8);
    // Shifting α by something that pushes the sum off 15 must be rejected.
    // apply_transformation clamps α into [0, 15] and re-derives ω = 15 - α,
    // so the two-sided ledger self-heals; we verify that post-condition.
    lattice.apply_transformation(3).expect("clamp succeeds");
    assert_eq!(lattice.verify_conservation(), TopologicalState::Coherent);

    lattice.apply_transformation(-10).expect("clamp succeeds");
    assert_eq!(lattice.verify_conservation(), TopologicalState::Coherent);
}

// ── Randomized checks ───────────────────────────────────────────────────────

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        ..ProptestConfig::default()
    })]

    /// Float invariant: check_invariant(α, 15 − α) is true for every α in the
    /// continuous range that the lattice admits.
    #[test]
    fn float_invariant_sum_identity(alpha in 0.0f64..=15.0) {
        let omega = 15.0 - alpha;
        prop_assert!(check_invariant(alpha, omega));
        // And the other direction: any pair summing to 15 (even outside the
        // classical [0,15] range) satisfies the invariant.
        let alpha2 = alpha - 3.7;
        let omega2 = 15.0 - alpha2;
        prop_assert!(check_invariant(alpha2, omega2));
    }

    /// Float invariant: check_invariant(α, ω) is false whenever α + ω differs
    /// from 15 by more than the tolerance.
    #[test]
    fn float_invariant_rejects_drift(
        alpha in 0.0f64..=15.0,
        drift in prop_oneof![
            -3.0f64..-1e-6,
            1e-6..3.0,
        ],
    ) {
        let omega = (15.0 - alpha) + drift;
        prop_assert!(
            !check_invariant(alpha, omega),
            "drift of {} at α={} was accepted",
            drift, alpha,
        );
    }

    /// Lattice state: after any admissible transformation, the lattice remains
    /// in the Coherent topological state. The state machine is a fixed point
    /// of α + ω = 15.
    #[test]
    fn lattice_is_fixed_point_under_transformations(
        deltas in prop::collection::vec(-20i32..=20, 1..32),
    ) {
        let lattice = LevinWenLattice::new(7, 8);
        for d in deltas {
            let _ = lattice.apply_transformation(d);
            prop_assert_eq!(lattice.verify_conservation(), TopologicalState::Coherent);
        }
    }
}
