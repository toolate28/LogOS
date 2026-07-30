//! # Octonion Algebra with Fano-Plane Multiplication Table
//!
//! Implements the 8-dimensional octonion algebra O with the correct
//! non-associative multiplication derived from the Fano plane.
//!
//! Ported from Gemini's battle-tested cQ-kitty-rips-wdbi (29/29 tests passing).
//!
//! ## Fano Plane Structure
//!
//! The 7 imaginary units e₁…e₇ satisfy:
//! - eᵢ² = −1 for all i
//! - eᵢeⱼ = εᵢⱼₖ eₖ for cyclic triples on the Fano plane
//!
//! The 7 cyclic triples (oriented lines, standard Baez/Conway convention):
//! (1,2,3), (1,4,5), (2,4,6), (2,5,7), (3,4,7), (3,6,5), (1,7,6)
//!
//! ## Non-Associativity
//!
//! Octonions are NOT associative: (ab)c ≠ a(bc) in general.
//! They ARE alternative: Moufang identities hold.

use serde::{Deserialize, Serialize};
use core::ops::{Add, Sub, Neg, Mul};

const FANO_TRIPLES: [(usize, usize, usize); 7] = [
    (1, 2, 3),
    (1, 4, 5),
    (2, 4, 6),
    (2, 5, 7),
    (3, 4, 7),
    (3, 6, 5),
    (1, 7, 6),
];

const fn build_octonion_table() -> [[(usize, i8); 8]; 8] {
    let mut table = [[(0usize, 0i8); 8]; 8];
    let mut i = 0;
    while i < 8 {
        table[0][i] = (i, 1);
        table[i][0] = (i, 1);
        i += 1;
    }
    let mut i = 1;
    while i < 8 {
        table[i][i] = (0, -1);
        i += 1;
    }
    let mut t = 0;
    while t < 7 {
        let (i, j, k) = FANO_TRIPLES[t];
        table[i][j] = (k, 1);  table[j][i] = (k, -1);
        table[j][k] = (i, 1);  table[k][j] = (i, -1);
        table[k][i] = (j, 1);  table[i][k] = (j, -1);
        t += 1;
    }
    table
}

const OCTONION_TABLE: [[(usize, i8); 8]; 8] = build_octonion_table();

/// An octonion: a = a₀ + a₁e₁ + a₂e₂ + … + a₇e₇
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Octonion {
    pub data: [f64; 8],
}

impl Octonion {
    pub const ZERO: Self = Octonion { data: [0.0; 8] };
    pub const ONE: Self = Octonion { data: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };

    pub const fn scalar(s: f64) -> Self {
        let mut d = [0.0; 8]; d[0] = s; Octonion { data: d }
    }

    pub fn unit(k: usize) -> Self {
        assert!(k < 8);
        let mut d = [0.0; 8]; d[k] = 1.0; Octonion { data: d }
    }

    #[inline(always)]
    pub fn real(&self) -> f64 { self.data[0] }

    pub fn imag(&self) -> [f64; 7] {
        let mut im = [0.0; 7];
        im.copy_from_slice(&self.data[1..8]);
        im
    }

    /// Octonion multiplication using the Fano-plane table.
    /// NOT associative. (ab)c ≠ a(bc) in general.
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

    pub fn conjugate(&self) -> Octonion {
        let mut result = self.data;
        for i in 1..8 { result[i] = -result[i]; }
        Octonion { data: result }
    }

    pub fn norm_squared(&self) -> f64 { self.data.iter().map(|x| x * x).sum() }
    pub fn norm(&self) -> f64 { self.norm_squared().sqrt() }

    pub fn inverse(&self) -> Option<Octonion> {
        let ns = self.norm_squared();
        if ns < 1e-15 { return None; }
        let conj = self.conjugate();
        Some(Octonion { data: conj.data.map(|x| x / ns) })
    }

    pub fn approx_eq(&self, other: &Octonion, eps: f64) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| (a - b).abs() < eps)
    }

    // ── Moufang identity checks ──────────────────────────────────

    /// Left Moufang: a(b(ac)) = ((ab)a)c
    pub fn check_left_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        let ac = Octonion::mul(self, c);
        let bac = Octonion::mul(b, &ac);
        let lhs = Octonion::mul(self, &bac);
        let ab = Octonion::mul(self, b);
        let aba = Octonion::mul(&ab, self);
        let rhs = Octonion::mul(&aba, c);
        lhs.approx_eq(&rhs, eps)
    }

    /// Right Moufang: ((ca)b)a = c(a(ba))
    pub fn check_right_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        let ca = Octonion::mul(c, self);
        let cab = Octonion::mul(&ca, b);
        let lhs = Octonion::mul(&cab, self);
        let ba = Octonion::mul(b, self);
        let aba = Octonion::mul(self, &ba);
        let rhs = Octonion::mul(c, &aba);
        lhs.approx_eq(&rhs, eps)
    }

    /// Middle Moufang: (ab)(ca) = (a(bc))a
    pub fn check_middle_moufang(&self, b: &Octonion, c: &Octonion, eps: f64) -> bool {
        let ab = Octonion::mul(self, b);
        let ca = Octonion::mul(c, self);
        let lhs = Octonion::mul(&ab, &ca);
        let bc = Octonion::mul(b, c);
        let abc = Octonion::mul(self, &bc);
        let rhs = Octonion::mul(&abc, self);
        lhs.approx_eq(&rhs, eps)
    }
}

impl Mul for Octonion {
    type Output = Octonion;
    fn mul(self, rhs: Octonion) -> Octonion { Octonion::mul(&self, &rhs) }
}

impl Add for Octonion {
    type Output = Octonion;
    fn add(self, rhs: Octonion) -> Octonion {
        let mut r = [0.0; 8];
        for i in 0..8 { r[i] = self.data[i] + rhs.data[i]; }
        Octonion { data: r }
    }
}

impl Sub for Octonion {
    type Output = Octonion;
    fn sub(self, rhs: Octonion) -> Octonion {
        let mut r = [0.0; 8];
        for i in 0..8 { r[i] = self.data[i] - rhs.data[i]; }
        Octonion { data: r }
    }
}

impl Neg for Octonion {
    type Output = Octonion;
    fn neg(self) -> Octonion { Octonion { data: self.data.map(|x| -x) } }
}

/// Expose tables for verification.
pub mod tables {
    pub const FANO_TRIPLES: &[(usize, usize, usize); 7] = &super::FANO_TRIPLES;
    pub const OCTONION_TABLE: &[[(usize, i8); 8]; 8] = &super::OCTONION_TABLE;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imaginary_units_square_to_minus_one() {
        for i in 1..8 {
            let ei = Octonion::unit(i);
            let sq = Octonion::mul(&ei, &ei);
            assert!((sq.real() - (-1.0)).abs() < 1e-12, "e{}^2 != -1", i);
            assert!(sq.imag().iter().all(|x| x.abs() < 1e-12));
        }
    }

    #[test]
    fn fano_triples_correct() {
        for &(i, j, k) in FANO_TRIPLES.iter() {
            let ei = Octonion::unit(i);
            let ej = Octonion::unit(j);
            let ek = Octonion::unit(k);
            assert!(Octonion::mul(&ei, &ej).approx_eq(&ek, 1e-12), "e{} * e{} != e{}", i, j, k);
            assert!(Octonion::mul(&ej, &ei).approx_eq(&(-ek), 1e-12));
        }
    }

    #[test]
    fn not_associative() {
        let e3 = Octonion::unit(3);
        let e5 = Octonion::unit(5);
        let e4 = Octonion::unit(4);
        let left = Octonion::mul(&Octonion::mul(&e3, &e5), &e4);
        let right = Octonion::mul(&e3, &Octonion::mul(&e5, &e4));
        assert!(!left.approx_eq(&right, 1e-12));
    }

    #[test]
    fn moufang_identities() {
        let a = Octonion { data: [1.0, 0.5, -0.3, 0.2, 0.1, -0.4, 0.6, -0.1] };
        let b = Octonion { data: [0.3, -0.2, 0.7, -0.1, 0.4, 0.2, -0.5, 0.3] };
        let c = Octonion { data: [-0.1, 0.3, 0.1, 0.6, -0.2, 0.5, 0.1, -0.4] };
        assert!(a.check_left_moufang(&b, &c, 1e-10));
        assert!(a.check_right_moufang(&b, &c, 1e-10));
        assert!(a.check_middle_moufang(&b, &c, 1e-10));
    }

    #[test]
    fn norm_multiplicative() {
        let a = Octonion { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] };
        let b = Octonion { data: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] };
        let ab = Octonion::mul(&a, &b);
        assert!((a.norm() * b.norm() - ab.norm()).abs() < 1e-10);
    }
}
