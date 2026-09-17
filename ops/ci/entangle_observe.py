#!/usr/bin/env python3
"""ENTANGLE observation envelope capture/reconcile/close helpers.

ATOM: ATOM-ENTANGLE-OBSERVE-20260917

Evidence only. Fail closed. External media and model output never carry
promotion or merge authority on their own.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = ROOT / "ops" / "entangle" / "manifest.yaml"
DEFAULT_SCHEMA = ROOT / "ops" / "entangle" / "observation-envelope.schema.json"
ATOM_ID = "ATOM-ENTANGLE-OBSERVE-20260917"
SCHEMA_VERSION = "0.1.0"
KIND = "entangle_observation_envelope"

SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
COMMIT_RE = re.compile(r"^[0-9a-f]{7,40}$")
DISCUSSION_URL_RE = re.compile(r"^https://github\.com/[^/]+/[^/]+/discussions/\d+$")
USES_RE = re.compile(r"^\s*uses:\s*(?P<action>[^\s@#\"']+)@(?P<ref>[^\s#\"']+)")
FULL_SHA_RE = re.compile(r"^[0-9a-fA-F]{40}$")
AUTHORITY_RE = re.compile(
    r"\b(authority|authorized|promotion|promote|merge authority|deployment authority|proof of truth)\b",
    re.I,
)
TIMESTAMP_RE = re.compile(r"\b\d{1,2}:\d{2}(?::\d{2})?\b")
UI_CHROME_RE = re.compile(
    r"\[(music|applause|laughter|subscribe|like|share|comment)\]|\b(subscribe|like and subscribe|smash the like button|hit the bell)\b",
    re.I,
)
OCR_NOISE_RE = re.compile(r"[_|~`#]{2,}|[^\w\s]{4,}")
WHITESPACE_RE = re.compile(r"\s+")
CODEQL_SUFFIX_LANGUAGE = {
    ".rs": "rust",
    ".py": "python",
    ".js": "javascript",
    ".ts": "javascript",
    ".jsx": "javascript",
    ".tsx": "javascript",
    ".java": "java",
    ".go": "go",
    ".rb": "ruby",
    ".cpp": "cpp",
    ".c": "cpp",
    ".cs": "csharp",
}
BANNED_ACTIONS = {
    "actions/cache",
    "swatinem/rust-cache",
    "actions/upload-artifact",
    "actions/upload-pages-artifact",
}
REQUIRED_STATES = ["observed", "normalized", "reconciled", "drifted", "closure-ready", "held"]


class ValidationError(RuntimeError):
    pass


def load_yaml(path: Path) -> dict[str, Any]:
    import yaml  # type: ignore

    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValidationError(f"{path} must contain a YAML mapping")
    return data


def load_json(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValidationError(f"{path} must contain a JSON object")
    return data


def dump_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def _now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _canonical_fragment(text: str) -> str:
    return " ".join(re.findall(r"[a-z0-9]+", text.casefold()))


def _normalize_line(text: str, erasures: list[str]) -> str:
    out = text
    if "timestamp-jitter" in erasures:
        out = TIMESTAMP_RE.sub(" ", out)
    if "ui-chrome" in erasures:
        out = UI_CHROME_RE.sub(" ", out)
    if "ocr-noise" in erasures:
        out = OCR_NOISE_RE.sub(" ", out)
    out = WHITESPACE_RE.sub(" ", out).strip()
    return out


def normalize_text(text: str, erasures: list[str]) -> tuple[str, dict[str, int]]:
    raw_parts = [p.strip() for p in re.split(r"[\n.!?]+", text) if p.strip()]
    if not raw_parts:
        return "", {"input_fragments": 0, "kept_fragments": 0, "duplicate_fragments": 0}
    kept: list[str] = []
    seen: set[str] = set()
    duplicates = 0
    for part in raw_parts:
        normalized = _normalize_line(part, erasures)
        if not normalized:
            continue
        if "duplicate-semantic-fragments" in erasures:
            key = _canonical_fragment(normalized)
            if key in seen:
                duplicates += 1
                continue
            seen.add(key)
        kept.append(normalized)
    return ". ".join(kept), {
        "input_fragments": len(raw_parts),
        "kept_fragments": len(kept),
        "duplicate_fragments": duplicates,
    }


def build_manifest_index(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    comps = manifest.get("components") or []
    return {str(comp["id"]): comp for comp in comps if isinstance(comp, dict) and "id" in comp}


def source_entries(envelope: dict[str, Any]) -> list[tuple[str, int, dict[str, Any]]]:
    entries: list[tuple[str, int, dict[str, Any]]] = []
    sources = envelope.get("sources") or {}
    if not isinstance(sources, dict):
        return entries
    for key, items in sources.items():
        if isinstance(items, list):
            for index, item in enumerate(items):
                if isinstance(item, dict):
                    entries.append((str(key), index, item))
    return entries


def _relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def _path_matches_component(rel_path: str, component_paths: list[str]) -> bool:
    probe = Path(rel_path).as_posix().rstrip("/")
    for base in component_paths:
        normalized = Path(base).as_posix().rstrip("/")
        if probe == normalized or probe.startswith(normalized + "/"):
            return True
    return False


def _validate_local_hash(rel_path: str, expected: str, errors: list[str]) -> None:
    target = ROOT / rel_path
    if not target.is_file():
        errors.append(f"missing local snapshot path: {rel_path}")
        return
    actual = sha256_file(target)
    if actual != expected:
        errors.append(f"hash mismatch for {rel_path}: expected {expected}, got {actual}")


def validate_envelope(manifest: dict[str, Any], envelope: dict[str, Any]) -> None:
    errors: list[str] = []
    if envelope.get("schemaVersion") != SCHEMA_VERSION:
        errors.append("schemaVersion must be 0.1.0")
    if envelope.get("kind") != KIND:
        errors.append("kind must be entangle_observation_envelope")
    component = envelope.get("component") or {}
    component_id = component.get("id")
    if not component_id:
        errors.append("component.id required")
    components = build_manifest_index(manifest)
    if component_id and component_id not in components:
        errors.append(f"unknown component id: {component_id}")
    code = envelope.get("code") or {}
    paths = code.get("paths") or []
    if not isinstance(paths, list) or not paths:
        errors.append("code.paths must be a non-empty list")
    commit = str(code.get("commit", ""))
    if not COMMIT_RE.match(commit):
        errors.append("code.commit must look like a git sha")
    if not str(code.get("ref", "")):
        errors.append("code.ref required")
    wormhole = envelope.get("wormhole") or {}
    if wormhole.get("mode") != "narrow-wormhole":
        errors.append("wormhole.mode must be narrow-wormhole")
    ephemeral_root = str(wormhole.get("ephemeralRoot", ""))
    if not ephemeral_root:
        errors.append("wormhole.ephemeralRoot required")
    discussion = wormhole.get("discussion") or {}
    if int(discussion.get("number", 0) or 0) <= 0:
        errors.append("wormhole.discussion.number must be > 0")
    if not DISCUSSION_URL_RE.match(str(discussion.get("url", ""))):
        errors.append("wormhole.discussion.url must be a GitHub discussion URL")
    trust = envelope.get("trust") or {}
    if trust.get("authorityEffect") != "none":
        errors.append("trust.authorityEffect must be none")
    source_classes = trust.get("sourceClasses") or {}
    manifest_bindings = manifest.get("source_bindings") or {}
    for key in ("youtube", "notebooklm", "grokCollections", "googleDrive", "githubDiscussion"):
        expected = ((manifest_bindings.get(key) or {}).get("trust_class"))
        if source_classes.get(key) != expected:
            errors.append(f"trust.sourceClasses.{key} must equal {expected!r}")
    operator = envelope.get("operator") or {}
    if not str(operator.get("principal", "")):
        errors.append("operator.principal required")
    if not str(operator.get("capturedAt", "")):
        errors.append("operator.capturedAt required")
    if not str(operator.get("purpose", "")):
        errors.append("operator.purpose required")
    noise = envelope.get("noisePolicy") or {}
    profile = str(noise.get("profile", ""))
    if profile not in (manifest.get("invariant_profiles") or {}):
        errors.append(f"unknown noisePolicy.profile: {profile}")
    erasures = noise.get("erasures") or []
    if not isinstance(erasures, list) or not erasures:
        errors.append("noisePolicy.erasures must be non-empty list")
    observations = envelope.get("observations") or []
    if not isinstance(observations, list) or not observations:
        errors.append("observations must be non-empty list")
    state = envelope.get("state")
    if state not in REQUIRED_STATES:
        errors.append(f"state must be one of {REQUIRED_STATES}")
    handoff = envelope.get("handoff") or {}
    if handoff.get("conservationTag") != "α + ω = 15" or handoff.get("category") != "C":
        errors.append("handoff must preserve α + ω = 15 as Category C label only")

    for key, index, item in source_entries(envelope):
        if AUTHORITY_RE.search(json.dumps(item, ensure_ascii=False)):
            errors.append(f"{key}[{index}] contains authority-bearing language")
        if key == "youtube":
            if not str(item.get("videoId", "")):
                errors.append("youtube entry needs videoId")
            if not str(item.get("url", "")).startswith("https://www.youtube.com/"):
                errors.append("youtube entry needs https://www.youtube.com/ URL")
            segments = item.get("segments") or []
            if not isinstance(segments, list) or not segments:
                errors.append("youtube entry needs non-empty segments")
            if not SHA256_RE.match(str(item.get("transcriptSha256", ""))):
                errors.append("youtube entry needs transcriptSha256")
        elif key == "notebooklm":
            if not str(item.get("documentId", "")):
                errors.append("notebooklm entry needs documentId")
            rel = str(item.get("snapshotPath", ""))
            if not rel:
                errors.append("notebooklm entry needs snapshotPath")
            expected = str(item.get("snapshotSha256", ""))
            if not SHA256_RE.match(expected):
                errors.append("notebooklm entry needs snapshotSha256")
            elif rel:
                _validate_local_hash(rel, expected, errors)
        elif key == "grokCollections":
            if not str(item.get("collectionId", "")):
                errors.append("grokCollections entry needs collectionId")
            rel = str(item.get("snapshotPath", ""))
            expected = str(item.get("snapshotSha256", ""))
            if not rel:
                errors.append("grokCollections entry needs snapshotPath")
            if not SHA256_RE.match(expected):
                errors.append("grokCollections entry needs snapshotSha256")
            elif rel:
                _validate_local_hash(rel, expected, errors)
        elif key == "googleDrive":
            if not str(item.get("objectId", "")):
                errors.append("googleDrive entry needs objectId")
            if not SHA256_RE.match(str(item.get("sha256", ""))):
                errors.append("googleDrive entry needs sha256")
            if not str(item.get("pathHint", "")):
                errors.append("googleDrive entry needs pathHint")
        elif key == "githubDiscussion":
            if int(item.get("number", 0) or 0) <= 0:
                errors.append("githubDiscussion entry needs positive number")
            if not DISCUSSION_URL_RE.match(str(item.get("url", ""))):
                errors.append("githubDiscussion entry needs discussion URL")
        else:
            errors.append(f"unsupported source key: {key}")

    for obs in observations:
        if not isinstance(obs, dict):
            errors.append("observation must be object")
            continue
        blob = json.dumps(obs, ensure_ascii=False)
        if AUTHORITY_RE.search(blob):
            errors.append(f"observation {obs.get('id', '<unknown>')} contains authority-bearing language")
        if not str(obs.get("id", "")):
            errors.append("observation.id required")
        if not str(obs.get("summary", "")):
            errors.append("observation.summary required")
        if not str(obs.get("transcript", "")):
            errors.append("observation.transcript required")
        evidence = obs.get("evidence") or []
        if not isinstance(evidence, list) or not evidence:
            errors.append("observation.evidence must be non-empty list")

    if errors:
        raise ValidationError("; ".join(errors))


def capture_envelope(
    manifest: dict[str, Any],
    envelope: dict[str, Any],
    *,
    ephemeral_root: str,
    discussion_number: int | None = None,
) -> dict[str, Any]:
    result = copy.deepcopy(envelope)
    result.setdefault("atomTrailId", ATOM_ID)
    result["wormhole"]["ephemeralRoot"] = ephemeral_root
    if discussion_number:
        result["wormhole"]["discussion"]["number"] = discussion_number
        result["wormhole"]["discussion"]["url"] = (
            f"https://github.com/toolate28/LogOS/discussions/{discussion_number}"
        )
        if result.get("sources", {}).get("githubDiscussion"):
            result["sources"]["githubDiscussion"][0]["number"] = discussion_number
            result["sources"]["githubDiscussion"][0]["url"] = (
                f"https://github.com/toolate28/LogOS/discussions/{discussion_number}"
            )
    validate_envelope(manifest, result)

    erasures = list(result["noisePolicy"]["erasures"])
    total_in = 0
    total_dupes = 0
    normalized_obs: list[dict[str, Any]] = []
    for obs in result["observations"]:
        new_obs = copy.deepcopy(obs)
        summary, s_stats = normalize_text(str(obs.get("summary", "")), erasures)
        transcript, t_stats = normalize_text(str(obs.get("transcript", "")), erasures)
        total_in += s_stats["input_fragments"] + t_stats["input_fragments"]
        total_dupes += s_stats["duplicate_fragments"] + t_stats["duplicate_fragments"]
        new_obs["summary"] = summary
        new_obs["transcript"] = transcript
        normalized_obs.append(new_obs)
    result["observations"] = normalized_obs
    duplicate_ratio = 0.0 if total_in == 0 else total_dupes / total_in
    budget = float(result["noisePolicy"].get("duplicateFragmentBudget", 0.35))
    if duplicate_ratio > budget:
        raise ValidationError(
            f"noise budget exceeded: duplicate fragment ratio {duplicate_ratio:.3f} > {budget:.3f}"
        )
    result["normalization"] = {
        "profile": result["noisePolicy"]["profile"],
        "appliedErasures": erasures,
        "duplicateFragmentRatio": round(duplicate_ratio, 6),
        "capturedAt": _now(),
    }
    result["state"] = "normalized"
    return result


def _codeql_compatibility(paths: list[str]) -> dict[str, Any]:
    languages = sorted({CODEQL_SUFFIX_LANGUAGE.get(Path(p).suffix.lower()) for p in paths if CODEQL_SUFFIX_LANGUAGE.get(Path(p).suffix.lower())})
    if not languages:
        return {"status": "not_applicable", "languages": [], "reason": "no CodeQL-supported code files in envelope paths"}
    return {"status": "compatible", "languages": languages, "reason": "supported code files present"}


def _codex_compatibility(paths: list[str]) -> dict[str, Any]:
    script = ROOT / "ops" / "ci" / "codex_scan.py"
    missing = [p for p in paths if not (ROOT / p).exists()]
    if not script.is_file():
        return {"status": "incompatible", "reason": "ops/ci/codex_scan.py missing"}
    if missing:
        return {"status": "incompatible", "reason": f"missing repo paths: {missing}"}
    return {"status": "compatible", "reason": "component paths can be inspected by local CODEX scanner"}


def reconcile_envelope(manifest: dict[str, Any], envelope: dict[str, Any]) -> dict[str, Any]:
    validate_envelope(manifest, envelope)
    component = build_manifest_index(manifest)[envelope["component"]["id"]]
    component_paths = [str(p) for p in component.get("paths") or []]
    matched: list[str] = []
    drifted: list[str] = []
    for rel in envelope["code"]["paths"]:
        if _path_matches_component(str(rel), component_paths):
            matched.append(str(rel))
        else:
            drifted.append(str(rel))

    admissible = set(component.get("admissible_sources") or [])
    if not admissible and component.get("class"):
        admissible = set((manifest.get("component_classes") or {}).get(component["class"], {}).get("admissible_sources") or [])
    disallowed_sources = sorted({key for key, _, _ in source_entries(envelope) if admissible and key not in admissible})

    compatibility = {
        "codeql": _codeql_compatibility(matched),
        "codex": _codex_compatibility(matched or [str(p) for p in envelope["code"]["paths"]]),
    }
    unresolved: list[str] = []
    blockers: list[str] = []
    if drifted:
        blockers.append("component-path-drift")
        unresolved.append(f"paths outside component slice: {', '.join(drifted)}")
    if disallowed_sources:
        blockers.append("source-class-violation")
        unresolved.append(f"disallowed sources for component: {', '.join(disallowed_sources)}")
    if compatibility["codeql"]["status"] == "incompatible":
        blockers.append("codeql-incompatible")
        unresolved.append(compatibility["codeql"]["reason"])
    if compatibility["codex"]["status"] == "incompatible":
        blockers.append("codex-incompatible")
        unresolved.append(compatibility["codex"]["reason"])

    state = "reconciled" if not blockers else "drifted"
    return {
        "schemaVersion": SCHEMA_VERSION,
        "kind": "entangle_observation_reconcile",
        "atomTrailId": envelope.get("atomTrailId", ATOM_ID),
        "component": envelope["component"],
        "wormhole": envelope["wormhole"],
        "state": state,
        "componentPathCheck": {"matched": matched, "drifted": drifted},
        "sourceCheck": {"admissible": sorted(admissible), "disallowed": disallowed_sources},
        "compatibility": compatibility,
        "unresolved": unresolved,
        "blockers": blockers,
        "generatedAt": _now(),
    }


def close_envelope(envelope: dict[str, Any], reconcile: dict[str, Any]) -> dict[str, Any]:
    blockers = list(reconcile.get("blockers") or [])
    normalization = envelope.get("normalization") or {}
    duplicate_ratio = float(normalization.get("duplicateFragmentRatio", 0.0) or 0.0)
    budget = float(envelope.get("noisePolicy", {}).get("duplicateFragmentBudget", 0.35) or 0.35)
    final_state = "closure-ready"
    unresolved = list(reconcile.get("unresolved") or [])
    if duplicate_ratio > budget:
        blockers.append("noise-budget-exceeded")
        unresolved.append("duplicate fragment ratio exceeded budget")
    if blockers:
        final_state = "held" if any(b.endswith("incompatible") or b == "source-class-violation" for b in blockers) else "drifted"
    state_history = ["observed", envelope.get("state", "normalized"), reconcile.get("state", "drifted"), final_state]
    closure = {
        "schemaVersion": SCHEMA_VERSION,
        "kind": "entangle_observation_closure",
        "atomTrailId": envelope.get("atomTrailId", ATOM_ID),
        "component": envelope["component"],
        "wormhole": envelope["wormhole"],
        "state": final_state,
        "stateHistory": state_history,
        "matchedPaths": reconcile.get("componentPathCheck", {}).get("matched", []),
        "driftedPaths": reconcile.get("componentPathCheck", {}).get("drifted", []),
        "compatibility": reconcile.get("compatibility", {}),
        "blockers": blockers,
        "unresolved": unresolved,
        "handoff": envelope.get("handoff", {}),
        "generatedAt": _now(),
    }
    summary_path = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary_path:
        with open(summary_path, "a", encoding="utf-8") as f:
            f.write("## ENTANGLE Observe\n\n")
            f.write(f"- Component: `{closure['component']['id']}`\n")
            f.write(f"- State: `{final_state}`\n")
            f.write(f"- Discussion: `{closure['wormhole']['discussion']['number']}`\n")
            f.write(f"- Ephemeral root: `{closure['wormhole']['ephemeralRoot']}`\n")
            f.write(f"- Matched paths: {len(closure['matchedPaths'])}\n")
            f.write(f"- Drifted paths: {len(closure['driftedPaths'])}\n")
            if unresolved:
                f.write("- Unresolved:\n")
                for item in unresolved:
                    f.write(f"  - {item}\n")
            else:
                f.write("- Unresolved: none\n")
            f.write("\n")
    return closure


def workflow_guard(workflow_path: Path) -> None:
    data = load_yaml(workflow_path)
    triggers = data.get("on", data.get(True))
    if isinstance(triggers, dict):
        keys = set(triggers)
    elif isinstance(triggers, str):
        keys = {triggers}
    else:
        keys = set()
    if keys != {"workflow_dispatch"}:
        raise ValidationError("workflow must be manual-only with on: workflow_dispatch")
    permissions = data.get("permissions") or {}
    if permissions != {"contents": "read"}:
        raise ValidationError("workflow permissions must be exactly contents: read")
    text = workflow_path.read_text(encoding="utf-8")
    if "narrow-wormhole" not in text or "runner.temp" not in text:
        raise ValidationError("workflow must use runner.temp narrow-wormhole ephemeral root")
    for i, line in enumerate(text.splitlines(), 1):
        stripped = line.lstrip()
        if stripped.startswith("#"):
            continue
        m = USES_RE.match(line)
        if not m:
            continue
        action = m.group("action")
        ref = m.group("ref")
        if action.casefold() in BANNED_ACTIONS:
            raise ValidationError(f"workflow uses banned cache/artifact action {action} at line {i}")
        if not FULL_SHA_RE.match(ref):
            raise ValidationError(f"workflow action {action}@{ref} at line {i} is not SHA-pinned")
    print(f"workflow-guard: OK {workflow_path}")


def _arg_path(value: str) -> Path:
    return Path(value)


def cmd_capture(args: argparse.Namespace) -> int:
    manifest = load_yaml(args.manifest_path)
    envelope = load_json(args.envelope_path)
    normalized = capture_envelope(
        manifest,
        envelope,
        ephemeral_root=args.ephemeral_root,
        discussion_number=args.discussion_number,
    )
    dump_json(args.out, normalized)
    print(f"capture: OK state={normalized['state']} out={args.out}")
    return 0


def cmd_reconcile(args: argparse.Namespace) -> int:
    manifest = load_yaml(args.manifest_path)
    envelope = load_json(args.envelope_path)
    report = reconcile_envelope(manifest, envelope)
    dump_json(args.out, report)
    print(f"reconcile: OK state={report['state']} out={args.out}")
    return 0


def cmd_close(args: argparse.Namespace) -> int:
    envelope = load_json(args.envelope_path)
    reconcile = load_json(args.reconcile_path)
    closure = close_envelope(envelope, reconcile)
    dump_json(args.out, closure)
    print(f"close: OK state={closure['state']} out={args.out}")
    return 0


def cmd_workflow_guard(args: argparse.Namespace) -> int:
    workflow_guard(args.workflow_path)
    return 0


def build_parser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(description="ENTANGLE observation envelope helper")
    sub = ap.add_subparsers(dest="cmd", required=True)

    c = sub.add_parser("capture")
    c.add_argument("--manifest-path", type=_arg_path, default=DEFAULT_MANIFEST)
    c.add_argument("--envelope-path", type=_arg_path, required=True)
    c.add_argument("--ephemeral-root", required=True)
    c.add_argument("--discussion-number", type=int)
    c.add_argument("--out", type=_arg_path, required=True)
    c.set_defaults(func=cmd_capture)

    r = sub.add_parser("reconcile")
    r.add_argument("--manifest-path", type=_arg_path, default=DEFAULT_MANIFEST)
    r.add_argument("--envelope-path", type=_arg_path, required=True)
    r.add_argument("--out", type=_arg_path, required=True)
    r.set_defaults(func=cmd_reconcile)

    cl = sub.add_parser("close")
    cl.add_argument("--envelope-path", type=_arg_path, required=True)
    cl.add_argument("--reconcile-path", type=_arg_path, required=True)
    cl.add_argument("--out", type=_arg_path, required=True)
    cl.set_defaults(func=cmd_close)

    wg = sub.add_parser("workflow-guard")
    wg.add_argument("--workflow-path", type=_arg_path, required=True)
    wg.set_defaults(func=cmd_workflow_guard)
    return ap


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
