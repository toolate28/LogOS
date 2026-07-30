use crate::crypto::{atomic_write, decode_cbor_zstd, encode_cbor_zstd, SpiralSafeSeal};
use blake3::Hasher;
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::io;

/// Live oscillator globals exposed to egui / coherence-mcp.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OscillatorGlobals {
    pub breath_strength: f32,
    pub resonance_threshold: f32,
    pub leakage_floor: f32,
    pub coupling_strength: f32,
}

impl Default for OscillatorGlobals {
    fn default() -> Self {
        Self {
            breath_strength: 0.12,
            resonance_threshold: 0.82,
            leakage_floor: 0.015,
            coupling_strength: 0.3,
        }
    }
}

/// Unsigned body used for deterministic signing.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct OscillatorChangeAuditBody {
    timestamp: u64,
    key_sequence: u64,
    invariant_proof_hash: String,
    before: OscillatorGlobals,
    after: OscillatorGlobals,
}

/// Signed audit entry for oscillator parameter changes.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OscillatorChangeAudit {
    pub timestamp: u64,
    pub key_sequence: u64,
    pub invariant_proof_hash: String,
    pub before: OscillatorGlobals,
    pub after: OscillatorGlobals,
    pub seal: SpiralSafeSeal,
}

impl OscillatorChangeAudit {
    pub fn sign(
        before: OscillatorGlobals,
        after: OscillatorGlobals,
        invariant_proof_hash: &str,
        signing_key: &SigningKey,
        key_sequence: u64,
    ) -> io::Result<Self> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let body = OscillatorChangeAuditBody {
            timestamp,
            key_sequence,
            invariant_proof_hash: invariant_proof_hash.to_string(),
            before,
            after,
        };

        let mut cbor = Vec::new();
        ciborium::ser::into_writer(&body, &mut cbor)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let seal = SpiralSafeSeal::sign_payload(&cbor, signing_key, key_sequence);

        Ok(Self {
            timestamp: body.timestamp,
            key_sequence: body.key_sequence,
            invariant_proof_hash: body.invariant_proof_hash,
            before: body.before,
            after: body.after,
            seal,
        })
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> io::Result<()> {
        let body = OscillatorChangeAuditBody {
            timestamp: self.timestamp,
            key_sequence: self.key_sequence,
            invariant_proof_hash: self.invariant_proof_hash.clone(),
            before: self.before.clone(),
            after: self.after.clone(),
        };

        let mut cbor = Vec::new();
        ciborium::ser::into_writer(&body, &mut cbor)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut hasher = Hasher::new();
        hasher.update(&cbor);
        let digest: [u8; 32] = hasher.finalize().into();

        if digest != self.seal.blake3_digest {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "audit digest mismatch",
            ));
        }

        self.seal.verify(verifying_key)
    }
}

/// Rolling Merkle-style root over signed audit entries.
pub fn compute_audit_log_root(entries: &[OscillatorChangeAudit]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(b"spiralsafe-audit-root-v1");
    for entry in entries {
        hasher.update(&entry.seal.blake3_digest);
    }
    hasher.finalize().into()
}

/// Append an audit entry and return updated root + count.
pub fn append_audit_entry(
    entries: &mut Vec<OscillatorChangeAudit>,
    entry: OscillatorChangeAudit,
) -> ([u8; 32], u64) {
    entries.push(entry);
    (compute_audit_log_root(entries), entries.len() as u64)
}

pub fn save_audit_log(path: &str, entries: &Vec<OscillatorChangeAudit>) -> io::Result<()> {
    let bytes = encode_cbor_zstd(entries)?;
    atomic_write(path, &bytes)
}

pub fn load_audit_log(path: &str) -> io::Result<Vec<OscillatorChangeAudit>> {
    let compressed = std::fs::read(path)?;
    decode_cbor_zstd(&compressed)
}

pub fn load_and_verify_audit_log(
    path: &str,
    verifying_key: &VerifyingKey,
    expected_root: Option<[u8; 32]>,
) -> io::Result<Vec<OscillatorChangeAudit>> {
    let entries = load_audit_log(path)?;
    for entry in &entries {
        entry.verify(verifying_key)?;
    }
    if let Some(root) = expected_root {
        let actual = compute_audit_log_root(&entries);
        if actual != root {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "audit_log_root mismatch",
            ));
        }
    }
    Ok(entries)
}