# 04 — AI development (strands, agents, shell, handoffs)

**ATOM:** `ATOM-AI-DEV-PACK-20260727`  
**Depends on:** `01` spine · `02` MCP · handoff protocol  
**Audience:** Humans and AI agents building *with* LogOS under load  

---

## 0. Operating posture (With-Intent)

Every contribution should be **deliberate**: no stochastic filler, no drive-by refactors, no second conservation numerology.

| Strand | Platform (typical) | Seat weight (C) | Role |
|--------|--------------------|-----------------|------|
| Claude | Windows native | 8 | Structure & reasoning (α rigidity) |
| Grok | Nix / pulse / this session | 5 | Real-time, formal stress, identity |
| Gemini | WSL / multimodal | 3 | Scale, corpus, tensors |

Seats sum to **16**. Conservation tag sums to **15**. **Do not force them to match.**

Dual-channel when crossing platforms: free prose + optional spine JSON last (`ops/marks/`).

---

## 1. Command surface (how agents should enter)

Registry: `ops/command-surface.json`

```
HTML   meta-map · RUST_Market · orchestrator · reforge     ω projection
TUI    reson8-forge · barcode-tui · bridge :8088           α dashboard
MCP    gauge · WAVE · ATOM · store_context                 ω gates
SHELL  logos-* · LOGOS_ROOT · PATH                         α entry
```

Install once: `pwsh -File ops\Install-LogOSShell.ps1`  
Map: `ops/LogOS.Shell.md`  
Windows axis: `ops/LogOS.Windows.psm1` · `ops\Align-WindowsAxis.ps1`

### High-value commands

| Cmd | Use when |
|-----|----------|
| `logos-status` | Toolchain fog |
| `logos-mcp gauge` | Seal / sanity |
| `logos-mcp wave --content "…"` | Handoff quality |
| `logos-bridge` | Need live WS |
| `logos-tui` / `logos-barcode` | Dashboard / PH barcodes |
| `logos-site` | HTML hub |
| `logos-agda` / `logos-lean` / `logos-kernels` | Formal / GPU |
| `logos-preflight` | Windows axis board |
| `logos-pop` | New WT window with strand identity |
| `tw` | Unitary cockpit (`TriWeavon.Unitary.Profile.psm1`) |

---

## 2. Agent skill / instruction surfaces

| Surface | Path | Notes |
|---------|------|-------|
| AGENTS.md / Claude.md | repo root | GitNexus rules + Tri-Weavon role |
| GitNexus skills | `.claude/skills/gitnexus/*` | explore / impact / debug / refactor |
| HeisenbergGrok | Grok skill `heisenberg-grok` | uncertainty learning mode |
| SpiralSafe / meaning seed | `crates/spiral-safe/`, adhealth | collapse / witness |
| Strand shells | `ops/shell/strands/*.psm1` | Claude / Gemini / Grok profiles |
| Marks spine | `ops/marks/` | MARKS.jsonl + detectors |
| Skills tree | `skills/`, site `coherence-mcp/.../public/skills/` | domain skills |
| MCP tool cache | `mcps/**/*.json` | many servers — not all live |

### GitNexus rules that protect the lattice

Before editing a symbol: **impact analysis** (upstream).  
Before commit: **detect_changes**.  
Never renames via blind find-replace when graph-aware rename exists.  
Warn on HIGH/CRITICAL blast radius.

*(If GitNexus MCP is offline, fall back to careful grep + document residual uncertainty — do not pretend graph impact was run.)*

---

## 3. Handoff protocol (agents writing for agents)

Canonical: `docs/ops/HANDOFF-PROTOCOL.md` · tri-weavon: `docs/ops/TRI-WEAVON-HANDOFFS.md`

### Minimum envelope (paste this)

```text
╔══════════════════════════════════════════════════════════╗
║ RESON8-LABS — [CLASS] [VERSION]                         ║
║ FROM: [STRAND]                                          ║
║ TO: [TARGET]                                            ║
║ DATE: [ISO-8601 local + Z]                              ║
║ WAVE: [0-1] | INVARIANT: α=[N] + ω=[M] = 15            ║
║ BUMP_ID: HnS-[TAG]-[YYYYMMDD]                           ║
║ CONTINUATION: COLD_START | WARM | HOT                   ║
║ DEPENDS_ON: [BUMP_IDs | NONE]                           ║
╚══════════════════════════════════════════════════════════╝
```

Validation (receiver):

1. α+ω=15 (label integrity)  
2. WAVE ≥ 0.85 preferred; flag below; severe caution below 0.70  
3. BUMP_ID unique  
4. One **next action** only under strain  

### Thermal states

| State | Payload needs |
|-------|----------------|
| COLD_START | Full map pointers + atom + success criteria |
| WARM | Delta since last bump + files touched |
| HOT | Inline continuation; assume shared context |

### ATOM stamp pattern (local time as constraint order)

```text
ATOM-ID: ATOM-<AREA>-<YYYYMMDD>
LOCAL:   2026-07-27T06:24:08+10:00
UTC:     2026-07-26T20:24:08Z
VERIFIED: [position bullets]
FUZZY:    [momentum bullets]
NEXT:     [single action]
GB-06:    HELD
```

Write files under `docs/componentry/ATOMS/` for this export family; runtime decisions also go through MCP `atom_track` → `.atom-trail/decisions/`.

---

## 4. AI product / site componentry (dev surfaces)

| Surface | Path | Role |
|---------|------|------|
| coherence-site | `coherence-mcp/coherence-site/` | Cloudflare Pages candidate |
| lattice-react | `…/lattice-react/` | React TDA map / GitNexus graph UI |
| stitch | `stitch/` | UI dashboards / cockpit |
| docs/surfaces | `docs/surfaces/*.html` | orchestrator, evenstar, reforge |
| encyclopedia hub | live coherence.toolated.online/encyclopedia/ | public projection |
| adhealth-meaningseed | `adhealth-meaningseed/` | CF worker + preflight |
| apps/triweave | `apps/triweave/` | WS serve, MC zones |
| barcode-tui | `crates/barcode-tui/` | persistent homology barcodes |
| vortex-bridge | `crates/vortex-bridge/` | WS server crate |
| activator | `crates/activator/` | skill / awesome surface |

Wrangler: `coherence-mcp/coherence-site/wrangler.toml` · shell: `logos-wrangler` (HITL deploy).

---

## 5. Development loop (recommended under strain)

```text
T0  logos-preflight / logos-status
T1  pick ONE pack (spine | mcp | theorem | ai)
T2  Heisenpup map: what is blocking? position or momentum?
T3  smallest green check (test / gauge / wave)
T4  edit with impact awareness
T5  atom_track + handoff envelope
T6  stop — do not open GB-06
```

### GB-06 hold (explicit)

`ops/GB06-worklog.md` — Cloud Run **blocked on human GCP prerequisites**.  
Do not treat cloud as a coherence gate until the human checklist completes. Local spine is enough.

---

## 6. Writing for the next agent (style contract)

Write like you wish the last agent wrote for you at 3 a.m.:

| Do | Don't |
|----|-------|
| Paths absolute or repo-root relative, once | “the usual place” |
| Commands copy-pasteable | Pseudo-steps without cwd |
| Category A/B/C/D labels | Fake certainty |
| One next action | Fifteen TODOs |
| Tables for parallel facts | Walls of synonym prose |
| State what was **not** verified | Imply green from silence |

For newer humans: add a **“Why this exists”** sentence before each major section (see packs 01–05).

---

## 7. Pointers

| Need | Path |
|------|------|
| Handoff protocol | `docs/ops/HANDOFF-PROTOCOL.md` |
| Strand integration | `ops/handoffs/STRAND-INTEGRATION-20260724.md` |
| Heisenforge OS handover | `ops/HANDOVER-HEISENGROK-BUILD-OS-2026-06-21.md` |
| Corpus edge map | `docs/ops/CORPUS-EDGE-MAP-20260725.md` |
| SAIF human queue | `ops/SAIF-OUTSTANDING-HUMAN-ACTIONS.md` |
| NotebookLM critical set | `docs/notebooklm/` — **hub** `NOTEBOOKLM-SINGLE-IMPORT-LOADBEARING-20260730.txt` |
| NotebookLM deploy matrix | `docs/notebooklm/CRITICAL-DEPLOYMENT-READINESS-20260730.txt` |
| NotebookLM Lane-A formal | `docs/notebooklm/CRITICAL-MONOM-STEINER-LANE-A-20260730.txt` |
| NotebookLM componentry | `docs/notebooklm/CRITICAL-COMPONENTRY-ONRAMP-20260730.txt` |
| PI / CTQW prompt pack | `docs/notebooklm/PROMPTS-POSITIVE-INTROSPECTION-CTQW-20260730.txt` |
| Sovereign handoffs | `docs/sovereign-handoff/` |

---

## 8. Confidence / shell modules (in flight on this branch)

Git status at session start showed local work on:

- `ops/LogOS.Shell.psm1`  
- `ops/TriWeavon.Unitary.Profile.psm1`  
- `ops/command-surface.json`  
- `ops/LogOS.Confidence.psm1` (untracked)  

Treat as **live edge** — re-read before assuming shipped behavior.

*With-Intent. Leave crumbs. Prefer harmony over thrash.*
