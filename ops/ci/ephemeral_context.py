#!/usr/bin/env python3
"""Capture and validate ephemeral zero-trust shared-state evidence."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from datetime import UTC, datetime, timedelta
from pathlib import Path
from typing import Any

SCHEMA_VERSION = "ephemeral-context/v1"
MANIFEST_SCHEMA_VERSION = "ephemeral-context-manifest/v1"
WORKFLOW_SCHEMA_VERSION = "ephemeral-context-workflow/v1"
ALLOWED_SOURCE_CLASSIFICATIONS = {"OBSERVATION", "USER_AUTHORED_NARRATIVE"}
ALLOWED_HUMAN_CLASSIFICATIONS = {"OBSERVATION", "USER_AUTHORED_NARRATIVE"}
SHA256_RE = re.compile(r"^[0-9a-f]{64}$", re.IGNORECASE)
COMMIT_SHA_RE = re.compile(r"^[0-9a-f]{40}$", re.IGNORECASE)
RUN_ID_RE = re.compile(r"^[0-9]+$")
ISO_8601_RE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
FORBIDDEN_KEY_RE = re.compile(
    r"(promotion|approve|deployment|environment|publish|override|emergency|"
    r"state[_-]?transition|merge_authority|execution_authority|allowed_execution)",
    re.IGNORECASE,
)
FORBIDDEN_VALUE_RE = re.compile(
    r"\b(promot(e|ion)|approve|deployment|publish|override|force|emergency|"
    r"state[_ -]?transition|authority|proof|authentication|authorization|"
    r"execution authority|allowed execution)\b",
    re.IGNORECASE,
)
ALPHA_OMEGA_AUTHORITY_RE = re.compile(
    r"(α|ω|alpha|omega).{0,80}\b(authority|authorize|approval|approve|promot(e|ion)|"
    r"proof|authenticate|authentication|deploy|merge)\b|"
    r"\b(authority|authorize|approval|approve|promot(e|ion)|proof|authenticate|"
    r"authentication|deploy|merge)\b.{0,80}(α|ω|alpha|omega)",
    re.IGNORECASE | re.DOTALL,
)
BODY_CONTENT_KEYS = {
    "body",
    "comment_body",
    "pull_request_body",
    "issue_body",
    "raw_body",
    "raw_event",
    "event_payload",
}
SECRET_PATTERNS = (
    re.compile(r"gh[pousr]_[A-Za-z0-9]{20,}"),
    re.compile(r"github_pat_[A-Za-z0-9_]{20,}"),
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"AIza[0-9A-Za-z\-_]{35}"),
    re.compile(r"xox[baprs]-[A-Za-z0-9-]{10,}"),
    re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----"),
    re.compile(r"(?i)\bbearer\s+[A-Za-z0-9._=-]{20,}\b"),
)
REQUIRED_ENVELOPE_FIELDS = {
    "schema_version",
    "repository",
    "event",
    "run_id",
    "commit_sha",
    "ref",
    "actor",
    "generated_at",
    "expires_at",
    "source_classification",
    "authority_effect",
}
REQUIRED_MANIFEST_FIELDS = {
    "schema_version",
    "commit_sha",
    "workflow_run_id",
    "generated_at",
    "expires_at",
    "entries",
}


class ValidationError(ValueError):
    """Raised when evidence is malformed or unsafe."""


def utc_now() -> datetime:
    return datetime.now(UTC).replace(microsecond=0)


def isoformat_utc(value: datetime) -> str:
    return value.astimezone(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_iso8601(value: str) -> datetime:
    if not ISO_8601_RE.match(value):
        raise ValidationError(f"invalid ISO-8601 timestamp: {value!r}")
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def canonical_json_bytes(payload: Any) -> bytes:
    return json.dumps(payload, sort_keys=True, indent=2, separators=(",", ": ")).encode("utf-8") + b"\n"


def write_json(path: Path, payload: Any) -> None:
    path.write_bytes(canonical_json_bytes(payload))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_event(path: Path) -> dict[str, Any]:
    if not path.is_file():
        raise ValidationError(f"missing event payload: {path}")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValidationError("event payload must be a JSON object")
    return data


def extract_human_context(event_name: str, event: dict[str, Any]) -> list[dict[str, Any]]:
    entries: list[dict[str, Any]] = []
    narrative: dict[str, Any] | None = None
    if event_name == "pull_request":
        pull_request = event.get("pull_request")
        if isinstance(pull_request, dict):
            narrative = {
                "classification": "USER_AUTHORED_NARRATIVE",
                "event": event_name,
                "number": pull_request.get("number") or event.get("number"),
                "author_login": ((pull_request.get("user") or {}).get("login")),
            }
            body = pull_request.get("body")
            if isinstance(body, str) and body:
                narrative["body_sha256"] = sha256_text(body)
            else:
                narrative["classification"] = "OBSERVATION"
    elif event_name == "issue_comment":
        issue = event.get("issue")
        comment = event.get("comment")
        if isinstance(issue, dict) and isinstance(comment, dict):
            narrative = {
                "classification": "USER_AUTHORED_NARRATIVE",
                "event": event_name,
                "number": issue.get("number") or event.get("number"),
                "author_login": ((comment.get("user") or {}).get("login")),
            }
            body = comment.get("body")
            if isinstance(body, str) and body:
                narrative["body_sha256"] = sha256_text(body)
            else:
                narrative["classification"] = "OBSERVATION"
    elif event_name == "issues":
        issue = event.get("issue")
        if isinstance(issue, dict):
            narrative = {
                "classification": "USER_AUTHORED_NARRATIVE",
                "event": event_name,
                "number": issue.get("number") or event.get("number"),
                "author_login": ((issue.get("user") or {}).get("login")),
            }
            body = issue.get("body")
            if isinstance(body, str) and body:
                narrative["body_sha256"] = sha256_text(body)
            else:
                narrative["classification"] = "OBSERVATION"
    if narrative:
        entries.append({k: v for k, v in narrative.items() if v not in (None, "")})
    return entries


def build_envelope(
    *,
    repository: str,
    event_name: str,
    run_id: str,
    commit_sha: str,
    ref: str,
    actor: str,
    generated_at: str,
    expires_at: str,
) -> dict[str, Any]:
    return {
        "schema_version": SCHEMA_VERSION,
        "repository": repository,
        "event": event_name,
        "run_id": str(run_id),
        "commit_sha": commit_sha,
        "ref": ref,
        "actor": actor,
        "generated_at": generated_at,
        "expires_at": expires_at,
        "source_classification": "OBSERVATION",
        "authority_effect": "none",
    }


def ensure_safe_string(value: str, location: str) -> None:
    for pattern in SECRET_PATTERNS:
        if pattern.search(value):
            raise ValidationError(f"secret-like content rejected at {location}")
    if ALPHA_OMEGA_AUTHORITY_RE.search(value):
        raise ValidationError(f"α/ω authority language rejected at {location}")
    if FORBIDDEN_VALUE_RE.search(value):
        raise ValidationError(f"forbidden authority semantics rejected at {location}")


def walk_payload(payload: Any, location: str = "root") -> None:
    if isinstance(payload, dict):
        for key, value in payload.items():
            key_str = str(key)
            key_lower = key_str.lower()
            if key_lower in BODY_CONTENT_KEYS:
                raise ValidationError(f"body content field rejected at {location}.{key_str}")
            if key_lower != "authority_effect" and FORBIDDEN_KEY_RE.search(key_str):
                raise ValidationError(f"forbidden field rejected at {location}.{key_str}")
            walk_payload(value, f"{location}.{key_str}")
    elif isinstance(payload, list):
        for idx, value in enumerate(payload):
            walk_payload(value, f"{location}[{idx}]")
    elif isinstance(payload, str):
        ensure_safe_string(payload, location)


def validate_envelope(envelope: dict[str, Any], expected_repository: str) -> None:
    missing = REQUIRED_ENVELOPE_FIELDS - set(envelope)
    if missing:
        raise ValidationError(f"envelope missing fields: {sorted(missing)}")
    if envelope["schema_version"] != SCHEMA_VERSION:
        raise ValidationError("unexpected envelope schema_version")
    if envelope["repository"] != expected_repository:
        raise ValidationError("repository mismatch")
    for field in ("repository", "event", "ref", "actor"):
        if not isinstance(envelope[field], str) or not envelope[field].strip():
            raise ValidationError(f"invalid {field}")
    if envelope["authority_effect"] != "none":
        raise ValidationError("authority_effect must be 'none'")
    if envelope["source_classification"] not in ALLOWED_SOURCE_CLASSIFICATIONS:
        raise ValidationError("invalid source_classification")
    if not COMMIT_SHA_RE.match(str(envelope["commit_sha"])):
        raise ValidationError("invalid commit_sha")
    if not RUN_ID_RE.match(str(envelope["run_id"])):
        raise ValidationError("invalid run_id")
    generated_at = parse_iso8601(str(envelope["generated_at"]))
    expires_at = parse_iso8601(str(envelope["expires_at"]))
    if expires_at <= generated_at:
        raise ValidationError("expires_at must be after generated_at")
    if expires_at <= utc_now():
        raise ValidationError("context has expired")
    walk_payload(envelope)


def validate_human_context(entries: list[dict[str, Any]]) -> None:
    if not isinstance(entries, list):
        raise ValidationError("human context must be a list")
    for idx, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValidationError(f"human context entry {idx} must be an object")
        classification = entry.get("classification")
        if classification not in ALLOWED_HUMAN_CLASSIFICATIONS:
            raise ValidationError(f"invalid human context classification at entry {idx}")
        event_name = entry.get("event")
        if not isinstance(event_name, str) or not event_name.strip():
            raise ValidationError(f"missing event at entry {idx}")
        if "body_sha256" in entry and not SHA256_RE.match(str(entry["body_sha256"])):
            raise ValidationError(f"invalid body_sha256 at entry {idx}")
        walk_payload(entry, f"human_context[{idx}]")


def parse_permissions(lines: list[str]) -> dict[str, str]:
    permissions: dict[str, str] = {}
    in_block = False
    block_indent = 0
    for raw_line in lines:
        line = raw_line.split("#", 1)[0].rstrip("\n")
        stripped = line.strip()
        if not stripped:
            continue
        indent = len(line) - len(line.lstrip(" "))
        if stripped == "permissions:":
            in_block = True
            block_indent = indent
            continue
        if in_block:
            if indent <= block_indent:
                in_block = False
            else:
                key, _, value = stripped.partition(":")
                permissions[key.strip()] = value.strip()
                continue
        if not in_block and permissions:
            break
    return permissions


def validate_workflow_file(workflow_path: Path) -> dict[str, Any]:
    if not workflow_path.is_file():
        raise ValidationError(f"missing workflow file: {workflow_path}")
    lines = workflow_path.read_text(encoding="utf-8").splitlines()
    permissions = parse_permissions(lines)
    if permissions != {"contents": "read", "actions": "read"}:
        raise ValidationError(f"unexpected permissions block: {permissions}")

    active_text = "\n".join(line.split("#", 1)[0].strip().lower() for line in lines if line.strip())
    if re.search(r"\b(write|id-token|attestations)\s*:", active_text):
        raise ValidationError("write-capable permissions are not allowed")
    if re.search(r"^\s*environment\s*:", "\n".join(lines), re.MULTILINE):
        raise ValidationError("deployment environments are not allowed")
    if re.search(
        r"\b(promot(e|ion)|deploy(ment)?|publish|approve|override|force|emergency|"
        r"state[_ -]?transition|execution authority|allowed execution)\b",
        active_text,
    ):
        raise ValidationError("workflow contains prohibited state-changing semantics")

    return {
        "schema_version": WORKFLOW_SCHEMA_VERSION,
        "workflow_path": workflow_path.as_posix(),
        "permissions": permissions,
        "promotion_job_present": False,
        "deployment_permission_present": False,
        "write_permission_present": False,
        "override_flag_present": False,
        "state_transition_authority_present": False,
        "authority_effect": "none",
    }


def build_manifest(
    output_dir: Path,
    *,
    commit_sha: str,
    run_id: str,
    generated_at: str,
    expires_at: str,
    evidence_paths: list[Path],
) -> dict[str, Any]:
    entries = [
        {
            "path": path.relative_to(output_dir).as_posix(),
            "sha256": sha256_file(path),
        }
        for path in sorted(evidence_paths)
    ]
    return {
        "schema_version": MANIFEST_SCHEMA_VERSION,
        "commit_sha": commit_sha,
        "workflow_run_id": run_id,
        "generated_at": generated_at,
        "expires_at": expires_at,
        "entries": entries,
    }


def validate_manifest(manifest: dict[str, Any], output_dir: Path, envelope: dict[str, Any]) -> None:
    missing = REQUIRED_MANIFEST_FIELDS - set(manifest)
    if missing:
        raise ValidationError(f"manifest missing fields: {sorted(missing)}")
    if manifest["schema_version"] != MANIFEST_SCHEMA_VERSION:
        raise ValidationError("unexpected manifest schema_version")
    if manifest["commit_sha"] != envelope["commit_sha"]:
        raise ValidationError("manifest commit_sha mismatch")
    if str(manifest["workflow_run_id"]) != str(envelope["run_id"]):
        raise ValidationError("manifest workflow_run_id mismatch")
    if manifest["generated_at"] != envelope["generated_at"]:
        raise ValidationError("manifest generated_at mismatch")
    if manifest["expires_at"] != envelope["expires_at"]:
        raise ValidationError("manifest expires_at mismatch")

    entries = manifest["entries"]
    output_root = output_dir.resolve()
    if not isinstance(entries, list) or not entries:
        raise ValidationError("manifest entries must be a non-empty list")
    paths = [entry.get("path") for entry in entries]
    if paths != sorted(paths):
        raise ValidationError("manifest entries must be sorted by path")
    for idx, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValidationError(f"manifest entry {idx} must be an object")
        path_value = entry.get("path")
        hash_value = entry.get("sha256")
        if not isinstance(path_value, str) or not path_value or path_value.startswith("/"):
            raise ValidationError(f"invalid manifest path at entry {idx}")
        if not isinstance(hash_value, str) or not SHA256_RE.match(hash_value):
            raise ValidationError(f"invalid sha256 at entry {idx}")
        candidate = (output_dir / path_value).resolve()
        try:
            candidate.relative_to(output_root)
        except ValueError as exc:
            raise ValidationError(f"manifest path escapes output_dir at entry {idx}: {path_value}") from exc
        if not candidate.is_file():
            raise ValidationError(f"missing evidence file for manifest entry {idx}: {path_value}")
        if sha256_file(candidate) != hash_value:
            raise ValidationError(f"sha256 mismatch for manifest entry {idx}: {path_value}")
    walk_payload(manifest, "manifest")


def write_bundle(args: argparse.Namespace) -> int:
    output_dir = Path(args.output_dir).resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    event = load_event(Path(args.event_path))
    generated_at = isoformat_utc(utc_now())
    expires_at = isoformat_utc(parse_iso8601(generated_at) + timedelta(days=1))

    envelope = build_envelope(
        repository=args.repository,
        event_name=args.event_name,
        run_id=args.run_id,
        commit_sha=args.commit_sha,
        ref=args.ref,
        actor=args.actor,
        generated_at=generated_at,
        expires_at=expires_at,
    )
    human_context = extract_human_context(args.event_name, event)
    workflow_assertions = validate_workflow_file(Path(args.workflow_path).resolve())

    envelope_path = output_dir / "context-envelope.json"
    human_context_path = output_dir / "human-context.json"
    workflow_assertions_path = output_dir / "workflow-assertions.json"
    manifest_path = output_dir / "manifest.json"
    manifest_hash_path = output_dir / "manifest.sha256"

    write_json(envelope_path, envelope)
    write_json(human_context_path, human_context)
    write_json(workflow_assertions_path, workflow_assertions)

    manifest = build_manifest(
        output_dir,
        commit_sha=args.commit_sha,
        run_id=args.run_id,
        generated_at=generated_at,
        expires_at=expires_at,
        evidence_paths=[envelope_path, human_context_path, workflow_assertions_path],
    )
    write_json(manifest_path, manifest)
    manifest_hash_path.write_text(f"{sha256_file(manifest_path)}  manifest.json\n", encoding="utf-8")

    validate_envelope(envelope, args.repository)
    validate_human_context(human_context)
    validate_manifest(manifest, output_dir, envelope)

    print(f"ephemeral_context: wrote evidence to {output_dir}")
    for rel_path in (
        "context-envelope.json",
        "human-context.json",
        "workflow-assertions.json",
        "manifest.json",
        "manifest.sha256",
    ):
        print(rel_path)
    return 0


def workflow_guard(args: argparse.Namespace) -> int:
    assertions = validate_workflow_file(Path(args.workflow_path).resolve())
    print(json.dumps(assertions, sort_keys=True, indent=2))
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    capture_parser = subparsers.add_parser("capture", help="capture bounded evidence")
    capture_parser.add_argument("--output-dir", required=True)
    capture_parser.add_argument("--workflow-path", required=True)
    capture_parser.add_argument("--event-path", required=True)
    capture_parser.add_argument("--repository", required=True)
    capture_parser.add_argument("--event-name", required=True)
    capture_parser.add_argument("--run-id", required=True)
    capture_parser.add_argument("--commit-sha", required=True)
    capture_parser.add_argument("--ref", required=True)
    capture_parser.add_argument("--actor", required=True)
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
