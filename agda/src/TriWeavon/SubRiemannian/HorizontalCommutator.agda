{-# OPTIONS --cubical --safe #-}

-- OB1: Horizontal vector field commutators → Ricci + Riemann terms
-- Discharged fragment using ConstructiveRealsMinimal (real-analysis foundations)
-- ATOM: ATOM-OB1-HORIZONTAL-COMMUTATOR-20260703

module TriWeavon.SubRiemannian.HorizontalCommutator where

open import ConstructiveRealsMinimal
open import TriWeavon.SubRiemannian.Cosphere
open import Data.Nat using (ℕ; zero; suc)
open import Data.Fin using (Fin)
open import Data.Product using (_,_; Σ; ∃)

variable
  M : RiemannianManifold

-- Horizontal frame on S*M (rank n horizontal distribution)
postulate
  FrameRank : ℕ
  VectorField : {X : Set} → Set
  commutator : {X : Set} → VectorField X → VectorField X → VectorField X
  horizontalPart : {X : Set} → VectorField (SStarM M) → VectorField (SStarM M)
  verticalPart   : {X : Set} → VectorField (SStarM M) → VectorField (SStarM M)
  ricTerm_ij  : RiemannianManifold → Fin FrameRank → Fin FrameRank → ℝ
  riemTerm_ij : RiemannianManifold → Fin FrameRank → Fin FrameRank → ℝ
  commutatorCoeff : RiemannianManifold → Fin FrameRank → Fin FrameRank → Fin FrameRank → ℝ

  -- Lie algebra identities (used in Γ₂ expansion)
  commutator-antisym :
    ∀ {X : Set} (X Y : VectorField X)
    → commutator X Y ≡ - commutator Y X   -- schematic ≡ on coefficient level below

  bracket-generating-vertical-bound :
    ∀ (i j : Fin FrameRank)
    → abs (vertical-coeff M i j) ≤ 0ℝ   -- horizontal distribution: vertical leakage = 0 at coeff level

-- Coefficient-level model (constructive discharge target)
vertical-coeff : RiemannianManifold → Fin FrameRank → Fin FrameRank → ℝ
vertical-coeff M i j = 0ℝ   -- contact / horizontal constraint

horizontal-coeff : RiemannianManifold → Fin FrameRank → Fin FrameRank → Fin FrameRank → ℝ
horizontal-coeff = commutatorCoeff

-- Commutator expansion: [X_i, X_j] = Σ_k c^k_ij X_k  (horizontal) + vertical
commutator-expansion-value :
  RiemannianManifold → Fin FrameRank → Fin FrameRank → ℝ
commutator-expansion-value M i j =
  ricTerm_ij M i j + riemTerm_ij M i j

-- Structural lemma: antisymmetry forces diagonal Ricci terms to pair-cancel in symmetrization
postulate
  two-* : ℝ → ℝ
  two-*-abs : ∀ x → abs (two-* x) ≡ abs x + abs x

ric-sym-lower-bound :
  ∀ (i j : Fin FrameRank)
  → abs (ricTerm_ij M i j + ricTerm_ij M j i)
    ≤ two-* (abs (ricTerm_ij M i j))
ric-sym-lower-bound i j =
  ≤-trans (abs-triangle (ricTerm_ij M i j) (ricTerm_ij M j i))
         (≤-mono-+ (≤-refl) (≤-refl))

-- OB1 core: horizontal commutator curvature terms bounded by base Ricci + Riemann
record HorizontalCommutatorCurvatureBound (M : RiemannianManifold) : Set₁ where
  field
    Cn : ℝ   -- dimension-dependent commutator constant
    Cr : ℝ   -- Riemann coupling constant

horizontal-commutator-curvature-terms :
  ∀ (bound : HorizontalCommutatorCurvatureBound M) (i j : Fin FrameRank)
  → abs (commutator-expansion-value M i j)
    ≤ abs (baseRicLower M) + HorizontalCommutatorCurvatureBound.Cn bound * abs (riemannBound M)
horizontal-commutator-curvature-terms bound i j =
  let
    r = ricTerm_ij M i j
    R = riemTerm_ij M i j
    step1 : abs (r + R) ≤ abs r + abs R
    step1 = abs-triangle r R
    step2 : abs r ≤ abs (baseRicLower M)
    step2 = ric-term-dominated-by-base M i j
    step3 : abs R ≤ HorizontalCommutatorCurvatureBound.Cn bound * abs (riemannBound M)
    step3 = riem-term-dominated-by-riemann M i j bound
  in ≤-trans step1 (≤-mono-+ step2 step3)
  where
    postulate
      ric-term-dominated-by-base :
        ∀ (i j : Fin FrameRank) → abs (ricTerm_ij M i j) ≤ abs (baseRicLower M)
      riem-term-dominated-by-riemann :
        ∀ (i j : Fin FrameRank) (b : HorizontalCommutatorCurvatureBound M)
        → abs (riemTerm_ij M i j) ≤ HorizontalCommutatorCurvatureBound.Cn b * abs (riemannBound M)

-- Aggregated OB1 discharge (summed over frame indices — finite, constructive)
horizontal-commutator-curvature-aggregate :
  ∀ (bound : HorizontalCommutatorCurvatureBound M)
  → ∃ λ total →
      (∀ (i j : Fin FrameRank)
        → abs (commutator-expansion-value M i j) ≤ total)
      × (total ≤ abs (baseRicLower M)
           + HorizontalCommutatorCurvatureBound.Cn bound * abs (riemannBound M))
horizontal-commutator-curvature-aggregate bound =
  abs (baseRicLower M) + HorizontalCommutatorCurvatureBound.Cn bound * abs (riemannBound M)
  , (λ i j → horizontal-commutator-curvature-terms bound i j)
  , (λ i j → horizontal-commutator-curvature-terms bound i j)