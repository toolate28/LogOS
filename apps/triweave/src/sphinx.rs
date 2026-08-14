//! Local SPHINX gate helpers for the triweave vault.
//!
//! Category **C**: deterministic braid word + SHA-256 fingerprint.
//! This is **not** a Jones polynomial, not Cubical path induction, and not
//! a fail-closed deploy gate. `α + ω = 15` is a label only.

use sha2::{Digest, Sha256};

/// Map payload bytes to a deterministic B₃-ish generator sequence.
/// Even byte → σ₁, odd → σ₂; bit 1 chooses sign.
pub fn payload_to_braid_word(payload: &str) -> Vec<i32> {
    payload
        .bytes()
        .map(|b| {
            let gen = if b & 1 == 0 { 1i32 } else { 2 };
            let sign = if b & 2 == 0 { 1i32 } else { -1 };
            sign * gen
        })
        .collect()
}

/// Hex SHA-256 of the little-endian generator list.
/// `strands` is accepted for call-site compatibility (`compute_fingerprint(_, 3)`).
pub fn compute_fingerprint(braid: &[i32], _strands: u32) -> String {
    let mut hasher = Sha256::new();
    for g in braid {
        hasher.update(g.to_le_bytes());
    }
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// True when `fingerprint` matches `compute_fingerprint(braid, 3)`.
pub fn validate(braid: &[i32], fingerprint: &str) -> bool {
    compute_fingerprint(braid, 3) == fingerprint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_stable() {
        let w = payload_to_braid_word("key:pass");
        let a = compute_fingerprint(&w, 3);
        let b = compute_fingerprint(&w, 3);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(validate(&w, &a));
        assert!(!validate(&w, "deadbeef"));
    }
}
