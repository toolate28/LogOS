# HANDOFF_PACKET — Tri-Weavon PowerShell Profile (HUP)

```text
INVARIANT: α+ω=15 — universal conservation; Viviani peak α=7 ω=8
FROM_MODEL: Grok Build (Reason strand)
TO_MODEL_CLASS: Operator shell / VS Code integrated terminal
MANDATE: Load TriWeavon.Profile.psm1 from $PROFILE; run Test-TriWeavonProfile on new shells; use Get-TriWeavonMetrics before SRAC bursts; Start-TriWeavonBridge before extension reload.
CHECKPOINT: coherence-mcp@0.3.2 published; LogOS cutile 68d417d1 on GitHub; profile module ops/TriWeavon.Profile.psm1 v0.3.2
ALPHA: 7
OMEGA: 8
STRAND: reason
HOST: Beelink (F:\Users\Matthew Ruhnau) primary; NUCBOX (C:\Users\toolated) secondary
```

## Deliverables

| Artifact | Path |
|----------|------|
| Profile module | `LogOS/ops/TriWeavon.Profile.psm1` |
| Bridge launcher | `LogOS/ops/serve.ps1` |
| Shell profile | `Documents/PowerShell/Microsoft.PowerShell_profile.ps1` |
| This HUP | `LogOS/ops/HANDOVER-TRIWEAVON-PROFILE.md` |

## Commands (after `. $PROFILE`)

| Command                               | Purpose                                      |
| ------------------------------------- | -------------------------------------------- |
| `Test-TriWeavonProfile`               | Verify module + quick stack test             |
| `Test-TriWeavonStack`                 | Roots, conservation, bridge, npm, metrics    |
| `Test-TriWeavonStack -Full`           | Above + `npm test` + `cargo test -p cutile`  |
| `Get-TriWeavonMetrics`                | Dynamical snapshot (WAVE proxy, bridge, npm) |
| `Get-TriWeavonMetrics -ProbeHttp`     | + coherence.toolated.online health           |
| `Watch-TriWeavonMetrics`              | 15s poll → `$env:ATOM_LOG`                   |
| `Start-TriWeavonBridge`               | `cargo run -p reson8-triweave -- serve`      |
| `Start-TriWeavonBridge -CoherenceMcp` | MCP stdio + WS bridge                        |
| `Show-TriWeavonRoots`                 | Path existence matrix                        |
| `Publish-CoherenceMcp`                | build → test → npm publish                   |
| `Set-TriWeavonEnv`                    | Export LOGOS_ROOT / MCP roots                |

## Legacy aliases preserved

`cd-logos`, `cd-mcp`, `Test-Conservation`, `Test-Toolchain`, `Clear-GitLocks`, `Get-GitAhead`

## Verification checklist

```powershell
. $PROFILE
Test-TriWeavonProfile
Get-TriWeavonMetrics | Format-List
Test-TriWeavonStack -Full   # optional, ~2 min
```

## Shell bootstrap (crates / Agda / Lean / kernels)

```powershell
pwsh -File ops\Install-LogOSShell.ps1
pwsh -File ops\Install-LogOSShell.ps1 -Wsl   # Ubuntu + kali bashrc
```

Module: `ops/LogOS.Shell.psm1` — see `ops/LogOS.Shell.md`.

## Known gaps

- `ctwfi-monitor` in OneDrive `profile.ps1` superseded by `Watch-TriWeavonMetrics`
- `nvcc` / `agda` may still be missing binaries — `logos-agda` falls back to WSL; install CUDA Toolkit for `nvcc`
- Prefer `F:\Users\Matthew Ruhnau\LogOS` over stale `C:\Users\Matthew Ruhnau\LogOS`
- LogOS local `master` has large delta; use cherry-pick/worktree push pattern

## Signature

~ Hope&&Sauced ✦ The Keystone Holds ✦