use serde::{Deserialize, Serialize};

use crate::protocol::ConservationState;

pub const QUARANTINE_POLICY_PATH: &str = "docs/epistemics/ALPHA-OMEGA-QUARANTINE.md";
pub const QUARANTINE_MANIFEST_PATH: &str = "ops/quarantine/alpha-omega-quarantine.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineOperation {
    Inspect,
    RunSafeTests,
    RecordHandoffRecognition,
    AuthorizeStateTransition,
    PromoteState,
    PublishArtifact,
    DeployArtifact,
    GenerateSemanticClaim,
}

impl QuarantineOperation {
    pub const fn is_read_only(self) -> bool {
        matches!(
            self,
            Self::Inspect | Self::RunSafeTests | Self::RecordHandoffRecognition
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineMetadata {
    pub surface: String,
    pub manifest_path: String,
    pub policy_path: String,
    pub evidence_hash: String,
    pub conservation: ConservationState,
}

impl QuarantineMetadata {
    pub fn is_well_formed(&self) -> bool {
        !self.surface.trim().is_empty()
            && self.manifest_path == QUARANTINE_MANIFEST_PATH
            && self.policy_path == QUARANTINE_POLICY_PATH
            && self.evidence_hash.len() == 64
            && self.evidence_hash.chars().all(|c| c.is_ascii_hexdigit())
            && self.conservation.sum == self.conservation.alpha.saturating_add(self.conservation.omega)
            && self.conservation.valid == self.conservation.verify()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandoffRecognitionRecord {
    pub surface: String,
    pub evidence_hash: String,
    pub computation_result: bool,
    pub observed_sum: u8,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum QuarantineError {
    #[error("α/ω quarantine metadata missing; fail closed")]
    MissingMetadata,
    #[error("α/ω quarantine metadata malformed; fail closed")]
    MalformedMetadata,
    #[error("α/ω quarantine blocks {operation:?}; use evidence-derived state instead")]
    ProhibitedOperation { operation: QuarantineOperation },
}

pub fn enforce_quarantine_boundary(
    metadata: Option<&QuarantineMetadata>,
    operation: QuarantineOperation,
) -> Result<Option<HandoffRecognitionRecord>, QuarantineError> {
    let metadata = metadata.ok_or(QuarantineError::MissingMetadata)?;
    if !metadata.is_well_formed() {
        return Err(QuarantineError::MalformedMetadata);
    }

    if !operation.is_read_only() {
        return Err(QuarantineError::ProhibitedOperation { operation });
    }

    if matches!(operation, QuarantineOperation::RecordHandoffRecognition) {
        return Ok(Some(HandoffRecognitionRecord {
            surface: metadata.surface.clone(),
            evidence_hash: metadata.evidence_hash.clone(),
            computation_result: metadata.conservation.verify(),
            observed_sum: metadata.conservation.sum,
        }));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata() -> QuarantineMetadata {
        QuarantineMetadata {
            surface: "crates/core/src/superskill.rs".into(),
            manifest_path: QUARANTINE_MANIFEST_PATH.into(),
            policy_path: QUARANTINE_POLICY_PATH.into(),
            evidence_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
            conservation: ConservationState::new(7, 8),
        }
    }

    #[test]
    fn allows_read_only_handoff_recognition() {
        let record = enforce_quarantine_boundary(
            Some(&metadata()),
            QuarantineOperation::RecordHandoffRecognition,
        )
        .expect("read-only handoff recognition stays available")
        .expect("handoff record returned");

        assert!(record.computation_result);
        assert_eq!(record.observed_sum, 15);
        assert_eq!(record.surface, "crates/core/src/superskill.rs");
    }

    #[test]
    fn rejects_promotion_attempts_even_when_check_passes() {
        let err = enforce_quarantine_boundary(
            Some(&metadata()),
            QuarantineOperation::PromoteState,
        )
        .expect_err("promotion must stay blocked");

        assert_eq!(
            err,
            QuarantineError::ProhibitedOperation {
                operation: QuarantineOperation::PromoteState,
            }
        );
    }

    #[test]
    fn fails_closed_when_metadata_is_missing() {
        let err = enforce_quarantine_boundary(None, QuarantineOperation::Inspect)
            .expect_err("missing metadata must fail closed");
        assert_eq!(err, QuarantineError::MissingMetadata);
    }

    #[test]
    fn fails_closed_when_metadata_is_malformed() {
        let mut bad = metadata();
        bad.evidence_hash = "not-a-sha256".into();

        let err = enforce_quarantine_boundary(Some(&bad), QuarantineOperation::Inspect)
            .expect_err("malformed metadata must fail closed");
        assert_eq!(err, QuarantineError::MalformedMetadata);
    }
}
