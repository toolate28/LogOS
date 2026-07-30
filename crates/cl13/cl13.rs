//! # Cl(1,3) Clifford Algebra — The Bedrock
//!
//! Full 16-dimensional Clifford algebra over spacetime signature (+,-,-,-)
//! with the geometric product table derived at compile time from first principles.
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
/// Index i maps to the bitmask encoding which generators appear.
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

/// Reverse lookup: given a bitmask, return the linear index.
const fn bitmask_to_index(mask: u8) -> usize {
    let mut i = 0;
    while i < 16 {
        if BITMASK[i] == mask { return i; }
        i += 1;
    }
    0 // unreachable for valid masks
}

/// Metric signature: e₀² = +1, e₁² = e₂² = e₃² = −1
const fn metric(generator: u8) -> i8 {
    if generator == 0 { 1 } else { -1 }
}

/// Count bubble-sort swaps needed to canonically order the product of two blades.
///
/// When we multiply eₐ · eᵦ, we conceptually concatenate their generator lists
/// and count how many adjacent transpositions are needed to sort them.
/// Each transposition picks up a factor of −1.
const fn count_swaps(a_mask: u8, b_mask: u8) -> u32 {
    // For each generator j in b (scanning low to high),
    // count generators in a that are ABOVE j (would need to swap past j).
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

/// Compute the geometric product of two basis blades.
/// Returns (result_index, sign) where sign ∈ {−1, +1}.
const fn basis_product(a_idx: usize, b_idx: usize) -> (usize, i8) {
    let a = BITMASK[a_idx];
    let b = BITMASK[b_idx];

    // 1. Result blade bitmask = XOR (generators that don't cancel)
    let result_mask = a ^ b;
    let result_idx = bitmask_to_index(result_mask);

    // 2. Sign from reordering (bubble sort swaps)
    let swaps = count_swaps(a, b);
    let mut sign: i8 = if swaps % 2 == 0 { 1 } else { -1 };

    // 3. Sign from metric contraction of shared generators
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

/// The full 16×16 multiplication table: PRODUCT_INDEX[i][j] = result basis index
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

/// The full 16×16 sign table: PRODUCT_SIGN[i][j] = ±1
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

/// Grade of each basis element (number of generators)
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

// ──────────────────────────────────────────────────────────────────
// The tables — computed at compile time, zero runtime cost
// ──────────────────────────────────────────────────────────────────

/// PRODUCT_INDEX[i][j] = linear index of the basis blade resulting from eᵢ · eⱼ
const PRODUCT_INDEX: [[u8; 16]; 16] = build_index_table();

/// PRODUCT_SIGN[i][j] = sign (±1) of the product eᵢ · eⱼ
const PRODUCT_SIGN: [[i8; 16]; 16] = build_sign_table();

/// GRADE[i] = grade of basis element i
const GRADE: [u8; 16] = build_grade_table();

// ──────────────────────────────────────────────────────────────────
// Cl13 multivector type
// ──────────────────────────────────────────────────────────────────

/// A multivector in Cl(1,3) — 16 real components over the full Clifford algebra.
///
/// Components are stored in the basis order defined above:
/// `[1, e₀, e₁, e₂, e₃, e₀₁, e₀₂, e₀₃, e₁₂, e₁₃, e₂₃, e₀₁₂, e₀₁₃, e₀₂₃, e₁₂₃, e₀₁₂₃]`
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C, align(64))] // cache-line aligned for SIMD friendliness
pub struct Cl13 {
    pub data: [f64; 16],
}

impl Cl13 {
    pub const ZERO: Self = Cl13 { data: [0.0; 16] };

    pub const ONE: Self = Cl13 {
        data: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
               0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    /// Construct a pure scalar multivector.
    #[inline(always)]
    pub const fn scalar(s: f64) -> Self {
        let mut d = [0.0; 16];
        d[0] = s;
        Cl13 { data: d }
    }

    /// Construct a grade-1 vector from 4 components [e₀, e₁, e₂, e₃].
    #[inline(always)]
    pub fn vector(v: [f64; 4]) -> Self {
        let mut d = [0.0; 16];
        d[1] = v[0]; // e₀
        d[2] = v[1]; // e₁
        d[3] = v[2]; // e₂
        d[4] = v[3]; // e₃
        Cl13 { data: d }
    }

    /// Construct a grade-2 bivector from 6 components [e₀₁, e₀₂, e₀₃, e₁₂, e₁₃, e₂₃].
    #[inline(always)]
    pub fn bivector(b: [f64; 6]) -> Self {
        let mut d = [0.0; 16];
        d[5]  = b[0]; // e₀₁
        d[6]  = b[1]; // e₀₂
        d[7]  = b[2]; // e₀₃
        d[8]  = b[3]; // e₁₂
        d[9]  = b[4]; // e₁₃
        d[10] = b[5]; // e₂₃
        Cl13 { data: d }
    }

    /// Construct a unit basis blade at the given index.
    #[inline(always)]
    pub fn basis(idx: usize) -> Self {
        assert!(idx < 16, "Cl(1,3) has 16 basis elements");
        let mut d = [0.0; 16];
        d[idx] = 1.0;
        Cl13 { data: d }
    }

    // ── Grade extraction ─────────────────────────────────────────

    /// Extract the grade-k part of this multivector.
    #[inline]
    pub fn grade_project(&self, k: u8) -> Self {
        let mut result = [0.0; 16];
        for i in 0..16 {
            if GRADE[i] == k {
                result[i] = self.data[i];
            }
        }
        Cl13 { data: result }
    }

    /// Extract the scalar (grade-0) part.
    #[inline(always)]
    pub fn scalar_part(&self) -> f64 {
        self.data[0]
    }

    /// Extract the pseudoscalar (grade-4) part.
    #[inline(always)]
    pub fn pseudoscalar_part(&self) -> f64 {
        self.data[15]
    }

    /// Grade of the highest non-zero component.
    pub fn top_grade(&self) -> Option<u8> {
        for k in (0..=4u8).rev() {
            for i in 0..16 {
                if GRADE[i] == k && self.data[i].abs() > f64::EPSILON {
                    return Some(k);
                }
            }
        }
        None
    }

    // ── Geometric product ────────────────────────────────────────

    /// The geometric product — the fundamental operation of Clifford algebra.
    ///
    /// Computed using the compile-time tables. This is the EXACT product
    /// determined by the universal property of Cl(1,3) with signature (+,−,−,−).
    #[inline]
    pub fn geometric_product(&self, rhs: &Cl13) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            if self.data[i] == 0.0 { continue; } // skip zeros for speed
            for j in 0..16 {
                if rhs.data[j] == 0.0 { continue; }
                let target = PRODUCT_INDEX[i][j] as usize;
                let sign = PRODUCT_SIGN[i][j] as f64;
                result[target] += sign * self.data[i] * rhs.data[j];
            }
        }
        Cl13 { data: result }
    }

    // ── Derived products ─────────────────────────────────────────

    /// Inner product (grade-lowering): ⟨A⟩_k · ⟨B⟩_l = ⟨AB⟩_{|k−l|}
    pub fn inner_product(&self, rhs: &Cl13) -> Cl13 {
        let full = self.geometric_product(rhs);
        // For general multivectors, sum over all grade pairs
        let mut result = Cl13::ZERO;
        for k in 0..=4u8 {
            let a_k = self.grade_project(k);
            for l in 0..=4u8 {
                let b_l = rhs.grade_project(l);
                if k == 0 || l == 0 { continue; } // inner product with scalar is zero
                let target_grade = (k as i8 - l as i8).unsigned_abs();
                let prod = a_k.geometric_product(&b_l);
                let projected = prod.grade_project(target_grade);
                for i in 0..16 {
                    result.data[i] += projected.data[i];
                }
            }
        }
        result
    }

    /// Outer (wedge) product (grade-raising): ⟨A⟩_k ∧ ⟨B⟩_l = ⟨AB⟩_{k+l}
    pub fn outer_product(&self, rhs: &Cl13) -> Cl13 {
        let mut result = Cl13::ZERO;
        for k in 0..=4u8 {
            let a_k = self.grade_project(k);
            for l in 0..=4u8 {
                if k + l > 4 { continue; }
                let b_l = rhs.grade_project(l);
                let prod = a_k.geometric_product(&b_l);
                let projected = prod.grade_project(k + l);
                for i in 0..16 {
                    result.data[i] += projected.data[i];
                }
            }
        }
        result
    }

    /// Reverse: reverses the order of generators in each blade.
    /// rev(e_{i₁…iₖ}) = (−1)^{k(k−1)/2} e_{i₁…iₖ}
    pub fn reverse(&self) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            let k = GRADE[i] as i32;
            let sign = if (k * (k - 1) / 2) % 2 == 0 { 1.0 } else { -1.0 };
            result[i] = sign * self.data[i];
        }
        Cl13 { data: result }
    }

    /// Grade involution: (−1)^k for grade-k parts.
    pub fn involute(&self) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 {
            let sign = if GRADE[i] % 2 == 0 { 1.0 } else { -1.0 };
            result[i] = sign * self.data[i];
        }
        Cl13 { data: result }
    }

    /// Clifford conjugate: reverse ∘ involution.
    pub fn conjugate(&self) -> Cl13 {
        self.reverse().involute()
    }

    /// Squared norm: A · rev(A) (scalar part).
    #[inline]
    pub fn norm_squared(&self) -> f64 {
        self.geometric_product(&self.reverse()).scalar_part()
    }

    /// Norm: √|A · rev(A)|
    #[inline]
    pub fn norm(&self) -> f64 {
        self.norm_squared().abs().sqrt()
    }

    /// Inverse (when it exists): rev(A) / (A · rev(A))
    pub fn inverse(&self) -> Option<Cl13> {
        let ns = self.norm_squared();
        if ns.abs() < 1e-12 { return None; }
        let rev = self.reverse();
        Some(rev * (1.0 / ns))
    }

    /// L2 norm of the coefficient vector (for convergence checks).
    pub fn coefficient_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Check if approximately zero.
    pub fn is_zero(&self, eps: f64) -> bool {
        self.data.iter().all(|x| x.abs() < eps)
    }

    /// Check if approximately equal to another multivector.
    pub fn approx_eq(&self, other: &Cl13, eps: f64) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| (a - b).abs() < eps)
    }
}

// ──────────────────────────────────────────────────────────────────
// Operator overloads
// ──────────────────────────────────────────────────────────────────

impl Mul for Cl13 {
    type Output = Cl13;
    #[inline]
    fn mul(self, rhs: Cl13) -> Cl13 {
        self.geometric_product(&rhs)
    }
}

/// Scalar multiplication: Cl13 * f64
impl Mul<f64> for Cl13 {
    type Output = Cl13;
    #[inline]
    fn mul(self, rhs: f64) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 { result[i] = self.data[i] * rhs; }
        Cl13 { data: result }
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
    fn neg(self) -> Cl13 {
        let mut result = [0.0; 16];
        for i in 0..16 { result[i] = -self.data[i]; }
        Cl13 { data: result }
    }
}

// ──────────────────────────────────────────────────────────────────
// Named basis element constructors (convenience)
// ──────────────────────────────────────────────────────────────────

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

// ──────────────────────────────────────────────────────────────────
// Expose the tables for verification
// ──────────────────────────────────────────────────────────────────

/// Public access to the compile-time multiplication tables for verification crate.
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
        // e₀² = +1 (timelike)
        let e0 = Cl13::e0();
        assert_eq!((e0 * e0).scalar_part(), 1.0);

        // e₁² = −1 (spacelike)
        let e1 = Cl13::e1();
        assert_eq!((e1 * e1).scalar_part(), -1.0);

        // e₂² = −1
        let e2 = Cl13::e2();
        assert_eq!((e2 * e2).scalar_part(), -1.0);

        // e₃² = −1
        let e3 = Cl13::e3();
        assert_eq!((e3 * e3).scalar_part(), -1.0);
    }

    #[test]
    fn test_anticommutativity() {
        // eᵢeⱼ = −eⱼeᵢ for i ≠ j
        for i in 1..=4 {
            for j in (i + 1)..=4 {
                let ei = Cl13::basis(i);
                let ej = Cl13::basis(j);
                let ij = ei * ej;
                let ji = ej * ei;
                assert!(
                    (ij + ji).is_zero(1e-12),
                    "Anticommutativity failed for basis({}) * basis({})", i, j
                );
            }
        }
    }

    #[test]
    fn test_scalar_identity() {
        // 1 · A = A · 1 = A
        let a = Cl13 { data: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                               9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] };
        assert!(a.approx_eq(&(Cl13::ONE * a), 1e-12));
        assert!(a.approx_eq(&(a * Cl13::ONE), 1e-12));
    }

    #[test]
    fn test_pseudoscalar_square() {
        // e₀₁₂₃² = e₀²·e₁²·e₂²·e₃² × (−1)^(4·3/2) = (+1)(−1)(−1)(−1) × (−1)^6
        // = (−1) × 1 = −1
        let ps = Cl13::e0123();
        let ps_sq = ps * ps;
        assert!(
            (ps_sq.scalar_part() - (-1.0)).abs() < 1e-12,
            "e₀₁₂₃² should be −1, got {}", ps_sq.scalar_part()
        );
    }

    #[test]
    fn test_bivector_products() {
        // e₀₁ = e₀ · e₁
        let e01_direct = Cl13::e0() * Cl13::e1();
        assert!(e01_direct.approx_eq(&Cl13::e01(), 1e-12));

        // e₁₂ = e₁ · e₂
        let e12_direct = Cl13::e1() * Cl13::e2();
        assert!(e12_direct.approx_eq(&Cl13::e12(), 1e-12));
    }

    #[test]
    fn test_reverse() {
        // rev(scalar) = scalar
        assert_eq!(Cl13::scalar(5.0).reverse().scalar_part(), 5.0);

        // rev(vector) = vector (grade 1: k(k−1)/2 = 0, sign = +1)
        let v = Cl13::e1();
        assert!(v.reverse().approx_eq(&v, 1e-12));

        // rev(bivector) = −bivector (grade 2: k(k−1)/2 = 1, sign = −1)
        let b = Cl13::e12();
        assert!(b.reverse().approx_eq(&(-b), 1e-12));

        // rev(trivector) = −trivector (grade 3: k(k−1)/2 = 3, sign = −1)
        let t = Cl13::e123();
        assert!(t.reverse().approx_eq(&(-t), 1e-12));

        // rev(pseudoscalar) = pseudoscalar (grade 4: k(k−1)/2 = 6, sign = +1)
        let ps = Cl13::e0123();
        assert!(ps.reverse().approx_eq(&ps, 1e-12));
    }

    #[test]
    fn test_associativity_basis() {
        // (eᵢ · eⱼ) · eₖ = eᵢ · (eⱼ · eₖ) for all basis triples
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let ei = Cl13::basis(i);
                    let ej = Cl13::basis(j);
                    let ek = Cl13::basis(k);
                    let left = (ei * ej) * ek;
                    let right = ei * (ej * ek);
                    assert!(
                        left.approx_eq(&right, 1e-10),
                        "Associativity failed: ({} * {}) * {} ≠ {} * ({} * {})",
                        i, j, k, i, j, k
                    );
                }
            }
        }
    }
}
