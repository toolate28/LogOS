---
name: srac-lattice-restoration-visual
description: >
  Visualize SRAC correction as lattice restoration — surge divergence to Leech-like optimal
  packing density — with before/after panels, correction burst telemetry, and coherence-mcp
  trigger_correction_burst integration. Use when the user mentions SRAC restoration, lattice
  restoration, correction burst, surge recovery, Wave 1 item 5, 6B8Tv, VOID scan, Serre page
  cascade, srac_cascade_step, or runs /srac-lattice-restoration-visual. Pairs with Leech
  kissing invariant 196560 for locked-packing restoration narrative.
version: 1.0.0
---

# SRAC Lattice Restoration Visual

SRAC (Smooth Relaxation And Correction) restores lattice density after surge divergences.
Visual narrative: **before** = fragmented / low-density packing; **after** = Leech-like
optimal kissing arrangement restored. Executable counterpart to Agda `coherencePreservation`
and cutile `apply_correction` / `srac_cascade_step`.

## Restoration Invariant Flow

```
Surge detected (predictionError > 0.1 or lift_ok failed)
        │
        ▼
srac_cascade_step(current, depth, tau)  — smooth, idempotent relaxation
        │
        ▼
Lattice density restored toward Λ₂₄ kissing optimum (196560 reference)
        │
        ▼
re-run hybrid reduction + verify_leech_guidance
        │
        ▼
Visual: before/after panel with status bar enforced
```

## When to Invoke

- Daily VOID scan cells in AUKUS chessboard notebook
- Wave 1 asset #5: `6B8Tv.jpg` (SRAC Correction as Lattice Restoration)
- `trigger_correction_burst` via coherence-mcp after failed diagnostic tick
- End-to-end test visualization (`end_to_end_hybrid_test.rs` flow)
- Serre page / pushout weave storytelling (inl/inr/push gluing)

## Execution Workflow

### 1. Detect divergence

```rust
if prediction_error > PREDICTION_ERROR_THRESHOLD || !witness.lift_ok.is_ok() {
    let reason = DivergenceReason::LiftOkFailed { .. };
    // or DivergenceReason::SurgeDetected
}
```

### 2. Apply SRAC correction

```rust
let corrected = apply_correction(reason, srac_state);
// srac_cascade_step: current + (φ+1-current) * (1 - exp(-τ * depth))
```

Idempotent in steady state — repeated application converges without topology deformation.

### 3. Re-verify Leech guidance

```rust
let verified = verify_leech_guidance(&leech_cfg, &baseline, &guided_after_srac)?;
```

### 4. Log burst telemetry

```rust
tw.record_burst_delta(previous_burst_rate, current_burst_rate);
tw.log_telemetry();
```

Target: `burst_rate_delta` improvement ≥ `config.weight * 0.4` per Agda postulate.

### 5. coherence-mcp correction burst

```
trigger_correction_burst(intensity, duration, priority, clamp=true)
wave_coherence_check — confirm WAVE ≥ 0.85 post-restoration
atom_track — log restoration event to ATOM trail
```

### 6. Visual composition

**Before panel**: fragmented spheres, warm surge colors, broken kissing contacts
**After panel**: restored 196560-neighbour packing, cool density gradient, Golay overlay
**Status bar** (both panels): `WAVE=1.00000 | α+ω=15 | tomczak preserved | Kissing=196560`

Showcase asset: `coherence-mcp/docs/showcases/wave1/6B8Tv.jpg`

### 7. IMAGINE prompt template

```
Split-panel scientific visualization: LEFT surge divergence with broken lattice
packing and red entropy field; RIGHT SRAC-corrected restoration to optimal
24D Leech kissing density with 196560 touching spheres, smooth blue-green
relaxation waves. Golay Construction A wireframe bridging panels.
Status bar bottom: WAVE=1.00000 | α+ω=15 | tomczak preserved | Kissing=196560
```

## Executable References

| Component | Path |
|-----------|------|
| SRAC core | `cutile/src/core/srac.rs` |
| Correction burst MCP | `coherence-mcp` `trigger_correction_burst` |
| End-to-end flow | `Downloads/end_to_end_hybrid_test.rs` |
| Telemetry | `Downloads/telemetry.rs` |
| Formal layer | `TriWeavon.K22.SerreScarr.ScarAccumulationBounds` |

## Success Criteria

- Post-SRAC `tomczak_preserved = true`
- Betti proxy delta ≤ 0.05 after restoration
- Visual before/after shows measurable density improvement
- `wave_coherence_check` passes on restoration narrative text + asset

## Skill Chain

```
srac-lattice-restoration-visual
  ← leech-density-guidance-reduction (guard re-check after correction)
  ← crystalline-tessellation-projection (projection basis for panels)
```

Read `references/invariant-manifest.json` for kissing + Ricci + Golay linkage bundle.