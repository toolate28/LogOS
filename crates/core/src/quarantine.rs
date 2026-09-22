use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::protocol::{AtomEntry, ConservationState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineOperation {
    Inspect,
    RunSafeTests,
    RecordHandoffRecognition,
    MirrorContextWindow,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionStateChange {
    pub from_state: String,
    pub to_state: String,
}

impl ExecutionStateChange {
    pub fn is_well_formed(&self) -> bool {
        !self.from_state.trim().is_empty() && !self.to_state.trim().is_empty()
    }

    pub fn is_read_only(&self) -> bool {
        self.from_state == self.to_state
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineContext {
    pub surface: String,
    pub atom_trail: AtomEntry,
    pub state_change: ExecutionStateChange,
    pub conservation: ConservationState,
}

impl QuarantineContext {
    pub fn is_well_formed(&self) -> bool {
        let expected_hash = canonical_atom_hash(&self.atom_trail);
        !self.surface.trim().is_empty()
            && !self.atom_trail.id.trim().is_empty()
            && !self.atom_trail.atom_type.trim().is_empty()
            && !self.atom_trail.gate.trim().is_empty()
            && !self.atom_trail.description.trim().is_empty()
            && self.atom_trail.prev_hash.len() == 64
            && self
                .atom_trail
                .prev_hash
                .chars()
                .all(|c| c.is_ascii_hexdigit())
            && self.atom_trail.hash.len() == 64
            && self.atom_trail.hash.chars().all(|c| c.is_ascii_hexdigit())
            && self.atom_trail.hash.eq_ignore_ascii_case(&expected_hash)
            && self.state_change.is_well_formed()
            && self.conservation.sum
                == self
                    .conservation
                    .alpha
                    .saturating_add(self.conservation.omega)
            && self.conservation.valid == self.conservation.verify()
    }
}

fn canonical_atom_hash(entry: &AtomEntry) -> String {
    #[derive(Serialize)]
    struct CanonicalAtomEntry<'a> {
        id: &'a str,
        atom_type: &'a str,
        gate: &'a str,
        description: &'a str,
        coherence: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
        prev_hash: &'a str,
    }

    let canonical = CanonicalAtomEntry {
        id: &entry.id,
        atom_type: &entry.atom_type,
        gate: &entry.gate,
        description: &entry.description,
        coherence: entry.coherence,
        timestamp: entry.timestamp,
        prev_hash: &entry.prev_hash,
    };

    let serialized = serde_json::to_vec(&canonical).expect("canonical atom serialization");
    let digest = Sha256::digest(serialized);
    format!("{:x}", digest)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandoffRecognitionRecord {
    pub surface: String,
    pub evidence_hash: String,
    pub atom_trail_id: String,
    pub computation_result: bool,
    pub observed_sum: u8,
    pub from_state: String,
    pub to_state: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum QuarantineError {
    #[error("α/ω quarantine context missing; fail closed")]
    MissingContext,
    #[error("α/ω quarantine context malformed; fail closed")]
    MalformedContext,
    #[error("α/ω quarantine blocks {operation:?}; use evidence-derived state instead")]
    ProhibitedOperation { operation: QuarantineOperation },
}

pub fn enforce_quarantine_boundary(
    context: Option<&QuarantineContext>,
    operation: QuarantineOperation,
) -> Result<Option<HandoffRecognitionRecord>, QuarantineError> {
    let context = context.ok_or(QuarantineError::MissingContext)?;
    if !context.is_well_formed() {
        return Err(QuarantineError::MalformedContext);
    }

    if !operation.is_read_only()
        || (matches!(operation, QuarantineOperation::RecordHandoffRecognition)
            && !context.state_change.is_read_only())
    {
        return Err(QuarantineError::ProhibitedOperation { operation });
    }

    if matches!(operation, QuarantineOperation::RecordHandoffRecognition) {
        return Ok(Some(HandoffRecognitionRecord {
            surface: context.surface.clone(),
            evidence_hash: context.atom_trail.hash.clone(),
            atom_trail_id: context.atom_trail.id.clone(),
            computation_result: context.conservation.verify(),
            observed_sum: context.conservation.sum,
            from_state: context.state_change.from_state.clone(),
            to_state: context.state_change.to_state.clone(),
        }));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn context() -> QuarantineContext {
        let timestamp = Utc::now();
        let atom_trail = AtomEntry {
            id: "atom-1".into(),
            atom_type: "quarantine_observation".into(),
            gate: "observe".into(),
            description: "read-only α/ω observation".into(),
            coherence: 0.95,
            timestamp,
            prev_hash: "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789".into(),
            hash: String::new(),
        };
        let atom_hash = canonical_atom_hash(&atom_trail);

        QuarantineContext {
            surface: "crates/core/src/superskill.rs".into(),
            atom_trail: AtomEntry {
                hash: atom_hash,
                ..atom_trail
            },
            state_change: ExecutionStateChange {
                from_state: "running".into(),
                to_state: "running".into(),
            },
            conservation: ConservationState::new(7, 8),
        }
    }

    #[test]
    fn allows_read_only_handoff_recognition() {
        let record = enforce_quarantine_boundary(
            Some(&context()),
            QuarantineOperation::RecordHandoffRecognition,
        )
        .expect("read-only handoff recognition stays available")
        .expect("handoff record returned");

        assert!(record.computation_result);
        assert_eq!(record.observed_sum, 15);
        assert_eq!(record.surface, "crates/core/src/superskill.rs");
        assert_eq!(record.atom_trail_id, "atom-1");
        assert_eq!(record.from_state, "running");
        assert_eq!(record.to_state, "running");
    }

    #[test]
    fn rejects_promotion_attempts_even_when_check_passes() {
        let err = enforce_quarantine_boundary(Some(&context()), QuarantineOperation::PromoteState)
            .expect_err("promotion must stay blocked");

        assert_eq!(
            err,
            QuarantineError::ProhibitedOperation {
                operation: QuarantineOperation::PromoteState,
            }
        );
    }

    #[test]
    fn rejects_context_window_mirroring_attempts() {
        let err =
            enforce_quarantine_boundary(Some(&context()), QuarantineOperation::MirrorContextWindow)
                .expect_err("context-window mirroring must stay blocked");

        assert_eq!(
            err,
            QuarantineError::ProhibitedOperation {
                operation: QuarantineOperation::MirrorContextWindow,
            }
        );
    }

    #[test]
    fn rejects_handoff_recognition_when_it_implies_a_state_change() {
        let mut stateful = context();
        stateful.state_change.to_state = "completed".into();

        let err = enforce_quarantine_boundary(
            Some(&stateful),
            QuarantineOperation::RecordHandoffRecognition,
        )
        .expect_err("handoff recognition must stay read-only");

        assert_eq!(
            err,
            QuarantineError::ProhibitedOperation {
                operation: QuarantineOperation::RecordHandoffRecognition,
            }
        );
    }

    #[test]
    fn fails_closed_when_context_is_missing() {
        let err = enforce_quarantine_boundary(None, QuarantineOperation::Inspect)
            .expect_err("missing context must fail closed");
        assert_eq!(err, QuarantineError::MissingContext);
    }

    #[test]
    fn fails_closed_when_context_is_malformed() {
        let mut bad = context();
        bad.atom_trail.hash = "not-a-sha256".into();

        let err = enforce_quarantine_boundary(Some(&bad), QuarantineOperation::Inspect)
            .expect_err("malformed context must fail closed");
        assert_eq!(err, QuarantineError::MalformedContext);
    }
}
