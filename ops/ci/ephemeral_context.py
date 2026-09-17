#!/usr/bin/env python3
"""Capture, validate, and expire ephemeral zero-trust shared context."""
from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import os
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
POLICY_MANIFEST = ROOT / "ops" / "ephemeral-context" / "context-policy.manifest.json"
SCHEMA_PATH = ROOT / "ops" / "ephemeral-context" / "context-envelope.schema.json"
HEX64 = set("0123456789abcdef")
HEX40 = set("0123456789abcdef")

sys.path.insert(0, str(Path(__file__).resolve().parent))
from uncertainty_bands import assess_uncertainty  # noqa: E402


class ValidationError(ValueError):
    """Fail-closed validation error."""


REQUIRED_TOP_LEVEL = {
    "schema_version",
    "source_repository",
    "event",
    "commit",
    "observations",
    "generated_at",
    "expires_at",
    "parent_context_hash",
    "context_hash",
    "authority_effect",
    "evidence_hashes",
}
OPTIONAL_TOP_LEVEL = {"uncertainty"}
EVENT_REQUIRED = {"name", "event_id", "workflow_run_id", "workflow_run_attempt", "ref"}
EVENT_ALLOWED = EVENT_REQUIRED | {
    "workflow_job",
    "pull_request_number",
    "discussion_number",
    "comment_id",
}
COMMIT_REQUIRED = {"sha", "ref"}
OBS_REQUIRED = {
    "kind",
    "source",
    "summary",
    "truth_status",
    "authority_effect",
    "observed_at",
    "evidence_refs",
}
OBS_ALLOWED = OBS_REQUIRED | {"narrative"}
HASH_REQUIRED = {"label", "sha256", "origin"}
UNCERTAINTY_REQUIRED = {
    "stress",
    "strain",
    "provenance_confidence",
    "crosscheck_confidence",
    "rollback_available",
    "uncertainty_score",
    "execution_band",
    "restriction_reasons",
}
TRUTH_STATUSES = {"unverified", "observed", "crosschecked"}
BANDS = {"analysis", "reversible", "constrained", "quarantine", "fully_restricted"}
SECRET_LIKE_RE = re.compile(
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9]{20,}"
    r"|xox[baprs]-[A-Za-z0-9-]{10,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----"
)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    capture = subparsers.add_parser("capture", help="Capture an ephemeral context envelope")
    capture.add_argument("--repository", required=True)
    capture.add_argument("--event-name", required=True)
    capture.add_argument("--workflow-run-id", required=True, type=int)
    capture.add_argument("--workflow-run-attempt", required=True, type=int)
    capture.add_argument("--workflow-job", default=None)
    capture.add_argument("--event-path", required=True)
    capture.add_argument("--ref", required=True)
    capture.add_argument("--commit-sha", required=True)
    capture.add_argument("--generated-at", default=None)
    capture.add_argument("--ttl-seconds", type=int, default=None)
    capture.add_argument("--parent-context-hash", default=None)
    capture.add_argument("--output", required=True)
    capture.add_argument("--stress", type=float, default=None)
    capture.add_argument("--strain", type=float, default=None)
    capture.add_argument("--provenance-confidence", type=float, default=None)
    capture.add_argument("--crosscheck-confidence", type=float, default=None)
    capture.add_argument("--rollback-available", choices=["true", "false"], default=None)

    validate = subparsers.add_parser("validate", help="Validate an ephemeral context envelope")
    validate.add_argument("--input", required=True)
    validate.add_argument("--expected-repository", required=True)
    validate.add_argument("--policy-manifest", default=str(POLICY_MANIFEST))
    validate.add_argument("--now", default=None)
    validate.add_argument("--seen-hashes-file", default=None)
    validate.add_argument("--record-seen", action="store_true")

    expire = subparsers.add_parser("expire", help="Check whether an envelope has expired")
    expire.add_argument("--input", required=True)
    expire.add_argument("--now", default=None)
    expire.add_argument("--delete", action="store_true")

    return parser.parse_args()


def _read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise ValidationError(f"missing file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise ValidationError(f"invalid JSON at {path}: {exc}") from exc


def _parse_time(value: str) -> dt.datetime:
    if not isinstance(value, str) or not value:
        raise ValidationError("invalid timestamp")
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValidationError(f"invalid timestamp {value!r}") from exc
    if parsed.tzinfo is None:
        raise ValidationError(f"timestamp must be timezone-aware: {value!r}")
    return parsed.astimezone(dt.timezone.utc)


def _format_time(value: dt.datetime) -> str:
    return value.astimezone(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def _canonical_bytes(data: Any) -> bytes:
    return json.dumps(data, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")


def _sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _sha256_path(path: Path) -> str:
    return _sha256_bytes(path.read_bytes())


def _is_hex(text: str, length: int) -> bool:
    return isinstance(text, str) and len(text) == length and set(text) <= (HEX64 if length == 64 else HEX40)


def _load_policy_manifest(path: Path) -> dict[str, Any]:
    manifest = _read_json(path)
    if not isinstance(manifest, dict):
        raise ValidationError("policy manifest must be an object")
    return manifest


def _authority_fragments(policy: dict[str, Any]) -> tuple[str, ...]:
    fragments = policy.get("forbidden_field_name_fragments", [])
    if not isinstance(fragments, list) or not all(isinstance(x, str) and x for x in fragments):
        raise ValidationError("policy manifest forbidden_field_name_fragments must be a string array")
    return tuple(fragment.lower() for fragment in fragments)


def _check_forbidden_keys(node: Any, fragments: tuple[str, ...], path: str = "$") -> None:
    if isinstance(node, dict):
        for key, value in node.items():
            lowered = key.lower()
            if any(fragment in lowered for fragment in fragments):
                raise ValidationError(f"authority-bearing field rejected at {path}.{key}")
            if key == "authority_effect" and value != "none":
                raise ValidationError(f"authority_effect must be none at {path}.{key}")
            _check_forbidden_keys(value, fragments, f"{path}.{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            _check_forbidden_keys(value, fragments, f"{path}[{index}]")


def _event_identifier(payload: dict[str, Any], workflow_run_id: int) -> tuple[str, int | None, int | None, int | None]:
    pull_number = payload.get("pull_request", {}).get("number")
    discussion_number = payload.get("discussion", {}).get("number")
    comment_id = payload.get("comment", {}).get("id") or payload.get("review", {}).get("id")
    for candidate in (
        payload.get("comment", {}).get("id"),
        payload.get("review", {}).get("id"),
        payload.get("discussion", {}).get("id"),
        payload.get("pull_request", {}).get("id"),
        payload.get("issue", {}).get("id"),
        payload.get("workflow_run", {}).get("id"),
    ):
        if candidate:
            return str(candidate), pull_number, discussion_number, comment_id
    return f"run-{workflow_run_id}", pull_number, discussion_number, comment_id


def _build_observation(event_name: str, generated_at: str, evidence_label: str, payload: dict[str, Any]) -> dict[str, Any]:
    narrative = None
    if isinstance(payload.get("comment"), dict):
        narrative = payload["comment"].get("body")
    elif isinstance(payload.get("discussion"), dict):
        narrative = payload["discussion"].get("body")
    elif isinstance(payload.get("pull_request"), dict):
        narrative = payload["pull_request"].get("title")

    observation = {
        "kind": "github_event",
        "source": "github_actions",
        "summary": f"GitHub {event_name} observation captured as untrusted ephemeral context.",
        "truth_status": "unverified" if narrative else "observed",
        "authority_effect": "none",
        "observed_at": generated_at,
        "evidence_refs": [evidence_label],
    }
    if narrative:
        observation["narrative"] = narrative
        observation["truth_status"] = "unverified"
    return observation


def _reject_secret_like_narrative(observations: list[dict[str, Any]]) -> None:
    for index, observation in enumerate(observations):
        narrative = observation.get("narrative")
        if narrative and SECRET_LIKE_RE.search(narrative):
            raise ValidationError(f"secret-like narrative rejected in observations[{index}]")


def compute_context_hash(envelope: dict[str, Any]) -> str:
    candidate = copy.deepcopy(envelope)
    candidate.pop("context_hash", None)
    return _sha256_bytes(_canonical_bytes(candidate))


def _validate_top_level(envelope: dict[str, Any]) -> None:
    keys = set(envelope)
    if not REQUIRED_TOP_LEVEL.issubset(keys):
        missing = sorted(REQUIRED_TOP_LEVEL - keys)
        raise ValidationError(f"missing required fields: {', '.join(missing)}")
    extras = keys - REQUIRED_TOP_LEVEL - OPTIONAL_TOP_LEVEL
    if extras:
        raise ValidationError(f"unexpected top-level fields: {', '.join(sorted(extras))}")


def _validate_event(event: Any) -> None:
    if not isinstance(event, dict):
        raise ValidationError("event must be an object")
    if not EVENT_REQUIRED.issubset(event):
        missing = sorted(EVENT_REQUIRED - set(event))
        raise ValidationError(f"missing event fields: {', '.join(missing)}")
    extras = set(event) - EVENT_ALLOWED
    if extras:
        raise ValidationError(f"unexpected event fields: {', '.join(sorted(extras))}")
    if not isinstance(event["name"], str) or not event["name"]:
        raise ValidationError("event.name must be a non-empty string")
    if not isinstance(event["event_id"], str) or not event["event_id"]:
        raise ValidationError("event.event_id must be a non-empty string")
    if not isinstance(event["workflow_run_id"], int) or event["workflow_run_id"] <= 0:
        raise ValidationError("event.workflow_run_id must be a positive integer")
    if not isinstance(event["workflow_run_attempt"], int) or event["workflow_run_attempt"] <= 0:
        raise ValidationError("event.workflow_run_attempt must be a positive integer")
    if not isinstance(event["ref"], str) or not event["ref"]:
        raise ValidationError("event.ref must be a non-empty string")


def _validate_commit(commit: Any) -> None:
    if not isinstance(commit, dict):
        raise ValidationError("commit must be an object")
    if set(commit) != COMMIT_REQUIRED:
        raise ValidationError("commit must contain exactly sha and ref")
    if not _is_hex(commit["sha"], 40):
        raise ValidationError("commit.sha must be a 40-char lowercase hex SHA")
    if not isinstance(commit["ref"], str) or not commit["ref"]:
        raise ValidationError("commit.ref must be a non-empty string")


def _validate_hash_entries(evidence_hashes: Any) -> set[str]:
    if not isinstance(evidence_hashes, list) or not evidence_hashes:
        raise ValidationError("evidence_hashes must be a non-empty array")
    labels: set[str] = set()
    for item in evidence_hashes:
        if not isinstance(item, dict) or set(item) != HASH_REQUIRED:
            raise ValidationError("each evidence_hash entry must contain exactly label, sha256, and origin")
        label = item["label"]
        if not isinstance(label, str) or not label:
            raise ValidationError("evidence hash label must be a non-empty string")
        if label in labels:
            raise ValidationError(f"duplicate evidence hash label: {label}")
        labels.add(label)
        if not _is_hex(item["sha256"], 64):
            raise ValidationError(f"invalid sha256 for evidence hash label {label}")
        if not isinstance(item["origin"], str) or not item["origin"]:
            raise ValidationError(f"invalid origin for evidence hash label {label}")
    return labels


def _validate_observations(observations: Any, known_labels: set[str]) -> None:
    if not isinstance(observations, list) or not observations:
        raise ValidationError("observations must be a non-empty array")
    for observation in observations:
        if not isinstance(observation, dict):
            raise ValidationError("observation must be an object")
        if not OBS_REQUIRED.issubset(observation):
            missing = sorted(OBS_REQUIRED - set(observation))
            raise ValidationError(f"missing observation fields: {', '.join(missing)}")
        extras = set(observation) - OBS_ALLOWED
        if extras:
            raise ValidationError(f"unexpected observation fields: {', '.join(sorted(extras))}")
        if observation["truth_status"] not in TRUTH_STATUSES:
            raise ValidationError("observation.truth_status must be a recognized status")
        if observation["authority_effect"] != "none":
            raise ValidationError("observation.authority_effect must be none")
        _parse_time(observation["observed_at"])
        refs = observation["evidence_refs"]
        if not isinstance(refs, list) or not refs or not all(isinstance(x, str) and x for x in refs):
            raise ValidationError("observation.evidence_refs must be a non-empty string array")
        missing_refs = [label for label in refs if label not in known_labels]
        if missing_refs:
            raise ValidationError(f"unknown evidence refs: {', '.join(sorted(missing_refs))}")
        if "narrative" in observation and observation["truth_status"] != "unverified":
            raise ValidationError("narrative observations must use truth_status=unverified")


def _validate_uncertainty(uncertainty: Any) -> None:
    if uncertainty is None:
        return
    if not isinstance(uncertainty, dict):
        raise ValidationError("uncertainty must be an object")
    if set(uncertainty) != UNCERTAINTY_REQUIRED:
        raise ValidationError("uncertainty must contain exactly the required fields")
    for field in ("stress", "strain", "provenance_confidence", "crosscheck_confidence"):
        value = uncertainty[field]
        if value is not None and not isinstance(value, (int, float)):
            raise ValidationError(f"uncertainty.{field} must be a number or null")
        if isinstance(value, (int, float)) and not 0 <= float(value) <= 1:
            raise ValidationError(f"uncertainty.{field} must be within [0, 1]")
    if uncertainty["rollback_available"] not in (True, False, None):
        raise ValidationError("uncertainty.rollback_available must be boolean or null")
    score = uncertainty["uncertainty_score"]
    if not isinstance(score, (int, float)) or not 0 <= float(score) <= 1:
        raise ValidationError("uncertainty.uncertainty_score must be within [0, 1]")
    if uncertainty["execution_band"] not in BANDS:
        raise ValidationError("uncertainty.execution_band is invalid")
    reasons = uncertainty["restriction_reasons"]
    if not isinstance(reasons, list) or not all(isinstance(x, str) and x for x in reasons):
        raise ValidationError("uncertainty.restriction_reasons must be a string array")


def validate_envelope(
    envelope: dict[str, Any],
    *,
    expected_repository: str,
    policy_manifest_path: Path = POLICY_MANIFEST,
    now: dt.datetime | None = None,
    seen_hashes_file: Path | None = None,
    record_seen: bool = False,
) -> dict[str, Any]:
    if not SCHEMA_PATH.is_file():
        raise ValidationError(f"missing schema file: {SCHEMA_PATH}")
    policy = _load_policy_manifest(policy_manifest_path)
    fragments = _authority_fragments(policy)

    if not isinstance(envelope, dict):
        raise ValidationError("envelope must be an object")
    _validate_top_level(envelope)
    _check_forbidden_keys(envelope, fragments)

    if envelope["schema_version"] != "1.0.0":
        raise ValidationError("schema_version must be 1.0.0")
    if envelope["source_repository"] != expected_repository:
        raise ValidationError(
            f"repository mismatch: expected {expected_repository}, got {envelope['source_repository']}"
        )
    if envelope["authority_effect"] != "none":
        raise ValidationError("authority_effect must be none")

    _validate_event(envelope["event"])
    _validate_commit(envelope["commit"])
    labels = _validate_hash_entries(envelope["evidence_hashes"])
    _validate_observations(envelope["observations"], labels)
    _reject_secret_like_narrative(envelope["observations"])
    _validate_uncertainty(envelope.get("uncertainty"))

    generated_at = _parse_time(envelope["generated_at"])
    expires_at = _parse_time(envelope["expires_at"])
    if expires_at <= generated_at:
        raise ValidationError("expires_at must be later than generated_at")

    effective_now = now or dt.datetime.now(dt.timezone.utc)
    if effective_now.tzinfo is None:
        raise ValidationError("now must be timezone-aware")
    effective_now = effective_now.astimezone(dt.timezone.utc)

    freshness_window = int(policy.get("freshness_window_seconds", 0))
    max_ttl = int(policy.get("max_ttl_seconds", 0))
    age_seconds = (effective_now - generated_at).total_seconds()
    ttl_seconds = (expires_at - generated_at).total_seconds()

    if age_seconds < 0:
        raise ValidationError("generated_at is in the future")
    if effective_now >= expires_at:
        raise ValidationError("context expired")
    if freshness_window > 0 and age_seconds > freshness_window:
        raise ValidationError("context is stale")
    if max_ttl > 0 and ttl_seconds > max_ttl:
        raise ValidationError("context TTL exceeds policy")

    parent_hash = envelope["parent_context_hash"]
    if parent_hash is not None and not _is_hex(parent_hash, 64):
        raise ValidationError("parent_context_hash must be null or a 64-char lowercase hex SHA-256")
    if not _is_hex(envelope["context_hash"], 64):
        raise ValidationError("context_hash must be a 64-char lowercase hex SHA-256")

    expected_hash = compute_context_hash(envelope)
    if expected_hash != envelope["context_hash"]:
        raise ValidationError("context_hash mismatch")

    if seen_hashes_file is not None:
        seen_hashes = []
        if seen_hashes_file.exists():
            loaded = _read_json(seen_hashes_file)
            if not isinstance(loaded, list) or not all(isinstance(x, str) for x in loaded):
                raise ValidationError("seen hashes file must be a JSON string array")
            seen_hashes = loaded
        if envelope["context_hash"] in seen_hashes:
            raise ValidationError("replayed context hash rejected")
        if record_seen:
            seen_hashes.append(envelope["context_hash"])
            seen_hashes_file.parent.mkdir(parents=True, exist_ok=True)
            seen_hashes_file.write_text(json.dumps(seen_hashes, indent=2) + "\n", encoding="utf-8")

    return {
        "context_hash": envelope["context_hash"],
        "generated_at": envelope["generated_at"],
        "expires_at": envelope["expires_at"],
        "execution_band": envelope.get("uncertainty", {}).get("execution_band", "fully_restricted"),
    }


def capture_context(args: argparse.Namespace) -> int:
    policy = _load_policy_manifest(POLICY_MANIFEST)
    ttl_seconds = args.ttl_seconds or int(policy.get("freshness_window_seconds", 900))
    max_ttl = int(policy.get("max_ttl_seconds", ttl_seconds))
    if ttl_seconds <= 0 or ttl_seconds > max_ttl:
        raise ValidationError("ttl-seconds must be positive and within policy max_ttl_seconds")

    generated_at = _parse_time(args.generated_at) if args.generated_at else dt.datetime.now(dt.timezone.utc)
    expires_at = generated_at + dt.timedelta(seconds=ttl_seconds)
    payload_path = Path(args.event_path)
    payload = _read_json(payload_path)
    if not isinstance(payload, dict):
        raise ValidationError("event payload must be a JSON object")

    event_id, pull_number, discussion_number, comment_id = _event_identifier(payload, args.workflow_run_id)
    evidence_label = "github_event_payload"
    rollback_available = None
    if args.rollback_available is not None:
        rollback_available = args.rollback_available == "true"
    uncertainty = assess_uncertainty(
        stress=args.stress,
        strain=args.strain,
        provenance_confidence=args.provenance_confidence,
        crosscheck_confidence=args.crosscheck_confidence,
        rollback_available=rollback_available,
    )

    envelope = {
        "schema_version": "1.0.0",
        "source_repository": args.repository,
        "event": {
            "name": args.event_name,
            "event_id": event_id,
            "workflow_run_id": args.workflow_run_id,
            "workflow_run_attempt": args.workflow_run_attempt,
            "workflow_job": args.workflow_job,
            "pull_request_number": pull_number,
            "discussion_number": discussion_number,
            "comment_id": comment_id,
            "ref": args.ref,
        },
        "commit": {
            "sha": args.commit_sha.lower(),
            "ref": args.ref,
        },
        "observations": [
            _build_observation(args.event_name, _format_time(generated_at), evidence_label, payload)
        ],
        "generated_at": _format_time(generated_at),
        "expires_at": _format_time(expires_at),
        "parent_context_hash": args.parent_context_hash.lower() if args.parent_context_hash else None,
        "context_hash": "",
        "authority_effect": "none",
        "evidence_hashes": [
            {
                "label": evidence_label,
                "sha256": _sha256_path(payload_path),
                "origin": os.path.basename(args.event_path),
            }
        ],
        "uncertainty": uncertainty.as_dict(),
    }
    envelope["context_hash"] = compute_context_hash(envelope)

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    print(f"ephemeral context captured: {output_path}")
    print(f"context_hash={envelope['context_hash']}")
    print(f"execution_band={envelope['uncertainty']['execution_band']}")
    return 0


def validate_context(args: argparse.Namespace) -> int:
    input_path = Path(args.input)
    envelope = _read_json(input_path)
    now = _parse_time(args.now) if args.now else None
    result = validate_envelope(
        envelope,
        expected_repository=args.expected_repository,
        policy_manifest_path=Path(args.policy_manifest),
        now=now,
        seen_hashes_file=Path(args.seen_hashes_file) if args.seen_hashes_file else None,
        record_seen=args.record_seen,
    )
    print(json.dumps(result, indent=2))
    return 0


def expire_context(args: argparse.Namespace) -> int:
    envelope = _read_json(Path(args.input))
    if not isinstance(envelope, dict) or "expires_at" not in envelope:
        raise ValidationError("context envelope missing expires_at")
    now = _parse_time(args.now) if args.now else dt.datetime.now(dt.timezone.utc)
    expires_at = _parse_time(envelope["expires_at"])
    expired = now >= expires_at
    if expired and args.delete:
        Path(args.input).unlink(missing_ok=True)
    print(json.dumps({"expired": expired, "deleted": bool(expired and args.delete)}, indent=2))
    return 0 if expired else 1


def main() -> int:
    args = _parse_args()
    try:
        if args.command == "capture":
            return capture_context(args)
        if args.command == "validate":
            return validate_context(args)
        if args.command == "expire":
            return expire_context(args)
        raise ValidationError(f"unknown command: {args.command}")
    except ValidationError as exc:
        print(f"::error::{exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
