//! # atom-sig
//!
//! Canonical ATOM commitment format for the Reson8-Labs Tri-Weavon.
//!
//! An *ATOM* is a minted governance / structural / research commitment.
//! `atom-sig` is the canonicaliser: it takes an in-memory [`AtomPayload`]
//! and produces a byte-for-byte reproducible [`AtomCommitment`] that
//! can be:
//!
//! - stored in the 9P|Styx Bookshelf,
//! - anchored on NEAR via `conservation.spiralsafe.near`,
//! - cross-checked by any strand (Claude, Grok, Gemini, Manus) under
//!   their respective provider agreements.
//!
//! ## Layered commitment
//!
//! An [`AtomCommitment`] has three layers:
//!
//! 1. **Canonical bytes** — deterministic CBOR encoding of the payload.
//! 2. **Classical digest** — BLAKE3 hash over the canonical bytes.
//! 3. **Classical signature** — ed25519 signature over the digest.
//!
//! Optional fourth layer, owned by the companion crate `fib-braid-core`:
//!
//! 4. **Topological rail** — a braid word `w ∈ B₃` in Birman-Ko-Lee
//!    normal form, plus Jones polynomial at `ω₅`, plus Fibonacci
//!    representation evaluation. `atom-sig` exposes a hook
//!    ([`AtomCommitment::with_braid`]) without depending on
//!    `fib-braid-core` directly, to avoid cyclic feature coupling.
//!
//! ## Invariants
//!
//! - Canonicalisation is deterministic: encoding twice with the same
//!   payload produces bit-identical bytes.
//! - The `α + ω = 15` universal invariant is **not** enforced by
//!   `atom-sig`. That is the job of the Invariant Gate
//!   (`coherence-mcp::check_coherence`). `atom-sig` only guarantees
//!   *that whatever the payload is, its commitment is reproducible*.
//! - `no_std`-compatible (default-features = false, feature = "nostd").
//!
//! ## Non-goals
//!
//! - Network I/O. Callers hand us bytes; we hand bytes back.
//! - Schema evolution. Version the `AtomPayload` type at the call-site
//!   (e.g. `AtomPayloadV0_3`, `AtomPayloadV0_4`) and convert upward.
//! - Threshold signatures. A future `atom-sig-threshold` sibling
//!   crate may add that.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use blake3::Hasher;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

// ──────────────────────────────────────────────────────────────────────
// Error type
// ──────────────────────────────────────────────────────────────────────

/// All failure modes that `atom-sig` can produce.
#[derive(Debug)]
pub enum AtomSigError {
    /// CBOR encode failed. Should be unreachable for well-formed
    /// [`AtomPayload`]; indicates an internal serialiser bug if hit.
    EncodeFailed,
    /// CBOR decode failed. Payload bytes are corrupt or from a
    /// foreign encoder.
    DecodeFailed,
    /// Signature verification failed. Digest / sig / key mismatch.
    SignatureInvalid,
    /// Digest does not match canonical bytes. Storage corruption.
    DigestMismatch,
    /// Braid-rail hook payload was declared but has zero length.
    EmptyBraidPayload,
}

// ──────────────────────────────────────────────────────────────────────
// AtomPayload — the thing being committed
// ──────────────────────────────────────────────────────────────────────

/// Everything a strand wants to commit, in a form stable across
/// canonicalisation cycles.
///
/// This is deliberately a thin wrapper around `(id, class, body, ts)`
/// with an escape hatch (`extensions`) for future schema growth. The
/// canonical encoding is **CBOR with deterministic map ordering**
/// (RFC 8949 §4.2.1), which is why we roll our own serialiser in
/// [`canonical_encode`] rather than trust default serde output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomPayload {
    /// Stable ATOM identifier, e.g.
    /// `ATOM-LICENSE-DUAL-MIT-APACHE-20260418`.
    pub id: String,
    /// One-line class label, e.g. `"Governance · re-licensing"`.
    pub class: String,
    /// Arbitrary structured body. Kept as raw CBOR bytes so the
    /// canonicaliser doesn't need to know the schema — each ATOM
    /// class can define its own shape.
    pub body: Vec<u8>,
    /// Unix timestamp (seconds). UTC.
    pub timestamp_utc: u64,
    /// Conservation ledger summary. The Gate verifies; we only store.
    pub conservation: ConservationSummary,
    /// Forward-compatible extension slot. Always canonicalised after
    /// the core fields regardless of insertion order.
    pub extensions: Vec<(String, Vec<u8>)>,
}

/// Compact α+ω conservation summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConservationSummary {
    /// α: structural rigidity load.
    pub alpha: u16,
    /// ω: semantic intent load.
    pub omega: u16,
}

impl ConservationSummary {
    /// Universal invariant check. Does NOT reject; callers decide.
    #[must_use]
    pub fn sum(&self) -> u32 {
        u32::from(self.alpha) + u32::from(self.omega)
    }

    /// Convenience: is this ATOM on-invariant (α + ω = 15)?
    #[must_use]
    pub fn on_invariant(&self) -> bool {
        self.sum() == 15
    }

    /// Convenience: distance from the Viviani Peak (7, 8) in ℝ²
    /// (squared, to stay in integer arithmetic).
    #[must_use]
    pub fn viviani_distance_sq(&self) -> u32 {
        let da = i32::from(self.alpha) - 7;
        let do_ = i32::from(self.omega) - 8;
        (da * da + do_ * do_) as u32
    }
}

// ──────────────────────────────────────────────────────────────────────
// Canonical encoding
// ──────────────────────────────────────────────────────────────────────

/// Deterministic CBOR encode of an [`AtomPayload`].
///
/// Guarantees:
/// - Map keys serialised in lexicographic byte order (CBOR
///   deterministic profile, RFC 8949 §4.2.1).
/// - No indefinite-length items.
/// - `extensions` always last, sorted by key.
/// - Integers use shortest form.
///
/// Implementation is a placeholder; Phase 1 ships a full encoder.
pub fn canonical_encode(_payload: &AtomPayload) -> Result<Vec<u8>, AtomSigError> {
    // TODO(Phase 1): implement deterministic CBOR encode.
    // For now return an empty vector to keep the type signatures
    // exercised during crate bring-up.
    Err(AtomSigError::EncodeFailed)
}

/// Inverse of [`canonical_encode`]. Fails on any deviation from the
/// deterministic profile.
pub fn canonical_decode(_bytes: &[u8]) -> Result<AtomPayload, AtomSigError> {
    // TODO(Phase 1): implement decode with strict-profile enforcement.
    Err(AtomSigError::DecodeFailed)
}

// ──────────────────────────────────────────────────────────────────────
// Commitment
// ──────────────────────────────────────────────────────────────────────

/// The minted, signed commitment. This is what lands in the ledger.
#[derive(Debug, Clone)]
pub struct AtomCommitment {
    /// Canonical CBOR bytes of the payload.
    pub canonical_bytes: Vec<u8>,
    /// BLAKE3 digest of `canonical_bytes`.
    pub digest: [u8; 32],
    /// ed25519 signature over `digest`.
    pub signature: Signature,
    /// Signing key's public component. Strand identity on the classical
    /// rail.
    pub verifying_key: VerifyingKey,
    /// Optional topological-rail commitment produced by
    /// `fib-braid-core`. Opaque bytes here; `fib-braid-core` owns the
    /// interpretation.
    pub braid_commitment: Option<Vec<u8>>,
}

impl AtomCommitment {
    /// Mint a commitment from a payload + signing key.
    pub fn mint(
        payload: &AtomPayload,
        signing_key: &SigningKey,
    ) -> Result<Self, AtomSigError> {
        let canonical_bytes = canonical_encode(payload)?;

        let mut hasher = Hasher::new();
        hasher.update(&canonical_bytes);
        let digest: [u8; 32] = hasher.finalize().into();

        let signature = signing_key.sign(&digest);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            canonical_bytes,
            digest,
            signature,
            verifying_key,
            braid_commitment: None,
        })
    }

    /// Attach a topological-rail commitment (produced by
    /// `fib-braid-core`) to this classical commitment. Returns self
    /// by value for builder-style chaining.
    #[must_use]
    pub fn with_braid(mut self, braid_commitment: Vec<u8>) -> Self {
        if !braid_commitment.is_empty() {
            self.braid_commitment = Some(braid_commitment);
        }
        self
    }

    /// Verify the classical-rail commitment. Does NOT validate the
    /// braid rail — that's the Fibonacci-representation check inside
    /// `fib-braid-core::verify_braid_commitment`.
    pub fn verify_classical(&self) -> Result<(), AtomSigError> {
        let mut hasher = Hasher::new();
        hasher.update(&self.canonical_bytes);
        let recomputed: [u8; 32] = hasher.finalize().into();
        if recomputed != self.digest {
            return Err(AtomSigError::DigestMismatch);
        }

        self.verifying_key
            .verify(&self.digest, &self.signature)
            .map_err(|_| AtomSigError::SignatureInvalid)
    }
}

// ──────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conservation_invariant_arithmetic() {
        let peak = ConservationSummary { alpha: 7, omega: 8 };
        assert_eq!(peak.sum(), 15);
        assert!(peak.on_invariant());
        assert_eq!(peak.viviani_distance_sq(), 0);

        let off = ConservationSummary { alpha: 4, omega: 11 };
        assert_eq!(off.sum(), 15);
        assert!(off.on_invariant());
        // (4,11) - (7,8) = (-3, 3), squared-distance = 9 + 9 = 18
        assert_eq!(off.viviani_distance_sq(), 18);
    }

    #[test]
    fn off_invariant_detected() {
        let bad = ConservationSummary { alpha: 5, omega: 5 };
        assert!(!bad.on_invariant());
    }
}
