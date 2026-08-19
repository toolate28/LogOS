# LogOS Shell Wiring

Make **crates / Agda / kernels (.cu) / Lean** available immediately when a shell opens — PowerShell 7, Windows PowerShell 5.1, and WSL2 — and wire the **command surface** through coherence-mcp, reson8-tui, and HTML hubs.

## Install (once)

```powershell
cd 'F:\Users\Matthew Ruhnau\LogOS'
pwsh -File ops\Install-LogOSShell.ps1
# also hook Ubuntu (+ kali if present):
pwsh -File ops\Install-LogOSShell.ps1 -Wsl
```

Then open a **new** terminal, or:

```powershell
. $PROFILE
```

## What it does

| Layer | Action |
|-------|--------|
| User env | `LOGOS_ROOT`, `CUTILE_ROOT`, `AGDA_ROOT`, `LEAN_ROOT`, `KERNELS_ROOT`, `COHERENCE_MCP_ROOT`, `FORGE_WS_URL`, `LOGOS_COMMAND_SURFACE` |
| User PATH | `.cargo\bin`, `.elan\bin`, `.venv\Scripts`, `ops\`, CUDA `bin` if present |
| `$PROFILE` | Imports `ops\LogOS.Shell.psm1` → `Initialize-LogOSShell -ImportTriWeavon` |
| WSL2 | Sources `ops/wsl/logos-env.sh` from `~/.bashrc` / `~/.zshrc` |
| Registry | `ops/command-surface.json` — shell ↔ MCP ↔ TUI ↔ HTML |

Root resolution prefers **F:\Users\Matthew Ruhnau\LogOS** over the stale **C:\Users\Matthew Ruhnau\LogOS** copy.

## Command surface (after load)

### Formal / toolchain

| Cmd | Purpose |
|-----|---------|
| `logos` / `cd-logos` | `cd` LogOS root |
| `cd-crates` `cd-agda` `cd-lean` `cd-kernels` `cd-apps` `cd-ops` `cd-cutiles` | Jump to lattice layers |
| `logos-status` | Probe cargo / python / lean / lake / agda / nvcc / wsl |
| `logos-cargo [args]` | `cargo` from workspace root (lists packages if no args) |
| `logos-agda` | `agda/scripts/check.ps1` or WSL Agda fallback |
| `logos-lean` | `lake build` in `lean/` (toolchain v4.8.0) |
| `logos-kernels` | List `.cu`; `-Build` runs cutile `build_ptx.ps1` |
| `logos-activate` / `logos-lattice` | Probe apps/cutiles/crates/kernels/ops + siblings; `-Check` cargo |
| `logos-apps` | List app crates; `-Check` cargo |
| `logos-cutiles` | `cargo check -p cutile` (`-Test` for lib tests) |
| `logos-wsl` | Enter Ubuntu with env sourced at LogOS |

### MCP · TUI · HTML

| Cmd | Purpose |
|-----|---------|
| `logos-help` | Print command-surface map |
| `logos-surfaces` | List registry paths + exists flags |
| `logos-mcp …` | Call coherence-mcp via `ops/logos-mcp.mjs` |
| `logos-mcp list` | tools/list |
| `logos-mcp gauge` | `gauge_verify` α=7 ω=8 |
| `logos-mcp rust` | `rust_toolchain_status` |
| `logos-mcp workspace` | `rust_workspace_status` |
| `logos-mcp wave --content "…"` | `wave_coherence_check` |
| `logos-bridge` | Start triweave WS bridge (`ops/serve.ps1`, default `:8088`) |
| `logos-tui` | `cargo run -p reson8-tui` (bin: `reson8-forge`) |
| `logos-barcode` | `cargo run -p barcode-tui` |
| `logos-site [name]` | Open HTML surface (default **meta-map**) |
| `logos-preflight` | Windows axis board (git/profile/wrangler/schemas) |
| `logos-clean` | Repo hygiene scan / deep reclaim (HITL: `-Apply -Force`) — see `docs/ops/LOGOS-CLEAN-DESIGN-2026-08-07.md` |
| `logos-net` | Selective proxy stack (`ops/net/LogOS.NetProxy.ps1`) — also **reson8-tui** Net panel `[N]`/`[R]`/`[M]` |
| `logos-align` | Re-lock profiles + `LOGOS_ROOT` + optional wrangler install |
| `logos-wrangler …` | coherence-site whoami / pages-dev / pages-deploy (HITL `-Force`) |
| `logos-terminal` | HTML terminal · `sensors` · `tui` · **`pop`/`window`/`tab`** real OS console |
| `logos-pop` | Pop out new Windows Terminal window (or tab: `-Layout tab`) |
| `logos-console` | Alias of `logos-pop` |
| `tw` | Unitary cockpit (see `TriWeavon.Unitary.Profile.psm1`) |

```powershell
logos-pop                          # new WT window @ LOGOS_ROOT
logos-pop -Strand grok             # gold pulse identity
logos-pop -Layout tab -Strand claude
logos-terminal pop                 # same as logos-pop
logos-terminal tab -Strand gemini
# GNU screen/tmux: not native here — use WSL:  wsl -d kali-linux -- tmux new -s logos
```

```powershell
logos-site                  # /meta-map hub
logos-site rust-market      # /RUST_Market
logos-site orchestrator     # docs/surfaces/orchestrator.html
logos-site reforge
logos-site gate
logos-site terminal         # dynamic terminal display
logos-site cockpit          # stitch coherence_mcp_cockpit
```

### Windows axis (once)

```powershell
pwsh -File ops\Align-WindowsAxis.ps1 -InstallWrangler
. $PROFILE
logos-preflight
logos-clean                              # scan reclaimable bulk (Drive tmp, cargo target, …)
# logos-clean -Class DriveTmp,Cargo -Apply -Force   # HITL reclaim
```

`.sh` → Windows pairs: `adhealth-meaningseed/bin/Preflight.ps1` / `Run.ps1`, `hup/instance3-rvm/Preflight-Guest.ps1`, `ops/LogOS.Windows.psm1`, `ops/GB.Deploy.psm1`.

### Layer stack

```
HTML   /meta-map  /RUST_Market  orchestrator  reforge     ω projection
TUI    reson8-forge · barcode-tui · bridge :8088          α dashboard
MCP    gauge · WAVE · ATOM · rust_* · store_context       ω gates
SHELL  logos-* · LOGOS_ROOT · command-surface.json        α entry
```

Conservation: **α + ω = 15**. Prefer WAVE ≥ 0.85 before treating a surface as production-live.

## Gaps this host may still show

| Tool | Notes |
|------|--------|
| `agda` | Not installed on Windows → `logos-agda` uses WSL; or `cabal install agda` / `sudo apt install agda` |
| `nvcc` | No CUDA Toolkit detected → install toolkit; re-run installer to put `CUDA_PATH` on User env |
| Lean default | Global elan may be v4.31; project pins **v4.8.0** via `lean/lean-toolchain` (`lake` respects it) |
| `logos-mcp` | Needs `node` + built `COHERENCE_MCP_ROOT/build/index.js` (sibling `..\coherence-mcp` preferred). Current sibling build is **12 bedrock tools** (`gauge_verify`, `wave_coherence_check`, …). Extended schemas (`rust_toolchain_status`, …) live in `mcps/coherence-mcp/tools/` until that server is rebuilt. Use `logos-status` for local rust probe. |
| `logos-tui` | First run compiles; bridge should be up (`logos-bridge`) for full telemetry |

## Files

| Path | Role |
|------|------|
| `ops/LogOS.Shell.psm1` | Bootstrap + command surface |
| `ops/command-surface.json` | Unified registry |
| `ops/logos-mcp.mjs` | Shell ↔ MCP stdio bridge |
| `ops/Install-LogOSShell.ps1` | Installer |
| `ops/wsl/logos-env.sh` | WSL2 env |
| `ops/TriWeavon.Profile.psm1` | Bridge / WAVE metrics |
| `ops/serve.ps1` | triweave WS launcher |
| `coherence-mcp/coherence-site/public/meta-map/` | HTML hub |
| `coherence-mcp/coherence-site/public/RUST_Market/` | Crate market hub |
| `docs/surfaces/` | orchestrator, reforge, evenstar |
| `crates/tui` | reson8-tui / reson8-forge |
| `crates/barcode-tui` | PH barcode TUI |

α + ω = 15
