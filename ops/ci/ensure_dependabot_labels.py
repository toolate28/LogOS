#!/usr/bin/env python3
"""Ensure every label referenced by Dependabot exists in the repository.

ATOM: ATOM-LABELS-ENSURE-20260731

- Reads label names from .github/dependabot.yml
- Reads color/description catalog from .github/labels.json
- With --apply and GITHUB_TOKEN: creates any missing labels (never deletes)
- Without --apply: reports gaps and exits non-zero if any are missing

Capability ≠ authority: this only provisions labels so Dependabot can route;
it does not auto-merge or promote registers.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEPENDABOT = ROOT / ".github" / "dependabot.yml"
CATALOG = ROOT / ".github" / "labels.json"

API = "https://api.github.com"

# Fallback metadata when a Dependabot label is not yet in the catalog
FALLBACK_META: dict[str, dict[str, str]] = {
    "dependencies": {
        "color": "0366d6",
        "description": "Pull requests that update a dependency file",
    },
    "advisory": {
        "color": "d93f0b",
        "description": "Security / supply-chain advisory related (Dependabot, CodeQL, SPHINX-gated)",
    },
    "rust": {
        "color": "dea584",
        "description": "Rust / Cargo ecosystem updates and changes",
    },
    "github-actions": {
        "color": "2088FF",
        "description": "GitHub Actions workflow and action updates",
    },
    "python": {
        "color": "3572A5",
        "description": "Python / pip ecosystem updates and tooling",
    },
    "javascript": {
        "color": "168700",
        "description": "Pull requests that update javascript code",
    },
}


def load_catalog() -> dict[str, dict[str, str]]:
    meta = dict(FALLBACK_META)
    if CATALOG.is_file():
        data = json.loads(CATALOG.read_text(encoding="utf-8"))
        for entry in data.get("labels", []):
            name = entry.get("name")
            if not name:
                continue
            meta[name] = {
                "color": str(entry.get("color", "ededed")).lstrip("#"),
                "description": str(entry.get("description", ""))[:100],
            }
    return meta


def parse_dependabot_labels(path: Path) -> set[str]:
    """Extract label names from dependabot.yml without a YAML dependency."""
    if not path.is_file():
        print(f"::error file={path}::dependabot.yml not found")
        return set()

    text = path.read_text(encoding="utf-8")
    names: set[str] = set()
    in_labels = False
    for raw in text.splitlines():
        line = raw.rstrip()
        # Detect a labels: key (2+ spaces or list context)
        if re.match(r"^\s+labels:\s*$", line):
            in_labels = True
            continue
        if in_labels:
            m = re.match(r"^\s+-\s+([A-Za-z0-9][A-Za-z0-9._-]*)\s*$", line)
            if m:
                names.add(m.group(1))
                continue
            # Left the labels block (new key at same/less indent, or empty with next key)
            if re.match(r"^\s+[A-Za-z0-9_-]+:", line) or re.match(r"^\s*-\s+package-ecosystem:", line):
                in_labels = False
                continue
            if line.strip() == "":
                continue
            # Comment inside labels block
            if line.lstrip().startswith("#"):
                continue
            in_labels = False
    return names


def github_request(
    method: str,
    url: str,
    token: str,
    body: dict | None = None,
) -> tuple[int, object]:
    data = None if body is None else json.dumps(body).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        method=method,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "LogOS-ensure-dependabot-labels",
            "Content-Type": "application/json",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = resp.read().decode("utf-8")
            return resp.status, json.loads(raw) if raw else {}
    except urllib.error.HTTPError as e:
        raw = e.read().decode("utf-8", errors="replace")
        try:
            payload = json.loads(raw) if raw else {}
        except json.JSONDecodeError:
            payload = {"message": raw}
        return e.code, payload


def list_repo_labels(owner: str, repo: str, token: str) -> set[str]:
    names: set[str] = set()
    page = 1
    while True:
        status, payload = github_request(
            "GET",
            f"{API}/repos/{owner}/{repo}/labels?per_page=100&page={page}",
            token,
        )
        if status != 200 or not isinstance(payload, list):
            print(f"::error::list labels failed status={status} body={payload}")
            break
        if not payload:
            break
        for item in payload:
            if isinstance(item, dict) and "name" in item:
                names.add(item["name"])
        if len(payload) < 100:
            break
        page += 1
    return names


def create_label(
    owner: str,
    repo: str,
    token: str,
    name: str,
    color: str,
    description: str,
) -> bool:
    status, payload = github_request(
        "POST",
        f"{API}/repos/{owner}/{repo}/labels",
        token,
        {"name": name, "color": color, "description": description},
    )
    if status in (200, 201):
        print(f"created label: {name}")
        return True
    if status == 422:
        # already exists (race) — treat as success
        print(f"::notice::label already exists (422): {name}")
        return True
    print(f"::error::failed to create label {name!r}: status={status} body={payload}")
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--apply",
        action="store_true",
        help="Create missing labels via GitHub API (requires GITHUB_TOKEN)",
    )
    parser.add_argument(
        "--owner",
        default=os.environ.get("GITHUB_REPOSITORY", "/").split("/")[0] or "",
        help="Repository owner (default: GITHUB_REPOSITORY)",
    )
    parser.add_argument(
        "--repo",
        default=(
            os.environ.get("GITHUB_REPOSITORY", "/").split("/")[1]
            if "/" in os.environ.get("GITHUB_REPOSITORY", "")
            else ""
        ),
        help="Repository name (default: GITHUB_REPOSITORY)",
    )
    args = parser.parse_args()

    required = parse_dependabot_labels(DEPENDABOT)
    catalog = load_catalog()

    print(f"dependabot labels required: {sorted(required)}")

    unknown = sorted(n for n in required if n not in catalog)
    if unknown:
        print(
            f"::warning::labels referenced by dependabot.yml but absent from "
            f"catalog/fallback: {unknown} (will still attempt create with default color)"
        )

    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN") or ""

    if not args.apply:
        # Local / advisory mode: no API required
        if not token or not args.owner or not args.repo:
            print("ensure-dependabot-labels: dry-run (no API)")
            print(f"catalog covers: {sorted(set(catalog) & required)}")
            missing_meta = sorted(required - set(catalog))
            if missing_meta:
                print(f"::error::no metadata for: {missing_meta}")
                return 1
            print("ensure-dependabot-labels: PASS (dry-run, metadata complete)")
            return 0

        existing = list_repo_labels(args.owner, args.repo, token)
        missing = sorted(required - existing)
        if missing:
            print(f"::error::missing labels (re-run with --apply): {missing}")
            return 1
        print("ensure-dependabot-labels: PASS (all present)")
        return 0

    # --apply path
    if not token:
        print("::error::GITHUB_TOKEN required for --apply")
        return 1
    if not args.owner or not args.repo:
        print("::error::owner/repo required (set GITHUB_REPOSITORY or flags)")
        return 1

    existing = list_repo_labels(args.owner, args.repo, token)
    missing = sorted(required - existing)
    if not missing:
        print("ensure-dependabot-labels: PASS (nothing to create)")
        return 0

    print(f"creating missing labels: {missing}")
    failures = 0
    for name in missing:
        info = catalog.get(name, {"color": "ededed", "description": f"Auto-provisioned for Dependabot ({name})"})
        ok = create_label(
            args.owner,
            args.repo,
            token,
            name,
            info["color"],
            info["description"],
        )
        if not ok:
            failures += 1

    if failures:
        print(f"ensure-dependabot-labels: FAIL created_with_errors={failures}")
        return 1

    print("ensure-dependabot-labels: PASS (created missing labels)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
