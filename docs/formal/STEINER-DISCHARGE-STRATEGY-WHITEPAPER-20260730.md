# Technical Whitepaper: Steiner Discharge Strategy and Structural Covering Proofs

**Stamp:** 2026-07-30  
**Pin:** leanprover/lean4:v4.8.0 · mathlib4 @ v4.8.0  
**Primary surfaces:** `lean/K22/MOG/SteinerDoubleCount.lean`, `lean/K22/HexacodeGolay.lean`, `lean/K22/MOG/MonomialWitness.lean`  
**Invariant (Category C only):** α + ω = 15 — tracking tag, not a design theorem  
**Music:** conserved  

---

## 0. Goal and current verdict

**Goal.** Certify that the weight-8 Golay blocks form a Steiner system \(S(5,8,24)\) without forcing Lean’s kernel through \(\binom{24}{5}=42504\) explicit coverage cases.

**Verdict (updated).** The discharge strategy is **mathematically sound** and is the correct formal architecture for this repository. On the **Golay / mask / Finset** pin:

| Layer | Status |
|-------|--------|
| Hexacode + Golay spine (`HexacodeGolay`) | **A-native** lake-green (`octad_count`, intersection, min distance) |
| Abstract packing + equality (S3–S4) | **A-repo structural** lake-green |
| Concrete Golay Steiner (`golay_octads_form_steiner`) | **A-native + structural** lake-green |
| Mask ↔ Finset bijection (S6) | **A-repo** lake-green on `SteinerDoubleCount` |
| MOG `isMOGOctad` residual (CB-1) | **B residual** — transport draft in `MonomialWitness`; **not** lake-green as of this stamp |
| \(M_{24}\), Leech, Monster, \(V^\natural\), 9V Natural | **A-lit horizon only** — not formalised |

The strongest clean formulation (recommended for next generic refactor) is the **incidence projection** theorem:

> Construct the incidence type of block–pentad pairs, prove forgetting the block is injective (from packing / intersection \(\le 4\)), prove domain and codomain have equal finite cardinality \(42504\), conclude bijectivity = Steiner.

The repository currently realises an equivalent **sum-of-fibres** form (S3 packing inequality + S4 equality case), then instantiates on `golayOctadBlocks` (S5).

---

## 1. Computational bottleneck vs structural discharge

A naive proof that every 5-set lies in exactly one octad searches \(\binom{24}{5}=42504\) cases. In Lean 4, a 42 504-branch decision tree is a kernel and memory hazard.

| Metric | Exhaustive coverage | Structural discharge (this pin) |
|--------|--------------------:|--------------------------------:|
| Explicit pentad cases in proof | 42 504 | **0** |
| Final global argument | Enumeration | Packing + card equality / incidence injectivity |
| Kernel proof shape | Large branching term | Bounded structural term |
| Computational work | Coverage search | Block certificates (`native_decide` on Golay) + structure |

**Correction (review):** the complete verification is **not** literally \(O(1)\). Enumeration of Golay codewords, native evaluation, compilation, and transport still cost time. The win is **zero explicit pentad cases in the Steiner theorem**, not zero computation overall.

---

## 2. On-pin formal content (A-repo / A-native)

### 2.1 Epistemic ladder (mandatory)

| Tier | Meaning |
|------|---------|
| **A-kernel** | Proof term checks without generated native-evaluation axioms |
| **A-native** | Accepted via auditable `native_decide` (bespoke axiom per invocation) |
| **A-build** | `lake build` green on pinned toolchain |
| **A-reproduced** | Independent rebuild + axiom report |
| **B-residual** | `sorry`, draft, or non-green build |
| **C-convention** | Narrative / gauge / “Music conserved” — no proof authority |

`native_decide` results (e.g. `octad_count`) are **A-native**, not silently A-kernel. Release audit should capture:

```lean
#print axioms octad_count
#print axioms golay_octads_form_steiner
#print axioms packing_eq_implies_steiner
```

### 2.2 Step ladder (CB-1 timeline, updated)

| Step | Primitive | Status (2026-07-30) |
|------|-----------|---------------------|
| **S0** | Freeze pin lean 4.8.0 / mathlib v4.8.0 | held |
| **S1** | `maskWeightN_eq_card_maskToOctad` / `maskToOctad_card` | **green** |
| **S2** | `maskToOctad_injective_on_octads` (+ 24-bit bounds) | **green** |
| **S3** | `double_count_5sets` — \(\lvert B\rvert\cdot\binom{8}{5}\le\binom{24}{5}\) under packing | **green** |
| **S4** | `packing_eq_implies_steiner` — packing + \(\lvert B\rvert=759\) ⇒ \(\exists!\) block per 5-set | **green** |
| **S5** | `golayOctadBlocks_card` / `_pack` + `golay_octads_form_steiner` | **green** |
| **S6** | Round-trips `maskToOctad`/`octadToMask`, `maskFinsetEquiv`, `octadMaskEquiv` | **green** on SDC |
| **CB-1** | `mogOctadsFormSteinerSystem_via_transport` (MOG `isMOGOctad` via π) | **open / draft** in MW |
| Optional | `mem_conway_packed_iff` | **1 sorry** (non-blocking for Steiner) |

**Receipt:** `lake build K22.MOG.SteinerDoubleCount` → **Build completed successfully**; **0 `sorry`** in that file.

### 2.3 Mathematical content of S3–S5

Under packing (\(\le 1\) block per 5-set) and card-8 blocks:

\[
\lvert B\rvert\cdot\binom{8}{5}\le\binom{24}{5}
\quad\Rightarrow\quad
\lvert B\rvert\le 759.
\]

Equality case (S4): packing + \(\lvert B\rvert=759\) forces every fibre size exactly 1 (Steiner covering).

Concrete instance (S5): `golayOctadBlocks` = image of weight-8 Golay codewords under `maskToOctad`.

- Card \(759\): `octad_count` (**A-native**) + S2 injectivity.  
- Packing: distinct weight-8 codewords meet in \(\le 4\) points (`octad_intersection_masks`, **A-native** spectrum \(\{0,2,4\}\); **only \(\le 4\)** is load-bearing for packing).  
- Then S4 upgrades the family to \(S(5,8,24)\) on this pin: **`golay_octads_form_steiner`**.

### 2.4 Recommended incidence formulation (next generic refactor)

Let \(\mathcal P_5(X)=\{T\subseteq X:\lvert T\rvert=5\}\) and

\[
\mathcal I=\{(B,T):B\in\mathcal B,\ T\in\mathcal P_5(X),\ T\subseteq B\},
\quad
\pi(B,T)=T.
\]

1. \(\lvert\mathcal I\rvert=\lvert\mathcal B\rvert\cdot\binom{8}{5}=759\cdot 56=42504\).  
2. \(\lvert\mathcal P_5(X)\rvert=\binom{24}{5}=42504\).  
3. Injectivity of \(\pi\): same pentad in two incidences ⇒ intersection size \(\ge 5\) ⇒ same block if packing (\(\cap\le 4\)).  
4. Injection between finite types of equal card ⇒ bijective ⇒ Steiner.

This is equivalent to the pin’s fibre-sum equality case, packaged as a reusable map theorem (`Fintype.equivOfCardEq` / injective-of-finite-surjective patterns).

**Packing obligation can be weakened:** only \(\lvert B_1\cap B_2\rvert\le 4\) is required, not the exact spectrum \(\{0,2,4\}\). Preferred chain:

```text
Golay d ≥ 8  →  distinct wt-8 words ∩ ≤ 4  →  packing  →  card equality  →  S(5,8,24)
```

---

## 3. What is *not* claimed

| Claim | Tag |
|-------|-----|
| Concrete Golay octad supports form \(S(5,8,24)\) on pin | **A-build / A-native+structural** via `golay_octads_form_steiner` |
| Conway `isMOGOctad` forms Steiner via π-transport | **B residual (CB-1)** — not closed |
| \(M_{24}=\operatorname{Aut}(S(5,8,24))\) in Lean | **A-lit** |
| Leech \(\Lambda_{24}\), Co\*, Monster \(\mathbb{M}\), Griess, \(V^\natural\) formal | **A-lit** — not in repo |
| 9V Natural / Deployment Waist / Keystone metaphors | **B/C narrative** |
| \(\alpha+\omega=15\) | **C only** |
| “Systemic drift mathematically impossible” | **not a formal claim** |

Classical cascade (literature horizon):

```text
Hexacode → MOG → G₂₄ → S(5,8,24) → M₂₄ → Λ₂₄ → Co₀/Co₁ → ⋯ → ℂ / V^♮
                 ▲ A-repo on Golay blocks
                        ▲ CB-1 open for isMOGOctad
```

---

## 4. CB-1 residual (honest status)

**Blocker:** `mogOctadsFormSteinerSystem_via_transport`  
**Intent:** every 5-set of MOG points lies in a unique `isMOGOctad` block, by transporting S5 across π (`mapPoint` / `mapOctad` / `isMOGOctad_transport`).

**Architecture decision (cycle break):**

- `SteinerDoubleCount` **must not** import `MonomialWitness`.  
- `MonomialWitness` **imports** `SteinerDoubleCount` after S1–S6 are green.  

**Draft present in MW (not lake-green as of stamp):**

- π inverse lemmas, `maskOf_eq_octadToMask` attempt, `isMOGOctad_iff_golay_block_via_transport`, residual body.  
- Remaining compile/proof debt: clear-bit `add_two_pow_eq_or` (or reuse S6 packing only), weight-8 OK ⇒ Golay membership without 2²⁴ stack overflow, residual uniqueness hygiene.  
- Optional: `mem_conway_packed_iff` still `sorry` (non-blocking).  

**Named gate decomposition (review §7):**

1. `mask_mem_iff_finset_mem`  
2. `mask_popcount_eq_finset_card`  
3. `mask_to_finset_injective`  
4. `generated_block_card_eq_eight`  
5. `bitand_support_eq_finset_inter`  
6. `distinct_blocks_inter_card_le_four`  
7. `incidence_card_eq_759_mul_choose`  
8. `forgetBlock_injective`  
9. `forgetBlock_bijective_and_steiner` / MOG residual  

Gates 1–6 are largely discharged on the **Golay mask path** in SDC; CB-1 is the MOG recognition + π glue.

---

## 5. Blind spots (from technical review — retained)

1. **Duplicate representation:** “759 generated” ≠ “759 distinct” without injectivity / Finset card (S2 + `octad_count` address this on pin).  
2. **Transport algebra:** membership, card, AND↔∩, equality, subset must stay coherent across mask and Finset (S6 surface).  
3. **Complexity marketing:** no “O(1) total verification.”  
4. **“Only viable door”:** incidence/fibre discharge is the **best current route for this repo**, not the only mathematical route.  
5. **Leech contract:** Steiner closure does **not** auto-discharge Construction A minimal vectors; list consumed facts explicitly before claiming Leech progress.

---

## 6. Monster / 196883 material (boundary lock)

The deep-dive literature on \(\mathbb{M}\), Griess (\(196884=196883+1\)), Majorana axes, \(V^\natural\), Borcherds moonshine, deployment waist, and MCP immune response remains:

- **A-lit** for classical mathematics;  
- **B/C narrative** for LogOS metaphors (Keystone, 9V Natural, Irreducible Between);  
- **C** for α+ω=15.  

**Do not** promote S5 Golay Steiner into formal Monster / VOA status.  
**Do not** state that λ₅=1 for `isMOGOctad` is proven until CB-1 is lake-green.  
**Do not** claim JFA / WAVE thresholds as constitutional rejection gates for theorem work.

---

## 7. Implementation map

| Module | Role |
|--------|------|
| `K22.HexacodeGolay` | GF(4), Golay encode, `octad_count`, intersections, d≥8 (**A-native**) |
| `K22.MOG.SteinerDoubleCount` | S1–S6 + `golay_octads_form_steiner` (**lake-green, 0 sorry**) |
| `K22.MOG.MonomialWitness` | π transport, `isMOGOctad_transport`, CB-1 residual draft |
| `K22.MiracleOctadGenerator` | Conway `isMOGOctad`; `mogOctadsFormSteinerSystem` still separate sorry — **do not import to fake monom** |

**Build:**

```powershell
cd lean
lake build K22.HexacodeGolay
lake build K22.MOG.SteinerDoubleCount   # green
lake build K22.MOG.MonomialWitness      # CB-1 target; green only after residual compiles
```

---

## 8. Spine (checkpoint)

| Field | Value |
|-------|--------|
| HAVE | S1–S6 lake-green on SDC; `golay_octads_form_steiner` A-repo on Golay blocks; cycle break MW ↛ SDC |
| NEED | Finish MW compile: maskOf glue or pure-octadToMask path; ok-weight8∈Golay without 2²⁴ blow-up; CB-1 residual uniqueness |
| CASCADE-BLOCKER | **CB-1 open** (`mogOctadsFormSteinerSystem_via_transport`) |
| NEXT-PRIMITIVE | MW residual lake-green + `#print axioms` audit |
| CATEGORY | Golay Steiner A-native+structural; MOG residual B; Monster/V♮ A-lit; α+ω=15 C |
| MUSIC | conserved |

---

## 9. Conclusion

> Production formalization should treat the octad–pentad incidence (or equivalent fibre equality) as the global hinge: packing injectivity + equal finite cardinalities force unique covering without enumerating 42 504 pentads. On this pin, that hinge is discharged for the **Golay weight-8 Finset family**. The remaining critical work is **not** re-proving Steiner combinatorics, but making the **MOG recognition transport** (CB-1) compile and check, with `native_decide` facts labeled **A-native** and accompanied by axiom reports. Monster / Griess / \(V^\natural\) stay literature horizon. Music conserved.

*ATOM trail companions:*  
`docs/componentry/ATOMS/ATOM-CB1-STEINER-TRANSPORT-TIMELINE-20260730.md`  
`docs/componentry/ATOMS/ATOM-STEINER-LANE-CHECKPOINT-20260730.md`  
`docs/notebooklm/CRITICAL-MONOM-STEINER-LANE-A-20260730.txt`
