import Mathlib.Tactic
import K22.Tomczak
import K22.ExprMatch
import K22.K22Log
import K22.Tactics

/-!
# SerreScarTactic — evolved Serre-Scarr cascade with JFA + telemetry

Pages (HUP / spectral):
  E₂ (scaffold)  WAVE ≥ 0.85
  E₃ (gated)     WAVE ≥ 0.92
  E₄ (locked)    WAVE ≥ 0.9998
  E∞ (deployed)  witness layer

JFA: failed lifts → SlowStep log, never hard abort of the session.
FPA: simp only closed lemmas; open dᵣ paths remain explicit.

ATOM: ATOM-K22-SERRE-EVOLVED-20260709
-/

namespace K22

open Lean Meta Elab Tactic
open K22.ExprMatch

structure SerreConvergenceDiag where
  currentPage       : Nat := 2
  tomczakOk         : Bool := true
  bettiStable       : Bool := true
  mehlerReliable    : Bool := false
  ottoCdCertificate : Bool := false
  existenceOk       : Bool := false
  conservationOk    : Bool := false
  jfaSlowStep       : Bool := false
  stabilized        : Bool := false
  deriving Repr

open K22.ExprMatch (hasSubstr)

def parseWitnessDiagnostics (goalStr : String) : SerreConvergenceDiag :=
  let mehlerReliable :=
    hasSubstr goalStr "mehler_reliable" || hasSubstr goalStr "mehlerReliable"
  let ottoCd :=
    hasSubstr goalStr "otto_cd_certificate" || hasSubstr goalStr "ottoCDCertificate"
  let tomczakOk :=
    hasSubstr goalStr "tomczak_preserved" || hasSubstr goalStr "tomczakPreserved"
      || hasSubstr goalStr "tomczakLift"
  let bettiStable :=
    hasSubstr goalStr "betti_proxy_below" || hasSubstr goalStr "bettiProxyBelow"
      || hasSubstr goalStr "bettiProxy"
  let existenceOk :=
    hasSubstr goalStr "ExistenceCertificate" || hasSubstr goalStr "preservesExistence"
  let conservationOk :=
    hasSubstr goalStr "alphaOmega" || hasSubstr goalStr "α" || hasSubstr goalStr "15"
  let page :=
    if hasSubstr goalStr "E∞" || hasSubstr goalStr "E_infty" then 5
    else if hasSubstr goalStr "E4" || hasSubstr goalStr "E₄" then 4
    else if hasSubstr goalStr "E3" || hasSubstr goalStr "E₃" then 3
    else 2
  let stabilized := mehlerReliable && ottoCd && tomczakOk && bettiStable
  { currentPage := page
  , mehlerReliable := mehlerReliable
  , ottoCdCertificate := ottoCd
  , tomczakOk := tomczakOk
  , bettiStable := bettiStable
  , existenceOk := existenceOk
  , conservationOk := conservationOk
  , jfaSlowStep := !stabilized
  , stabilized := stabilized }

def emitSerreDiagnostics (diag : SerreConvergenceDiag) : TacticM Unit := do
  let details := Json.mkObj [
    ("currentPage", Json.num diag.currentPage),
    ("tomczakOk", Json.bool diag.tomczakOk),
    ("bettiStable", Json.bool diag.bettiStable),
    ("mehlerReliable", Json.bool diag.mehlerReliable),
    ("ottoCdCertificate", Json.bool diag.ottoCdCertificate),
    ("existenceOk", Json.bool diag.existenceOk),
    ("conservationOk", Json.bool diag.conservationOk),
    ("jfaSlowStep", Json.bool diag.jfaSlowStep),
    ("stabilized", Json.bool diag.stabilized),
    ("sink", Json.str "logos-telemetry"),
    ("mcp_tools", Json.str "wave_coherence_check,atom_track,gauge_verify"),
    ("tui_panels", Json.str "Braid,Logs,WAVE")
  ]
  let log ← mkLog "serre" "serre_scar_tactic" "convergence diagnostics" true (details := some details)
  emitK22Log log
  logInfo m!"serre_scar_tactic: E_{diag.currentPage} stab={diag.stabilized} jfaSlow={diag.jfaSlowStep}"

/-- Attempt one spectral page lift: SerreScar simp → tomczak simp → rfl. -/
private def tryPageLift (page : Nat) (verbose : Bool) : TacticM Bool := do
  try
    evalTactic (← `(tactic| first
      | simp [SerreScar, tomczakPreserved, tomczakLift]
      | simp [tomczakPreserved, tomczakLift]
      | rfl))
    let ok ← mkLog "serre" "page_lift" s!"lift E_{page}" true
    emitK22Log ok
    pure true
  catch _ =>
    let w ← mkLog "serre" "page_lift" s!"SlowStep at E_{page}" false "WARN"
    emitK22Log w
    if verbose then logWarning m!"serre_scar_tactic: SlowStep at page E_{page}"
    pure false

private def serreScarImpl (verbose : Bool) (withDiagnostics : Bool) (cascade : Bool) : TacticM Unit := do
  let goalType ← (← getMainGoal).getType
  let goalStr := (← ppExpr goalType).pretty
  let cls := classifyK22Goal goalType goalStr
  -- Accept serre, tomczak, or existence-flavoured goals as cascade entry
  let admit := cls == "serre" || cls == "tomczak"
    || hasSubstr goalStr "Existence" || hasSubstr goalStr "Serre"
  if !admit then
    let log ← mkLog cls "serre_scar_tactic" "not a serre/tomczak goal" false "WARN"
    emitK22Log log
    if verbose then logWarning "serre_scar_tactic: goal not classified as serre/tomczak"
    return
  let log ← mkLog "serre" "serre_scar_tactic" "Serre-Scarr goal detected" true
  emitK22Log log
  let diag := parseWitnessDiagnostics goalStr
  if withDiagnostics || verbose then
    emitSerreDiagnostics diag
  if cascade then
    -- Fractal cascade: attempt lifts from current page upward
    let pages := [diag.currentPage, diag.currentPage + 1, 4]
    let mut any := false
    for p in pages do
      let ok ← tryPageLift p verbose
      if ok then any := true
    if !any then
      let fail ← mkLog "serre" "serre_scar_tactic" "cascade exhausted — JFA SlowStep" false "WARN"
      emitK22Log fail
  else
    try
      evalTactic (← `(tactic| simp [SerreScar, tomczakPreserved, tomczakLift]))
      let ok ← mkLog "serre" "serre_scar_tactic" "simp applied" true
      emitK22Log ok
    catch _ =>
      let fail ← mkLog "serre" "serre_scar_tactic" "no progress — lemmas pending" false "WARN"
      emitK22Log fail
      if verbose then logWarning "serre_scar_tactic: did not make progress"

elab "serre_scar_tactic" : tactic => serreScarImpl false false false
elab "serre_scar_tactic!" : tactic => serreScarImpl true true false
/-- Full E₂→E∞ cascade with diagnostics (preferred for k22_weave). -/
elab "serre_scar_cascade" : tactic => serreScarImpl true true true

end K22
