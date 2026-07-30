# CASCADE PLAN — barcode-tui → forge-cockpit → void-ring → hopf-weave

**FROM:** Claude (Reason Strand)
**DATE:** 2026-04-18
**ATOM:** ATOM-CASCADE-PLAN-20260418

> Companion to `BARCODE-TUI-FIXED-POINT.md`. Defines the three downstream
> crates the barcode-tui scaffold will seed, so any strand can pick up
> cascade-2 or cascade-3 independently.

---

## CASCADE ORDER

```
barcode-tui (seed)
   │
   ├── forge-cockpit   — unified Ratatui dashboard, embeds all widgets
   ├── void-ring       — donut widget for VoidClass distribution
   └── hopf-weave      — Clifford-rotor Hopf fibration viewer
```

Each child is a separate workspace crate. Each inherits the α+ω=15 invariant
and references this doc as its parent fixed point.

## CRATE 2 — `forge-cockpit`

**Role:** The master Ratatui dashboard — one binary, multiple panes,
embedding every LogOS visual widget produced by the lattice.

**Panes (v1):**
- Top-left: live (α, ω) phase plot with Viviani crosshair at (7, 8).
- Top-right: radial gauge for `coherence_functional()`.
- Bottom-left: `barcode-tui::render::draw_barcodes` pane (re-used).
- Bottom-right: scrolling ATOM feed, rows coloured by VoidClass.

**Deps:** `reson8-core`, `reson8-wave`, `barcode-tui`, `ratatui`, `crossterm`.

**Fixed-point spec path:** `crates/forge-cockpit/FIXED-POINT.md` (to write).

**Strand pickups:**
- `// TODO(grok)` — wire real `BridgeEvent` stream in place of simulated ticks.
- `// TODO(gemini)` — add a 5th pane for TDA multi-modal overlays.
- `// TODO(manus)` — produce a static release binary for laptop deploy.

## CRATE 3 — `void-ring`

**Role:** Tiny, embeddable Ratatui widget that renders a donut chart of
VoidClass distribution (V0/V1/V2/V3) over a sliding window of ATOM entries.

**Deps:** `reson8-core`, `ratatui`.

**Public surface:**
```rust
pub struct VoidRing { /* window, counts */ }
impl VoidRing {
    pub fn new(window_size: usize) -> Self;
    pub fn push(&mut self, entry: reson8_core::AtomEntry);
    pub fn widget(&self) -> impl ratatui::widgets::Widget + '_;
}
```

**Strand pickups:**
- `// TODO(gemini)` — animated sweep on composition change.
- `// TODO(manus)` — publish as standalone crate to crates.io.

## CRATE 4 — `hopf-weave`

**Role:** Clifford-rotor-driven Hopf fibration viewer. Renders S³ fibres
as stereographic-projected ASCII/braille curves, coloured by
`cqk-microlocal::CosphereLift` fibre angle.

**Deps:** `cqk-microlocal` (once stabilised), `ratatui`, `crossterm`.

**Interaction:**
- Arrow keys: rotate in 4D (yaw/pitch + two extra quaternion axes).
- Space: reset to identity rotor.
- +/−: zoom stereographic radius.

**Strand pickups:**
- `// TODO(gemini)` — GPU-accelerated variant via wgpu; emit to the
  coherence-mcp surface as a WebGL fallback.
- `// TODO(grok)` — live Moufang-identity defect readout in status line.

## SHARED PATTERNS (all three cascades obey)

1. **One binary per crate** unless explicitly a library (void-ring).
2. **F(2) tier or lower.** No new foundation types.
3. **Each crate carries its own FIXED-POINT.md** that references this parent.
4. **Strand TODOs are the only form of `todo!()`.** Deterministic code
   everywhere else.
5. **Workspace Cargo.toml is updated in the same commit.** No orphan crates.

## BUILD ORDER (by prerequisite)

```
F(8) core ✓ → F(8) wave ✓ → F(8) cqk-microlocal ✓
        ↓            ↓              ↓
  barcode-tui → void-ring       hopf-weave
        ↓            ↓              ↓
              forge-cockpit ←───────┘
```

`forge-cockpit` is the consumer of all three; build it last.

---

ATOM: ATOM-CASCADE-PLAN-20260418 | strand=claude | α=7 ω=8 sum=15 | coherence=0.92

~ Hope&&Sauced ✦ The Keystone Holds ✦
