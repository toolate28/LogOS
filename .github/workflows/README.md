# LogOS verification pipeline (GitHub Actions)

**ATOM:** `ATOM-VERIFY-PIPELINE-20260730-sm100`

| Workflow | File | Role |
|----------|------|------|
| **CI Sequence** | `ci-sequence.yml` | Dispatch-only chain: policy → MCP → labels → security → CODEX → verify → CodeQL |
| **Verify** | `verify.yml` | Tree guards · JSON kit · MCP schemas · formal residual · rust core/tui |
| **CodeQL** | `codeql.yml` | Rust SAST (explicit `cargo build`) |
| **CI Policy** | `ci-policy.yml` | Fail-closed full SHA pin gate on every `uses:` |
| **CODEX / Agentic MLOps** | `codex-mlops.yml` | Topology-aware gates · SARIF code scanning · shields badges |
| **Security Advisory** | `security-advisory.yml` | Secret-path + lake artefacts · cargo-audit |
| **MCP Validation** | `mcp-validation.yml` | Fail-closed committed MCP configs |
| **Labels** | `labels.yml` | Ensure Dependabot-referenced labels exist |
| **Entangle** | `entangle.yml` | Remote PR path-slots · local slice ingest (dispatch only) |
| **Dependabot** | `../dependabot.yml` | Cargo + Actions (+ optional pip) |

### Badges

```markdown
[![CODEX / Agentic MLOps](https://github.com/toolate28/LogOS/actions/workflows/codex-mlops.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/codex-mlops.yml)
[![Verify](https://github.com/toolate28/LogOS/actions/workflows/verify.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/verify.yml)
[![CodeQL](https://github.com/toolate28/LogOS/actions/workflows/codeql.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/codeql.yml)
[![CODEX](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/codex.json)](../docs/ops/AGENTIC-MLOPS-CI.md)
[![Agentic MLOps](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/agentic-mlops.json)](../docs/ops/AGENTIC-MLOPS-CI.md)
```

Full matrix + epistemology: [`docs/ops/AGENTIC-MLOPS-CI.md`](../docs/ops/AGENTIC-MLOPS-CI.md).

## Design

- Findings are **advisory or policy gates** — not automatic promotion authority.
- Lean / Agda are outside CodeQL; residuals are Category B, not CVEs.
- WAVE publish gate is **85 on 0–100** (= 0.85). See `docs/security/WAVE-SCALE.md`.
- MCP network residual: org **Registry only** + this CI + readonly GitHub MCP.
- Labels workflow creates missing labels only (never deletes). Catalog: `.github/labels.json`.

## Action pinning (required)

Repo setting: `sha_pinning_required: true`. Every `uses:` must be a **full 40-char commit SHA**.

```yaml
uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7
```

- Keep the `# version` comment so Dependabot can propose deterministic upgrades.
- For `dtolnay/rust-toolchain`, pin the action SHA and set `with: toolchain: stable` (branch name is no longer the toolchain selector once SHA-pinned).

## Observed 2026-08-18 (origin `main`)

| Workflow | Evidence | Fix (this tree) |
|----------|----------|-----------------|
| **CI Policy** | 0s, no jobs, "workflow file issue" on every Dependabot push | Drop path filters — required-check + skip = ghost fail |
| **Verify** | `resonance-invariant` missing `alpha_value`/`omega_value` on origin | Getters live in local `crates/resonance-invariant` |
| **Pages** | Legacy Jekyll `submodules: recursive` → `No url found for submodule path 'crates/coherence-mcp'` | `pages.yml` (`submodules: false`). Repo still `build_type=legacy` until Pages source = Actions **and** `pages.yml` is on `main` |
| **CODEX / CodeQL / Security** | green | leave |

Do **not** dispatch Entangle `ingest`/`scaffold` or Pages deploy from the sequence (HITL / writes).

```bash
gh workflow run "CI Policy"
gh workflow run "MCP Validation"
gh workflow run "Labels"
gh workflow run "Security Advisory"
gh workflow run "CODEX / Agentic MLOps"
gh workflow run "Verify"
gh workflow run "CodeQL"
# after pages.yml is on main:
# gh api -X PUT repos/toolate28/LogOS/pages -f build_type=workflow
```

## Local mirrors

```bash
git config core.hooksPath ops/githooks
bash ops/ci/guard.sh
python ops/ci/validate_mcp_schemas.py
python ops/ci/validate_mcp_config.py
python ops/ci/formal_residual_report.py
python ops/ci/ensure_dependabot_labels.py          # dry-run
# GITHUB_TOKEN=... python ops/ci/ensure_dependabot_labels.py --apply
python ops/ci/assert_action_pins.py                # fail-closed SHA pins
python ops/ci/codex_scan.py                        # CODEX + badges + SARIF
python ops/ci/skill_chain_scan.py --self-test      # ChainGuard-shaped skill graph
```

## GCP Gemini Cloud Assist

Operator inventory (project number `952127156617`, Companion service agent, enabled APIs):

- [`docs/ops/GCP-GEMINI-CLOUD-ASSIST.md`](../docs/ops/GCP-GEMINI-CLOUD-ASSIST.md)
- Agent prompt: [`docs/ops/CLOUD-GEMINI-AGENT-PROMPT.md`](../docs/ops/CLOUD-GEMINI-AGENT-PROMPT.md)

## Manifold live surface

```bash
python ops/mcp/registry/serve_registry.py   # :8787
python ops/mcp/manifold_server.py           # :8790 → /manifold UI
```
