module ContextualityGeometryInterface where

open import Level using (Level)
open import Data.Bool using (Bool; true; false)
open import Data.Nat using (ℕ; zero; suc)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)

-- Common interface for all contextuality geometries (Fano, Peres-Mermin, Mermin-Permutahedron)

record ContextualityGeometry (ℓ : Level) : Set (suc ℓ) where
  field
    -- Type of possible assignments for this geometry
    Assignment : Set ℓ

    -- Returns true if the assignment violates the geometry's obstruction rules
    hasObstruction : Assignment → Bool

    -- Returns 1 if obstructed, 0 otherwise (for runtime guards)
    violationIndex : Assignment → ℕ
    violationIndex a with hasObstruction a
    ... | true  = suc zero
    ... | false = zero

    -- Human-readable name
    name : String

-- Example: Instance for Peres-Mermin (stub)
-- In a full development, each geometry would provide its own record instance.

postulate
  PeresMerminGeometry : ∀ {ℓ} → ContextualityGeometry ℓ

-- Common BUMP Handoff Guard interface
BUMPHandoffGuard : ∀ {ℓ} → ContextualityGeometry ℓ → Set ℓ → Set ℓ
BUMPHandoffGuard G Assignment =
  (a : Assignment) → ContextualityGeometry.violationIndex G a ≡ suc zero → ⊥
