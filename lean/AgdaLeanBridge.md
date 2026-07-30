# Lean ↔ Agda Equivalence Notes

**Cascade layer:** L10 — formal bridge  
**Invariant:** `α + ω = 15` (Category C label / software constant)  
**WAVE publish gate:** 85 on 0–100 (= 0.85)  
**Date:** 2026-07-09 · **Standardised:** 2026-07-30  
**Core map:** `docs/formal/CORE-SET.md` · `lean/CORE.md` · `agda/docs/CORE.md`

## Shared statement

Both systems encode:

```
α + ω = CONSERVATION_SUM
CONSERVATION_SUM = 15
```

## Translation mapping

| Concept | Lean | Agda |
|---------|------|------|
| Module | `TriWeavon.Conservation` | `TriWeavon.ConservationRMatrix` |
| Path | `lean/TriWeavon/ConservationInvariant.lean` | `agda/src/TriWeavon/ConservationRMatrix.agda` |
| Constant | `CONSERVATION_SUM : Nat` | `CONSERVATION-SUM : ℕ` |
| Record | `structure WavePair` | `record WavePair` |
| Property | `is_conserved` | `is-conserved` |
| Peak | `peak_resonance` (7, 8) | `peak-resonance` (7, 8) |
| Peak proof | `peak_resonance_conserved` (`rfl`) | `peak-conserved` (`refl`) |
| R-matrix | (executable: Rust/CUDA) | `fundamental-r-matrix` (symbolic tag) |
| K22 / Serre | `K22.SerreScarTactic` / MOG | `TriWeavon.K22.SerreScarr` · `SerrePage` |
| Steiner (Golay pin) | `K22.MOG.SteinerDoubleCount` S1–S6 | (A-lit twin; not required for S5) |
| Aggregate | `lake build K22…` | `src/Everything.agda` |

## Consistency claim (informal)

For any `WavePair` with a proof of `is_conserved` in Lean, the corresponding Agda
term of type `is-conserved` is inhabited (and conversely for peak / dual cases).

## Recommended next mechanization

1. Export Lean JSON / OpenTheory-style witness for peak pair.
2. Import as Agda postulates only after checksum match in `verification_orchestrator.ipynb`.
3. Keep numeric R-matrix **out** of Agda until complex formalization is ready; keep structural identity in Rust/CUDA/WGSL.

## Related

- `docs/sovereign-handoff/LAYER-CASCADE-MAP.md`
- `notebooks/verification_orchestrator.ipynb`
- `kernels/fundamental_r_matrix.cu`
