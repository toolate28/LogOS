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

REQUIRED_TOP = {"version", "atom", "components", "excludes", "remote"}
REQUIRED_COMP = {"id", "title", "paths", "priority"}


def main() -> int:
    try:
        import yaml  # type: ignore
    except ImportError:
        # stdlib fallback: minimal structural checks without PyYAML
        text = MANIFEST.read_text(encoding="utf-8")
        if "components:" not in text or "atom:" not in text:
            print("validate_manifest: missing components/atom")
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

    comps = data["components"]
    if not isinstance(comps, list) or not comps:
        print("validate_manifest: components must be non-empty list")
        return 1

    ids: set[str] = set()
    errors = 0
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
        if not isinstance(paths, list) or not paths:
            print(f"validate_manifest: {cid} needs non-empty paths")
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
