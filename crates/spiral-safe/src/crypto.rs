use blake3::Hasher;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::io;

/// SpiralSafe v2 signature envelope attached to persisted artifacts.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SpiralSafeSeal {
    pub blake3_digest: [u8; 32],
    pub ed25519_signature: Vec<u8>,
    pub ed25519_public_key: [u8; 32],
    pub key_sequence: u64,
}

impl SpiralSafeSeal {
    pub fn sign_payload(
        payload: &[u8],
        signing_key: &SigningKey,
        key_sequence: u64,
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(payload);
        let digest_bytes: [u8; 32] = hasher.finalize().into();

        let signature = signing_key.sign(&digest_bytes);
        let verifying_key = signing_key.verifying_key();

        Self {
            blake3_digest: digest_bytes,
            ed25519_signature: signature.to_bytes().to_vec(),
            ed25519_public_key: verifying_key.to_bytes(),
            key_sequence,
        }
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<(), io::Error> {
        if self.ed25519_public_key != verifying_key.to_bytes() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "public key mismatch",
            ));
        }

        let sig_bytes: [u8; 64] = self.ed25519_signature.as_slice().try_into().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "invalid signature length")
        })?;
        let signature = Signature::from_bytes(&sig_bytes);
        verifying_key
            .verify(&self.blake3_digest, &signature)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid Ed25519 signature"))
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey, io::Error> {
        VerifyingKey::from_bytes(&self.ed25519_public_key)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

/// Atomic write helper shared by MeaningSeed and KeyRing persistence.
pub fn atomic_write(path: &str, bytes: &[u8]) -> io::Result<()> {
    let temp_path = format!("{path}.tmp");
    {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;
        std::io::Write::write_all(&mut file, bytes)?;
        file.sync_all()?;
    }
    std::fs::rename(&temp_path, path)?;
    Ok(())
}

/// CBOR encode → zstd compress (level 3).
pub fn encode_cbor_zstd<T: serde::Serialize>(value: &T) -> io::Result<Vec<u8>> {
    let mut cbor_bytes = Vec::new();
    ciborium::ser::into_writer(value, &mut cbor_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    zstd::encode_all(&cbor_bytes[..], 3).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// zstd decompress → CBOR decode.
pub fn decode_cbor_zstd<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> io::Result<T> {
    let cbor_bytes = zstd::decode_all(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    ciborium::de::from_reader(std::io::Cursor::new(cbor_bytes))
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}