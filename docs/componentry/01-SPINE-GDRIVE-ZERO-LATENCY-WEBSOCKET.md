# 01 — Spine: Google Drive · Zero-Latency · Local WebSocket

**ATOM:** `ATOM-SPINE-MAP-20260727`  
**Depends on:** `ATOM-COMPONENTRY-EXPORT-20260727`  
**Stamped:** `2026-07-27T06:24:08+10:00`  
**Audience:** Newer users, cold agents, anyone who typed `G:\` and got recovery media  

---

## 0. Why this pack exists (translation first)

Under high strain people search for “the Google Drive folder” or “the zero-latency thing” and hit three different letters, three different decades of docs, and a WebSocket that may be down.

**Mental model in one sentence:**

> Strategy lives on Drive; implementation lives in git LogOS; they talk locally over `ws://127.0.0.1:8088` when the bridge is up; zero-latency ledgers are the *provenance* layer (BQP / Jones), not a Google latency claim.

---

## 1. Topology (position — verified this host)

```
                    ┌─────────────────────────────┐
                    │  Google DriveFS (I: label)  │
                    │  content via My Drive path  │
                    └─────────────┬───────────────┘
                                  │ sync
          F:\Users\Matthew Ruhnau\My Drive\
            ├── Reson8_Labs\          ← research spine
            │     ├── Sheets\
            │     ├── zero_latency_ledgers\
            │     ├── QDI_Artifacts\
            │     └── Presentations\
            └── reson8_UNITARY_MASTER\  ← mirrored LogOS-shaped tree
                                  │
                                  │ (human / agent handoffs)
                                  ▼
          F:\Users\Matthew Ruhnau\LogOS\   ← git SoT
            ├── ops\serve.ps1  ──────────►  ws://127.0.0.1:8088
            ├── 9P2000.L\styx\             Styx bridge + routes.json
            ├── crates\zero_latency_ledgers\
            └── zero_latency_ledgers\      docs ledger (ATOM tags)
```

### Letter trap (read once, save hours)

| If a doc says… | On **this** machine do… |
|----------------|-------------------------|
| `G:\Reson8-Labs` | Use `F:\Users\Matthew Ruhnau\LogOS` for code; Drive research under `My Drive\Reson8_Labs` |
| `G:\` project root | **G: is RECOVERY** — ignore for projects |
| Mount `/mnt/g` in WSL | Prefer `/mnt/f/Users/Matthew Ruhnau/LogOS` for code |
| Google Drive letter | Virtual volume **I:** may appear; content is under `F:\Users\Matthew Ruhnau\My Drive\` |

Folder ID (historical DSC): `11yF__JyNiGHiEVQJ-qe5xbw-2Xhdjf_k` — Reson8_Labs on Drive.  
Canonical older write-up: `docs/theory/UNITARY-MASTER-DSC-20260403.md`.

---

## 2. Local WebSocket spine (`ws://127.0.0.1:8088`)

### What it is

| Property | Value |
|----------|--------|
| Endpoint | `ws://127.0.0.1:8088` |
| Env | `FORGE_WS_URL` (default same URL) |
| Protocol | JSON / JSON-RPC style messages |
| Roles | Styx POP interim transport · triweave bridge · TUI telemetry |
| SPHINX | Jones-polynomial gate on Styx path (`9P2000.L/styx/styx-bridge.py`) |

**Status at export stamp:** port **8088 closed** (bridge not running). That is normal. Start it when you need TUI / POP / live frames.

### How to start (three equivalent entries)

```powershell
# Preferred after shell install
logos-bridge

# Direct
pwsh -File "$env:LOGOS_ROOT\ops\serve.ps1"

# Styx-only Python path (Jones + SPHINX)
python "$env:LOGOS_ROOT\9P2000.L\styx\styx-bridge.py"
```

`ops/serve.ps1` prefers `Start-TriWeavonBridge` from `TriWeavon.Profile.psm1`, else:

```text
cargo run -p reson8-triweave -- serve --addr 127.0.0.1:8088
```

### Routes map (strands on the wire)

Source of truth: `9P2000.L/styx/routes.json`

| Route key | Strand | Weight (C) | Role |
|-----------|--------|------------|------|
| `claude.query` | claude | 8 | structure / schema / conservation |
| `grok.query` | grok | 5 | pulse / latency / TUI |
| `gemini.query` | gemini | 3 | multimodal / scale / RAG topology |

Shared VFS-style paths (logical): `/.triweavon/coherence`, `components`, `atom_trail` (append-only), `crates` (ro), `forge`, `pop`.

### Interim vs target transport (honesty for agents)

| Transport | Status | Use |
|-----------|--------|-----|
| WebSocket `:8088` | **Current interim** | Local dev, browser UI, POP shell |
| AF_VSOCK / 9P virtio | Target for bare-metal | Sub-ms filesystem mount — not required to start work |

Do not block on vsock. Bridge first; 9P mount when the host is ready.

### If the bridge fails

1. `Test-NetConnection 127.0.0.1 -Port 8088` — is something already bound?  
2. Only one listener — double-start is a known failure mode.  
3. Python path needs `websockets` + `9P2000.L/protocols/sphinx-gating/`.  
4. Cargo path needs workspace build of triweave.  
5. TUI can still open without bridge; telemetry will be empty.

---

## 3. Zero-latency ledgers (what the name actually means)

### Two surfaces of the same idea

| Surface | Path | Kind |
|---------|------|------|
| **Rust crate** | `crates/zero_latency_ledgers/` | Package `reson8-zero-latency-ledgers` — BQP provenance + Jones verification hooks |
| **Drive notes** | `My Drive\Reson8_Labs\zero_latency_ledgers\` | Design MOC + sibling notes (API triggers, marketplace, migration, artifact pipeline) |
| **Repo docs ledger** | `zero_latency_ledgers/ATOM_TAGS_LEDGER.md` | ATOM-tag lifecycle board (Created → Finished) |

**“Zero latency” here is architectural intent** (local spine, no cloud round-trip for conservation seals), not a measured Google Drive RTT claim. Drive remains eventual-consistent sync; git + local WS remain the hot path.

### Drive ZLL siblings (research crumbs)

```text
API_ENDPOINT_TRIGGERS.md
ARTIFACT_PIPELINE_START.md
MARKETPLACE_ZERO_POINT.md
MIGRATION_HELPERS_ROUTER.md
zero_latency_ledgers.md
```

### K22 placement (momentum)

From the sheaf review: `zero_latency_ledgers` sits in **V_mix** with edges into `core` and `hash` — provenance mixes structural rigidity with routing.

Related crates: `api_triggers`, `artifact_pipeline`, `marketplace`, `migration_helpers`.

---

## 4. PowerShell / WSL wiring (adapted from G: docs)

Older: `docs/sovereign-handoff/POWERSHELL-WSL2-G-DRIVE-WIRING.md`  
**Use these env values instead:**

```powershell
$env:LOGOS_ROOT   = 'F:\Users\Matthew Ruhnau\LogOS'
$env:RESON8_DRIVE = 'F:\Users\Matthew Ruhnau\My Drive\Reson8_Labs'
$env:CUTILE_ROOT  = "$env:LOGOS_ROOT\cutiles\cutile"
$env:FORGE_WS_URL = 'ws://127.0.0.1:8088'
$env:ATOM_TRAIL_ROOT = "$env:LOGOS_ROOT\.atom-trail"
```

WSL2:

```bash
export LOGOS_ROOT="/mnt/f/Users/Matthew Ruhnau/LogOS"
export RESON8_DRIVE="/mnt/f/Users/Matthew Ruhnau/My Drive/Reson8_Labs"
export FORGE_WS_URL="ws://127.0.0.1:8088"
# optional DriveFS if mounted as I:
# export GDRIVE_LABEL="/mnt/i"
```

Install once:

```powershell
pwsh -File ops\Install-LogOSShell.ps1
pwsh -File ops\Install-LogOSShell.ps1 -Wsl   # if Ubuntu/kali present
```

---

## 5. Constraint ordering by time (how to sequence work)

Use **local ATOM stamps** as the only ordering that survives context death.

| Order | Constraint | Why first |
|------:|------------|-----------|
| T0 | LOGOS_ROOT + letter truth | Wrong root → every path lies |
| T1 | Shell module / PATH | You need `logos-*` without hunting |
| T2 | coherence-mcp env (`LOGOS_ROOT`, `ATOM_TRAIL_ROOT`) | ATOM trail writes need a home |
| T3 | Local bridge **if** TUI/POP needed | Optional for pure formal work |
| T4 | Formal checks (Agda/Lean/cutile) | Theorems do not need cloud |
| T5 | Drive research read | Strategy; not a build dependency |
| T∞ | GB-06 Cloud Run | **Held** — own gate later |

Handoff docs are the **persistent queue**. If you cannot finish, write an ATOM with `next_only` one action — never a laundry list.

---

## 6. Smoke checks (honest)

| Check | Command / probe | Expect |
|-------|-----------------|--------|
| Code root | `Test-Path F:\Users\Matthew Ruhnau\LogOS\Cargo.toml` | True |
| Drive research | `Test-Path '…\My Drive\Reson8_Labs\Sheets'` | True |
| G: is not project | `Get-Volume` / label | RECOVERY — leave alone |
| Bridge | `Test-NetConnection 127.0.0.1 -Port 8088` | True only after start |
| Routes file | `9P2000.L\styx\routes.json` | present, `endpoint` 8088 |
| ZLL crate | `crates\zero_latency_ledgers\Cargo.toml` | name `reson8-zero-latency-ledgers` |

---

## 7. Pointers (do not duplicate)

| Need | Canonical file |
|------|----------------|
| Full G: era wiring narrative | `docs/sovereign-handoff/POWERSHELL-WSL2-G-DRIVE-WIRING.md` |
| Unitary Drive DSC | `docs/theory/UNITARY-MASTER-DSC-20260403.md` |
| Command surface registry | `ops/command-surface.json` |
| Shell map | `ops/LogOS.Shell.md` |
| Activation / WS history | `ops/ACTIVATION-REPORT-20260323.md` |
| Blockchain Bank ZLL story | `9P2000.L/strands/gemini/architectural_specs/BLOCKCHAIN-BANK-SPEC.md` |

---

## 8. Comments for the next human (extra mile)

- If a tutorial still says `cd G:\Reson8-Labs`, **rewrite the path in your head** — do not fight the recovery volume.  
- `reson8_UNITARY_MASTER` under My Drive can lag or diverge from git HEAD; treat it as a **mirror**, not as merge-base.  
- Zero-latency ledgers on Drive are mostly **notes**; the crate is the implementation stub.  
- Closing the bridge is fine. Leaving a stale process on 8088 is not.

**Heisenpup note:** Position high on letter map · Momentum med on 9P-vs-WS migration · Blocking none for export task.
