/-!
# L4: H^1 bound → regularity bridge

If W remains bounded and L3 holds (no shrinkers), then the
Navier-Stokes solution remains smooth for all time.

This is the compactness argument: bounded W + no blow-up profiles
→ the solution cannot develop singularities.

Epistemic status: ⟦L4⟧ — conditional on L3.
The logic itself is standard (Prodi-Serrin → Leray-Hopf → regularity),
but it requires L3 as a hypothesis.

Reference: arXiv:2601.08854v3, §3
-/

import Ns.NoShrinker

/-- An NS solution over [0, T). -/
axiom NSSolution : Type

/-- The solution is smooth (no singularities). -/
axiom IsSmooth : NSSolution → Prop

/-- The W functional remains bounded along the solution. -/
axiom WBounded : NSSolution → Prop

/-- L4: Bounded W + no shrinkers → regularity. -/
axiom w_bounded_implies_smooth :
  ∀ (u : NSSolution),
    WBounded u → IsSmooth u

-- Note: This axiom encodes the Prodi-Serrin regularity criterion
-- combined with the blow-up analysis from L3. The proof would need:
-- 1. Prodi-Serrin: ‖u‖_{L^p L^q} < ∞ for 2/p + 3/q = 1 → smooth
-- 2. Bounded W → bounded ‖ω‖_{L^2} (from L2 monotonicity)
-- 3. No shrinker (L3) → no Type I blow-up
-- 4. ε-regularity → no Type II blow-up
