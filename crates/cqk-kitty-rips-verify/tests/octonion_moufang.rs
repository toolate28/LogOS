//! # Octonion Moufang Identities — Property Test
//!
//! Octonions are non-associative but *alternative* — they satisfy the three
//! Moufang identities, which are the weakest associativity laws compatible
//! with a composition algebra.
//!
//! - **Left Moufang:**   a (b (a c))   = ((a b) a) c
//! - **Right Moufang:**  ((c a) b) a   = c (a (b a))
//! - **Middle Moufang:** (a b)(c a)    = (a (b c)) a
//!
//! The existing unit test checks these on *one* hand-picked triple. If the
//! Fano-plane sign table in `cqk-ga::octonion` had a single flipped sign, that
//! singleton test might pass by coincidence while the algebra is fundamentally
//! broken. Random triples rule that out.
//!
//! Reference: Schafer, "An Introduction to Nonassociative Algebras" (1966);
//! Baez, "The Octonions" (2002), §2.
//!
//! Ported from @Akitti's JAX reference. Rigor category: A (derived law).

use cqk_kitty_rips_verify::{arb_octonion_bounded, assert_viviani_crossing};
use proptest::prelude::*;

const MOUFANG_EPS: f64 = 1e-8;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        max_shrink_iters: 8192,
        ..ProptestConfig::default()
    })]

    /// Left Moufang: a(b(ac)) = ((ab)a)c
    #[test]
    fn left_moufang(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
        c in arb_octonion_bounded(),
    ) {
        prop_assert!(
            a.check_left_moufang(&b, &c, MOUFANG_EPS),
            "left Moufang broken:\n a = {:?}\n b = {:?}\n c = {:?}",
            a.data, b.data, c.data,
        );
        assert_viviani_crossing();
    }

    /// Right Moufang: ((ca)b)a = c(a(ba))
    #[test]
    fn right_moufang(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
        c in arb_octonion_bounded(),
    ) {
        prop_assert!(
            a.check_right_moufang(&b, &c, MOUFANG_EPS),
            "right Moufang broken:\n a = {:?}\n b = {:?}\n c = {:?}",
            a.data, b.data, c.data,
        );
        assert_viviani_crossing();
    }

    /// Middle Moufang: (ab)(ca) = (a(bc))a
    #[test]
    fn middle_moufang(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
        c in arb_octonion_bounded(),
    ) {
        prop_assert!(
            a.check_middle_moufang(&b, &c, MOUFANG_EPS),
            "middle Moufang broken:\n a = {:?}\n b = {:?}\n c = {:?}",
            a.data, b.data, c.data,
        );
        assert_viviani_crossing();
    }

    /// All three Moufang identities hold simultaneously (the real load).
    #[test]
    fn all_three_moufang(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
        c in arb_octonion_bounded(),
    ) {
        prop_assert!(a.check_left_moufang(&b, &c, MOUFANG_EPS));
        prop_assert!(a.check_right_moufang(&b, &c, MOUFANG_EPS));
        prop_assert!(a.check_middle_moufang(&b, &c, MOUFANG_EPS));
        assert_viviani_crossing();
    }

    /// Alternative property (consequence of Moufang): the subalgebra generated
    /// by any two elements is associative. I.e. (aa)b = a(ab) and (ab)b = a(bb).
    #[test]
    fn alternative_law(
        a in arb_octonion_bounded(),
        b in arb_octonion_bounded(),
    ) {
        let lhs_left  = a.mul(&a).mul(&b);
        let rhs_left  = a.mul(&a.mul(&b));
        prop_assert!(
            lhs_left.approx_eq(&rhs_left, MOUFANG_EPS),
            "left-alternative broken: (aa)b != a(ab)",
        );

        let lhs_right = a.mul(&b).mul(&b);
        let rhs_right = a.mul(&b.mul(&b));
        prop_assert!(
            lhs_right.approx_eq(&rhs_right, MOUFANG_EPS),
            "right-alternative broken: (ab)b != a(bb)",
        );

        assert_viviani_crossing();
    }
}
