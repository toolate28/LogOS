//! cqk-anyon — Fibonacci Anyon TQFT: F/R/S Matrices and Jones Polynomial
//!
//! Implements the SU(2)₃ Fibonacci anyon model for BQP-complete
//! cryptographic anchoring of DNS simulation states.
//!
//! The fusion rule: τ ⊗ τ = 1 ⊕ τ
//! Quantum dimension: d_τ = φ ≈ 1.618 (golden ratio)
//!
//! Three matrix generators:
//!   F — recoupling (Mac Lane pentagon), spatial basis change
//!   R — braiding, counter-clockwise exchange phase
//!   S — modular conjugation, toroidal basis relation
//!
//! Braid group B_n generators: σ₁ = R, σ₂ = F·R·F
//! Yang-Baxter: σ₁σ₂σ₁ = σ₂σ₁σ₂
//!
//! References:
//!   S4: LogOS_qLDPC_Architecture.pdf (qLDPC anchoring)
//!   S6: Crystalline_Fibonacci_Blueprints.pdf
//!   Brief §: Cryptographic Anchoring via Anyonic Braiding

use num_complex::Complex64;
use reson8_core::fibonacci::PHI;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// ── Constants ───────────────────────────────────────────────────────

/// Quantum dimension of the τ particle.
pub const D_TAU: f64 = PHI; // 1.618033988749895

/// Total quantum dimension: D = √(2 + φ)
///
/// CRITICAL: D² = 2 + φ ≈ 3.618, NOT φ² ≈ 2.618.
/// The derivation: D² = 1² + d_τ² = 1 + φ² = 1 + (φ+1) = 2 + φ.
/// (Bug caught by Gemini during integration — the distinction matters
/// for Jones polynomial normalization and S-matrix unitarity.)
pub const D_SQUARED: f64 = 2.0 + PHI; // ≈ 3.618033988749895

pub fn total_quantum_dim() -> f64 {
    D_SQUARED.sqrt()
}

// ── F-Matrix (Recoupling) ───────────────────────────────────────────

/// The F-matrix: spatial recoupling gate.
/// Solves the Mac Lane Pentagon equation.
///
///   F = | φ⁻¹     φ^{-1/2} |
///       | φ^{-1/2}  -φ⁻¹   |
pub fn f_matrix() -> [[Complex64; 2]; 2] {
    let phi_inv = 1.0 / PHI;
    let phi_inv_sqrt = phi_inv.sqrt();
    [
        [Complex64::new(phi_inv, 0.0), Complex64::new(phi_inv_sqrt, 0.0)],
        [Complex64::new(phi_inv_sqrt, 0.0), Complex64::new(-phi_inv, 0.0)],
    ]
}

// ── R-Matrix (Braiding) ─────────────────────────────────────────────

/// The R-matrix: diagonal braiding phase.
/// Assigns fractional statistical phase for counter-clockwise exchange.
///
///   R = | e^{4πi/5}    0         |
///       |    0       e^{-3πi/5}  |
pub fn r_matrix() -> [[Complex64; 2]; 2] {
    let r11 = Complex64::from_polar(1.0, 4.0 * PI / 5.0);
    let r22 = Complex64::from_polar(1.0, -3.0 * PI / 5.0);
    [
        [r11, Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), r22],
    ]
}

// ── S-Matrix (Modular Conjugation) ──────────────────────────────────

/// The S-matrix: modular conjugation for toroidal manifold bases.
/// Scaled by the total quantum dimension D.
///
///   S = (1/D) | 1    φ  |
///             | φ   -1  |
pub fn s_matrix() -> [[Complex64; 2]; 2] {
    let d = total_quantum_dim();
    let d_inv = 1.0 / d;
    [
        [Complex64::new(d_inv, 0.0), Complex64::new(d_inv * PHI, 0.0)],
        [Complex64::new(d_inv * PHI, 0.0), Complex64::new(-d_inv, 0.0)],
    ]
}

// ── Matrix Operations ───────────────────────────────────────────────

/// Multiply two 2×2 complex matrices.
pub fn mat_mul(a: &[[Complex64; 2]; 2], b: &[[Complex64; 2]; 2]) -> [[Complex64; 2]; 2] {
    let zero = Complex64::new(0.0, 0.0);
    let mut c = [[zero; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    c
}

/// Trace of a 2×2 complex matrix.
pub fn mat_trace(m: &[[Complex64; 2]; 2]) -> Complex64 {
    m[0][0] + m[1][1]
}

// ── Braid Group Generators ──────────────────────────────────────────

/// σ₁ = R (primary braid generator)
pub fn sigma_1() -> [[Complex64; 2]; 2] {
    r_matrix()
}

/// σ₂ = F · R · F (secondary braid generator, F-conjugated)
pub fn sigma_2() -> [[Complex64; 2]; 2] {
    let f = f_matrix();
    let r = r_matrix();
    let fr = mat_mul(&f, &r);
    mat_mul(&fr, &f)
}

/// σ₁⁻¹ (inverse braiding — clockwise exchange)
pub fn sigma_1_inv() -> [[Complex64; 2]; 2] {
    // R⁻¹ = R† (R is unitary)
    let r = r_matrix();
    [
        [r[0][0].conj(), r[1][0].conj()],
        [r[0][1].conj(), r[1][1].conj()],
    ]
}

/// σ₂⁻¹
pub fn sigma_2_inv() -> [[Complex64; 2]; 2] {
    let s2 = sigma_2();
    [
        [s2[0][0].conj(), s2[1][0].conj()],
        [s2[0][1].conj(), s2[1][1].conj()],
    ]
}

// ── Braid Word ──────────────────────────────────────────────────────

/// A braid word: sequence of generator indices with signs.
/// Positive index = σ_i, negative = σ_i⁻¹.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraidWord {
    /// Sequence of (generator_index, positive=true/inverse=false).
    pub letters: Vec<(usize, bool)>,
}

impl BraidWord {
    pub fn new(letters: Vec<(usize, bool)>) -> Self {
        Self { letters }
    }

    /// Compute the Kauffman writhe: sum of ±1 for each crossing.
    pub fn writhe(&self) -> i64 {
        self.letters.iter().map(|(_, pos)| if *pos { 1i64 } else { -1 }).sum()
    }

    /// Evaluate the braid-group representation matrix for this word.
    pub fn evaluate(&self) -> [[Complex64; 2]; 2] {
        let s1 = sigma_1();
        let s2 = sigma_2();
        let s1_inv = sigma_1_inv();
        let s2_inv = sigma_2_inv();
        let identity = [
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        ];

        let mut result = identity;
        for &(gen, positive) in &self.letters {
            let m = match (gen, positive) {
                (1, true) => &s1,
                (1, false) => &s1_inv,
                (2, true) => &s2,
                (2, false) => &s2_inv,
                _ => &identity,
            };
            result = mat_mul(&result, m);
        }
        result
    }
}

// ── Jones Polynomial (Markov Trace) ─────────────────────────────────

/// Evaluate the Jones polynomial V(t) for a braid word via the
/// Markov trace of the Temperley-Lieb representation.
///
/// The trace is normalized by the quantum dimension d_τ.
pub fn jones_trace(braid: &BraidWord) -> Complex64 {
    let rep = braid.evaluate();
    let trace = mat_trace(&rep);
    trace / Complex64::new(D_TAU, 0.0)
}

/// Topological state classification for fluid anomaly anchoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalState {
    pub name: String,
    pub braid: BraidWord,
    pub writhe: i64,
    pub jones_trace: (f64, f64), // (real, imag) of evaluated trace
}

impl TopologicalState {
    pub fn evaluate(name: &str, braid: BraidWord) -> Self {
        let writhe = braid.writhe();
        let jt = jones_trace(&braid);
        Self {
            name: name.to_string(),
            braid,
            writhe,
            jones_trace: (jt.re, jt.im),
        }
    }
}

// ── Yang-Baxter Verification ────────────────────────────────────────

/// Verify the Yang-Baxter hexagon equation: σ₁σ₂σ₁ = σ₂σ₁σ₂
/// Returns the max element-wise deviation (should be < 1e-12).
pub fn verify_yang_baxter() -> f64 {
    let s1 = sigma_1();
    let s2 = sigma_2();

    // LHS: σ₁ · σ₂ · σ₁
    let lhs = mat_mul(&mat_mul(&s1, &s2), &s1);
    // RHS: σ₂ · σ₁ · σ₂
    let rhs = mat_mul(&mat_mul(&s2, &s1), &s2);

    let mut max_dev = 0.0f64;
    for i in 0..2 {
        for j in 0..2 {
            let diff = (lhs[i][j] - rhs[i][j]).norm();
            max_dev = max_dev.max(diff);
        }
    }
    max_dev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_matrix_is_unitary() {
        let f = f_matrix();
        let fh = [
            [f[0][0].conj(), f[1][0].conj()],
            [f[0][1].conj(), f[1][1].conj()],
        ];
        let product = mat_mul(&f, &fh);
        // Should be identity
        assert!((product[0][0].re - 1.0).abs() < 1e-10);
        assert!((product[1][1].re - 1.0).abs() < 1e-10);
        assert!(product[0][1].norm() < 1e-10);
        assert!(product[1][0].norm() < 1e-10);
    }

    #[test]
    fn r_matrix_is_unitary() {
        let r = r_matrix();
        // Diagonal matrix with unit-modulus entries → unitary
        assert!((r[0][0].norm() - 1.0).abs() < 1e-12);
        assert!((r[1][1].norm() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn yang_baxter_holds() {
        let dev = verify_yang_baxter();
        assert!(
            dev < 1e-10,
            "Yang-Baxter violated: max deviation = {:.3e}",
            dev
        );
    }

    #[test]
    fn total_quantum_dim_correct() {
        let d = total_quantum_dim();
        let expected = (2.0 + PHI).sqrt();
        assert!((d - expected).abs() < 1e-12);
    }

    #[test]
    fn gauge_closure_braid() {
        // Gauge closure: σ₁σ₂⁻¹σ₁σ₂⁻¹
        let braid = BraidWord::new(vec![(1, true), (2, false), (1, true), (2, false)]);
        let state = TopologicalState::evaluate("gauge_closure", braid);
        assert_eq!(state.writhe, 0);
    }

    #[test]
    fn writhe_computation() {
        let braid = BraidWord::new(vec![(1, true), (1, true), (2, false)]);
        assert_eq!(braid.writhe(), 1); // +1 +1 -1 = 1
    }
}
