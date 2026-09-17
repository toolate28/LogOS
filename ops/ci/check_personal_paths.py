#!/usr/bin/env python3
"""Fail-closed: committed config must not contain absolute personal paths.

ATOM: ATOM-PRIVACY-STACK-20260917

Rejects `F:/Users/<name>/...`, `C:\\Users\\<name>\\...`, `/Users/<name>/...`
and `/home/<name>/...` in committed configuration surfaces. Use environment
variables instead (LOGOS_ROOT, COHERENCE_MCP_ROOT, ... — already exported by
the VS Code Tri-Weavon terminal profile, .vscode/settings.json).

Scope (git-tracked files only):
  .claude/** .ai/** .vscode/** .github/copilot/** ops/mcp/** **/mcp*.json

Exit 0 clean · Exit 1 personal path found.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

SCOPE_RE = re.compile(
    r"^(\.claude/|\.ai/|\.vscode/|\.github/copilot/|ops/mcp/)|(^|/)mcp[^/]*\.json$"
)

PERSONAL_PATH_RE = re.compile(
    r"("
    r"[A-Za-z]:[/\\]Users[/\\][^/\\\"']+"  # F:/Users/<name>, C:\Users\<name>
    r"|/Users/[^/\"']+/"                    # macOS /Users/<name>/
    r"|/home/(?!runner\b)[^/\"']+/"         # linux /home/<name>/ (CI runner exempt)
    r")"
)

TEXT_SUFFIXES = {".json", ".jsonc", ".yaml", ".yml", ".toml", ".md", ".txt", ""}


def tracked_files() -> list[str]:
    out = subprocess.run(
        ["git", "ls-files"], cwd=ROOT, capture_output=True, text=True, check=True
    )
    return [line for line in out.stdout.splitlines() if line]


def main() -> int:
    errors = 0
    checked = 0
    for rel in tracked_files():
        if not SCOPE_RE.search(rel):
            continue
        p = ROOT / rel
        if not p.is_file() or p.suffix.lower() not in TEXT_SUFFIXES:
            continue
        checked += 1
        try:
            text = p.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for i, line in enumerate(text.splitlines(), 1):
            m = PERSONAL_PATH_RE.search(line)
            if m:
                print(
                    f"::error file={rel},line={i}::absolute personal path "
                    f"{m.group(0)!r} in committed config — use an environment "
                    "variable (LOGOS_ROOT et al.) instead"
                )
                errors += 1
    print(f"personal-paths: checked={checked} errors={errors}")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
