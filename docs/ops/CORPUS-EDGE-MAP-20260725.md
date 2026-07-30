# Corpus edge map — verified vs asserted · negative space

**ATOM:** `ATOM-CORPUS-EDGE-MAP-20260725`  
**Audience:** NotebookLM / Gemini / Claude routing · NLM-LOADBEARING-TRIPLE  
**Refresh:** 2026-07-30 hub → `docs/notebooklm/NOTEBOOKLM-SINGLE-IMPORT-LOADBEARING-20260730.txt`  
**Rule:** Grep-traced edges ≠ asserted edges. Voids are first-class output.  
**SoT cascade:** `docs/sovereign-handoff/LAYER-CASCADE-MAP.md`  
**Confidence:** `tw confidence` / `logos-confidence` (ops/LogOS.Confidence.psm1)

Legend:

| Mark | Meaning |
|------|---------|
| **[G]** | Grep/file/live probe verified this session or in-repo path exists + code reference |
| **[A]** | Asserted in docs/packets — not re-traced to a call site |
| **[V]** | Labeled void — three+ clusters orbit a concept none fully states |

Category: **A** observation · **B** build claim · **C** convention · **D** proposal.

---

## 1. Cluster inventory (nodes)

| ID | Cluster | Primary paths **[G]** |
|----|---------|------------------------|
| C1 | Formal Lean/Agda | `lean/`, `agda/`, `lean/AgdaLeanBridge.md` |
| C2 | Cutile / R-matrix / CUDA | `cutiles/cutile`, `kernels/*.cu` |
| C3 | reson8-tui needle | `crates/tui` · `lsp.rs` · bin `reson8-forge` |
| C4 | Unitary / shell axis | `ops/TriWeavon.Unitary.Profile.psm1`, `ops/LogOS.Shell.psm1`, `ops/LogOS.Windows.psm1` |
| C5 | Claude Code cert | `CLAUDECODE-INIT-v0_1.md`, `ops/claude-code/*`, `.atom-trail/certs/` (state) |
| C6 | Edge wrangler | root `wrangler.toml`, SpiralSafe, adhealth, reson8-Labs workers |
| C7 | coherence-mcp | sibling `coherence-mcp/`, `ops/logos-mcp.mjs`, site `public/` |
| C8 | AdHealth product | `adhealth-meaningseed/` |
| C9 | 9P / Styx / dump | `9P2000.L/`, User_Dropfiles dump (worktree) |
| C10 | Stitch / design HTML | `stitch/`, `docs/surfaces/` |
| C11 | HUP multi-instance | `hup/` |
| C12 | Cascade map / handoff | `docs/sovereign-handoff/` |
| C13 | Arrival detector | dump `ArrivalDetector*.agda` · **not** in `agda/src` **[G]** |

---

## 2. Edges — grep-verified **[G]**

| From → To | Evidence |
|-----------|----------|
| C4 → C3 | Shell docs list `logos-tui` → `cargo run -p reson8-tui` (`LogOS.Shell.md`) |
| C3 → C1 | `crates/tui/src/lsp.rs` spawns `lake serve` / `als`; Formal pane **[G]** |
| C5 → C6 | `Emit-ClaudeCodeCert.ps1` probes wrangler + toml candidates **[G]** |
| C5 → C1 | Same emit probes lean/agda paths + PATH binaries **[G]** |
| C5 → C4 | `CLAUDECODE-INIT` + settings name cert dir; Confidence module reads cert **[G]** |
| C4 → C5 | `tw confidence` → `Show-TriWeavonConfidence` → `LogOS.Confidence.psm1` **[G]** |
| C4 → C6 | `logos-wrangler` / preflight wrangler probe in `LogOS.Windows.psm1` **[G]** |
| C7 → C4 | `logos-mcp.mjs` + shell `logos-mcp` **[G]** (if node present) |
| C2 → C12 | LAYER-CASCADE-MAP lists L2 cutile · L3 CUDA **[G]** |
| C1 → C12 | Map L4 Lean · L6 Agda · L10 bridge **[G]** |
| C8 → C6 | `adhealth-meaningseed/wrangler.toml` **[G]** |
| C3 → C7 | Bridge default `ws://127.0.0.1:8088` in tui main **[G]** (port; peer process optional) |
| C4 → C9 | Unitary `tw up styx` starts styx-bookshelf **[G]** command string |
| C10 → C7 | stitch `_shared/mcp-client.js` exists under stitch **[G]** path presence |

---

## 3. Edges — asserted-not-traced **[A]**

| From → To | Claim | Gap |
|-----------|--------|-----|
| C3 → C5 | “TUI shows cert / sensors” | Sensor bus for `cc_cert` documented in MARKERS-SENSORS; not grepped to a live emit path in TUI loop |
| C1 → C2 | “R-matrix identity Lean↔CUDA” | AgdaLeanBridge + cascade map; no auto checksum gate in CI grepped |
| C7 → C5 | “MCP gate_transition = cert pass” | Doctrine only; no shared cert file reader in MCP tools grepped |
| C13 → C3 | “ArrivalDetector diagnostics in Formal pane” | Detector only in dump; not under `agda/src` for als |
| C6 → C7 | “Vectorize 768/cosine immutable live” | Config asserts bindings; **live** CF index dims not whoami-verified |
| C8 → C2 | “AdHealth GPU via cutile --gpu” | Docs claim; runtime path not traced this session |
| C11 → C1 | “HUP M1/M2/M3 seal formal” | Consensus docs; live unikernel health not grepped |
| NLM pack → all | “Mind-map branches = live tree” | NotebookLM import pack is routing **[A]** until notebook run |

---

## 4. Labeled voids **[V]** (negative space = product)

### V1 — **Live formal eye**
**Orbits:** C1 (sources), C3 (lsp.rs scaffold), C5 (cert pass rules).  
**Unstated center:** *Who proves Lean/Agda diagnostics are actually flowing into the Formal pane right now?*  
**Status:** Scaffold **[G]** · attach **[B]** · `als`/`agda` missing on host **[G]** · continuous Novikov **[B]**.  
**Need:** host als + cubical pin GB-01 + one grepped `publishDiagnostics` hit in TUI.

### V2 — **Deploy authority loop**
**Orbits:** C5 (cert), C6 (wrangler), C4 (human shell).  
**Unstated center:** *Single machine-readable object that means “VERIFY passed, BUILD may deploy” without self-cert.*  
**Status:** cert path **[G]** · emit never full pass **[G]** · CF login FILL_ME **[G]** · mark-is-sensor hits **[A]**.  
**Need:** Claude Code cold-start overwrite of `latest.json` + optional TUI Formal green only when live.

### V3 — **R-matrix isomorphism seal**
**Orbits:** C1, C2, C12.  
**Unstated center:** *One receipt that Lean proof, cutile matrix, and CUDA host agree bit-for-bit (or explicitly refuse).*  
**Status:** cascade map + interface headers **[G]** · verification orchestrator notebook **[A]** completeness · ML-1..5 open in map **[G]**.  
**Need:** orchestrator receipt path grepped green on this host.

### V4 — **Arrival as product**
**Orbits:** C13 dump, C3 Formal, C10 Four Gates HTML.  
**Unstated center:** *Detector category label survives round-trip into deploy telemetry.*  
**Status:** Agda files in dump **[G]** · not in `agda/src` **[G]** · HTML surfaces **[G]** · wire **[A/B]**.  
**Need:** promote ArrivalDetector under pin; sensors at gate boundaries into TUI.

### V5 — **768-D product spine**
**Orbits:** C6 Vectorize, C7 embeddings, C8 AdHealth graph, cascade L18 collapse.  
**Unstated center:** *One embedding contract (dims/metric) enforced at write and read.*  
**Status:** wrangler bindings **[G]** · live index config **[V/A]** · dimensional_collapse.py **[G]** path · not wired to Workers **[A]**.  
**Need:** whoami + Vectorize describe; freeze contract in schema.

### V6 — **Tri-product “load-bearing triple”**
**Orbits:** LogOS (C3–C5), coherence-mcp (C7), AdHealth (C8).  
**Unstated center:** *Shared conservation + WAVE gate that all three products emit without forking α/ω semantics.*  
**Status:** NLM pack **[A]** · separate repos/paths **[G]** · coherence-mcp tools vs docs count drift **[A]** in pack notes.  
**Need:** single HAVE/NEED matrix regenerated by `logos-confidence` + MCP list.

---

## 5. Cascade order (load-bearing) — for NotebookLM

```
formal (C1) → cutile R-matrix (C2) → MCP gates (C7) → product surfaces (C3,C6,C8,C10)
                 ↑                            ↑
            void V3                      void V2/V6
```

**Blockers (HAVE / NEED):**

| HAVE **[G]** | NEED | BLOCKER void |
|--------------|------|----------------|
| wrangler 4.x CLI repaired | CF auth when deploying | V2 |
| cert emit path + honest pass:false | Claude Code real init cert | V2 |
| tui lsp.rs scaffold | live lake/als attach | V1 |
| lean sources + lake | toolchain pin vs elan 4.32 | V1 |
| agda sources | als binary + cubical pin | V1/V4 |
| confidence board | endemic in default profile banner (optional) | — |
| LAYER-CASCADE-MAP | orchestrator receipts on host | V3 |

---

## 6. 1-click confidence (endemic)

```powershell
. $PROFILE
tw confidence                 # or: logos-confidence
tw confidence -Refresh        # re-probe cert emit
logos-pop -Command "logos-confidence"
```

Edge marks on the board: **[G]** grepped · **[A]** asserted · **[V]** void.

---

---

## 7. NLM load-bearing triple overlay (2026-07-25)

Pack: `NLM-LOADBEARING-TRIPLE-2026-07-25` · products LogOS · coherence-mcp · AdHealth.

### Cascade (do not invert) — edge class

```
L1 empirical [A notebooks exist]
  → L2 cutile R-matrix [G path]
    → L3/L5 CUDA/WGSL [G path · host nvcc often missing = amber]
      → L4/L6 Lean/Agda [G sources · B live typecheck/als]
        → L11/L12 receipts [A orchestrator completeness]
          → L22 MCP stdio [V live tool count 12 vs 58–64 catalog]
            → product surfaces AdHealth/stitch/triweave/SAIF [G paths · B ship]
```

### LB → void map

| LB | Blocks | Void |
|----|--------|------|
| LB-1 Epistemic hygiene | false green / magic 15 | — (discipline) |
| LB-2 R-matrix receipts | MCP gauge without layer hash | **V3** |
| LB-3 Live MCP tool truth | AdHealth calling missing tools | **V6** catalog drift |
| LB-4 VERIFY Claude Code cert | deploy, “production ready” | **V2** |
| LB-5 Human deploy waist | SAIF cloud / receipt bucket | **V2** + human A1 |
| LB-6 Formal shadow (sorry/cubical/OB2) | flawless-formal claims | **V1** / **V4** |
| LB-7 Product orbit (coadjoint→apps) | theory without buyer pull | **V5** / AdHealth R-A* |

### Human ship gates (SAIF) — category

| Gate | Owner | Class |
|------|-------|--------|
| A1 GCP/Cloud Run | Human ⚑ | B until observed |
| A2 git surface reconcile | Human/agent after unlock | B · blocks D6 head_sha |
| B1 Claude Code cold-start cert | Claude Code ⚒ after A2 | **LB-4** · `tw confidence` |
| Formal 9 sorry MonomialWitness | Formal strand | A observation when grepped |
| Cubical pin | Decision | B |
| OB2 strain/vorticity | Formal | B |

### Product HAVE/NEED (compress)

| Product | HAVE **[G]** sample | NEED (pack) | Void |
|---------|---------------------|-------------|------|
| LogOS | cascade map, tui lsp scaffold, cert path, wrangler CLI, K22/MOG tree | origin reconcile, live D1–D5, als, ML-1 angles | V1 V2 V3 |
| coherence-mcp | package trees, capability docs | **list_tools truth**, tests, ATOM-AUTH, Mehler-in-process | V6 |
| AdHealth | CTQW CLI, portal, wrangler path | live ad APIs, weekly brief, calibration vs CPA | V5 |

---

**Music conserved. Negative space named. Do not promote [A] or [V] to [G] without a trace.**

