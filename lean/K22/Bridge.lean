import Mathlib.Tactic
import K22.Syntax
import K22.ExprMatch
import K22.Tactics
import K22.TomczakBridge
import K22.Jones

/-!
# K22Bridge — classifyK22Goal-driven k22_bridge

ATOM: ATOM-K22-BRIDGE-20260706 | α + ω = 15
-/

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch

elab "k22_bridge" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let tag := classifyK22Goal goalType goalStr

  match tag with
  | "jones" =>
      logInfo "k22_bridge: jones class — simp_exponent_sum + ring_matrix"
      if let some _ := tryExtractBurauWordFromGoal goalStr then
        logInfo "k22_bridge: braid word detected in goal"
      evalTactic (← `(tactic| ring_matrix))
      try evalTactic (← `(tactic| simp_exponent_sum)) catch _ => pure ()
  | "burau" =>
      logInfo "k22_bridge: burau class — ring_matrix + conditional simp_exponent_sum"
      evalTactic (← `(tactic| ring_matrix))
      if goalStr.containsSubstr "exponent_sum" || goalStr.containsSubstr "det_burau" then
        try evalTactic (← `(tactic| simp_exponent_sum)) catch _ => pure ()
      logInfo "k22_bridge: see det_burau / exponent_sum in K22.Syntax"
  | "tomczak" =>
      logInfo "k22_bridge: tomczak class — delegate to tomczak_bridge"
      evalTactic (← `(tactic| tomczak_bridge))
  | "serre" =>
      logWarning "k22_bridge: serre class — serre_scar_tactic not yet implemented"
  | _ =>
      logWarning m!"k22_bridge: unclassified goal ({goalStr})"

section Examples

variable {R : Type*} [CommRing R]
open Mat2

theorem burau_hexagon_k22_bridge (t : R) :
    (burau_σ₁ t * burau_σ₂ t) * burau_σ₁ t
      = (burau_σ₂ t * burau_σ₁ t) * burau_σ₂ t := by
  k22_bridge

theorem det_burau_σ₁_eq (t : R) : det_burau burau_σ₁ t = -t := by
  k22_bridge

theorem exponent_sum_example : exponent_sum [(1, 0), (-1, 1)] = 0 := by
  k22_bridge

end Examples

end K22