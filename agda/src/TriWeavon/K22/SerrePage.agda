{-# OPTIONS --cubical --safe #-}

-- Serre spectral sequence page scaffolding (d_r via path algebra).
module TriWeavon.K22.SerrePage where

open import Cubical.Foundations.Prelude
open import Cubical.Foundations.Path
open import Cubical.HITs.Pushout
open import Cubical.Data.Nat using (ℕ; zero; suc)
open import Cubical.Data.Sigma using (Σ; _,_)

record PageCell : Type where
  field
    r : ℕ
    p : ℕ

record Differential (a b : PageCell) : Type where
  field
    witness : a ≡ b

pathInductionPage :
  (A : PageCell → Type) →
  (a : (c : PageCell) → A c) →
  (base : PageCell) →
  (p : base ≡ base) →
  A base
pathInductionPage A a base p = transport (λ i → A (p i)) (a base)

sracPageStep : PageCell → PageCell
sracPageStep c = record { PageCell.r = suc (PageCell.r c); PageCell.p = PageCell.p c }