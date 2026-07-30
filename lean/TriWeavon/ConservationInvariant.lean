/-!
# ConservationInvariant — α + ω = CONSERVATION_SUM

Formal skeleton for the D15 conservation **tag** used by:
- `notebooks/triweave-backends.ipynb` (Qiskit empirical check)
- `cutiles/cutile` R-matrix / WAVE stack
- Agda `TriWeavon.ConservationRMatrix`

Category C: `CONSERVATION_SUM = 15` is a labeled software constant / epistemic tag,
not a physical law and not a deploy gate. See CLAUDE.md and CLAUDECODE-INIT-v0_1.md.

ATOM: ATOM-CONSERVATION-LEAN-20260709 | cascade L4
-/

namespace TriWeavon.Conservation

structure WavePair where
  alpha : Nat
  omega : Nat
  deriving Repr, DecidableEq

/-- Matches Qiskit `CONSERVATION_SUM` and Universal Invariant. -/
def CONSERVATION_SUM : Nat := 15

def is_conserved (p : WavePair) : Prop :=
  p.alpha + p.omega = CONSERVATION_SUM

/-- Viviani peak resonance (α=7, ω=8). -/
def peak_resonance : WavePair := { alpha := 7, omega := 8 }

theorem peak_resonance_conserved : is_conserved peak_resonance := by
  rfl

/-- Dual rail: ω := 15 − α (caller must ensure α ≤ 15). -/
def dual (alpha : Nat) : WavePair :=
  { alpha := alpha, omega := CONSERVATION_SUM - alpha }

/-- Peak dual sanity: α=7 → ω=8. -/
theorem dual_peak : dual 7 = peak_resonance := by
  rfl

/-- List-level predicate used by verification receipts. -/
def all_conserved (pairs : List WavePair) : Prop :=
  ∀ p ∈ pairs, is_conserved p

theorem empty_conserved : all_conserved [] := by
  intro p hp
  cases hp

end TriWeavon.Conservation
