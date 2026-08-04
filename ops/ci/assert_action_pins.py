#!/usr/bin/env python3
"""Fail-closed: every GitHub Actions `uses:` must be a full 40-char commit SHA.

ATOM: ATOM-CI-POLICY-PINS-20260804
Repo setting sha_pinning_required=true makes tag refs (v7, stable) illegal.

Exit 0 if all pins are full SHAs (or no uses found).
Exit 1 if any floating ref is present.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WF = ROOT / ".github" / "workflows"
USES_RE = re.compile(r"^\s*uses:\s*(?P<action>[^\s@#\"']+)@(?P<ref>[^\s#\"']+)")
SHA_RE = re.compile(r"^[0-9a-fA-F]{40}$")


def main() -> int:
    if not WF.is_dir():
        print("assert_action_pins: no .github/workflows — skip")
        return 0

    bad: list[str] = []
    ok = 0
    for path in sorted(WF.glob("*.yml")) + sorted(WF.glob("*.yaml")):
        for i, line in enumerate(path.read_text(encoding="utf-8", errors="ignore").splitlines(), 1):
            # skip comments
            stripped = line.lstrip()
            if stripped.startswith("#"):
                continue
            m = USES_RE.match(line)
            if not m:
                continue
            action, ref = m.group("action"), m.group("ref")
            rel = path.relative_to(ROOT).as_posix()
            if SHA_RE.match(ref):
                ok += 1
            else:
                bad.append(f"{rel}:{i}: {action}@{ref}")

    print(f"assert_action_pins: pinned={ok} floating={len(bad)}")
    for b in bad:
        print(f"::error file={b.split(':')[0]},line={b.split(':')[1]}::{b} must be full 40-char commit SHA")
    if bad:
        print("sha_pinning_required: fix uses: owner/name@<40-hex> # version")
        return 1
    print("assert_action_pins: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
