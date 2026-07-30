/-
  GF4RowAction.lean — GF(4) action on MOG rows + symbol scaling.

  Ingested from dump/MiracleOctadGenerator (3).lean (applyRowActionViaMultiplier,
  rowPairToSymbol) and aligned with Conway MOG core.

  ATOM: sm100-TriWeavon-GF4-ROW-ACTION-20260709
-/

import Mathlib.Data.Fintype.Basic
import K22.MiracleOctadGenerator

namespace K22.MOG

open MiracleOctadGenerator

abbrev MOGRow := Fin 4
abbrev Row := Fin 2 × Fin 2

def rowToPair (r : Fin 4) : Row :=
  match r.val with
  | 0 => (0, 0)
  | 1 => (0, 1)
  | 2 => (1, 0)
  | _ => (1, 1)

def pairToRow (p : Row) : Fin 4 :=
  match p with
  | (0, 0) => 0
  | (0, 1) => 1
  | (1, 0) => 2
  | (1, 1) => 3

/-- Natural GF(4) action on four MOG rows (Curtis table from dump (3)). -/
def applyRowActionViaMultiplier (μ : GF4) (r : MOGRow) : MOGRow :=
  match μ, r.val with
  | GF4.zero, _ => 0
  | GF4.one, v => ⟨v, r.isLt⟩
  | GF4.omega, 0 => 1
  | GF4.omega, 1 => 3
  | GF4.omega, 2 => 0
  | GF4.omega, 3 => 2
  | GF4.omegabar, 0 => 2
  | GF4.omegabar, 1 => 0
  | GF4.omegabar, 2 => 3
  | GF4.omegabar, 3 => 1
  | _, v => ⟨v, r.isLt⟩

/-- Two distinct rows → GF(4) symbol (symmetric). -/
def rowPairToSymbolGF4 (r1 r2 : MOGRow) : GF4 :=
  let a := min r1.val r2.val
  let b := max r1.val r2.val
  match a, b with
  | 0, 1 => GF4.one
  | 0, 2 => GF4.omega
  | 0, 3 => GF4.omegabar
  | 1, 2 => GF4.omegabar
  | 1, 3 => GF4.omega
  | 2, 3 => GF4.one
  | _, _ => GF4.zero

/-- Legacy Fin-4 encoding of symbols (0,1,ω,ω̄ ↦ 0..3). -/
def rowPairToSymbol (r1 r2 : Fin 4) : Fin 4 :=
  match rowPairToSymbolGF4 r1 r2 with
  | GF4.zero => 0
  | GF4.one => 1
  | GF4.omega => 2
  | GF4.omegabar => 3

/-- GF(4) multiplication as linear action on F₂² (Fin encoding). -/
def gf4MulAction (λ : Fin 4) (p : Row) : Row :=
  match λ, p with
  | 1, p => p
  | 2, (a, b) => (b, a + b)
  | 3, (a, b) => (a + b, a)
  | _, p => p

/--
Symbol scaling compatibility (dump (3) core lemma).
Full case analysis on μ ∈ GF(4) and rows — SlowStep if decide fails on edge cases.
-/
lemma rowPairToSymbol_compat_scaling
    (μ : GF4) (r1 r2 : MOGRow) (_h : r1 ≠ r2) :
    rowPairToSymbolGF4 (applyRowActionViaMultiplier μ r1)
                       (applyRowActionViaMultiplier μ r2) =
      GF4.mul (rowPairToSymbolGF4 r1 r2) μ := by
  cases μ <;> cases r1 using Fin.cases <;> try cases r1 using Fin.cases
  all_goals
    try cases r2 using Fin.cases
    try cases r2 using Fin.cases
    try cases r2 using Fin.cases
    try cases r2 using Fin.cases
  -- Exhaustive residual: discharge by simp/native where definitions close
  all_goals
    try (simp [applyRowActionViaMultiplier, rowPairToSymbolGF4, GF4.mul, min, max]; done)
  -- Remaining corners SlowStep (JFA)
  all_goals sorry

lemma rowPairToSymbol_compatible_with_action
    (r1 r2 : Fin 4) (λ : Fin 4) :
    rowPairToSymbol (pairToRow (gf4MulAction λ (rowToPair r1)))
                    (pairToRow (gf4MulAction λ (rowToPair r2))) =
      (rowPairToSymbol r1 r2 * λ) % 4 := by
  sorry

end K22.MOG
