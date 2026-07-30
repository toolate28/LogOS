use crate::crypto::{atomic_write, decode_cbor_zstd, encode_cbor_zstd, SpiralSafeSeal};
use crate::key_rotation::KeyRotationProof;
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MEANING_SEED_VERSION: u32 = 2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MetaParameters {
    pub buckling_wavelength: f32,
    pub twist_radians: f32,
    pub regularization_radius: Option<f32>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MeaningSeed {
    pub coherence_snapshot: f32,
    pub key_parameters: MetaParameters,
    pub invariant_proof_hash: String,
    pub timestamp: u64,
    pub version: u32,
    /// Monotonic SpiralSafe key generation that signed this seed.
    pub key_sequence: u64,
    /// Present on the first seed saved immediately after a rotation event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_proof: Option<KeyRotationProof>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spiral_safe_seal: Option<SpiralSafeSeal>,
    /// Machine-readable link: Lean4 theorem anchor (e.g. TriWeavon/VanishingResilience.lean#...).
    #[serde(default)]
    pub current_public_key: Option<[u8; 32]>,
    /// BLAKE3 Merkle root of signed OscillatorChangeAudit log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_root: Option<[u8; 32]>,
    #[serde(default)]
    pub audit_log_count: u64,
}

impl MeaningSeed {
    pub fn new(
        coherence: f32,
        params: MetaParameters,
        proof_hash: String,
        key_sequence: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            coherence_snapshot: coherence,
            key_parameters: params,
            invariant_proof_hash: proof_hash,
            timestamp,
            version: MEANING_SEED_VERSION,
            key_sequence,
            rotation_proof: None,
            spiral_safe_seal: None,
            current_public_key: None,
            audit_log_root: None,
            audit_log_count: 0,
        }
    }

    pub fn with_audit_linkage(
        mut self,
        public_key: [u8; 32],
        audit_root: [u8; 32],
        audit_count: u64,
    ) -> Self {
        self.current_public_key = Some(public_key);
        self.audit_log_root = Some(audit_root);
        self.audit_log_count = audit_count;
        self
    }

    pub fn with_rotation_proof(mut self, proof: KeyRotationProof) -> Self {
        self.rotation_proof = Some(proof);
        self
    }

    /// Unsigned CBOR + zstd persistence (legacy / dev path).
    pub fn save(&self, path: &str) -> io::Result<()> {
        let compressed = encode_cbor_zstd(self)?;
        atomic_write(path, &compressed)
    }

    /// SpiralSafe v2: CBOR → BLAKE3 → Ed25519 sign → zstd.
    pub fn save_signed(
        &self,
        path: &str,
        signing_key: &SigningKey,
        key_sequence: u64,
    ) -> io::Result<()> {
        let mut unsigned = self.clone();
        unsigned.spiral_safe_seal = None;

        let mut cbor_bytes = Vec::new();
        ciborium::ser::into_writer(&unsigned, &mut cbor_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let seal = SpiralSafeSeal::sign_payload(&cbor_bytes, signing_key, key_sequence);

        let mut signed = unsigned;
        signed.spiral_safe_seal = Some(seal);

        let compressed = encode_cbor_zstd(&signed)?;
        atomic_write(path, &compressed)
    }

    pub fn load(path: &str) -> io::Result<Self> {
        if !Path::new(path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "MeaningSeed not found",
            ));
        }

        let compressed = std::fs::read(path)?;
        let seed: MeaningSeed = decode_cbor_zstd(&compressed)?;

        if seed.version != MEANING_SEED_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported MeaningSeed version {}", seed.version),
            ));
        }

        Ok(seed)
    }

    /// Load and verify SpiralSafe v2 seal against the provided keyring verifier.
    pub fn load_and_verify(
        path: &str,
        verifying_key: &VerifyingKey,
    ) -> io::Result<Self> {
        let seed = Self::load(path)?;
        seed.verify_seal(verifying_key)?;
        Ok(seed)
    }

    /// Verify the embedded seal against a known verifying key.
    pub fn verify_seal(&self, verifying_key: &VerifyingKey) -> io::Result<()> {
        let seal = self
            .spiral_safe_seal
            .as_ref()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "missing SpiralSafe v2 seal")
            })?;

        seal.verify(verifying_key)
    }

    pub fn delete(path: &str) -> io::Result<()> {
        if Path::new(path).exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}