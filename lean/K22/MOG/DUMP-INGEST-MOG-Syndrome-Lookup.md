# Dump ingest — MOG Syndrome Lookup (skeletal + concrete)

```
▓▓  P0 GEOMETRY JOINT · real H from golayBasisN · mask sphere certified  ▓▓
```

**ATOM:** `MOG-SYNDROME-LOOKUP-FORMALIZATION-20260705` + `MOG-CONCRETE-GEOMETRY-PROOFS-20260705`  
**Patch:** 2026-07-10 — toy syndrome replaced; sphere decoder wired to HexacodeGolay  

**Sources:**  
- `9P2000.L/.../dump/MOG_Syndrome_Lookup.lean` (skeletal)  
- User paste 2026-07-10 (concrete geometry layer)  
- P0 deploy: bind H + exhaustive wt≤3 decode  

**Canonical targets:**

| File | Role |
|------|------|
| `lean/K22/MOG/SyndromeLookup.lean` | Skeletal types, axioms, stub lookup |
| `lean/K22/MOG/SyndromeLookupConcrete.lean` | 4×6 grid, **real** syndrome, sphere lookup |
| `lean/K22/HexacodeGolay.lean` | **Verified** membership / 759 octads / enumerator / **`golaySyndromeN`** |
| `lean/K22/MiracleOctadGenerator.lean` | Conway `isMOGOctad` on Finsets |

---

## Salvage map (post-P0)

| Dump content | Canonical status |
|--------------|------------------|
| `Position`, `Octad`, `ErrorPattern` | ✓ both modules |
| `MOGArray` structure | ✓ skeletal + concrete instance |
| Abstract `axiom syndrome` / `standardMOG` | ✓ skeletal (abstract layer only) |
| Concrete `standardMOGGrid` row-major | ✓ `SyndromeLookupConcrete` + bijective |
| **Real** syndrome from `golayBasisN` | ✓ `concreteSyndromeOfSupport` / `golaySyndromeN` |
| Toy row/col syndrome | ✗ **removed** (was not Golay H) |
| Sphere decoder wt ≤ 3 | ✓ `mogSyndromeLookupConcrete` → `golayDecodeSyndromeN` |
| Mask uniqueness / encode→decode | ✓ `mogLookupUnique_masks` / `mogLookupCorrect_masks` |
| Finset glue `mogLookupCorrect` / `Unique` | ◐ SlowStep (mask layer certified; subtype iso open) |
| Lipschitz / projection | ◐ SlowStep |
| Octad pattern ↔ Golay octad | ✓ `isOctadPatternConcrete` via `golayMaskOkN` |
| BitVec 12/24 APIs from dump | ✗ rewritten to `Fin 4096` / `Finset` for Mathlib portability |

---

## What is certified vs scaffold

| Layer | Claim |
|-------|--------|
| Combinatorics | `HexacodeGolay`: Field, hexacode, 4096 words, 759 octads, d=8 |
| Syndrome map | `golaySyndromeN` = GF(2) dots vs `golayBasisN` (self-dual H ≅ G) |
| Sphere uniqueness | `golay_syndrome_injective_correctable` (`native_decide` Finset card) |
| Sphere decode | `golay_decode_correct_on_correctable` on all 2325 masks |
| `ErrorPattern` subtype | Glue: `maskOfSupport` / `supportOfMask` iso — SlowStep |
| Lipschitz | Open SlowStep |

**Deployment-ready for mask-layer witness telemetry.**  
Finset subtype equality and 1-bit syndrome Lipschitz remain SlowStep, not blockers for Hero-2 mask telemetry.

---

## RESON8:QWALK bind

| Hero | Use |
|------|-----|
| 2 Witness | `mogSyndromeLookupConcrete` + `isHexacodeword` / `golayMaskOkN` over density→error |
| 4 Wave 2.5 | Cascade uses octad / syndrome window as satellite input |
| 5 TDA | Acceptance window = regular set in syndrome space (projection goals SlowStep) |

Telemetry symbol list: `witnessTelemetrySymbols` in `SyndromeLookupConcrete`.

---

## Build

```powershell
cd lean
lake build K22.HexacodeGolay
lake build K22.MOG.SyndromeLookup
lake build K22.MOG.SyndromeLookupConcrete
```

Music conserved · Keystone holds · α + ω = 15
