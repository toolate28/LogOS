//! # Octonion Algebra with Fano-Plane Multiplication Table
//!
//! Implements the 8-dimensional octonion algebra O with the correct
//! non-associative multiplication derived from the Fano plane.
//!
//! ## Fano Plane Structure
//!
//! The 7 imaginary units e₁…e₇ satisfy:
//! - eᵢ² = −1 for all i
//! - eᵢeⱼ = εᵢⱼₖ eₖ for cyclic triples on the Fano plane
//!
//! The 7 cyclic triples (oriented lines of the Fano plane) are:
//! (1,2,3), (2,4,6), (4,5,1), (1,6,7), (2,5,7), (3,4,7), (3,6,5)
//!
//! ## Rigor Category
//!
//! - Multiplication table: Category A (derived from Fano plane — unique up to automorphism)
//! - Fuzzy coefficients: Category D (unanchored — modulated by empirical parameters)
//!
//! ## Non-Associativity Warning
//!
//! Octonions are NOT associative: (ab)c ≠ a(bc) in general.
//! They ARE alternative: a(ab) = a²b and (ab)b = ab² (Moufang identities hold).
//! Any computation involving 3+ multiplications is path-dependent.

use serde::{Deserialize, Serialize};
use core::ops::{Add, Sub, Neg, Mul};

/// The 7 oriented triples of the Fano plane.
/// Using the STANDARD orientation (Baez/Conway convention) that produces
/// a valid alternative algebra satisfying the Moufang identities.
///
/// Each triple (i, j, k) means eᵢ · eⱼ = eₖ (and eⱼ · eᵢ = −eₖ).
///
/// ⚠️ NOT all orientations of the Fano plane give valid octonion algebras!
/// Only orientations satisfying the quadratic residue consistency condition work.
/// This specific set is derived from the standard Cayley-Dickson construction.
const FANO_TRIPLES: [(usize, usize, usize); 7] = [
    (1, 2, 3),
    (1, 4, 5),
    (2, 4, 6),
    (2, 5, 7),
    (3, 4, 7),
    (3, 6, 5),
    (1, 7, 6),
];

/// Precomputed multiplication table for imaginary units.
/// OCTONION_TABLE[i][j] = (result_index, sign) for eᵢ · eⱼ where i,j ∈ 1..7
/// Returns (0, 0) for the scalar part (eᵢ · eᵢ = −1).
const fn build_octonion_table() -> [[(usize, i8); 8]; 8] {
    let mut table = [[(0usize, 0i8); 8]; 8];

    // e₀ (scalar = 1) multiplication
    let mut i = 0;
    while i < 8 {
        table[0][i] = (i, 1);  // 1 · eᵢ = eᵢ
        table[i][0] = (i, 1);  // eᵢ · 1 = eᵢ
        i += 1;
    }

    // eᵢ · eᵢ = −1 for i > 0
    let mut i = 1;
    while i < 8 {
        table[i][i] = (0, -1); // squares to −1 (result is scalar)
        i += 1;
    }

    // Fano plane triples: eᵢ · eⱼ = eₖ, eⱼ · eᵢ = −eₖ
    let mut t = 0;
    while t < 7 {
        let (i, j, k) = FANO_TRIPLES[t];
        table[i][j] = (k, 1);   // eᵢ · eⱼ = +eₖ
        table[j][i] = (k, -1);  // eⱼ · eᵢ = −eₖ

        table[j][k] = (i, 1);   // cyclic: eⱼ · eₖ = +eᵢ
        table[k][j] = (i, -1);

        table[k][i] = (j, 1);   // cyclic: eₖ · eᵢ = +eⱼ
        table[i][k] = (j, -1);

        t += 1;
    }

    table
}

/// The compile-time multiplication table.
const OCTONION_TABLE: [[(usize, i8); 8]; 8] = build_octonion_table();

/// An octonion: a = a₀ + a₁e₁ + a₂e₂ + … + a₇e₇
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Octonion {
    pub data: [f64; 8],
}

impl Octonion {
    pub const ZERO: Self = Octonion { data: [0.0; 8] };
    pub const ONE: Self = Octonion { data: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };

    /// Construct from scalar part only.
    pub const fn scalar(s: f64) -> Self {
        let mut d = [0.0; 8];
        d[0] = s;
        Octonion { data: d }
    }

    /// Construct a unit imaginary octonion eₖ.
    pub fn unit(k: usize) -> Self {
        assert!(k < 8);
        let mut d = [0.0; 8];
        d[k] = 1.0;
        Octonion { data: d }
    }

    /// Scalar (real) part.
    #[inline(always)]
    pub fn real(&self) -> f64 { self.data[0] }

    /// Imaginary part as 7-vector.
    pub fn imag(&self) -> [f64; 7] {
        let mut im = [0.0; 7];
        im.copy_from_slice(&self.data[1..8]);
        im
    }

    /// Octonion multiplication using the Fano-plane table.
    ///
    /// ⚠️ This is NOT associative. (ab)c ≠ a(bc) in general.
    pub fn mul(&self, rhs: &Octonion) -> Octonion {
        let mut result = [0.0; 8];
        for i in 0..8 {
            if self.data[i] == 0.0 { continue; }
            for j in 0..8 {
                if rhs.data[j] == 0.0 { continue; }
                let (target, sign) = OCTONION_TABLE[i][j];
                result[target] += (sign as f64) * self.data[i] * rhs.data[j];
            }
        }
        Octonion { data: result }
    }

    /// Conjugate: a* = a₀ − a₁e₁ − … − a₇e₇
    pub fn conjugate(&self) -> Octonion {
        let mut result = self.data;
        for i in 1..8 { result[i] = -result[i]; }
        Octonion { data: result }
    }

    /// Norm squared: ‖a‖² = a · a* (always real and non-negative)
    pub fn norm_squared(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum()
    }

    /// Norm: ‖a‖
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Inverse: a⁻¹ = a* / ‖a‖² (exists for all nonzero octonions)
    pub fn inverse(&self) -> Option<Octonion> {
        let ns = self.norm_squared();
        if ns < 1e-15 { return None; }
        let conj = self.conjugate();
        Some(Octonion {
            data: conj.data.map(|x| x / ns),
        })
    }

    /// Check approximate equality.
    pub fn approx_eq(&self, other: &Octonion, eps: f64) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| (a - b).abs() < eps)
    }

    // ── Moufang identity checks ──────────────────────────────────

    /// Check left Moufang identity: a(b(ac)) = ((ab)a)c
    pub fn check_left_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        // LHS: a · (b · (a · c))
        let ac = Octonion::mul(self, c);
        let bac = Octonion::mul(b, &ac);
        let lhs = Octonion::mul(self, &bac);

        // RHS: ((a · b) · a) · c
        let ab = Octonion::mul(self, b);
        let aba = Octonion::mul(&ab, self);
        let rhs = Octonion::mul(&aba, c);

        lhs.approx_eq(&rhs, eps)
    }

    /// Check right Moufang identity: ((ca)b)a = c(a(ba))
    pub fn check_right_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        // LHS: ((c · a) · b) · a
        let ca = Octonion::mul(c, self);
        let cab = Octonion::mul(&ca, b);
        let lhs = Octonion::mul(&cab, self);

        // RHS: c · (a · (b · a))
        let ba = Octonion::mul(b, self);
        let aba = Octonion::mul(self, &ba);
        let rhs = Octonion::mul(c, &aba);

        lhs.approx_eq(&rhs, eps)
    }

    /// Check middle Moufang identity: (ab)(ca) = a(bc)a
    pub fn check_middle_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        // LHS: (a · b) · (c · a)
        let ab = Octonion::mul(self, b);
        let ca = Octonion::mul(c, self);
        let lhs = Octonion::mul(&ab, &ca);

        // RHS: (a · (b · c)) · a  — note: a(bc)a means (a·(b·c))·a
        let bc = Octonion::mul(b, c);
        let abc = Octonion::mul(self, &bc);
        let rhs = Octonion::mul(&abc, self);

        lhs.approx_eq(&rhs, eps)
    }
}

impl Mul for Octonion {
    type Output = Octonion;
    fn mul(self, rhs: Octonion) -> Octonion {
        Octonion::mul(&self, &rhs)
    }
}

impl Add for Octonion {
    type Output = Octonion;
    fn add(self, rhs: Octonion) -> Octonion {
        let mut result = [0.0; 8];
        for i in 0..8 { result[i] = self.data[i] + rhs.data[i]; }
        Octonion { data: result }
    }
}

impl Sub for Octonion {
    type Output = Octonion;
    fn sub(self, rhs: Octonion) -> Octonion {
        let mut result = [0.0; 8];
        for i in 0..8 { result[i] = self.data[i] - rhs.data[i]; }
        Octonion { data: result }
    }
}

impl Neg for Octonion {
    type Output = Octonion;
    fn neg(self) -> Octonion {
        Octonion { data: self.data.map(|x| -x) }
    }
}

// ──────────────────────────────────────────────────────────────────
// Fuzzy Octonion Triad (Category D — experimental)
// ──────────────────────────────────────────────────────────────────

/// Fuzzy octonion triad with coefficients modulated by tilt and fractal density.
///
/// ⚠️ Category D: unanchored. The modulation formula is a design convention
/// from @Akitti's THTW stack, not derived from first principles.
///
/// Gated behind `experimental` feature flag.
#[cfg(feature = "experimental")]
#[derive(Clone, Debug)]
pub struct FuzzyOctonionTriad {
    /// The three basis octonions of the triad
    pub triads: [Octonion; 3],
    /// Fuzzy coefficients modulated by tilt
    pub fuzzy_coeffs: [f64; 3],
}

#[cfg(feature = "experimental")]
impl FuzzyOctonionTriad {
    /// Create a fuzzy triad modulated by Weyl cone tilt and fractal density.
    ///
    /// - `tilt_w0`: Weyl cone tilt parameter (w₀ > v_F for tilted regime)
    /// - `fractal_rho`: MandelbulbFoam density ρ
    /// - `viscoelastic_eta`: Viscoelastic back-reaction parameter η
    pub fn new(tilt_w0: f64, fractal_rho: f64, viscoelastic_eta: f64) -> Self {
        // Fuzzy coefficients: ã_i = w₀ / (1 + ρ · η)
        // Category D: this formula is a design convention
        let denom = 1.0 + fractal_rho * viscoelastic_eta;
        let base_coeff = tilt_w0 / denom;

        // Three triads using Fano-plane generating triples
        let t1 = Octonion::unit(1); // e₁
        let t2 = Octonion::unit(2); // e₂
        let t3 = Octonion::unit(4); // e₄ (chosen for maximal Fano separation)

        FuzzyOctonionTriad {
            triads: [t1, t2, t3],
            fuzzy_coeffs: [base_coeff, base_coeff * 0.618, base_coeff * 0.382],
            // φ⁻¹ ≈ 0.618 and 1−φ⁻¹ ≈ 0.382 — Fibonacci weighting
        }
    }

    /// Evaluate the fuzzy WDBI leading ξ⁰ term.
    ///
    /// Returns the contribution to the effective action from the
    /// fuzzy octonion sector: Σ ãᵢ² ‖tᵢ‖²
    pub fn wdbi_leading_term(&self) -> f64 {
        self.triads.iter()
            .zip(self.fuzzy_coeffs.iter())
            .map(|(t, &a)| a * a * t.norm_squared())
            .sum()
    }
}

/// Expose the Fano-plane table for verification.
pub mod tables {
    pub const FANO_TRIPLES: &[(usize, usize, usize); 7] = &super::FANO_TRIPLES;
    pub const OCTONION_TABLE: &[[(usize, i8); 8]; 8] = &super::OCTONION_TABLE;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imaginary_units_square_to_minus_one() {
        for i in 1..8 {
            let ei = Octonion::unit(i);
            let sq = ei.mul(ei);
            assert!(
                (sq.real() - (-1.0)).abs() < 1e-12 &&
                sq.imag().iter().all(|x| x.abs() < 1e-12),
                "e{}² should be −1", i
            );
        }
    }

    #[test]
    fn test_fano_triples() {
        // For each triple (i,j,k): eᵢ · eⱼ = +eₖ
        for &(i, j, k) in FANO_TRIPLES.iter() {
            let ei = Octonion::unit(i);
            let ej = Octonion::unit(j);
            let ek = Octonion::unit(k);
            let prod = ei.mul(ej);
            assert!(
                prod.approx_eq(&ek, 1e-12),
                "e{} · e{} should be e{}", i, j, k
            );

            // And reverse: eⱼ · eᵢ = −eₖ
            let rev = ej.mul(ei);
            assert!(
                rev.approx_eq(&(-ek), 1e-12),
                "e{} · e{} should be −e{}", j, i, k
            );
        }
    }

    #[test]
    fn test_not_associative() {
        // Octonions are NOT associative. Demonstrate with e₃, e₅, e₄.
        // (e₃·e₅)·e₄ ≠ e₃·(e₅·e₄) — these span different quaternion subalgebras.
        let e3 = Octonion::unit(3);
        let e5 = Octonion::unit(5);
        let e4 = Octonion::unit(4);

        let left = Octonion::mul(&Octonion::mul(&e3, &e5), &e4);
        let right = Octonion::mul(&e3, &Octonion::mul(&e5, &e4));

        assert!(
            !left.approx_eq(&right, 1e-12),
            "Octonions should NOT be associative"
        );
    }

    #[test]
    fn test_moufang_identities() {
        // Moufang identities MUST hold — they're the weakened associativity
        let a = Octonion { data: [1.0, 0.5, -0.3, 0.2, 0.1, -0.4, 0.6, -0.1] };
        let b = Octonion { data: [0.3, -0.2, 0.7, -0.1, 0.4, 0.2, -0.5, 0.3] };
        let c = Octonion { data: [-0.1, 0.3, 0.1, 0.6, -0.2, 0.5, 0.1, -0.4] };

        assert!(a.check_left_moufang(&b, &c, 1e-10), "Left Moufang failed");
        assert!(a.check_right_moufang(&b, &c, 1e-10), "Right Moufang failed");
        assert!(a.check_middle_moufang(&b, &c, 1e-10), "Middle Moufang failed");
    }

    #[test]
    fn test_norm_multiplicative() {
        // ‖ab‖ = ‖a‖ · ‖b‖ (composition algebra property)
        let a = Octonion { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] };
        let b = Octonion { data: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] };
        let ab = a.mul(b);

        let norm_product = a.norm() * b.norm();
        let product_norm = ab.norm();
        assert!(
            (norm_product - product_norm).abs() < 1e-10,
            "‖ab‖ should equal ‖a‖·‖b‖"
        );
    }

    #[test]
    fn test_inverse() {
        let a = Octonion { data: [1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
        let a_inv = a.inverse().expect("Should have inverse");
        let product = a.mul(a_inv);
        assert!(
            product.approx_eq(&Octonion::ONE, 1e-10),
            "a · a⁻¹ should be 1"
        );
    }
}
