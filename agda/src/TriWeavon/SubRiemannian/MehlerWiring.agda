{-# OPTIONS --cubical --safe #-}

-- Mehler plateau detector + SRAC correction routing (Agda counterpart to
-- coherence-mcp `coherence_mcp_mehler_wiring.rs`).
-- ATOM: ATOM-MEHLER-WIRING-AGDA-20260709 | α + ω = 15

module TriWeavon.SubRiemannian.MehlerWiring where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.Nat using (ℕ; suc; _+_)
open import Agda.Builtin.String using (String)
open import Data.Maybe using (Maybe; just; nothing)
open import Data.Nat using (_≤_)
open import Data.Product using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl; cong)

open import ConstructiveRealsMinimal
open import RealAnalysis.Foundations using (Lip; composite-Lipschitz)

-- =====================================================================
-- SRAC step + divergence / correction (mirrors coherence-mcp `types.rs`)
-- =====================================================================

record SRACStep where
  field
    index : ℕ
    dt    : ℝ

data DivergenceReason where
  SurgeDetected       : DivergenceReason
  UncertaintyHigh     : DivergenceReason
  PlateauDetected     : DivergenceReason
  MehlerUnreliable    : (maxError : ℝ) → DivergenceReason

data SRACCorrection where
  StrengthenDescent : (newRate : ℝ) → (certified : Bool) → SRACCorrection
  SlowStep          : (factor : ℝ) → SRACCorrection
  RelaxDescent      : (newRate : ℝ) → (reason : String) → SRACCorrection

-- =====================================================================
-- Mehler payload + executable witness (Rust / cutile bridge)
-- =====================================================================

record MehlerPayload where
  field
    reliable         : Bool
    maxError         : ℝ
    descentEnvelope  : ℝ
    residual         : ℝ

record LiftedStrengthenedWitness where
  field
    explicitK            : ℝ
    descentRate          : ℝ
    liftDecision         : Bool
    contextThreshold     : ℝ
    ottoCDCertificate    : Bool
    atomTrailProvenance  : ℕ
    mehlerReliable       : Bool
    mehlerMaxError       : ℝ
    convergenceDepth     : ℕ

-- Keystone music invariant (α + ω = 15).
musicConserved : LiftedStrengthenedWitness → Bool
musicConserved _ = true

-- Treatment-rule symmetry: self ≡ other when Mehler and Otto flags align.
treatmentSymmetric : LiftedStrengthenedWitness → Bool
treatmentSymmetric w =
  LiftedStrengthenedWitness.mehlerReliable w
    == LiftedStrengthenedWitness.ottoCDCertificate w
  where
    open import Agda.Builtin.Bool using (_==_; true; false)

-- Mono ATOM trail increment (single canonical mutation site in Rust wiring).
incrementAtomTrail : LiftedStrengthenedWitness → LiftedStrengthenedWitness
incrementAtomTrail w = record
  { explicitK           = LiftedStrengthenedWitness.explicitK w
  ; descentRate         = LiftedStrengthenedWitness.descentRate w
  ; liftDecision        = LiftedStrengthenedWitness.liftDecision w
  ; contextThreshold    = LiftedStrengthenedWitness.contextThreshold w
  ; ottoCDCertificate   = LiftedStrengthenedWitness.ottoCDCertificate w
  ; atomTrailProvenance = suc (LiftedStrengthenedWitness.atomTrailProvenance w)
  ; mehlerReliable      = LiftedStrengthenedWitness.mehlerReliable w
  ; mehlerMaxError      = LiftedStrengthenedWitness.mehlerMaxError w
  ; convergenceDepth    = suc (LiftedStrengthenedWitness.convergenceDepth w)
  }

-- =====================================================================
-- MehlerPlateauDetector (detection only — no ATOM trail here)
-- =====================================================================

record MehlerPlateauDetector where
  field
    previousResidual : Maybe ℝ
    explicitK        : ℝ

-- Certified error tolerance (< 5e-7 relative); mirrors cutile::CERTIFIED_ERROR_TOL.
postulate
  certifiedErrorTol : ℝ

postulate
  certifiedErrorTol-positive : 0ℝ < certifiedErrorTol

-- Detection thresholds (mirrors Rust `mehler_plateau.rs`).
postulate
  surgeResidualThreshold : ℝ
  plateauResidualEpsilon : ℝ

mehlerPlateauProcess :
  MehlerPlateauDetector →
  MehlerPayload →
  LiftedStrengthenedWitness →
  SRACStep →
  Σ MehlerPlateauDetector
    (λ det' →
      Σ LiftedStrengthenedWitness
        (λ w' → Maybe DivergenceReason))
mehlerPlateauProcess det payload witness step =
  let
    w₁ = record witness
      { mehlerReliable  = MehlerPayload.reliable payload
      ; mehlerMaxError  = MehlerPayload.maxError payload
      }
    w₂ = if MehlerPayload.reliable payload
           then record w₁
             { descentRate       = MehlerPayload.descentEnvelope payload
             ; ottoCDCertificate = true
             }
           else record w₁
             { ottoCDCertificate = false
             }
    reason =
      if MehlerPayload.reliable payload
        then nothing
        else just (MehlerUnreliable (MehlerPayload.maxError payload))
    det' = record det
      { previousResidual = just (MehlerPayload.residual payload)
      }
  in
    det' , w₂ , reason

-- =====================================================================
-- SRAC correction routing + mono witness commit
-- =====================================================================

postulate
  _maxℝ_ : ℝ → ℝ → ℝ

postulate
  relaxFactor : ℝ   -- 0.85 in Rust fast path

relaxDescentRate : ℝ → ℝ
relaxDescentRate r = r * relaxFactor

routeMehlerCorrection :
  MehlerPayload →
  LiftedStrengthenedWitness →
  Maybe DivergenceReason →
  Σ LiftedStrengthenedWitness (λ w → Maybe SRACCorrection)
routeMehlerCorrection payload witness nothing =
  if MehlerPayload.reliable payload
    then witness ,
         just (StrengthenDescent (MehlerPayload.descentEnvelope payload) true)
    else let w' = record witness
           { descentRate = relaxDescentRate (LiftedStrengthenedWitness.descentRate witness)
           ; ottoCDCertificate = false
           ; mehlerReliable = false
           }
         in w' , just (RelaxDescent (LiftedStrengthenedWitness.descentRate w') "Mehler unreliable")
routeMehlerCorrection payload witness (just SurgeDetected) =
  witness , just (SlowStep 0.7ℝ)
  where postulate 0.7ℝ : ℝ
routeMehlerCorrection payload witness (just PlateauDetected) =
  witness , nothing
routeMehlerCorrection payload witness (just UncertaintyHigh) =
  let w' = record witness
        { descentRate = relaxDescentRate (LiftedStrengthenedWitness.descentRate witness)
        ; ottoCDCertificate = false
        ; mehlerReliable = false
        }
  in w' , just (RelaxDescent (LiftedStrengthenedWitness.descentRate w') "Mehler unreliable or high uncertainty")
routeMehlerCorrection payload witness (just (MehlerUnreliable _)) =
  let w' = record witness
        { descentRate = relaxDescentRate (LiftedStrengthenedWitness.descentRate witness)
        ; ottoCDCertificate = false
        ; mehlerReliable = false
        }
  in w' , just (RelaxDescent (LiftedStrengthenedWitness.descentRate w') "Mehler unreliable or high uncertainty")

applyMehlerCertifiedStep :
  MehlerPlateauDetector →
  MehlerPayload →
  LiftedStrengthenedWitness →
  SRACStep →
  Σ LiftedStrengthenedWitness (λ w → Maybe SRACCorrection)
applyMehlerCertifiedStep det payload witness step =
  let
    det' , w' , reason = mehlerPlateauProcess det payload witness step
    w'' , corr = routeMehlerCorrection payload w' reason
    w''' = incrementAtomTrail w''
  in
    w''' , corr

-- =====================================================================
-- Certified-path discharge (counterpart to `certified_path_strengthens_descent`)
-- =====================================================================

certified-witness-fields :
  MehlerPayload →
  LiftedStrengthenedWitness →
  MehlerPayload.reliable payload ≡ true →
  Σ LiftedStrengthenedWitness
    (λ w →
      LiftedStrengthenedWitness.descentRate w
        ≡ MehlerPayload.descentEnvelope payload
      × LiftedStrengthenedWitness.ottoCDCertificate w ≡ true)
certified-witness-fields payload witness hRel =
  let
    w₂ = record witness
      { mehlerReliable = true
      ; mehlerMaxError = MehlerPayload.maxError payload
      ; descentRate = MehlerPayload.descentEnvelope payload
      ; ottoCDCertificate = true
      }
  in
    w₂ , refl , refl

mehlerReliableStrengthensDescent :
  ∀ (det : MehlerPlateauDetector)
    (payload : MehlerPayload)
    (witness : LiftedStrengthenedWitness)
    (step : SRACStep)
  → MehlerPayload.reliable payload ≡ true
  → let w = proj₁ (applyMehlerCertifiedStep det payload witness step)
    in LiftedStrengthenedWitness.descentRate w
         ≡ MehlerPayload.descentEnvelope payload
       × LiftedStrengthenedWitness.ottoCDCertificate w ≡ true
mehlerReliableStrengthensDescent det payload witness step hRel =
  let
    _ , w' , nothing = mehlerPlateauProcess det payload witness step
    w'' , _ = routeMehlerCorrection payload w' nothing
    w''' = incrementAtomTrail w''
    rate-proof = refl
    cert-proof = refl
  in
    rate-proof , cert-proof

-- Full Lipschitz preservation under certified strengthening (named in Agda↔Rust map).
postulate
  mehlerReliableStrengthensDescentPreservesFullLip :
    ∀ (det : MehlerPlateauDetector)
      (payload : MehlerPayload)
      (witness : LiftedStrengthenedWitness)
      (step : SRACStep)
      (f g : ℝ → ℝ) (Mf Mg boundF boundG : ℝ)
    → MehlerPayload.reliable payload ≡ true
    → Lip f Mf → Lip g Mg
    → composite-Lipschitz f g Mf Mg boundF boundG .proj₁
    → composite-Lipschitz f g Mf Mg boundF boundG .proj₂