{-# OPTIONS --cubical --safe #-}

-- RealAnalysis.ConstructiveBisection — positivity-preserving root finding scaffold
-- ATOM: ATOM-CONSTRUCTIVE-BISECTION-20260706

module RealAnalysis.ConstructiveBisection where

open import Agda.Builtin.Float using (Float)
open import Data.Product using (Σ; _,_)

open import RealAnalysis.LogLipschitz using (PositivelyContinuous)

postulate
  _<ᶠ_ : Float → Float → Set

-- Find δ > 0 with f(x+δ) still positive when f is positively continuous at x.
postulate
  constructive-bisection-positivity :
    ∀ (f : Float → Float) (pc : PositivelyContinuous f) (x : Float)
    → 0.0 <ᶠ f x
    → Σ Float (λ δ → 0.0 <ᶠ δ × 0.0 <ᶠ f (x + δ))

-- Interface for JesusAxiomLoop measure decrease (feeds Termination module).
postulate
  lemma-exists-decrease-point :
    ∀ (currentε x depth : Float)
    → Σ Float (λ δ → δ <ᶠ depth)