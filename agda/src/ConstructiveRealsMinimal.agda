{-# OPTIONS --cubical --safe #-}

module ConstructiveRealsMinimal where

open import Agda.Primitive using (Level)
open import Data.Product using (_,_; Σ)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong; sym; trans)
open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ; zero; suc)

postulate
  ℝ : Set
  _+_ : ℝ → ℝ → ℝ
  _-_ : ℝ → ℝ → ℝ
  _*_ : ℝ → ℝ → ℝ
  -_  : ℝ → ℝ
  0ℝ  : ℝ
  1ℝ  : ℝ
  abs : ℝ → ℝ
  _≤_ : ℝ → ℝ → Set
  _<_ : ℝ → ℝ → Set

postulate
  abs-triangle : ∀ x y → abs (x + y) ≤ abs x + abs y
  abs-hom      : ∀ x y → abs (x * y) ≡ abs x * abs y
  abs-neg      : ∀ x → abs (- x) ≡ abs x
  abs-nonneg   : ∀ x → 0ℝ ≤ abs x
  ≤-refl   : ∀ {x} → x ≤ x
  ≤-trans  : ∀ {x y z} → x ≤ y → y ≤ z → x ≤ z
  ≤-mono-+ : ∀ {a b c d} → a ≤ c → b ≤ d → a + b ≤ c + d
  abs-mono-* : ∀ k x y → abs (k * (x - y)) ≡ abs k * abs (x - y)

infixl 6 _+_
infixl 6 _-_
infixl 7 _*_
infix  4 _≤_
infix  4 _<_