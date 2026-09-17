#!/usr/bin/env python3
from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import validate_manifest as vm


class ValidateManifestTests(unittest.TestCase):
    def make_manifest(self) -> dict:
        return {
            "version": 1,
            "atom": "ATOM-ENTANGLE-MANIFEST-20260809",
            "declaration_policy": {
                "source_scope": "repository",
                "disallow_external_paths": True,
                "require_declared_components": True,
                "require_declared_observe_only": True,
                "require_declared_mirrors": True,
            },
            "components": [
                {
                    "id": "ci-verify",
                    "title": "CI policy + verify pipeline",
                    "declaration": {
                        "declared": True,
                        "source_scope": "repository",
                        "observe_only": False,
                        "mirror": False,
                    },
                    "paths": [".github/workflows", "ops/ci"],
                    "priority": "A",
                }
            ],
            "excludes": ["**/target/**"],
            "remote": {"base": "main", "branch_prefix": "entangle/", "pr_title_template": "x"},
        }

    def test_valid_manifest_data_passes(self) -> None:
        self.assertEqual(vm.validate_manifest_data(self.make_manifest()), [])

    def test_observe_only_keyword_requires_declaration(self) -> None:
        data = self.make_manifest()
        data["components"][0]["notes"] = "residual-zero observe-only"
        data["components"][0]["declaration"]["observe_only"] = False
        errors = vm.validate_manifest_data(data)
        self.assertTrue(any("observe_only=true" in err for err in errors))

    def test_outside_path_is_rejected(self) -> None:
        data = self.make_manifest()
        data["components"][0]["paths"] = ["../outside"]
        errors = vm.validate_manifest_data(data)
        self.assertTrue(any("inside repository" in err for err in errors))


if __name__ == "__main__":
    unittest.main()
