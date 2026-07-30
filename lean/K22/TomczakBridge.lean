import Mathlib.Tactic
import K22.Tomczak
import K22.ExprMatch
import K22.Tactics

/-!
# TomczakBridge — pattern-aware Tomczak lift automation

ATOM: ATOM-K22-TOMCZAK-BRIDGE-20260706 | α + ω = 15
-/

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch

elab "tomczak_bridge" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  if mentionsTomczakLift goalType goalStr then
    logInfo "tomczak_bridge: Tomczak lift / preservation goal detected."
    if shouldUseRingMatrix goalType goalStr then
      try evalTactic (← `(tactic| ring_matrix)) catch _ => pure ()
    try
      evalTactic (← `(tactic| simp [tomczakPreserved, tomczakLift]))
      logInfo "tomczak_bridge: Tomczak simp applied"
    catch _ =>
      logWarning "tomczak_bridge: Tomczak simp did not fully succeed"
  else
    logWarning m!"tomczak_bridge: not a Tomczak goal ({goalStr})"

end K22