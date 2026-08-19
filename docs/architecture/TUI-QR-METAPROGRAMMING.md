# Ratatui / Tokio Metaprogramming × Quantum-Redstone Circuits

```
kind:     BUILD-DIRECTIVE
about:    reson8-tui event loop · phase_evolution · reactiveness
level:    document
branch:   Grok / Pulse · crates/tui
α+ω:      15
status:   Track A hygiene live (v0.2.2 HITL+git) · residual-zero observe only
gate:     residual-zero · none (Track A hygiene permitted; no Track B promotion)
```

**ATOM:** `ATOM-GROK-TUI-QR-META-20260806` · audit `ATOM-CLAUDE-REASON-QDI-DRAIN-AUDIT-20260807`  
**Code surface:** `crates/tui/src/qr_meta.rs` · `main.rs` · `phase_evolution.rs` · `codes/` · `lattice.rs`  
**Lean twin:** `lean/TriWeavon/QuantumRedstone.lean` · `lean/TriWeavon/LatticeLayers.lean`  
**Tagline:** *Where the last thing you've done becomes the first thing you need*  
**Package:** `reson8-tui` **0.2.2** · HITL `ATOM-GROK-TUI-HITL-GATE-20260818`

---

## 0. Why this map exists

`reson8-tui` already runs a **quantum-redstone × SPHINX** phase table (`H → CNOT → RS-NOR → ε`)
on a Tokio runtime with Ratatui frames. Development velocity stalls when every panel, key,
and bus is hand-wired as ad-hoc `match` arms.

This document defines **metaprogramming principles** so that:

1. New UI surfaces compile from **circuit declarations**, not copy-pasted event loops.  
2. Reactiveness follows **QDI-style async** (quasi-delay-insensitive): no blocked render, no silent drops without a latch.  
3. The isomorphism to quantum-redstone / Minecraft teaching circuits stays **load-bearing**, not decorative.

---

## 1. Master isomorphism (one table)

| Quantum-redstone | SPHINX | TaskPhase edge | Tokio / Ratatui construct | Metaprogramming law |
|------------------|--------|----------------|---------------------------|---------------------|
| **Hadamard (H)** | KENL | Pending → Initialized | `tokio::select!` / multi-source poll | **Superpose** inputs; never collapse early |
| **CNOT** | AWI | Initialized → Executing | pure `App` reducer + key/cmd intent | **Entangle** intent with state; no side-effects in reduce |
| **RS-NOR latch** | ATOM | Executing → Validated | latched fields, debounce, popup hold | **Hold** one bit of truth; idempotent re-draw |
| **ε-Tetrahedron** | SAIF | Validated → Completed → seed | fixed-tick frame + `last_done → first_need` | **Resonate** then regenerate |

Conservation on every edge: **α + ω = 15** (Category C tag, not a reject gate).

**Lattice activate (2026-08-15):** key `A` is `CircuitIntent::RefreshLattice` —
H-superpose a filesystem probe of apps/cutiles/crates/kernels/ops, CNOT-reduce
into `App.lattice`, RS-NOR latch the `lat n/5` strip, ε notify. Shell twin
`logos-activate`. HOPE/quantum-redstone mcfunctions teach the same four gates;
do not copy those trees into LogOS.

---

## 2. Circuit vocabulary → async UI

### 2.1 Dust (wires) = channels

| Redstone | Tokio | Rule |
|----------|-------|------|
| Dust line | `mpsc::channel` | Bounded; drop policy explicit |
| Repeater | `broadcast` / fan-out clone | One source, many panels |
| Comparator | `watch::Receiver` | Shared latched snapshot |
| Pulse torch | `oneshot` | Single-shot handshake |
| Observer | `terminal.draw` | Measurement collapses framebuffer only |

**Principle P-DUST:** Never share `App` across tasks. Share **wires**. Tasks emit dust; the main loop latches into `App`.

### 2.2 QDI (quasi-delay-insensitive) loop

Classical Minecraft redstone is delay-sensitive; QDI circuits tolerate unknown latency.

```text
loop {
  drain all ready dust     // H — superpose sources
  reduce intents           // CNOT — pure entangle
  latch decisions          // RS-NOR — hold
  draw frame               // ε — measure / resonate
  poll keys with budget    // never block forever
}
```

**Principle P-QDI:** `try_recv` / short `poll` only. Long work is a **gate tick** on a worker wire, not a blocking call in the draw path.

### 2.3 Measurement = render

Ratatui `draw` is **projective measurement**: read-only on latched state. Mutating `App` inside a widget builder is a **category error** (axiom leakage into the fiber).

**Principle P-MEASURE:** Widgets are pure functions `(&App) → Widget`. All mutation happens pre-draw in the CNOT/RS-NOR stages.

---

## 3. Four gate laws (metaprogramming)

### G1 · Hadamard — open the superposition (`select!` / drain)

```rust
// Concept (see qr_meta::drain_dust!)
// Superpose: bridge | superskill | lsp | keys | timer
```

- Add a new event source by **declaring a wire**, not by nesting another `while` in five places.  
- Prefer `qr_meta::CircuitBus` registration over open-coding channels in `main`.  
- KENL/H is knowledge: unknown readiness → stay open; do not invent green status.

### G2 · CNOT — entangle intent with structure (reducer)

```text
intent (control qubit) ──CNOT──► state (target qubit)
```

- Keys, bridge events, LSP diagnostics are **controls**.  
- `App` fields are **targets**.  
- Reducer is pure: `(App, Intent) → (App, Effects)`.  
- Effects (spawn task, write file) are **ε-phase only**, scheduled after latch.

**Principle P-CNOT:** No `engine.handle(...).await` inside a widget. Emit `Effect::EngineHandle(ev)` from reduce; main runs effects after latch.

### G3 · RS-NOR — latch one bit (memory without race)

RS-NOR holds S/R without oscillation if inputs are clean.

| Latch field | Example |
|-------------|---------|
| Focus | `FocusPanel` |
| Layout | `LayoutKind` |
| Sequence | `SequenceEngine` |
| Popup | `Option<UrgentPopup>` (blocking = set line high) |
| Notif TTL | tick-down counters |

**Principle P-LATCH:** Overlays (help, popup) are **set-dominant latches**: while high, they mask lower dust (key routing). Document the priority ring once; generate match arms with macros.

### G4 · ε-Tetrahedron — resonate and regenerate

- Target cadence: operator-feel ~**42–100 Hz** poll / draw budget (ε motif at 42.00055 Hz is symbolic Category C; actual poll may be 10 ms).  
- On SAIF complete: `last_done → first_need` (tagline).  
- Metaprogramming: sequences are **declared tables**, not ad-hoc `for _ in 0..4 { gate_tick() }` forever.

**Principle P-EPSILON:** Every completed cycle **must** write a seed string. Empty seed is a dead spiral.

---

## 4. Layout as circuit boards

| LayoutKind | Board metaphor | Primary gates visible |
|------------|----------------|------------------------|
| `ops` | Full table | All panels |
| `formal` | Observer eye | Formal + support dust |
| `agent` | Control room | Providers · logs · braid · formal |
| `monitor` | Scope | Braid · logs · tests |
| `quantum` | QR×SPHINX board | Phases primary |
| `minimal` | Two-wire | Braid + logs |

**Principle P-BOARD:** `HostSurface::default_layout` is the **default power rail**. Env pin (`RESON8_LAYOUT`) is a **lever**. Detection is Category B heuristic — never residual-zero.

---

## 5. Declarative metaprogramming surface

Code lives in `crates/tui/src/qr_meta.rs`:

| Macro / type | Gate | Purpose |
|--------------|------|---------|
| `qr_gate_row!` | table | Compile phase-evolution rows consistently |
| `drain_dust!` | H | Generate multi-source try_recv drains |
| `latch_priority!` | RS-NOR | Ordered overlay routing (popup > help > normal) |
| `intent_map!` | CNOT | KeyCode → Intent enum arms |
| `CircuitEvent` | dust | Typed bus messages |
| `Effect` | ε | Post-latch work (spawn, notify, engine) |

### 5.1 Target end-state main loop (sketch)

```rust
while app.running {
    // H — superpose
    drain_dust!(app, bridge, ss_ev_rx, lsp_rx);

    // CNOT + RS-NOR — already applied inside handle_* reducers
    app.tick_notifications();

    // ε — measure
    terminal.draw(|f| ui::draw(f, &app))?;

    // H — key dust with timeout (QDI)
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(k) = event::read()? {
            app.apply_key(k); // generated from intent_map! + latch_priority!
        }
    }
}
```

Migration is **incremental**: macros wrap existing handlers first; pure `Intent` enum is phase 2.

---

## 6. Reactiveness checklist (fail-closed)

| Check | Redstone analog | Fail mode |
|-------|-----------------|-----------|
| Draw never awaits network | No dust through lamp | Frozen TUI |
| Channels bounded | Dust limit | OOM / lag |
| Popup masks keys | RS-NOR set line | Ghost keystrokes |
| LSP on worker | Observer on separate wire | Draw stalls |
| Diagnostics amber for sorry | Comparator < threshold | False green |
| Sequence seed written | ε regenerate | Dead spiral |
| α+ω logged on pipeline | Conservation tag | Category C only — do not block |

---

## 7. Minecraft / FreeCAD library ids (stable)

Keep names aligned with the gate library (tests in `phase_evolution.rs`):

| Enum | id |
|------|-----|
| Hadamard | `Hadamard_Gate` |
| Cnot | `CNOT_Gate` |
| RsNorLatch | `RS_NOR_Latch` |
| EpsilonTetrahedron | `Epsilon_Tetrahedron` |

`mc-bridge` and Museum-of-Computation models should use the **same ids** when teaching the isomorphism.

---

## 8. Acceleration playbook (how to ship faster)

1. **New panel** → add `FocusPanel` variant + layout board slot + pure widget; no new bus unless needed.  
2. **New async source** → new `CircuitEvent` variant + one drain arm (H), one reduce arm (CNOT).  
3. **New key** → one line in `intent_map!` (or match), never deep in `draw`.  
4. **New sequence** → `SequenceKind` + table row roles; reuse `SequenceEngine::tick`.  
5. **Never** open residual-zero / Track B from the TUI; TUI is Track A reactiveness + operator eye.

---

## 9. Relationship to residual-zero MCP

| Surface | Role |
|---------|------|
| `logos-residual-zero` MCP | Gate status / dual-kernel **read** tools |
| `reson8-tui` | Operator **observe + drive** sequences |
| This doc | How TUI code is **structured** so both stay honest |

TUI must not display dual-kernel PASS while Discussion #47 NOVIKOV remains broken without Category labels.

---

## 10. Implementation status

| Item | Status |
|------|--------|
| Phase table H×CNOT×RS-NOR×ε | **Live** (`phase_evolution.rs`) |
| Host surface → board default | **Live** (`surface.rs`) |
| Macro / CircuitEvent layer | **Live** (`qr_meta.rs`) |
| Main loop H→CNOT→RS-NOR→ε | **Live** (`main.rs` + `App::apply_intent`) |
| **Bounded `drain_dust!`** | **Live** — `DRAIN_BUDGET=32` per source/frame (QDI ack window) |
| **`engine.handle` off H** | **Live** — queue on `App`, ε after draw, `ENGINE_HANDLE_BUDGET=16` |
| Residual \(R\) Tier-1 eye | **Live** — Braid panel; Category C; amber lab claim ≠ deploy-green |
| Codes lab (Hex·G24·RM·SC) | **Live** — layout `7` / key `c` |
| Pure Intent reducer refactor | **Partial** — intents wired; further `intent_map!` optional |
| Full `select!` multi-wait loop | **Open** (optional; try_recv+poll remains QDI-valid) |

### Isochronic-fork rule (checkable)

> Every `.await` or unbounded drain reachable from H is an isochronic fork.  
> Legal only if something acknowledges it. Name the ack, or bound the branch.

| Branch | Status |
|--------|--------|
| `drain_dust!` | **Bounded** |
| `engine.handle` | **ε + budget + named queue** |
| `Effect::Net` | HITL-confirmed blocking (documented) |
| `engine.trigger` | ε after latch (permitted) |

Music conserved. Structure sovereign. Last → first.
