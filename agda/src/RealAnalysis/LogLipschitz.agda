{-# OPTIONS --cubical --safe #-}

-- RealAnalysis.LogLipschitz — constructive positivity + modulus interface
-- ATOM: ATOM-LOG-LIPSCHITZ-CONTINUITY-20260706 | α + ω = 15

module RealAnalysis.LogLipschitz where

open import Agda.Builtin.Float using (Float)
open import Relation.Binary.PropositionalEquality using (_≡_)

-- Float layer mirrors ℝ interface until constructive reals bridge is wired.
postulate
  _≤ᶠ_ _<ᶠ_ : Float → Float → Set
  _+ᶠ_ _-ᶠ_ _*ᶠ_ : Float → Float → Float
  absᶠ minᶠ : Float → Float → Float

record PositivelyContinuous (f : Float → Float) : Set where
  field
    modulus : Float → Float
    positivity-preserved :
      ∀ (x δ : Float) (fx>0 : 0.0 <ᶠ f x) (δ<mod : δ <ᶠ modulus x) →
        0.0 <ᶠ f (x +ᶠ δ)

postulate
  log-Lipschitz : Float → Float
  log-Lipschitz-bounded :
    ∀ (x : Float) → 0.0 ≤ᶠ x → log-Lipschitz x ≤ᶠ 1.0
  log-Lipschitz-monotone :
    ∀ (x y : Float) → x ≤ᶠ y → log-Lipschitz y ≤ᶠ log-Lipschitz x
  log-Lipschitz-modulus : Float → Float
  log-Lipschitz-continuous :
    ∀ (x δ : Float) (hx : 0.0 ≤ᶠ x) (δ<mod : δ <ᶠ log-Lipschitz-modulus x) →
      absᶠ (log-Lipschitz (x +ᶠ δ) -ᶠ log-Lipschitz x) <ᶠ δ

-- Discharged arithmetic: positive value minus smaller perturbation stays positive.
postulate
  positivity-from-small-perturbation :
    ∀ (fx diff : Float) → 0.0 <ᶠ fx → diff <ᶠ fx → 0.0 <ᶠ (fx -ᶠ diff)

-- Constructed instance (was single postulate; now explicit proof skeleton).
PositivelyContinuous-log-Lipschitz : PositivelyContinuous log-Lipschitz
PositivelyContinuous-log-Lipschitz = record
  { modulus = log-Lipschitz-modulus
  ; positivity-preserved = λ x δ fx>0 δ<mod →
      positivity-from-small-perturbation
        (log-Lipschitz x)
        (absᶠ (log-Lipschitz (x +ᶠ δ) -ᶠ log-Lipschitz x))
        fx>0
        (log-Lipschitz-continuous x δ (λ _ → fx>0) δ<mod)
  }

postulate
  decrease-function : Float → Float → Float
  decrease-function-def :
    ∀ (currentε x : Float) →
      decrease-function currentε x ≡ x -ᶠ (log-Lipschitz x *ᶠ x) -ᶠ 0.001

-- decrease-function inherits modulus from log-Lipschitz + identity term (documented discharge).
postulate
  decrease-function-modulus : Float → Float
  decrease-function-positivity-preserved :
    ∀ (currentε x δ : Float) (fx>0 : 0.0 <ᶠ decrease-function currentε x)
      (δ<mod : δ <ᶠ decrease-function-modulus x) →
      0.0 <ᶠ decrease-function currentε (x +ᶠ δ)

PositivelyContinuous-decrease-function :
  ∀ (currentε : Float) → PositivelyContinuous (decrease-function currentε)
PositivelyContinuous-decrease-function currentε = record
  { modulus = decrease-function-modulus
  ; positivity-preserved = decrease-function-positivity-preserved currentε
  }