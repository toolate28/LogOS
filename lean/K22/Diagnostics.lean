import Mathlib.Tactic
import K22.Syntax
import K22.Tomczak
import K22.ExprMatch
import K22.K22Log
import K22.Tactics

/-!
# K22 Diagnostics — k22_simp? with structured K22Log output

ATOM: ATOM-K22-DIAGNOSTICS-20260706 | α + ω = 15
-/

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch

elab "k22_simp?" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let classification := classifyK22Goal goalType goalStr
  let preview ← goalPreview
  let log ← mkLog classification "k22_simp?" s!"Classification = {classification}" true
    (details := some (Json.mkObj [("goal", Json.str preview)]))
  emitK22Log log
  logInfo m!"k22_simp?: classification = {classification}"
  match classification with
  | "burau" =>
      try
        evalTactic (← `(tactic| ring_matrix))
        emitK22Log (← mkLog classification "k22_simp?" "ring_matrix applied" true)
      catch _ => emitK22Log (← mkLog classification "k22_simp?" "ring_matrix did not apply" false "WARN")
      if goalStr.containsSubstr "exponent_sum" || goalStr.containsSubstr "det_burau" then
        try
          evalTactic (← `(tactic| simp_exponent_sum))
          emitK22Log (← mkLog classification "k22_simp?" "simp_exponent_sum applied" true)
        catch _ => emitK22Log (← mkLog classification "k22_simp?" "simp_exponent_sum skipped" false "WARN")
  | "tomczak" =>
      try
        evalTactic (← `(tactic| simp [tomczakPreserved, tomczakLift]))
        emitK22Log (← mkLog classification "k22_simp?" "Tomczak simp applied" true)
      catch _ => emitK22Log (← mkLog classification "k22_simp?" "Tomczak simp partial" false "WARN")
  | "serre" =>
      logInfo "k22_simp?: serre — use serre_scar_tactic or k22_auto"
  | _ =>
      logInfo "k22_simp?: no K22 pattern — goal unchanged"

end K22