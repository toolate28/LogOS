/-!
# L3: No-shrinker — The Clay-grade open problem

There exists no non-trivial self-similar shrinking solution to the
Navier-Stokes equations in 3D.

This is equivalent to showing that the only critical point of W
on the space of self-similar profiles is trivial (u ≡ 0).

If L3 is proved, combined with L1+L2+L4, it yields:
  H^1 data → W well-defined → W monotone → no blow-up profiles
  → regularity → Millennium Prize.

Epistemic status: ⟦L3⟧ — CLAY-GRADE OPEN.
This lemma is the mathematical bottleneck. It is NOT expected to
be provable with current techniques. Its role in the pipeline is
as a formally stated open problem that DNS evidence can test
empirically (via Case A/B monitoring).

The DeepMind PINN catalog (arXiv:2509.14185) found UNSTABLE
singularity families (λ₂ ≈ 0.4703), which is evidence FOR L3
but not a proof.

Reference: arXiv:2601.08854v3, §3.2 (Perelman shrinking soliton analogy)
-/

import Ns.Monotone

/-- A self-similar shrinking profile for NS. -/
axiom ShrinkingProfile : Type

/-- The trivial (zero) profile. -/
axiom trivial_profile : ShrinkingProfile

/-- L3: No non-trivial shrinker exists.
    THIS IS THE OPEN PROBLEM. -/
axiom no_shrinker :
  ∀ (σ : ShrinkingProfile), σ = trivial_profile

-- ⚠️ THIS AXIOM IS AN OPEN CONJECTURE.
-- It is stated here as a formal target, not as a proven fact.
-- The `axiom` keyword means Lean accepts it without proof.
-- Replacing this with `theorem` requires solving the Millennium Problem.
