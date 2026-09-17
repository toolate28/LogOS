from __future__ import annotations

import copy
import hashlib
import tempfile
import unittest
from pathlib import Path

from ops.ci import entangle_observe as eo


class EntangleObserveTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        (self.root / "docs/notebooklm").mkdir(parents=True)
        (self.root / "docs/ops").mkdir(parents=True)
        (self.root / "docs/encyclopedia-equilibria/surfaces").mkdir(parents=True)
        (self.root / "ops/ci").mkdir(parents=True)
        self.nb_path = self.root / "docs/notebooklm/sample.txt"
        self.nb_path.write_text("NotebookLM closure snapshot\n", encoding="utf-8")
        self.grok_path = self.root / "docs/ops/grok.md"
        self.grok_path.write_text("Grok collection snapshot\n", encoding="utf-8")
        self.surface_path = self.root / "docs/encyclopedia-equilibria/surfaces/sample.html"
        self.surface_path.write_text("<html>surface</html>\n", encoding="utf-8")
        (self.root / "ops/ci/codex_scan.py").write_text("print('codex')\n", encoding="utf-8")
        self.manifest = {
            "version": 1,
            "atom": "ATOM-ENTANGLE-MANIFEST-20260809",
            "component_classes": {
                "visual-observation": {
                    "admissible_sources": [
                        "youtube",
                        "notebooklm",
                        "grokCollections",
                        "googleDrive",
                        "githubDiscussion",
                    ],
                    "verify": ["python ops/ci/entangle_observe.py reconcile --help"],
                }
            },
            "source_bindings": {
                "youtube": {"trust_class": "observational-media", "authority_effect": "none", "required_fields": ["videoId", "segments"]},
                "notebooklm": {"trust_class": "secondary-synthesis", "authority_effect": "none", "required_fields": ["documentId", "snapshotPath", "snapshotSha256"]},
                "grokCollections": {"trust_class": "clustered-commentary", "authority_effect": "none", "required_fields": ["collectionId", "snapshotPath", "snapshotSha256"]},
                "googleDrive": {"trust_class": "bulk-archive", "authority_effect": "none", "required_fields": ["objectId", "sha256", "pathHint"]},
                "githubDiscussion": {"trust_class": "narrow-waist-coordination", "authority_effect": "none", "required_fields": ["number", "url"]},
            },
            "invariant_profiles": {
                "norton-sakuma-3a": {
                    "allowed_involutions": ["swap-observer-observed"],
                    "return_conditions": ["double-normalization-fixed-point"],
                    "noise_erasures": ["timestamp-jitter"],
                    "failure_states": ["component-path-drift"],
                }
            },
            "closure_states": eo.REQUIRED_STATES,
            "narrow_wormhole": {"authority_effect": "none", "discussion_required": True, "ephemeral_root_required": True},
            "components": [
                {
                    "id": "visual-observation",
                    "class": "visual-observation",
                    "title": "Video-linked observation envelopes",
                    "paths": ["docs/notebooklm/", "docs/ops/", "docs/encyclopedia-equilibria/surfaces/"],
                    "priority": "A",
                    "admissible_sources": [
                        "youtube",
                        "notebooklm",
                        "grokCollections",
                        "googleDrive",
                        "githubDiscussion",
                    ],
                    "invariant_profile": "norton-sakuma-3a",
                }
            ],
            "excludes": ["**/target/**"],
            "remote": {"base": "main", "branch_prefix": "entangle/", "pr_title_template": "entangle({id}): {title}"},
        }
        eo.ROOT = self.root
        self.envelope = {
            "schemaVersion": "0.1.0",
            "kind": "entangle_observation_envelope",
            "component": {"id": "visual-observation", "class": "visual-observation"},
            "code": {
                "commit": "deadbeef",
                "ref": "refs/heads/main",
                "paths": [
                    "docs/notebooklm/sample.txt",
                    "docs/ops/grok.md",
                    "docs/encyclopedia-equilibria/surfaces/sample.html",
                ],
            },
            "wormhole": {
                "mode": "narrow-wormhole",
                "ephemeralRoot": "runner://narrow-wormhole",
                "discussion": {"number": 47, "url": "https://github.com/toolate28/LogOS/discussions/47"},
            },
            "sources": {
                "youtube": [
                    {
                        "videoId": "abc123xyz89",
                        "url": "https://www.youtube.com/watch?v=abc123xyz89",
                        "segments": [{"start": "00:00:10", "end": "00:01:10"}],
                        "transcriptSha256": "a" * 64,
                    }
                ],
                "notebooklm": [
                    {
                        "documentId": "nb-1",
                        "snapshotPath": "docs/notebooklm/sample.txt",
                        "snapshotSha256": hashlib.sha256(self.nb_path.read_bytes()).hexdigest(),
                    }
                ],
                "grokCollections": [
                    {
                        "collectionId": "grok-1",
                        "snapshotPath": "docs/ops/grok.md",
                        "snapshotSha256": hashlib.sha256(self.grok_path.read_bytes()).hexdigest(),
                    }
                ],
                "googleDrive": [
                    {
                        "objectId": "drive-1",
                        "sha256": "b" * 64,
                        "pathHint": "My Drive/Reson8_Labs/example.pdf",
                    }
                ],
                "githubDiscussion": [
                    {
                        "number": 47,
                        "url": "https://github.com/toolate28/LogOS/discussions/47",
                    }
                ],
            },
            "trust": {
                "authorityEffect": "none",
                "sourceClasses": {
                    "youtube": "observational-media",
                    "notebooklm": "secondary-synthesis",
                    "grokCollections": "clustered-commentary",
                    "googleDrive": "bulk-archive",
                    "githubDiscussion": "narrow-waist-coordination",
                },
            },
            "operator": {
                "principal": "toolate28",
                "capturedAt": "2026-09-17T20:00:00Z",
                "purpose": "Bind code and visual observation under a narrow wormhole.",
            },
            "noisePolicy": {
                "profile": "norton-sakuma-3a",
                "erasures": [
                    "timestamp-jitter",
                    "ocr-noise",
                    "ui-chrome",
                    "duplicate-semantic-fragments",
                ],
                "involutionTransforms": ["swap-observer-observed"],
                "returnConditions": ["double-normalization-fixed-point"],
                "duplicateFragmentBudget": 0.5,
            },
            "observations": [
                {
                    "id": "obs-1",
                    "kind": "visual-claim",
                    "summary": "00:01 [Music] Lattice holds. Lattice holds.",
                    "transcript": "00:02 subscribe Lattice holds. Lattice holds.",
                    "evidence": ["youtube:0", "notebooklm:0", "grokCollections:0", "githubDiscussion:0"],
                }
            ],
            "state": "observed",
            "handoff": {"conservationTag": "α + ω = 15", "category": "C"},
        }

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_capture_normalizes_and_injects_ephemeral_root(self) -> None:
        normalized = eo.capture_envelope(
            self.manifest,
            copy.deepcopy(self.envelope),
            ephemeral_root="/tmp/narrow-wormhole",
            discussion_number=52,
        )
        self.assertEqual(normalized["state"], "normalized")
        self.assertEqual(normalized["wormhole"]["discussion"]["number"], 52)
        self.assertIn("Lattice holds", normalized["observations"][0]["summary"])
        self.assertLessEqual(normalized["normalization"]["duplicateFragmentRatio"], 0.5)

    def test_capture_rejects_authority_effect(self) -> None:
        bad = copy.deepcopy(self.envelope)
        bad["trust"]["authorityEffect"] = "promote"
        with self.assertRaises(eo.ValidationError):
            eo.capture_envelope(self.manifest, bad, ephemeral_root="/tmp/narrow-wormhole")

    def test_reconcile_rejects_path_drift(self) -> None:
        normalized = eo.capture_envelope(self.manifest, copy.deepcopy(self.envelope), ephemeral_root="/tmp/narrow-wormhole")
        normalized["code"]["paths"].append("README.md")
        report = eo.reconcile_envelope(self.manifest, normalized)
        self.assertEqual(report["state"], "drifted")
        self.assertIn("component-path-drift", report["blockers"])

    def test_close_emits_closure_ready(self) -> None:
        normalized = eo.capture_envelope(self.manifest, copy.deepcopy(self.envelope), ephemeral_root="/tmp/narrow-wormhole")
        report = eo.reconcile_envelope(self.manifest, normalized)
        closure = eo.close_envelope(normalized, report)
        self.assertEqual(closure["state"], "closure-ready")
        self.assertEqual(closure["stateHistory"][-1], "closure-ready")


if __name__ == "__main__":
    unittest.main()
