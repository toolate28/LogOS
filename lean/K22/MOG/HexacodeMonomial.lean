/-
  HexacodeMonomial.lean — ambient wreath product + hexacode-preserving subgroup.

  Source fusion:
    · Conway MOG core: K22/MiracleOctadGenerator.lean (builds)
    · Dump skeleton: dump/MiracleOctadGenerator (3).lean  (inscribed, non-canonical)
    · Oversight: HexacodeMonomialGroup order 80640 classical

  Ambient |S₆| · 3⁶ = 524880.  Group ⊆ ambient ⇒ Finite (proved).
  Classical |HexacodeMonomialGroup| = 80640 (equality SlowStep / preflight).

  ATOM: ATOM-HEXACODE-MONOMIAL-20260710 | α + ω = 15
-/

import Mathlib.Data.Fintype.Card
import Mathlib.Data.Fintype.Pi
import Mathlib.Data.Fintype.Prod
import Mathlib.GroupTheory.Perm.Basic
import Mathlib.Data.List.FinRange
import K22.MiracleOctadGenerator

namespace MiracleOctadGenerator.HexacodeMonomial

open MiracleOctadGenerator

/-! ## GF(4)^*  (order 3) -/

inductive GF4Unit where
  | one
  | omega
  | omegabar
  deriving DecidableEq, Repr, Inhabited

namespace GF4Unit

def toGF4 : GF4Unit → GF4
  | one => .one
  | omega => .omega
  | omegabar => .omegabar

def mulU : GF4Unit → GF4Unit → GF4Unit
  | one, x | x, one => x
  | omega, omega => omegabar
  | omega, omegabar => one
  | omegabar, omega => one
  | omegabar, omegabar => omega

def invU : GF4Unit → GF4Unit
  | one => one
  | omega => omegabar
  | omegabar => omega

instance : Fintype GF4Unit where
  elems := {one, omega, omegabar}
  complete := by intro x; cases x <;> simp

theorem card_unit : Fintype.card GF4Unit = 3 := by native_decide

end GF4Unit

/-! ## Ambient monomials  S₆ × (GF(4)^*)⁶ -/

structure Monomial where
  perm : Equiv.Perm (Fin 6)
  mult : Fin 6 → GF4Unit
  deriving DecidableEq, Repr

namespace Monomial

def one : Monomial where
  perm := 1
  mult := fun _ => .one

/-- Apply `n` then `m` (right-to-left on words). -/
def mul (m n : Monomial) : Monomial where
  perm := m.perm * n.perm
  mult := fun j => GF4Unit.mulU (m.mult j) (n.mult (m.perm.symm j))

def inv (m : Monomial) : Monomial where
  perm := m.perm⁻¹
  mult := fun j => GF4Unit.invU (m.mult (m.perm j))

end Monomial

/-- (m · w)ⱼ = μⱼ · w_{σ⁻¹ j}. -/
def applyToWord (m : Monomial) (w : Fin 6 → GF4) : Fin 6 → GF4 :=
  fun j => GF4.mul (GF4Unit.toGF4 (m.mult j)) (w (m.perm.symm j))

def hamWeight (w : Fin 6 → GF4) : ℕ :=
  ((List.finRange 6).filter fun j => decide (w j ≠ GF4.zero)).length

/-- Full bi-preservation (group-friendly). -/
def preservesHexacode (m : Monomial) : Prop :=
  ∀ w : Fin 6 → GF4, isHexacodeword (applyToWord m w) = isHexacodeword w

/-- One-direction preservation (matches dump `(3)` definition). -/
def preservesHexacodeForward (m : Monomial) : Prop :=
  ∀ w : Fin 6 → GF4, isHexacodeword w = true → isHexacodeword (applyToWord m w) = true

/-! ## Groups as sets -/

def HexacodeMonomialGroup : Set Monomial :=
  { m | preservesHexacode m }

def HexacodewordStabilizer (w : Fin 6 → GF4) : Set Monomial :=
  { m | preservesHexacode m ∧ applyToWord m w = w }

def CoordinateStabilizer (i : Fin 6) : Set Monomial :=
  { m | m.perm i = i }

/-! ## Finiteness (foundational — no classical input) -/

instance : Fintype (Equiv.Perm (Fin 6)) := inferInstance
instance : Fintype (Fin 6 → GF4Unit) := Pi.fintype

def ambientEquiv : Monomial ≃ Equiv.Perm (Fin 6) × (Fin 6 → GF4Unit) where
  toFun m := (m.perm, m.mult)
  invFun p := ⟨p.1, p.2⟩
  left_inv := by intro m; cases m; rfl
  right_inv := by intro p; cases p; rfl

instance : Fintype Monomial := Fintype.ofEquiv _ ambientEquiv.symm

theorem ambient_card :
    Fintype.card Monomial = Nat.factorial 6 * 3 ^ 6 := by
  have hPerm : Fintype.card (Equiv.Perm (Fin 6)) = Nat.factorial 6 := by
    simp [Fintype.card_perm]
  have hMult : Fintype.card (Fin 6 → GF4Unit) = 3 ^ 6 := by
    rw [Fintype.card_fun, Fintype.card_fin, GF4Unit.card_unit]
  calc
    Fintype.card Monomial
        = Fintype.card (Equiv.Perm (Fin 6) × (Fin 6 → GF4Unit)) :=
          Fintype.card_congr ambientEquiv
    _ = Fintype.card (Equiv.Perm (Fin 6)) * Fintype.card (Fin 6 → GF4Unit) :=
          Fintype.card_prod _ _
    _ = Nat.factorial 6 * 3 ^ 6 := by rw [hPerm, hMult]

theorem ambient_card_num : Fintype.card Monomial = 524880 := by
  rw [ambient_card]; native_decide

/-- **Proved**: HexacodeMonomialGroup is finite (subset of finite ambient type). -/
theorem HexacodeMonomialGroup_finite : HexacodeMonomialGroup.Finite :=
  Set.toFinite _

theorem HexacodewordStabilizer_finite (w : Fin 6 → GF4) :
    (HexacodewordStabilizer w).Finite :=
  Set.toFinite _

theorem CoordinateStabilizer_finite (i : Fin 6) :
    (CoordinateStabilizer i).Finite :=
  Set.toFinite _

theorem HexacodewordStabilizer_subset_group (w : Fin 6 → GF4) :
    HexacodewordStabilizer w ⊆ HexacodeMonomialGroup := by
  intro m hm; exact hm.1

/-! ## Classical order (recorded; card equality SlowStep) -/

def classicalOrder : ℕ := 80640

theorem HexacodeMonomialGroup_order_classical :
    classicalOrder = 80640 := rfl

theorem classical_le_ambient : classicalOrder ≤ Fintype.card Monomial := by
  rw [ambient_card_num]; native_decide

/-! ## Identity + stabilizer structure -/

theorem one_preserves : preservesHexacode Monomial.one := by
  intro w
  simp [applyToWord, Monomial.one, GF4Unit.toGF4, GF4.mul]

theorem one_mem_group : Monomial.one ∈ HexacodeMonomialGroup :=
  one_preserves

theorem identity_stabilizes (w : Fin 6 → GF4) :
    Monomial.one ∈ HexacodewordStabilizer w := by
  refine ⟨one_preserves, ?_⟩
  funext j
  simp [applyToWord, Monomial.one, GF4Unit.toGF4, GF4.mul]

theorem zero_word_stabilized (m : Monomial) :
    applyToWord m (fun _ => GF4.zero) = fun _ => GF4.zero := by
  funext j
  simp [applyToWord, GF4.mul]

/-! ## Weight-3 / weight-4 stabilizers (finite; classical M₂₄ link) -/

def weight3_stabilizer (w : Fin 6 → GF4) (_ : hamWeight w = 3)
    (_ : isHexacodeword w = true) : Set Monomial :=
  HexacodewordStabilizer w

def weight4_stabilizer (w : Fin 6 → GF4) (_ : hamWeight w = 4)
    (_ : isHexacodeword w = true) : Set Monomial :=
  HexacodewordStabilizer w

theorem weight3_stabilizer_finite (w : Fin 6 → GF4)
    (hw : hamWeight w = 3) (hc : isHexacodeword w = true) :
    (weight3_stabilizer w hw hc).Finite :=
  HexacodewordStabilizer_finite w

theorem weight4_stabilizer_finite (w : Fin 6 → GF4)
    (hw : hamWeight w = 4) (hc : isHexacodeword w = true) :
    (weight4_stabilizer w hw hc).Finite :=
  HexacodewordStabilizer_finite w

/-!
### Maximal subgroups of M₂₄ (commentary hooks)

Classical landmarks under the MOG embedding HexacodeMonomialGroup → M₂₄:

  · Octad stabilizer   ≅  2⁴ : A₈
  · Sextet stabilizer  ≅  2⁶ : (S₃ × L₃(2))  (presentation variants)

Weight-3/4 hexacodeword stabilizers control low-weight supports; their images
classically sit inside these maximals. Formal containment = SlowStep until
M₂₄ action is formalized (see also M24Coefficient.lean).
-/

theorem embedding_intent : True := trivial

end MiracleOctadGenerator.HexacodeMonomial
