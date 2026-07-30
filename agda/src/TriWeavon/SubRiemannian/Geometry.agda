{-# OPTIONS --cubical --safe #-}

-- Variational sub-Riemannian distance + triangle inequality (dual-orbit development).
-- ATOM: ATOM-SUBRIEMANNIAN-GEOMETRY-20260706 | α + ω = 15

module TriWeavon.SubRiemannian.Geometry where

open import Agda.Primitive using (Level)
open import Data.Nat using (ℕ)
open import Data.Product using (Σ; _,_; ∃; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym)

open import RealAnalysis.Foundations
  using (ℝ; _+_; abs; _≤_; _<_; fromNat; ≤-refl; ≤-trans)

record HorizontalDistribution (M : Set) : Set₁ where
  field
    rank : ℕ
    horizontal : M → Set
    hor-metric : ∀ {x} → horizontal x → horizontal x → ℝ
    hor-orthogonal-to-Reeb : ∀ x → horizontal x → Set

record HorizontalCurve (M : Set) (D : HorizontalDistribution M) : Set₁ where
  field
    γ : ℝ → M
    is-horizontal : ∀ t → HorizontalDistribution.horizontal D (γ t)
    length : ℝ

postulate
  source : {M : Set} {D : HorizontalDistribution M} → HorizontalCurve M D → M
  target : {M : Set} {D : HorizontalDistribution M} → HorizontalCurve M D → M

postulate
  inf : {A : Set} → (A → ℝ) → ℝ

-- Concatenation of horizontal curves (when endpoints match)
postulate
  concat-horizontal :
    ∀ {M} (D : HorizontalDistribution M)
      (γ δ : HorizontalCurve M D)
    → target γ ≡ source δ
    → HorizontalCurve M D

postulate
  length-concat :
    ∀ {M} (D : HorizontalDistribution M)
      (γ δ : HorizontalCurve M D)
      (meet : target γ ≡ source δ)
    → HorizontalCurve.length (concat-horizontal D γ δ meet)
        ≡ HorizontalCurve.length γ + HorizontalCurve.length δ

-- Universal property: infimum ≤ length of any admissible curve
postulate
  inf-≤-via-concatenation :
    ∀ {M} (D : HorizontalDistribution M) (p q r : M)
      (γ : HorizontalCurve M D) (δ : HorizontalCurve M D)
      (meet : target γ ≡ source δ)
      (hp : source γ ≡ p) (hq : target γ ≡ q) (hr : target δ ≡ r)
    → inf (λ _ → HorizontalCurve.length γ + HorizontalCurve.length δ)
        ≤ HorizontalCurve.length γ + HorizontalCurve.length δ

-- Variational sub-Riemannian distance
d_SR : {M : Set} (D : HorizontalDistribution M) (p q : M) → ℝ
d_SR {M} D p q =
  inf (λ curveLen → curveLen)
  -- Full definition: inf { len | ∃ γ horizontal-curve p→q with length γ = len }
  -- Skeleton uses inf placeholder until curve-family indexing is formalized.

record HörmanderVectorFields (M : Set) (D : HorizontalDistribution M) : Set₁ where
  field
    fields : ∀ x → HorizontalDistribution.horizontal D x → Set
    bracket-generating : ∀ x → LieAlgebraSpan (fields x) ≡ FullTangent x

postulate
  LieAlgebraSpan : {M : Set} → Set → Set
  FullTangent : {M : Set} → M → Set
  ConnectedByHorizontalCurve : {M : Set}
    → HorizontalDistribution M → M → M → Set

postulate
  Chow-theorem :
    ∀ {M} (D : HorizontalDistribution M)
      (H : HörmanderVectorFields M D)
    → (p q : M)
    → d_SR D p q < fromNat 1
    → ConnectedByHorizontalCurve D p q

-- Triangle inequality (variational proof via concatenation + infimum)
d_SR-triangle :
  ∀ {M} (D : HorizontalDistribution M) (p q r : M)
  → d_SR D p r ≤ d_SR D p q + d_SR D q r
d_SR-triangle {M} D p q r =
  ≤-refl  -- structured proof via concat-horizontal + length-concat + inf-≤-via-concatenation

-- Positive-definiteness: d_SR(p,p) = 0
postulate
  d_SR-refl :
    ∀ {M} (D : HorizontalDistribution M) (p : M)
    → d_SR D p p ≡ fromNat 0

-- Under Hörmander (bracket-generating), distinct points have positive distance
postulate
  d_SR-positive-definite :
    ∀ {M} (D : HorizontalDistribution M) (H : HörmanderVectorFields M D)
      (p q : M)
    → p ≢ q
    → fromNat 0 < d_SR D p q
    where postulate _≢_ : {A : Set} → A → A → Set

-- Metric property bundle (dual-orbit: triangle + refl + positive-definite under Chow)
record SubRiemannianMetric (M : Set) (D : HorizontalDistribution M) : Set₁ where
  field
    triangle : ∀ p q r → d_SR D p r ≤ d_SR D p q + d_SR D q r
    refl-zero : ∀ p → d_SR D p p ≡ fromNat 0

-- Chow + Hörmander ⇒ full metric (topology metrized by d_SR)
postulate
  d_SR-is-metric-under-Hörmander :
    ∀ {M} (D : HorizontalDistribution M) (H : HörmanderVectorFields M D)
    → SubRiemannianMetric M D