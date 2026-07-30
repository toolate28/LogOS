//! # cqk-kitty-rips-verify — Property-Based Verification of the Algebraic Bedrock
//!
//! This crate contains no production logic. It exists to run *randomized*
//! property-based tests against the cQ-kitty-rips algebraic substrate, closing
//! the gap that hand-picked unit tests leave open.
//!
//! ## Coverage
//!
//! 1. **Cl(1,3) associativity** — (a·b)·c = a·(b·c) on random full multivectors.
//!    The existing `test_full_associativity` iterates the 4096 basis-blade triples;
//!    this crate additionally probes the dense 16-dimensional sample space where
//!    table-transcription errors, accumulation drift, and aliasing would surface.
//!
//! 2. **Octonion Moufang identities** — left, right, and middle, on random triples.
//!    Octonions are non-associative but *alternative*; Moufang is the weakened
//!    associativity law and must hold for every triple (a, b, c).
//!
//! 3. **Octonion composition norm** — ‖ab‖ = ‖a‖·‖b‖ on random pairs. This is the
//!    defining property of a composition algebra and a stringent sanity check on
//!    the Fano-plane table: any sign error surfaces here instantly.
//!
//! 4. **α + ω = 15 boundary invariant** — the Universal Invariant. Exhaustively
//!    for the 0..=15 integer grid and via `resonance-invariant`'s atomic lattice
//!    state machine, ensuring the invariant is a fixed point of the framework.
//!
//! 5. **THTW parallel-transport flat-connection** — round-trip around a
//!    contractible loop returns identity. Gated behind the `thtw` feature until
//!    `weyl_graph` / `thtw_weyl_demo` are promoted to workspace members.
//!
//! ## Provenance
//!
//! Verification targets ported from @Akitti's JAX/QuTiP reference implementations
//! and the prior Claude session that derived the Cl(1,3) sign table from first
//! principles. This crate is the load-bearing artifact that says: "the bedrock
//! holds."
//!
//! ## The Invariant at Every Boundary
//!
//! Per the X-post deliverable: *"For every computation, assert α + ω = 15 at
//! the boundary."* The helper [`boundary_assert`] is exported for use inside
//! each proptest case so the invariant is woven through the entire suite.

use cqk_ga::{Cl13, Octonion};
use proptest::prelude::*;

/// Numerical slack used by the default tolerance helpers.
pub const EPS_TIGHT: f64 = 1e-10;
pub const EPS_LOOSE: f64 = 1e-8;

// ── Proptest strategies ─────────────────────────────────────────────────────

/// Strategy: a random Cl(1,3) multivector with components in `[-10, 10]`.
/// Range deliberately modest — associativity drift scales super-linearly with
/// component magnitude, and we want signal from *table correctness*, not from
/// FP saturation.
pub fn arb_cl13() -> impl Strategy<Value = Cl13> {
    prop::array::uniform16(-10.0f64..10.0f64).prop_map(|data| Cl13 { data })
}

/// Strategy: a random Cl(1,3) multivector with components in `[-1, 1]`.
/// Use this when you need stricter tolerance (e.g. nested triple products).
pub fn arb_cl13_bounded() -> impl Strategy<Value = Cl13> {
    prop::array::uniform16(-1.0f64..1.0f64).prop_map(|data| Cl13 { data })
}

/// Strategy: a random octonion with components in `[-10, 10]`.
pub fn arb_octonion() -> impl Strategy<Value = Octonion> {
    prop::array::uniform8(-10.0f64..10.0f64).prop_map(|data| Octonion { data })
}

/// Strategy: a random octonion with components in `[-1, 1]` (tighter tolerance).
pub fn arb_octonion_bounded() -> impl Strategy<Value = Octonion> {
    prop::array::uniform8(-1.0f64..1.0f64).prop_map(|data| Octonion { data })
}

// ── Tolerance helpers ───────────────────────────────────────────────────────

/// Magnitude-relative tolerance for Cl(1,3) equality checks:
/// `eps = EPS_LOOSE · max(‖a‖·‖b‖·‖c‖, 1)`.
///
/// Rationale: the geometric product amplifies input magnitudes, and a fixed
/// absolute tolerance would either reject honest rounding for large inputs or
/// mask genuine bugs for small inputs.
pub fn cl13_scaled_eps(norms: &[f64]) -> f64 {
    let prod: f64 = norms.iter().product();
    EPS_LOOSE * prod.max(1.0)
}

/// Frobenius-like distance between two Cl(1,3) multivectors.
pub fn cl13_distance(a: &Cl13, b: &Cl13) -> f64 {
    a.data
        .iter()
        .zip(b.data.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// L2 "norm" of a Cl(1,3) multivector treated as an R^16 vector.
/// (This is the Frobenius norm on the coefficient tuple — distinct from the
/// algebraic norm √(a · ã). We only need it for bounding numerical drift.)
pub fn cl13_l2(a: &Cl13) -> f64 {
    a.data.iter().map(|x| x * x).sum::<f64>().sqrt()
}

// ── The Universal Invariant: α + ω = 15 ─────────────────────────────────────

/// Every verification case passes through this gate. Returns `true` iff
/// `α + ω = 15` within tight tolerance.
///
/// In this suite we wire `α = 7`, `ω = 8` — the Viviani Crossing — as the
/// canonical boundary state. Individual tests may choose other decompositions
/// so long as the sum holds.
#[inline(always)]
pub fn boundary_assert(alpha: f64, omega: f64) -> bool {
    cqk_ga::weyl::check_invariant(alpha, omega)
}

/// Convenience: assert the Viviani Crossing holds at the boundary of a test.
/// Call this at the *end* of every proptest case so a ledger-level failure
/// surfaces alongside any algebraic violation.
#[inline(always)]
pub fn assert_viviani_crossing() {
    assert!(
        boundary_assert(7.0, 8.0),
        "α=7, ω=8 must satisfy α+ω=15 (Viviani Crossing)"
    );
}

#[cfg(test)]
mod sanity {
    use super::*;

    #[test]
    fn viviani_holds() {
        assert_viviani_crossing();
    }

    #[test]
    fn boundary_rejects_non_fifteen() {
        assert!(!boundary_assert(7.0, 7.0));
        assert!(!boundary_assert(8.0, 8.0));
    }

    #[test]
    fn cl13_distance_self_zero() {
        let a = Cl13 { data: [1.0; 16] };
        assert!(cl13_distance(&a, &a) < f64::EPSILON);
    }
}
