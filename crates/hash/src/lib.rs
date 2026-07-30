//! reson8-hash — Cryptographic hashing and ATOM-TAG provenance

/// SHA-256 prefix tag for ATOM trail entries.
pub fn atom_tag(content: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(content);
    format!("ATOM:{}", hex::encode(&hash[..16]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atom_tag_is_deterministic() {
        let a = atom_tag(b"alpha+omega=15");
        let b = atom_tag(b"alpha+omega=15");
        assert_eq!(a, b);
        assert!(a.starts_with("ATOM:"));
        assert_eq!(a.len(), 5 + 32);
    }

    #[test]
    fn atom_tag_differs_for_different_input() {
        assert_ne!(atom_tag(b"a"), atom_tag(b"b"));
    }
}