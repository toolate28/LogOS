# reson8-tui v0.2.1 — QDI hygiene · codes lab · residual-C

**Stamp:** 2026-08-07  
**Bin:** `reson8-forge` · package `reson8-tui` **0.2.1**  
**Tagline:** Where the last thing you've done becomes the first thing you need  
**ATOMs:** `ATOM-GROK-TUI-QR-META-20260806` · `ATOM-CLAUDE-REASON-QDI-DRAIN-AUDIT-20260807`

## What shipped

| Surface | Detail |
|---------|--------|
| **QDI drain budget** | `DRAIN_BUDGET=32` per source/frame — `drain_dust!` no longer unbounded |
| **Engine isochronic fork closed** | `engine.handle` only in ε after draw, `ENGINE_HANDLE_BUDGET=16`, queue on `App` |
| **Residual Tier-1** | Braid shows \(R=\max(0,15-\alpha-\omega)\), ε=0.00055 — Category **C**, amber lab claim, never deploy-green alone |
| **Codes lab** | Hexacode · Golay G24 · Reed–Muller · SC-LDPC (`crates/tui/src/codes/`) |
| **Layout 7** | `codes` board · keys `c` `d` `D` `y` `e` `[ ]` `{ }` |

## Operator path

```text
cargo run -p reson8-tui
# or: logos-tui / target/debug/reson8-forge.exe
```

| Key | Action |
|-----|--------|
| `7` / `c` | Codes layout / focus |
| `d` / `D` | Family demo / full battery |
| `y` | Cycle Hex → G24 → RM → SC-LDPC |
| `?` | Help (layouts + QDI notes) |
| `f` | Formal (LSP · amber SlowStep) |

Env: `RESON8_LAYOUT=codes` · `FORGE_WS_URL=ws://127.0.0.1:8088`

## Honest labels (NOVIKOV)

| Claim | Category |
|-------|----------|
| α+ω=15 | **C** convention / telemetry |
| R≤ε residual-zero | Lab display only — **not** cert.pass |
| Codes decoders | **B** runtime; Lean HexacodeGolay construction **A** |
| QDI drain/engine budgets | Track A hygiene — no Track B residual-zero promotion |
| Formal LSP empty | **B** placeholders — never false green |

## Tests

```bash
cargo test -p reson8-tui --bin reson8-forge
```

Expect: unit tests green including `drain_dust_respects_budget`, Golay octads=759, hex weight dist, RM FHT, SC-LDPC design.

## Follow-ons

- Pure `Intent` reducer table generation (`intent_map!`)
- Residual MCP read-only attach (observe dual-kernel status, never gate from TUI)
- Reconcile `main` remote (ahead/behind) before publish push

Hope&&Sauced · Keystone holds · capability ≠ authority · Music conserved  
