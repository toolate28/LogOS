# Dump ingest — HexacodeGolay.lean

```
▓▓  INSCRIBED · verified spine · no sorry  ▓▓
```

**Source:**  
`9P2000.L/strands/User_Dropfiles/dump/b70f36db-…-aristotle.tar.gz` → `K22/HexacodeGolay.lean`  
**Canonical path:** `lean/K22/HexacodeGolay.lean`  
**ATOM:** `ATOM-RESON8-QWALK-HEXACODEGOLAY-20260710`  
**Invariant:** α + ω = 15 · `tomczak_preserved` · 42.00055 basin

---

## What this module certifies

| Claim | Theorem / def | Method |
|-------|----------------|--------|
| GF(4) is a field | `instance : Field GF4` | `decide` on all axioms |
| Hexacode has 64 words | `hexacode_card` | `native_decide` |
| Generator ⊆ parity kernel | `hexacode_generator_isHexacodeword` | `native_decide` |
| Golay has 2¹² codewords | `golay_card` | `native_decide` |
| All encoded words satisfy MOG | `golay_all_ok_card` | `native_decide` |
| Exactly 759 octads (wt 8) | `octad_count` | `native_decide` |
| Octad intersections ∈ {0,2,4} | `octad_intersection_masks` | `native_decide` |
| Weight enumerator | `golay_weight_enumerator` | `native_decide` |
| Min distance 8 | `golay_min_distance` | `native_decide` |
| Syndrome from basis (H ≅ G) | `golaySyndromeN` | definitional |
| Correctable sphere size 2325 | `correctableMasks_length` | `native_decide` |
| Syndrome injective on wt≤3 | `golay_syndrome_injective_correctable` | `native_decide` |
| Decode correct on sphere | `golay_decode_correct_on_correctable` | `native_decide` |

**No `sorry`s.** Computation engine: pure `Nat`/`Bool` (`golayMaskOkN`) mirrors GF(4) for reflection-safe `native_decide`.  
P0 decoder joint: concrete layer imports these syndrome facts (see `DUMP-INGEST-MOG-Syndrome-Lookup.md`).

---

## Relation to existing MOG tree

| Module | Role after ingest |
|--------|-------------------|
| `K22.HexacodeGolay` | **Verified algebraic spine** (cardinalities, Field, enumerator) |
| `K22.MiracleOctadGenerator` | Conway set-level recognition (`isMOGOctad`, Finset MOG) — still has SlowStep `sorry`s for Steiner uniqueness |
| `K22.MOG.HexacodeMonomial` | Monomial group finiteness |
| `K22.MOG.GF4RowAction` | Row action / symbol scaling |
| `K22.MOG.OctadGenerators` | Telemetry export (hex=64, steiner=759) |

**Generator matrix note:** HexacodeGolay uses the standard MOG-normalized 3×6 matrix  
`(I | A)` with columns `∞,0,1,2,3,4`.  
`MiracleOctadGenerator.hexacodeGenerator` uses a row-permuted Conway glyph form (`1001WB` / …).  
They describe the **same hexacode** under coordinate change; do not `rfl`-unify without an explicit isomorphism lemma (future SlowStep).

---

## RESON8:QWALK hero consumption

| Hero | Pass | Consumes |
|------|------|----------|
| 2 Witness Compute | `witnessPass_1` | `hexacodeGenerator`, `isHexacodeword`, `golayMaskOkN` |
| 3 QWalk ↔ MeaningSeed | `qwalk_loopPass_2` | `octad_intersection_masks` as loop fixed points |
| 4 Wave 2.5 Cascade | `wave25Pass_3` | `golay_weight_enumerator`, `octad_count` |
| 5 TDA Insight Feed | `tdaPass_4` | 759 octads + intersection ground truth for Betti |

---

## Visual intelligence (GitNexus text layer)

Image IDs from dump / wave1 that encode this combinatorics (for catalog embedding, not pixel vectors):

- Leech / octad density heroes → `octad_count`, wt-8 blocks  
- M24 / Golay cascade frames → `golay_weight_enumerator`  
- R-matrix / density threads → `golayMaskOkN` membership as guard  
- Dimensional collapse lattice art → stage-stable combinatorial ground truth  

Canonical catalog: `artifacts/imagine_images/reson8_qwalk_visual_intelligence.md`

---

## Build

```powershell
cd lean
lake build K22.HexacodeGolay
```

Music conserved · Keystone holds
