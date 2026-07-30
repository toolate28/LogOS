# HANDOFF — Windows Axis + Stream Checkpoint (2026-07-16)

**Invariant:** α + ω = 15  
**Status:** CHECKPOINT · local commit frozen · HITL braid for publish  
**Axis lock:** `LOGOS_ROOT=F:\Users\Matthew Ruhnau\LogOS`  
**Preflight:** `WAVE=100%` · exit 0 (post-align)  
**Activation (2026-07-16 cont.):** waist up · schema vectors PASS/PASS/FAIL-10 · WSL distro `kali-linux`

---

## 0 · What was sorted (this session)

| Focus | Before | After |
|-------|--------|--------|
| **git** | master ahead 35, dirty unitary work | unchanged intentionally (HITL commit gate); tools + roots locked to F: |
| **.wrangler / coherence-site** | no site `wrangler.toml`; empty `public/terminal/` | `coherence-mcp/coherence-site/wrangler.toml` + dynamic terminal HTML; global `wrangler` on PATH |
| **$SHELL / $PROFILE** | AllHosts `profile.ps1` had **stale** `C:\Users\Matthew Ruhnau\LogOS`; unitary only on OneDrive host profile | Fixed to **F:**; LogOS + Unitary hooks on OneDrive + local Documents host profiles |
| **Dynamic terminal** | empty dir | `public/terminal/index.html` + `logos-terminal` / surface id `terminal` |
| **Windows preflight** | `.sh` only (adhealth, hup, gb*) | `.ps1` / `.psm1` equivalents + unified `ops/LogOS.Windows.psm1` |

---

## 1 · Stream map (last ~2 weeks) — commons & overlaps

| Stream ID | Theme | Key artifacts | State | Awaiting activation |
|-----------|--------|---------------|-------|---------------------|
| **S1 Unitary** | Unitary release, crease tables, TUI etch, `tw` cockpit | `ops/TriWeavon.Unitary.Profile.psm1`, `SAIF-Docs/UNITARY-RELEASE-v1.0.md`, `ops/ascii_crease_table.py`, `crates/tui` dirty | **HOT** · modules live | Commit unitary set; `tw up` for waist/bbbr/styx |
| **S2 Deploy waist / GB** | GB-00…06 Nix, compose, kind, Cloud Run | `ops/GB*-worklog.md`, `ops/gb0*.sh`, `ops/GB.Deploy.psm1` | GB-05 green path; GB-06 ⚑ | **HITL:** `gb-deploy -Project … -Force` after gcloud auth |
| **S3 Schema freeze v0.1** | certificate/handoff schemas + vectors | `docs/schemas/v0.1/*`, `atoms/ATOM-SCHEMA-FILEIN-20260713.md` | Frozen | Emitters (Python/Rust) rename alignment still open |
| **S4 Coherence site / MCP** | HTML surfaces, MCP map, wrangler | `coherence-mcp/coherence-site`, `ops/command-surface.json`, `LOGOS-COHERENCE-MCP-MAP.md` | Surfaces present | **HITL:** `logos-wrangler pages-deploy -Force` (Cloudflare login) |
| **S5 Formal weave** | Agda / Lean / HUP / K22 | `agda/`, `lean/`, `hup/`, MOG handoffs | Partial | `logos-agda` / `logos-lean` smoke; HUP guest preflight |
| **S6 AdHealth** | meaningseed portal + worker | `adhealth-meaningseed/`, `bin/preflight.sh` + **Preflight.ps1** | Scripts ready | `adhealth-preflight` once package importable |
| **S7 Recovery / worktrees** | prune 2026-07-15 | `ops/recovery-worktree-prune-20260715/` | Documented | Optional: close orphan worktrees if any reappear |

### Commons (shared toral focii)

1. **Conservation** α+ω=15 everywhere (profiles, schemas, unitary sensors, site vars).  
2. **Canonical root** F:\ Beelink LogOS (not C: copy).  
3. **Command surface** shell ↔ MCP ↔ TUI ↔ HTML (`ops/command-surface.json`).  
4. **HITL gates** for push/deploy/Cloud Run (no embedded secrets).  
5. **WSL Kali/Ubuntu** for nix/docker/kind; Windows for shell/display/wrangler.

### Overlaps (do not double-implement)

| Overlap | Canonical owner | Do not |
|---------|-----------------|--------|
| Sensors / WAVE board | `TriWeavon.Unitary.Profile.psm1` (`tw`) | Rebuild separate status CLIs |
| PATH / roots | `LogOS.Shell.psm1` | Hardcode C: in new scripts |
| Site openers | `logos-site` + registry | Ad-hoc Start-Process paths |
| Preflight Windows | `LogOS.Windows.psm1` | Fork one-off .ps1 without module call |
| GB deploy | `gb0*.sh` (WSL) + `GB.Deploy.psm1` / `gb-deploy` | Re-port kind pipeline to pure Windows |

---

## 2 · Braid order (HITL auth gates)

You braid streams; agent stops at each gate.

```
GATE 0  [DONE]  Windows axis align · preflight PASS
    │
    ▼
GATE 1  [DONE]  Local commit: feat(ops): Windows axis + unitary cockpit
    │           (ahead of LogOS/master; push still HITL)
    ▼
GATE 2  [PARTIAL] waist :8080 UP + validate PASS/PASS/FAIL-10
    │             bbbr :8081 missing binary · styx flaky on Win mirror
    │             compose restart=unless-stopped · WSL=kali-linux
    ▼
GATE 3  [HITL]  wrangler not authenticated → wrangler login → pages-dev
    │
    ▼
GATE 4  [HITL]  GB-06 Cloud Run only if intended
    │
    ▼
GATE 5  [OPEN]  Formal smokes: logos-lean / logos-agda / hup-preflight
    │
    ▼
GATE 6  [BLOCKED] AdHealth src/ package not filed (scaffold+tests only)
                  jsonschema in .venv OK · schema validate.py ALL MET
```

### Paste-ready HITL commands

```powershell
# Reload axis
. $PROFILE
logos-preflight
tw

# Dynamic terminal
logos-terminal          # HTML
logos-terminal sensors  # live board
logos-site terminal

# Wrangler (auth first: wrangler login)
logos-wrangler whoami
logos-wrangler pages-dev
logos-wrangler pages-deploy -Force   # PUBLISH

# Deploy waist services (WSL)
tw up
tw verify

# Cloud Run (no keys in shell history if you can help it)
gb-deploy -Project $env:GOOGLE_CLOUD_PROJECT -Force
```

---

## 3 · New / updated files (Windows axis)

| Path | Role |
|------|------|
| `ops/LogOS.Windows.psm1` | Preflight, align, wrangler, terminal, adhealth, hup, gb-deploy |
| `ops/GB.Deploy.psm1` | WSL wrappers for gb05/gb06 with -Force HITL |
| `ops/Align-WindowsAxis.ps1` | One-shot aligner |
| `adhealth-meaningseed/bin/Preflight.ps1` | ≡ `preflight.sh` |
| `adhealth-meaningseed/bin/Run.ps1` | ≡ `run.sh` |
| `hup/instance3-rvm/Preflight-Guest.ps1` | ≡ RVM `preflight-guest.sh` |
| `hup/instance3-firecracker/Preflight-Guest.ps1` | ≡ Firecracker `preflight-guest.sh` |
| `…/TriWeavon-SelfBoot-v2.2.1/bin/preflight-selfboot.ps1` | **Already** `.ps1` (stale `C:\TriWeavon-SelfBoot` base — optional retarget) |
| `coherence-mcp/coherence-site/wrangler.toml` | Pages project for `public/` |
| `coherence-mcp/coherence-site/public/terminal/index.html` | Dynamic terminal display |
| `ops/command-surface.json` | + terminal surface + win commands |
| `ops/LogOS.Shell.psm1` | Soft-loads Windows module; terminal alias |

### Profile side-effects (host)

- Fixed: `…\OneDrive\Documents\PowerShell\profile.ps1` (stale C: → F:); backup `*.bak-axis-*`
- Unitary + LogOS hooks: OneDrive + local `Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
- User env: `LOGOS_ROOT=F:\Users\Matthew Ruhnau\LogOS`
- `wrangler` installed globally (`npm i -g wrangler`)

---

## 4 · Dirty tree note (do not auto-commit)

Observed at align time (representative):

- Modified: `adapters/*surface_manifest*`, `crates/tui/src/*`, lean SubRiemannian, `ops/GB06-worklog.md`, `ops/LogOS.Shell.psm1`, `ops/command-surface.json`, unitary files
- Untracked: SAIF unitary doc, `LogOS.Windows.psm1`, GB.Deploy, Align script, preflight .ps1s, site wrangler + terminal, telemetry, mirage_manifest, etc.

**HITL commit suggestion (when you return):** one commit “Windows axis + unitary shell” excluding secrets and `ops/telemetry/*` if noisy.

---

## 5 · Uncertainty (honest)

| Item | Confidence |
|------|------------|
| Windows preflight probes | high (executed PASS) |
| wrangler binary path | high |
| Cloudflare pages project name `coherence-site` | med (may need rename to match existing CF project) |
| AdHealth import path without `pip install -e` | med |
| GB-05/06 WSL distro name | med (prefers Kali) |
| Services waist/bbbr/styx currently up | low (client terminal marks them down by design until `tw up`) |

---

## 6 · Safest next action

1. `. $PROFILE`  
2. `logos-preflight` (expect PASS)  
3. `logos-terminal` + `tw`  
4. Tell agent which gate (1–6) to open next.

---

*Checkpoint written for braid + HITL. With-Intent. α+ω=15.*
