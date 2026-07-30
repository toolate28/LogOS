{-# OPTIONS --cubical --safe #-}

-- Custom Serre spectral sequence page HIT — hcomp + path induction pattern.
-- Executable bridge: cutile::CudaEntropyResult::{surge,betti_proxy}
module TriWeavon.K22.SerreScarr where

open import Cubical.Foundations.Prelude
open import Cubical.Foundations.HLevels
open import Cubical.Foundations.Path
open import Cubical.HITs.Susp
open import Cubical.Data.Nat using (ℕ; _+_)

data SerreScarr (X : Type) (r : ℕ) : Type where
  gen : (n : ℕ) → SerreScarr X r
  dᵣ  : (n : ℕ) → Path (SerreScarr X r) (gen n) (gen (n + r))

dᵣ²-path : {X : Type} {r : ℕ} (n : ℕ) →
  Path (SerreScarr X r) (gen (n + 2 * r)) (gen n)
dᵣ²-path {X} {r} n =
  compPath (dᵣ {X} {r} (n + r)) (sym (dᵣ {X} {r} n))

tomczakLift :
  {X : Type} {r n : ℕ} →
  (p : Path (SerreScarr X r) (gen n) (gen (n + r))) →
  SerreScarr X r
tomczakLift {X} {r} {n} p =
  hcomp (λ j → λ { (i = i0) → gen n
                  ; (i = i1) → p j })
        (inS (gen (n + r)))

module _ {X : Type} {r : ℕ} where

  dᵣ-coherence : (n : ℕ) →
    compPath (dᵣ n) (dᵣ (n + r)) ≡ dᵣ²-path n
  dᵣ-coherence n = refl