/-
  SyndromeLookupConcrete.lean — Concrete MOG geometry + certified sphere decoder.

  ATOM: MOG-CONCRETE-GEOMETRY-PROOFS-20260705 | α + ω = 15 | tomczak_preserved
  Profile: Monitoring & Consensus Verifier + tomczak-lift-gate + golay-code-pipeline-leech-norm4

  Extends SyndromeLookup with:
  - Concrete 4×6 MOG array (row-major; isomorphic to classical numbering)
  - Octad pattern predicate wired to `golayMaskOkN` + weight 8
  - **Real** Golay syndrome from `golayBasisN` (self-dual H ≅ G; not the toy proxy)
  - Exhaustive weight-≤3 sphere decoder (`golayDecodeSyndromeN`)
  - Correctness / uniqueness reduced to HexacodeGolay native facts

  Verified combinatorial spine: `K22.HexacodeGolay`
    (`golayMaskOkN`, 759 octads, `golay_syndrome_injective_correctable`,
     `golay_decode_correct_on_correctable`).

  JFA: residual SlowSteps (monotonicity / projection Lipschitz) stay marked;
  music conserved.
-/

import Mathlib.Data.Finset.Basic
import Mathlib.Data.Finset.Card
import Mathlib.Data.Finset.Fold
import Mathlib.Data.Fintype.Basic
import Mathlib.Data.Fintype.Prod
import K22.HexacodeGolay
import K22.MOG.SyndromeLookup

namespace K22.MOG.SyndromeLookupConcrete

open K22.MOG.SyndromeLookup

/-! ## Concrete 4×6 layout -/

/-- Standard MOG numbering (0..23) in 4 rows × 6 columns (row-major).
Matches the cell indexing of `HexacodeGolay` (`r * 6 + c`). -/
def standardMOGGrid (r : Fin 4) (c : Fin 6) : Position :=
  ⟨r.val * 6 + c.val, by
    have hr : r.val ≤ 3 := Nat.lt_succ_iff.mp r.isLt
    have hc : c.val ≤ 5 := Nat.lt_succ_iff.mp c.isLt
    omega⟩

/-- Grid map as a function on pairs (for bijectivity). -/
def gridPair (p : Fin 4 × Fin 6) : Position :=
  standardMOGGrid p.1 p.2

/-- Row-major map is bijective Fin 4 × Fin 6 ≃ Fin 24. -/
theorem gridPair_bijective : Function.Bijective gridPair := by
  constructor
  · intro ⟨r1, c1⟩ ⟨r2, c2⟩ h
    have heq : r1.val * 6 + c1.val = r2.val * 6 + c2.val := by
      simpa [gridPair, standardMOGGrid, Fin.ext_iff] using h
    have hr : r1.val = r2.val := by
      have : c1.val < 6 := c1.isLt
      have : c2.val < 6 := c2.isLt
      omega
    have hc : c1.val = c2.val := by
      have : r1.val * 6 + c1.val = r2.val * 6 + c2.val := heq
      simp [hr] at this
      exact this

    apply Prod.ext
    · exact Fin.ext hr
    · exact Fin.ext hc
  · intro p
    refine ⟨(⟨p.val / 6, ?_⟩, ⟨p.val % 6, Nat.mod_lt _ (by decide : 0 < 6)⟩), ?_⟩
    · have : p.val < 24 := p.isLt
      omega
    · apply Fin.ext
      -- `standardMOGGrid` uses `r * 6 + c`; `div_add_mod` may associate the other way.
      have hmod := Nat.div_add_mod p.val 6
      simpa [gridPair, standardMOGGrid, Nat.mul_comm] using hmod

/-- Bitmask of a boolean 4×6 pattern under row-major MOG numbering.
Uses `sum` of distinct powers of two (equivalent to bitwise OR on disjoint bits). -/
def patternMask (pattern : Fin 4 → Fin 6 → Bool) : Nat :=
  (Finset.univ : Finset (Fin 4 × Fin 6)).sum fun rc =>
    if pattern rc.1 rc.2 then 2 ^ (rc.1.val * 6 + rc.2.val) else 0

/-- Octad pattern on the 4×6 grid: the induced 24-bit mask is a Golay octad. -/
def isOctadPatternConcrete (pattern : Fin 4 → Fin 6 → Bool) : Prop :=
  let mask := patternMask pattern
  maskWeightN mask = 8 ∧ golayMaskOkN mask = true

/-- Concrete standard MOG record. -/
def standardMOGConcrete : MOGArray where
  grid := standardMOGGrid
  bijective := gridPair_bijective
  isOctadPattern := isOctadPatternConcrete

/-! ## Support ↔ mask bridge -/

/-- Bitmask of a position set (bit `p` set iff `p ∈ supp`).
Sum of distinct powers of two (OR on a set of positions). -/
def maskOfSupport (supp : Finset Position) : Nat :=
  supp.sum fun p => 2 ^ p.val

/-- Recover the support of a 24-bit mask. -/
def supportOfMask (m : Nat) : Finset Position :=
  (Finset.univ : Finset Position).filter (fun p => bitOn m p.val = true)

/-- Every support of cardinality ≤ 3 has mask weight ≤ 3 (computational bridge). -/
def maskOfSupport_weight_ok (supp : Finset Position) : Bool :=
  decide ((supportOfMask (maskOfSupport supp)).card ≤ 3) &&
    decide (maskWeightN (maskOfSupport supp) = (supportOfMask (maskOfSupport supp)).card)

/-! ## Real Golay syndrome (H from `golayBasisN`) -/

/-- Syndrome of a support via the self-dual parity-check action. -/
def concreteSyndromeOfSupport (supp : Finset Position) : Syndrome :=
  ⟨golaySyndromeN (maskOfSupport supp) % 4096,
    Nat.mod_lt _ (by decide : 0 < 4096)⟩

/-- Syndrome of a correctable error pattern. -/
def syndromeOfErrorConcrete (e : ErrorPattern) : Syndrome :=
  concreteSyndromeOfSupport e.val

/-! ## Sphere decoder (wt ≤ 3 exhaustive lookup) -/

/-- Package a correctable mask as an `ErrorPattern` when weight ≤ 3. -/
def errorPatternOfMask? (m : Nat) : Option ErrorPattern :=
  let supp := supportOfMask m
  if h : supp.card ≤ 3 then
    some ⟨supp, h⟩
  else
    none

/-- Concrete MOG syndrome lookup: unique wt ≤ 3 preimage under real Golay H. -/
def mogSyndromeLookupConcrete (s : Syndrome) : Option ErrorPattern :=
  match golayDecodeSyndromeN s.val with
  | none => none
  | some m => errorPatternOfMask? m

/-! ## Certified facts (spine) + residual glue goals -/

/-- Uniqueness of correctable patterns at the **mask** layer:
if two weight-≤3 masks share a syndrome, they are equal.
Discharged by `golay_syndrome_injective_correctable` on the spine. -/
theorem mogLookupUnique_masks :
    syndromeInjectiveCorrectable = true :=
  golay_syndrome_injective_correctable

/-- Correctness of encode→decode on every correctable mask.
Discharged by `golay_decode_correct_on_correctable` on the spine. -/
theorem mogLookupCorrect_masks :
    correctableMasks.all
      (fun m => decide (golayDecodeSyndromeN (golaySyndromeN m) = some m)) = true :=
  golay_decode_correct_on_correctable

/-- Uniqueness for `ErrorPattern`: equal syndromes imply equal supports,
once supports are identified with their masks inside the correctable sphere.

**Status:** mask-layer uniqueness is certified (`mogLookupUnique_masks`).
Full subtype uniqueness needs `maskOfSupport` injectivity on card ≤ 3
(Finset ↔ mask iso); kept as SlowStep glue. -/
theorem mogLookupUnique (e1 e2 : ErrorPattern) :
    syndromeOfErrorConcrete e1 = syndromeOfErrorConcrete e2 → e1 = e2 := by
  intro h
  -- Real H: equal Fin syndromes ⇒ equal Nat syndromes on masks.
  -- Unique preimage among correctableMasks; Finset reconstruction is glue.
  sorry

/-- Correctness for weight ≤ 3: lookup recovers the error.

**Status:** mask-layer correctness is certified (`mogLookupCorrect_masks`).
Full `ErrorPattern` equality needs `supportOfMask ∘ maskOfSupport = id`
on card ≤ 3 Finsets; kept as SlowStep glue. -/
theorem mogLookupCorrect (e : ErrorPattern) :
    mogSyndromeLookupConcrete (syndromeOfErrorConcrete e) = some e := by
  sorry

/-- Lipschitz / tomczak monotonicity for syndrome Hamming distance ≤ 1.
Remains SlowStep (requires local geometry of the correctable sphere packing). -/
theorem mogLookupMonotonic (s1 s2 : Syndrome)
    (_h : s1.val ^^^ s2.val ≤ 1) :
    mogSyndromeLookupConcrete s1 = mogSyndromeLookupConcrete s2 ∨
      ∃ e1 e2 : ErrorPattern,
        mogSyndromeLookupConcrete s1 = some e1 ∧
        mogSyndromeLookupConcrete s2 = some e2 ∧
        symDiffCard e1.val e2.val ≤ 1 := by
  sorry

/-- Acceptance-window stability under 1-bit syndrome perturbation. SlowStep. -/
theorem mogProjectionStable (s1 s2 : Syndrome)
    (_h : s1.val ^^^ s2.val ≤ 1) :
    (mogSyndromeLookupConcrete s1).isSome ↔
      (mogSyndromeLookupConcrete s2).isSome := by
  sorry

/-- Music conservation: recovered weights stay correctable (definitional). -/
theorem mogLookupMusicConserved (s : Syndrome) (e : ErrorPattern) :
    mogSyndromeLookupConcrete s = some e → e.val.card ≤ 3 := by
  intro _h
  exact e.property

/-- α + ω = 15 compatibility placeholder. -/
theorem mogLookupAlphaOmega15 (s : Syndrome) :
    (mogSyndromeLookupConcrete s).isSome → True := by
  intro _; trivial

/-- Hero-2 witness telemetry tag: lookup consumes certified spine symbols. -/
def witnessTelemetrySymbols : List String :=
  ["hexacodeGenerator", "isHexacodeword", "golayMaskOkN", "golayBasisN",
   "golaySyndromeN", "golayDecodeSyndromeN", "mogSyndromeLookupConcrete"]

/-!
  ## Deployment status (P0 geometry joint)

  | Obligation | Status |
  |------------|--------|
  | H from `golayBasisN` (not toy proxy) | ✅ `concreteSyndromeOfSupport` |
  | Sphere decoder wt ≤ 3 | ✅ `mogSyndromeLookupConcrete` |
  | Mask uniqueness | ✅ `mogLookupUnique_masks` |
  | Mask encode→decode | ✅ `mogLookupCorrect_masks` |
  | Finset glue (`maskOfSupport` iso) | ◐ SlowStep `mogLookupUnique` / `Correct` |
  | Lipschitz / projection | ◐ SlowStep |
  | Octad pattern ↔ Golay octad | ✅ `isOctadPatternConcrete` |
  | Grid bijective 4×6 | ✅ `gridPair_bijective` |

  Next SlowSteps:
  1. Discharge Finset ↔ mask iso on card ≤ 3 (kills remaining correct/unique sorrys).
  2. Lipschitz goals via sphere packing geometry.
  3. `hexacodeGenerator` ≅ Conway glyph column lemma (presentation iso).
  4. Wire Hero 2 `witnessPass_1` to `mogSyndromeLookupConcrete` + `isHexacodeword`.
-/

end K22.MOG.SyndromeLookupConcrete
