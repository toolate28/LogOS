#!/usr/bin/env python3
"""Fail-closed: every workflow must declare its network egress classes.

ATOM: ATOM-PRIVACY-STACK-20260917

Deny-by-default allowlist lives in ops/ci/egress-allowlist.yaml.
Rules:
1. Every .github/workflows/*.yml|*.yaml has an entry in `workflows`.
2. Every declared class is defined under `classes`.
3. No stale declarations for workflows that no longer exist.

Exit 0 OK · Exit 1 policy violation or malformed allowlist.
"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

ROOT = Path(
    os.environ.get("LOGOS_PRIVACY_ROOT", Path(__file__).resolve().parents[2])
)
WF_DIR = ROOT / ".github" / "workflows"
ALLOWLIST = ROOT / "ops" / "ci" / "egress-allowlist.yaml"


def load_allowlist() -> tuple[dict[str, list[str]], set[str]] | None:
    text = ALLOWLIST.read_text(encoding="utf-8")
    try:
        import yaml  # type: ignore

        data = yaml.safe_load(text)
        if not isinstance(data, dict):
            print("egress-policy: allowlist root must be a mapping")
            return None
        classes = data.get("classes")
        workflows = data.get("workflows")
        if not isinstance(classes, dict) or not isinstance(workflows, dict):
            print("egress-policy: allowlist needs `classes` and `workflows` mappings")
            return None
        wf: dict[str, list[str]] = {}
        for name, cls in workflows.items():
            wf[str(name)] = [str(c) for c in (cls or [])]
        return wf, {str(c) for c in classes}
    except ImportError:
        # stdlib fallback: minimal line parser for the two mappings.
        # Handles both inline lists (`name: [a, b]`) and block lists
        # (`name:` followed by `  - a` items).
        classes: set[str] = set()
        workflows: dict[str, list[str]] = {}
        section = None
        current_wf: str | None = None
        for raw in text.splitlines():
            line = raw.split("#", 1)[0].rstrip()
            if not line.strip():
                continue
            if line == "classes:":
                section = "classes"
                current_wf = None
                continue
            if line == "workflows:":
                section = "workflows"
                current_wf = None
                continue
            if not line.startswith(" "):
                section = None
                current_wf = None
                continue
            item = re.match(r"^\s+-\s+(\S.*)$", line)
            if item:
                if section == "workflows" and current_wf is not None:
                    workflows[current_wf].append(item.group(1).strip().strip('"'))
                continue
            m = re.match(r"^  (\S[^:]*):\s*(.*)$", line)
            if not m:
                continue
            key, rest = m.group(1).strip().strip('"'), m.group(2).strip()
            if section == "classes":
                classes.add(key)
                current_wf = None
            elif section == "workflows":
                if rest:
                    inner = rest.strip("[]")
                    workflows[key] = [
                        c.strip().strip('"') for c in inner.split(",") if c.strip()
                    ]
                    current_wf = None
                else:
                    workflows[key] = []
                    current_wf = key
        if not classes or not workflows:
            print("egress-policy: lite parse failed (install PyYAML)")
            return None
        return workflows, classes


def main() -> int:
    if not WF_DIR.is_dir():
        print("egress-policy: no .github/workflows — skip")
        return 0
    if not ALLOWLIST.is_file():
        print("::error file=ops/ci/egress-allowlist.yaml::missing egress allowlist")
        return 1

    loaded = load_allowlist()
    if loaded is None:
        return 1
    declared, classes = loaded

    present = sorted(
        {p.name for p in list(WF_DIR.glob("*.yml")) + list(WF_DIR.glob("*.yaml"))}
    )

    errors = 0
    for name in present:
        if name not in declared:
            print(
                f"::error file=.github/workflows/{name}::no egress declaration in "
                "ops/ci/egress-allowlist.yaml (deny-by-default)"
            )
            errors += 1
            continue
        for cls in declared[name]:
            if cls not in classes:
                print(
                    f"::error file=ops/ci/egress-allowlist.yaml::workflow {name} "
                    f"references undefined egress class {cls!r}"
                )
                errors += 1

    for name in sorted(set(declared) - set(present)):
        print(
            f"::error file=ops/ci/egress-allowlist.yaml::stale declaration for "
            f"missing workflow {name}"
        )
        errors += 1

    print(f"egress-policy: workflows={len(present)} errors={errors}")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
