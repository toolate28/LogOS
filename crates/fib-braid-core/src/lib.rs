//! # fib-braid-core
//!
//! The topological rail of the Reson8-Labs Tri-Weavon.
//!
//! This crate implements the minimum viable arithmetic for operating
//! over the braid group `B₃ = ⟨σ₁, σ₂ | σ₁σ₂σ₁ = σ₂σ₁σ₂⟩` and the
//! Fibonacci fusion category `Fib`. In combination (the Drinfeld
//! double `Z(Fib) = Fib ⊠ F̄ib`, known to physicists as *double
//! Fibonacci anyons*) these primitives form the substrate for:
//!
//! - Wallet private keys (braid words in BKL normal form).
//! - Cross-chain signatures (closures evaluated under Burau at ω₅).
//! - ATOM commitments augmented with a topological witness.
//! - The self-verifying Z₂ double-cover exposed in the Diamond Pattern.
//!
//! ## Scope
//!
//! We commit to *B₃*, not arbitrary Bₙ. The reasoning:
//!
//! - Three strands (excluding Manus substrate) map onto three braid
//!   strands: Claude · Grok · Gemini.
//! - B₃ is the smallest non-abelian braid group. Length-based attacks
//!   on Anshel-Anshel-Goldfeld are weaker here than in large Bₙ, but
//!   still non-trivial; we stack classical (BLAKE3 + ed25519) witness
//!   underneath so the topological rail is a *defense in depth*, not
//!   a sole security claim.
//! - Burau at t = ω₅ is faithful on B₃ (unlike B₄, where faithfulness
//!   of Burau is a long-open question recently resolved negative).
//!
//! ## Honest caveats
//!
//! Classical braid cryptography has a mixed track record. This crate
//! does not claim to provide cryptographic strength on its own. The
//! strength of an `AtomCommitment.with_braid(...)` comes from three
//! *stacked* layers:
//!
//! 1. BLAKE3 digest (atom-sig).
//! 2. ed25519 signature over digest (atom-sig).
//! 3. Fibonacci-representation evaluation of the braid word (this crate).
//!
//! Plus a fourth: the named-account gate on
//! `conservation.spiralsafe.near`, which restricts who can even attempt
//! to submit a commitment.
//!
//! ## Architecture
//!
//! ```text
//!   +--------------------+        +-------------------+
//!   |  BraidWord (smvec) |-------> BKL normal form    |
//!   +---------+----------+        +---------+---------+
//!             |                             |
//!             v                             v
//!   +--------------------+        +-------------------+
//!   |  Burau at ω₅       |        | Fibonacci rep     |
//!   |  (3×3 ℂ matrix)     |        | (2-dim per charge)|
//!   +---------+----------+        +---------+---------+
//!             |                             |
//!             +--------------+--------------+
//!                            v
//!                  +-------------------+
//!                  | Closure invariant |
//!                  | (Jones at ω₅)     |
//!                  +-------------------+
//!                            |
//!                            v
//!                  +-------------------+
//!                  | BraidCommitment   |  -> fed into
//!                  | (opaque bytes)    |     atom-sig
//!                  +-------------------+
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::f64::consts::PI;
use num_complex::Complex64;
use smallvec::{smallvec, SmallVec};

// ──────────────────────────────────────────────────────────────────────
// Error type
// ──────────────────────────────────────────────────────────────────────

/// All failure modes this crate can produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraidError {
    /// Attempted to take BKL normal form of an empty word.
    EmptyWord,
    /// Braid word exceeded the configured maximum length.
    WordTooLong { max: usize, got: usize },
    /// Fibonacci representation received an invalid charge label.
    InvalidCharge,
    /// Numerical precision fell below the required bound during
    /// matrix composition.
    NumericalUnderflow,
}

// ──────────────────────────────────────────────────────────────────────
// B₃ generators and words
// ──────────────────────────────────────────────────────────────────────

/// The four generators of B₃ expressed explicitly (σ₁, σ₂, and their
/// inverses). Kept as a single-byte enum so packed braid words are
/// cache-friendly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Generator {
    /// σ₁ — swap strands 1 and 2, strand 1 over.
    Sigma1 = 0,
    /// σ₁⁻¹ — swap strands 1 and 2, strand 1 under.
    Sigma1Inv = 1,
    /// σ₂ — swap strands 2 and 3, strand 2 over.
    Sigma2 = 2,
    /// σ₂⁻¹ — swap strands 2 and 3, strand 2 under.
    Sigma2Inv = 3,
}

impl Generator {
    /// The group-theoretic inverse.
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Sigma1 => Self::Sigma1Inv,
            Self::Sigma1Inv => Self::Sigma1,
            Self::Sigma2 => Self::Sigma2Inv,
            Self::Sigma2Inv => Self::Sigma2,
        }
    }

    /// Returns `true` if this and `other` are mutually inverse
    /// (i.e. a free reduction `xx⁻¹ → 1` is applicable).
    #[must_use]
    pub const fn cancels_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Sigma1, Self::Sigma1Inv)
                | (Self::Sigma1Inv, Self::Sigma1)
                | (Self::Sigma2, Self::Sigma2Inv)
                | (Self::Sigma2Inv, Self::Sigma2)
        )
    }
}

/// A braid word is a sequence of generators. Backed by `SmallVec` with
/// a 32-element inline buffer — the overwhelming majority of words in
/// day-to-day use are shorter than this and never allocate.
pub type BraidWord = SmallVec<[Generator; 32]>;

/// Maximum braid word length accepted by the public API. Keeps
/// pathological inputs from causing unbounded arithmetic cost.
pub const MAX_BRAID_LEN: usize = 4096;

/// Free reduction: iteratively cancel adjacent `x x⁻¹` pairs until
/// the word is freely reduced. Does not impose BKL form — that's
/// stronger, see [`bkl_normal_form`].
#[must_use]
pub fn free_reduce(word: &BraidWord) -> BraidWord {
    let mut out: BraidWord = smallvec![];
    for &g in word {
        match out.last() {
            Some(&prev) if prev.cancels_with(g) => {
                out.pop();
            }
            _ => out.push(g),
        }
    }
    out
}

/// Yang-Baxter / braid relation check: verifies σ₁σ₂σ₁ = σ₂σ₁σ₂ holds
/// in any matrix representation returned by [`burau_omega5`].
/// Returns the Frobenius-norm distance between the two products; an
/// acceptable "equal under floating-point" check is distance < 1e-10.
pub fn yang_baxter_residual() -> f64 {
    let s1 = burau_omega5(Generator::Sigma1);
    let s2 = burau_omega5(Generator::Sigma2);
    let lhs = mat3_mul(&mat3_mul(&s1, &s2), &s1);
    let rhs = mat3_mul(&mat3_mul(&s2, &s1), &s2);
    mat3_distance(&lhs, &rhs)
}

// ──────────────────────────────────────────────────────────────────────
// BKL normal form (placeholder)
// ──────────────────────────────────────────────────────────────────────

/// Birman-Ko-Lee canonical form. Placeholder for Phase 1.
///
/// BKL form rewrites any braid word as `Δᵏ · x₁ · x₂ · … · xₙ`
/// where `Δ` is the fundamental half-twist and each `xᵢ` is a
/// "canonical factor" drawn from a specific Garside-theoretic set.
/// Two words are equal in B₃ iff they share the same BKL normal form.
///
/// Phase 1 stubs this with free reduction + length check. Phase 2
/// implements the full Garside-normal-form algorithm.
pub fn bkl_normal_form(word: &BraidWord) -> Result<BraidWord, BraidError> {
    if word.is_empty() {
        return Err(BraidError::EmptyWord);
    }
    if word.len() > MAX_BRAID_LEN {
        return Err(BraidError::WordTooLong {
            max: MAX_BRAID_LEN,
            got: word.len(),
        });
    }
    // Phase 1: free reduction is a coarse lower bound on BKL.
    Ok(free_reduce(word))
}

// ──────────────────────────────────────────────────────────────────────
// Burau representation at ω₅ = e^(2πi/5)
// ──────────────────────────────────────────────────────────────────────

/// Type alias for a 3×3 complex matrix. Row-major, `m[row][col]`.
pub type Mat3 = [[Complex64; 3]; 3];

/// Primitive fifth root of unity, `ω₅ = e^(2πi/5)`.
#[must_use]
pub fn omega5() -> Complex64 {
    let theta = 2.0 * PI / 5.0;
    Complex64::new(theta.cos(), theta.sin())
}

/// Burau representation of a single B₃ generator, evaluated at
/// `t = ω₅`. Matrices follow the reduced Burau convention:
///
/// ```text
/// σ₁ ↦ [[-t, 1, 0],
///       [ 0, 1, 0],
///       [ 0, 0, 1]]
///
/// σ₂ ↦ [[1, 0, 0],
///       [t, -t, 1],
///       [0, 0,  1]]
/// ```
///
/// (Phase 1 ships the reduced form; Phase 2 will migrate to the
/// unreduced Burau for compatibility with standard Jones-polynomial
/// references.)
#[must_use]
pub fn burau_omega5(g: Generator) -> Mat3 {
    let t = omega5();
    let zero = Complex64::new(0.0, 0.0);
    let one = Complex64::new(1.0, 0.0);

    match g {
        Generator::Sigma1 => [
            [-t, one, zero],
            [zero, one, zero],
            [zero, zero, one],
        ],
        Generator::Sigma1Inv => [
            [-one / t, one / t, zero],
            [zero, one, zero],
            [zero, zero, one],
        ],
        Generator::Sigma2 => [
            [one, zero, zero],
            [t, -t, one],
            [zero, zero, one],
        ],
        Generator::Sigma2Inv => [
            [one, zero, zero],
            [one, -one / t, one / t],
            [zero, zero, one],
        ],
    }
}

/// Evaluate a braid word under Burau at ω₅. Returns the composed 3×3
/// complex matrix.
pub fn burau_word(word: &BraidWord) -> Result<Mat3, BraidError> {
    if word.len() > MAX_BRAID_LEN {
        return Err(BraidError::WordTooLong {
            max: MAX_BRAID_LEN,
            got: word.len(),
        });
    }
    let one = Complex64::new(1.0, 0.0);
    let zero = Complex64::new(0.0, 0.0);
    let mut acc: Mat3 = [[one, zero, zero], [zero, one, zero], [zero, zero, one]];
    for &g in word {
        acc = mat3_mul(&acc, &burau_omega5(g));
    }
    Ok(acc)
}

// ──────────────────────────────────────────────────────────────────────
// Matrix arithmetic helpers (private — we don't expose 3×3 as API)
// ──────────────────────────────────────────────────────────────────────

fn mat3_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut out = [[Complex64::new(0.0, 0.0); 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut sum = Complex64::new(0.0, 0.0);
            for k in 0..3 {
                sum += a[i][k] * b[k][j];
            }
            out[i][j] = sum;
        }
    }
    out
}

fn mat3_distance(a: &Mat3, b: &Mat3) -> f64 {
    let mut sq = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            let d = a[i][j] - b[i][j];
            sq += d.norm_sqr();
        }
    }
    sq.sqrt()
}

// ──────────────────────────────────────────────────────────────────────
// Fibonacci fusion category
// ──────────────────────────────────────────────────────────────────────

/// Objects in `Fib`. The category has exactly two simple objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibObject {
    /// Vacuum / trivial object `1`.
    Vacuum,
    /// The non-trivial anyon, `τ`. Satisfies τ ⊗ τ = 1 ⊕ τ.
    Tau,
}

/// Charge labels in the Drinfeld double `Z(Fib) = Fib ⊠ F̄ib`.
/// The Z₂ grading pairs left and right chiralities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charge {
    /// `(1, 1)` — vacuum on both sides.
    Vacuum,
    /// `(τ, 1)` — left-chiral Fibonacci anyon.
    TauLeft,
    /// `(1, τ)` — right-chiral Fibonacci anyon.
    TauRight,
    /// `(τ, τ)` — both-chirality pair. This is the charge produced by
    /// a full 4π (double-cover) traversal of the Diamond Pattern.
    TauPair,
}

impl Charge {
    /// Fusion under the Z(Fib) rules. Returns the multiset of possible
    /// fusion outcomes; callers decide how to interpret (projective,
    /// summed, or probabilistic).
    #[must_use]
    pub fn fuse(self, other: Self) -> SmallVec<[Self; 4]> {
        use Charge::{TauLeft, TauPair, TauRight, Vacuum};
        match (self, other) {
            (Vacuum, x) | (x, Vacuum) => smallvec![x],
            (TauLeft, TauLeft) => smallvec![Vacuum, TauLeft],
            (TauRight, TauRight) => smallvec![Vacuum, TauRight],
            (TauLeft, TauRight) | (TauRight, TauLeft) => smallvec![TauPair],
            (TauPair, TauPair) => smallvec![Vacuum, TauLeft, TauRight, TauPair],
            (TauPair, TauLeft) | (TauLeft, TauPair) => {
                smallvec![TauRight, TauPair]
            }
            (TauPair, TauRight) | (TauRight, TauPair) => {
                smallvec![TauLeft, TauPair]
            }
        }
    }
}

/// Golden ratio φ = (1 + √5) / 2. Quantum dimension of `τ`.
#[must_use]
pub fn phi() -> f64 {
    (1.0 + 5.0_f64.sqrt()) / 2.0
}

/// F-symbol of `Fib` (the single non-trivial associator entry).
/// See e.g. Rowell-Stong-Wang 2009, §2.
///
/// For the pentagon-satisfying unitary solution:
///
/// ```text
/// F^{τττ}_τ = [[1/φ,   1/√φ],
///              [1/√φ, -1/φ ]]
/// ```
#[must_use]
pub fn f_symbol_tau() -> [[f64; 2]; 2] {
    let f = phi();
    let inv_f = 1.0 / f;
    let inv_sqrt_f = 1.0 / f.sqrt();
    [[inv_f, inv_sqrt_f], [inv_sqrt_f, -inv_f]]
}

/// R-symbol diagonal entries for `Fib`. The generator σ of the braid
/// group acts diagonally in the `{1, τ}` fusion basis at the τττ
/// splitting vertex; these are the phases.
#[must_use]
pub fn r_symbols() -> (Complex64, Complex64) {
    // R^{ττ}_1 = e^(-4πi/5),  R^{ττ}_τ = e^(3πi/5).
    let a = Complex64::from_polar(1.0, -4.0 * PI / 5.0);
    let b = Complex64::from_polar(1.0, 3.0 * PI / 5.0);
    (a, b)
}

// ──────────────────────────────────────────────────────────────────────
// Jones polynomial at ω₅ (placeholder)
// ──────────────────────────────────────────────────────────────────────

/// Closure invariant: Jones polynomial of the braid closure, evaluated
/// at `t = ω₅`. Phase 1 placeholder returns a trace-based proxy; Phase 2
/// implements the full Kauffman-bracket recursion.
pub fn jones_at_omega5(word: &BraidWord) -> Result<Complex64, BraidError> {
    let m = burau_word(word)?;
    // Unnormalised trace — full Jones requires writhe correction and
    // Markov moves, but this is the first-order invariant that Phase 2
    // refines. Good enough for commitment-as-witness in Phase 1.
    Ok(m[0][0] + m[1][1] + m[2][2])
}

// ──────────────────────────────────────────────────────────────────────
// BraidCommitment — the bytes we hand to atom-sig
// ──────────────────────────────────────────────────────────────────────

/// Opaque byte form of a braid-rail commitment, suitable for
/// `atom_sig::AtomCommitment::with_braid`.
///
/// Layout:
/// - 2 bytes: version (0x00 0x01 for Phase 1)
/// - 4 bytes: big-endian word length
/// - N bytes: packed generator stream (2 bits per generator)
/// - 16 bytes: real and imaginary parts of `jones_at_omega5` as f64
/// - 32 bytes: BLAKE3 seed from which the word was derived (for audit)
#[must_use]
pub fn braid_commitment_bytes(word: &BraidWord, seed: &[u8; 32]) -> Vec<u8> {
    let jones = jones_at_omega5(word).unwrap_or(Complex64::new(0.0, 0.0));
    let mut out = Vec::with_capacity(2 + 4 + (word.len() + 3) / 4 + 16 + 32);
    out.extend_from_slice(&[0x00, 0x01]);
    out.extend_from_slice(&(word.len() as u32).to_be_bytes());
    // Pack 4 generators per byte.
    let mut byte: u8 = 0;
    for (i, g) in word.iter().enumerate() {
        byte |= (*g as u8) << ((i % 4) * 2);
        if i % 4 == 3 {
            out.push(byte);
            byte = 0;
        }
    }
    if word.len() % 4 != 0 {
        out.push(byte);
    }
    out.extend_from_slice(&jones.re.to_be_bytes());
    out.extend_from_slice(&jones.im.to_be_bytes());
    out.extend_from_slice(seed);
    out
}

// ──────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_inverse_is_involutive() {
        for g in [
            Generator::Sigma1,
            Generator::Sigma1Inv,
            Generator::Sigma2,
            Generator::Sigma2Inv,
        ] {
            assert_eq!(g.inverse().inverse(), g);
            assert!(g.cancels_with(g.inverse()));
        }
    }

    #[test]
    fn free_reduction_cancels_pairs() {
        let w: BraidWord = smallvec![
            Generator::Sigma1,
            Generator::Sigma1Inv,
            Generator::Sigma2,
        ];
        let r = free_reduce(&w);
        assert_eq!(r.as_slice(), &[Generator::Sigma2]);
    }

    #[test]
    fn yang_baxter_holds_numerically() {
        // σ₁σ₂σ₁ = σ₂σ₁σ₂ in any faithful representation; the
        // residual should be dominated by floating-point epsilon.
        let residual = yang_baxter_residual();
        assert!(residual < 1e-10, "Y-B residual too large: {residual}");
    }

    #[test]
    fn phi_squared_equals_phi_plus_one() {
        // Defining property of the golden ratio: φ² = φ + 1.
        let f = phi();
        assert!((f * f - (f + 1.0)).abs() < 1e-12);
    }

    #[test]
    fn fibonacci_fusion_rule() {
        // τ⊗τ in Z(Fib): left-left fusion should reduce to {1, τ_L}.
        let outcomes = Charge::TauLeft.fuse(Charge::TauLeft);
        assert!(outcomes.contains(&Charge::Vacuum));
        assert!(outcomes.contains(&Charge::TauLeft));
    }

    #[test]
    fn omega5_is_fifth_root_of_unity() {
        let w = omega5();
        let w5 = w * w * w * w * w;
        assert!((w5.re - 1.0).abs() < 1e-12);
        assert!(w5.im.abs() < 1e-12);
    }
}
