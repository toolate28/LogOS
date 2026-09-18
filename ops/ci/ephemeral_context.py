#!/usr/bin/env python3
"""Capture and validate source-scoped ephemeral context."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from datetime import UTC, datetime, timedelta
from pathlib import Path, PurePosixPath
from typing import Any

SCHEMA_VERSION = "ephemeral-context/v2"
ATOM_TAG = "ATOM-EPHEMERAL-CONTEXT-SOURCE-ROOT-20260917"
SHA256_RE = re.compile(r"^[0-9a-f]{64}$", re.IGNORECASE)
COMMIT_SHA_RE = re.compile(r"^[0-9a-f]{40}$", re.IGNORECASE)
ISO_8601_RE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
ALLOWED_ENVELOPE_FIELDS = {
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
}
REQUIRED_ENVELOPE_FIELDS = ALLOWED_ENVELOPE_FIELDS - {"expires_at"}


class ValidationError(ValueError):
    """Raised when the ephemeral context bundle is malformed or unsafe."""


def utc_now() -> datetime:
    return datetime.now(UTC).replace(microsecond=0)


def isoformat_utc(value: datetime) -> str:
    return value.astimezone(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_iso8601(value: str) -> datetime:
    if not ISO_8601_RE.match(value):
        raise ValidationError(f"invalid ISO-8601 timestamp: {value!r}")
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def canonical_json_bytes(payload: Any) -> bytes:
    return json.dumps(payload, sort_keys=True, indent=2, separators=(",", ": ")).encode("utf-8") + b"\n"


def write_json(path: Path, payload: Any) -> None:
    path.write_bytes(canonical_json_bytes(payload))


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def normalize_repo_relative_path(repo_root: Path, raw_path: str, *, field_name: str) -> tuple[Path, str]:
    if not isinstance(raw_path, str) or not raw_path.strip():
        raise ValidationError(f"{field_name} must be a non-empty path")
    raw = PurePosixPath(raw_path.strip())
    if raw.is_absolute():
        raise ValidationError(f"{field_name} must be repository-relative")
    candidate = (repo_root / Path(*raw.parts)).resolve()
    try:
        candidate.relative_to(repo_root)
    except ValueError as exc:
        raise ValidationError(f"{field_name} escapes repository root") from exc
    if candidate == repo_root:
        raise ValidationError(f"{field_name} must not be the repository root")
    return candidate, candidate.relative_to(repo_root).as_posix()


def collect_source_entries(repo_root: Path, source_root: Path, source_path: Path) -> tuple[str, list[dict[str, str]]]:
    try:
        source_path.relative_to(source_root)
    except ValueError as exc:
        raise ValidationError("source_path is outside source_root") from exc

    if source_path.is_file():
        files = [source_path]
        source_kind = "file"
    elif source_path.is_dir():
        files = sorted(path for path in source_path.rglob("*") if path.is_file())
        if not files:
            raise ValidationError("source_path directory must contain at least one file")
        source_kind = "directory"
    else:
        raise ValidationError("source_path must exist as a file or directory")

    entries = [
        {
            "path": path.relative_to(repo_root).as_posix(),
            "sha256": sha256_file(path),
        }
        for path in files
    ]
    return source_kind, entries


def compute_source_hash(entries: list[dict[str, str]]) -> str:
    return sha256_bytes(canonical_json_bytes(entries))


def build_envelope(
    *,
    commit_sha: str,
    generated_at: str,
    source_root: str,
    source_path: str,
    source_kind: str,
    entries: list[dict[str, str]],
    expires_at: str | None = None,
) -> dict[str, Any]:
    envelope: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION,
        "atom_tag": ATOM_TAG,
        "commit_sha": commit_sha,
        "generated_at": generated_at,
        "source_root": source_root,
        "source_path": source_path,
        "source_kind": source_kind,
        "source_sha256": compute_source_hash(entries),
        "entries": entries,
    }
    if expires_at is not None:
        envelope["expires_at"] = expires_at
    return envelope


def validate_envelope(envelope: dict[str, Any]) -> None:
    if not isinstance(envelope, dict):
        raise ValidationError("envelope must be a JSON object")
    unknown = set(envelope) - ALLOWED_ENVELOPE_FIELDS
    if unknown:
        raise ValidationError(f"envelope contains unknown fields: {sorted(unknown)}")
    missing = REQUIRED_ENVELOPE_FIELDS - set(envelope)
    if missing:
        raise ValidationError(f"envelope missing fields: {sorted(missing)}")
    if envelope["schema_version"] != SCHEMA_VERSION:
        raise ValidationError("unexpected schema_version")
    if envelope["atom_tag"] != ATOM_TAG:
        raise ValidationError("unexpected atom_tag")
    if not COMMIT_SHA_RE.match(str(envelope["commit_sha"])):
        raise ValidationError("invalid commit_sha")
    parse_iso8601(str(envelope["generated_at"]))
    expires_at = envelope.get("expires_at")
    if expires_at is not None:
        generated_at_dt = parse_iso8601(str(envelope["generated_at"]))
        expires_at_dt = parse_iso8601(str(expires_at))
        if expires_at_dt <= generated_at_dt:
            raise ValidationError("expires_at must be after generated_at")
    for field in ("source_root", "source_path"):
        value = envelope[field]
        if not isinstance(value, str) or not value or value.startswith("/") or ".." in PurePosixPath(value).parts:
            raise ValidationError(f"invalid {field}")
    if envelope["source_kind"] not in {"file", "directory"}:
        raise ValidationError("invalid source_kind")
    entries = envelope["entries"]
    if not isinstance(entries, list) or not entries:
        raise ValidationError("entries must be a non-empty list")

    paths: list[str] = []
    for idx, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValidationError(f"entry {idx} must be an object")
        if set(entry) != {"path", "sha256"}:
            raise ValidationError(f"entry {idx} must contain only path and sha256")
        path_value = entry["path"]
        hash_value = entry["sha256"]
        if not isinstance(path_value, str) or not path_value or path_value.startswith("/") or ".." in PurePosixPath(path_value).parts:
            raise ValidationError(f"invalid path at entry {idx}")
        if not isinstance(hash_value, str) or not SHA256_RE.match(hash_value):
            raise ValidationError(f"invalid sha256 at entry {idx}")
        paths.append(path_value)

    if paths != sorted(paths):
        raise ValidationError("entries must be sorted by path")
    source_root = str(envelope["source_root"])
    source_path = str(envelope["source_path"])
    for path_value in paths:
        if path_value != source_path and not path_value.startswith(f"{source_path}/"):
            raise ValidationError("entries include files outside source_path")
        if source_root != source_path and path_value != source_root and not path_value.startswith(f"{source_root}/"):
            raise ValidationError("entries include files outside source_root")

    if compute_source_hash(entries) != envelope["source_sha256"]:
        raise ValidationError("source_sha256 mismatch")


def validate_workflow_file(workflow_path: Path) -> dict[str, str]:
    if not workflow_path.is_file():
        raise ValidationError(f"missing workflow file: {workflow_path}")
    text = workflow_path.read_text(encoding="utf-8")
    compact = "\n".join(line.split("#", 1)[0].rstrip() for line in text.splitlines())
    if "push:" in compact or "pull_request:" in compact:
        raise ValidationError("workflow must not run broad capture on push or pull_request")
    if "workflow_dispatch:" not in compact:
        raise ValidationError("workflow must be manually invoked with workflow_dispatch")
    if "permissions:\n  contents: read" not in compact:
        raise ValidationError("workflow must use contents: read")
    if re.search(r"^\s+[^#\n]+:\s*write(?:-all)?\s*$", compact, re.MULTILINE):
        raise ValidationError("workflow must not request write permissions")
    for required in ("source_root", "source_path"):
        if f"{required}:" not in compact:
            raise ValidationError(f"workflow is missing required input: {required}")
    return {
        "schema_version": SCHEMA_VERSION,
        "workflow_path": workflow_path.as_posix(),
        "trigger": "workflow_dispatch",
        "permissions": "contents:read",
    }


def write_bundle(args: argparse.Namespace) -> int:
    repo_root = Path(args.repo_root).resolve()
    output_dir = Path(args.output_dir).resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    source_root_path, source_root_rel = normalize_repo_relative_path(
        repo_root, args.source_root, field_name="source_root"
    )
    source_path_path, source_path_rel = normalize_repo_relative_path(
        repo_root, args.source_path, field_name="source_path"
    )
    if not source_root_path.exists():
        raise ValidationError("source_root does not exist")
    source_kind, entries = collect_source_entries(repo_root, source_root_path, source_path_path)
    generated_at = isoformat_utc(utc_now())
    expires_at = None
    if args.expiry_hours is not None:
        expires_at = isoformat_utc(parse_iso8601(generated_at) + timedelta(hours=args.expiry_hours))

    envelope = build_envelope(
        commit_sha=args.commit_sha,
        generated_at=generated_at,
        expires_at=expires_at,
        source_root=source_root_rel,
        source_path=source_path_rel,
        source_kind=source_kind,
        entries=entries,
    )
    validate_envelope(envelope)
    validate_workflow_file(Path(args.workflow_path).resolve())

    envelope_path = output_dir / "source-context.json"
    checksum_path = output_dir / "source-context.sha256"
    write_json(envelope_path, envelope)
    checksum_path.write_text(f"{sha256_file(envelope_path)}  source-context.json\n", encoding="utf-8")

    print(f"ephemeral_context: wrote source-scoped context to {output_dir}")
    print("source-context.json")
    print("source-context.sha256")
    return 0


def workflow_guard(args: argparse.Namespace) -> int:
    print(json.dumps(validate_workflow_file(Path(args.workflow_path).resolve()), sort_keys=True, indent=2))
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    capture_parser = subparsers.add_parser("capture", help="capture source-scoped context")
    capture_parser.add_argument("--repo-root", required=True)
    capture_parser.add_argument("--output-dir", required=True)
    capture_parser.add_argument("--workflow-path", required=True)
    capture_parser.add_argument("--source-root", required=True)
    capture_parser.add_argument("--source-path", required=True)
    capture_parser.add_argument("--commit-sha", required=True)
    capture_parser.add_argument("--expiry-hours", required=False, type=int, default=24)
    capture_parser.set_defaults(func=write_bundle)

    guard_parser = subparsers.add_parser("workflow-guard", help="validate workflow shape")
    guard_parser.add_argument("--workflow-path", required=True)
    guard_parser.set_defaults(func=workflow_guard)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return int(args.func(args))
    except ValidationError as exc:
        print(f"::error::{exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
