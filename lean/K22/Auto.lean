import Mathlib.Tactic
import K22.Syntax
import K22.Tomczak
import K22.ExprMatch
import K22.K22Log
import K22.Tactics
import K22.TomczakBridge
import K22.SerreScarTactic
import K22.Macros

/-!
# K22 Auto — k22_auto / k22_auto! daily development surface

Also exposes k22_weave / k22_cascade / k22_mog / k22_telemetrise via Macros.

ATOM: ATOM-K22-AUTO-20260706 | α + ω = 15
-/

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch

private def k22AutoImpl (verbose : Bool) : TacticM Unit := do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let cls := classifyK22Goal goalType goalStr
  let action := if verbose then "k22_auto!" else "k22_auto"
  let preview ← goalPreview
  let open ← mkLog cls action s!"dispatch class={cls}" true
    (details := some (Json.mkObj [("goal", Json.str preview)]))
  emitK22Log open
  if verbose then logInfo m!"k22_auto!: classification = {cls}"
  match cls with
  | "jones" =>
      try
        evalTactic (← `(tactic| ring_matrix))
        evalTactic (← `(tactic| simp_exponent_sum))
        let ok ← mkLog cls action "jones: ring_matrix + simp_exponent_sum" true
        emitK22Log ok
      catch _ =>
        let w ← mkLog cls action "jones automation partial" false "WARN"
        emitK22Log w
  | "burau" =>
      try
        evalTactic (← `(tactic| ring_matrix))
        let ok ← mkLog cls action "ring_matrix applied" true
        emitK22Log ok
      catch _ =>
        let w ← mkLog cls action "ring_matrix did not apply" false "WARN"
        emitK22Log w
        if verbose then logWarning "k22_auto: ring_matrix did not apply"
      if K22.ExprMatch.hasSubstr goalStr "exponent_sum"
          || K22.ExprMatch.hasSubstr goalStr "det_burau" then
        try
          evalTactic (← `(tactic| simp_exponent_sum))
          let ok ← mkLog cls action "simp_exponent_sum applied" true
          emitK22Log ok
        catch _ =>
          let w ← mkLog cls action "simp_exponent_sum skipped" false "WARN"
          emitK22Log w
  | "tomczak" =>
      try
        evalTactic (← `(tactic| tomczak_bridge))
        let ok ← mkLog cls action "tomczak_bridge applied" true
        emitK22Log ok
      catch _ =>
        let w ← mkLog cls action "tomczak_bridge partial" false "WARN"
        emitK22Log w
  | "serre" =>
      try
        evalTactic (← `(tactic| serre_scar_cascade))
        let ok ← mkLog cls action "serre_scar_cascade invoked" true
        emitK22Log ok
      catch _ =>
        try
          evalTactic (← `(tactic| serre_scar_tactic))
          let ok ← mkLog cls action "serre_scar_tactic fallback" true
          emitK22Log ok
        catch _ =>
          let w ← mkLog cls action "serre SlowStep" false "WARN"
          emitK22Log w
          if verbose then logWarning "k22_auto: serre cascade SlowStep"
  | "mog" =>
      try
        evalTactic (← `(tactic| k22_mog))
        let ok ← mkLog cls action "k22_mog invoked" true
        emitK22Log ok
      catch _ =>
        let w ← mkLog cls action "mog SlowStep" false "WARN"
        emitK22Log w
  | "existence" =>
      try
        evalTactic (← `(tactic| k22_cascade))
        let ok ← mkLog cls action "existence via k22_cascade" true
        emitK22Log ok
      catch _ =>
        let w ← mkLog cls action "existence SlowStep" false "WARN"
        emitK22Log w
  | _ =>
      if verbose then
        try evalTactic (← `(tactic| k22_telemetrise))
        catch _ => pure ()
      let w ← mkLog cls action "no K22 automation" true "INFO"
      emitK22Log w
      if verbose then logInfo "k22_auto: goal left unchanged (class=other)"

elab "k22_auto" : tactic => k22AutoImpl false
elab "k22_auto!" : tactic => k22AutoImpl true

end K22