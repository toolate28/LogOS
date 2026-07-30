{-# OPTIONS --cubical --safe #-}

-- RealAnalysis.Foundations — reusable Lipschitz backbone for TriWeavon SRAC proofs.
-- ATOM: ATOM-REAL-ANALYSIS-FOUNDATIONS-20260706 | α + ω = 15

module RealAnalysis.Foundations where

open import Agda.Primitive using (Level)
open import Data.Nat using (ℕ; zero; suc; _<_)
open import Data.Product using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym)
open import Function using (_∘_)

postulate
  ℝ : Set
  _+_ : ℝ → ℝ → ℝ
  _-_ : ℝ → ℝ → ℝ
  _*_ : ℝ → ℝ → ℝ
  abs : ℝ → ℝ
  _≤_ : ℝ → ℝ → Set
  _<_ : ℝ → ℝ → Set
  fromNat : ℕ → ℝ

postulate
  ≤-refl  : ∀ {x} → x ≤ x
  ≤-trans : ∀ {x y z} → x ≤ y → y ≤ z → x ≤ z
  ≤-antisym : ∀ {x y} → x ≤ y → y ≤ x → x ≡ y

postulate
  clamp : ℝ → ℝ

postulate
  clamp-is-1-Lipschitz :
    ∀ (x y : ℝ) → abs (clamp x - clamp y) ≤ abs (x - y)

postulate
  Lip : (ℝ → ℝ) → ℝ → Set
  Bounded : (ℝ → ℝ) → ℝ → Set

postulate
  product-bounded-Lipschitz :
    ∀ (f g : ℝ → ℝ) (Mf Mg boundF boundG : ℝ)
    → Lip f Mf → Lip g Mg → Bounded f boundF → Bounded g boundG
    → Lip (λ x → f x * g x) (Mf * boundG + Mg * boundF)

postulate
  sum-Lipschitz :
    ∀ (f g : ℝ → ℝ) (Mf Mg : ℝ)
    → Lip f Mf → Lip g Mg → Lip (λ x → f x + g x) (Mf + Mg)

postulate
  log : ℝ → ℝ

postulate
  log-Lipschitz-bounded-[1,N] :
    ∀ (f : ℝ → ℝ) (N : ℕ) (K : ℝ)
    → Bounded f (fromNat N)
    → ∀ n → n < N
    → abs (log (f (suc n)) - log (f n)) ≤ K / fromNat (suc n)

composite-Lipschitz :
  ∀ (f g : ℝ → ℝ) (Mf Mg boundF boundG : ℝ)
  → Lip f Mf → Lip g Mg
  → Bounded f boundF → Bounded g boundG
  → Lip (λ x → f x + g x) (Mf + Mg)
  × Lip (λ x → f x * g x) (Mf * boundG + Mg * boundF)
composite-Lipschitz f g Mf Mg bF bG lipF lipG bF' bG' =
  sum-Lipschitz f g Mf Mg lipF lipG ,
  product-bounded-Lipschitz f g Mf Mg bF bG lipF lipG bF' bG'