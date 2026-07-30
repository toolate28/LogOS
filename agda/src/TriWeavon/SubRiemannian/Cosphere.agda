{-# OPTIONS --cubical --safe #-}

module TriWeavon.SubRiemannian.Cosphere where

open import ConstructiveRealsMinimal
open import TriWeavon.SubRiemannian.Core
open import Data.Product using (_,_)

postulate
  RiemannianManifold : Set
  SStarM : RiemannianManifold → Set

variable
  M : RiemannianManifold

postulate
  HorizontalDistribution : RiemannianManifold → Distribution (SStarM M)
  HorizontalMetric : RiemannianManifold → MetricOnDistribution (HorizontalDistribution M)
  PoppVolumeOnCosphere : RiemannianManifold → VolumeForm (SStarM M)
  SubRiemannianRicciOnCosphere : RiemannianManifold → CurvatureTensor (SStarM M)
  HorizontalSubLaplacian : RiemannianManifold → DiffusionOperator (SStarM M)
  HorizontalHypoelliptic : {M : RiemannianManifold} → Hypoelliptic (HorizontalSubLaplacian M)
  HorizontalBracketGenerating :
    {M : RiemannianManifold} → BracketGenerating (HorizontalDistribution M)

  -- Base curvature data from Riemannian + NS coupling
  ricLower : RiemannianManifold → ℝ
  rmBound  : RiemannianManifold → ℝ
  baseRicLower : RiemannianManifold → ℝ
  riemannBound : RiemannianManifold → ℝ

CosphereSubRiemannian : SubRiemannianManifold (SStarM M)
CosphereSubRiemannian = record
  { distribution = HorizontalDistribution M
  ; metric       = HorizontalMetric M
  ; poppVolume   = PoppVolumeOnCosphere M
  ; subRicci     = SubRiemannianRicciOnCosphere M
  }

CosphereHorizontalGenerator : HorizontalGenerator (SStarM M) CosphereSubRiemannian
CosphereHorizontalGenerator = record
  { L                 = HorizontalSubLaplacian M
  ; hypoelliptic      = HorizontalHypoelliptic
  ; bracketGenerating = HorizontalBracketGenerating
  }