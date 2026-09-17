from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from validate_quarantine import load_manifest, scan_quarantined_files, validate_manifest


class ValidateQuarantineTests(unittest.TestCase):
    def test_repository_manifest_validates(self) -> None:
        manifest = load_manifest()
        self.assertEqual(validate_manifest(manifest), [])

    def test_scan_rejects_prohibited_authority_language(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "docs" / "epistemics").mkdir(parents=True)
            (root / "ops" / "quarantine").mkdir(parents=True)
            (root / "crates" / "core" / "src").mkdir(parents=True)
            (root / "docs" / "epistemics" / "ALPHA-OMEGA-QUARANTINE.md").write_text(
                "policy", encoding="utf-8"
            )
            target = root / "crates" / "core" / "src" / "lib.rs"
            target.write_text(
                "alpha metadata may authorize deployment\n",
                encoding="utf-8",
            )

            manifest = {
                "version": 1,
                "status": "quarantined",
                "fail_closed": True,
                "policy_path": "docs/epistemics/ALPHA-OMEGA-QUARANTINE.md",
                "manifest_path": "ops/quarantine/alpha-omega-quarantine.json",
                "surfaces": [
                    {
                        "id": "core",
                        "paths": ["crates/core/src/lib.rs"],
                        "capabilities": ["compute_alpha_omega_check"],
                        "allowed_read_only_operations": ["inspect"],
                        "prohibited_operations": ["authorize_state_transition"],
                        "status": "quarantined",
                        "scan": True,
                    }
                ],
                "scan_rules": {
                    "tracked_only": True,
                    "authority_patterns": [
                        "(alpha|omega|conservation|invariant).{0,40}(authori[sz]e|deploy)"
                    ],
                    "artifact_inscription_patterns": ["alpha\\s*\\+\\s*omega\\s*=\\s*15"],
                    "artifact_extensions": [".json"],
                    "exempt_paths": [
                        "docs/epistemics/ALPHA-OMEGA-QUARANTINE.md",
                        "ops/quarantine/alpha-omega-quarantine.json",
                    ],
                },
            }
            (root / "ops" / "quarantine" / "alpha-omega-quarantine.json").write_text(
                json.dumps(manifest), encoding="utf-8"
            )

            subprocess.run(["git", "init"], cwd=root, check=True, capture_output=True)
            subprocess.run(["git", "add", "."], cwd=root, check=True, capture_output=True)

            errors = scan_quarantined_files(manifest, root)
            self.assertEqual(len(errors), 1)
            self.assertIn("prohibited authority language", errors[0])


if __name__ == "__main__":
    unittest.main()
