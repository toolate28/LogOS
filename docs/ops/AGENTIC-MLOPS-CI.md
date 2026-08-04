# Agentic MLOps CI — CODEX code scanning & badges

**ATOM:** `ATOM-CODEX-MLOPS-20260804-sm100`  
**Status:** Active · deterministic gates · honest categories  
**capability ≠ authority**

## Purpose

Bespoke CI/CD for LogOS that:

1. **Fails closed** on real security / policy defects (unpinned Actions, secrets, MCP wildcards).
2. **Uploads SARIF** to GitHub Code Scanning under category `logos-codex`.
3. **Publishes shields endpoint badges** under `docs/badges/` for Agentic MLOps posture.
4. Treats **α + ω = 15** as **Category C telemetry only** — never a reject gate and never a proof.

## Workflow

| File | Name |
|------|------|
| `.github/workflows/codex-mlops.yml` | **CODEX / Agentic MLOps** |

Triggers: `push` / `pull_request` to default branches, weekly schedule (Tue 07:00 UTC), `workflow_dispatch`.

All third-party actions are **full-length commit SHA** pins (repo `sha_pinning_required: true`).

## Scanner

```bash
python ops/ci/codex_scan.py \
  --out artifacts/codex \
  --badge-dir docs/badges \
  --fail-on-security
```

| Layer | ID | Epistemic | Gate? |
|-------|----|-----------|-------|
| Action SHA pinning | L0 | A (policy-checkable) | **yes** |
| Secret path/content | L1 | B | **yes** |
| MCP fail-closed | L2 | A | **yes** |
| Formal residuals | L3 | B | no (inventory) |
| CI surface completeness | L4 | B | soft (score) |
| Agentic surface markers | L5 | B | soft (score) |
| Topology α/ω tag | L6 | **C** | **never** |

Composite score is a weighted rollup on **0–100**.  
**Publish posture** aligns with WAVE gate **≥ 85** (`docs/security/WAVE-SCALE.md`).

Exit code **1** only when security-category errors exist (unless `--no-fail-on-security`).

## Code Scanning (SARIF)

- Output: `artifacts/codex/codex.sarif`
- Upload: `github/codeql-action/upload-sarif` (SHA-pinned)
- Category: `logos-codex`
- Complements CodeQL Rust analysis (`codeql.yml`); does **not** replace it

Findings appear under **Security → Code scanning**.

## Badges

Endpoint JSON (Shields schemaVersion 1) in `docs/badges/`:

| File | Label |
|------|-------|
| `codex.json` | Composite CODEX score |
| `action-pin.json` | SHA-pin posture |
| `agentic-mlops.json` | pass / amber / blocked |
| `wave-posture.json` | publish≥85 · score |
| `topology-tag.json` | α+ω Category C tag |
| `ci-matrix.json` | agentic + codex rollup |

Markdown (after Pages serves `/docs`):

```markdown
[![CODEX](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/codex.json)](docs/ops/AGENTIC-MLOPS-CI.md)
[![Actions pin](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/action-pin.json)](docs/ops/AGENTIC-MLOPS-CI.md)
[![Agentic MLOps](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/agentic-mlops.json)](docs/ops/AGENTIC-MLOPS-CI.md)
[![WAVE posture](https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/wave-posture.json)](docs/security/WAVE-SCALE.md)
```

Workflow status badges (native GitHub):

```markdown
[![Verify](https://github.com/toolate28/LogOS/actions/workflows/verify.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/verify.yml)
[![CodeQL](https://github.com/toolate28/LogOS/actions/workflows/codeql.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/codeql.yml)
[![CODEX / Agentic MLOps](https://github.com/toolate28/LogOS/actions/workflows/codex-mlops.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/codex-mlops.yml)
[![Security Advisory](https://github.com/toolate28/LogOS/actions/workflows/security-advisory.yml/badge.svg)](https://github.com/toolate28/LogOS/actions/workflows/security-advisory.yml)
```

On pushes to `main`, the `publish-badges` job regenerates and commits `docs/badges/*` when values change. The workflow ignores `docs/badges/**` paths to avoid loops.

## Related automations

| Workflow | Role |
|----------|------|
| `verify.yml` | Tree guards · MCP schemas · formal residual · rust core/tui |
| `codeql.yml` | Rust SAST (CodeQL) |
| `security-advisory.yml` | Secret-path + lake artefacts · cargo-audit |
| `mcp-validation.yml` | Fail-closed committed MCP configs |
| `labels.yml` | Dependabot label ensure |
| `dependabot.yml` | Cargo + Actions (+ pip) with SHA-aware Actions updates |

## Deterministic upgrades

1. Pin every `uses:` to `@<40-hex> # version`.
2. Dependabot `github-actions` ecosystem updates those pins weekly (grouped).
3. Re-run CODEX after pin bumps; L0 must stay at 100.

## Local mirrors

```bash
python ops/ci/codex_scan.py
bash ops/ci/guard.sh
python ops/ci/validate_mcp_config.py
python ops/ci/formal_residual_report.py
# legacy topology demo (not the CI gate):
python docs/scripts/triweavon_codex_scanner.py .
```

## Honesty rules

- Do **not** re-badge Category B/C scores as Category A proofs.
- Formal `sorry` / `axiom` are **not** CVEs (see `SECURITY.md`).
- Badge green means **tooling posture**, not sovereign mathematical certification.
