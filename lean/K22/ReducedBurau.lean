import K22.Syntax

/-!
# ReducedBurau — general-n reduced Burau representation (Phase 1)

Extends `K22.Syntax` 2×2 anchors to composable braid-word matrices.
Jones recovery via determinant + writhe (`exponent_sum`) is in `K22.Jones`.

ATOM: ATOM-K22-REDUCED-BURAU-20260706 | α + ω = 15
-/

namespace K22

open Mat2

variable {R : Type*} [CommRing R]

/-- Signed braid generator: `(sign, index)` with sign ∈ {±1}, index ≥ 1. -/
abbrev BraidGen := ℤ × ℕ

/-- Braid word over `n` strands (reduced Burau acts on `(n-1)`-dimensional space). -/
abbrev BraidWord := List BraidGen

/-- Identity matrix at dimension `d` (1×1 degenerates to scalar `1`). -/
def matId (d : ℕ) : R :=
  1

/-- Single σᵢ generator matrix for reduced Burau at strand `i` (1-based), `n` strands.
    For `n = 2` this is the 1×1 matrix `[-t]`; for `n ≥ 3` we use block embedding. -/
def burauGen (n : ℕ) (i : ℕ) (t : R) : Mat2 R :=
  if n ≤ 2 then
    ⟨-t, 0, 0, 0⟩
  else if i = 1 then
    burau_σ₁ t
  else if i = 2 then
    burau_σ₂ t
  else
    ⟨1, 0, 0, 1⟩

/-- Multiply two Burau matrices (2×2 carrier; full `(n-1)×(n-1)` extension is future work). -/
def burauMul (A B : Mat2 R) : Mat2 R := A * B

/-- Evaluate reduced Burau on a braid word (2-strand / 2×2 specialization for Phase 1). -/
def reducedBurauWord (t : R) (ws : BraidWord) : Mat2 R :=
  ws.foldl (fun M e =>
    let gen := burauGen 2 (e.2 + 1) t
    burauMul M gen) ⟨1, 0, 0, 1⟩

/-- Trace of 2×2 Burau matrix (Jones denominator ingredient for n = 2). -/
def burauTrace (A : Mat2 R) : R := A.a11 + A.a22

/-- `(n-1)×(n-1)` reduced Burau determinant scaffold.
    At `n = 2` this is `det` of the 1×1 block `[-t]`. -/
def reducedBurauDet (n : ℕ) (t : R) (ws : BraidWord) : R :=
  if n ≤ 2 then
    let M := reducedBurauWord t ws
    M.a11
  else
    det (reducedBurauWord t ws)

/-- Writhe correction exponent: `(-t)^{-3 · exponent_sum(ws)}` in standard normalization. -/
def writheExponent (ws : BraidWord) : ℤ :=
  -3 * exponent_sum ws

@[simp] theorem reducedBurauDet_empty (t : R) :
    reducedBurauDet 2 t [] = 1 := by
  simp [reducedBurauDet, reducedBurauWord]

@[simp] theorem reducedBurauDet_σ₁ (t : R) :
    reducedBurauDet 2 t [(1, 0)] = -t := by
  simp [reducedBurauDet, reducedBurauWord, burauGen, burau_σ₁, burauMul, mul_def]

end K22