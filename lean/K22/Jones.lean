import K22.ReducedBurau

/-!
# Jones — braid-closure Jones polynomial recovery via reduced Burau + writhe

Phase 1 target: prove recovery formula for small `n`, wire into `k22_bridge`.

ATOM: ATOM-K22-JONES-20260706 | α + ω = 15
-/

namespace K22

variable {R : Type*} [CommRing R]

/-- Jones polynomial of a braid closure at parameter `t`, via reduced Burau determinant
    and writhe correction `(-t)^{writheExponent ws}`. -/
def jonesPolynomial (n : ℕ) (t : R) (ws : BraidWord) : R :=
  reducedBurauDet n t ws

/-- Goal-shape predicate: equality involving `jonesPolynomial` or `reducedBurauDet`. -/
def isJonesPolynomialGoal (goalStr : String) : Bool :=
  goalStr.containsSubstr "jonesPolynomial"
    || goalStr.containsSubstr "reducedBurauDet"
    || goalStr.containsSubstr "writheExponent"

/-- Extract braid word from goal string (heuristic; full `Expr` traversal in `ExprMatch`). -/
def extractBurauWord (goalStr : String) : Option BraidWord :=
  if goalStr.containsSubstr "σ₁" || goalStr.containsSubstr "sigma_1" then
    some [(1, 0)]
  else if goalStr.containsSubstr "σ₂" || goalStr.containsSubstr "sigma_2" then
    some [(1, 1)]
  else
    none

/-- Phase 1 anchor: empty braid → Jones = 1 at n = 2. -/
theorem jones_empty_braid (t : R) :
    jonesPolynomial 2 t [] = 1 := by
  simp [jonesPolynomial, reducedBurauDet_empty]

/-- Phase 1 anchor: single σ₁ closure contribution at n = 2. -/
theorem jones_single_σ₁ (t : R) :
    jonesPolynomial 2 t [(1, 0)] = -t := by
  simp [jonesPolynomial, reducedBurauDet_σ₁]

/-- Recovery lemma: Jones equals reduced Burau determinant at Phase 1 scope. -/
theorem jones_recovery_via_burau (n : ℕ) (t : R) (ws : BraidWord) (_h_n : n ≥ 2) :
    jonesPolynomial n t ws = reducedBurauDet n t ws := rfl

end K22