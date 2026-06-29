---
name: leech-density-guidance-reduction
description: >
  Apply Leech lattice density guidance to hybrid M24/M12 K22 reductions with Golay-derived
  norm-4 vectors, Agda-extracted verification guards, and SRAC-safe tie-breaking. Use when
  the user mentions Leech density, kissing number 196560, Construction A, Golay code,
  hybrid reduction, reduce_k22_hybrid_m24_m12, verify_leech_guidance, leech_density_score,
  apply_leech_density_guidance, Phase 2 Task 2.2, or runs /leech-density-guidance-reduction.
  Triggers for notebook evcxr cells, triweavon-cudarc wiring, and Agda preservation proofs.
version: 1.0.0
---

# Leech Density Guidance Reduction

One-click workflow for Leech-density-aware hybrid M24/M12 K22 reduction. Grounds executable
reduction in the formal invariant: Leech lattice Λ₂₄ kissing number = **196,560** (optimal,
unique up to isometry), constructed via binary Golay code and Construction A.

## Hard Invariants (never override alone)

| Invariant | Value | Role |
|-----------|-------|------|
| Kissing number | 196,560 | Geometric anchor for density prior |
| α + ω | 15 | Conservation law |
| tomczak_lift | preserved | Structural foldability |
| Betti proxy delta | ≤ 0.05 | Mirrored verification tolerance |
| WAVE | ≥ 0.85 (target 1.0) | Coherence gate |

Treat kissing number as **one hard invariant among several** — not the sole selector. This
preserves topological bendability for lower-dimensional projections.

## When to Invoke

- Wiring `leech.rs` / `golay.rs` into `triweavon-cudarc`
- Running `diagnostic_tick_with_leech_verifier` after sm_100 roundtrip
- Filling Agda holes in `TriWeavon.K22.LeechDensityGuidance`
- Phase 2 Task 2.1 + 2.2 (hybrid reduction + Leech layer)
- A/B testing `use_golay_vectors` vs toy norm-4 table

## Execution Workflow

### 1. Configure Leech prior

```rust
let leech_cfg = LeechDensityConfig::default().with_weight(0.35);
// weight 0.0–1.0; norm_threshold 4.0 for minimal vectors
```

### 2. Run hybrid reduction

```rust
let guided = reduce_k22_hybrid_m24_m12(&fragment, level)?;
```

Three-tier tie-breaker (documented in `m24.rs`):
1. **Primary**: higher `leech_density_score`
2. **Secondary**: lower `betti_proxy` on density tie
3. **Tertiary**: deterministic average

### 3. Verify with Agda-extracted guard

```rust
let verified = verify_leech_guidance(&leech_cfg, &baseline, &guided)?;
```

Guard checks:
- `rigid_lift_check()` (tomczak preserved)
- `betti_proxy_delta` ≤ 0.05
- burst reduction ≥ `weight * 0.4`
- density improvement per `leech-density-reduces-burst-rate` postulate

### 4. Log telemetry + coherence-mcp

```rust
wire_leech_telemetry(witness, Some(density), Some(weight));
```

Emit to `coherence-mcp`: `wave_coherence_check`, `trigger_correction_burst` if guard fails.

### 5. Formal cross-check

Agda module: `TriWeavon.K22.LeechDensityGuidance` (v0.4.1+)
- `leech-guidance-preserves-tomczak-lift`
- `leech-guidance-bounded-betti-delta`
- `leech-density-reduces-burst-rate`

Filled proof variant uses `pathToEquiv` + `ua` transport (see `LeechDensityGuidance (1).agda`).

## Artifact Paths

| File | Location |
|------|----------|
| `leech.rs` | `Downloads/` → `triweavon-cudarc/src/leech.rs` |
| `golay.rs` | `Downloads/` → `triweavon-cudarc/src/golay.rs` |
| `m24.rs` | hybrid + mirrored verification |
| `LeechGuidanceExtracted.rs` | Agda → Rust guard prototype |
| `diagnostic_tick_with_leech_verifier.rs` | CutileHarness integration |
| `LeechDensityGuidance.agda` | formal preservation theorems |

## Success Criteria

- `tomczak_preserved = true` after guidance
- Betti divergence < 3% on hybrid path
- `verify_leech_guidance` returns `Ok(true)` on representative fragments
- `coherence-mcp invoke invariant_check` passes

## Skill Chain

```
leech-density-guidance-reduction
  → crystalline-tessellation-projection (lower-D slice viz)
  → srac-lattice-restoration-visual (surge → restored density)
```

Read `references/invariant-manifest.json` for full kissing + Ricci + Golay linkage.