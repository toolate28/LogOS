//! Non-Abelian Fibonacci Anyon Braiding — provenance verification for code.
//!
//! # Theory
//!
//! Fibonacci anyons are quasiparticles with two types: **1** (vacuum) and **τ** (tau).
//! Their fusion rules are:
//!   - 1 ⊗ 1 = 1
//!   - 1 ⊗ τ = τ
//!   - τ ⊗ τ = 1 ⊕ τ  (non-Abelian: two possible outcomes)
//!
//! Braiding two τ anyons produces a *topological* phase that depends ONLY on
//! the topology of the braid — not on local perturbations. This makes braids
//! inherently robust against noise and tampering.
//!
//! # Application: Farm-to-Table Code Provenance
//!
//! Every code artifact entering or leaving the Reson8 marketplace gets a
//! **braid signature**:
//!
//! 1. Hash the code content → SHA-256 seed
//! 2. Convert seed bits into a sequence of braid generators (σ₁, σ₁⁻¹, σ₂, σ₂⁻¹)
//! 3. Multiply the corresponding 2×2 matrices in the Fibonacci representation
//! 4. The resulting matrix IS the signature — topologically invariant
//! 5. To verify: re-braid the code, compare matrices
//!
//! If the code was modified, the braid matrix changes. If the braid was
//! re-ordered (supply chain attack), the non-Abelian property means the
//! result differs. This is *topological code signing*.

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// The golden ratio — fundamental to Fibonacci anyons.
const PHI: f64 = 1.618_033_988_749_895;

/// φ⁻¹ = 1/φ = φ - 1
const PHI_INV: f64 = 0.618_033_988_749_894_9;

// ---------------------------------------------------------------------------
// 2×2 complex matrix (minimal, no external dep)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct C64 {
    re: f64,
    im: f64,
}

impl C64 {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };
    const ONE: Self = Self { re: 1.0, im: 0.0 };

    fn new(re: f64, im: f64) -> Self { Self { re, im } }

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self { re: self.re + rhs.re, im: self.im + rhs.im }
    }

    fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

/// 2×2 matrix over ℂ.
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: C64, b: C64,
    c: C64, d: C64,
}

impl Mat2 {
    fn identity() -> Self {
        Self { a: C64::ONE, b: C64::ZERO, c: C64::ZERO, d: C64::ONE }
    }

    fn mul(self, rhs: Self) -> Self {
        Self {
            a: self.a.mul(rhs.a).add(self.b.mul(rhs.c)),
            b: self.a.mul(rhs.b).add(self.b.mul(rhs.d)),
            c: self.c.mul(rhs.a).add(self.d.mul(rhs.c)),
            d: self.c.mul(rhs.b).add(self.d.mul(rhs.d)),
        }
    }

    /// Frobenius distance between two matrices.
    fn distance(self, other: Self) -> f64 {
        let da = C64 { re: self.a.re - other.a.re, im: self.a.im - other.a.im };
        let db = C64 { re: self.b.re - other.b.re, im: self.b.im - other.b.im };
        let dc = C64 { re: self.c.re - other.c.re, im: self.c.im - other.c.im };
        let dd = C64 { re: self.d.re - other.d.re, im: self.d.im - other.d.im };
        (da.norm_sq() + db.norm_sq() + dc.norm_sq() + dd.norm_sq()).sqrt()
    }
}

// ---------------------------------------------------------------------------
// Fibonacci anyon braid generators
// ---------------------------------------------------------------------------

/// Braid generator σ₁ in the Fibonacci representation (2×2 unitary).
///
/// σ₁ = e^{-4πi/5} * |  φ⁻¹     φ^{-1/2} |
///                     | φ^{-1/2}  -φ⁻¹    |
fn sigma1() -> Mat2 {
    let phase = C64::new((-4.0 * PI / 5.0).cos(), (-4.0 * PI / 5.0).sin());
    let phi_inv_half = PHI_INV.sqrt();

    Mat2 {
        a: phase.mul(C64::new(PHI_INV, 0.0)),
        b: phase.mul(C64::new(phi_inv_half, 0.0)),
        c: phase.mul(C64::new(phi_inv_half, 0.0)),
        d: phase.mul(C64::new(-PHI_INV, 0.0)),
    }
}

/// σ₁⁻¹ — the inverse braid.
fn sigma1_inv() -> Mat2 {
    let phase = C64::new((4.0 * PI / 5.0).cos(), (4.0 * PI / 5.0).sin());
    let phi_inv_half = PHI_INV.sqrt();

    Mat2 {
        a: phase.mul(C64::new(PHI_INV, 0.0)),
        b: phase.mul(C64::new(phi_inv_half, 0.0)),
        c: phase.mul(C64::new(phi_inv_half, 0.0)),
        d: phase.mul(C64::new(-PHI_INV, 0.0)),
    }
}

/// σ₂ — second generator (different strand crossing).
fn sigma2() -> Mat2 {
    let phase = C64::new((-4.0 * PI / 5.0).cos(), (-4.0 * PI / 5.0).sin());
    let phi_inv_half = PHI_INV.sqrt();

    Mat2 {
        a: phase.mul(C64::new(-PHI_INV, 0.0)),
        b: phase.mul(C64::new(phi_inv_half, 0.0)),
        c: phase.mul(C64::new(phi_inv_half, 0.0)),
        d: phase.mul(C64::new(PHI_INV, 0.0)),
    }
}

/// σ₂⁻¹
fn sigma2_inv() -> Mat2 {
    let phase = C64::new((4.0 * PI / 5.0).cos(), (4.0 * PI / 5.0).sin());
    let phi_inv_half = PHI_INV.sqrt();

    Mat2 {
        a: phase.mul(C64::new(-PHI_INV, 0.0)),
        b: phase.mul(C64::new(phi_inv_half, 0.0)),
        c: phase.mul(C64::new(phi_inv_half, 0.0)),
        d: phase.mul(C64::new(PHI_INV, 0.0)),
    }
}

// ---------------------------------------------------------------------------
// Public API: BraidSignature
// ---------------------------------------------------------------------------

/// A braid word — sequence of generator indices.
/// 0 = σ₁, 1 = σ₁⁻¹, 2 = σ₂, 3 = σ₂⁻¹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraidWord(pub Vec<u8>);

/// The computed braid signature — stored as 8 f64 values (real+imag of 2×2 matrix).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraidSignature {
    /// The braid word used to generate this signature.
    pub word: BraidWord,
    /// Flattened 2×2 complex matrix: [a.re, a.im, b.re, b.im, c.re, c.im, d.re, d.im]
    pub matrix: [f64; 8],
    /// SHA-256 of the input content (hex).
    pub content_hash: String,
}

impl BraidSignature {
    /// Sign a byte slice using anyon braiding.
    ///
    /// 1. SHA-256 hash the content
    /// 2. Convert hash bits → braid generators (2 bits per generator)
    /// 3. Multiply generators → topological signature
    pub fn sign(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = hasher.finalize();
        let content_hash = hex::encode_upper(&hash);

        // Convert 32 bytes → 128 generator applications (2 bits each)
        let word = BraidWord(
            hash.iter()
                .flat_map(|byte| {
                    (0..4).map(move |i| (byte >> (6 - 2 * i)) & 0b11)
                })
                .collect(),
        );

        let matrix = Self::evaluate_braid(&word);

        Self {
            word,
            matrix: [
                matrix.a.re, matrix.a.im,
                matrix.b.re, matrix.b.im,
                matrix.c.re, matrix.c.im,
                matrix.d.re, matrix.d.im,
            ],
            content_hash,
        }
    }

    /// Verify that a braid signature matches the given content.
    pub fn verify(&self, content: &[u8]) -> BraidVerification {
        let fresh = Self::sign(content);
        let distance = self.frobenius_distance(&fresh);

        // Topological signatures should be exactly equal for identical content.
        // Allow tiny floating-point tolerance.
        let valid = distance < 1e-10;

        BraidVerification {
            valid,
            distance,
            original_hash: self.content_hash.clone(),
            computed_hash: fresh.content_hash,
        }
    }

    /// Frobenius distance between two braid signatures.
    pub fn frobenius_distance(&self, other: &BraidSignature) -> f64 {
        let a = self.to_mat2();
        let b = other.to_mat2();
        a.distance(b)
    }

    fn evaluate_braid(word: &BraidWord) -> Mat2 {
        let generators = [sigma1(), sigma1_inv(), sigma2(), sigma2_inv()];
        word.0.iter().fold(Mat2::identity(), |acc, &g| {
            acc.mul(generators[g as usize % 4])
        })
    }

    fn to_mat2(&self) -> Mat2 {
        Mat2 {
            a: C64::new(self.matrix[0], self.matrix[1]),
            b: C64::new(self.matrix[2], self.matrix[3]),
            c: C64::new(self.matrix[4], self.matrix[5]),
            d: C64::new(self.matrix[6], self.matrix[7]),
        }
    }
}

/// Result of a braid verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraidVerification {
    pub valid: bool,
    pub distance: f64,
    pub original_hash: String,
    pub computed_hash: String,
}

// We don't pull in the `hex` crate — inline it
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

    pub fn encode_upper(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            s.push(HEX_CHARS[(b >> 4) as usize] as char);
            s.push(HEX_CHARS[(b & 0xf) as usize] as char);
        }
        s
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_identical() {
        let code = b"fn main() { println!(\"hello anyon\"); }";
        let sig = BraidSignature::sign(code);
        let result = sig.verify(code);
        assert!(result.valid, "Signature should verify against same content");
        assert!(result.distance < 1e-10);
    }

    #[test]
    fn tampered_content_fails() {
        let code = b"fn main() { println!(\"hello anyon\"); }";
        let sig = BraidSignature::sign(code);

        let tampered = b"fn main() { println!(\"hello HACKED\"); }";
        let result = sig.verify(tampered);
        assert!(!result.valid, "Tampered content should fail verification");
        assert!(result.distance > 0.01);
    }

    #[test]
    fn non_abelian_property() {
        // σ₁ * σ₂ ≠ σ₂ * σ₁ — this is what makes it non-Abelian
        let s1s2 = sigma1().mul(sigma2());
        let s2s1 = sigma2().mul(sigma1());
        let dist = s1s2.distance(s2s1);
        assert!(dist > 0.01, "Braids must be non-commutative: distance = {}", dist);
    }

    #[test]
    fn braid_word_deterministic() {
        let code = b"deterministic test content";
        let sig1 = BraidSignature::sign(code);
        let sig2 = BraidSignature::sign(code);
        assert_eq!(sig1.content_hash, sig2.content_hash);
        assert!(sig1.frobenius_distance(&sig2) < 1e-15);
    }
}
