{-# OPTIONS --cubical --safe #-}

-- Tomczak lifting predicate + path coherence.
-- Executable bridge: cutile::betti_tomczak_lift_check
module TriWeavon.Tomczak.Lifting where

open import Cubical.Foundations.Prelude
open import Cubical.Foundations.HLevels
open import Cubical.Foundations.Path
open import Cubical.Data.Nat using (ℕ)
open import Cubical.Data.Float using (Float)
open import Cubical.Data.Bool using (Bool; _∧_)

record TomczakLifting {ℓ} (X : Type ℓ) : Type ℓ where
  field
    preserved : X → Type ℓ
    stable    : (x : X) → isProp (preserved x)
    lift      : (x : X) → preserved x → X

record LiftGate : Type where
  field
    bettiProxy         : Float
    liftingThreshold   : Float
    tomczakPreserved   : Bool

open LiftGate

liftOk : LiftGate → Bool
liftOk g = (bettiProxy g) < (liftingThreshold g) ∧ (tomczakPreserved g)

hcompFiller : {X : Type} → (x : X) → x ≡ x → x ≡ x
hcompFiller _ p = p