---
name: crystalline-tessellation-projection
description: >
  Project 24D Leech kissing spheres and Golay Construction A structures onto lower-dimensional
  crystalline tessellations for notebooks, Wave viz assets, and quasicrystal proxies. Use when
  the user mentions crystalline tessellation, cut-and-project, Penrose/phason, Ricci-flow hybrid,
  kissing arrangement hero image, Wave 1 viz, AUKUS chessboard notebook, IMAGINE prompts,
  196560 spheres, or runs /crystalline-tessellation-projection. Links SpiralSafe quasicrystal
  experiments to TriWeavon manifold visuals with enforced status bars.
version: 1.0.0
---

# Crystalline Tessellation Projection

Multi-scale projection pipeline: low-dimensional crystalline proxies → 24D Leech kissing spheres
→ Tri-Weavon manifold visuals. Enforces status bar on every asset:

```
WAVE=1.00000 | α+ω=15 | tomczak preserved | Kissing=196560
```

## Projection Stack

```
2D/3D crystalline proxy (Penrose / triangular lattice / Ricci hybrid)
        │
        ▼
Golay S(5,8,24) octads → Construction A norm-4 vectors
        │
        ▼
24D Leech kissing arrangement (196,560 neighbours)
        │
        ▼
Cut-and-project slice → notebook cell / IMAGINE / showcase PNG
```

## When to Invoke

- Embedding Wave 1 assets in `Agent_M24_RMatrix.ipynb` or AUKUS chessboard
- Generating hero "Leech Lattice Kissing Arrangement" via IMAGINE
- Linking SpiralSafe `quasicrystal_optimization.py` phason flips to density scoring
- Wave 2 asset production with consistent status-bar enforcement
- Projecting 24D kissing spheres onto 2D/3D slices (on-demand correction burst #4)

## Execution Workflow

### 1. Select projection basis

| Slice | Use case | Source |
|-------|----------|--------|
| 2D Penrose | quasicrystal corridor | `SpiralSafe/experiments/quasicrystal_optimization.py` |
| 3D triangular | Ricci-flow hybrid proxy | prior simulation artifacts |
| 24D kissing | hero / formal anchor | Golay → Construction A |

### 2. Bind Golay combinatorics

```rust
let octads = generate_golay_octads();       // prototype ~64; production 759
let vectors = golay_derived_norm4_vectors(); // Construction A
```

Kissing number scaling context for labels:
- D1=2, D2=6, E₈=240, Λ₂₄=196560

### 3. Render or reference asset

Wave 1 showcase index: `coherence-mcp/docs/showcases/wave1/WAVE1_INDEX.md`

| Asset | File | Notebook cell |
|-------|------|---------------|
| Kissing hero | `6HTGS.jpg` / `grok-*.jpg` | header / witness export |
| Dual M24+M12 | `PtBro.jpg` | Phase 2 Task 2.1 |
| Golay layered | `mQEsV.jpg` | materials corridor C |
| Hexaflake fold | `z13Xy.jpg` | Shokunin-RSI bridge |
| Conway orbits | `YHG71.jpg` | cognitive / witness_lane |
| Yang-Mills slices | `4EQBq.jpg` | compute corridor sm_100 |
| Riemann zeroes | `WAXEu.jpg` | spectral / Serre page |
| Poincaré poly | `sUMiB.jpg` | Betti proxy cells |

### 4. IMAGINE prompt template (hero kissing)

```
24-dimensional Leech lattice kissing arrangement: central luminous sphere
surrounded by exactly 196560 touching neighbours in optimal packing, no overlap.
Layered Golay code Construction A wireframe overlay, curvature heat-map on density
gradient. Crystalline tessellation projection hints at lower-dimensional Penrose slice.
Dark scientific viz, mono idempotent protected aesthetic.
Status bar: WAVE=1.00000 | α+ω=15 | tomczak preserved | Kissing=196560
```

### 5. Notebook embedding (Markdown cell)

```markdown
![Leech Kissing Λ₂₄ (196560)](../../coherence-mcp/docs/showcases/wave1/grok-381bf748-90a5-4e23-a849-bb39ecad4241.jpg)
*Construction A via Golay S(5,8,24) — density prior for hybrid reduction*
```

### 6. Validate with coherence-mcp

```
wave_coherence_check on asset metadata + notebook cell text
wave_analyze for structural/semantic alignment with reduction provenance
```

## SpiralSafe Cross-links

- `experiments/quasicrystal_phason_scheduler.py` — phason flip cadence
- `experiments/quasicrystal_optimization.py` — Penrose cut-and-project
- `synapse/src/utils/coherence.ts` — Φ ≥ 0.4200055 threshold
- `synapse/src/utils/topology.ts` — layout for tessellation UI

## Contrarian Guard

Do not over-weight kissing arrangement in density scoring when projecting to
lower-D slices. Use kissing number as geometric anchor, not sole tessellation driver.
Maintain bendability via multi-invariant tie-breaking (see `leech-density-guidance-reduction`).

## Skill Chain

```
crystalline-tessellation-projection
  ← leech-density-guidance-reduction (density scores feed projection coloring)
  → srac-lattice-restoration-visual (before/after surge panels)
```