//! # Cl(1,3) Associativity — Full Multivector Property Test
//!
//! The existing `cqk_ga::cl13::tests::test_full_associativity` iterates the
//! 4096 basis-blade triples (16³). This test closes the remaining gap: random
//! *full* multivectors with all 16 coefficients populated, where accumulation
//! drift, aliasing, and table-transcription errors would surface.
//!
//! Associativity of the geometric product is a direct consequence of the
//! universal property of the Clifford algebra. If it fails for any triple of
//! full multivectors, the multiplication tables are wrong.

use cqk_ga::Cl13;
use cqk_kitty_rips_verify::{
    arb_cl13_bounded, assert_viviani_crossing, cl13_distance, cl13_l2, EPS_LOOSE,
};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        max_shrink_iters: 8192,
        ..ProptestConfig::default()
    })]

    /// (a·b)·c = a·(b·c) on random bounded Cl(1,3) multivectors.
    #[test]
    fn geometric_product_is_associative(
        a in arb_cl13_bounded(),
        b in arb_cl13_bounded(),
        c in arb_cl13_bounded(),
    ) {
        let left  = (a * b) * c;
        let right = a * (b * c);

        let na = cl13_l2(&a);
        let nb = cl13_l2(&b);
        let nc = cl13_l2(&c);

        // Scale tolerance with the triple product of input magnitudes — a
        // principled bound on the worst-case rounding in 16³ partial sums.
        let eps = EPS_LOOSE * (na * nb * nc).max(1.0) * 16.0;

        let d = cl13_distance(&left, &right);
        prop_assert!(
            d < eps,
            "associativity broken: ‖(ab)c − a(bc)‖ = {:.3e} > eps = {:.3e}\n \
             a = {:?}\n b = {:?}\n c = {:?}",
            d, eps, a.data, b.data, c.data,
        );

        // Boundary invariant must hold at the end of every case.
        assert_viviani_crossing();
    }

    /// Scalar multiplication commutes with the geometric product on both sides.
    /// (λa)·b = λ(a·b) = a·(λb). Catches accidental sign ambiguity in blade ordering.
    #[test]
    fn scalar_linearity(
        a in arb_cl13_bounded(),
        b in arb_cl13_bounded(),
        lambda in -5.0f64..5.0,
    ) {
        let lhs1 = (a * lambda) * b;
        let lhs2 = a * (b * lambda);
        let rhs  = (a * b) * lambda;

        let eps = EPS_LOOSE * (cl13_l2(&a) * cl13_l2(&b) * lambda.abs()).max(1.0) * 16.0;
        prop_assert!(cl13_distance(&lhs1, &rhs) < eps);
        prop_assert!(cl13_distance(&lhs2, &rhs) < eps);

        assert_viviani_crossing();
    }

    /// Distributivity: a·(b + c) = a·b + a·c and (a + b)·c = a·c + b·c.
    #[test]
    fn geometric_product_distributes_over_addition(
        a in arb_cl13_bounded(),
        b in arb_cl13_bounded(),
        c in arb_cl13_bounded(),
    ) {
        let left_dist = a * (b + c);
        let left_sum  = (a * b) + (a * c);

        let right_dist = (a + b) * c;
        let right_sum  = (a * c) + (b * c);

        let eps = EPS_LOOSE * (cl13_l2(&a) * (cl13_l2(&b) + cl13_l2(&c))).max(1.0) * 16.0;
        prop_assert!(cl13_distance(&left_dist,  &left_sum)  < eps);
        prop_assert!(cl13_distance(&right_dist, &right_sum) < eps);

        assert_viviani_crossing();
    }

    /// Scalar 1 is a two-sided identity for the geometric product.
    #[test]
    fn scalar_one_is_identity(a in arb_cl13_bounded()) {
        let one = Cl13 {
            data: {
                let mut d = [0.0; 16];
                d[0] = 1.0;
                d
            },
        };
        let left  = one * a;
        let right = a * one;
        prop_assert!(cl13_distance(&left,  &a) < EPS_LOOSE);
        prop_assert!(cl13_distance(&right, &a) < EPS_LOOSE);

        assert_viviani_crossing();
    }
}
