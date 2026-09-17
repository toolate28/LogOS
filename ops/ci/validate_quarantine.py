#!/usr/bin/env python3
"""Validate the α/ω quarantine manifest and scanned scope."""
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "ops" / "quarantine" / "alpha-omega-quarantine.json"
REQUIRED_TOP = {
    "version",
    "status",
    "fail_closed",
    "policy_path",
    "manifest_path",
    "surfaces",
    "scan_rules",
}
REQUIRED_SURFACE = {
    "id",
    "paths",
    "capabilities",
    "allowed_read_only_operations",
    "prohibited_operations",
    "status",
    "scan",
}
READ_ONLY_OPS = {"inspect", "run_safe_tests", "record_handoff_recognition"}
PROHIBITED_OPS = {
    "authorize_state_transition",
    "promote_state",
    "publish_artifact",
    "deploy_artifact",
    "generate_semantic_claim",
    "certify_artifact",
    "override_acceptance",
}


def load_manifest(root: Path = ROOT) -> dict:
    manifest_path = root / "ops" / "quarantine" / "alpha-omega-quarantine.json"
    if not manifest_path.is_file():
        raise ValueError(f"missing manifest: {manifest_path}")
    with manifest_path.open(encoding="utf-8") as handle:
        data = json.load(handle)
    if not isinstance(data, dict):
        raise ValueError("manifest root must be an object")
    return data


def tracked_files(root: Path = ROOT) -> set[str]:
    try:
        proc = subprocess.run(
            ["git", "ls-files", "--cached", "--others", "--exclude-standard"],
            cwd=root,
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError):
        return set()
    return {line.strip() for line in proc.stdout.splitlines() if line.strip()}


def validate_manifest(manifest: dict, root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    missing = REQUIRED_TOP - set(manifest)
    if missing:
        errors.append(f"manifest missing keys: {sorted(missing)}")
        return errors

    if manifest["status"] != "quarantined":
        errors.append("manifest status must be 'quarantined'")
    if manifest["fail_closed"] is not True:
        errors.append("manifest must set fail_closed=true")
    if manifest["policy_path"] != "docs/epistemics/ALPHA-OMEGA-QUARANTINE.md":
        errors.append("policy_path must point to the quarantine policy")
    if manifest["manifest_path"] != "ops/quarantine/alpha-omega-quarantine.json":
        errors.append("manifest_path must self-reference the quarantine manifest")

    surfaces = manifest["surfaces"]
    if not isinstance(surfaces, list) or not surfaces:
        errors.append("surfaces must be a non-empty list")
        return errors

    tracked = tracked_files(root)
    seen_ids: set[str] = set()
    seen_paths: set[str] = set()
    for idx, surface in enumerate(surfaces):
        if not isinstance(surface, dict):
            errors.append(f"surface[{idx}] must be an object")
            continue
        missing_surface = REQUIRED_SURFACE - set(surface)
        if missing_surface:
            errors.append(f"surface[{idx}] missing keys: {sorted(missing_surface)}")
            continue

        surface_id = surface["id"]
        if surface_id in seen_ids:
            errors.append(f"duplicate surface id: {surface_id}")
        seen_ids.add(surface_id)

        if surface["status"] != "quarantined":
            errors.append(f"{surface_id}: status must be 'quarantined'")

        paths = surface["paths"]
        if not isinstance(paths, list) or not paths:
            errors.append(f"{surface_id}: paths must be a non-empty list")
            continue

        allowed = set(surface["allowed_read_only_operations"])
        blocked = set(surface["prohibited_operations"])
        if not allowed <= READ_ONLY_OPS:
            errors.append(f"{surface_id}: unknown allowed operation declared")
        if not blocked <= PROHIBITED_OPS:
            errors.append(f"{surface_id}: unknown prohibited operation declared")
        if "authorize_state_transition" not in blocked:
            errors.append(f"{surface_id}: must prohibit authorize_state_transition")

        for rel_path in paths:
            if rel_path in seen_paths:
                errors.append(f"duplicate path across surfaces: {rel_path}")
            seen_paths.add(rel_path)
            if not (root / rel_path).exists():
                errors.append(f"{surface_id}: missing path {rel_path}")
            if tracked and rel_path not in tracked:
                errors.append(f"{surface_id}: untracked path {rel_path}")

    scan_rules = manifest["scan_rules"]
    if not isinstance(scan_rules, dict):
        errors.append("scan_rules must be an object")
        return errors

    for key in ("authority_patterns", "artifact_inscription_patterns", "artifact_extensions", "exempt_paths"):
        if key not in scan_rules or not isinstance(scan_rules[key], list) or not scan_rules[key]:
            errors.append(f"scan_rules.{key} must be a non-empty list")
    for pattern in scan_rules.get("authority_patterns", []):
        try:
            re.compile(pattern, re.IGNORECASE | re.DOTALL)
        except re.error as exc:
            errors.append(f"invalid authority pattern {pattern!r}: {exc}")
    for pattern in scan_rules.get("artifact_inscription_patterns", []):
        try:
            re.compile(pattern, re.IGNORECASE)
        except re.error as exc:
            errors.append(f"invalid artifact pattern {pattern!r}: {exc}")

    return errors


def scan_quarantined_files(manifest: dict, root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    tracked = tracked_files(root)
    rules = manifest["scan_rules"]
    exempt_paths = set(rules["exempt_paths"])
    authority_patterns = [re.compile(pattern, re.IGNORECASE) for pattern in rules["authority_patterns"]]
    artifact_patterns = [
        re.compile(pattern, re.IGNORECASE)
        for pattern in rules["artifact_inscription_patterns"]
    ]
    artifact_extensions = set(rules["artifact_extensions"])

    for surface in manifest["surfaces"]:
        if not surface["scan"]:
            continue
        for rel_path in surface["paths"]:
            if rel_path in exempt_paths or (tracked and rel_path not in tracked):
                continue
            file_path = root / rel_path
            text = file_path.read_text(encoding="utf-8")
            for line_no, line in enumerate(text.splitlines(), start=1):
                for pattern in authority_patterns:
                    if pattern.search(line):
                        errors.append(
                            f"{rel_path}:{line_no}: prohibited authority language matched {pattern.pattern!r}"
                        )
            if file_path.suffix in artifact_extensions:
                for pattern in artifact_patterns:
                    if pattern.search(text):
                        errors.append(f"{rel_path}: prohibited artifact inscription matched {pattern.pattern!r}")
    return errors


def main(root: Path = ROOT) -> int:
    try:
        manifest = load_manifest(root)
    except (ValueError, json.JSONDecodeError) as exc:
        print(f"validate_quarantine: FAIL {exc}")
        return 1

    errors = validate_manifest(manifest, root)
    errors.extend(scan_quarantined_files(manifest, root))
    if errors:
        for error in errors:
            print(f"validate_quarantine: {error}")
        print(f"validate_quarantine: FAIL errors={len(errors)}")
        return 1
    print("validate_quarantine: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
