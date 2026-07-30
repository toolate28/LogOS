//! # cqk-ga — Geometric Algebra for the cQ-kitty-rips Pipeline
//!
//! Provides the algebraic substrate for the NS singularity hunt:
//!
//! - **Cl(1,3)** — Spacetime Clifford algebra with signature (+,−,−,−)
//!   Complete geometric product via compile-time multiplication tables.
//!   Ported from Gemini's battle-tested implementation (29/29 tests, 4096-triple associativity).
//!
//! - **Weyl** — Weyl gauge theory: R̂, F̂, Lagrangian, covariant derivative.
//!   Standard results from local scale invariance (Category A).
//!
//! - **Octonion** — 8D non-associative normed division algebra with Fano-plane table.
//!   Moufang identities verified. Composition algebra property: ‖ab‖ = ‖a‖·‖b‖.
//!
//! ## Architecture
//!
//! ```text
//! cqk-ga
//! ├── cl13      Cl(1,3) Clifford algebra (compile-time tables)
//! ├── weyl      Weyl gauge theory (R̂, F̂, L)
//! ├── octonion  Fano-plane octonion algebra
//! └── bridge    Cl(1,3) bivector → octonion embedding
//! ```
//!
//! ## Provenance
//!
//! Core algebra ported from Gemini's cQ-kitty-rips-ga (Scale strand).
//! 29/29 verification checks passing. Two bugs caught during integration:
//! - D² = 2+φ (not φ²) — see cqk-anyon for details
//! - Yang-Baxter hexagon = σ₁σ₂σ₁ = σ₂σ₁σ₂ where σ₂ = F·R·F
//!
//! References:
//!   Ghilencea arXiv:2604.07508 (Weyl gauge theory)
//!   Baez "The Octonions" (Fano plane conventions)

pub mod cl13;
pub mod weyl;
pub mod octonion;

// Re-exports for convenience
pub use cl13::Cl13;
pub use octonion::Octonion;

/// Map from Cl(1,3) bivector to octonion via the exceptional embedding.
/// The 6 bivector components map to octonion imaginary units 1..6;
/// unit 7 is reserved for the pseudoscalar lift.
pub fn bivector_to_octonion(mv: &Cl13) -> Octonion {
    let bv = mv.grade_project(2);
    let mut oct = Octonion::ZERO;
    // Bivector components are at indices 5..10 in Cl(1,3)
    for i in 0..6 {
        oct.data[i + 1] = bv.data[5 + i];
    }
    oct
}

/// Map from octonion back to Cl(1,3) bivector (inverse of the embedding).
pub fn octonion_to_bivector(oct: &Octonion) -> Cl13 {
    let mut bv = [0.0f64; 6];
    for i in 0..6 {
        bv[i] = oct.data[i + 1];
    }
    Cl13::bivector(bv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bivector_octonion_roundtrip() {
        let bv = Cl13::bivector([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let oct = bivector_to_octonion(&bv);
        let bv2 = octonion_to_bivector(&oct);
        assert!(bv.approx_eq(&bv2, 1e-12));
    }

    #[test]
    fn octonion_unit_norm() {
        assert!((Octonion::ONE.norm() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn cl13_metric() {
        // e₀² = +1 (timelike), e₁² = −1 (spacelike)
        assert_eq!((Cl13::e0() * Cl13::e0()).scalar_part(), 1.0);
        assert_eq!((Cl13::e1() * Cl13::e1()).scalar_part(), -1.0);
    }
}
