#!/usr/bin/env python3
"""Fail-closed policy for committed MCP configurations.

ATOM: ATOM-MCP-VALIDATE-20260730-sm100

Rules:
1. JSON well-formedness
2. No tools: ["*"] wildcards
3. No embedded secrets / classic PATs
4. Prefer GitHub MCP readonly endpoint (warn on full endpoint)
5. Every server entry carries an x-privacy egress declaration
   (network: none|local|remote, data_scope: repository|declared-paths,
   telemetry: none). Remote network additionally requires declared
   endpoints. Mirrors the ENTANGLE manifest declaration-first pattern
   (ATOM-ENTANGLE-MANIFEST-20260809).
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Globs relative to repo root (resolved manually for portability)
CANDIDATE_GLOBS = [
    "**/mcp.json",
    "**/mcp-config.json",
    "**/.vscode/mcp.json",
    "**/.ai/mcp/*.json",
    ".github/mcp/**/*.json",
    ".github/copilot/**/*.json",
    "ops/mcp/**/*.json",
]

SECRET_RE = re.compile(
    r"("
    r"ghp_[A-Za-z0-9]{20,}"
    r"|github_pat_[A-Za-z0-9_]{20,}"
    r"|sk-[A-Za-z0-9]{20,}"
    r"|xox[baprs]-[A-Za-z0-9-]{10,}"
    r"|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----"
    r"|api[_-]?key\s*[:=]\s*['\"][^'\"]{8,}"
    r"|Bearer\s+[A-Za-z0-9\-._~+/]+=*"
    r")",
    re.I,
)

FULL_GH_MCP = "https://api.githubcopilot.com/mcp/"
READONLY_GH_MCP = "https://api.githubcopilot.com/mcp/readonly"

SKIP_DIR_PARTS = {
    "node_modules",
    ".git",
    "target",
    ".lake",
    "vendor",
    "artifacts",
}


def iter_candidates() -> list[Path]:
    found: list[Path] = []
    # Explicit well-known locations
    well_known = [
        ROOT / "mcp.json",
        ROOT / "mcp-config.json",
        ROOT / ".vscode" / "mcp.json",
        ROOT / ".ai" / "mcp" / "mcp.json",
        ROOT / ".github" / "copilot" / "mcp-config.json",
        ROOT / ".github" / "copilot" / "mcp-config.example.json",
        ROOT / "ops" / "mcp" / "copilot-mcp.example.json",
    ]
    for p in well_known:
        if p.is_file():
            found.append(p)

    for base in (
        ROOT / ".github" / "mcp",
        ROOT / ".github" / "copilot",
        ROOT / "ops" / "mcp",
        ROOT / ".vscode",
        ROOT / ".ai" / "mcp",
    ):
        if not base.exists():
            continue
        for p in base.rglob("*.json"):
            if any(part in SKIP_DIR_PARTS for part in p.parts):
                continue
            if p.name.endswith(".schema.json"):
                continue
            found.append(p)

    # Dedup
    uniq = sorted({p.resolve() for p in found})
    return [Path(p) for p in uniq]


def walk_for_tools(obj, path: str = "$") -> list[tuple[str, list]]:
    hits: list[tuple[str, list]] = []
    if isinstance(obj, dict):
        if "tools" in obj and isinstance(obj["tools"], list):
            hits.append((path + ".tools", obj["tools"]))
        for k, v in obj.items():
            hits.extend(walk_for_tools(v, f"{path}.{k}"))
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            hits.extend(walk_for_tools(v, f"{path}[{i}]"))
    return hits


def walk_urls(obj) -> list[str]:
    urls: list[str] = []
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k in ("url", "serverUrl", "endpoint") and isinstance(v, str):
                urls.append(v)
            else:
                urls.extend(walk_urls(v))
    elif isinstance(obj, list):
        for v in obj:
            urls.extend(walk_urls(v))
    return urls


def walk_servers(obj) -> list[tuple[str, dict]]:
    """Yield (path, server_entry) for each server under servers/mcpServers maps."""
    hits: list[tuple[str, dict]] = []
    if isinstance(obj, dict):
        for key in ("servers", "mcpServers"):
            block = obj.get(key)
            if isinstance(block, dict):
                for name, entry in block.items():
                    if isinstance(entry, dict):
                        hits.append((f"$.{key}.{name}", entry))
        for v in obj.values():
            if isinstance(v, (dict, list)) and v is not obj.get("servers") and v is not obj.get("mcpServers"):
                hits.extend(walk_servers(v))
    elif isinstance(obj, list):
        for v in obj:
            hits.extend(walk_servers(v))
    return hits


PRIVACY_NETWORK = {"none", "local", "remote"}
PRIVACY_SCOPE = {"repository", "declared-paths"}


def check_privacy_declaration(rel, spath: str, entry: dict) -> int:
    """Fail-closed x-privacy egress declaration on each server entry."""
    errors = 0
    decl = entry.get("x-privacy")
    if not isinstance(decl, dict):
        print(
            f"::error file={rel}::server {spath} missing x-privacy declaration "
            "(network/data_scope/telemetry) — declaration-first boundary"
        )
        return 1
    network = decl.get("network")
    if network not in PRIVACY_NETWORK:
        print(
            f"::error file={rel}::server {spath} x-privacy.network must be one of "
            f"{sorted(PRIVACY_NETWORK)} (got {network!r})"
        )
        errors += 1
    scope = decl.get("data_scope")
    if scope not in PRIVACY_SCOPE:
        print(
            f"::error file={rel}::server {spath} x-privacy.data_scope must be one of "
            f"{sorted(PRIVACY_SCOPE)} (got {scope!r})"
        )
        errors += 1
    if decl.get("telemetry") != "none":
        print(
            f"::error file={rel}::server {spath} x-privacy.telemetry must be \"none\" "
            "(third-party telemetry forbidden)"
        )
        errors += 1
    if network == "remote":
        endpoints = decl.get("endpoints")
        if not isinstance(endpoints, list) or not endpoints:
            print(
                f"::error file={rel}::server {spath} declares network=remote but no "
                "x-privacy.endpoints allowlist"
            )
            errors += 1
    return errors


def main() -> int:
    files = iter_candidates()
    # Filter out pure registry catalog / package metadata that are not agent MCP configs
    config_files = [
        p
        for p in files
        if p.name
        in {
            "mcp.json",
            "mcp-config.json",
            "mcp-config.example.json",
            "copilot-mcp.example.json",
            "servers.json",
        }
        or "copilot" in p.parts
        or p.name.startswith("mcp")
    ]

    if not config_files:
        print("::notice::No committed MCP configuration found — OK (UI-only settings out of scope)")
        print("mcp-validation: PASS (empty set)")
        return 0

    errors = 0
    warnings = 0
    for f in config_files:
        rel = f.relative_to(ROOT)
        text = f.read_text(encoding="utf-8")
        # examples may contain placeholders but not real secrets
        try:
            data = json.loads(text)
        except json.JSONDecodeError as e:
            print(f"::error file={rel}::invalid JSON: {e}")
            errors += 1
            continue

        if SECRET_RE.search(text):
            # allow obvious placeholders
            if not re.search(r"YOUR_|REPLACE_|<.*>|\$\{", text):
                print(f"::error file={rel}::possible embedded secret material")
                errors += 1

        for tpath, tools in walk_for_tools(data):
            if any(t == "*" for t in tools if isinstance(t, str)):
                print(
                    f"::error file={rel}::wildcard tools [\"*\"] forbidden at {tpath} "
                    "(expands unconstrained network surface outside agent firewall)"
                )
                errors += 1

        for spath, entry in walk_servers(data):
            errors += check_privacy_declaration(rel, spath, entry)

        for url in walk_urls(data):
            if url.rstrip("/") == FULL_GH_MCP.rstrip("/") or url == FULL_GH_MCP:
                print(
                    f"::warning file={rel}::prefer GitHub MCP readonly endpoint "
                    f"({READONLY_GH_MCP}) over full {FULL_GH_MCP}"
                )
                warnings += 1
            elif FULL_GH_MCP.rstrip("/") in url and "readonly" not in url:
                print(
                    f"::warning file={rel}::non-readonly GitHub MCP URL: {url}"
                )
                warnings += 1

        print(f"checked: {rel}")

    print(f"mcp-validation: errors={errors} warnings={warnings}")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
