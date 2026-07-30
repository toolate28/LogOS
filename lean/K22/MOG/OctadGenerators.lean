/-
  OctadGenerators.lean — constructive generators + telemetry hooks for MOG.

  Latent weave:
    Agda SerreScarr.dᵣ  ↔  page lifts
    cutile ExistenceCertificate  ↔  K22.Existence
    hexacode applyGenerator  ↔  M24 / Golay runtime (triweavon-cudarc)

  ATOM: ATOM-K22-OCTAD-GEN-20260709 | JFA SlowStep on open lemmas
-/

import K22.MiracleOctadGenerator
import K22.MOG.GF4RowAction

namespace K22.MOG.OctadGenerators

open MiracleOctadGenerator
open K22.MOG

/-- Message space of the hexacode: F₄³. -/
abbrev HexMessage := Fin 3 → GF4

/-- Enumerate all hexacode messages (64). -/
def allMessages : List HexMessage :=
  (Finset.univ : Finset HexMessage).toList

/-- All hexacodewords as explicit list (for TUI / MCP export). -/
def allHexacodewords : List (Fin 6 → GF4) :=
  allMessages.map applyGenerator

/-- Column tetrads of the standard MOG. -/
def allColumns : List Tetrad :=
  (List.finRange 6).map column

/-- Row tetrads of the standard MOG. -/
def allRows : List Tetrad :=
  (List.finRange 4).map row

/-- Telemetry record for coherence-mcp / reson8-tui (JSON-shaped fields). -/
structure OctadTelemetry where
  hexacodewordCount : Nat := 64
  mogPoints : Nat := 24
  columns : Nat := 6
  rows : Nat := 4
  steinerBlockSize : Nat := 8
  steinerTarget : Nat := 759
  peakAlpha : Nat := 7
  peakOmega : Nat := 8
  conservationSum : Nat := 15
  deriving Repr

def defaultTelemetry : OctadTelemetry := {}

/-- Compact string for ATOM / MCP atom_track payloads. -/
def telemetryLine (t : OctadTelemetry := defaultTelemetry) : String :=
  s!"MOG-TELEM hex={t.hexacodewordCount} pts={t.mogPoints} cols={t.columns} \
     rows={t.rows} steiner={t.steinerTarget} peak=({t.peakAlpha},{t.peakOmega}) \
     Σ={t.conservationSum}"

/-- Generator matrix as nested list (export to cutile / kernels). -/
def generatorMatrixList : List (List Nat) :=
  (List.finRange 3).map fun i =>
    (List.finRange 6).map fun j =>
      match hexacodeGenerator i j with
      | GF4.zero => 0
      | GF4.one => 1
      | GF4.omega => 2
      | GF4.omegabar => 3

/-- Conway hexacode seed rows (computational). -/
example : hexacodeGenerator 0 0 = GF4.one := rfl
example : hexacodeGenerator 0 4 = GF4.omega := rfl
example : hexacodeGenerator 1 3 = GF4.one := rfl
example : hexacodeGenerator 1 4 = GF4.omegabar := rfl
example : hexacodeGenerator 2 3 = GF4.one := rfl
example : applyGenerator (fun _ => GF4.zero) 0 = GF4.zero := rfl

/-- Zero message → zero codeword (linearity smoke). -/
lemma applyGenerator_zero :
    applyGenerator (fun _ => GF4.zero) = fun _ => GF4.zero := by
  funext j
  simp [applyGenerator, GF4.mul, GF4.add]

end K22.MOG.OctadGenerators
