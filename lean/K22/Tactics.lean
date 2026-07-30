import Mathlib.Tactic
import K22.Syntax
import K22.Tomczak
import K22.ExprMatch

/-!
# K22Tactics — ring_matrix, simp_exponent_sum, ring_matrix?

ATOM: ATOM-K22-TACTICS-20260706 | α + ω = 15
-/

namespace K22

open Lean Meta Elab Tactic
open Mat2
open K22.ExprMatch

macro "ring_matrix" : tactic => `(tactic|
  ext <;> simp [Mat2.mul_def] <;> ring
)

private def runSimpExponentSum : TacticM Unit := do
  evalTactic (← `(tactic|
    simp [exponent_sum, det_burau, det, det_def, det_burau_σ₁, det_burau_σ₂,
          burau_σ₁, burau_σ₂, Mat2.mul_def]
  ))

elab "simp_exponent_sum" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  if isBurauDetExponentGoal goalType goalStr then
    logInfo "simp_exponent_sum: K22 burau pattern; running simp"
    runSimpExponentSum
  else
    logInfo "simp_exponent_sum: no burau pattern; skipping"

/-- Dry-run: log whether `ring_matrix` would apply (AQoL-safe). -/
elab "ring_matrix?" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let tag := classifyK22Goal goalType goalStr
  if shouldUseRingMatrix goalType goalStr then
    logInfo m!"ring_matrix?: would apply (class={tag})"
  else
    logInfo m!"ring_matrix?: would skip (class={tag})"

end K22