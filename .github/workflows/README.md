# LogOS verification pipeline (GitHub Actions)

**ATOM:** `ATOM-VERIFY-PIPELINE-20260730-sm100`

| Workflow | File | Role |
|----------|------|------|
| **Verify** | `verify.yml` | Tree guards · MCP schemas · formal residual report · rust core/tui |
| **CodeQL** | `codeql.yml` | Rust SAST (explicit `cargo build`) |
| **Security Advisory** | `security-advisory.yml` | Secret-path + lake artefacts · cargo-audit |
| **MCP Validation** | `mcp-validation.yml` | Fail-closed committed MCP configs |
| **Dependabot** | `../dependabot.yml` | Cargo + Actions (+ optional pip) |

## Design

- Findings are **advisory or policy gates** — not automatic promotion authority.
- Lean / Agda are outside CodeQL; residuals are Category B, not CVEs.
- WAVE publish gate is **85 on 0–100** (= 0.85). See `docs/security/WAVE-SCALE.md`.
- MCP network residual: org **Registry only** + this CI + readonly GitHub MCP.

## Local mirrors

```bash
git config core.hooksPath ops/githooks
bash ops/ci/guard.sh
python ops/ci/validate_mcp_schemas.py
python ops/ci/validate_mcp_config.py
python ops/ci/formal_residual_report.py
```

## Manifold live surface

```bash
python ops/mcp/registry/serve_registry.py   # :8787
python ops/mcp/manifold_server.py           # :8790 → /manifold UI
```
