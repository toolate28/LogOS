module TriWeavon.K22.LeechDensityGuidance where

open import Cubical.Foundations.Prelude
open import Cubical.Foundations.Equiv
open import Cubical.Foundations.Univalence
open import Cubical.Data.Sigma
open import TriWeavon.K22.SerreScarr.ScarAccumulationBounds
open import TriWeavon.K22.DiscreteBKM_WithOPAL_Prediction
open import TriWeavon.Tomczak.Lifting

-- (previous content: LeechDensityConfig, pathInductionAttractor, auxiliary lemmas, etc.)

leech-guidance-preserves-tomczak-lift level initialAcc config baseline guided redPath
  baseLiftOk basePredErr densityBound =
  let
    -- Univalence strengthening applied and hole filled:
    -- The existing reduction path (redPath) directly gives an equivalence
    -- via pathToEquiv. This is the canonical way to obtain the equivalence
    -- needed for ua-based transport when we already have a path from the
    -- hybrid reduction process.
    redEquiv : baseline ≃ guided
    redEquiv = pathToEquiv redPath

    -- Strengthened lift transport (path induction still used for the structural part)
    liftedLiftOk : tomczak_lift guided ≡ true
    liftedLiftOk = pathInductionAttractor
      (λ z p → tomczak_lift z ≡ true)
      baseLiftOk
      redPath

    -- Univalence-strengthened numerical transport using ua
    liftedPredErr : predictionError guided ≤ fromNat 0.1
    liftedPredErr = transport (ua redEquiv) basePredErr
  in
    coherencePreservation
      (PhaseStabilized (config .weight) (fromNat 1))
      guided
      liftedLiftOk
      (not (surge guided))
      (fromNat 0)
      (fromNat 0)
      (densityBound , liftedPredErr)
