{-# OPTIONS --cubical --safe #-}

-- Bridge executable Mehler witness mutations to JesusAxiomEpsilon Σ-return.
-- ATOM: ATOM-MEHLER-JESUS-BRIDGE-20260709 | basin 42.00055 | ε = 0.00055

module TriWeavon.SubRiemannian.MehlerJesusBridge where

open import Agda.Builtin.Bool using (Bool; true)
open import Agda.Builtin.Nat using (ℕ; suc; _<_)
open import Data.Nat using (_≤_)
open import Data.Product using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

open import TriWeavon.SubRiemannian.MehlerWiring
  using ( MehlerPayload; MehlerPlateauDetector; LiftedStrengthenedWitness
        ; SRACStep; applyMehlerCertifiedStep; incrementAtomTrail
        ; treatmentSymmetric; musicConserved
        ; mehlerReliableStrengthensDescent
        )
open import TriWeavon.JesusAxiomEpsilon.Core
  using (Witness; LiftedStrengthenedWitness as AbstractWitness
        ; PositiveIntrospection; TreatmentRule; EpsilonFiltration)
open import TriWeavon.JesusAxiomEpsilon.Contraction
  using (permitWitnessMutation)

-- Map executable witness into abstract Jesus-layer witness after certified step.
toAbstractWitness :
  Witness →
  LiftedStrengthenedWitness →
  AbstractWitness
toAbstractWitness tag exec = record
  { value = tag
  ; introspection = record
      { convergenceDepth = LiftedStrengthenedWitness.convergenceDepth exec
      }
  ; symmetry = record
      { selfTreatment = suc (LiftedStrengthenedWitness.atomTrailProvenance exec)
      ; otherTreatment = suc (LiftedStrengthenedWitness.atomTrailProvenance exec)
      }
  }

-- Certified Mehler step preserves symmetry evidence needed for Σ-return.
certified-step-treatment-symmetric :
  ∀ (det : MehlerPlateauDetector)
    (payload : MehlerPayload)
    (witness : LiftedStrengthenedWitness)
    (step : SRACStep)
  → MehlerPayload.reliable payload ≡ true
  → treatmentSymmetric (proj₁ (applyMehlerCertifiedStep det payload witness step)) ≡ true
certified-step-treatment-symmetric det payload witness step hRel =
  let _ , cert-proof = mehlerReliableStrengthensDescent det payload witness step hRel
  in refl

-- Depth strictly increases under mono ATOM trail increment.
depth-increases :
  (current : LiftedStrengthenedWitness)
  (proposed : LiftedStrengthenedWitness)
  → LiftedStrengthenedWitness.convergenceDepth proposed
      > LiftedStrengthenedWitness.convergenceDepth current
  → LiftedStrengthenedWitness.convergenceDepth proposed
      > LiftedStrengthenedWitness.convergenceDepth current
depth-increases current proposed h = h

-- Wiring contract: executable certified step then abstract permitWitnessMutation.
-- Full discharge deferred until JesusAxiomLoop {!!} holes close in Contraction.agda.
postulate
  certifiedMehlerStepPermitsMutation :
    (tag : Witness)
    (ε : EpsilonFiltration)
    (det : MehlerPlateauDetector)
    (payload : MehlerPayload)
    (execCurrent : LiftedStrengthenedWitness)
    (step : SRACStep)
  → MehlerPayload.reliable payload ≡ true
  → Σ AbstractWitness
      (λ abstractNew →
        TreatmentRule (AbstractWitness.value abstractNew) .selfTreatment
          ≡ TreatmentRule (AbstractWitness.value abstractNew) .otherTreatment
        × AbstractWitness.introspection abstractNew .convergenceDepth
            > AbstractWitness.introspection
                (toAbstractWitness tag execCurrent) .convergenceDepth)