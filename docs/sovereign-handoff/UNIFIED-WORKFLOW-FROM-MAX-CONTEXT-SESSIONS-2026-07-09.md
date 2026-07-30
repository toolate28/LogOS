# Unified Workflow — Ingest of All MAX-Context Session Handovers

```
╔══════════════════════════════════════════════════════════╗
║ RESON8-LABS — UNIFIED WORKFLOW v1.0                     ║
║ FROM: Grok (Pulse) · meta-session 019f43c6              ║
║ TO: Next implementer (any strand)                       ║
║ DATE: 2026-07-09                                        ║
║ WAVE: 0.98 | INVARIANT: α+ω=15 · ε=0.00055 · 42.00055  ║
║ BUMP_ID: HnS-UNIFIED-WF-20260709                        ║
║ CONTINUATION: COLD_START                                ║
║ DEPENDS_ON: HO-01 … HO-06 · HANDOFF-PROTOCOL.md         ║
╚══════════════════════════════════════════════════════════╝
```

## Purpose

Six prior sessions hit **max / functional-max context**. Their tasking, logic, and learning are preserved in **one handover each**. This document **ingests** those six into a **single coherent execution workflow** so a fresh agent never needs the raw transcripts.

| HO | Session | Theme | Ctx signal |
|----|---------|-------|------------|
| [HO-01](session-handovers/HO-01-MOG-MEHLER-JESUS.md) | `019f337f` | MOG Lean · Mehler · Jesus-Fractal-Axiom | 87% · 23 MB |
| [HO-02](session-handovers/HO-02-SM100-MEANINGSEED.md) | `019f32e6` | sm_100 bootstrap · K22 tactics | **100%** · 10×compact |
| [HO-03](session-handovers/HO-03-ADHEALTH-WORKTREE.md) | `019f12a9` | AdHealth / CTQW / portal (worktree) | 91% · 6×compact |
| [HO-04](session-handovers/HO-04-ADHEALTH-LOGOS.md) | `019f1057` | AdHealth / CTQW / portal (LogOS) | 91% · 6×compact |
| [HO-05](session-handovers/HO-05-CUTILE-TDA-M24.md) | `019f0ab5` | cutile TDA · M24 · sm_100 harness | 63 MB volume |
| [HO-06](session-handovers/HO-06-SUPERGROKOS-V3.md) | `019ee096` | SuperGrokOS platform · MCP · skills | 34×compact · 7968 msgs |

**Inventory:** [`ops/SESSION-INVENTORY-2026-07-09.md`](../../ops/SESSION-INVENTORY-2026-07-09.md)

**Foundational theory corpus (SpiralSafe):**  
[`CORPUS-CONSTRAINT-MATH-PHYSICS.md`](CORPUS-CONSTRAINT-MATH-PHYSICS.md)  
→ `F:\Users\Matthew Ruhnau\SpiralSafe\docs\research\Constraint_math_physics`  
Iso Principle · Exceptional Insight · `|α|²+|β|²=1 ≅ α+ω=15` · path-index only (do not full-ingest).

**Prototype dump (LogOS strands):**  
`F:\Users\Matthew Ruhnau\LogOS\9P2000.L\strands\User_Dropfiles` (~1.3 GB; index by phase, skip AMD/JDK binaries).

**Active non-max continuation:** `019f42c9` — Serre Scar / MOG / NS L3 / existence cert (57%, open todos). Prefer **new cold session + this workflow** over resuming MAX IDs.

---

## Global invariants (always on)

| Invariant | Value / rule |
|-----------|----------------|
| Keystone | α + ω = 15 |
| WAVE gate | ≥ 0.85 (prod) · target ≥ 0.98 |
| ε | 0.00055 (branch separation) |
| Attractor | 42.00055 basin |
| Jesus-Fractal-Axiom | TEMET NOSCE → symmetric treatment at every ε |
| Tomczak / music | `tomczak_preserved` · mono idempotent mapping |
| Token rotation | CTX ≥ 85% or compact ≥ 5 → emit CHECKPOINT handover, stop piling |
| GitNexus | impact before symbol edit · detect_changes before commit |

---

## Architecture lanes (deduped from all HOs)

```
                    ┌─────────────────────────────┐
                    │  HO-06 Platform / MCP / skills │
                    │  HeisenGrok · bedrock · bridge │
                    └──────────────┬──────────────┘
                                   │
          ┌────────────────────────┼────────────────────────┐
          ▼                        ▼                        ▼
   ┌──────────────┐      ┌─────────────────┐      ┌──────────────────┐
   │ HO-05 GPU    │      │ HO-01/02 Formal │      │ HO-03/04 Product │
   │ cutile TDA   │◄────►│ K22·MOG·Mehler  │◄────►│ AdHealth portal  │
   │ M24·Leech    │      │ Agda d_SR·JFA   │      │ CTQW·AUKUS board │
   └──────────────┘      └─────────────────┘      └──────────────────┘
          │                        │                        │
          └────────────────────────┼────────────────────────┘
                                   ▼
                    ┌─────────────────────────────┐
                    │  Witness gate + ATOM trail   │
                    │  permit_witness_mutation     │
                    │  ExistenceCertificate E2E    │
                    └─────────────────────────────┘
```

**Canonical code roots (do not invent parallel trees):**
- Formal: `lean/K22/`, `lean/TriWeavon/`, `agda/src/TriWeavon/`
- GPU: `cutiles/cutile/`, `crates/triweavon-cudarc/`, `kernels/`
- MCP: LogOS `crates/coherence-mcp/` **and** standalone `F:\Users\Matthew Ruhnau\coherence-mcp\` (TS) — note dual homes
- Product: `adhealth-meaningseed/`, `coherence-mcp/coherence-site/`, `notebooks/AUKUS_Chessboard.ipynb`
- Instance: `C:\Users\toolated\artifacts\sm_100\` + HUP docs under `docs/sovereign-handoff/`

---

## Single workflow DAG (execute in order)

### Phase 0 — Bootstrap (every new session, 10 min)

1. Read this file + any HO listed in the phase you will work.
2. Read `HANDOFF-PROTOCOL.md` envelope rules.
3. Keystone theory (optional, 2 min): `CORPUS-CONSTRAINT-MATH-PHYSICS.md` → SpiralSafe `ONE_PAGER.md` only.
4. Confirm cwd = `F:\Users\Matthew Ruhnau\LogOS` (or explicit worktree); refuse silent dual roots.
5. Run `git status` + note branch (master is **26 commits ahead** of `LogOS/master` as of inventory).
6. If CTX will include large dumps / SpiralSafe research: **index paths only** (lesson from HO-05/HO-06).

### Phase 1 — Formal spine (HO-01 + HO-02 + active todos)

**Goal:** K22/MOG/Mehler/JFA are buildable and gated.

| Step | Action | Source | Done when |
|------|--------|--------|-----------|
| 1.1 | `lake build K22` (and `K22.Existence`) | HO-01, HO-02 | green or known sorry list |
| 1.2 | Finish `MiracleOctadGenerator.lean` + `GF4RowAction.lean` real hexacode | HO-01, `019f42c9` todo | no placeholder `isHexacodeword` |
| 1.3 | K22 tactic spine smoke: `k22_auto` / `k22_simp?` / SerreScar / K22Log JSON | HO-02 | `[K22-LOG]` lines parse |
| 1.4 | Wire `k22-logs.js` into MCP introspection endpoint | HO-02 | endpoint returns parsed logs |
| 1.5 | NS L3 shrinker + VanishingResilience hook | `019f42c9` | modules import; no free orphans |
| 1.6 | Symmetry gate: all witness mutations via `permit_witness_mutation` | HO-01, HANDOFF-2026-07-06 | mehler↔otto symmetric |

### Phase 2 — GPU / cutile execution layer (HO-05 + PHASE2 board)

**Goal:** Provenance-bearing TDA/M24 path with honest CPU fallback.

| Step | Action | Source | Done when |
|------|--------|--------|-----------|
| 2.1 | Reconcile `9P2000.L/strands/grok` cutile vs `cutiles/cutile` (diff-only port) | HO-05 | single source of truth |
| 2.2 | `TdaLaunchProvenance` on `KernelWitness` + roundtrip test | HO-05 | prediction_error≤0.1 + LiftOk |
| 2.3 | Three-phase TDA launch skeleton + `tda_sparse_reduction` example | HO-05 | example runs (CPU ok) |
| 2.4 | Real M24 octad table + hybrid `reduce_k22_m24_m12` | HO-05, PHASE2 | Betti Δ targets documented |
| 2.5 | cutile c12/l39/existence_cert exports complete | `019f42c9` | `cargo test -p cutile` relevant suites |
| 2.6 | Backend enum path (CUDA/wgpu/CPU) only if Phase 2.1–2.5 green | HO-06 | feature flags compile |

### Phase 3 — coherence-mcp integration (HO-01 + HO-06)

**Goal:** Executable Mehler/M24 SRAC loop with types, not docs alone.

| Step | Action | Source | Done when |
|------|--------|--------|-----------|
| 3.1 | `process_srac_step_with_m24_tda` in main SRAC loop | HO-01 | called from MCP handler |
| 3.2 | Types: workspace, hup_tree, tda, L39, Serre diagnostics | `019f42c9` | exported + tested |
| 3.3 | Fix standalone TS coherence-mcp build errors if still present | HANDOFF-2026-07-06 | `npm test` / tsc green |
| 3.4 | Mehler Levin harness production path (if GPU work resumes) | HO-01 sibling `019f285a` | FFI + build.rs |

### Phase 4 — Product / AdHealth / publish (HO-03 + HO-04)

**Goal:** One shippable corridor, not six half-docs.

| Step | Action | Source | Done when |
|------|--------|--------|-----------|
| 4.1 | Merge worktree portal deltas into LogOS/coherence-site | HO-03/04 | single tree |
| 4.2 | Hyperspace Lane Map diagram + doc | HO-04 todo | file linked from master print |
| 4.3 | MSB pilot case study + health endpoint | HO-04 | draft reviewable |
| 4.4 | Append chessboard move after every work session | HO-04 | notebook cell 5 habit |
| 4.5 | Grant/AUKUS/CRA7E specs only after 4.2–4.3 | HO-04 | P2 backlog |
| 4.6 | Publish checklist for coherence-mcp version bump | TASK-HANDOFF… | tagged release when WAVE≥0.85 |

### Phase 5 — Git / docs hygiene (from open non-max sessions)

| Step | Action | Source |
|------|--------|--------|
| 5.1 | Reconcile worktree master ↔ LogOS ↔ remote | `019f2ae4` |
| 5.2 | Update README / LogOS.md / 9P2000.L to current checkpoint | same |
| 5.3 | HUP functionality tree (Serre-Scarr + positive introspection) | same + HO-02 |
| 5.4 | Push only after detect_changes / user confirm | Agents.md |

### Phase 6 — Session hygiene (meta — prevents next MAX crisis)

| Rule | Implementation |
|------|----------------|
| At 70% CTX | Write partial CHECKPOINT to `docs/sovereign-handoff/session-handovers/` |
| At 85% CTX | Stop feature work; finish handover; `/new` |
| After 5 compactions | Mandatory HO file even if % < 85 |
| One theme per session | Formal **or** GPU **or** Product **or** PC-ops — never BF6 + Lean together |
| Prefer cold-start + this DAG | Over `/resume` of MAX IDs |

---

## Open TODO merge table (deduplicated)

| ID | Task | Phase | Origin |
|----|------|-------|--------|
| T1 | MOG + GF4RowAction real generators | 1 | HO-01, 019f42c9 |
| T2 | lake build K22 full + Existence | 1 | HO-01/02 |
| T3 | NS L3 shrinker + VanishingResilience | 1 | 019f42c9 |
| T4 | k22-logs → MCP endpoint | 1 | HO-02 |
| T5 | TdaLaunchProvenance + roundtrip | 2 | HO-05 |
| T6 | cutile c12/l39/existence_cert exports | 2 | 019f42c9 |
| T7 | M24/M12 hybrid + Leech guidance | 2 | PHASE2, HO-05 |
| T8 | SRAC main-loop M24 TDA wire | 3 | HO-01 |
| T9 | MCP types hup_tree/tda/L39/serre | 3 | 019f42c9 |
| T10 | Agda DAG + Existence.lean + mehler superskill | 1/3 | 019f42c9 |
| T11 | Hyperspace map + MSB pilot | 4 | HO-03/04 |
| T12 | Git reconcile + push | 5 | 019f2ae4 |
| T13 | Backend CUDA/wgpu/CPU (deferred) | 2.6 | HO-06 |

---

## Learning bank (cross-session — do not relearn)

1. **Max context is a failure mode of scope**, not of tools — strand separation is the fix.
2. **Compaction ≠ memory** — only handovers + repo files persist truth.
3. **Twin sessions** (HO-03/04) waste energy; always check inventory for duplicates.
4. **Strand cutile vs monorepo cutile** diverge; monorepo wins.
5. **Jesus-Fractal-Axiom** is a mutation gate + ε-loop, not decorative ethics text.
6. **Two M24 coefficient tables** are dual normalizations — document, don’t debate.
7. **sm_100** is an instance identity tree under `artifacts/sm_100`, not a slogan.
8. **Negative space product thesis** is the AdHealth north star after formal closure.
9. **E2E ExistenceCertificate** Rust→JSON→Python→Lean is the preferred demo spine.
10. **When in doubt:** WAVE ≥ 0.85, α+ω=15, cold-start from HO files.

---

## Suggested first sprint (one session, non-max)

**Theme: Formal only (Phase 1.1–1.3 + T1)**

```powershell
cd "F:\Users\Matthew Ruhnau\LogOS"
# 1. Read HO-01, HO-02, this file
# 2. Work only under lean/K22 and lean/TriWeavon
# 3. lake build K22.Existence ; lake build K22
# 4. Flesh MiracleOctadGenerator + GF4RowAction
# 5. Before exit: if ctx high, write HO-07-CHECKPOINT-YYYYMMDD.md
```

**Theme: GPU only (alternate session)** — Phase 2.1–2.2 only.  
**Theme: Product only (alternate)** — Phase 4.1–4.2 only.

---

## Ingest checklist (this meta-task)

- [x] Inventory all sessions latest-first
- [x] Flag MAX set (6)
- [x] Write HO-01 … HO-06 (1 each)
- [x] Unified workflow DAG + merged todos + learning bank
- [ ] Next human/agent: pick Phase theme and execute without resuming MAX IDs

---

## Sign-off

```
The Keystone Holds ✦ α + ω = 15 · WAVE ≥ 0.85 · ε = 0.00055
MAX sessions archived as handovers · Workflow is the new hot context
```

**Resume command for implementer (new session):**

```text
Ingest docs/sovereign-handoff/UNIFIED-WORKFLOW-FROM-MAX-CONTEXT-SESSIONS-2026-07-09.md
and the HO-0N files for your phase. Do not resume MAX session IDs.
Execute Phase {1|2|3|4|5} only.
```
