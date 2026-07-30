# LogOS MCP company registry (skeleton)

**ATOM:** `ATOM-MCP-REGISTRY-20260730-sm100`

Higher-leverage control complementary to file-based `mcp-validation.yml`.
Point GitHub Org/Enterprise → AI controls → MCP → **Registry URL** at this
service base (do **not** append `/v0.1/servers` — Copilot adds the path).

Then set **Restrict MCP access to registry servers = Registry only**.

## API (registry v0.1)

| Method | Path |
|--------|------|
| GET | `/v0.1/servers` |
| GET | `/v0.1/servers/{serverName}/versions/latest` |
| GET | `/v0.1/servers/{serverName}/versions/{version}` |

CORS on those routes:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, OPTIONS
Access-Control-Allow-Headers: Authorization, Content-Type
```

## Curated default list

| Server | Endpoint | Tools posture |
|--------|----------|---------------|
| github | `https://api.githubcopilot.com/mcp/readonly` | read-only subset |
| coherence-mcp | local / npm `@toolated` or `toolate28/coherence-mcp` | snake_case 0.4.2 surface |

Do **not** treat `registry.modelcontextprotocol.io` as an automatic allowlist.

## Run locally

```bash
# from repo root
python ops/mcp/registry/serve_registry.py --host 127.0.0.1 --port 8787
# Registry base URL for GitHub settings (tunnel or deploy HTTPS in prod):
#   http://127.0.0.1:8787
```

Static catalog: `ops/mcp/registry/catalog/servers.json`.

## WAVE scale (canonical)

Publish gate is **85 on 0–100** (same as normalised 0.85).  
SAIF-era WAVE ≥ 0.98 is **superseded** as the default publish gate (see
`docs/security/WAVE-SCALE.md`).
