/-
  Macros.lean — advanced K22 proof macros (Serre · Tomczak · Existence · MOG).

  Design (HUP / FPA / JFA):
  · Position: local simp / native_decide / ring paths
  · Momentum: page cascade + conservation + WAVE floor
  · SlowStep: never fail the session hard — emit [K22-LOG] and leave goal

  ATOM: ATOM-K22-MACROS-20260709
-/

import Mathlib.Tactic
import K22.Tomczak
import K22.Existence
import K22.ExprMatch
import K22.K22Log
import K22.Tactics
import K22.TomczakBridge
import K22.SerreScarTactic
import K22.MiracleOctadGenerator

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch
open K22.Existence

/-- Serre page index for cascade (E₂..E∞ mapped as 2..5). -/
structure PageState where
  page : Nat := 2
  waveFloor : Float := 0.85
  jfaSlowStep : Bool := false
  deriving Repr

def pageGate (p : Nat) : Float :=
  match p with
  | 2 => 0.85
  | 3 => 0.92
  | 4 => 0.9998
  | _ => 0.9998

/-- Emit a structured page-cascade log for MCP / TUI. -/
def emitPageCascade (ps : PageState) (msg : String) (ok : Bool) : TacticM Unit := do
  let details := Json.mkObj [
    ("page", Json.num ps.page),
    ("waveFloor", Json.str s!"{ps.waveFloor}"),
    ("jfaSlowStep", Json.bool ps.jfaSlowStep),
    ("gate", Json.str s!"{pageGate ps.page}"),
    ("surface", Json.str "serre_cascade")
  ]
  let log ← mkLog "serre" "page_cascade" msg ok
    (level := if ok then "INFO" else "WARN")
    (details := some details)
  emitK22Log log

/-- k22_cascade: attempt tomczak → serre → existence simp in one SlowStep chain. -/
elab "k22_cascade" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let cls := classifyK22Goal goalType goalStr
  let open ← mkLog cls "k22_cascade" s!"begin cascade class={cls}" true
  emitK22Log open
  -- Stage A: Tomczak
  try
    evalTactic (← `(tactic| tomczak_bridge))
    emitPageCascade { page := 2 } "tomczak stage" true
  catch _ =>
    emitPageCascade { page := 2, jfaSlowStep := true } "tomczak SlowStep" false
  -- Stage B: Serre with diagnostics
  try
    evalTactic (← `(tactic| serre_scar_tactic!))
    emitPageCascade { page := 3 } "serre stage" true
  catch _ =>
    emitPageCascade { page := 3, jfaSlowStep := true } "serre SlowStep" false
  -- Stage C: existence / conservation simp
  try
    evalTactic (← `(tactic| simp [preservesExistence, alphaOmegaOk, contextFromCertificate,
      tomczakLift, tomczakPreserved]))
    emitPageCascade { page := 4 } "existence simp" true
  catch _ =>
    emitPageCascade { page := 4, jfaSlowStep := true } "existence SlowStep" false

/-- k22_peak: discharge α+ω peak-style goals via native_decide when possible. -/
elab "k22_peak" : tactic => do
  try
    evalTactic (← `(tactic| first
      | native_decide
      | simp [alphaOmegaOk]
      | rfl))
    let ok ← mkLog "conservation" "k22_peak" "peak discharged" true
    emitK22Log ok
  catch _ =>
    let w ← mkLog "conservation" "k22_peak" "SlowStep — peak not closed" false "WARN"
    emitK22Log w

/-- k22_mog: classify MOG/octad goals and run lightweight simp. -/
elab "k22_mog" : tactic => do
  let goalStr := (← goalPreview)
  let isMog := K22.ExprMatch.hasSubstr goalStr "MOG"
    || K22.ExprMatch.hasSubstr goalStr "isMOGOctad"
    || K22.ExprMatch.hasSubstr goalStr "hexacode"
    || K22.ExprMatch.hasSubstr goalStr "Octad"
  if !isMog then
    let w ← mkLog "mog" "k22_mog" "not an MOG goal" false "WARN"
    emitK22Log w
    return
  try
    evalTactic (← `(tactic| simp [isMOGOctad, isEvenTypeOctad, isOddTypeOctad,
      isHexacodeword, applyGenerator, columnSymbol]))
    let ok ← mkLog "mog" "k22_mog" "MOG simp applied" true
    emitK22Log ok
  catch _ =>
    let w ← mkLog "mog" "k22_mog" "SlowStep — MOG lemmas pending" false "WARN"
    emitK22Log w

/-- k22_telemetrise: emit full goal classification JSON for LogOS TUI ingest. -/
elab "k22_telemetrise" : tactic => do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let cls := classifyK22Goal goalType goalStr
  let preview := if goalStr.length > 240 then goalStr.extract 0 ⟨240⟩ ++ "…" else goalStr
  let details := Json.mkObj [
    ("class", Json.str cls),
    ("goal", Json.str preview),
    ("alpha", Json.num 7),
    ("omega", Json.num 8),
    ("sum", Json.num 15),
    ("sink", Json.str "logos-telemetry"),
    ("mcp", Json.str "atom_track|wave_coherence_check"),
    ("tui", Json.str "reson8-tui:Braid|Logs")
  ]
  let log ← mkLog cls "k22_telemetrise" "telemetry snapshot" true (details := some details)
  emitK22Log log
  logInfo m!"k22_telemetrise: class={cls} → [K22-LOG] for MCP/TUI"

/-- Composite: telemetrise then cascade (daily driver). -/
elab "k22_weave" : tactic => do
  evalTactic (← `(tactic| k22_telemetrise))
  evalTactic (← `(tactic| k22_cascade))

end K22
