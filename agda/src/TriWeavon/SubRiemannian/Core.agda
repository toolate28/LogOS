{-# OPTIONS --cubical --safe #-}

-- Sub-Riemannian structure on S*M — common interface (Agda constructive layer)
-- ATOM: ATOM-SUBRIEMANNIAN-CORE-20260703 | α + ω = 15

module TriWeavon.SubRiemannian.Core where

open import ConstructiveRealsMinimal
open import Data.Product using (_,_; Σ; ∃)
open import Data.Nat using (ℕ)

-- Schematic differential-geometric carriers (full cubical development deferred)
postulate
  Manifold : Set → Set
  Distribution : {M : Set} → Manifold M → Set
  MetricOnDistribution : {M : Set} → {H : Distribution M} → Set
  VolumeForm : {M : Set} → Manifold M → Set
  CurvatureTensor : {M : Set} → Manifold M → Set
  DiffusionOperator : {M : Set} → Manifold M → Set
  Hypoelliptic : {M : Set} → DiffusionOperator M → Set
  BracketGenerating : {M : Set} → Distribution M → Set
  CD : ℝ → {M : Set} → Set → Set   -- Bakry–Émery CD(K,∞)

record SubRiemannianManifold (M : Set) : Set₁ where
  field
    distribution : Distribution M
    metric       : MetricOnDistribution distribution
    poppVolume   : VolumeForm M
    subRicci     : CurvatureTensor M

record HorizontalGenerator (M : Set) (SR : SubRiemannianManifold M) : Set₁ where
  field
    L                 : DiffusionOperator M
    hypoelliptic      : Hypoelliptic L
    bracketGenerating : BracketGenerating (SubRiemannianManifold.distribution SR)

record SubRiemannianRicciLowerBound (M : Set) (SR : SubRiemannianManifold M) : Set₁ where
  field
    K  : ℝ
    cd : CD K (SubRiemannianManifold M)