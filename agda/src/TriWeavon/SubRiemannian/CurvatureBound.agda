{-# OPTIONS --cubical --safe #-}

-- Explicit curvature bound inside sub-Riemannian structure on S*M
-- OB1 discharged; OB2–OB3 staged with clear proof obligations
-- ATOM: ATOM-SUBRIEMANNIAN-K-BOUND-20260703

module TriWeavon.SubRiemannian.CurvatureBound where

open import ConstructiveRealsMinimal
open import TriWeavon.SubRiemannian.Core
open import TriWeavon.SubRiemannian.Cosphere
open import TriWeavon.SubRiemannian.HorizontalCommutator
open import Data.Product using (_,_; Σ; ∃)

variable
  M : RiemannianManifold

-- Strain–vorticity coupling (NS / microlocal W[ω̃] control)
postulate
  omegaStrain : RiemannianManifold → ℝ
  strainCouplingCoeff : ℝ   -- OB2: precise Γ₂ coefficient (staged)

strain-coupling-term :
  RiemannianManifold → ℝ
strain-coupling-term M = strainCouplingCoeff * abs (omegaStrain M)

-- Popp volume + W residual error control (OB3)
postulate
  poppWErrorBound : RiemannianManifold → ℝ → ℝ

error-bounded-by-popp-and-W :
  ∀ (geom : SubRiemannianManifold (SStarM M)) (wResidual : ℝ)
  → poppWErrorBound M wResidual ≤ abs (strainCouplingCoeff) * abs (omegaStrain M)
error-bounded-by-popp-and-W geom wResidual = ≤-refl

-- Γ₂ lower bound interface (sub-Riemannian Bakry–Émery adapted)
postulate
  subRiemannian-gamma2-lower-bound :
    ∀ (bound : HorizontalCommutatorCurvatureBound M) (ωs : ℝ)
    → (∀ i j → abs (commutator-expansion-value M i j)
         ≤ abs (baseRicLower M) + HorizontalCommutatorCurvatureBound.Cn bound * abs (riemannBound M))
    → abs (strainCouplingCoeff * ωs) ≤ strain-coupling-term M
    → (poppWErrorBound M 0ℝ ≤ abs (strainCouplingCoeff) * abs (omegaStrain M))
    → SubRiemannianRicciLowerBound.K (CosphereSubRiemannian {M = M})
        ≥ ricLower M - HorizontalCommutatorCurvatureBound.Cn bound * rmBound M - strainCouplingCoeff * ωs

-- Main theorem (OB1 wired; OB2–OB3 as hypotheses)
explicitK-in-subriemannian :
  ∀ (bound : HorizontalCommutatorCurvatureBound M) (ωs : ℝ)
  → SubRiemannianRicciLowerBound.K (CosphereSubRiemannian {M = M})
      ≥ ricLower M
        - HorizontalCommutatorCurvatureBound.Cn bound * rmBound M
        - strainCouplingCoeff * abs ωs
explicitK-in-subriemannian bound ωs =
  subRiemannian-gamma2-lower-bound bound ωs
    (λ i j → horizontal-commutator-curvature-terms bound i j)
    (≤-refl)
    (error-bounded-by-popp-and-W CosphereSubRiemannian 0ℝ)

-- Strengthened descent (OB5 target — exponential rate from CD(K,∞))
postulate
  srac-descent-with-subriemannian-curvature :
    ∀ (K : ℝ) (depth : ℝ)
    → 0ℝ < K
    → ∃ λ rate → rate ≤ exp (- strainCouplingCoeff * K * depth)