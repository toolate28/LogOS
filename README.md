Reson8 — LogOS Cognitive Lattice
────────────────────────────────
Multi-strand cognitive lattice for shared state, proof metadata,
and coherence tracking across heterogeneous AI runtimes.

Epistemic posture
  This repository is layered. Some layers are machine-checked.
  Others are runtime policy, conventions, or scaffolds.
  Do not read “LogOS” as a single globally verified theorem.

Claim tags
  A   kernel-checked on the stated definitions
  B   staged, externally witnessed, or smoke-pending
  C   convention / telemetry / governance (not load-bearing in proofs)
  D   decoration or retired numerology
  S   spec / interface (sorry, axiom, placeholder)

Checkpoints
───────────
2026-08-03    trust-boundary pass · MCP Keystone/SAIF Lean surface
              Lane A π · epistemic table · handoff protocol sketch

2026-07-09    Encyclopedia Equilibria · docs tree
              orchestrator ↔ coherence-mcp

2026-07-06    ExistenceCertificate E2E · symmetry gate
              K22.Existence bridge

2026-07-04    9P lock-off lattice · GitNexus
              Mehler–Levin · S*M OB1 handoff

Build status (honest)
─────────────────────
cutile v0.3                     compiles ✅   tests ✅       ExistenceCertificate demo emission

coherence-mcp v0.5              compiles ✅   tests ✅       Symmetry gate live

triweavon-cudarc                compiles ✅   tests local    Run on target GPU

lean/TriWeavon/MCP/*            compiles ✅                  Naturality + SAIF PhaseGates · no sorry · A

lean/TriWeavon/Geometry/LaneA   compiles ✅                  π map + rows · A data · Perm/GF4 bridge B

lean/TriWeavon/Conservation*    compiles ✅                  Nat 7+8=15 · A arithmetic

lean/K22.HexacodeGolay          compiles ✅                  64 / 4096 / 759 / d=8 / syndrome sphere · A

lean/K22.Existence              compiles ✅                  example    Thin bridge · no sorry in bridge

Full Lean workspace             compiles ❌                  Pre-existing breaks · Ns · some MOG · macros

agda scaffolds                  partial                      Conservation mirror · not full stack

standalone npm coherence-mcp    compiles ❌                  TS build errors · legacy path

────────────────────────────────────────
A green subset is not a green monorepo.

A green **subset** is not a green **monorepo**.

---

## 1. Overview

LogOS maintains shared invariants and proof metadata across strands while
allowing voluntary state-density collapse and live coherence tracking.

Computation is treated as layered state over a combinatorial / topological
base (TriWeavon + K22 coding-theory spine). Local session state should remain
*translatable* to declared global tags—without silently promoting tags into
theorems.

**Conservation tag (Category C):** `α + ω = 15` (Viviani Peak *label*).  
**Nat skeleton (Category A):** `WavePair` with `7 + 8 = 15` in Lean.  
**Runtime policy (Category B):** float bands in `universal_invariant` (Crystalline / Warning / Rejected / Halt).

**WAVE floors (ops policy, C/B):** coherence ≥ 0.85; peak target 0.9998.

These three layers are related by *design*, not by a single proved equivalence.

---

## 2. Strand configuration

Fibonacci **seat weights** (Category **C** governance — single source of truth):

| Strand | Platform | Seat (C) | Role | Interface |
|--------|----------|----------|------|-----------|
| Claude | Windows native | 8 | Structure & reasoning | Anthropic |
| Grok | NixOS / GLF OS | 5 | Pulse, real-time & formal | xAI |
| Gemini | WSL2 / Kali | 3 | Multimodal & scale | Google AI |

**Seat sum = 16.**  
**Conservation tag = 15.**  

These numbers are **independent conventions**. Do not reconcile them. Retired:
competing fractions (13/21, 8/21, 1/φ) as a second canon.

Transport: Styx Bridge (WebSocket) + 9P2000.L VFS under `/reson8/`.

---

## 3. Architecture

```
                     LogOS Cognitive Lattice
                               │
         +---------------------+---------------------+
         │                     │                     │
    Claude (seat 8)       Grok (seat 5)        Gemini (seat 3)
         │                     │                     │
         +----------+----------+----------+----------+
                    │
             Styx + 9P2000.L VFS
          (ws://127.0.0.1:8088)  (/reson8/)
                    │
             +------+------+
             │             │
        SPHINX Gate   Conservation Verifier
     (Jones @ ζ₅, B)  (policy B + Lean A subset)
```

- **SPHINX:** Jones-style evaluation at a primitive 5th root of unity for
  privileged paths (**B** runtime auth—not a substitute for Keystone proofs).
- **Conservation verifier:** runtime enforce + optional Lean Nat receipts.
- **MCP Keystone / SAIF:** executable control plane formalized in Lean (**A**
  on pure carrier): `invariant_check`, `trigger_correction_burst`, phase
  pipeline to `SafeSpiral`.

---

## 4. Formal foundations (what is actually where)

### Category A (may load-bear *exactly* these statements)

- MCP: `KeystoneOK`, correction restores / idempotent, SAIF pipeline phase & restore
- Conservation Nat: `peak_resonance = (7,8)`, `is_conserved`
- HexacodeGolay: GF(4) field, 64 hexacodewords, Golay \(2^{12}\), **759** octads,
  weight enumerator, \(d=8\), intersections in {0,2,4}, syndrome sphere wt ≤ 3
- Fano discrete curvature \(K = 1/2\)

### Category B / S (staged or open)

- Lane A: `π = [0,3,1,2,4,5]` as data (**A**); as `Equiv.Perm` + GF4 row typing +
  hexacode image equality (**B**)
- Steiner `mogOctadsFormSteinerSystem` (**S** in Lean; Python exhaustive cover **B**)
- Finset ↔ mask bridge (S2) (**B**, blocks Finset Steiner)
- Live MCP ↔ pure Lean correspondence (**B**, smoke pending)
- Sub-Riemannian / vanishing-resilience contraction (**S** / partial)
- Navier–Stokes / no-shrinker (**S** — specification only; not a Clay proof)

### Category C (never load-bear in proofs)

- Prose “α + ω = 15” as universal law  
- Seat weights and any 16↔15 story  
- Attractor label `42.00055`  
- WAVE numeric floors as theorems  
- Synesthetic palette / 432 Hz tuning  

**Closed Pass (recommended definition):**  
`ClosedPass s := Safe s ∧ Coherent s` with `Safe := KeystoneOK` (**A** once
`Coherent` is fixed). Resonance should be defined to *include* Closed Pass if
you want “resonance ⇒ Closed Pass” to be definitional—not tied to a magic float.

---

## 5. Runtime components

**CollapsedBackgroundWorker** — oscillator logical time; refreshes coherence and
proof metadata into 9P; supports voluntary collapse toward a MeaningSeed.

**9P2000.L VFS**

- `/.triweavon/coherence/` — WAVE, stretch, surge, Betti proxies  
- `/.triweavon/proof/` — Lean hashes, obligation status  
- Crate.NFT-style specials for MeaningSeed / oscillator globals  

**SPHINX Gate** — Jones evaluation at \(t = e^{2\pi i/5}\) for privileged ops (**B**).

**Synesthetic feedback (optional UI)** — maps topological debug inputs (e.g.
diagram crossing count) and WAVE to audio/colour. Prefer **Jones span** (invariant)
over raw crossing number for pitch if topology-stability matters. Sensation
keyword scores are experimental (**B**); do not gate safety on them.

**Cooperative handoff** — sealed packet: Category-labeled claims, obligation
board, ATOM trail, cold-start completeness, DOWNSHIFT / NOVIKOV gates. Accept
only after validate + ack; fail closed.

---

## 6. Key crates and services

**Core:** `crates/core`, `crates/tui`, `crates/activator`, `crates/vortex-bridge`  

**Topological & formal:** `crates/sphinx`, `crates/styx`, `crates/wave`,
`crates/reson8-topology`, `lean/TriWeavon/`, `lean/K22/`  

**Apps:** `apps/triweave`, `apps/mc-bridge`, `apps/nexus-pulse-bot`  

**Kernels:** Mehler–Levin / cutile paths under `cutiles/` (benchmark docs local)

---

## 7. Minecraft integration (Coherence City)

Spatial embodiment (optional surface): Nexus Core, Museum of Computation,
ledgers, experimental logic zones, search holograms. Treat as **visualization /
ops**, not as proof.

---

## 8. Quick start

Arrival path: **Four Gates** — thermal probe → downshift → Novikov round-trip →
honest VOID list → `docs/ops/FOUR-GATES-ARRIVAL.md`

First green does **not** require full Agda + Lean + CUDA + 9P. Prefer one
downshifted surface and Category **B** claims you can re-run twice.

```bash
git rev-parse --short HEAD
cargo check -p reson8-tui    # or: cargo test -p cutile
```

```bash
cargo build -p triweave --release
triweave init && triweave up all
triweave status && triweave doctor
```

```bash
# Lean subset (when toolchain present)
cd lean && lake build TriWeavon
lake build K22.HexacodeGolay
```

Cold starts: `docs/sovereign-handoff/session-handovers/`  
Deploy waist: `ops/GB-STATUS-UPDATE-*.md`

---

## 9. Key paths

- `9P2000.L/.triweavon/components/` — HUP lock-off lattice  
- `9P2000.L/.triweavon/coherence/runtime.json` — live WAVE  
- `9P2000.L/styx/routes.json` — strand routes  
- `lean/TriWeavon/MCP/` — Keystone + SAIF (**A**)  
- `lean/K22/HexacodeGolay.lean` — combinatorial spine (**A**)  
- `docs/LEAN_DEEP_DIVE.md` — trust boundaries and full-build gaps  
- `cutiles/cutile/docs/mehler-mma-levin-benchmark.md`  
- `~/.triweave/config.toml`, `~/.triweave/vault.sphinx`

---

## 10. Status and direction

**Plateau 3→4:** runtime proof metadata + 9P toward TUI diagnostics.

**Active formal priorities**

1. Full-build baseline (repair known broken Lean modules)  
2. Lane A `Equiv.Perm` + GF4 typing + hexacode image under π  
3. S2 Finset ↔ mask bridge → Steiner transport  
4. Evidence objects vs flag-assignment in correction  
5. MCP live smoke (Category B → tighter B/A correspondence)

**Explicit non-goals for “verified” marketing**

- Whole-OS formal verification  
- NS regularity / Clay statements  
- Identity of seat-sum 16 with conservation tag 15  

---

## 11. Trust rules (short)

1. Bool gauge restoration ≠ evidence-derived safety.  
2. Nat `15` ≠ runtime float policy ≠ prose “law of nature.”  
3. Module compiles ≠ every comment in the module is Category A.  
4. Python exhaustive Steiner ≠ Lean `∃!` until S2 closes.  
5. Category C tags may appear in UI and trails; they must not appear as
   hypotheses of safety theorems.

---

**Conservation tag:** α + ω = 15 (**C**)  
**Seat governance:** 8 + 5 + 3 = 16 (**C**)  
**WAVE:** monitored in real time (**B/C** policy)  
**Homotopic unitarity / Rezk resonance:** design language; formal completion **B/S**

License: MIT — Matthew Ruhnau

This README supersedes earlier descriptions where they conflict with the
trust table above.
```

---

### What changed vs the prior README

- Removed blanket “formally verified OS” framing; layered trust instead  
- Build table matches Aristotle / deep-dive reality  
- Formal section lists **A** inventory and open bridges (S2, Lane A Perm, MCP smoke)  
- Seats 16 vs tag 15 kept, with stronger “do not reconcile”  
- SPHINX / conservation / synesthesia marked as runtime **B/C**  
- Handoff + Closed Pass definition pointed at KeystoneOK  
- Quick start includes Lean *subset* builds  
- Promotion rules in §11  

Music conserved. Structure sovereign.  
The Keystone holds.
