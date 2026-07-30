# ATOM — Reconcile sm_100 progress notes vs live pin

```
╔══════════════════════════════════════════════════════════╗
║ ATOM-STEINER-RECONCILE-SM100-20260730                    ║
║ Purpose: Prevent parallel-instance regression            ║
║ Live pin wins over artifacts/ and stale SlowStep lists   ║
║ α + ω = 15 (Category C only) · Music conserved           ║
╚══════════════════════════════════════════════════════════╝
```

**Stamp:** 2026-07-30  
**Trigger:** sm_100 notes claiming S1/S4-only advance + residual injectivity/bit-subset SlowSteps  
**Authority:** `lean/K22/MOG/*` on HEAD, not `artifacts/lean/**` (path **does not exist** in this tree)

---

## 1. What sm_100 got **right** (HexacodeGolay)

| Claim | Pin status |
|-------|------------|
| `octad_count` = 759 | **A-native** GREEN (`native_decide`) |
| `octad_intersection_masks` — ∩ ∈ {0,2,4} for distinct wt-8 | **A-native** GREEN |
| `golay_weight_enumerator`, `golay_min_distance` | **A-native** GREEN |
| Design eq \(759\cdot 56=42504=\binom{24}{5}\) | **A-repo** in SDC (`steiner_count_identity`) |
| Pair census histogram (0/2/4 only) | **B computational companion** (Python/external); consistent with formal theorem, not a second Lean certificate |
| Packing: ∩ ≤ 4 ⇒ at most one octad per 5-set | load-bearing; spectrum {0,2,4} is stronger than needed |

`maskWeightN_le_of_submask` is already in `HexacodeGolay.lean` and is used by `steiner_uniqueness_mask_level`.

---

## 2. What sm_100 got **stale / wrong** (do not re-open)

| sm_100 claim | Live pin (this workspace) |
|--------------|---------------------------|
| Primary surface = `artifacts/lean/K22/MOG/SteinerDoubleCount.lean` | **False** — use `lean/K22/MOG/SteinerDoubleCount.lean` |
| S1/S4 only advanced; injectivity / round-trip / double-count open | **False** — **S1–S6 lake-green, 0 sorry** on SDC |
| Next = bit-subset mono + injectivity | **Already discharged** (`maskWeightN_le_of_submask`, `maskToOctad_injective_on_octads`) |
| `steiner_uniqueness_mask_level` almost discharged | **Already discharged** (no sorry on that theorem) |
| Residual list includes `isMOGOctad_transport`, `mapOctad_card`, … as SlowSteps | **Those are already proved** in MW (when module builds); residual is **CB-1** only |
| “S5 instantiation becomes mechanical after bit-subset” | **S5 already closed** (`golay_octads_form_steiner`) |

### Live SDC status (authoritative)

```text
S1 mask weight ↔ card          GREEN
S2 injectivity on octad masks  GREEN
S3 double_count_5sets          GREEN
S4 packing_eq_implies_steiner  GREEN
S5 golayOctadBlocks + steiner  GREEN
S6 round-trips + equiv         GREEN
```

**Receipt:** `lake build K22.MOG.SteinerDoubleCount` → Build completed successfully.

### Live residual (only CB-1)

```text
mogOctadsFormSteinerSystem_via_transport   OPEN (CB-1)
mem_conway_packed_iff                      optional sorry (non-blocking)
MiracleOctadGenerator.mogOctadsFormSteinerSystem
                                           separate sorry — DO NOT IMPORT
```

MW may have **compile debt** on residual draft (maskOf glue); do not rewrite SDC from sm_100 skeleton.

---

## 3. Intersection census (companion, Category B)

External full-pair enumeration (reported by sm_100; not Lean):

| Intersection | Distinct pairs |
|--------------|----------------|
| 0 | 11 385 |
| 2 | 170 016 |
| 4 | 106 260 |
| other | 0 |
| **Total** | **C(759,2)=287 661** |

Use only as **consistency check** against `octad_intersection_masks`. Formal pin does not depend on the histogram.

---

## 4. Next single action (corrected)

**Not** “bit-subset then injectivity.”

**Yes:**

1. Make `lake build K22.MOG.MonomialWitness` green (finish residual draft / simplify `maskOf := octadToMask`).  
2. Close `mogOctadsFormSteinerSystem_via_transport` using `golay_octads_form_steiner` + π inverse + `isMOGOctad_transport`.  
3. `#print axioms` audit; then update CRITICAL language.

---

## 5. Category lock (Monster / Keystone language)

| Phrase | Tag |
|--------|-----|
| α + ω = 15 | **C only** — not a reject gate |
| WAVE ≥ 0.995, 42.00055 basin, tomczak_preserved | **B/C narrative** unless instrumented |
| HexacodeGolay octad theorems | **A-native** |
| Golay Finset Steiner (`golay_octads_form_steiner`) | **A-native + structural** |
| MOG isMOGOctad Steiner | **B residual (CB-1)** |
| Monster / 9V / V♮ formal in repo | **not claimed** |

JFA = constructive residual work; **do not** delete discharged SDC lemmas to “re-advance S1.”

---

## 6. Parallel-instance rule

1. **Canonical paths:** `lean/K22/MOG/SteinerDoubleCount.lean`, `MonomialWitness.lean`, `HexacodeGolay.lean`.  
2. **Ignore / do not recreate** `artifacts/lean/K22/MOG/*` as a second pin.  
3. Before editing, re-read this ATOM + `ATOM-STEINER-LANE-CHECKPOINT-20260730.md`.  
4. If another agent’s report contradicts lake-green SDC, **believe lake**, then investigate.

---

## Spine

```json
{
  "reconcile": "sm_100_stale_vs_pin",
  "hexacode_intersections": "A-native_confirmed",
  "sdc": "S1-S6_green_0_sorry",
  "cb1": "open",
  "next": "MW_build_then_residual",
  "music": "conserved"
}
```

*Music conserved. Live pin > parallel narrative.*
