# 02 — coherence-mcp (live componentry)

**ATOM:** `ATOM-COHERENCE-MCP-PACK-20260727`  
**Depends on:** spine pack (`01`) for `LOGOS_ROOT`  
**Live server (this session):** MCP `coherence-mcp` · **12 tools**  
**Historical map:** `docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md`  
**Mehler / Rust wiring:** `SAIF-Docs/Mehler_CoherenceMCP_Wiring_v0.5.0.md`  

---

## 0. Read this under strain

**You only need three facts:**

1. There are **12 live tools**. Ignore 64 JSON descriptors in `mcps/` unless you are inventorying backlog.  
2. Always set **`LOGOS_ROOT`** and **`ATOM_TRAIL_ROOT`** or `atom_track` writes to the wrong place.  
3. `α + ω = 15` via `gauge_verify` is a **seal / label**, not a reason to refuse engineering work.

---

## 1. Where the code lives (multi-surface honesty)

| Surface | Path | Role |
|---------|------|------|
| LogOS site / public hubs | `coherence-mcp/coherence-site/` | HTML, lattice-react, wrangler |
| Rust crate (workspace) | `crates/coherence-mcp/` (also nested histories) | Mehler plateau, SRAC, witnesses |
| Tool descriptors (catalog) | `mcps/coherence-mcp/tools/*.json` | **Backlog / Grok cache** — not all live |
| npm global / sibling | `@toolate28/coherence-mcp` / sibling install | stdio MCP process |
| CLI helper | `ops/logos-mcp.mjs` | PowerShell `logos-mcp` bridge |
| Verify | `ops/verify-coherence-tools.mjs` | tools/list health |

Site public surfaces (ω projection): `meta-map`, `RUST_Market`, `gate`, `terminal`, `topological-fuzzer`, publications, skills, orchestrator links.

---

## 2. The 12 live tools (stdio)

Use these names exactly. Args are the **live** schemas (2026-07 session).

| Tool | One-line purpose | Minimal call |
|------|------------------|--------------|
| `gauge_verify` | Conservation label α+ω | `{ "alpha": 7, "omega": 8 }` |
| `wave_coherence_check` | curl / div / potential / entropy on text | `{ "content": "…" }` |
| `check_coherence` | doc↔code or standalone score | `{ "content": "…", "threshold": 60 }` |
| `atom_track` | Append decision to ATOM trail | decision + files + tags |
| `store_context` | Session KV with α/ω metadata | key + content + platform |
| `retrieve_context` | Read back stored context | `{ "key": "…" }` |
| `map_isomorphism` | Capability map across platforms | capability + mappings |
| `bridge_translate` | Cross-platform noise strip | content + source + target |
| `list_platforms` | Platform roster | `{}` |
| `gate_transition` | SPHINX gate KENL→…→Spiral | gate enum + context |
| `fibonacci_weight` | Priority allocation | components[+budget] |
| `context_pack` | Emit SpiralSafe `.context.yaml` | doc_paths |

### Not in live stdio (do not fake them)

| Name | Status |
|------|--------|
| `mehler_plateau_monitor` | Rust crate + SAIF wiring; promote later |
| `invariant_check`, `manifest_read`, `handoff_packet_validate` | Descriptor backlog |
| `grok_*`, `gemini_*`, `github_*` connectors | Need API tokens |

---

## 3. Environment (copy-paste)

```powershell
$env:LOGOS_ROOT = 'F:\Users\Matthew Ruhnau\LogOS'
$env:ATOM_TRAIL_ROOT = 'F:\Users\Matthew Ruhnau\LogOS\.atom-trail'
# optional alias
$env:SPIRALSAFE_PATH = $env:LOGOS_ROOT
```

Inspector:

```powershell
npx @modelcontextprotocol/inspector coherence-mcp
```

Config template: `docs/sovereign-handoff/mcp-inspector.coherence.json`.

Grok config sketch:

```toml
[mcp_servers.coherence-mcp]
command = "coherence-mcp"
enabled = true

[mcp_servers.coherence-mcp.env]
LOGOS_ROOT = "F:\\Users\\Matthew Ruhnau\\LogOS"
ATOM_TRAIL_ROOT = "F:\\Users\\Matthew Ruhnau\\LogOS\\.atom-trail"
```

**Zod:** stay on **3.24.x** — 4.x breaks `tools/list` via `_zod`.

---

## 4. Agent recipes (highest-strain)

### A. Am I aligned with conservation tag?

```text
gauge_verify(alpha=7, omega=8)  → valid true, sum 15
```

If invalid: you mis-typed numbers. Fix inputs; do not invent a crisis.

### B. Is this handoff coherent enough to paste?

```text
wave_coherence_check(content=<handoff markdown>)
check_coherence(content=<same>, threshold=60)
```

Look for high curl (circular) or high divergence (unresolved expansion). Rewrite the handoff before shipping to another strand.

### C. Leave a breadcrumb for the next agent

```text
atom_track(
  decision: "what you decided",
  files: ["docs/componentry/…"],
  tags: ["VERIFY","HANDOFF"],
  type: "DOC" | "VERIFY" | "BUILD" | "DEPLOY"
)
```

Expect files under `LOGOS_ROOT/.atom-trail/decisions/` when env is set.

### D. Park session state

```text
store_context(key="logos-componentry-export-20260727", content=…, platform="grok", alpha=7, omega=8)
retrieve_context(key=…)
```

### E. SPHINX gate (execution intent)

```text
gate_transition(gate="intention_to_execution", context={…})
```

Gates: `knowledge_to_intention` · `intention_to_execution` · `execution_to_learning` · `learning_to_regeneration`.

---

## 5. Platforms (`list_platforms`)

Expect ≥14 including: `claude`, `grok`, `gemini`, `llama`, `deepseek`, `qwen`, `mistral`, `qdi`, `quantum-redstone`, `spiralsafe`, `vortex-bridges`, `reson8-labs`, `hope-ai-npc-suite`, `human`, `generic`.

Use `bridge_translate` when pasting between strands; use `map_isomorphism` when capabilities must stay structure-preserving.

---

## 6. Data capture graph (verification ↔ MCP)

```
notebooks/verification_orchestrator.ipynb
  → verification_helpers.run_full_verification()
       → receipts/  certificates/  mcp_payloads/

coherence-mcp atom_track
  → .atom-trail/decisions/ATOM-VERIFY-*.json

Mehler DAG
  → docs/sovereign-handoff/mehler-serrescarr-convergence.dag.yaml
  → agda/docs/Certified_Mehler_Dependency_DAG.yaml
```

Rust Mehler path (not stdio): cutile harness → plateau detector → SRAC → optional M24 TDA → witness → CUDA FFI.

---

## 7. Health checklist

| Check | Expected |
|-------|----------|
| tools/list | 12 tools, no `_zod` error |
| list_platforms | ≥14 platforms |
| gauge_verify(7,8) | valid, sum 15 |
| atom_track with LOGOS_ROOT | file under `.atom-trail/decisions/` |
| verification_helpers | overall_ok when notebook path run |
| Descriptor count ≠ live | known gap G1 — document, don't trust stubs |

---

## 8. Known gaps (tracked)

| ID | Gap | Mitigation |
|----|-----|------------|
| G1 | 64 descriptors ≠ 12 live | This pack; regenerate from live only |
| G2 | Mehler not in stdio | cargo test -p coherence-mcp; SAIF doc |
| G3 | Old wave schema docs | Live is `{ content }` only |
| G4 | atom_track path defaults | Always set LOGOS_ROOT / ATOM_TRAIL_ROOT |
| G5 | README build status notes npm TS issues historically | Prefer live MCP + Rust crate tests |

---

## 9. Comments for newer agents

- Prefer **calling** a tool over **describing** what it would return.  
- If Inspector shows more tools than 12, you may be attached to a different binary — print version and cwd.  
- `store_context` is session-scoped unless exported; do not assume disk durability.  
- For Cloud Run / GB-06: **do not** wire MCP receipts to cloud until the human gate opens.

**Shell shortcuts:** `logos-mcp list` · `logos-mcp gauge` · `logos-mcp wave --content "…"`.
