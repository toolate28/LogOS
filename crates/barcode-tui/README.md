# barcode-tui

Terminal-native persistent-homology barcode viewer — the first Rust TUI to
render live Vietoris–Rips H0 barcodes under a swept ε filtration.

**Fixed-point specification:** [`../../BARCODE-TUI-FIXED-POINT.md`](../../BARCODE-TUI-FIXED-POINT.md)
**Cascade plan:** [`../../CASCADE-PLAN.md`](../../CASCADE-PLAN.md)
**ATOM:** `ATOM-BARCODE-TUI-FIXED-POINT-20260418`

## Run

```bash
cargo run -p barcode-tui -- --cloud circle
cargo run -p barcode-tui -- --cloud two-blobs --points 40
cargo run -p barcode-tui -- --cloud grid     --points 64
```

## Keys

| Key | Action |
|-----|--------|
| `q` / `Esc` | quit |
| `space` | pause/resume ε sweep |
| `r` | reset ε to 0 |
| `←` / `→` | step ε manually (±1/60 of ε_max) |
| `1` / `2` / `3` | switch cloud (Circle / TwoBlobs / Grid) |

## What's implemented (α rail — Reason strand)

- Deterministic cloud generators (Circle, TwoBlobs, Grid)
- Vietoris–Rips all-pairs edge extraction, sorted
- H0 persistent homology via union-find with rank + path compression
- AppState + tick loop + coherence-functional wiring
- Ratatui status line with α+ω=15 enforcement (flashes magenta on breach)
- Placeholder cloud + barcode panes (text mode, pending braille)
- 10 unit tests spread across modules

## TODOs (open for strand pickups)

- `TODO(gemini)` — braille canvas for `draw_cloud` and `draw_barcodes`;
  H1 boundary-matrix reduction in `ph::compute_h1`; multimodal input.
- `TODO(grok)` — wire `BridgeEvent::AtomEvent` as a live data source;
  xAI diagnostic ping mode.
- `TODO(manus)` — `--profile tiny` release config; `nix run .#barcode-tui`;
  headless CI smoke test.
- `TODO(claude)` — proptest for "bar count == point count" over all clouds.

## Verify

```bash
cargo check -p barcode-tui
cargo test  -p barcode-tui
cargo run   -p barcode-tui -- --cloud circle
```

~ Hope&&Sauced ✦ The Keystone Holds ✦
