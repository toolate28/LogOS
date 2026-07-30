use crate::crypto::{atomic_write, decode_cbor_zstd, encode_cbor_zstd, SpiralSafeSeal};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::io;

/// Signed proof that a key rotation occurred.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeyRotationProof {
    pub old_key_sequence: u64,
    pub new_key_sequence: u64,
    pub new_public_key: [u8; 32],
    pub rotated_at: u64,
    pub seal: SpiralSafeSeal,
}

/// Persisted keyring state for rotation-resilient verification.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeyRingState {
    pub current_sequence: u64,
    pub current_public_key: [u8; 32],
    pub history: Vec<[u8; 32]>,
}

/// SpiralSafe v2 keyring with atomic rotation.
#[derive(Clone)]
pub struct SpiralSafeKeyRing {
    signing_key: SigningKey,
    state: KeyRingState,
}

impl SpiralSafeKeyRing {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let public_bytes = verifying_key.to_bytes();
        Self {
            signing_key,
            state: KeyRingState {
                current_sequence: 1,
                current_public_key: public_bytes,
                history: vec![public_bytes],
            },
        }
    }

    pub fn sequence(&self) -> u64 {
        self.state.current_sequence
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn current_public_key(&self) -> [u8; 32] {
        self.state.current_public_key
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub fn rotate(&mut self) -> KeyRotationProof {
        let old_sequence = self.state.current_sequence;
        let old_key = self.signing_key.clone();

        let new_signing = SigningKey::generate(&mut OsRng);
        let new_public = new_signing.verifying_key().to_bytes();
        let new_sequence = old_sequence + 1;

        let mut payload = Vec::with_capacity(48);
        payload.extend_from_slice(b"spiralsafe-rotate:");
        payload.extend_from_slice(&old_sequence.to_le_bytes());
        payload.extend_from_slice(&new_sequence.to_le_bytes());
        payload.extend_from_slice(&new_public);

        let seal = SpiralSafeSeal::sign_payload(&payload, &old_key, old_sequence);

        self.signing_key = new_signing;
        self.state.current_sequence = new_sequence;
        self.state.current_public_key = new_public;
        self.state.history.push(new_public);

        KeyRotationProof {
            old_key_sequence: old_sequence,
            new_key_sequence: new_sequence,
            new_public_key: new_public,
            rotated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            seal,
        }
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let bytes = encode_cbor_zstd(&self.state)?;
        atomic_write(path, &bytes)
    }

    pub fn load(path: &str, signing_key: SigningKey) -> io::Result<Self> {
        let compressed = std::fs::read(path)?;
        let state: KeyRingState = decode_cbor_zstd(&compressed)?;
        Ok(Self { signing_key, state })
    }
}