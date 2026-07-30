# GitHub MCP Registry — LogOS posture

**ATOM:** `ATOM-MCP-REGISTRY-20260730-sm100`

## Two concepts

| Concept | Role |
|---------|------|
| Public MCP Registry (`registry.modelcontextprotocol.io`) | Discovery only — **not** an automatic allowlist |
| Company registry (self-hosted or Azure API Center) | Curated allowlist for Copilot when **Registry only** is set |

## Org / Enterprise settings

1. AI controls → MCP → **Enabled**
2. MCP Registry URL → **base only** (e.g. `https://mcp-registry.example.com`)
3. Restrict MCP access → **Registry only**

Skeleton: `ops/mcp/registry/` (`serve_registry.py` + `catalog/servers.json`).

## Complements

| Control | Layer |
|---------|-------|
| Registry only | Discovery / installation |
| `.github/workflows/mcp-validation.yml` | Committed config policy |
| `.github/copilot/mcp-config.example.json` | Readonly tool list shape |
| Agent firewall | Bash only — **does not** cover MCP |

## Prefer

- GitHub MCP `…/mcp/readonly`
- Explicit tool lists (never `"*"`)
- No committed secrets

Any expansion of the allowed server set is a capability grant requiring a receipt (`tomczak_preserved`).
