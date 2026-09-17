# Privacy-Preservation Stack

ATOM: `ATOM-PRIVACY-STACK-20260917` · Category B (bounded, operational) — a
declared + validated boundary, not a constitutional gate. capability ≠ authority.

The stack extends the declaration-first pattern already used by Ephemeral
Context (declared `source_root` scope) and ENTANGLE (`source_scope=repository`
+ `observe_only` manifests, `ops/entangle/manifest.yaml`) into a uniform
boundary around **all** tooling. Third-party indexers, analytics, and
telemetry are forbidden (AGENTS.md "Never Do").

## 1 · Local-only code intelligence

GitNexus and all third-party repository indexers are removed. Use the local
toolchain only — everything runs in-process with zero network egress:

| Need | Tool |
|------|------|
| Symbol navigation / call graphs (Rust) | `rust-analyzer` (`.vscode/settings.json` linked projects) |
| Lean | `lake serve` (`lean/`) |
| Agda | `als` (Category B: planned, not built) |
| Cross-language search | `ripgrep` / editor search |

If a persistent index is ever needed, build it with a local script under
`ops/` into a gitignored store (`.gitnexus/` stays in `.gitignore` as a
tombstone — never commit index artifacts).

## 2 · MCP allow-list with egress declarations

`coherence-mcp` is the sole first-party MCP server. Every server entry in a
committed MCP config (`.vscode/mcp.json`, `.ai/mcp/mcp.json`,
`.github/copilot/*.json`, `ops/mcp/*.json`) MUST carry an `x-privacy`
declaration:

```json
"x-privacy": {
  "network": "none | local | remote",
  "data_scope": "repository | declared-paths",
  "telemetry": "none",
  "endpoints": ["required when network=remote"]
}
```

Enforced fail-closed by `ops/ci/validate_mcp_config.py` (Rule 5) in
`.github/workflows/mcp-validation.yml`, mirroring how
`ops/entangle/validate_manifest.py` gates ENTANGLE components. `telemetry`
must be literally `"none"`.

## 3 · CI egress and provenance controls

- **Action pins** — every `uses:` is a full 40-char commit SHA
  (`ops/ci/assert_action_pins.py`, `ci-policy.yml`).
- **Egress declarations** — deny-by-default allowlist in
  `ops/ci/egress-allowlist.yaml`; every workflow must declare its egress
  classes or CI fails (`ops/ci/validate_egress_declarations.py`, wired into
  `ci-policy.yml`). Runtime egress-audit agents (e.g. harden-runner) were
  evaluated and **rejected**: they ship runner telemetry to an external
  service.
- **Manual-only external state** — workflows that touch external state stay
  `workflow_dispatch`-gated (Ephemeral Context / ENTANGLE Observe pattern).
- **Dependency review** — `.github/workflows/dependency-review.yml`
  (first-party action, SHA-pinned) surfaces new third-party packages on every
  PR; CodeQL and `security-advisory.yml` continue unchanged.

## 4 · Secret and PII hygiene

- **gitleaks** in `.pre-commit-config.yaml` (runs fully locally, no upload).
- **Personal-path guard** — `ops/ci/check_personal_paths.py` rejects absolute
  personal paths (`F:/Users/<name>/…`, `/Users/<name>/…`, `/home/<name>/…`)
  in committed config (`.claude/`, `.ai/`, `.vscode/`, `.github/copilot/`,
  `ops/mcp/`, any `mcp*.json`). Runs in `ops/ci/guard.sh` (Verify tree
  guards), `ops/githooks/pre-commit`, and pre-commit. Use environment
  variables instead — the Tri-Weavon terminal profile
  (`.vscode/settings.json`) already exports `LOGOS_ROOT`,
  `COHERENCE_MCP_ROOT`, `RESON8_LOGOS_ROOT`; machine-local roots referenced
  by `.claude/settings.json` additionally expect `SPIRALSAFE_ROOT`,
  `RESON8_LABS_ROOT`, `QDI_ROOT`, `ORCHESTRATOR_TUI_ROOT` in the local
  environment.
- **`.claude/settings.local.json` is local-only** — untracked and gitignored.
  It holds per-machine permission grants. Required local permissions to
  recreate it (grant only what you use):
  - `WebFetch(domain:raw.githubusercontent.com)` — fetch pinned upstream refs
  - `WebFetch(domain:christitus.com)` — optional, local tooling docs
  - `Bash(...)` loops used by local maintenance scripts
  - `Read(...)` grants for machine-local mirrors outside the repository

## 5 · Provenance as the source of truth

External observations route through the ATOM-trail / MARKS spine
(`ops/marks/MARKS.jsonl`, schema `ops/marks/spine.schema.json`) — never
through third-party analytics. Every tool that reads the repository emits a
declared, hash-anchored record; the 9P|Styx Bookshelf remains the single
persistent source of truth with no shadow copies in external services.

## Verification

```
python3 ops/ci/validate_mcp_config.py           # MCP x-privacy declarations
python3 ops/ci/validate_egress_declarations.py  # workflow egress allowlist
python3 ops/ci/check_personal_paths.py          # PII path hygiene
bash    ops/ci/guard.sh                         # tree guards (incl. above)
```
