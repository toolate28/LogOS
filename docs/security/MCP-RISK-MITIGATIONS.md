# MCP network residual — mitigations

The Copilot agent firewall does **not** constrain MCP servers.

## Ordered by leverage

1. **Org/Enterprise registry + Registry only** — `docs/security/MCP-REGISTRY.md`
2. **Repo MCP config** — readonly GitHub MCP; explicit tools; env-injected secrets
3. **CI `mcp-validation.yml`** — fail-closed on wildcards / embedded secrets
4. **Local custom servers** — bind `127.0.0.1`, least privilege, non-root
5. **CodeQL + secret scanning + githooks** — independent hygiene

## What not to do

- Do not disable the agent firewall because MCP is exempt
- Do not enable `"*"` tools for convenience
- Do not expose write tools / broad PATs by default
- Do not treat MCP config as a substitute for CodeQL or secret scanning

## WAVE (canonical)

| Gate | Scale 0–100 | Normalised |
|------|-------------|------------|
| BASELINE | 60 | 0.60 |
| EMERGENT | 80 | 0.80 |
| **PUBLISH** | **85** | **0.85** |
| CRITICAL | 99 | 0.99 |

SAIF-era WAVE ≥ 0.98 as default publish gate is **superseded** (see `WAVE-SCALE.md`).
