#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import validate_slice_declaration as vsd


class ValidateSliceDeclarationTests(unittest.TestCase):
    def write_payload(self, root: Path, payload: dict) -> None:
        decl_path = root / "ops" / "entangle"
        decl_path.mkdir(parents=True, exist_ok=True)
        (decl_path / "DECLARATION.json").write_text(
            json.dumps(payload, sort_keys=True, indent=2) + "\n",
            encoding="utf-8",
        )

    def make_payload(self) -> dict:
        return {
            "atom": "ATOM-ENTANGLE-MANIFEST-20260809",
            "component_id": "ci-verify",
            "title": "CI policy + verify pipeline",
            "declared": True,
            "source_scope": "repository",
            "observe_only": False,
            "mirror": False,
            "allowed_paths": [".github/workflows", "ops/ci"],
            "emitted_files": [".github/workflows/verify.yml", "ops/ci/assert_action_pins.py"],
            "head": "a" * 40,
            "branch": "feature/test",
            "generated": "2026-09-17T21:00:00Z",
        }

    def test_valid_declaration_passes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / ".github" / "workflows").mkdir(parents=True, exist_ok=True)
            (root / ".github" / "workflows" / "verify.yml").write_text("name: Verify\n", encoding="utf-8")
            (root / "ops" / "ci").mkdir(parents=True, exist_ok=True)
            (root / "ops" / "ci" / "assert_action_pins.py").write_text("print('ok')\n", encoding="utf-8")
            payload = self.make_payload()
            self.write_payload(root, payload)
            vsd.validate_payload(payload, root)

    def test_missing_declaration_file_fails(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(vsd.ValidationError):
                vsd.load_payload(Path(tmp))

    def test_hidden_extra_file_fails(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / ".github" / "workflows").mkdir(parents=True, exist_ok=True)
            (root / ".github" / "workflows" / "verify.yml").write_text("name: Verify\n", encoding="utf-8")
            (root / "ops" / "ci").mkdir(parents=True, exist_ok=True)
            (root / "ops" / "ci" / "assert_action_pins.py").write_text("print('ok')\n", encoding="utf-8")
            (root / "ops" / "ci" / "hidden.py").write_text("print('hidden')\n", encoding="utf-8")
            payload = self.make_payload()
            self.write_payload(root, payload)
            with self.assertRaises(vsd.ValidationError):
                vsd.validate_payload(payload, root)

    def test_outside_path_fails(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            payload = self.make_payload()
            payload["emitted_files"] = ["../outside.txt"]
            self.write_payload(root, payload)
            with self.assertRaises(vsd.ValidationError):
                vsd.validate_payload(payload, root)

    def test_observe_only_requires_explicit_flag(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "ops" / "ci").mkdir(parents=True, exist_ok=True)
            (root / "ops" / "ci" / "assert_action_pins.py").write_text("print('ok')\n", encoding="utf-8")
            payload = self.make_payload()
            payload["title"] = "residual-zero observe-only packet"
            payload["allowed_paths"] = ["ops/ci"]
            payload["emitted_files"] = ["ops/ci/assert_action_pins.py"]
            self.write_payload(root, payload)
            with self.assertRaises(vsd.ValidationError):
                vsd.validate_payload(payload, root)


if __name__ == "__main__":
    unittest.main()
