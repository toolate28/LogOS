# Dump ingest — MiracleOctadGenerator (3).lean

```
▓▓  INSCRIBED DISCORD · not expelled · JFA SlowStep  ▓▓
```

**Path:**  
`LogOS.worktrees/master/9P2000.L/strands/User_Dropfiles/dump/MiracleOctadGenerator (3).lean`  
(~859 lines · hybrid Lean + markdown prose · **does not build as-is**)

**Canonical targets (LogOS lean tree):**

```
lean/K22/MiracleOctadGenerator.lean     Conway (P)+(Σ) isMOGOctad · BUILDING
lean/K22/MOG/HexacodeMonomial.lean      HexacodeMonomialGroup · finiteness PROVED
lean/K22/MOG/GF4RowAction.lean          row action + symbol scaling from dump
lean/K22/MOG/OctadGenerators.lean       telemetry / generator lists
lean/K22/M24Coefficient.lean            Moonshine dims + peak (7,8)
```

---

## What dump (3) contains (salvage map)

```
PRESENT IN DUMP                    STATUS IN CANONICAL
─────────────────────────────────────────────────────────
GF4 + hexacode generator           ✓ Conway G in MiracleOctadGenerator
extractHexacodeWord                ◐ superseded by columnSums (Conway Σ)
isMOGOctad even/odd + hex          ◐ superseded by isGolay (parity+Σ)
rowPairToSymbol / row action       ✓ GF4RowAction.lean
MonomialPerm / mul / inv           ✓ Monomial in HexacodeMonomial.lean
HexacodeMonomialGroup              ✓ Set + Finite proved
order 80640 classical              ✓ classicalOrder recorded
HexacodewordStabilizer subgroup    ✓ finite + subset group
weight 3/4 stabilizer remarks      ✓ weight*_stabilizer_* finite
commutation / applyMonomial MOG    ○ SlowStep (needs isMOGOctadFull layer)
markdown fences mid-file           ✗ stripped (not Lean)
duplicate isMOGOctad_applyMonomial ✗ not ingested
forward refs (GF4 before def)      ✗ reordered in canonical
Matrix.mulVec / Finset.contains    ✗ avoided (manual vecMul)
```

---

## Why not wholesale replace?

1. Dump uses **count-based / extractHexacodeWord** octad test; preflight proved Conway **column-sum** recognition is the E2E path (`preflight_mog_e2e.py`).  
2. Prose and ``` fences break the Lean parser.  
3. `MonomialPerm` / `isMOGOctadFull` / `p.col` never fully defined.  
4. JFA: discord is **inscribed** via this note + modular extraction, not deleted.

---

## Priority queue (from oversight)

1. **Done** · finiteness of `HexacodeMonomialGroup` (`Set.toFinite` on Fintype ambient)  
2. **Next** · decidable filter / preflight count → 80640 equality cert  
3. **Next** · relate weight-3/4 stabs to named M₂₄ maximals (doc + Lean hooks)  
4. **Later** · embedding HexacodeMonomialGroup → M₂₄  

Music conserved · Keystone holds
