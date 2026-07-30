# coherence-mcp 0.4.2 · drift ledger (LogOS view)

**Upstream:** `toolate28/coherence-mcp@main` · `b0d43208ae3a` · **0.4.2** · **58 tools**  
**LogOS tree:** `mcps/coherence-mcp/tools/` may carry **out-of-band** schemas beyond 58.

States: `!` broken · `~` stale wording · `?` unverified

| State | Item | Finding |
|-------|------|---------|
| ! | `server.version` | Upstream `src/index.ts` may still report 0.3.2 while package.json is 0.4.2 — patch upstream |
| ! | `npm run lint` | CONTRIBUTING historically referenced a missing script — fixed in docs-out rewrite |
| ~ | test path | `tests/` not `__tests__/`; watch = `test:watch` |
| ~ | WAVE ≥ 0.98 | SAIF-era; LogOS canonical publish gate is **85/100** |
| ~ | dotted tool names | `wave.analyze` / `ops.health` pre-0.4 — will not resolve; use snake_case |
| ? | security domain | advisories vs homepage host — confirm, do not guess |
| ? | out-of-band tools | LogOS extras (e.g. manifold / ifdown tools, and any tool not in the 58-array) stay LogOS-local until upstreamed |

## Tool naming contract

- **snake_case only** (`wave_analyze`, `ops_health`, `gauge_verify`)
- File stem == `name` field in JSON descriptor
- Enforced by `ops/ci/validate_mcp_schemas.py`

## Proven vs chosen

| Kind | Examples |
|------|----------|
| Proven (shipped 0.4.2) | 58-tool array, WAVE 0–100 gate table, snake_case |
| Chosen (LogOS) | company registry skeleton, manifold 2D projection, ifdown/ifup tools, CI verification pipeline |

Drift is **recorded, not hidden**.
