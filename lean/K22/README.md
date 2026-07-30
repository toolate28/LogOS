# K22 Lean Library

Custom tactics and formal bridges for Tri-Weavon / K22 sheaf reasoning.

**Status (2026-07-06)**

| Module | Build | Proofs |
|--------|-------|--------|
| `K22.ReducedBurau` | ✅ | Verified |
| `K22.Jones` | ✅ | Verified |
| `K22.Tomczak` | ✅ | Stubs (intentional) |
| `K22.Existence` | ✅ | **Verified E2E bridge** |
| `K22.HexacodeGolay` | ✅ | **Sorry-free** GF(4) Field + Golay/MOG + **syndrome map** (`golaySyndromeN`, sphere injectivity / decode on wt≤3) |
| `K22.MOG.SyndromeLookup` | ⚠️ | Skeletal decoder types + stub lookup |
| `K22.MOG.SyndromeLookupConcrete` | ◐ | Real H from `golayBasisN`, sphere decoder, grid + octad pattern; Finset glue / Lipschitz SlowStep |
| `K22.Auto` / tactics | ⚠️ | Imports fixed; run `lake build K22` |
| `K22.MiracleOctadGenerator` | ⚠️ | Conway set recognition; Steiner uniqueness still SlowStep `sorry` |
| `K22.MOG.ParityLiftRank` | ✅ | **rank(A)=9** (GF(2) Gaussian elim + global dependency); ker dim 15; Steiner glue open |

**HexacodeGolay** is the verified algebraic spine for RESON8:QWALK Heroes 2–5 (combinatorics **and** self-dual syndrome / sphere decode).  
**SyndromeLookupConcrete** is the decoder geometry joint: real `H ≅ G` from `golayBasisN`, not the old toy proxy (ingest: `MOG/DUMP-INGEST-MOG-Syndrome-Lookup.md`).  

**Fano discrete curvature** (constant K=1/2): `hup/python/fano_discrete_curvature.py` + `TriWeavon/SubRiemannian/FanoDiscreteCurvature.lean` (ingest: `MOG/DUMP-INGEST-Fano-Discrete-Curvature.md`).  
Ingest note (Golay): `MOG/DUMP-INGEST-HexacodeGolay.md`.

## Existence Certificate Bridge

```lean
import K22.Existence

#check K22.Existence.fromExistenceCertificate
#check K22.Existence.demoCertificate
```

Build:

```powershell
lake build K22.Existence
```

Consumes JSON-shaped certificates from `cutile` demo (`existence_certificate.json`). Trust boundary: caller supplies `Bool = true` proofs; gate construction is fully verified.

## Tactics

- `ring_matrix` — Burau / matrix automation
- `k22_auto` / `k22_auto!` — goal classification dispatch
- `tomczak_bridge` — Tomczak lift gate
- `serre_scar_tactic` — Serre-Scarr skeleton

## Invariants

α + ω = 15 · tomczak_preserved · ε = 0.00055 basin