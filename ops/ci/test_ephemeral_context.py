from __future__ import annotations

import copy
import datetime as dt
import json
import sys
import tempfile
import unittest
from pathlib import Path

THIS_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(THIS_DIR))

from ephemeral_context import ValidationError, compute_context_hash, validate_envelope  # noqa: E402
from uncertainty_bands import assess_uncertainty  # noqa: E402


def _ts(text: str) -> dt.datetime:
    return dt.datetime.fromisoformat(text.replace("Z", "+00:00"))


class EphemeralContextTests(unittest.TestCase):
    def setUp(self) -> None:
        self.now = _ts("2026-09-17T19:30:00Z")
        self.valid = {
            "schema_version": "1.0.0",
            "source_repository": "toolate28/LogOS",
            "event": {
                "name": "pull_request",
                "event_id": "run-1234",
                "workflow_run_id": 1234,
                "workflow_run_attempt": 1,
                "workflow_job": "ephemeral-context",
                "pull_request_number": 106,
                "discussion_number": None,
                "comment_id": None,
                "ref": "refs/pull/106/merge"
            },
            "commit": {
                "sha": "a" * 40,
                "ref": "refs/pull/106/merge"
            },
            "observations": [
                {
                    "kind": "github_event",
                    "source": "github_actions",
                    "summary": "GitHub pull_request observation captured as untrusted ephemeral context.",
                    "truth_status": "observed",
                    "authority_effect": "none",
                    "observed_at": "2026-09-17T19:30:00Z",
                    "evidence_refs": ["github_event_payload"]
                }
            ],
            "generated_at": "2026-09-17T19:30:00Z",
            "expires_at": "2026-09-17T19:40:00Z",
            "parent_context_hash": None,
            "context_hash": "",
            "authority_effect": "none",
            "evidence_hashes": [
                {
                    "label": "github_event_payload",
                    "sha256": "b" * 64,
                    "origin": "event.json"
                }
            ],
            "uncertainty": assess_uncertainty(
                stress=0.10,
                strain=0.20,
                provenance_confidence=0.90,
                crosscheck_confidence=0.85,
                rollback_available=True,
            ).as_dict(),
        }
        self.valid["context_hash"] = compute_context_hash(self.valid)

    def test_valid_context_passes(self) -> None:
        result = validate_envelope(self.valid, expected_repository="toolate28/LogOS", now=self.now)
        self.assertEqual(result["execution_band"], self.valid["uncertainty"]["execution_band"])

    def test_expired_context_fails_closed(self) -> None:
        invalid = copy.deepcopy(self.valid)
        with self.assertRaisesRegex(ValidationError, "context expired"):
            validate_envelope(
                invalid,
                expected_repository="toolate28/LogOS",
                now=_ts("2026-09-17T19:45:00Z"),
            )

    def test_repository_binding_mismatch_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValidationError, "repository mismatch"):
            validate_envelope(self.valid, expected_repository="toolate28/Elsewhere", now=self.now)

    def test_hash_mismatch_is_rejected(self) -> None:
        invalid = copy.deepcopy(self.valid)
        invalid["commit"]["ref"] = "refs/heads/main"
        with self.assertRaisesRegex(ValidationError, "context_hash mismatch"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_replay_protection_rejects_seen_hash(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            seen = Path(tmpdir) / "seen.json"
            seen.write_text(json.dumps([self.valid["context_hash"]]), encoding="utf-8")
            with self.assertRaisesRegex(ValidationError, "replayed context hash rejected"):
                validate_envelope(
                    self.valid,
                    expected_repository="toolate28/LogOS",
                    now=self.now,
                    seen_hashes_file=seen,
                )

    def test_missing_required_fields_are_rejected(self) -> None:
        invalid = copy.deepcopy(self.valid)
        del invalid["expires_at"]
        with self.assertRaisesRegex(ValidationError, "missing required fields"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_missing_uncertainty_is_rejected(self) -> None:
        invalid = copy.deepcopy(self.valid)
        del invalid["uncertainty"]
        with self.assertRaisesRegex(ValidationError, "missing required fields"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_authority_only_transition_fields_are_rejected(self) -> None:
        invalid = copy.deepcopy(self.valid)
        invalid["observations"][0]["approve"] = True
        invalid["context_hash"] = compute_context_hash(invalid)
        with self.assertRaisesRegex(ValidationError, "authority-bearing field rejected"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_narrative_must_remain_unverified(self) -> None:
        invalid = copy.deepcopy(self.valid)
        invalid["observations"][0]["narrative"] = "LGTM"
        invalid["observations"][0]["truth_status"] = "observed"
        invalid["context_hash"] = compute_context_hash(invalid)
        with self.assertRaisesRegex(ValidationError, "narrative observations must use truth_status=unverified"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_secret_like_narrative_is_rejected(self) -> None:
        invalid = copy.deepcopy(self.valid)
        invalid["observations"][0]["narrative"] = "sk-1234567890abcdefghijklmnop"
        invalid["observations"][0]["truth_status"] = "unverified"
        invalid["context_hash"] = compute_context_hash(invalid)
        with self.assertRaisesRegex(ValidationError, "secret-like narrative rejected"):
            validate_envelope(invalid, expected_repository="toolate28/LogOS", now=self.now)

    def test_missing_signals_increase_restriction(self) -> None:
        assessment = assess_uncertainty(
            stress=None,
            strain=None,
            provenance_confidence=None,
            crosscheck_confidence=None,
            rollback_available=None,
        )
        self.assertEqual(assessment.execution_band, "fully_restricted")
        self.assertGreaterEqual(assessment.uncertainty_score, 0.80)


if __name__ == "__main__":
    unittest.main()
