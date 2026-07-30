import Lean
import Lean.Data.Json

/-!
# K22Log — structured JSON diagnostics for Lean → Rust/TS bridge

Emits `[K22-LOG] {json}` via logInfo + optional trace[K22.diagnostics].

ATOM: ATOM-K22-LOG-20260706 | α + ω = 15
-/

namespace K22

open Lean Elab Tactic Meta

/-- Structured K22 log entry (JSON-serializable). -/
structure K22Log where
  ts             : String
  level          : String := "INFO"
  classification : String
  action         : String
  message        : String
  success        : Bool
  details        : Option Json := none

def K22Log.toJson (l : K22Log) : Json :=
  Json.mkObj [
    ("timestamp",      Json.str l.ts),
    ("level",          Json.str l.level),
    ("classification", Json.str l.classification),
    ("action",         Json.str l.action),
    ("message",        Json.str l.message),
    ("success",        Json.bool l.success),
    ("details",        l.details.getD Json.null)
  ]

def emitK22Log (log : K22Log) : TacticM Unit := do
  let j := (log.toJson.compress)
  logInfo m!"[K22-LOG] {j}"
  trace[K22.diagnostics] "{log.message}"

initialize registerTraceClass `K22.diagnostics

def nowMs : TacticM String := do
  pure (toString (← IO.monoMsNow))

def goalPreview : TacticM String := do
  let goalType ← (← getMainGoal).getType
  pure ((← ppExpr goalType).pretty)

def mkLog (classification action message : String) (success : Bool)
    (level : String := "INFO") (details : Option Json := none) : TacticM K22Log := do
  pure {
    ts := (← nowMs)
    level := level
    classification := classification
    action := action
    message := message
    success := success
    details := details
  }

end K22