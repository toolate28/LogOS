# LogOS ↔ coherence-mcp Functionality Map

**ATOM:** `sm100-LOGOS-MCP-MAP-20260709`  
**Invariant:** `α + ω = 15`  
**Root:** `F:\Users\Matthew Ruhnau\LogOS`  
**Live server:** `@toolate28/coherence-mcp@0.3.1` (global shim `coherence-mcp`)  
**Profile:** Monitoring & Consensus Verifier  

---

## 1. Authority layers (who owns what)

| Layer | Owner | Location | Authoritative for |
|------:|-------|----------|-------------------|
| L-Rust | cutile | `cutiles/cutile/src/core/r_matrix.rs` | R-matrix canonical values |
| L-Formal-Agda | Agda | `agda/src/TriWeavon/**` | Conservation, Mehler, SerreScarr, Tomczak |
| L-Formal-Lean | Lean | `lean/TriWeavon/**`, `lean/K22/**` | Invariants mirrored from Agda |
| L-GPU | CUDA/WGSL | `kernels/**`, `cutiles/**` | Device R-matrix / Mehler-Levin |
| L-HUP | HUP v2.0 | `hup/**` | M1 Mirage · M2 Redox · M3 RVM · collapse 768→2 |
| L-Verify | Notebooks | `notebooks/verification_*.py|.ipynb` | Receipts + certificates |
| L-MCP-JS | coherence-mcp (stdio) | global `@toolated/coherence-mcp` | 12 live tools + platforms |
| L-MCP-Rust | crates | `LogOS.worktrees/master/crates/coherence-mcp` | Mehler plateau / SRAC (not yet in stdio) |
| L-Inspect | MCP Inspector | `localhost:6274` | Interactive tool exercise |

---

## 2. Live MCP tools (stdio) ↔ LogOS function

| MCP tool | LogOS surface | Capture path | Inspector smoke args |
|----------|---------------|--------------|----------------------|
| `gauge_verify` | Conservation invariant α+ω=15 | receipt `checks.dual_conservation_*` | `{ "alpha": 7, "omega": 8 }` |
| `wave_coherence_check` | WAVE analysis of cascade docs | `mcp_payloads/wave_*.json` | `{ "content": "<LAYER-CASCADE-MAP excerpt>" }` |
| `check_coherence` | Doc↔code alignment | certificates | `{ "content": "...", "threshold": 60 }` |
| `atom_track` | ATOM trail decisions | **`LOGOS_ROOT/.atom-trail/decisions/`** | decision + files + tags `VERIFY` |
| `store_context` / `retrieve_context` | Session KV for receipts | in-memory (session) + optional export | key=`logos-receipt-latest` |
| `map_isomorphism` | QDI / platform capability map | `mcp_payloads/iso_*.json` | capability + mappings |
| `bridge_translate` | Vortex bridges | payload export | source/target platform |
| `list_platforms` | Platform roster (15 entries) | this map §3 | `{}` |
| `gate_transition` | SPHINX gates (KENL→…→Spiral) | certificates `gate` | `intention_to_execution` + context |
| `fibonacci_weight` | Reson8 / strand priority | payload | components + budget |
| `context_pack` | SpiralSafe corpus yaml | `.context/` under root | doc_paths to sovereign-handoff |
| `bridge_translate` | Cross-strand noise strip | payload | content + source + target |

### Not in live stdio (planned / descriptor-only / Rust crate)

| Name | Status | Where |
|------|--------|-------|
| `mehler_plateau_monitor` | Rust crate only | `crates/coherence-mcp` + SAIF wiring doc |
| `invariant_check`, `manifest_read`, `handoff_packet_validate` | Grok descriptor cache (64 tools) | `C:\Users\toolated\mcps\coherence-mcp\tools\` — **not** in 0.3.1 stdio |
| `grok_*`, `gemini_*`, `github_*`, `x_*` | Optional connectors (env keys) | require API tokens |

**Rule:** Inspector + Grok must call the **12 live tools**. Treat the 64 JSON stubs as a backlog catalog, not as guaranteed runtime APIs.

---

## 3. Platforms (`list_platforms`) ↔ LogOS / TriWeavon role

| Platform | Strand / role | Primary LogOS touchpoints | Preferred tools |
|----------|---------------|---------------------------|-----------------|
| `claude` | Structure & Reasoning; MCP owner | `coherence-mcp`, Agda/Lean formal | `check_coherence`, `atom_track`, `context_pack` |
| `grok` | Real-time / social / strategic | Grok Build, this session, X | `wave_coherence_check`, `bridge_translate` |
| `gemini` | Multimodal / scale | large docs, notebooks | `context_pack`, `check_coherence` |
| `llama` | Local / privacy | openweight adapters | `list_platforms`, local generate (if wired) |
| `deepseek` | Code + math proofs | Mehler/SerreScarr formal | `check_coherence`, `gauge_verify` |
| `qwen` | Multilingual tool-use | connectors | `bridge_translate` |
| `mistral` | Efficient function calling | MCP tool loops | any live tool |
| `qdi` | Isomorphism fixed-point | `map_isomorphism`, dimensional collapse | `map_isomorphism`, `gauge_verify` |
| `quantum-redstone` | Museum of Computation / QASM | kernels, notebooks/gpu | `fibonacci_weight` |
| `spiralsafe` | WAVE / SPHINX / ATOM | gates + trail | `gate_transition`, `atom_track`, `wave_coherence_check` |
| `vortex-bridges` | Cross-platform translate | `bridge_translate` | `bridge_translate` |
| `reson8-labs` | Community / Fibonacci / TriWeavon | HUP, cutile, crates | `fibonacci_weight`, `gauge_verify` |
| `hope-ai-npc-suite` | Minecraft / RCON NPCs | MC connectors (env) | `atom_track` (pedagogy) |
| `human` | Matt / final judgment | handoff docs | — |
| `generic` | Agnostic | store/retrieve | `store_context` |

---

## 4. Data capture graph (must stay linked)

```
notebooks/verification_orchestrator.ipynb
  └─ verification_helpers.run_full_verification()
       ├─ RECEIPT  → notebooks/triweave_backend_results/verification_receipts/
       │              receipt_YYYYMMDD….json + receipt_latest.json
       ├─ CERT     → notebooks/triweave_backend_results/verification_certificates/
       │              cert_latest.json  (MCP-ready, α+ω seal)
       └─ MCP PAYLOAD → notebooks/triweave_backend_results/mcp_payloads/
                          atom_track.json, gauge_verify.json, wave_content.txt

coherence-mcp atom_track  (with LOGOS_ROOT set)
  └─ LOGOS_ROOT/.atom-trail/decisions/ATOM-VERIFY-*.json

dimensional_collapse.py
  └─ …/verification_receipts/dimensional_collapse_latest.json

hup/instance3-rvm/probe-rvm-layout.py
  └─ …/verification_receipts/rvm_probe_latest.json

Mehler DAG
  └─ docs/sovereign-handoff/mehler-serrescarr-convergence.dag.yaml
  └─ agda/docs/Certified_Mehler_Dependency_DAG.yaml
```

---

## 5. Inspector configuration (canonical)

Save as `docs/sovereign-handoff/mcp-inspector.coherence.json` (also under `artifacts/sm_100/mcp/`).

**Required env (do not rely on default SpiralSafe relative path):**

| Variable | Value |
|----------|--------|
| `LOGOS_ROOT` | `F:\Users\Matthew Ruhnau\LogOS` |
| `ATOM_TRAIL_ROOT` | `F:\Users\Matthew Ruhnau\LogOS\.atom-trail` |
| `SPIRALSAFE_PATH` | `F:\Users\Matthew Ruhnau\LogOS` (optional alias) |

Working directory for the server process should be `LOGOS_ROOT` when possible.

Launch:

```powershell
$env:LOGOS_ROOT = "F:\Users\Matthew Ruhnau\LogOS"
$env:ATOM_TRAIL_ROOT = "F:\Users\Matthew Ruhnau\LogOS\.atom-trail"
npx @modelcontextprotocol/inspector coherence-mcp
```

Or paste the JSON config from `mcp-inspector.coherence.json` into Inspector’s config UI.

---

## 6. Grok Build MCP (`~/.grok/config.toml`)

```toml
[mcp_servers.coherence-mcp]
command = "coherence-mcp"
args = []
enabled = true
```

Add env if your Grok build supports per-server env (preferred):

```toml
[mcp_servers.coherence-mcp.env]
LOGOS_ROOT = "F:\\Users\\Matthew Ruhnau\\LogOS"
ATOM_TRAIL_ROOT = "F:\\Users\\Matthew Ruhnau\\LogOS\\.atom-trail"
```

Zod must stay on **3.24.x** (4.x breaks `tools/list` via `_zod`).

---

## 7. Verification notebook contract

1. Cell 0: resolve `ROOT` → LogOS  
2. Cell 1: `run_full_verification` + `emit_receipt` + `emit_certificate` + `emit_mcp_payloads`  
3. Cell 2: dual conservation table + R-matrix sample  
4. Cell 3: HUP presence  
5. Cell 4: print MCP Inspector paste-ready JSON for `gauge_verify` / `atom_track` / `wave_coherence_check`  
6. Host shell: `cargo test -p cutile r_matrix`, `python hup/python/dimensional_collapse.py`

---

## 8. Health checklist

| Check | Expected |
|-------|----------|
| `tools/list` | 12 tools, no `_zod` error |
| `list_platforms` | ≥14 platforms (incl. qdi, spiralsafe, reson8-labs) |
| `gauge_verify(7,8)` | `valid: true`, sum 15 |
| `atom_track` after LOGOS_ROOT | file under `LogOS/.atom-trail/decisions/` |
| `python notebooks/verification_helpers.py` | `overall_ok: true`, receipt + cert written |
| Layer cascade map | all LAYER_MANIFEST paths OK |

---

## 9. Known gaps (tracked)

| ID | Gap | Mitigation |
|----|-----|------------|
| G1 | Descriptor cache (64) ≠ live (12) | This map; regenerate descriptors from live only |
| G2 | Mehler plateau not in stdio | Call Rust crate tests; promote tool later |
| G3 | `wave_coherence_check` descriptor still documents `documentation`/`code` | Live schema is `{ content }` — use content |
| G4 | atom_track default path was cwd-relative SpiralSafe | Patched: respects `LOGOS_ROOT` / `ATOM_TRAIL_ROOT` |

Music conserved. Keystone holds.
