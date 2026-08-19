# Lattice activation — 2026-08-15

**ATOM:** `ATOM-LATTICE-ACTIVATE-20260815`  
**Category:** B (runtime probe) · Lean skeletons A · α+ω=15 C only  
**Surfaces:** `logos-activate` · `reson8-tui` `[A]` · WSL `logos-lattice`

Make **apps / cutiles / crates / kernels / ops** reachable from the same
command surface as `logos-tui` and `$PROFILE` / WSL `.zshrc`.

## Probe (honest)

```powershell
. $PROFILE          # or: Import-Module ops\LogOS.Shell.psm1; Initialize-LogOSShell
logos-activate      # board — no cargo
logos-activate -Check   # cargo check cutile + reson8-tui + reson8-triweave + spiral-safe
logos-lattice       # alias
cd-apps; cd-cutiles; cd-crates; cd-kernels; cd-ops
logos-tui           # status strip shows lat n/5 · key A re-probes
```

```bash
# WSL2 after Install-LogOSShell.ps1 -Wsl
source "$LOGOS_ROOT/ops/wsl/logos-env.sh"
logos-lattice
```

## Interweave (thin — no fat copy)

| Name | In-tree | Sibling |
|------|---------|---------|
| coherence-mcp | `crates/coherence-mcp` · site | `../coherence-mcp` (`COHERENCE_MCP_ROOT`) |
| SpiralSafe | `crates/spiral-safe` | `../SpiralSafe` |
| quantum-redstone | TUI `phase_evolution` · `qr_meta` · Lean `QuantumRedstone.lean` | `../quantum-redstone` or `../HOPE-AI-NPC-SUITE/quantum-redstone` |
| HOPE-AI-NPC | `apps/mc-bridge` | `../HOPE-AI-NPC-SUITE` |

TUI Quantum layout is the QR×SPHINX table. HOPE mcfunctions
(`place_hadamard_gate`, `place_cnot_gate`, …) teach the same four gates.

## Lean 4

```
cd lean
lake build TriWeavon
```

Green subset in `lakefile.lean`: QuantumRedstone · LatticeLayers · ConservationInvariant.
Epistemics / SubRiemannian / NS are **not** in this lib until they compile.

Four QR constructors and five Layer constructors are Category **A**.
Hz / datapack / WAVE floors stay **C**. Do not promote.

## Ship

Entangle slots `ops-portable-root` (shell/WSL/README) and `reson8-tui`
(TUI + QR intent). Formal Lean files ride `formal-srac`.
**No fat `git push origin main`.**

## CI note (this host, 2026-08-15)

- CodeQL run `31847943597` — success (attempt 2, Rust analyze)
- CODEX / Agentic MLOps `31847943519` — success
- `ci-policy` `31860185340` — **0 jobs**, same-second failure (path-filter /
  re-run ghost). Not an action-pin fail. Re-dispatch only if workflow files move.
