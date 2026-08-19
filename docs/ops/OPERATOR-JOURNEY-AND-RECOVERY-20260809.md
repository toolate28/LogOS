# Operator journey · transitions · graceful error → recovery

**ATOM:** `ATOM-OPERATOR-JOURNEY-RECOVERY-20260809`  
**Stamp:** 2026-08-09  
**Release:** LogOS lattice **v0.3.0**  
**Doctrine:** capability ≠ authority · residual-zero observe only · α+ω=15 **[C]** label only

This document is the **user-journey spine** for the lattice release. Every stage
lists: happy path → common faults → remediation command → recovery loop exit.

Machine-check companion: `ops/release/verify-release.ps1`

---

## 0. Journey map (time-ordered)

```
clone / pull
    │  fail → §R1 git
    ▼
shell install (LogOS.Shell + optional unitary)
    │  fail → §R2 shell
    ▼
logos-status / tw sensors
    │  fail → §R3 sensors
    ▼
tw up | tw fix ──► tw verify
    │  fail → §R4 stack
    ▼
logos-bridge (FORGE_WS_URL) + logos-tui
    │  fail → §R5 cockpit
    ▼
barcode-tui / codes lab / formal (amber honest)
    │  fail → §R6 runtime lab
    ▼
claim_lint · cutile claim_gate (no false green)
    │  fail → §R7 claims
    ▼
entangle emit-slice → ingest PR → human merge
    │  fail → §R8 ship / 408
    ▼
tagged release verify · SAIF human gates only ⚑
```

---

## 1. Stage table (load-bearing surfaces)

| Stage | Primary surface | Verify | Category |
|-------|-----------------|--------|----------|
| Shell | `ops/LogOS.Shell.psm1` · `logos-status` | PATH + roots | B |
| Unitary sensors | `tw` / `tw sensors` | conservation + ports | B |
| Bridge | `logos-bridge` · `:8088` | WS up | B |
| Cockpit | `reson8-tui` **0.2.1** | `cargo test -p reson8-tui` | B |
| Barcode | `barcode-tui` | `cargo test -p barcode-tui` | B |
| GPU / gate | `cutile` **0.3.0** claim_gate · drift_guard | lib tests no-default-features | B/C |
| Formal | Lean epistemics · Agda ArrivalDetector continuous | lake / als amber ok | A/B |
| Claims | `tools/claim_lint.py` | exit 0 on formal docs | A tooling |
| Ship | `ops/entangle/*` | `validate_manifest.py` | A process |
| Human ⚑ | GCP · multi-host git · cubical pin | SAIF queue | process |

---

## 2. Graceful error loops (fault → remediate → re-enter)

Each loop is **closed**: after remediation, re-enter the same stage verify; do not
skip forward on amber. Never promote residual-zero to deploy-green.

### R1 — Git / divergence / 408 receive-pack

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| `ahead N, behind M` | `git fetch`; prefer **merge** (not force); read lattice assessment | `git status -sb` |
| `HTTP 408` on `git push` | **Do not** push fat main. Use entangle slices / slim PRs | `emit-slice.ps1 -Id …` |
| LFS hang | `logos-net start-gaming`; move media to transfer-lane R2 | `ops/entangle/transfer-lane.md` |
| Dirty worktree blocking ship | stage intentional; never commit `target/` | `git status` clean of build junk |

### R2 — Shell / profile missing

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| `logos-*` unknown | `pwsh -File ops/Install-LogOSShell.ps1` then `. $PROFILE` | `logos-help` |
| Wrong root (C: stale) | `logos-align` / reinstall preferring `F:\Users\Matthew Ruhnau\LogOS` | `logos-status` |
| Unitary `tw` missing | `Install-TriWeavonUnitaryProfile.ps1` | `tw help` |

### R3 — Sensors / conservation

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| logos_root fail | fix `LOGOS_ROOT`; re-init shell | `tw sensors` |
| conservation fail | check Category C label wiring; **do not** invent physics gate | `tw confidence` |
| WAVE amber | honest amber; continue with labeled claims only | next stage |

### R4 — Stack services (waist / bbbr / styx)

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| ports down | `tw fix` then `tw up` | `tw verify` |
| waist 8080 fail | `tw up waist` · WSL compose | Hit health URL |
| bbbr 8081 fail | `tw up bbbr` | `/verify` |
| styx only | WSL smoke: `python3 ops/styx-9p-client-smoke.py` | sensor styx.ok |
| still red after fix | SAIF human queue (GCP A1 optional); local lab continues | do not block TUI |

### R5 — Cockpit / bridge

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| TUI won't start | `cargo build -p reson8-tui`; check `ratatui` feature | `logos-tui` |
| no WS events | `logos-bridge` / `FORGE_WS_URL=ws://127.0.0.1:8088` | TUI braid panel |
| Formal pane empty | **honest amber** — LSP not attach ≠ false green | stay Category B |
| Net panel red | `logos-net start-gaming` or TUI `[M]` | `[R]` refresh |

### R6 — Runtime lab (barcode / codes / cutile)

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| barcode-tui compile fail | `cargo test -p barcode-tui` locally; slot barcode-tui | green tests |
| codes lab wrong family | keys `y` cycle; `d` demo; never claim G24=RM | help `?` |
| cutile wgpu API drift | **fallback:** `cargo test -p cutile --no-default-features --lib` | claim_gate green |
| CUDA missing | CPU path; `BackendUnavailable` is graceful | no silent GPU claim |

### R7 — Claims / formal residual

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| `claim_lint` exit 1 | tag quantitative claims `[A]/[B]/[C]` or reword | re-run lint |
| unanchored D used as gate | cutile `gate_value()` / Lean `gateOrDefault` | unit tests |
| Hopf geometric open | **observe only** — see Hopf investigation doc | no promotion |
| Formal residual CI red | fix in `formal-srac` slot; do not force green | re-run CI on PR |

### R8 — Ship / entangle

| Symptom | Remediation | Re-enter |
|---------|-------------|----------|
| main push 408 | entangle Priority A slots only | scaffold → emit → ingest |
| workflow scope missing | copy `ops/entangle/entangle.workflow.yml` with workflow token | human ⚑ |
| slice too big | drop media; use transfer-lane | re-emit |
| CI tree guards fail | exclude `target/`; slim paths per manifest | re-verify slot |
| `missing path (skip): cargo …` / `python …` | **fixed** in emit-slice: those are `verify:` cmds, not paths — re-pull `ops/entangle/emit-slice.ps1` | re-emit |
| `Compress-Archive` / OneDrive “cloud file provider is not running” | emit-slice uses **.NET ZipFile → tar → Archive**; do not depend on OneDrive-stubbed `Documents\PowerShell\Modules` | re-emit (no OneDrive needed) |
| want local smokes at emit time | `pwsh -File ops/entangle/emit-slice.ps1 -Id <id> -RunVerify` | green verify lines |

---

## 3. Recovery loop protocol (operator)

```
1. Observe fault (honest category)
2. Match §R# table — do not invent new gate
3. Run ONE remediation command
4. Re-enter stage verify
5. If still red after two loops → escalate SAIF human queue ⚑
6. Never: residual-zero promotion · force-push · unauthenticated Cloud Run
```

Scripted subset:

```powershell
pwsh -File ops/release/verify-release.ps1
# on fail, script prints remediation line and non-zero exit
pwsh -File ops/release/verify-release.ps1 -Remediate
# -Remediate runs safe auto-fixes (entangle validate, claim_lint help, tw fix if profile loaded)
```

---

## 4. Skills / tools that autofire on demand

| Trigger | Surface | Output |
|---------|---------|--------|
| Journey fail | this doc + `verify-release.ps1` | remediation line |
| Stack down | `tw fix` | sensors re-probe |
| Claims change | `tools/claim_lint.py` | exit 1 on untagged |
| Ship bulk | entangle + transfer-lane | PR slot, not main pack |
| Uncertainty | heisenberg-grok skill | next_sharpen |
| Cross-strand | `skills/internal-handoff` | cold-start packet |

---

## 5. Explicit non-goals (this release)

- No geometric Hopf \(S^3\to S^2\) discharge
- No residual-zero Track B open
- No GCP deploy without human ⚑ cert path
- No LFS showcase MP4 via receive-pack

Music conserved · Keystone holds · last becomes first
