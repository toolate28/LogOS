#!/usr/bin/env python3
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

import ephemeral_context as ec


class EphemeralContextTests(unittest.TestCase):
    def test_workflow_guard_accepts_expected_shape(self) -> None:
        workflow_path = Path(__file__).resolve().parents[2] / ".github" / "workflows" / "ephemeral-context.yml"
        assertions = ec.validate_workflow_file(workflow_path)
        self.assertEqual(assertions["trigger"], "workflow_dispatch")
        self.assertEqual(assertions["permissions"], "contents:read")

    def test_workflow_guard_rejects_push_trigger(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            workflow_path = Path(tmp) / "bad-workflow.yml"
            workflow_path.write_text(
                "\n".join(
                    [
                        "name: Bad Workflow",
                        "on:",
                        "  push:",
                        "    branches: [main]",
                        "  workflow_dispatch:",
                        "permissions:",
                        "  contents: read",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            with self.assertRaises(ec.ValidationError):
                ec.validate_workflow_file(workflow_path)

    def test_capture_valid_source_file(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            source_file = repo_root / "ops" / "ci" / "ephemeral_context.py"
            source_file.parent.mkdir(parents=True, exist_ok=True)
            source_file.write_text("print('ok')\n", encoding="utf-8")

            source_root, source_root_rel = ec.normalize_repo_relative_path(
                repo_root, "ops/ci", field_name="source_root"
            )
            source_path, source_path_rel = ec.normalize_repo_relative_path(
                repo_root, "ops/ci/ephemeral_context.py", field_name="source_path"
            )
            source_kind, entries = ec.collect_source_entries(repo_root, source_root, source_path)
            envelope = ec.build_envelope(
                commit_sha="a" * 40,
                generated_at=ec.isoformat_utc(ec.utc_now()),
                expires_at=ec.isoformat_utc(ec.utc_now() + ec.timedelta(hours=24)),
                source_root=source_root_rel,
                source_path=source_path_rel,
                source_kind=source_kind,
                entries=entries,
            )

            ec.validate_envelope(envelope)
            self.assertEqual(source_kind, "file")
            self.assertEqual(envelope["source_root"], "ops/ci")
            self.assertEqual(envelope["source_path"], "ops/ci/ephemeral_context.py")
            self.assertEqual(entries, [{"path": "ops/ci/ephemeral_context.py", "sha256": ec.sha256_file(source_file)}])

    def test_capture_bounded_directory_excludes_unrelated_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            bounded_dir = repo_root / "src" / "bounded"
            bounded_dir.mkdir(parents=True, exist_ok=True)
            (bounded_dir / "a.txt").write_text("a\n", encoding="utf-8")
            (bounded_dir / "nested").mkdir()
            (bounded_dir / "nested" / "b.txt").write_text("b\n", encoding="utf-8")
            unrelated = repo_root / "src" / "other.txt"
            unrelated.parent.mkdir(parents=True, exist_ok=True)
            unrelated.write_text("other\n", encoding="utf-8")

            source_root, _ = ec.normalize_repo_relative_path(repo_root, "src/bounded", field_name="source_root")
            source_path, source_path_rel = ec.normalize_repo_relative_path(
                repo_root, "src/bounded", field_name="source_path"
            )
            source_kind, entries = ec.collect_source_entries(repo_root, source_root, source_path)

            self.assertEqual(source_kind, "directory")
            self.assertEqual(source_path_rel, "src/bounded")
            self.assertEqual([entry["path"] for entry in entries], ["src/bounded/a.txt", "src/bounded/nested/b.txt"])
            self.assertNotIn("src/other.txt", [entry["path"] for entry in entries])

    def test_path_normalization_rejects_traversal(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            with self.assertRaises(ec.ValidationError):
                ec.normalize_repo_relative_path(repo_root, "../escape.txt", field_name="source_path")

    def test_path_normalization_rejects_repository_root_scope(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            with self.assertRaises(ec.ValidationError):
                ec.normalize_repo_relative_path(repo_root, ".", field_name="source_root")

    def test_source_root_mismatch_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "src" / "allowed").mkdir(parents=True, exist_ok=True)
            outside = repo_root / "src" / "outside.txt"
            outside.write_text("nope\n", encoding="utf-8")
            source_root, _ = ec.normalize_repo_relative_path(repo_root, "src/allowed", field_name="source_root")
            source_path, _ = ec.normalize_repo_relative_path(repo_root, "src/outside.txt", field_name="source_path")
            with self.assertRaises(ec.ValidationError):
                ec.collect_source_entries(repo_root, source_root, source_path)

    def test_source_hash_integrity_detects_tampering(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            source_file = repo_root / "src" / "module.py"
            source_file.parent.mkdir(parents=True, exist_ok=True)
            source_file.write_text("print('ok')\n", encoding="utf-8")
            source_root, source_root_rel = ec.normalize_repo_relative_path(repo_root, "src", field_name="source_root")
            source_path, source_path_rel = ec.normalize_repo_relative_path(repo_root, "src/module.py", field_name="source_path")
            source_kind, entries = ec.collect_source_entries(repo_root, source_root, source_path)
            envelope = ec.build_envelope(
                commit_sha="b" * 40,
                generated_at=ec.isoformat_utc(ec.utc_now()),
                source_root=source_root_rel,
                source_path=source_path_rel,
                source_kind=source_kind,
                entries=entries,
            )
            envelope["source_sha256"] = "0" * 64
            with self.assertRaises(ec.ValidationError):
                ec.validate_envelope(envelope)

    def test_write_bundle_outputs_only_source_scoped_data(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp) / "repo"
            repo_root.mkdir()
            source_file = repo_root / "docs" / "guide.md"
            source_file.parent.mkdir(parents=True, exist_ok=True)
            source_file.write_text("hello\n", encoding="utf-8")
            workflow_path = repo_root / ".github" / "workflows" / "ephemeral-context.yml"
            workflow_path.parent.mkdir(parents=True, exist_ok=True)
            workflow_path.write_text(
                "\n".join(
                    [
                        "name: Ephemeral Context",
                        "on:",
                        "  workflow_dispatch:",
                        "    inputs:",
                        "      source_root:",
                        "        required: true",
                        "        type: string",
                        "      source_path:",
                        "        required: true",
                        "        type: string",
                        "permissions:",
                        "  contents: read",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            output_dir = Path(tmp) / "out"
            exit_code = ec.write_bundle(
                type(
                    "Args",
                    (),
                    {
                        "repo_root": str(repo_root),
                        "output_dir": str(output_dir),
                        "workflow_path": str(workflow_path),
                        "source_root": "docs",
                        "source_path": "docs/guide.md",
                        "commit_sha": "c" * 40,
                        "expiry_hours": 24,
                    },
                )()
            )
            self.assertEqual(exit_code, 0)
            payload = json.loads((output_dir / "source-context.json").read_text(encoding="utf-8"))
            self.assertEqual(
                set(payload),
                {
                    "schema_version",
                    "atom_tag",
                    "commit_sha",
                    "generated_at",
                    "expires_at",
                    "source_root",
                    "source_path",
                    "source_kind",
                    "source_sha256",
                    "entries",
                },
            )
            self.assertNotIn("repository", payload)
            self.assertNotIn("event", payload)
            self.assertNotIn("run_id", payload)
            self.assertNotIn("actor", payload)
            self.assertEqual(payload["entries"], [{"path": "docs/guide.md", "sha256": ec.sha256_file(source_file)}])


if __name__ == "__main__":
    unittest.main()
