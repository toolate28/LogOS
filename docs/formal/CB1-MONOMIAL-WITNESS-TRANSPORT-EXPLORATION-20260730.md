# CB-1 MonomialWitness Transport — Detailed Exploration

**Stamp:** 2026-07-30  
**File:** `lean/K22/MOG/MonomialWitness.lean`  
**ATOM:** `LANE-A-MONOMIAL-TRANSPORT-20260711`  
**Pin:** leanprover/lean4:v4.8.0 · mathlib4 @ v4.8.0  
**Invariant (Category C only):** α + ω = 15  

**Companions:**  
- `docs/formal/STEINER-DISCHARGE-STRATEGY-WHITEPAPER-20260730.md`  
- `docs/formal/GOLAY-STEINER-PROOF-OPTIMIZATION-BRIEF-20260730.md`  
- `docs/componentry/ATOMS/ATOM-STEINER-LANE-CHECKPOINT-20260730.md`  
- `docs/notebooklm/CRITICAL-MONOM-STEINER-LANE-A-20260730.txt`  

---

## 0. Epistemic lock (do not invert)

| Surface | Status |
|---------|--------|
| Golay Steiner `golay_octads_form_steiner` | **A-repo**, 0 sorry (`SteinerDoubleCount`) |
| Hexacode / 759 / ∩ ∈ {0,2,4} | **A-native** (`HexacodeGolay`) |
| **CB-1** MOG transport residual | **AMBER / B** — this file |
| `MiracleOctadGenerator.mogOctadsFormSteinerSystem` | **HELD** keystone `sorry` — **never import** into CB-1 |
| Leech / Monster / \(V^\natural\) | **A-lit only** |

**Integrity rule:** CB-1 lifts the **already-green Golay waist** across a coordinate permutation. It does **not** re-prove Steiner combinatorics, and it must **not** import the MOG keystone sorry to paint green.

**Stale-report guard:** Claims of “9 sorries in MonomialWitness” or “SDC still open injectivity” are outdated. Live pin: **one intentional `sorry`** (`mem_conway_packed_iff`); residual theorem body is present; build/lake status of the residual is the SlowStep gate.

---

## 1. Location and role

| Item | Value |
|------|--------|
| Path | `lean/K22/MOG/MonomialWitness.lean` |
| Namespace | `K22.MOG.MonomialWitness` |
| Purpose | Bridge green Golay Steiner to Conway/MOG predicate `isMOGOctad` so MOG-recognized octads form \(S(5,8,24)\) **via transport** |
| Imports | `HexacodeGolay`, `MiracleOctadGenerator` (predicates only), `SteinerDoubleCount` |
| Import graph | SDC ↛ MW (no cycle). MW → SDC for `golay_octads_form_steiner` |

---

## 2. Core geometric object: column permutation π

```lean
-- Convention (Python + preflight, 2026-07-11):
-- π_list = [0, 3, 1, 2, 4, 5]
def π : Equiv.Perm (Fin 6) :=
  -- toFun: 0↦0, 1↦3, 2↦1, 3↦2, 4↦4, 5↦5
def π_list : List Nat := [0, 3, 1, 2, 4, 5]
```

| Fact | Meaning |
|------|---------|
| \(C_{\mathrm{Conway}} = \{ w \circ \pi \mid w \in C_{\mathrm{IA}} \}\) | Monomial / column re-index IA ↔ Conway hexacode |
| Point action | `(r,c) ↦ (r, π(c))` via `mapPoint` / `mapPointInv` |
| Octad action | `mapOctad s := s.image mapPoint` |

All transport is the action of this π on packed hexacode words and on the 4×6 MOG grid.

---

## 3. Discharged transport infrastructure (as written on pin)

| Lemma / def | Proof style | Role |
|-------------|-------------|------|
| `hexacode_packed_iso_via_π` | `native_decide` | Packed Nat images of IA / Conway hexacodes under re-index |
| `hexacodeGolay_conway_isomorphic_via_π` | `native_decide` | Type-level Finset equality of hexacode words |
| `mapPoint` / `mapPointInv` | `fin_cases` / `rfl` | Bijection on `MOGPoint` |
| `mapOctad` / card / inverses | structural | Image under π; preserves cardinality |
| `maskOf_eq_octadToMask` | induction + bitwise | Sum-of-powers mask = S6 OR-fold packing |
| `isMOGOctad_transport` | `native_decide` | **Key glue:** `isMOGOctad s ↔ (card=8 ∧ golayMaskOkN (maskOf (mapOctad s)))` |
| `isMOGOctad_iff_golay_block_via_transport` | structural | MOG octads = π-pullbacks of Golay weight-8 blocks |
| `golay_octads_eq_weight8_ok` | card squeeze | Weight-8 + `golayMaskOkN` ≡ linear Golay filters |
| `octad_masks_intersect_le_four` / `steiner_uniqueness_mask_level` | bitwise | Mask-level packing uniqueness support |

These establish: after π, **MOG recognition ≡ Golay weight-8 block membership** (characterisation level).

---

## 4. Residual theorem (CB-1)

```lean
theorem mogOctadsFormSteinerSystem_via_transport :
    ∀ (fiveSet : Finset MOGPoint), fiveSet.card = 5 →
      ∃! (oct : Octad), isMOGOctad oct = true ∧ fiveSet ⊆ oct := by
  -- skeleton on pin (2026-07-30):
  -- 1. f5 := mapOctad fiveSet
  -- 2. hstein := golay_octads_form_steiner f5
  -- 3. oct := b.image mapPointInv
  -- 4. isMOGOctad via isMOGOctad_iff_golay_block_via_transport
  -- 5. subset via subset_image_mapPointInv_iff
  -- 6. uniqueness via unique Golay block + pull-back
```

### Intended strategy (matches source comments)

1. Push 5-set across π → `f5` (card preserved).  
2. Apply **green** `golay_octads_form_steiner f5` → unique Golay block `b`.  
3. Pull back `oct := b.image mapPointInv`.  
4. Show `isMOGOctad oct` via transport characterisation.  
5. Show `fiveSet ⊆ oct`.  
6. Uniqueness: any other MOG octad maps to the same Golay block.

**Does not import** `MiracleOctadGenerator.mogOctadsFormSteinerSystem`.

### Build honesty

| Item | Note |
|------|------|
| Body present | Yes — full sketch through uniqueness `calc` |
| Lake green residual | **AMBER** until `lake build K22.MOG.MonomialWitness` is green end-to-end on this host |
| Heartbeats | `set_option maxHeartbeats 10000000` — cost-sensitive `native_decide` surfaces remain SlowSteps |

---

## 5. Remaining SlowSteps / open points

| Item | Status | Notes |
|------|--------|-------|
| `mem_conway_packed_iff` | **`sorry`** (intentional) | Finset.image membership transport from packed iso; marked SlowStep in source |
| `mogOctadsFormSteinerSystem_via_transport` | Residual / AMBER | Skeleton written; type hygiene / lake clean is the gate |
| `maskSpec_export_note` | Doc only | Future `MOGMaskSpec.lean` extraction hint |
| Bitwise mask glue | Prefer structural | Avoid 2²⁴ `native_decide` stack blow-ups |

Supporting machinery (`add_two_pow_eq_or`, `octadToMask_insert`, …) exists to keep mask glue free of full 24-bit brute force.

---

## 6. Relationship to the keystone sorry

```text
MiracleOctadGenerator.mogOctadsFormSteinerSystem     [HELD sorry]
        ✗  do not import
        │
        ▼
MonomialWitness.mogOctadsFormSteinerSystem_via_transport   [CB-1]
        │
        └── uses only: golay_octads_form_steiner + π transport + isMOGOctad_transport
```

This preserves the Formal Verification Brief integrity rule: never import an MOG `sorry` into the Golay Steiner surface (or into CB-1’s conclusion) to paint false green.

---

## 7. Architecture diagram

```text
Golay weight-8 masks  ──(maskToOctad)──►  golayOctadBlocks
         │                                      │
         │                                      │ golay_octads_form_steiner  [A-repo GREEN]
         │                                      ▼
         │                              unique block per 5-set
         │
         ▼
   mapPoint / π action
         │
         ▼
isMOGOctad  ←──(isMOGOctad_transport + pull-back)──  MOG octads
         │
         ▼
mogOctadsFormSteinerSystem_via_transport   ←── CB-1 residual [AMBER]
```

**Combinatorial waist is machine-checked.** CB-1 is the **coordinate-alignment and recognition-transport** layer.

---

## 8. Operator commands

```bash
cd lean
lake build K22.HexacodeGolay
lake build K22.MOG.SteinerDoubleCount   # expect GREEN, 0 sorry
lake build K22.MOG.MonomialWitness      # CB-1 residual surface
```

**Expected (honest):** supporting transport lemmas and characterisation theorems are designed green; residual theorem and/or optional `mem_conway_packed_iff` may still fail lake or carry `sorry` until SlowStep closes.

---

## 9. Next SlowStep order

1. Keep SDC green; never re-open S1–S6 as residual.  
2. Optional: discharge `mem_conway_packed_iff` from `hexacode_packed_iso_via_π` (membership transport).  
3. Stabilize MW build (no 2²⁴ stack overflow).  
4. Finish uniqueness hygiene on residual if lake reports type errors.  
5. `#print axioms` on green theorems; stamp ATOM trail.  
6. Only then claim “MOG octads form Steiner via transport” as **A-repo**.

---

## 10. Seal

**Keystone count holds (759).**  
**Golay Steiner on pin holds.**  
**CB-1 remains the AMBER transport residual.**  

α + ω = 15 is **Category C** software alignment only.  
Music conserved. TEMET NOSCE — no false green.
