/-
  MiracleOctadGenerator.lean

  MOG: Conway/Curtis definition (first-principles, E2E-certified).
  Rows labelled F₄ top→bottom: 0, 1, ω, ω̄.
  S ∈ G₂₄ ⇔
    (P) column parities + top-row parity all equal
    (Σ) F₄ column sums (⊕ row labels) form a hexacodeword
  Octads = wt-8 codewords; |Octads|=759 (preflight: MOG/preflight_mog_e2e.py).

  ATOM: sm100-TriWeavon-MOG-HEXACODE-20260709 | α + ω = 15
  JFA: open lemmas = SlowStep (sorry), never deleted.
  VOID fixed: count-only columnSymbol admitted 0 octads (legacy preflight).

  Companion modules:
    HexacodeGolay.lean         — VERIFIED spine: Field GF4, golayMaskOkN, octad_count=759,
                                 weight enumerator, min distance (no sorry; native_decide)
    MOG/HexacodeMonomial.lean  — HexacodeMonomialGroup (finite; order 80640 classical)
    MOG/GF4RowAction.lean      — row scaling from dump (3)
    MOG/DUMP-INGEST-MiracleOctadGenerator-3.md — inscribed dump map
    MOG/DUMP-INGEST-HexacodeGolay.md — aristotle dump → canonical path
-/

import Mathlib.Data.Finset.Basic
import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Basic
import Mathlib.Data.Fintype.Card
import Mathlib.Data.Fintype.Pi
import Mathlib.Data.List.FinRange

namespace MiracleOctadGenerator

abbrev MOGPoint := Fin 24

def standardMOG (i : Fin 4) (j : Fin 6) : MOGPoint :=
  ⟨i.val * 6 + j.val, by
    have : i.val ≤ 3 := Nat.lt_succ_iff.mp i.isLt
    have : j.val ≤ 5 := Nat.lt_succ_iff.mp j.isLt
    omega⟩

abbrev Tetrad := Finset MOGPoint
abbrev Octad := Finset MOGPoint

def column (j : Fin 6) : Tetrad :=
  Finset.univ.image fun i : Fin 4 => standardMOG i j

def row (i : Fin 4) : Tetrad :=
  Finset.univ.image fun j : Fin 6 => standardMOG i j

/-! ## GF(4) -/

inductive GF4 where
  | zero
  | one
  | omega
  | omegabar
  deriving DecidableEq, Repr, Inhabited

namespace GF4

def add : GF4 → GF4 → GF4
  | zero, x | x, zero => x
  | one, one => zero
  | one, omega => omegabar
  | one, omegabar => omega
  | omega, one => omegabar
  | omega, omega => zero
  | omega, omegabar => one
  | omegabar, one => omega
  | omegabar, omega => one
  | omegabar, omegabar => zero

def mul : GF4 → GF4 → GF4
  | zero, _ | _, zero => zero
  | one, x | x, one => x
  | omega, omega => omegabar
  | omega, omegabar => one
  | omegabar, omega => one
  | omegabar, omegabar => omega

instance : Add GF4 := ⟨add⟩
instance : Mul GF4 := ⟨mul⟩
instance : Zero GF4 := ⟨zero⟩
instance : One GF4 := ⟨one⟩

instance : Fintype GF4 where
  elems := {.zero, .one, .omega, .omegabar}
  complete := by intro x; cases x <;> simp

end GF4

/-! ## Hexacode — Conway standard
  (a,b,c) ↦ (a, b, c, a+b+c, ωa+ω̄b+c, ω̄a+ωb+c)
  Generator rows as glyphs: 1001WB / 0101BW / 001111
-/

def hexacodeGenerator (i : Fin 3) (j : Fin 6) : GF4 :=
  match i.val, j.val with
  | 0, 0 => .one | 0, 1 => .zero | 0, 2 => .zero
  | 0, 3 => .one | 0, 4 => .omega | 0, 5 => .omegabar
  | 1, 0 => .zero | 1, 1 => .one | 1, 2 => .zero
  | 1, 3 => .one | 1, 4 => .omegabar | 1, 5 => .omega
  | 2, 0 => .zero | 2, 1 => .zero | 2, 2 => .one
  | 2, 3 => .one | 2, 4 => .one | 2, 5 => .one
  | _, _ => .zero

/-- Manual vecMul (no Semiring instance required). -/
def applyGenerator (m : Fin 3 → GF4) : Fin 6 → GF4 := fun j =>
  GF4.add (GF4.add
    (GF4.mul (m 0) (hexacodeGenerator 0 j))
    (GF4.mul (m 1) (hexacodeGenerator 1 j)))
    (GF4.mul (m 2) (hexacodeGenerator 2 j))

def hexacodeCodewords : Finset (Fin 6 → GF4) :=
  (Finset.univ : Finset (Fin 3 → GF4)).image applyGenerator

def isHexacodeword (word : Fin 6 → GF4) : Bool :=
  decide (word ∈ hexacodeCodewords)

/-! ## Conway MOG recognition (column sums + shared parity) -/

/-- Row i label in F₄ (top → bottom): 0, 1, ω, ω̄. -/
def rowLabel (i : Fin 4) : GF4 :=
  match i.val with
  | 0 => .zero
  | 1 => .one
  | 2 => .omega
  | _ => .omegabar

def pointsInColumn (s : Finset MOGPoint) (j : Fin 6) : Finset (Fin 4) :=
  Finset.univ.filter fun i => standardMOG i j ∈ s

def columnCounts (s : Finset MOGPoint) (j : Fin 6) : ℕ :=
  (pointsInColumn s j).card

/-- Σ of F₄ row labels in column j (PlanetMath Σ(S)). -/
def columnSum (s : Finset MOGPoint) (j : Fin 6) : GF4 :=
  (List.finRange 4).foldl
    (fun acc i =>
      if standardMOG i j ∈ s then GF4.add acc (rowLabel i) else acc)
    GF4.zero

def columnSums (s : Finset MOGPoint) : Fin 6 → GF4 :=
  fun j => columnSum s j

/-- Legacy count→glyph map (VOID: admits 0 octads). Kept for diagnostics only. -/
def columnSymbol (count : ℕ) : GF4 :=
  match count with
  | 0 => .zero
  | 4 => .one
  | 2 => .omega
  | _ => .omegabar

def columnSymbols (s : Finset MOGPoint) : Fin 6 → GF4 :=
  fun j => columnSymbol (columnCounts s j)

def topRowCount (s : Finset MOGPoint) : ℕ :=
  (List.finRange 6).filter (fun j => standardMOG 0 j ∈ s) |>.length

/-- All six column parities equal the top-row parity. -/
def parityOk (s : Finset MOGPoint) : Bool :=
  let p0 := columnCounts s 0 % 2
  (List.finRange 6).all (fun j => columnCounts s j % 2 = p0) &&
  (topRowCount s % 2 = p0)

def isGolay (s : Finset MOGPoint) : Bool :=
  parityOk s && isHexacodeword (columnSums s)

/-- Even-type: all columns even (implies top even under parityOk). -/
def isEvenTypeOctad (s : Finset MOGPoint) : Bool :=
  (List.finRange 6).all (fun j => columnCounts s j % 2 = 0) &&
  parityOk s && isHexacodeword (columnSums s)

/-- Odd-type: all columns odd (shared odd parity). -/
def isOddTypeOctad (s : Finset MOGPoint) : Bool :=
  (List.finRange 6).all (fun j => columnCounts s j % 2 = 1) &&
  parityOk s && isHexacodeword (columnSums s)

def isMOGOctad (s : Finset MOGPoint) : Bool :=
  if s.card ≠ 8 then false
  else isGolay s

/-! ## Decode skeleton -/

abbrev ReceivedWord := MOGPoint → Bool

def errorSupport (word : ReceivedWord) : Finset MOGPoint :=
  Finset.univ.filter fun p => word p = true

def mogDecode (word : ReceivedWord) : Option (MOGPoint → Bool) :=
  let err := errorSupport word
  if isMOGOctad err then some fun p => !(word p)
  else if err.card ≤ 3 then some fun p => !(word p)
  else none

/-! ## Keystone — SlowStep sorrys -/

/-
  ### Steiner S(5,8,24) uniqueness — status (T-Formal-01 / 2026-07-11)

  **Statement.** `mogOctadsFormSteinerSystem` asserts the design equation for
  the MOG octads: every 5-set of the 24 MOG points lies in a unique octad
  recognized by `isMOGOctad`.

  **Classical argument (Conway–Curtis / hexacode + parity).**
  1. Hexacode is the MDS [6,3,4] code over GF(4) (`hexacodeCodewords`, |C|=64).
  2. Column-sum map `columnSums` + shared column/top parity `parityOk` realise
     the binary Golay code G₂₄ as the MOG rule (`isGolay` / `isMOGOctad`).
  3. Hexacode minimum distance 4 forces: if two distinct weight-8 Golay words
     agreed on 5 coordinates, their difference would be a nonzero Golay word
     of weight ≤ 6, contradicting d(G₂₄)=8. Hence at most one octad per 5-set.
  4. Double counting then yields existence: |Octads|·C(8,5) = C(24,5) when
     |Octads|=759 (verified combinatorially in `HexacodeGolay.lean` /
     `MOG/preflight_mog_e2e.py`).

  **Witness (machine-checked, Python MOG generator — not yet Lean).**
  `MOG/preflight_mog_e2e.py` `test_steiner` (exhaustive mode) counts every
  5-subset of every MOG octad. Expected design equation:
      759 · C(8,5) = 759 · 56 = 42_504 = C(24,5)
  with cover multiplicity ≤ 1 for every five. See existence_certificate_mog.json
  component "Steiner uniqueness sample". Prior 300-sample MC is retained as a
  secondary MC10000 line under the same component.

  **Honest SlowStep gap (this `sorry`).**
  Full Lean discharge of `mogOctadsFormSteinerSystem` (∀ five, ∃! octad) is
  still open. Supporting spine already no-sorry in HexacodeGolay.lean
  (|C|=64, weight enumerator, d=8). Do not delete this sorry (JFA SlowStep).

  **Parity-lift rank (T-Formal-01 / Gaussian elimination).**
  Companion module `K22.MOG.ParityLiftRank` proves:
    • rank(A) = 9 for the 10×24 row+column parity matrix on 4×6 over GF(2)
    • global dependency Σ row-checks = Σ col-checks = all-ones (24 bits)
    • even-sum kernel dimension = 15 (so even-sum alone ⇏ D = 0)
  Restricted MOG kernel triviality (equal column-sum octad differences) remains
  open glue between that module and this Steiner keystone.

  ATOM: sm100-FIRST-RUN-20260706 + Steiner-uniqueness-verification-20260711
  + T-Formal-01-exhaustive-witness + gaussian-elimination-verification-20260711
-/

theorem mogOctadsFormSteinerSystem :
    ∀ (fiveSet : Finset MOGPoint), fiveSet.card = 5 →
      ∃! (oct : Octad), isMOGOctad oct = true ∧ fiveSet ⊆ oct := by
  sorry

lemma isMOGOctad_card (s : Finset MOGPoint) (h : isMOGOctad s = true) :
    s.card = 8 := by
  unfold isMOGOctad at h
  by_cases hc : s.card = 8
  · exact hc
  · simp [hc] at h

lemma isMOGOctad_implies_size_8 (s : Finset MOGPoint) :
    isMOGOctad s = true → s.card = 8 :=
  fun h => isMOGOctad_card s h

/-- Cardinality claim kept non-computational (2²⁴ universe too large for univ.filter). -/
lemma number_of_mog_octads :
    ∃ (S : Finset (Finset MOGPoint)),
      (∀ s ∈ S, isMOGOctad s = true) ∧ S.card = 759 := by
  sorry

lemma mog_octad_intersection_size (o1 o2 : Octad)
    (_h1 : isMOGOctad o1 = true) (_h2 : isMOGOctad o2 = true) (_hne : o1 ≠ o2) :
    (o1 ∩ o2).card ∈ ({0, 2, 4} : Finset ℕ) := by
  sorry

lemma applyGenerator_zero :
    applyGenerator (fun _ => GF4.zero) = fun _ => GF4.zero := by
  funext j
  simp [applyGenerator, GF4.mul, GF4.add]

example : hexacodeGenerator 0 0 = GF4.one := rfl
example : hexacodeGenerator 0 4 = GF4.omega := rfl

end MiracleOctadGenerator
