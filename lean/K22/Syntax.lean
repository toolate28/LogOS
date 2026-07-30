import Mathlib.Algebra.Ring.Basic
import Mathlib.Algebra.BigOperators.Group.List
import Mathlib.Data.List.Basic
import Mathlib.Data.Finset.Basic

/-!
# K22Syntax — Burau, determinant, and exponent-sum primitives

Canonical definitions for `k22_bridge` and K22/TriWeavon formal layer.
Use `det_burau` and `exponent_sum` lemmas when `k22_bridge` fires.

ATOM: ATOM-K22-SYNTAX-BRIDGE-20260706 | α + ω = 15
-/

namespace K22

universe u

/-- 2×2 matrix carrier for Burau / hexagon proofs. -/
structure Mat2 (α : Type u) where
  a11 : α
  a12 : α
  a21 : α
  a22 : α
  deriving DecidableEq

namespace Mat2

variable {R : Type u} [CommRing R]

def mul (A B : Mat2 R) : Mat2 R where
  a11 := A.a11 * B.a11 + A.a12 * B.a21
  a12 := A.a11 * B.a12 + A.a12 * B.a22
  a21 := A.a21 * B.a11 + A.a22 * B.a21
  a22 := A.a21 * B.a12 + A.a22 * B.a22

instance : Mul (Mat2 R) := ⟨mul⟩

@[simp] theorem mul_def (A B : Mat2 R) :
    (A * B) =
      ⟨A.a11 * B.a11 + A.a12 * B.a21,
       A.a11 * B.a12 + A.a12 * B.a22,
       A.a21 * B.a11 + A.a22 * B.a21,
       A.a21 * B.a12 + A.a22 * B.a22⟩ := rfl

/-- Determinant for custom `Mat2` (used by `det_burau` goals). -/
def det (A : Mat2 R) : R :=
  A.a11 * A.a22 - A.a12 * A.a21

@[simp] theorem det_def (A : Mat2 R) :
    det A = A.a11 * A.a22 - A.a12 * A.a21 := rfl

end Mat2

open Mat2

variable {R : Type*} [CommRing R]

/-- Standard Burau generators (hexagon test anchors). -/
def burau_σ₁ (t : R) : Mat2 R := ⟨-t, 1, 0, 1⟩
def burau_σ₂ (t : R) : Mat2 R := ⟨1, 0, t, -t⟩

/-- Determinant of a Burau generator at parameter `t`. -/
def det_burau (gen : R → Mat2 R) (t : R) : R :=
  det (gen t)

@[simp] theorem det_burau_σ₁ (t : R) :
    det_burau burau_σ₁ t = -t := by
  simp [det_burau, burau_σ₁, det_def]

@[simp] theorem det_burau_σ₂ (t : R) :
    det_burau burau_σ₂ t = -t := by
  simp [det_burau, burau_σ₂, det_def]

/-- Exponent sum over a braid-word style list `(sign, generator_index)`. -/
def exponent_sum (ws : List (ℤ × ℕ)) : ℤ :=
  (ws.map Prod.fst).sum

@[simp] theorem exponent_sum_nil : exponent_sum [] = 0 := rfl

@[simp] theorem exponent_sum_cons (e : ℤ × ℕ) (ws : List (ℤ × ℕ)) :
    exponent_sum (e :: ws) = e.1 + exponent_sum ws := by
  simp [exponent_sum, List.sum_cons, List.map_cons]

end K22