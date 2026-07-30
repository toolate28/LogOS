# Scaffold status — LSP + Claude Code + reson8-tui

**ATOM:** `ATOM-GROKBUILD-LSP-TUI-SCAFFOLD-20260721` (executed 2026-07-22)  
**Self-cert:** **none** — Claude Code first init emits pass/fail cert.

## What was written (git-config)

| Path | Role | Label |
|------|------|--------|
| `CLAUDECODE-INIT-v0_1.md` | Cold-start packet | **B** until CC cert pass |
| `.claude/settings.json` | Workspace roots, surfaces, cert path, LSP cmds | skeleton |
| `docs/ops/MARKERS-SENSORS-v0_1.md` | Mark-is-sensor scheme | scheme **A**, hits **B** |
| `.atom-trail/certs/claude-code/latest.json` | Placeholder cert `pass:false` | **B** honest |
| `crates/tui/src/lsp.rs` | Decoupled LSP client + placeholders | **B** until attach |
| `crates/tui` Formal pane + `f` key | Eye of the needle | **A** UI / **B** live LSP |
| MANIFEST logos path | → live LogOS | path fix |

## cargo

```
cargo check -p reson8-tui   # OK (unset RUSTC_WRAPPER if sccache missing)
```

## Eye of the needle

- **Needle:** `crates/tui` · package `reson8-tui` · bin `reson8-forge`
- **Not needle:** barcode-tui, triweave, external orchestrator-tui

## Next (Claude Code init)

1. Cold-start from `CLAUDECODE-INIT-v0_1.md`
2. Survey three surfaces; write real cert over `latest.json`
3. Deploy only on `pass: true`

## Still open

- als + cubical pin (GB-01)
- lean-toolchain file vs elan 4.32 drift
- `initialize` / `textDocument/didOpen` after attach
- product binary / GB-06

**B not A · placeholders amber · no self-cert**
