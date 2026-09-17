#!/usr/bin/env python3
"""Validate ops/entangle/manifest.yaml shape.

ATOM: ATOM-ENTANGLE-MANIFEST-20260809
Exit 0 OK · Exit 1 schema/path issues · Exit 2 missing deps
"""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "ops" / "entangle" / "manifest.yaml"

REQUIRED_TOP = {
    "version",
    "atom",
    "components",
    "excludes",
    "remote",
    "component_classes",
    "source_bindings",
    "invariant_profiles",
    "closure_states",
    "narrow_wormhole",
}
REQUIRED_COMP = {"id", "title", "paths", "priority"}
REQUIRED_SOURCE_BINDING = {"trust_class", "authority_effect", "required_fields"}
REQUIRED_PROFILE = {"allowed_involutions", "return_conditions", "noise_erasures", "failure_states"}
REQUIRED_WORMHOLE = {"authority_effect", "discussion_required", "ephemeral_root_required"}
REQUIRED_STATES = {"observed", "normalized", "reconciled", "drifted", "closure-ready", "held"}


def _is_string_list(value: object, *, allow_empty: bool = False) -> bool:
    return isinstance(value, list) and all(isinstance(v, str) and v for v in value) and (allow_empty or bool(value))


def main() -> int:
    try:
        import yaml  # type: ignore
    except ImportError:
        text = MANIFEST.read_text(encoding="utf-8")
        for key in REQUIRED_TOP:
            if f"{key}:" not in text:
                print(f"validate_manifest: missing {key} (lite)")
                return 1
        print("validate_manifest: OK (lite — PyYAML not installed)")
        return 0

    if not MANIFEST.is_file():
        print(f"::error file={MANIFEST}::missing manifest")
        return 1

    data = yaml.safe_load(MANIFEST.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        print("validate_manifest: root must be mapping")
        return 1

    missing = REQUIRED_TOP - set(data)
    if missing:
        print(f"validate_manifest: missing top keys {sorted(missing)}")
        return 1

    errors = 0
    component_classes = data.get("component_classes") or {}
    if not isinstance(component_classes, dict) or not component_classes:
        print("validate_manifest: component_classes must be non-empty mapping")
        errors += 1
    else:
        for name, spec in component_classes.items():
            if not isinstance(spec, dict):
                print(f"validate_manifest: component_classes.{name} must be mapping")
                errors += 1
                continue
            if not _is_string_list(spec.get("admissible_sources")):
                print(f"validate_manifest: component_classes.{name}.admissible_sources must be non-empty string list")
                errors += 1
            if not _is_string_list(spec.get("verify"), allow_empty=True):
                print(f"validate_manifest: component_classes.{name}.verify must be string list")
                errors += 1

    source_bindings = data.get("source_bindings") or {}
    if not isinstance(source_bindings, dict) or not source_bindings:
        print("validate_manifest: source_bindings must be non-empty mapping")
        errors += 1
    else:
        for name, spec in source_bindings.items():
            if not isinstance(spec, dict):
                print(f"validate_manifest: source_bindings.{name} must be mapping")
                errors += 1
                continue
            miss = REQUIRED_SOURCE_BINDING - set(spec)
            if miss:
                print(f"validate_manifest: source_bindings.{name} missing {sorted(miss)}")
                errors += 1
            if spec.get("authority_effect") != "none":
                print(f"validate_manifest: source_bindings.{name}.authority_effect must be none")
                errors += 1
            if not _is_string_list(spec.get("required_fields")):
                print(f"validate_manifest: source_bindings.{name}.required_fields must be non-empty string list")
                errors += 1

    invariant_profiles = data.get("invariant_profiles") or {}
    if not isinstance(invariant_profiles, dict) or not invariant_profiles:
        print("validate_manifest: invariant_profiles must be non-empty mapping")
        errors += 1
    else:
        for name, spec in invariant_profiles.items():
            if not isinstance(spec, dict):
                print(f"validate_manifest: invariant_profiles.{name} must be mapping")
                errors += 1
                continue
            miss = REQUIRED_PROFILE - set(spec)
            if miss:
                print(f"validate_manifest: invariant_profiles.{name} missing {sorted(miss)}")
                errors += 1
            for key in REQUIRED_PROFILE:
                if not _is_string_list(spec.get(key)):
                    print(f"validate_manifest: invariant_profiles.{name}.{key} must be non-empty string list")
                    errors += 1
            if spec.get("authority_effect") != "none":
                print(f"validate_manifest: invariant_profiles.{name}.authority_effect must be none")
                errors += 1

    states = data.get("closure_states")
    if not _is_string_list(states):
        print("validate_manifest: closure_states must be non-empty string list")
        errors += 1
    elif not REQUIRED_STATES.issubset(set(states)):
        print(f"validate_manifest: closure_states must include {sorted(REQUIRED_STATES)}")
        errors += 1

    wormhole = data.get("narrow_wormhole") or {}
    if not isinstance(wormhole, dict):
        print("validate_manifest: narrow_wormhole must be mapping")
        errors += 1
    else:
        miss = REQUIRED_WORMHOLE - set(wormhole)
        if miss:
            print(f"validate_manifest: narrow_wormhole missing {sorted(miss)}")
            errors += 1
        if wormhole.get("authority_effect") != "none":
            print("validate_manifest: narrow_wormhole.authority_effect must be none")
            errors += 1

    comps = data["components"]
    if not isinstance(comps, list) or not comps:
        print("validate_manifest: components must be non-empty list")
        return 1

    ids: set[str] = set()
    for i, c in enumerate(comps):
        if not isinstance(c, dict):
            print(f"validate_manifest: component[{i}] not a mapping")
            errors += 1
            continue
        miss = REQUIRED_COMP - set(c)
        if miss:
            print(f"validate_manifest: component[{i}] missing {sorted(miss)}")
            errors += 1
        cid = str(c.get("id", ""))
        if not cid or cid in ids:
            print(f"validate_manifest: bad/duplicate id {cid!r}")
            errors += 1
        ids.add(cid)
        paths = c.get("paths") or []
        if not _is_string_list(paths):
            print(f"validate_manifest: {cid} needs non-empty string paths")
            errors += 1
        verify = c.get("verify", [])
        if verify != [] and not _is_string_list(verify):
            print(f"validate_manifest: {cid}.verify must be string list")
            errors += 1
        cclass = c.get("class")
        if cclass is not None and cclass not in component_classes:
            print(f"validate_manifest: {cid}.class unknown {cclass!r}")
            errors += 1
        admissible = c.get("admissible_sources")
        if admissible is not None:
            if not _is_string_list(admissible):
                print(f"validate_manifest: {cid}.admissible_sources must be string list")
                errors += 1
            else:
                unknown = sorted(set(admissible) - set(source_bindings))
                if unknown:
                    print(f"validate_manifest: {cid}.admissible_sources unknown {unknown}")
                    errors += 1
        profile = c.get("invariant_profile")
        if profile is not None and profile not in invariant_profiles:
            print(f"validate_manifest: {cid}.invariant_profile unknown {profile!r}")
            errors += 1

    remote = data.get("remote") or {}
    for k in ("base", "branch_prefix", "pr_title_template"):
        if k not in remote:
            print(f"validate_manifest: remote missing {k}")
            errors += 1

    if errors:
        print(f"validate_manifest: FAIL errors={errors}")
        return 1
    print(f"validate_manifest: OK components={len(ids)} atom={data.get('atom')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
