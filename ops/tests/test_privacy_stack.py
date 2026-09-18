"""Privacy-stack validator tests (ATOM-PRIVACY-STACK-20260917).

Covers ops/ci/check_personal_paths.py and
ops/ci/validate_egress_declarations.py against temp trees via the
LOGOS_PRIVACY_ROOT override. Run with `pytest ops`.
"""
from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CHECK_PATHS = REPO / "ops" / "ci" / "check_personal_paths.py"
CHECK_EGRESS = REPO / "ops" / "ci" / "validate_egress_declarations.py"


def run(script: Path, root: Path, extra_env: dict | None = None):
    env = dict(os.environ, LOGOS_PRIVACY_ROOT=str(root))
    if extra_env:
        env.update(extra_env)
    return subprocess.run(
        [sys.executable, str(script)], capture_output=True, text=True, env=env
    )


# ---------------------------------------------------------------- personal paths


def git_tree(tmp_path: Path, files: dict[str, str]) -> Path:
    subprocess.run(["git", "init", "-q", str(tmp_path)], check=True)
    for rel, content in files.items():
        p = tmp_path / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(content, encoding="utf-8")
    subprocess.run(
        ["git", "-C", str(tmp_path), "add", "-A"],
        check=True,
        env=dict(
            os.environ,
            GIT_AUTHOR_NAME="t",
            GIT_AUTHOR_EMAIL="t@t",
            GIT_COMMITTER_NAME="t",
            GIT_COMMITTER_EMAIL="t@t",
        ),
    )
    return tmp_path


def test_rejects_windows_drive_path(tmp_path):
    root = git_tree(
        tmp_path, {".claude/settings.json": '{"root": "F:/Users/Someone/LogOS"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 1
    assert "absolute personal path" in r.stdout


def test_rejects_backslash_windows_path(tmp_path):
    root = git_tree(
        tmp_path, {".ai/mcp/mcp.json": '{"cwd": "C:\\\\Users\\\\Someone\\\\repo"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 1


def test_rejects_macos_users_path(tmp_path):
    root = git_tree(
        tmp_path, {".vscode/mcp.json": '{"path": "/Users/someone/repo/x"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 1


def test_rejects_linux_home_path(tmp_path):
    root = git_tree(
        tmp_path, {"ops/mcp/server.json": '{"path": "/home/someone/repo/x"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 1


def test_runner_home_is_exempt(tmp_path):
    root = git_tree(
        tmp_path, {".vscode/mcp.json": '{"path": "/home/runner/work/LogOS/x"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 0, r.stdout


def test_env_placeholders_pass(tmp_path):
    root = git_tree(
        tmp_path, {".claude/settings.json": '{"root": "${LOGOS_ROOT}"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 0, r.stdout


def test_out_of_scope_file_ignored(tmp_path):
    root = git_tree(
        tmp_path, {"docs/example.md": "F:/Users/Someone/LogOS is an example"}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 0, r.stdout


def test_nested_mcp_json_in_scope(tmp_path):
    root = git_tree(
        tmp_path, {"apps/foo/mcp-config.json": '{"cwd": "F:/Users/Someone/x"}'}
    )
    r = run(CHECK_PATHS, root)
    assert r.returncode == 1, r.stdout


# ------------------------------------------------------------------- egress


ALLOWLIST_INLINE = """\
version: 1
policy: deny-by-default
classes:
  none: []
  github-core:
    - github.com
workflows:
  a.yml: [github-core]
"""

ALLOWLIST_BLOCK = """\
version: 1
policy: deny-by-default
classes:
  none: []
  github-core:
    - github.com
workflows:
  a.yml:
    - github-core
"""


def egress_tree(tmp_path: Path, allowlist: str, workflows: list[str]) -> Path:
    wf = tmp_path / ".github" / "workflows"
    wf.mkdir(parents=True)
    for name in workflows:
        (wf / name).write_text("name: x\n", encoding="utf-8")
    al = tmp_path / "ops" / "ci"
    al.mkdir(parents=True)
    (al / "egress-allowlist.yaml").write_text(allowlist, encoding="utf-8")
    return tmp_path


def test_egress_declared_ok(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_INLINE, ["a.yml"])
    r = run(CHECK_EGRESS, root)
    assert r.returncode == 0, r.stdout


def test_egress_undeclared_workflow_fails(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_INLINE, ["a.yml", "b.yml"])
    r = run(CHECK_EGRESS, root)
    assert r.returncode == 1
    assert "no egress declaration" in r.stdout


def test_egress_stale_declaration_fails(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_INLINE, [])
    r = run(CHECK_EGRESS, root)
    assert r.returncode == 1
    assert "stale declaration" in r.stdout


def test_egress_undefined_class_fails(tmp_path):
    bad = ALLOWLIST_INLINE.replace("[github-core]", "[nonexistent]")
    root = egress_tree(tmp_path, bad, ["a.yml"])
    r = run(CHECK_EGRESS, root)
    assert r.returncode == 1
    assert "undefined egress class" in r.stdout


def _no_yaml_env(tmp_path: Path) -> dict:
    shim = tmp_path / "noyaml"
    shim.mkdir()
    (shim / "yaml.py").write_text(
        'raise ImportError("simulated absence")\n', encoding="utf-8"
    )
    return {"PYTHONPATH": str(shim)}


def test_egress_fallback_parser_inline(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_INLINE, ["a.yml"])
    r = run(CHECK_EGRESS, root, _no_yaml_env(tmp_path))
    assert r.returncode == 0, r.stdout


def test_egress_fallback_parser_block_list(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_BLOCK, ["a.yml"])
    r = run(CHECK_EGRESS, root, _no_yaml_env(tmp_path))
    assert r.returncode == 0, r.stdout


def test_egress_fallback_undeclared_fails(tmp_path):
    root = egress_tree(tmp_path, ALLOWLIST_BLOCK, ["a.yml", "b.yml"])
    r = run(CHECK_EGRESS, root, _no_yaml_env(tmp_path))
    assert r.returncode == 1
    assert "no egress declaration" in r.stdout
