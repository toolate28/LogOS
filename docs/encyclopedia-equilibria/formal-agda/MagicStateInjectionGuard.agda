module MagicStateInjectionGuard where

open import Cubical.Foundations.Prelude
open import Cubical.Data.Bool
open import Cubical.Data.Empty

open import SerreScarrPathInduction-KS-Bounds
open import MerminPermutahedron-KS-Bounds

-- ============================================================
-- Magic State Injection Guard
-- Checks KS / Mermin obstruction before allowing injection
-- ============================================================

record MagicStateInjectionGuard : Type where
  field
    -- Which contextuality geometry to check
    geometry : ContextualityGeometry   -- Fano | PeresMermin | MerminPermutahedron

    -- Pre-injection check: must have a contextuality violation
    -- (i.e., non-contextual simulation is impossible)
    preInjectionCheck : (f : NonContextualAssignment) → Type
    preInjectionCheck f = contextualityViolationIndex f ≡ 1

    -- Post-injection verification (optional)
    postInjectionVerified : Bool

-- Default guard using Mermin-Permutahedron (currently strongest)
defaultMagicStateInjectionGuard : MagicStateInjectionGuard
defaultMagicStateInjectionGuard = record
  { geometry = MerminPermutahedron
  ; preInjectionCheck = λ f → contextualityViolationIndexPM f ≡ 1
  ; postInjectionVerified = true
  }

-- Combined BUMP + Magic State Injection Guard
BUMPMagicStateGuard : NonContextualAssignment → Type
BUMPMagicStateGuard f =
  BUMPHandoffGuard f × MagicStateInjectionGuard.preInjectionCheck defaultMagicStateInjectionGuard f

-- Lemma: Injection is only allowed when a contextuality obstruction is present
injectionOnlyWhenObstructed :
  (f : NonContextualAssignment) →
  BUMPMagicStateGuard f →
  contextualityViolationIndex f ≡ 1
injectionOnlyWhenObstructed f guard = snd guard

-- ============================================================
-- Post-Injection Verification
-- ============================================================

record PostInjectionVerification : Type where
  field
    logicalGateApplied : Bool
    syndromeWeightBelowThreshold : Bool
    magicStateConsumed : Bool
    overallSuccess : Bool

postInjectionCheck : PostInjectionVerification → Type
postInjectionCheck v = v .overallSuccess ≡ true

-- Integration test hook (to be called from Rust/Kani harness)
postInjectionTest :
  (f : NonContextualAssignment) →
  BUMPMagicStateGuard f →
  PostInjectionVerification →
  Type
postInjectionTest f guard verification =
  injectionOnlyWhenObstructed f guard × postInjectionCheck verification

