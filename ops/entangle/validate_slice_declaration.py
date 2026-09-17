#!/usr/bin/env python3
"""Fail-closed validation for self-declared entangle slice payloads."""
from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime
from pathlib import Path, PurePosixPath
from typing import Any

DECLARATION_PATH = PurePosixPath("ops/entangle/DECLARATION.json")
REQUIRED_FIELDS = {
    "atom",
    "component_id",
    "title",
    "declared",
    "source_scope",
    "observe_only",
    "mirror",
    "allowed_paths",
    "emitted_files",
    "head",
    "branch",
    "generated",
}
SHA_RE = re.compile(r"^[0-9a-f]{40}$", re.IGNORECASE)
OBSERVE_ONLY_RE = ("observe-only", "observe only")
MIRROR_RE = ("mirror",)
FORBIDDEN_SEGMENTS = {"target", ".git", "node_modules", ".lake", "__pycache__"}


class ValidationError(ValueError):
    pass


def is_safe_relative_path(raw: str) -> bool:
    path = PurePosixPath(raw)
    return bool(raw) and not path.is_absolute() and ".." not in path.parts


def normalize_paths(values: list[str], field: str) -> list[str]:
    normalized: list[str] = []
    for raw in values:
        if not isinstance(raw, str) or not is_safe_relative_path(raw):
            raise ValidationError(f"{field} contains unsafe path {raw!r}")
        normalized.append(PurePosixPath(raw).as_posix().rstrip("/"))
    if normalized != sorted(normalized):
        raise ValidationError(f"{field} must be sorted")
    if len(normalized) != len(set(normalized)):
        raise ValidationError(f"{field} contains duplicates")
    return normalized


def validate_payload(payload: dict[str, Any], root: Path) -> None:
    missing = REQUIRED_FIELDS - set(payload)
    if missing:
        raise ValidationError(f"missing fields {sorted(missing)}")
    if payload["declared"] is not True:
        raise ValidationError("declared must be true")
    if payload["source_scope"] != "repository":
        raise ValidationError("source_scope must be 'repository'")
    if not isinstance(payload["observe_only"], bool) or not isinstance(payload["mirror"], bool):
        raise ValidationError("observe_only and mirror must be booleans")
    if not SHA_RE.match(str(payload["head"])):
        raise ValidationError("head must be a 40-char commit SHA")
    if not isinstance(payload["branch"], str) or not payload["branch"].strip():
        raise ValidationError("branch must be non-empty")
    if not isinstance(payload["generated"], str):
        raise ValidationError("generated must be a timestamp")
    try:
        datetime.fromisoformat(payload["generated"].replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValidationError("generated must be ISO-8601") from exc

    allowed_paths = normalize_paths(list(payload["allowed_paths"]), "allowed_paths")
    emitted_files = normalize_paths(list(payload["emitted_files"]), "emitted_files")
    if not emitted_files:
        raise ValidationError("emitted_files must be non-empty")

    title_text = str(payload["title"]).lower()
    if any(token in title_text for token in OBSERVE_ONLY_RE) and payload["observe_only"] is not True:
        raise ValidationError("observe-only title must declare observe_only=true")
    if any(token in title_text for token in MIRROR_RE) and payload["mirror"] is not True:
        raise ValidationError("mirror title must declare mirror=true")

    for rel_path in emitted_files:
        parts = PurePosixPath(rel_path).parts
        if any(part in FORBIDDEN_SEGMENTS for part in parts):
            raise ValidationError(f"forbidden segment present in emitted_files: {rel_path}")
        if not any(
            rel_path == allowed or rel_path.startswith(f"{allowed}/")
            for allowed in allowed_paths
        ):
            raise ValidationError(f"emitted file escapes allowed_paths: {rel_path}")
        if not (root / rel_path).is_file():
            raise ValidationError(f"declared file missing from slice: {rel_path}")

    actual_files = sorted(
        path.relative_to(root).as_posix()
        for path in root.rglob("*")
        if path.is_file() and path.relative_to(root).as_posix() != DECLARATION_PATH.as_posix()
    )
    if actual_files != emitted_files:
        raise ValidationError("actual files do not match emitted_files declaration")


def load_payload(root: Path) -> dict[str, Any]:
    path = root / DECLARATION_PATH
    if not path.is_file():
        raise ValidationError(f"missing {DECLARATION_PATH.as_posix()}")
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ValidationError("declaration JSON is invalid") from exc
    if not isinstance(payload, dict):
        raise ValidationError("declaration must be a JSON object")
    return payload


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, help="Extracted slice root directory")
    args = parser.parse_args(argv)
    root = Path(args.root).resolve()
    try:
        payload = load_payload(root)
        validate_payload(payload, root)
    except ValidationError as exc:
        print(f"validate_slice_declaration: FAIL {exc}")
        return 1
    print(f"validate_slice_declaration: OK component={payload['component_id']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
