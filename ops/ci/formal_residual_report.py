#!/usr/bin/env python3
"""Report Lean/Agda formal residuals (Category B) — not security defects.

ATOM: ATOM-VERIFY-PIPELINE-20260730-sm100
Always exits 0 unless the report itself cannot be produced.
"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

SORRY_RE = re.compile(r"\bsorry\b")
AXIOM_RE = re.compile(r"\baxiom\b")
HOLE_RE = re.compile(r"\{!!\}|\bpostulate\b")


def scan(dir_name: str, patterns: list[str]) -> dict:
    base = ROOT / dir_name
    stats = {"files": 0, "sorry": 0, "axiom": 0, "hole_or_postulate": 0, "paths": []}
    if not base.is_dir():
        return stats
    for pat in patterns:
        for f in base.rglob(pat):
            if any(p in f.parts for p in (".lake", "vendor", "node_modules", "MAlonzo")):
                continue
            try:
                text = f.read_text(encoding="utf-8", errors="ignore")
            except OSError:
                continue
            stats["files"] += 1
            s = len(SORRY_RE.findall(text))
            a = len(AXIOM_RE.findall(text))
            h = len(HOLE_RE.findall(text))
            stats["sorry"] += s
            stats["axiom"] += a
            stats["hole_or_postulate"] += h
            if s or a or h:
                stats["paths"].append(
                    f"{f.relative_to(ROOT)} sorry={s} axiom={a} hole/postulate={h}"
                )
    return stats


def main() -> int:
    lean = scan("lean", ["*.lean"])
    agda = scan("agda", ["*.agda", "*.lagda", "*.lagda.md"])

    print("## Formal residual report (Category B — not CVEs)")
    print()
    print("| Register | Files scanned | sorry | axiom | hole/postulate |")
    print("|----------|---------------|-------|-------|----------------|")
    print(
        f"| lean | {lean['files']} | {lean['sorry']} | {lean['axiom']} | {lean['hole_or_postulate']} |"
    )
    print(
        f"| agda | {agda['files']} | {agda['sorry']} | {agda['axiom']} | {agda['hole_or_postulate']} |"
    )
    print()
    print(
        "These residuals are **formal honesty markers**, not security vulnerabilities "
        "(see SECURITY.md). CodeQL does not analyze Lean/Agda."
    )
    print()
    for line in (lean["paths"] + agda["paths"])[:40]:
        print(f"- {line}")
    if len(lean["paths"] + agda["paths"]) > 40:
        print(f"- … {len(lean['paths'] + agda['paths']) - 40} more")

    summary_path = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary_path:
        with open(summary_path, "a", encoding="utf-8") as fh:
            fh.write("## Formal residual report (Category B)\n\n")
            fh.write(
                f"- Lean: files={lean['files']} sorry={lean['sorry']} axiom={lean['axiom']}\n"
            )
            fh.write(
                f"- Agda: files={agda['files']} sorry={agda['sorry']} axiom={agda['axiom']}\n"
            )
            fh.write("\nNot CVEs. tomczak_preserved · capability ≠ authority.\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())
