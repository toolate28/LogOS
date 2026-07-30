//! # Cl(1,3) Clifford Algebra — The Bedrock
//!
//! Full 16-dimensional Clifford algebra over spacetime signature (+,-,-,-)
//! with the geometric product table derived at compile time from first principles.
//!
//! Ported from Gemini's battle-tested cQ-kitty-rips-ga (29/29 tests passing).
//!
//! ## Basis Element Indexing
//!
//! | Index | Element  | Bitmask | Grade |
//! |-------|----------|---------|-------|
//! |   0   |    1     | 0b0000  |   0   |
//! |   1   |   e₀     | 0b0001  |   1   |
//! |   2   |   e₁     | 0b0010  |   1   |
//! |   3   |   e₂     | 0b0100  |   1   |
//! |   4   |   e₃     | 0b1000  |   1   |
//! |   5   |  e₀₁     | 0b0011  |   2   |
//! |   6   |  e₀₂     | 0b0101  |   2   |
//! |   7   |  e₀₃     | 0b1001  |   2   |
//! |   8   |  e₁₂     | 0b0110  |   2   |
//! |   9   |  e₁₃     | 0b1010  |   2   |
//! |  10   |  e₂₃     | 0b1100  |   2   |
//! |  11   |  e₀₁₂    | 0b0111  |   3   |
//! |  12   |  e₀₁₃    | 0b1011  |   3   |
//! |  13   |  e₀₂₃    | 0b1101  |   3   |
//! |  14   |  e₁₂₃    | 0b1110  |   3   |
//! |  15   |  e₀₁₂₃   | 0b1111  |   4   |
//!
//! ## Metric Signature
//!
//! e₀² = +1 (timelike), e₁² = e₂² = e₃² = −1 (spacelike)
//!
//! ## Derivation Method
//!
//! The geometric product of basis blades eₐ · eᵦ is computed by:
//! 1. Result blade = A ⊕ B (symmetric difference of index sets, i.e. XOR of bitmasks)
//! 2. Sign = (−1)^swaps × ∏(metric of shared generators)
//!    where swaps = bubble sort count to canonically order the concatenated index sequence
//!
//! This is the UNIQUE correct table determined by the universal property of Cl(1,3).

use serde::{Deserialize, Serialize};
use core::ops::{Add, Sub, Mul, Neg};

// ──────────────────────────────────────────────────────────────────
// Compile-time table generation
// ──────────────────────────────────────────────────────────────────

/// Bitmask representation for each of the 16 basis elements.
const BITMASK: [u8; 16] = [
    0b0000, // 0:  1
    0b0001, // 1:  e₀
    0b0010, // 2:  e₁
    0b0100, // 3:  e₂
    0b1000, // 4:  e₃
    0b0011, // 5:  e₀₁
    0b0101, // 6:  e₀₂
    0b1001, // 7:  e₀₃
    0b0110, // 8:  e₁₂
    0b1010, // 9:  e₁₃
    0b1100, // 10: e₂₃
    0b0111, // 11: e₀₁₂
    0b1011, // 12: e₀₁₃
    0b1101, // 13: e₀₂₃
    0b1110, // 14: e₁₂₃
    0b1111, // 15: e₀₁₂₃
];

const fn bitmask_to_index(mask: u8) -> usize {
    let mut i = 0;
    while i < 16 {
        if BITMASK[i] == mask { return i; }
        i += 1;
    }
    0
}

/// Metric signature: e₀² = +1, e₁² = e₂² = e₃² = −1
const fn metric(generator: u8) -> i8 {
    if generator == 0 { 1 } else { -1 }
}

const fn count_swaps(a_mask: u8, b_mask: u8) -> u32 {
    let mut swaps = 0u32;
    let mut j: u8 = 0;
    while j < 4 {
        if (b_mask >> j) & 1 == 1 {
            let mut i: u8 = j + 1;
            while i < 4 {
                if (a_mask >> i) & 1 == 1 {
                    swaps += 1;
                }
                i += 1;
            }
        }
        j += 1;
    }
    swaps
}

const fn basis_product(a_idx: usize, b_idx: usize) -> (usize, i8) {
    let a = BITMASK[a_idx];
    let b = BITMASK[b_idx];
    let result_mask = a ^ b;
    let result_idx = bitmask_to_index(result_mask);
    let swaps = count_swaps(a, b);
    let mut sign: i8 = if swaps % 2 == 0 { 1 } else { -1 };
    let common = a & b;
    let mut k: u8 = 0;
    while k < 4 {
        if (common >> k) & 1 == 1 {
            sign *= metric(k);
        }
        k += 1;
    }
    (result_idx, sign)
}

const fn build_index_table() -> [[u8; 16]; 16] {
    let mut table = [[0u8; 16]; 16];
    let mut i = 0;
    while i < 16 {
        let mut j = 0;
        while j < 16 {
            let (idx, _) = basis_product(i, j);
            table[i][j] = idx as u8;
            j += 1;
        }
        i += 1;
    }
    table
}

const fn build_sign_table() -> [[i8; 16]; 16] {
    let mut table = [[0i8; 16]; 16];
    let mut i = 0;
    while i < 16 {
        let mut j = 0;
        while j < 16 {
            let (_, sign) = basis_product(i, j);
            table[i][j] = sign;
            j += 1;
        }
        i += 1;
    }
    table
}

const fn build_grade_table() -> [u8; 16] {
    let mut grades = [0u8; 16];
    let mut i = 0;
    while i < 16 {
        let mask = BITMASK[i];
        let mut g = 0u8;
        let mut k = 0;
        while k < 4 {
            if (mask >> k) & 1 == 1 { g += 1; }
            k += 1;
        }
        grades[i] = g;
        i += 1;
    }
    grades
}

// The tables — computed at compile time, zero runtime cost
const PRODUCT_INDEX: [[u8; 16]; 16] = build_index_table();
const PRODUCT_SIGN: [[i8; 16]; 16] = build_sign_table();
const GRADE: [u8; 16] = build_grade_table();

// ──────────────────────────────────────────────────────────────────
// Cl13 multivector type
// ──────────────────────────────────────────────────────────────────

/// A multivector in Cl(1,3) — 16 real components over the full Clifford algebra.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C, align(64))]
pub struct Cl13 {
    pub data: [f64; 16],
}

impl Cl13 {
    pub const ZERO: Self = Cl13 { data: [0.0; 16] };
    pub const ONE: Self = Cl13 {
        data: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
               0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    #[inline(always)]
    pub const fn scalar(s: f64) -> Self {
        let mut d = [0.0; 16];
        d[0] = s;
        Cl13 { data: d }
    }

    #[inline(always)]
    pub fn vector(v: [f64; 4]) -> Self {
        let mut d = [0.0; 16];
        d[1] = v[0]; d[2] = v[1]; d[3] = v[2]; d[4] = v[3];
        Cl13 { data: d }
    }

    #[inline(always)]
    pub fn bivector(b: [f64; 6]) -> Self {
        let mut d = [0.0; 16];
        d[5] = b[0]; d[6] = b[1]; d[7] = b[2];
        d[8] = b[3]; d[9] = b[4]; d[10] = b[5];
        Cl13 { data: d }
    }

    #[inline(always)]
    pub fn basis(idx: usize) -> Self {
        assert!(idx < 16, "Cl(1,3) has 16 basis elements");
        let mut d = [0.0; 16];
        d[idx] = 1.0;
        Cl13 { data: d }
    }

    // ── Grade extraction ─────────────────────────────────────────

    #[inline]
    pub fn grade_project(&self, k: u8) -> Self {
        let mut result = [0.0; 16];
        for i in 0..16 {
            if GRADE[i] == k { result[i] = self.data[i]; }
        }
        Cl13 { data: result }
    }

    #[inline(always)]
    pub fn scalar_part(&self) -> f64 { self.data[0] }

    #[inline(always)]
    pub fn pseudoscalar_part(&self) -> f64 { self.data[15] }

    // ── Geometric product ────────────────────────────────────────

    /// The geometric product — the fundamental operation of Clifford algebra.
    /// Computed using compile-time tables. EXACT per universal property of Cl(1,3).
    #[inline]
    pub fn geometric_product(&self, rhs: &Cl13) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            if self.data[i] == 0.0 { continue; }
            for j in 0..16 {
                if rhs.data[j] == 0.0 { continue; }
                let target = PRODUCT_INDEX[i][j] as usize;
                let sign = PRODUCT_SIGN[i][j] as f64;
                result[target] += sign * self.data[i] * rhs.data[j];
            }
        }
        Cl13 { data: result }
    }

    /// Inner product (grade-lowering).
    pub fn inner_product(&self, rhs: &Cl13) -> Cl13 {
        let mut result = Cl13::ZERO;
        for k in 0..=4u8 {
            let a_k = self.grade_project(k);
            for l in 0..=4u8 {
                let b_l = rhs.grade_project(l);
                if k == 0 || l == 0 { continue; }
                let target_grade = (k as i8 - l as i8).unsigned_abs();
                let prod = a_k.geometric_product(&b_l);
                let projected = prod.grade_project(target_grade);
                for i in 0..16 { result.data[i] += projected.data[i]; }
            }
        }
        result
    }

    /// Outer (wedge) product (grade-raising).
    pub fn outer_product(&self, rhs: &Cl13) -> Cl13 {
        let mut result = Cl13::ZERO;
        for k in 0..=4u8 {
            let a_k = self.grade_project(k);
            for l in 0..=4u8 {
                if k + l > 4 { continue; }
                let b_l = rhs.grade_project(l);
                let prod = a_k.geometric_product(&b_l);
                let projected = prod.grade_project(k + l);
                for i in 0..16 { result.data[i] += projected.data[i]; }
            }
        }
        result
    }

    /// Reverse: (−1)^{k(k−1)/2} for grade-k parts.
    pub fn reverse(&self) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            let k = GRADE[i] as i32;
            let sign = if (k * (k - 1) / 2) % 2 == 0 { 1.0 } else { -1.0 };
            result[i] = sign * self.data[i];
        }
        Cl13 { data: result }
    }

    /// Grade involution.
    pub fn involute(&self) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            let sign = if GRADE[i] % 2 == 0 { 1.0 } else { -1.0 };
            result[i] = sign * self.data[i];
        }
        Cl13 { data: result }
    }

    /// Clifford conjugate: reverse ∘ involution.
    pub fn conjugate(&self) -> Cl13 { self.reverse().involute() }

    #[inline]
    pub fn norm_squared(&self) -> f64 {
        self.geometric_product(&self.reverse()).scalar_part()
    }

    #[inline]
    pub fn norm(&self) -> f64 { self.norm_squared().abs().sqrt() }

    pub fn inverse(&self) -> Option<Cl13> {
        let ns = self.norm_squared();
        if ns.abs() < 1e-12 { return None; }
        Some(self.reverse() * (1.0 / ns))
    }

    pub fn coefficient_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn is_zero(&self, eps: f64) -> bool {
        self.data.iter().all(|x| x.abs() < eps)
    }

    pub fn approx_eq(&self, other: &Cl13, eps: f64) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| (a - b).abs() < eps)
    }
}

// ── Operator overloads ──────────────────────────────────────────

impl Mul for Cl13 {
    type Output = Cl13;
    #[inline]
    fn mul(self, rhs: Cl13) -> Cl13 { self.geometric_product(&rhs) }
}

impl Mul<f64> for Cl13 {
    type Output = Cl13;
    #[inline]
    fn mul(self, rhs: f64) -> Cl13 {
        Cl13 { data: self.data.map(|x| x * rhs) }
    }
}

impl Add for Cl13 {
    type Output = Cl13;
    #[inline]
    fn add(self, rhs: Cl13) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 { result[i] = self.data[i] + rhs.data[i]; }
        Cl13 { data: result }
    }
}

impl Sub for Cl13 {
    type Output = Cl13;
    #[inline]
    fn sub(self, rhs: Cl13) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 { result[i] = self.data[i] - rhs.data[i]; }
        Cl13 { data: result }
    }
}

impl Neg for Cl13 {
    type Output = Cl13;
    #[inline]
    fn neg(self) -> Cl13 { Cl13 { data: self.data.map(|x| -x) } }
}

// ── Named basis element constructors ────────────────────────────

impl Cl13 {
    pub fn e0() -> Self { Self::basis(1) }
    pub fn e1() -> Self { Self::basis(2) }
    pub fn e2() -> Self { Self::basis(3) }
    pub fn e3() -> Self { Self::basis(4) }
    pub fn e01() -> Self { Self::basis(5) }
    pub fn e02() -> Self { Self::basis(6) }
    pub fn e03() -> Self { Self::basis(7) }
    pub fn e12() -> Self { Self::basis(8) }
    pub fn e13() -> Self { Self::basis(9) }
    pub fn e23() -> Self { Self::basis(10) }
    pub fn e012() -> Self { Self::basis(11) }
    pub fn e013() -> Self { Self::basis(12) }
    pub fn e023() -> Self { Self::basis(13) }
    pub fn e123() -> Self { Self::basis(14) }
    pub fn e0123() -> Self { Self::basis(15) }
}

/// Public access to compile-time tables for verification.
pub mod tables {
    pub const PRODUCT_INDEX: &[[u8; 16]; 16] = &super::PRODUCT_INDEX;
    pub const PRODUCT_SIGN: &[[i8; 16]; 16] = &super::PRODUCT_SIGN;
    pub const GRADE: &[u8; 16] = &super::GRADE;
    pub const BITMASK: &[u8; 16] = &super::BITMASK;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_signature() {
        assert_eq!((Cl13::e0() * Cl13::e0()).scalar_part(), 1.0);
        assert_eq!((Cl13::e1() * Cl13::e1()).scalar_part(), -1.0);
        assert_eq!((Cl13::e2() * Cl13::e2()).scalar_part(), -1.0);
        assert_eq!((Cl13::e3() * Cl13::e3()).scalar_part(), -1.0);
    }

    #[test]
    fn test_anticommutativity() {
        for i in 1..=4 {
            for j in (i + 1)..=4 {
                let ei = Cl13::basis(i);
                let ej = Cl13::basis(j);
                assert!((ei * ej + ej * ei).is_zero(1e-12));
            }
        }
    }

    #[test]
    fn test_pseudoscalar_square() {
        let ps = Cl13::e0123();
        assert!((ps * ps).scalar_part() - (-1.0) < 1e-12);
    }

    #[test]
    fn test_full_associativity() {
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let ei = Cl13::basis(i);
                    let ej = Cl13::basis(j);
                    let ek = Cl13::basis(k);
                    assert!(
                        ((ei * ej) * ek).approx_eq(&(ei * (ej * ek)), 1e-10),
                        "Associativity failed: ({} * {}) * {} ≠ {} * ({} * {})",
                        i, j, k, i, j, k
                    );
                }
            }
        }
    }

    #[test]
    fn test_reverse() {
        assert_eq!(Cl13::scalar(5.0).reverse().scalar_part(), 5.0);
        assert!(Cl13::e1().reverse().approx_eq(&Cl13::e1(), 1e-12));
        assert!(Cl13::e12().reverse().approx_eq(&(-Cl13::e12()), 1e-12));
        assert!(Cl13::e0123().reverse().approx_eq(&Cl13::e0123(), 1e-12));
    }
}
