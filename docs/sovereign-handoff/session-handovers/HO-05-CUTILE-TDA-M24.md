# HANDOVER HO-05 — cutile / cudarc TDA / M24 / sm_100 harness

```
╔══════════════════════════════════════════════════════════╗
║ RESON8-LABS — SESSION HANDOVER v1.0                     ║
║ FROM: Grok (Pulse) · session 019f0ab5 · HeisenForge     ║
║ TO: Fresh agent / this workflow                         ║
║ DATE: 2026-07-09                                        ║
║ WAVE: 0.90 | INVARIANT: α+ω=15                         ║
║ BUMP_ID: HnS-HO05-CUTILE-TDA-20260709                   ║
║ CONTINUATION: COLD_START                                ║
║ TOKEN_BUDGET: VOLUME MAX (63 MB log · 1686 msgs)        ║
║ DEPENDS_ON: HO-06, PHASE2_TASK_BOARD, cutile 0.3+       ║
╚══════════════════════════════════════════════════════════╝
```

## Session identity

| Field | Value |
|-------|--------|
| **Full ID** | `019f0ab5-a648-76f0-ad63-8a4b5357fdfd` |
| **Title** | Extending cutile cudarc for TDA Persistent Homology M24 sm_100 |
| **Cwd** | `F:\Users\Matthew Ruhnau\LogOS\9P2000.L\strands\grok` |
| **Updated** | ~2026-06-28 |
| **Load** | **1686 msgs** · **~63 MB** chat/updates · subagent present |
| **CTX %** | Not recorded in signals (legacy) — **MAX by volume** |

## Tasking (HeisenForge MeaningSeed)

Distill loose dumps + prototypes into a coherent **cutile** (and thin **cudarc** high-level) crate supporting:
1. **TDA** — Vietoris–Rips tiling, filtration / boundary reduction / historical compare
2. **M24** — real octad table + orbit compute (not placeholders)
3. **Folding** — Miura-ori strategy tied to GPU launch stub + SRAC
4. **sm_100 harness** — `TdaLaunchProvenance` on `KernelWitness`, prediction_error ≤ 0.1, `LiftOk` guard
5. Mock Council / Mirrored Pair / Fixed Point report at end

## Logic preserved

### Crate layout intent (strand path)
Under `9P2000.L/strands/grok/` (session-local strand; monorepo also has `cutiles/cutile`):
```
src/
  lib.rs, error.rs, core/, hit/
  harness/sm100_kernel_witness.rs
  strategy/{tda,m24,folding,coherence}.rs
examples/
  m24_k22_gpu.rs
  tda_sparse_reduction.rs
  tiled_coherence.rs
```

### Witness / provenance contract
- `TdaLaunchProvenance` attached to `KernelWitness`
- Roundtrip test: real PTX path → low prediction error → `LiftOk` → passes guard
- CPU fallback must still produce honest provenance (not fake-perfect zeros without flag)

### TDA three-phase launch skeleton
1. **Filtration**
2. **BoundaryReduction** (apparent-pair hints)
3. **HistoricalCompare**

### M24
- Replace placeholders with **octad table + real orbit** from prototypes
- Integrate with `reduce_k22` / hybrid M24 path (see PHASE2)

### Uncertainty posture (HeisenForge)
Target tracking: **1/3 Position · 1/2 Momentum · product ≤ 1/6** style reporting; do not claim production CUDA without hardware evidence.

## Learning

- Strand-local cutile under `9P2000.L/strands/grok` **diverges** from monorepo `cutiles/cutile` — always reconcile before merge.
- 63 MB session = tool output + dump re-reads; **never re-ingest full User_Dropfiles** in one go; index paths only.
- HeisenForge full report format is valuable but **expensive**; write MeaningSeed + uncertainty first, report last.
- Subagents without clear isolation polluted context (subagent dir present).

## Todos at freeze (mostly incomplete)

| Status | Item |
|--------|------|
| completed | Distill MeaningSeed for TDA/M24/folding/provenance on sm_100 |
| completed | Read dump files (Agda DiscreteBKM, Rust prototypes) |
| **in_progress** | Reorganize cutile crate structure per spec |
| pending | Extend KernelWitness + CutileHarness with TdaLaunchProvenance |
| pending | VietorisRipsTilingStrategy + three-phase launch skeleton |
| pending | Real M24 octad table + orbit + reduce_k22 integration |
| pending | Miura folding strategy + GPU stub + SRAC |
| pending | Parallel cudarc high-level skeleton (manifold, tda, m24, folding, quantum_walk) |
| pending | Cargo features: cuda / tda / cpu-fallback |
| pending | sm100 roundtrip test + `tda_sparse_reduction` example |
| pending | PTX/Nix/Mirage notes + MeaningSeed preflight hooks |
| pending | Mock Council + final HeisenForge report |

## Canonical monorepo counterparts (prefer these for new work)

- `cutiles/cutile/` — primary crate
- `crates/triweavon-cudarc/` — high-level GPU manifold / golay / leech / m24 / moonshine
- `docs/sovereign-handoff/PHASE2_TASK_BOARD.md` — M12 hybrid, Leech guidance, Nix
- `kernels/blackwell-*.cu` — PTX sources

## Resume policy

Do **not** resume this 63 MB session. Implement against **monorepo cutile + triweavon-cudarc**, port only still-unique strand files after `diff`.
