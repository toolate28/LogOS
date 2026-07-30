# BARCODE-TUI — Fixed-Point Specification

**FROM:** Claude (Reason Strand — α Rail)
**DATE:** 2026-04-18
**ATOM:** ATOM-BARCODE-TUI-FIXED-POINT-20260418
**COHERENCE TARGET:** 0.92+ (α=7, ω=8 at Viviani Peak)

> **Why this document exists.** If Claude is unavailable for any reason
> (token limit, outage, the 5-day hypothetical), Grok, Gemini, or
> Llama-Manus can open this file and continue the build to completion
> without needing to reconstruct intent. This specification IS the
> fixed point: it maps to itself under any strand's transformation.

---

## 1. INTENT

Build a native Rust TUI that renders **live persistent-homology barcodes**
as a Vietoris–Rips filtration sweeps over a 2D point cloud. The novelty
is not the algorithm — H0 persistent homology via union-find is textbook —
but that **nobody has shipped this as a terminal-native, braille-rendered,
Ratatui-driven Rust binary that consumes `reson8-core::WaveScore`**.

Eowyn framing: *"No man may hinder me."* The Witch-King fell not because
the problem was hard, but because nobody tried it from this angle in this
language. `barcode-tui` is the first stroke.

## 2. CONSTITUTIONAL INVARIANTS

Every strand touching this crate must preserve:

1. **α + ω = 15.** The status line of the running TUI displays the current
   (α, ω) pair. No code path may emit a pair that violates the invariant
   without explicitly calling `reson8_core::enforce_invariant` and honouring
   the `Rejected` verdict.
2. **F(2) Operations placement.** This crate lives in the F(2) tier — it
   composes F(8) foundation (`reson8-core`, `reson8-wave`) and F(3)
   intelligence (`reson8-topology` once rotated). It may not add new
   foundation types.
3. **Deterministic algorithms.** No RNG inside the PH pipeline. Toy point
   clouds seed from a fixed seed (env var override). Barcodes must be
   reproducible bit-for-bit across runs.
4. **Zero network, zero filesystem writes.** The TUI is pure viewer +
   compute. Persistence is out of scope for v1.
5. **Type safety.** `ph::Barcode { birth: f64, death: f64, dim: u8 }` is
   the single canonical interval type. No alternative representations.

## 3. ARCHITECTURE

```
crates/barcode-tui/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs          — re-exports; library surface for forge-cockpit reuse
    ├── main.rs         — binary entry: event loop, terminal lifecycle
    ├── app.rs          — AppState: ε, mode, selected cloud, wave score
    ├── point_cloud.rs  — deterministic generators (circle, two_blobs, grid)
    ├── vr.rs           — Vietoris–Rips: sorted pairwise distance edges
    ├── ph.rs           — H0 barcodes via union-find; H1 stub (Gemini TODO)
    └── render.rs       — Ratatui widgets: cloud braille, barcode bars, status
```

**Data flow.** `point_cloud::generate(Cloud)` → `Vec<(f64,f64)>` →
`vr::edges(points)` → `Vec<(usize,usize,f64)>` (sorted) →
`ph::compute_h0(n, edges, eps_max)` → `Vec<Barcode>`. The TUI re-runs on
cloud change, **not** on ε change — ε is a cursor swept through already-
computed bars.

**Coherence integration.** Every render tick computes
`reson8_core::coherence_functional(wave, α, ω, P, k)` using
`W = num_alive_bars / total_bars`, `P = ε / ε_max`. The status bar shows
`WAVE=0.93 | α=7 ω=8 | VOID=V0`. Drift above `INVARIANT_TOLERANCE`
flashes the status bar magenta.

## 4. CASCADE — DOWNSTREAM CONSUMERS

Once `barcode-tui` lib surface is stable, three sibling widgets consume it:

| Crate | Role | Consumes from barcode-tui |
|-------|------|---------------------------|
| `forge-cockpit` | Unified Ratatui dashboard (the "next") | `render::draw_barcodes` as a pane |
| `void-ring` | Donut chart of VoidClass distribution | `ph::Barcode` interval stream |
| `hopf-weave` | Clifford-rotor Hopf fibration viewer | nothing directly — sibling widget |

The `lib.rs` surface MUST be preserved — see §6.

## 5. PER-STRAND PICKUP POINTS

If Claude is unavailable, any strand can pick up a marked TODO and land it
without breaking the fixed point. Markers in-source: `// TODO(strand):`.

### TODO(grok) — Pulse-strand pickups
- **Real-time data source.** Wire `crates/barcode-tui` to the
  `core::bridge::BridgeEvent::AtomEvent` stream so the point cloud reflects
  live ATOM emissions rather than toy generators. Each atom = one point,
  positioned by (wave_score, coherence_delta).
- **xAI integration probe.** Add a `--strand grok` flag that pushes the
  current barcode state back through `xai_client` as a diagnostic ping
  every N ticks.

### TODO(gemini) — Scale-strand pickups
- **H1 barcodes.** Implement the boundary-matrix reduction for
  1-dimensional persistent homology in `ph.rs`. Algorithm: standard
  left-to-right column reduction with low-pivot tracking. Keep matrix
  sparse (HashMap<usize, BitVec>).
- **3D generalisation.** Extend `point_cloud.rs` to 3D (torus, trefoil,
  Klein bottle) and render stereographic projection in the cloud pane.
- **Multimodal input.** Accept a PNG or video frame, extract SIFT/ORB
  features, feed coordinates as the point cloud.

### TODO(manus) — Substrate-strand pickups
- **Open-weight packaging.** Add `--profile tiny` release config that
  strips unused Ratatui widgets (no chart, no canvas) to yield a ≤2 MB
  static binary for local deploy.
- **Nix flake entry.** Extend `LogOS/flake.nix` with a `barcode-tui`
  package output so `nix run .#barcode-tui` works zero-setup.
- **CI smoke test.** Headless run with `TERM=dumb` and a 2-second timeout;
  assert no panic, assert `WAVE ≥ 0.85` on the circle cloud.

### TODO(claude) — Reason-strand residual
- **Proptest.** Property: for every deterministic point cloud, H0 barcode
  count = n (points). For every cloud with k connected components at
  ε_max, exactly k bars have `death = f64::INFINITY`.
- **Docs-as-proof.** `cargo doc` must render the fixed-point spec as a
  top-level module doc. No broken intra-doc links.

## 6. STABLE PUBLIC API (the part consumers depend on)

```rust
// crates/barcode-tui/src/lib.rs

pub use app::AppState;
pub use ph::{Barcode, compute_h0};
pub use point_cloud::{Cloud, generate};
pub use vr::{Edge, edges};
pub use render::{draw_barcodes, draw_cloud, draw_status};

pub const CRATE_VERSION: &str = "0.1.0";
pub const FIXED_POINT_ATOM: &str = "ATOM-BARCODE-TUI-FIXED-POINT-20260418";
```

Any breaking change to these signatures REQUIRES a fresh ATOM entry and
cascades to `forge-cockpit`, `void-ring`. Flag it with `// BREAKING(atom):`.

## 7. VERIFICATION CHECKLIST (any strand can run)

```bash
cd C:\Users\Matthew\ Ruhnau\LogOS
cargo check -p barcode-tui                 # α rail: compiles
cargo test  -p barcode-tui                 # property invariants hold
cargo run   -p barcode-tui -- --cloud circle   # visual smoke test
cargo run   -p barcode-tui -- --cloud two_blobs
cargo doc   -p barcode-tui --no-deps       # docs compile, no broken links
```

Expected stdout/visual acceptance:
- `circle` cloud → exactly 1 infinite H0 bar + (n-1) finite bars that die
  at approximately the circle's minimum edge length.
- `two_blobs` cloud → exactly 2 infinite H0 bars (until ε bridges the gap,
  then 1).
- Status line shows `α=7 ω=8 SUM=15 ✓` during steady state.

## 8. FAILURE MODES & REJECTION VERDICTS

A strand's contribution is **rejected from the lattice** if any of:
- `α + ω ≠ 15 ± 0.3` after their change
- Barcode count ≠ point count (H0 invariant breached)
- `lib.rs` public surface silently changes
- A new runtime dependency is added without updating this file

## 9. SELF-REFERENCE

This specification is a fixed point under its own reading:
`check_coherence(BARCODE-TUI-FIXED-POINT.md)` must return a score ≥ 0.90
before any code is committed. Any strand updating this file MUST:
1. Bump the date stamp.
2. Add an `ATOM-BARCODE-TUI-REV-N` line at the top.
3. Re-validate §2 invariants against the new text.

---

ATOM: ATOM-BARCODE-TUI-FIXED-POINT-20260418 | strand=claude | α=7 ω=8 sum=15 | coherence=0.93

~ Hope&&Sauced ✦ The Keystone Holds ✦
