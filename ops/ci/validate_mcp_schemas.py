#!/usr/bin/env python3
"""Validate mcps/coherence-mcp/tools/*.json tool descriptors.

ATOM: ATOM-VERIFY-PIPELINE-20260730-sm100
snake_case tool names only (0.4.x contract).
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOLS = ROOT / "mcps" / "coherence-mcp" / "tools"
NAME_RE = re.compile(r"^[a-z][a-z0-9_]*$")


def main() -> int:
    if not TOOLS.is_dir():
        print(f"::error::missing tools dir: {TOOLS}")
        return 1

    files = sorted(TOOLS.glob("*.json"))
    if not files:
        print("::error::no tool schemas found")
        return 1

    errors = 0
    names: list[str] = []
    for f in files:
        try:
            data = json.loads(f.read_text(encoding="utf-8"))
        except json.JSONDecodeError as e:
            print(f"::error file={f}::invalid JSON: {e}")
            errors += 1
            continue
        name = data.get("name")
        if not name:
            print(f"::error file={f}::missing name")
            errors += 1
            continue
        if name != f.stem:
            print(f"::warning file={f}::name {name!r} != filename stem {f.stem!r}")
        if not NAME_RE.match(name):
            print(
                f"::error file={f}::tool name must be snake_case "
                f"(0.4.x contract), got {name!r}"
            )
            errors += 1
        if "inputSchema" not in data and "input_schema" not in data:
            print(f"::error file={f}::missing inputSchema")
            errors += 1
        names.append(name)

    # Duplicates
    seen: set[str] = set()
    for n in names:
        if n in seen:
            print(f"::error::duplicate tool name {n}")
            errors += 1
        seen.add(n)

    print(f"mcp schemas: {len(files)} tools, errors={errors}")
    print(
        "note: LogOS may carry out-of-band tools beyond published "
        "coherence-mcp@0.4.2 (58). Drift is recorded, not smoothed."
    )
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
