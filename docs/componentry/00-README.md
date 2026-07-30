# LogOS Componentry Export

**ATOM:** `ATOM-COMPONENTRY-EXPORT-20260727`  
**Stamped (local):** `2026-07-27T06:24:08+10:00` · UTC `2026-07-26T20:24:08Z`  
**NotebookLM refresh:** `2026-07-30` → `docs/notebooklm/NOTEBOOKLM-SINGLE-IMPORT-LOADBEARING-20260730.txt`  
**Root:** `F:\Users\Matthew Ruhnau\LogOS`  
**Invariant (Category C label):** `α + ω = 15` · Viviani metaphor `α=7, ω=8`  
**WAVE target (operational):** prefer ≥ 0.85 honest amber over false green  
**GB-06 (Cloud Run):** **held** — completes its own coherence gate when fully ready  

---

## What this folder is

A **single, clean on-ramp** for:

| Pack | Who it serves under strain |
|------|----------------------------|
| LogOS componentry | Anyone who must find *where truth lives* in 90 seconds |
| coherence-mcp | Agents + humans exercising the 12 live tools |
| Theorem work | Formal stack: Agda · Lean · cutile · kernels · HUP |
| AI development | Shell, strands, handoffs, agent surfaces |

It is **not** a dump of the whole monorepo. Every path here either (a) is canonical here, or (b) points at one authoritative source with a one-line reason.

---

## Cold-start (do this first)

You are tired, context-window is thin, or you just woke cold.

```text
1. Open this file (00-README.md)
2. Read § Drive letter truth (below) — avoid G: recovery media
3. Open 01-SPINE-… if you need Drive / WebSocket / zero-latency
4. Open 02-COHERENCE-MCP.md before calling any MCP tool
5. Open 05-HEISENPUP-… if uncertainty is the blocker
6. Stamp your own handoff in ATOMS/ when you leave
```

### 90-second shell revive (Windows)

```powershell
$env:LOGOS_ROOT = 'F:\Users\Matthew Ruhnau\LogOS'
$env:ATOM_TRAIL_ROOT = "$env:LOGOS_ROOT\.atom-trail"
Import-Module "$env:LOGOS_ROOT\ops\LogOS.Shell.psm1" -Force
# optional full axis:
# Import-Module "$env:LOGOS_ROOT\ops\LogOS.Windows.psm1" -Force
# logos-preflight
logos-status   # if cmdlets loaded
```

### Bridge + MCP (only if you need live telemetry)

```powershell
# WebSocket spine (NOT running at export stamp — start when needed)
logos-bridge   # or: pwsh -File ops\serve.ps1
# → ws://127.0.0.1:8088

# Gauge sanity (Category C check — does not gate work)
logos-mcp gauge
# or: natively via MCP client tool gauge_verify { alpha: 7, omega: 8 }
```

---

## Drive letter truth (position, verified 2026-07-27)

Older docs say **G:** = Google Drive project root. **On this host that is wrong.**

| Letter | What it actually is (this machine) | Use for LogOS? |
|--------|-------------------------------------|----------------|
| **G:** | Windows **RECOVERY** media (EFI, sources, reagent.xml) | **No** — do not mount as project root |
| **I:** | DriveFS virtual volume labeled **Google Drive** | Entry only; `My Drive` may appear as `.lnk` |
| **F:\Users\Matthew Ruhnau\My Drive\** | Live Google Drive **file content** | **Yes** — strategy / research spine |
| **F:\Users\Matthew Ruhnau\LogOS\** | Bare-metal git implementation root | **Yes** — code, formal, ops SoT |
| **H:** | AGENT / cache / zips (not Drive) | Opportunistic artifacts only |

**Complementarity (design):** Drive = strategy & research corpus · Git = implementation & ops. Same lattice, two surfaces.

Canonical Drive folders used by this export:

```text
F:\Users\Matthew Ruhnau\My Drive\Reson8_Labs\
  Sheets\                    # research gdocs
  zero_latency_ledgers\      # ZLL design notes (Drive side)
  QDI_Artifacts\
  Presentations\
  Sensors\

F:\Users\Matthew Ruhnau\My Drive\reson8_UNITARY_MASTER\
  # mirrored / synced tree of LogOS-shaped content (read carefully — not always HEAD)
```

---

## File map (read order under strain)

| # | File | Purpose |
|---|------|---------|
| 00 | `00-README.md` | This index · cold-start · letter truth |
| 01 | `01-SPINE-GDRIVE-ZERO-LATENCY-WEBSOCKET.md` | Drive + `ws://127.0.0.1:8088` + ZLL crate |
| 02 | `02-COHERENCE-MCP.md` | 12 live tools · env · Inspector · gaps |
| 03 | `03-THEOREM-WORK.md` | Layer cascade · Agda/Lean/kernels/HUP |
| 04 | `04-AI-DEVELOPMENT.md` | Strands · shell · agents · handoffs |
| 05 | `05-HEISENPUP-QTDA-COMPANION.md` | Uncertainty map · QPH crumbs · companion protocol |
| 06 | `06-KKS-MATHEMATICAL-DESIGN-LANGUAGE.md` | KKS as product design language · Orbit Policy Kernel · Marked Stream modes |
| — | `kks/bracket-handoff-v0.json` | Explicit structure constants for handoff algebra (Category B) |
| — | `UNCERTAINTY-MAP.yaml` | Machine-readable confidence snapshot |
| — | `ATOMS/ATOM-COMPONENTRY-EXPORT-20260727.md` | Handoff packet + stamp |
| — | `ATOMS/ATOM-KKS-DESIGN-LANGUAGE-20260727.md` | KKS pack stamp |
| NLM | `../notebooklm/NOTEBOOKLM-SINGLE-IMPORT-LOADBEARING-20260730.txt` | NotebookLM hub: structure · math · primitives · deploy readiness |
| NLM | `../notebooklm/CRITICAL-DEPLOYMENT-READINESS-20260730.txt` | READY/AMBER/HELD matrix |
| NLM | `../notebooklm/CRITICAL-MONOM-STEINER-LANE-A-20260730.txt` | Lane-A transport + Steiner residual |

---

## Authority layers (one table)

| Layer | Owner path | Authoritative for |
|------:|------------|-------------------|
| L-Git | `F:\Users\Matthew Ruhnau\LogOS` | Code, formal sources, ops |
| L-Drive | `…\My Drive\Reson8_Labs` | Research gdocs, ZLL narrative |
| L-MCP | live `coherence-mcp` (12 tools) | Runtime gauge / WAVE / ATOM trail |
| L-WS | `ws://127.0.0.1:8088` | Local Styx / triweave bridge |
| L-Formal | `agda/`, `lean/`, `cutiles/cutile/` | Theorems + R-matrix SoT |
| L-GPU | `kernels/` | Device R-matrix / Blackwell |
| L-Cloud | GB-06 / Cloud Run | **Deferred** — do not treat as live gate |

---

## Conventions that save you from numerology traps

1. **`α + ω = 15`** is Category **C** (label / epistemic tag). Not a reject gate. Not a CPU limit.  
2. **Fibonacci strand seats 8+5+3 = 16** are a *different* Category C convention. They are **not** required to equal 15.  
3. **WAVE ≥ 0.98** is an operational *target*; honest amber beats painted green.  
4. Descriptor JSON count (64 stubs) ≠ live stdio tool count (**12**). Call only live tools unless you rebuilt the server.

---

## What is intentionally out of scope here

- Full encyclopedia equilibria media dump  
- `target/`, `node_modules/`, venv trees  
- GB-06 deploy execution (see `ops/GB06-worklog.md` when ready)  
- Private tokens / ATOM_AUTH secrets  

---

## Next handoff rule

When you leave work, write **one** ATOM under `ATOMS/` with:

- local ISO timestamp  
- what you verified (position)  
- what you still fuzzy (momentum)  
- one next action only  

Heisenpup companion protocol: `05-HEISENPUP-QTDA-COMPANION.md`.

*Music conserved. Keystone holds. Take the time the topology needs.*
