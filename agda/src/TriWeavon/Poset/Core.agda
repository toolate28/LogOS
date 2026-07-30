{-# OPTIONS --cubical --safe #-}

-- Minimal poset carrier for cohomological bundle / admissible-split induction.
-- ATOM: VANISHING-HIGHER-BASE-COHOMOLOGY-20260721-sm100
-- Source bridge: Hurmuzov, "A Cohomological Bundle Theory for Sheaf Cohomology"
-- α + ω = 15 is Category C label only (not a load-bearing gate).

module TriWeavon.Poset.Core where

open import Cubical.Foundations.Prelude
open import Cubical.Data.Nat
open import Cubical.Data.Bool
open import Cubical.Data.Sigma
open import Cubical.Data.Empty as ⊥
open import Cubical.Data.Unit

------------------------------------------------------------------------
-- Abstract finite poset (skeleton — enough for inductive vanishing)
------------------------------------------------------------------------

record Poset : Type₁ where
  field
    Carrier : Type
    _≤_     : Carrier → Carrier → Type
    ≤-refl  : ∀ x → x ≤ x
    ≤-trans : ∀ {x y z} → x ≤ y → y ≤ z → x ≤ z
    ≤-antisym : ∀ {x y} → x ≤ y → y ≤ x → x ≡ y

open Poset public

-- Boolean lattice of rank 1: two elements with bottom < top (order complex = one edge).
data Bool1 : Type where
  bot : Bool1
  top : Bool1

_≤B1_ : Bool1 → Bool1 → Type
bot ≤B1 bot = Unit
bot ≤B1 top = Unit
top ≤B1 bot = ⊥.⊥
top ≤B1 top = Unit

Bool1-Poset : Poset
Bool1-Poset = record
  { Carrier   = Bool1
  ; _≤_       = _≤B1_
  ; ≤-refl    = λ { bot → tt ; top → tt }
  ; ≤-trans   = λ { {bot} {bot} {z} _ q → q
                  ; {bot} {top} {top} _ _ → tt
                  ; {top} {top} {top} _ _ → tt
                  ; {top} {bot} {_} p _ → ⊥.rec p
                  ; {bot} {top} {bot} _ q → ⊥.rec q
                  ; {top} {top} {bot} _ q → ⊥.rec q }
  ; ≤-antisym = λ { {bot} {bot} _ _ → refl
                  ; {top} {top} _ _ → refl
                  ; {bot} {top} _ q → ⊥.rec q
                  ; {top} {bot} p _ → ⊥.rec p }
  }

-- Coefficient "sheaf" as a family over the carrier (local system skeleton).
Sheaf : Poset → Type₁
Sheaf P = Carrier P → Type

-- Degree index for sheaf cohomology (placeholder groupoid).
-- Category B until Cech / derived-functor realization is wired.
postulate
  SheafCohomology : (P : Poset) → Sheaf P → ℕ → Type
