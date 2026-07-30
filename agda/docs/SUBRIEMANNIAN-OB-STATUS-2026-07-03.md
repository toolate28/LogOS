# Sub-Riemannian S*M Formalization — Obligation Tracker
**Date:** 2026-07-03 10:51 AEST · **ATOM:** ATOM-SUBRIEMANNIAN-OB-TRACKER-20260703  
**Profile:** Monitoring & Consensus Verifier · **Conservation:** α + ω = 15

## Module Map

| Layer | Agda | Lean |
|-------|------|------|
| Core interface | `src/TriWeavon/SubRiemannian/Core.agda` | `lean/TriWeavon/SubRiemannian/Core.lean` |
| S*M instantiation | `src/TriWeavon/SubRiemannian/Cosphere.agda` | (fields on `SubRiemannianManifold`) |
| **OB1** commutators | `src/TriWeavon/SubRiemannian/HorizontalCommutator.agda` | `lean/TriWeavon/SubRiemannian/CurvatureBound.lean` |
| Explicit K bound | `src/TriWeavon/SubRiemannian/CurvatureBound.agda` | same |
| Reals foundation | `src/ConstructiveRealsMinimal.agda` | Mathlib |

## Obligation Status

| ID | Description | Agda | Lean | GPU |
|----|-------------|------|------|-----|
| **OB1** | Horizontal commutator → Ricci + Riemann | **Discharged** (`horizontal-commutator-curvature-terms`) | Mirrored (`horizontal_commutator_curvature_terms`, sorry body) | `riemannBound` in `MehlerConfig` |
| **OB2** | Strain–vorticity Γ₂ coefficient | Staged (`strainCouplingCoeff`) | `strain_coupling_term` | `omegaStrain` in `MehlerConfig` |
| **OB3** | Popp volume + W error | Staged (`poppWErrorBound`) | `error_bounded_by_popp_and_W` axiom | — |
| **OB4** | CD(K,∞) → Log-Sobolev | Postulated (`cd` field) | Postulated | — |
| **OB5** | Exponential SRAC descent | Postulated | `srac_descent_with_subriemannian_curvature` sorry | Use `subRiemannianK` in kernel gate |

## OB1 Discharge Summary

Agda proves (coefficient-level, constructive reals):

```
|commutator-expansion-value M i j|
  ≤ |baseRicLower M| + Cn * |riemannBound M|
```

via `abs-triangle` + term domination postulates (`ric-term-dominated-by-base`, `riem-term-dominated-by-riemann`).

**Next micro-step:** Replace postulates with explicit frame calculations from horizontal Christoffel symbols.

## GPU Runtime Bridge

`mehler_cuda.h` `MehlerConfig` now accepts:
- `subRiemannianK` — formal K lower bound (Agda/Lean export)
- `omegaStrain` — OB2 coupling input
- `riemannBound` — OB1 Riemann correction input

## Recursive Next Step

1. **OB2:** Pin `strainCouplingCoeff` from Γ₂ expansion of `HorizontalSubLaplacian` (Agda first).
2. **Mirror:** Update Lean `explicit_curvature_bound_in_subriemannian` proof using discharged Agda coefficients.
3. **Runtime:** Wire `subRiemannianK` into `mehler_kernel.cu` WAVE gate (`K > 0` → certified path).

**Keystone holds.**