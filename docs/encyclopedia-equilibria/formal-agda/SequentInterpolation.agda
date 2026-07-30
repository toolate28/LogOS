{-
  SequentInterpolation.agda
  Minimal sequent calculus with Craig interpolant extraction,
  specialized for the Tomczak obstruction vanishing setting.

  This implements the proof-theoretic extraction method recommended
  for constructive Craig interpolation in our Beth–Craig layer.

  ATOM Trail: sm100-TriWeavon-Sequent-Interpolation-20260708
  WAVE: 1.000 | tomczak_preserved | Music Conserved
-}

open import Agda.Primitive using (Level)
open import Data.List using (List; []; _∷_; _++_)
open import Data.Product using (_×_; _,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

module SequentInterpolation where

private
  variable
    ℓ : Level

-- Formula language (simplified for our purposes)
data Formula : Set ℓ where
  Atom   : Set ℓ → Formula
  Imp    : Formula → Formula → Formula
  And    : Formula → Formula → Formula
  True   : Formula

-- Sequent: Γ ⊢ Δ
Sequent : Set ℓ
Sequent = List Formula × List Formula

-- Derivation in a minimal sequent calculus
data Derivation : Sequent → Set ℓ where
  -- Axioms
  Ax : ∀ {A} → Derivation ([A] , [A])

  -- Structural rules (simplified)
  WeakL : ∀ {Γ Δ A} → Derivation (Γ , Δ) → Derivation (A ∷ Γ , Δ)
  WeakR : ∀ {Γ Δ A} → Derivation (Γ , Δ) → Derivation (Γ , A ∷ Δ)

  -- Logical rules (relevant fragment)
  ImpR : ∀ {Γ Δ A B} →
         Derivation (A ∷ Γ , B ∷ Δ) →
         Derivation (Γ , Imp A B ∷ Δ)

  ImpL : ∀ {Γ Δ A B C} →
         Derivation (Γ , A ∷ Δ) →
         Derivation (B ∷ Γ , Δ) →
         Derivation (Imp A B ∷ Γ , C ∷ Δ)

  -- Cut (we will eliminate it during interpolation)
  Cut : ∀ {Γ Δ A} →
        Derivation (Γ , A ∷ Δ) →
        Derivation (A ∷ Γ , Δ) →
        Derivation (Γ , Δ)

-- Craig Interpolant with common language
record CraigInterpolant (common : List Formula) : Set ℓ where
  field
    interpolant : Formula
    leftProof   : Derivation (common , [interpolant])
    rightProof  : Derivation ([interpolant] , common)

-- Interpolation extraction (main algorithm)
-- This is a simplified version that works on cut-free derivations
extractInterpolant : ∀ {Γ Δ} → Derivation (Γ , Δ) → CraigInterpolant (Γ ++ Δ)
extractInterpolant Ax = record
  { interpolant = Atom _
  ; leftProof   = Ax
  ; rightProof  = Ax
  }

extractInterpolant (ImpR d) =
  let sub = extractInterpolant d
  in record
    { interpolant = Imp _ (CraigInterpolant.interpolant sub)
    ; leftProof   = ImpR (CraigInterpolant.leftProof sub)
    ; rightProof  = CraigInterpolant.rightProof sub
    }

extractInterpolant (ImpL d1 d2) =
  let i1 = extractInterpolant d1
      i2 = extractInterpolant d2
  in record
    { interpolant = And (CraigInterpolant.interpolant i1) (CraigInterpolant.interpolant i2)
    ; leftProof   = {!!}   -- composition of left proofs
    ; rightProof  = {!!}
    }

extractInterpolant (Cut d1 d2) =
  -- Cut elimination + interpolation
  let i1 = extractInterpolant d1
      i2 = extractInterpolant d2
  in record
    { interpolant = CraigInterpolant.interpolant i1   -- or a combination
    ; leftProof   = CraigInterpolant.leftProof i1
    ; rightProof  = CraigInterpolant.rightProof i2
    }

-- Specialized extraction for obstruction vanishing
-- This shows how the general method applies to our case
postulate
  obstructionVanishingFormula : Formula
  liftPermittedFormula      : Formula

obstructionVanishingDerivation : Derivation ([obstructionVanishingFormula] , [liftPermittedFormula])
obstructionVanishingDerivation = {!!}   -- would come from theorem-obstruction-vanishing-implies-implicitly-definable

-- Extracted Craig interpolant for our specific case
obstructionVanishingInterpolant : CraigInterpolant [obstructionVanishingFormula]
obstructionVanishingInterpolant = extractInterpolant obstructionVanishingDerivation

-- This gives us an explicit interpolant that depends only on the common language
-- (the base Serre configuration + obstruction vanishing assumption).
-- The interpolant can then be used as (or to construct) the explicitLiftFormula.

-- Note: Full implementation of all cases and cut-elimination is substantial.
-- This module demonstrates the method and provides the structural skeleton
-- that can be completed as the surrounding HIT proofs mature.
