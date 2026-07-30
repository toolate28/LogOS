//! SpiralSafe v2 — cryptographic persistence and key rotation for the
//! 1-Pixel Reduction paradigm.
//!
//! Provides:
//! - [`MeaningSeed`] persistence (CBOR → BLAKE3 → Ed25519 → zstd)
//! - [`SpiralSafeKeyRing`] with atomic key rotation and historical verification
//! - [`OscillatorChangeAudit`] signing with multi-key audit trail support

pub mod audit;
pub mod crypto;
pub mod key_rotation;
pub mod meaning_seed;

pub use audit::{load_and_verify_audit_log, OscillatorChangeAudit, OscillatorGlobals};
pub use key_rotation::{KeyRotationProof, KeyRingState, SpiralSafeKeyRing};
pub use meaning_seed::{MeaningSeed, MetaParameters, MEANING_SEED_VERSION};