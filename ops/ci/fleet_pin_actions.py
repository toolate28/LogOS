#!/usr/bin/env python3
"""Fleet treatment: pin GitHub Actions uses: to full SHAs + optional sha_pinning_required.

ATOM: ATOM-FLEET-ACTION-PINS-20260804

Usage:
  python ops/ci/fleet_pin_actions.py --owner toolate28 --dry-run
  python ops/ci/fleet_pin_actions.py --owner toolate28 --apply
  python ops/ci/fleet_pin_actions.py --owner toolate28 --apply --enable-sha-pinning
  python ops/ci/fleet_pin_actions.py --owner toolate28 --only LogOS,coherence-mcp --apply

Requires: gh auth with repo + admin:org/actions permissions as needed.
"""
from __future__ import annotations

import argparse
import base64
import json
import re
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

# Match both `uses:` and list-item `- uses:` forms
USES_RE = re.compile(
    r"^(?P<indent>\s*)(?P<list>-\s*)?uses:\s*(?P<action>[^\s@#'\"]+)@(?P<ref>[^\s#'\"]+)(?P<rest>.*)$"
)
SHA_RE = re.compile(r"^[0-9a-fA-F]{40}$")
LOCAL_SHA_CACHE: dict[str, str] = {}

# Prefer pinning these first-party surfaces fully; forks still get pin rewrites
# when they have floating uses, but sha_pinning enable is opt-in for all.
FIRST_PARTY_HINTS = {
    "LogOS",
    "coherence-mcp",
    "SpiralSafe",
    "QDI",
    "reson8-Labs",
    "spiralsafe-mono",
    "compose-for-agents",
    "HOPE-AI-NPC-SUITE",
    "wave-toolkit",
    "spiralsafe-metrics-e",
    "AGI",
    "quantum-redstone",
    "vortex-bridges",
    "LEANN-QuaRC",
    "kenl",
    "jenkins",
    "AutoFigure",
    "hopeisclaudeishope",
    "locus-proxmox-infra",
}


def run_gh(args: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    cmd = ["gh", *args]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if check and proc.returncode != 0:
        raise RuntimeError(
            f"gh {' '.join(args)} failed ({proc.returncode}): {proc.stderr or proc.stdout}"
        )
    return proc


def gh_json(args: list[str]) -> Any:
    proc = run_gh(args, check=False)
    if proc.returncode != 0:
        return None
    text = (proc.stdout or "").strip()
    if not text:
        return None
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return None


def action_repo(action: str) -> str:
    # owner/name[/path...] -> owner/name
    parts = action.split("/")
    if len(parts) < 2:
        return action
    return f"{parts[0]}/{parts[1]}"


def resolve_sha(action: str, ref: str) -> str | None:
    key = f"{action}@{ref}"
    if key in LOCAL_SHA_CACHE:
        return LOCAL_SHA_CACHE[key]
    if SHA_RE.match(ref):
        LOCAL_SHA_CACHE[key] = ref
        return ref
    repo = action_repo(action)
    # Prefer commits API which accepts tags/branches
    data = gh_json(["api", f"repos/{repo}/commits/{ref}", "--jq", "{sha:.sha}"])
    if isinstance(data, dict) and data.get("sha"):
        LOCAL_SHA_CACHE[key] = data["sha"]
        return data["sha"]
    # Fallback: git ref tags
    proc = run_gh(
        ["api", f"repos/{repo}/git/ref/tags/{ref}", "--jq", ".object.sha,.object.type"],
        check=False,
    )
    if proc.returncode == 0 and proc.stdout.strip():
        lines = [ln.strip() for ln in proc.stdout.splitlines() if ln.strip()]
        if lines:
            sha = lines[0]
            # annotated tag
            if len(lines) > 1 and lines[1] == "tag":
                ann = gh_json(
                    ["api", f"repos/{repo}/git/tags/{sha}", "--jq", "{sha:.object.sha}"]
                )
                if isinstance(ann, dict) and ann.get("sha"):
                    LOCAL_SHA_CACHE[key] = ann["sha"]
                    return ann["sha"]
            if SHA_RE.match(sha):
                LOCAL_SHA_CACHE[key] = sha
                return sha
    return None


def pin_workflow_text(text: str) -> tuple[str, list[str], list[str]]:
    """Return (new_text, changes, unresolved)."""
    out_lines: list[str] = []
    changes: list[str] = []
    unresolved: list[str] = []
    for line in text.splitlines():
        m = USES_RE.match(line)
        if not m:
            out_lines.append(line)
            continue
        action = m.group("action")
        ref = m.group("ref")
        indent = m.group("indent")
        list_prefix = m.group("list") or ""
        # local composite actions ./
        if action.startswith("./") or action.startswith(".\\"):
            out_lines.append(line)
            continue
        if SHA_RE.match(ref):
            out_lines.append(line)
            continue
        # Map non-resolvable / bogus action refs to known good alternatives
        alias = {
            "PowerShell/Setup-PowerShell@v2": (
                "milliewalky/setup-pwsh",
                "v1",
            ),
            "zricethezav/gitleaks-action@latest": (
                "zricethezav/gitleaks-action",
                "v2",
            ),
        }
        lookup_key = f"{action}@{ref}"
        if lookup_key in alias:
            action, ref = alias[lookup_key]
        sha = resolve_sha(action, ref)
        if not sha:
            unresolved.append(f"{action}@{ref}")
            out_lines.append(line)
            continue
        comment = f" # {ref}"
        new_line = f"{indent}{list_prefix}uses: {action}@{sha}{comment}"
        if new_line != line and f"{action}@{sha}" not in line:
            changes.append(f"{lookup_key} -> {action}@{sha}")
        out_lines.append(new_line)
    # preserve final newline style
    new_text = "\n".join(out_lines)
    if text.endswith("\n"):
        new_text += "\n"
    return new_text, changes, unresolved


def list_repos(owner: str) -> list[dict[str, Any]]:
    data = gh_json(
        [
            "repo",
            "list",
            owner,
            "--limit",
            "200",
            "--json",
            "name,defaultBranchRef,isFork,isArchived",
        ]
    )
    return data or []


def list_workflow_files(owner: str, repo: str) -> list[str]:
    data = gh_json(["api", f"repos/{owner}/{repo}/contents/.github/workflows"])
    if not isinstance(data, list):
        return []
    names = []
    for item in data:
        name = item.get("name", "")
        if name.endswith((".yml", ".yaml")):
            names.append(name)
    return names


def get_file(owner: str, repo: str, path: str) -> tuple[str, str] | None:
    data = gh_json(["api", f"repos/{owner}/{repo}/contents/{path}"])
    if not isinstance(data, dict) or "content" not in data:
        return None
    content_b64 = data["content"].replace("\n", "")
    text = base64.b64decode(content_b64).decode("utf-8", errors="replace")
    return text, data.get("sha", "")


def put_file(
    owner: str,
    repo: str,
    path: str,
    content: str,
    message: str,
    branch: str,
    sha: str,
    dry_run: bool,
) -> bool:
    if dry_run:
        return True
    b64 = base64.b64encode(content.encode("utf-8")).decode("ascii")
    body = {
        "message": message,
        "content": b64,
        "branch": branch,
        "sha": sha,
    }
    proc = subprocess.run(
        [
            "gh",
            "api",
            "--method",
            "PUT",
            f"repos/{owner}/{repo}/contents/{path}",
            "--input",
            "-",
        ],
        input=json.dumps(body),
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(f"  !! put {path} failed: {proc.stderr or proc.stdout}")
        return False
    return True


def get_actions_permissions(owner: str, repo: str) -> dict[str, Any] | None:
    return gh_json(["api", f"repos/{owner}/{repo}/actions/permissions"])


def enable_sha_pinning(owner: str, repo: str, dry_run: bool) -> bool:
    perm = get_actions_permissions(owner, repo) or {}
    body = {
        "enabled": True if perm.get("enabled", True) else False,
        "allowed_actions": perm.get("allowed_actions") or "all",
        "sha_pinning_required": True,
    }
    # GitHub API: PUT /repos/{}/actions/permissions
    # Note: some accounts require allowed_actions when setting sha pin
    if dry_run:
        print(f"  [dry-run] would set sha_pinning_required=true for {owner}/{repo}")
        return True
    if not body["enabled"]:
        body["enabled"] = True
    proc = subprocess.run(
        [
            "gh",
            "api",
            "--method",
            "PUT",
            f"repos/{owner}/{repo}/actions/permissions",
            "--input",
            "-",
        ],
        input=json.dumps(body),
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        # try minimal body
        proc2 = subprocess.run(
            [
                "gh",
                "api",
                "--method",
                "PUT",
                f"repos/{owner}/{repo}/actions/permissions",
                "-f",
                "enabled=true",
                "-f",
                "allowed_actions=all",
                "-F",
                "sha_pinning_required=true",
            ],
            capture_output=True,
            text=True,
        )
        if proc2.returncode != 0:
            print(f"  !! enable sha pin failed: {proc.stderr or proc2.stderr}")
            return False
    print(f"  + sha_pinning_required=true")
    return True


def ensure_ci_policy(owner: str, repo: str, branch: str, dry_run: bool) -> bool:
    """Add a minimal ci-policy.yml if missing (checkout+setup-python pinned)."""
    path = ".github/workflows/ci-policy.yml"
    existing = get_file(owner, repo, path)
    if existing:
        return False
    checkout = resolve_sha("actions/checkout", "v7") or resolve_sha("actions/checkout", "v4")
    setup_py = resolve_sha("actions/setup-python", "v7") or resolve_sha(
        "actions/setup-python", "v5"
    )
    if not checkout or not setup_py:
        print("  !! cannot resolve pins for ci-policy scaffold")
        return False
    content = f"""# CI policy — fail-closed Action SHA pins
# ATOM: ATOM-FLEET-ACTION-PINS-20260804
# capability != authority

name: CI Policy

on:
  push:
    branches: [main, master]
    paths:
      - ".github/workflows/**"
  pull_request:
    branches: [main, master]
    paths:
      - ".github/workflows/**"
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ci-policy-${{{{ github.ref }}}}
  cancel-in-progress: true

jobs:
  action-pins:
    name: Action SHA pins (fail-closed)
    runs-on: ubuntu-latest
    timeout-minutes: 5
    steps:
      - name: Checkout
        uses: actions/checkout@{checkout} # v7-or-v4
      - name: Assert full-length commit SHAs
        run: |
          set -euo pipefail
          bad=0
          while IFS= read -r line; do
            file="${{line%%:*}}"
            rest="${{line#*:}}"
            ln="${{rest%%:*}}"
            uses="${{rest#*:}}"
            ref="${{uses##*@}}"
            ref="${{ref%% #*}}"
            ref="${{ref%%#*}}"
            ref="$(echo "$ref" | tr -d '[:space:]')"
            if ! echo "$ref" | grep -Eq '^[0-9a-fA-F]{{40}}$'; then
              echo "::error file=$file,line=$ln::unpinned uses: $uses"
              bad=1
            fi
          done < <(grep -RInE '^[[:space:]]*uses:[[:space:]]+[^./][^@]*@' .github/workflows --include='*.yml' --include='*.yaml' || true)
          if [[ "$bad" -ne 0 ]]; then
            echo "sha_pinning_required: pin every uses: to a full 40-char commit SHA"
            exit 1
          fi
          echo "assert_action_pins: OK"
"""
    if dry_run:
        print(f"  [dry-run] would add {path}")
        return True
    # create without prior sha
    b64 = base64.b64encode(content.encode("utf-8")).decode("ascii")
    body = {
        "message": "ci(policy): add fail-closed Action SHA pin workflow\n\nATOM: ATOM-FLEET-ACTION-PINS-20260804",
        "content": b64,
        "branch": branch,
    }
    proc = subprocess.run(
        [
            "gh",
            "api",
            "--method",
            "PUT",
            f"repos/{owner}/{repo}/contents/{path}",
            "--input",
            "-",
        ],
        input=json.dumps(body),
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        print(f"  !! create ci-policy failed: {proc.stderr or proc.stdout}")
        return False
    print(f"  + added {path}")
    return True


def process_repo(
    owner: str,
    repo: str,
    branch: str,
    *,
    apply: bool,
    enable_pin: bool,
    add_policy: bool,
    skip_forks: bool,
    is_fork: bool,
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "repo": repo,
        "branch": branch,
        "is_fork": is_fork,
        "workflows": 0,
        "files_changed": 0,
        "pins": 0,
        "unresolved": [],
        "sha_pin_before": None,
        "sha_pin_after": None,
        "errors": [],
    }
    if skip_forks and is_fork:
        result["skipped"] = "fork"
        return result

    perm = get_actions_permissions(owner, repo) or {}
    result["sha_pin_before"] = perm.get("sha_pinning_required")

    files = list_workflow_files(owner, repo)
    result["workflows"] = len(files)
    if not files:
        if enable_pin and apply:
            # still enable policy for empty workflow repos? skip
            pass
        return result

    print(f"\n== {owner}/{repo} ({branch}) workflows={len(files)} fork={is_fork}")
    for wf in files:
        path = f".github/workflows/{wf}"
        got = get_file(owner, repo, path)
        if not got:
            result["errors"].append(f"read-fail:{path}")
            continue
        text, sha = got
        new_text, changes, unresolved = pin_workflow_text(text)
        result["unresolved"].extend(unresolved)
        if not changes:
            continue
        result["pins"] += len(changes)
        print(f"  {wf}: {len(changes)} pin(s)")
        for c in changes[:8]:
            print(f"    - {c}")
        if len(changes) > 8:
            print(f"    ... +{len(changes)-8} more")
        msg = (
            f"ci(actions): pin workflow uses to full commit SHAs ({wf})\n\n"
            f"Fleet treatment for sha_pinning_required.\n"
            f"ATOM: ATOM-FLEET-ACTION-PINS-20260804"
        )
        if apply:
            ok = put_file(owner, repo, path, new_text, msg, branch, sha, dry_run=False)
            if ok:
                result["files_changed"] += 1
                time.sleep(0.4)  # gentle rate limit
            else:
                result["errors"].append(f"write-fail:{path}")
        else:
            result["files_changed"] += 1  # would change

    if add_policy and (repo in FIRST_PARTY_HINTS or not is_fork):
        if apply:
            ensure_ci_policy(owner, repo, branch, dry_run=False)
        else:
            ensure_ci_policy(owner, repo, branch, dry_run=True)

    if enable_pin:
        # Only enable after pins when we had floating refs that are now resolved,
        # or when already clean.
        if result["unresolved"] and apply:
            print(f"  !! skip enable sha_pin: unresolved {result['unresolved'][:5]}")
        else:
            ok = enable_sha_pinning(owner, repo, dry_run=not apply)
            if ok:
                result["sha_pin_after"] = True

    return result


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Fleet Action SHA pin treatment")
    ap.add_argument("--owner", default="toolate28")
    ap.add_argument("--apply", action="store_true", help="Commit changes via GitHub API")
    ap.add_argument(
        "--enable-sha-pinning",
        action="store_true",
        help="Set sha_pinning_required=true after pin rewrites",
    )
    ap.add_argument(
        "--add-ci-policy",
        action="store_true",
        help="Scaffold ci-policy.yml when missing (first-party/non-fork)",
    )
    ap.add_argument(
        "--include-forks",
        action="store_true",
        help="Also rewrite forks (default: skip forks)",
    )
    ap.add_argument(
        "--only",
        default="",
        help="Comma-separated repo names to process",
    )
    ap.add_argument(
        "--report",
        type=Path,
        default=Path("ops/ci/fleet-pin-report.json"),
    )
    args = ap.parse_args(argv)

    only = {x.strip() for x in args.only.split(",") if x.strip()}
    repos = list_repos(args.owner)
    if only:
        repos = [r for r in repos if r.get("name") in only]

    report: list[dict[str, Any]] = []
    print(
        f"Fleet pin: owner={args.owner} apply={args.apply} "
        f"enable_sha_pin={args.enable_sha_pinning} repos={len(repos)}"
    )

    for r in repos:
        if r.get("isArchived"):
            continue
        name = r["name"]
        branch = (r.get("defaultBranchRef") or {}).get("name") or "main"
        is_fork = bool(r.get("isFork"))
        try:
            res = process_repo(
                args.owner,
                name,
                branch,
                apply=args.apply,
                enable_pin=args.enable_sha_pinning,
                add_policy=args.add_ci_policy,
                skip_forks=not args.include_forks,
                is_fork=is_fork,
            )
        except Exception as exc:  # noqa: BLE001
            res = {"repo": name, "errors": [str(exc)]}
            print(f"\n== {name} ERROR: {exc}")
        report.append(res)

    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    changed = sum(1 for x in report if x.get("files_changed"))
    pins = sum(int(x.get("pins") or 0) for x in report)
    errs = sum(len(x.get("errors") or []) for x in report)
    print(
        f"\nDone. repos={len(report)} with_changes={changed} pins={pins} errors={errs}"
    )
    print(f"Report: {args.report}")
    return 1 if errs else 0


if __name__ == "__main__":
    sys.exit(main())
