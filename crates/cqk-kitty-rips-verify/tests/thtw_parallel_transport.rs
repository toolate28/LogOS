//! # THTW Parallel-Transport Flat-Connection Test — STUB
//!
//! Per the X-post deliverable:
//! > *"Generate random THTW graphs, verify that parallel transport around
//! > contractible loops returns identity (flat connection test)."*
//!
//! This test is gated behind the `thtw` feature and currently `#[ignore]`d
//! because the `weyl_graph` and `thtw_weyl_demo` crates are not yet promoted
//! to root-workspace members. When they are, the skeleton below should be
//! fleshed out and the `#[ignore]` removed.
//!
//! ## Strategy sketch (for when we fill this in)
//!
//! 1. Build a random THTW graph `G = (V, E)` with |V| in 4..32 and a random
//!    Weyl connection ω : E → Cl(1,3) bivector (or scalar per edge if we're
//!    testing the abelian reduction first).
//!
//! 2. Identify a contractible loop — easiest source is a single triangle face
//!    — and compute the holonomy = product of exp(ω) around the loop.
//!
//! 3. On a *flat* connection (dω = 0, which we enforce by construction for the
//!    test), the holonomy must be the identity element of the gauge group.
//!
//! 4. Boundary: assert α + ω = 15 on each case.
//!
//! Reference: Ghilencea arXiv:2604.07508 (Weyl gauge theory, local scale
//! invariance). Conventions match @Akitti's JAX reference.

#[cfg(feature = "thtw")]
mod thtw_enabled {
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

        #[test]
        #[ignore = "pending weyl_graph / thtw_weyl_demo promotion to root workspace"]
        fn flat_connection_holonomy_is_identity(
            _n_vertices in 4usize..32,
            _seed in any::<u64>(),
        ) {
            // TODO: once weyl_graph is a workspace dep,
            //   let g = weyl_graph::random_thtw(n_vertices, seed);
            //   let omega = weyl_graph::flat_connection(&g);
            //   for face in g.triangular_faces() {
            //       let h = weyl_graph::holonomy(&g, &omega, &face);
            //       prop_assert!(h.approx_eq(&Cl13::identity(), 1e-10));
            //   }
            //   cqk_kitty_rips_verify::assert_viviani_crossing();
            prop_assume!(false);
        }
    }
}

#[cfg(not(feature = "thtw"))]
#[test]
#[ignore = "enable with --features thtw after weyl_graph is added to root workspace"]
fn thtw_parallel_transport_placeholder() {
    // Deliberately ignored; exists so `cargo test -p cqk-kitty-rips-verify --
    // --ignored` lists this as a known-pending verification target.
}
