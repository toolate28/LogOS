//! # Octonion Composition-Norm Property Test
//!
//! The octonions form a *composition algebra*: the norm is multiplicative,
//!
//!   ‖a · b‖ = ‖a‖ · ‖b‖   for all a, b ∈ O.
//!
//! This is the defining property — equivalently, `‖·‖²` is a quadratic form
//! that permits composition. Any sign error in the Fano-plane multiplication
//! table manifests here immediately: a single flipped sign breaks the
//! multiplicative norm on a dense set of pairs.
//!
//! The existing unit test `norm_multiplicative` checks this on *one* hand-
//! picked pair. This file closes the gap.

use cqk_kitty_rips_verify::{arb_octonion_bounded, assert_viviani_crossing};
use proptest::prelude::*;

/// Relative tolerance — the composition identity is exact in exact arithmetic,
/// but we're using f64 through an 8×8 double loop with sign products.
const REL_EPS: f64 = 1e-10;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        max_shrink_iters: 8192,
        ..ProptestConfig::default()
    })]

    /// ‖a · b‖ = ‖a‖ · ‖b‖
    #[test]
    fn norm_is_multiplicative(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
    ) {
        let na  = a.norm();
        let nb  = b.norm();
        let nab = a.mul(&b).norm();

        let expected = na * nb;
        let diff     = (nab - expected).abs();

        // Relative tolerance, falling back to absolute when expected is tiny.
        let tol = REL_EPS * expected.max(1.0);

        prop_assert!(
            diff < tol,
            "composition norm broken: |‖ab‖ − ‖a‖·‖b‖| = {:.3e}, tol = {:.3e}\n\
             ‖a‖ = {}\n ‖b‖ = {}\n ‖ab‖ = {}\n\
             a = {:?}\n b = {:?}",
            diff, tol, na, nb, nab, a.data, b.data,
        );

        assert_viviani_crossing();
    }

    /// Squared-norm variant: ‖a · b‖² = ‖a‖² · ‖b‖². No sqrt round-trip, so
    /// this is the tightest form of the check and catches the smallest drift.
    #[test]
    fn norm_squared_is_multiplicative(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
    ) {
        let nsq_a  = a.norm_squared();
        let nsq_b  = b.norm_squared();
        let nsq_ab = a.mul(&b).norm_squared();

        let expected = nsq_a * nsq_b;
        let diff     = (nsq_ab - expected).abs();
        let tol      = REL_EPS * expected.max(1.0);

        prop_assert!(
            diff < tol,
            "‖ab‖² − ‖a‖²·‖b‖² = {:.3e} > tol {:.3e}",
            diff, tol,
        );

        assert_viviani_crossing();
    }

    /// Conjugation is an involution: (ā)̄ = a.
    #[test]
    fn conjugation_involution(a in arb_octonion_bounded()) {
        let aa = a.conjugate().conjugate();
        prop_assert!(a.approx_eq(&aa, 1e-12));
        assert_viviani_crossing();
    }

    /// a · ā = ‖a‖² · 1 — the scalar-valued norm expressed algebraically.
    #[test]
    fn product_with_conjugate_is_norm_squared(a in arb_octonion_bounded()) {
        let prod = a.mul(&a.conjugate());
        let nsq  = a.norm_squared();

        // prod should be (nsq, 0, 0, 0, 0, 0, 0, 0)
        prop_assert!((prod.data[0] - nsq).abs() < 1e-10 * nsq.max(1.0));
        for i in 1..8 {
            prop_assert!(
                prod.data[i].abs() < 1e-10 * nsq.max(1.0),
                "a·ā has non-zero imaginary part at index {}: {}",
                i, prod.data[i],
            );
        }

        assert_viviani_crossing();
    }
}
