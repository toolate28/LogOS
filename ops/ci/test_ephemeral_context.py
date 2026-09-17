#!/usr/bin/env python3
from __future__ import annotations

import tempfile
import unittest
from datetime import timedelta
from pathlib import Path

import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

import ephemeral_context as ec


class EphemeralContextTests(unittest.TestCase):
    def test_workflow_guard_accepts_expected_shape(self) -> None:
        workflow_path = Path(__file__).resolve().parents[2] / ".github" / "workflows" / "ephemeral-context.yml"
        assertions = ec.validate_workflow_file(workflow_path)
        self.assertEqual(assertions["permissions"], {"contents": "read", "actions": "read"})

    def test_workflow_guard_rejects_job_level_id_token_write(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            workflow_path = Path(tmp) / "bad-workflow.yml"
            workflow_path.write_text(
                "\n".join(
                    [
                        "name: Bad Workflow",
                        "on: workflow_dispatch",
                        "permissions:",
                        "  contents: read",
                        "  actions: read",
                        "jobs:",
                        "  example:",
                        "    runs-on: ubuntu-latest",
                        "    permissions:",
                        "      id-token: write",
                        "    steps:",
                        "      - run: echo bad",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            with self.assertRaises(ec.ValidationError):
                ec.validate_workflow_file(workflow_path)

    def test_valid_context_and_manifest(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="pull_request",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/pull/106/merge",
            actor="toolate28",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        human_context = ec.extract_human_context(
            "pull_request",
            {
                "number": 106,
                "pull_request": {
                    "number": 106,
                    "user": {"login": "toolate28"},
                    "body": "bounded narrative only",
                },
            },
        )
        self.assertEqual(human_context[0]["classification"], "USER_AUTHORED_NARRATIVE")
        self.assertIn("body_sha256", human_context[0])
        self.assertNotIn("body", human_context[0])

        ec.validate_envelope(envelope, "toolate28/LogOS")
        ec.validate_human_context(human_context)

        with tempfile.TemporaryDirectory() as tmp:
            output_dir = Path(tmp)
            envelope_path = output_dir / "context-envelope.json"
            human_context_path = output_dir / "human-context.json"
            workflow_assertions_path = output_dir / "workflow-assertions.json"
            ec.write_json(envelope_path, envelope)
            ec.write_json(human_context_path, human_context)
            ec.write_json(
                workflow_assertions_path,
                {
                    "schema_version": ec.WORKFLOW_SCHEMA_VERSION,
                    "workflow_path": ".github/workflows/ephemeral-context.yml",
                    "permissions": {"contents": "read", "actions": "read"},
                    "promotion_job_present": False,
                    "deployment_permission_present": False,
                    "write_permission_present": False,
                    "override_flag_present": False,
                    "state_transition_authority_present": False,
                    "authority_effect": "none",
                },
            )
            manifest = ec.build_manifest(
                output_dir,
                commit_sha=envelope["commit_sha"],
                run_id=envelope["run_id"],
                generated_at=envelope["generated_at"],
                expires_at=envelope["expires_at"],
                evidence_paths=[envelope_path, human_context_path, workflow_assertions_path],
            )
            ec.validate_manifest(manifest, output_dir, envelope)

    def test_issue_comment_uses_comment_body_hash(self) -> None:
        human_context = ec.extract_human_context(
            "issue_comment",
            {
                "issue": {
                    "number": 42,
                    "user": {"login": "issue-author"},
                    "body": "issue body should not be used",
                },
                "comment": {
                    "user": {"login": "comment-author"},
                    "body": "comment body should be hashed",
                },
            },
        )
        self.assertEqual(human_context[0]["author_login"], "comment-author")
        self.assertEqual(human_context[0]["number"], 42)
        self.assertEqual(human_context[0]["body_sha256"], ec.sha256_text("comment body should be hashed"))

    def test_expired_context_fails(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at="2026-09-18T19:30:00Z",
            expires_at="2026-09-17T19:30:00Z",
        )
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_already_expired_context_fails(self) -> None:
        generated_at = ec.isoformat_utc(ec.utc_now() - timedelta(days=2))
        expires_at = ec.isoformat_utc(ec.utc_now() - timedelta(days=1))
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at=generated_at,
            expires_at=expires_at,
        )
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_missing_fields_fail(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        envelope.pop("actor")
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_repository_mismatch_fails(self) -> None:
        envelope = ec.build_envelope(
            repository="other/repo",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_authority_fields_are_rejected(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        envelope["promotion_authority"] = "yes"
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_secret_like_content_is_rejected(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZ123456",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        with self.assertRaises(ec.ValidationError):
            ec.validate_envelope(envelope, "toolate28/LogOS")

    def test_alpha_omega_authority_language_is_rejected(self) -> None:
        human_context = [
            {
                "classification": "OBSERVATION",
                "event": "pull_request",
                "note": "α + ω authorizes promotion",
            }
        ]
        with self.assertRaises(ec.ValidationError):
            ec.validate_human_context(human_context)

    def test_manifest_path_traversal_is_rejected(self) -> None:
        envelope = ec.build_envelope(
            repository="toolate28/LogOS",
            event_name="push",
            run_id="123456",
            commit_sha="a" * 40,
            ref="refs/heads/main",
            actor="toolate28",
            generated_at="2026-09-17T19:30:00Z",
            expires_at="2026-09-18T19:30:00Z",
        )
        with tempfile.TemporaryDirectory() as tmp:
            output_dir = Path(tmp)
            safe_file = output_dir / "context-envelope.json"
            ec.write_json(safe_file, envelope)
            outside_file = output_dir.parent / "escaped.txt"
            outside_file.write_text("outside", encoding="utf-8")
            manifest = {
                "schema_version": ec.MANIFEST_SCHEMA_VERSION,
                "commit_sha": envelope["commit_sha"],
                "workflow_run_id": envelope["run_id"],
                "generated_at": envelope["generated_at"],
                "expires_at": envelope["expires_at"],
                "entries": [
                    {
                        "path": "../escaped.txt",
                        "sha256": ec.sha256_file(outside_file),
                    }
                ],
            }
            with self.assertRaises(ec.ValidationError):
                ec.validate_manifest(manifest, output_dir, envelope)


if __name__ == "__main__":
    unittest.main()
