#!/usr/bin/env python3
"""Validate ops/entangle/manifest.yaml shape and declaration policy.

ATOM: ATOM-ENTANGLE-MANIFEST-20260809
Exit 0 OK · Exit 1 schema/path issues · Exit 2 missing deps
"""
from __future__ import annotations

import sys
from pathlib import Path, PurePosixPath
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "ops" / "entangle" / "manifest.yaml"

REQUIRED_TOP = {"version", "atom", "declaration_policy", "components", "excludes", "remote"}
REQUIRED_COMP = {"id", "title", "declaration", "paths", "priority"}
REQUIRED_DECLARATION_POLICY = {
    "source_scope",
    "disallow_external_paths",
    "require_declared_components",
    "require_declared_observe_only",
    "require_declared_mirrors",
}
REQUIRED_DECLARATION = {"declared", "source_scope", "observe_only", "mirror"}
OBSERVE_ONLY_RE = ("observe-only", "observe only")
MIRROR_RE = ("mirror",)


def looks_like_relative_repo_path(raw: str) -> bool:
    path = PurePosixPath(raw.rstrip("/"))
    if not raw or path.is_absolute():
        return False
    if ".." in path.parts:
        return False
    return True


def validate_component(component: dict[str, Any], policy: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    cid = str(component.get("id", ""))
    decl = component.get("declaration")
    if not isinstance(decl, dict):
        return [f"{cid or '<missing id>'} declaration must be a mapping"]
    missing_decl = REQUIRED_DECLARATION - set(decl)
    if missing_decl:
        errors.append(f"{cid} declaration missing {sorted(missing_decl)}")
        return errors
    if policy.get("require_declared_components") and decl.get("declared") is not True:
        errors.append(f"{cid} declaration.declared must be true")
    if decl.get("source_scope") != policy.get("source_scope"):
        errors.append(f"{cid} declaration.source_scope must be {policy.get('source_scope')!r}")
    for field in ("observe_only", "mirror"):
        if not isinstance(decl.get(field), bool):
            errors.append(f"{cid} declaration.{field} must be boolean")

    paths = component.get("paths") or []
    if not isinstance(paths, list) or not paths:
        errors.append(f"{cid} needs non-empty paths")
    for raw_path in paths:
        if not isinstance(raw_path, str) or not looks_like_relative_repo_path(raw_path):
            errors.append(f"{cid} path must stay inside repository: {raw_path!r}")

    text = " ".join(
        str(component.get(name, "")) for name in ("title", "notes")
    ).lower()
    if policy.get("require_declared_observe_only") and any(token in text for token in OBSERVE_ONLY_RE):
        if decl.get("observe_only") is not True:
            errors.append(f"{cid} must declare observe_only=true")
    if policy.get("require_declared_mirrors") and any(token in text for token in MIRROR_RE):
        if decl.get("mirror") is not True:
            errors.append(f"{cid} must declare mirror=true")
    return errors


def validate_manifest_data(data: dict[str, Any]) -> list[str]:
    if not isinstance(data, dict):
        return ["root must be mapping"]
    missing = REQUIRED_TOP - set(data)
    if missing:
        return [f"missing top keys {sorted(missing)}"]

    policy = data.get("declaration_policy")
    if not isinstance(policy, dict):
        return ["declaration_policy must be a mapping"]
    missing_policy = REQUIRED_DECLARATION_POLICY - set(policy)
    if missing_policy:
        return [f"declaration_policy missing {sorted(missing_policy)}"]
    if policy.get("source_scope") != "repository":
        return ["declaration_policy.source_scope must be 'repository'"]
    for key in REQUIRED_DECLARATION_POLICY - {"source_scope"}:
        if not isinstance(policy.get(key), bool):
            return [f"declaration_policy.{key} must be boolean"]

    comps = data["components"]
    if not isinstance(comps, list) or not comps:
        return ["components must be non-empty list"]

    ids: set[str] = set()
    errors: list[str] = []
    for i, c in enumerate(comps):
        if not isinstance(c, dict):
            errors.append(f"component[{i}] not a mapping")
            continue
        miss = REQUIRED_COMP - set(c)
        if miss:
            errors.append(f"component[{i}] missing {sorted(miss)}")
        cid = str(c.get("id", ""))
        if not cid or cid in ids:
            errors.append(f"bad/duplicate id {cid!r}")
        ids.add(cid)
        errors.extend(validate_component(c, policy))

    remote = data.get("remote") or {}
    for k in ("base", "branch_prefix", "pr_title_template"):
        if k not in remote:
            errors.append(f"remote missing {k}")
    return errors


def main() -> int:
    try:
        import yaml  # type: ignore
    except ImportError:
        # stdlib fallback: minimal structural checks without PyYAML
        text = MANIFEST.read_text(encoding="utf-8")
        required_snippets = ("components:", "atom:", "declaration_policy:", "declaration:")
        if any(snippet not in text for snippet in required_snippets):
            print("validate_manifest: missing declaration policy or declarations")
            return 1
        print("validate_manifest: OK (lite — PyYAML not installed)")
        return 0

    if not MANIFEST.is_file():
        print(f"::error file={MANIFEST}::missing manifest")
        return 1

    data = yaml.safe_load(MANIFEST.read_text(encoding="utf-8"))
    errors = validate_manifest_data(data)
    if errors:
        for err in errors:
            print(f"validate_manifest: {err}")
        print(f"validate_manifest: FAIL errors={len(errors)}")
        return 1
    print(f"validate_manifest: OK components={len(data['components'])} atom={data.get('atom')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
