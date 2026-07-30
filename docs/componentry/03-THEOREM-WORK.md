# 03 — Theorem work (formal + executable cascade)

**ATOM:** `ATOM-THEOREM-PACK-20260727`  
**Depends on:** `LAYER-CASCADE-MAP` (canonical) · this file is the **strain-optimized** view  
**Invariant tag (C):** `α + ω = 15`  
**Audience:** Anyone opening Agda/Lean/CUDA cold and needing the *chain*, not the whole lattice  

---

## 0. The one diagram that matters

```
Empirical / notebook (L1)
        ↓
Rust R-matrix SoT (L2 cutile)  ←── prefer this for numbers
        ↓
CUDA (L3) · WGSL (L5) · C++ host (L9)
        ↓
Lean (L4) · Agda (L6)  ←── prefer these for proofs
        ↓
Receipts + certificates (L12)
        ↓
Handoffs + MCP seals (docs / atom_track)
```

**Canonical long map:** `docs/sovereign-handoff/LAYER-CASCADE-MAP.md`  
**Overlays:** `docs/sovereign-handoff/overlays/`  
**Mehler DAG:** `docs/sovereign-handoff/mehler-serrescarr-convergence.dag.yaml`

---

## 1. Layer table (materialized on disk)

| L | Tech | Path | What “done” means |
|---|------|------|-------------------|
| 1 | Qiskit notebook | `notebooks/triweave-backends.ipynb` | conservation loop + bitstring |
| 1b | Rust notebook | `notebooks/gpu/Agent_M24_RMatrix.ipynb` | nalgebra R-matrix |
| 2 | **cutile Rust** | `cutiles/cutile/src/core/r_matrix.rs` | **Canonical matrix + tests** |
| 3 | CUDA | `kernels/fundamental_r_matrix.cu` | device matrix |
| 4 | Lean 4 | `lean/TriWeavon/`, `lean/K22/` | `is_conserved`, K22, existence |
| 5 | WGSL | `kernels/fundamental_r_matrix.wgsl` | WebGPU |
| 6 | Agda | `agda/src/TriWeavon/**` | HITs, Mehler, SerreScarr, Tomczak |
| 7 | Results | `notebooks/triweave_backend_results/` | JSON receipts |
| 8 | Docs | `docs/sovereign-handoff/` | survival maps |
| 9 | C++ host | `cutiles/r_matrix_host.cpp` | launch + CPU fallback |
| 10 | Bridge note | `lean/AgdaLeanBridge.md` | name map Lean↔Agda |
| 11 | C ABI | `kernels/r_matrix_interface.h` | shared interface |
| 12 | Orchestrator | `notebooks/verification_orchestrator.ipynb` | emit certs |
| 13–20 | HUP M1–M3 | `hup/**` | Mirage · Redox · RVM |
| 21 | Mehler DAG | yaml + Agda SubRiemannian | formal dependency seal |

---

## 2. Structural R-matrix (identity to protect)

All executable layers should realize the same skeleton:

```
[ q     0      0     0  ]
[ 0    1/q   1-q²    0  ]
[ 0     0      q     0  ]
[ 0     0      0    1/q ]
```

If two layers disagree: **cutile tests win for numbers**; **Agda/Lean win for proof statements**. Document the mismatch; do not silently average.

---

## 3. Formal entry points (Agda)

Root lib: `agda/TriWeavon.agda-lib` · umbrella: `agda/src/Everything.agda`

| Area | Paths | Why you care |
|------|-------|--------------|
| Conservation + R | `TriWeavon/ConservationRMatrix.agda`, `Core.agda` | gauge spine |
| HITs / manifold | `TriWeavon/HITs/TriWeavonManifold.agda` | topology of the weave |
| K22 / Serre–Scarr | `TriWeavon/K22/*`, `SerreScarr.agda` | spectral page structure |
| Mehler / sub-Riemannian | `TriWeavon/SubRiemannian/*` | MehlerWiring, MehlerJesusBridge |
| Tomczak lifting | `TriWeavon/Tomczak/*`, `TomczakLifting.agda` | lift / obstruction |
| Magic / Mermin | `MagicStateInjectionGuard.agda`, `MerminPermutahedron-KS-Bounds.agda` | KS bounds |
| JesusAxiom ε | `JesusAxiomEpsilon/{Core,Contraction,Termination}.agda` | contraction / termination scaffold |
| Path induction | `SerreScarPathInduction.agda`, `TriWeavonPathInduction.agda` | higher paths |

**Check (prefer WSL if Windows lacks Agda):**

```powershell
logos-agda
# or
pwsh -File agda/scripts/check.ps1
```

Vendor scripts: `agda/scripts/vendor.ps1`, HTML: `agda/scripts/html.ps1`.

---

## 4. Formal entry points (Lean)

Toolchain pin: `lean/lean-toolchain` (**v4.8.0** — lake respects pin even if global elan is newer).

| Area | Paths |
|------|-------|
| Conservation | `lean/TriWeavon/ConservationInvariant.lean` |
| Vanishing resilience / strange loop | `lean/TriWeavon/VanishingResilience.lean` |
| Sub-Riemannian | `lean/TriWeavon/SubRiemannian/*` — **[B/open]** skeletons; do not call A-repo plant model |
| NS shrinker / ansatz | `lean/TriWeavon/NS/*` |
| K22 / M24 / existence | `lean/K22/**` |
| Lane-A monom transport | `lean/K22/MOG/MonomialWitness.lean` — π transport lemmas; **CB-1 residual open** (draft; compile debt); optional `mem_conway_packed_iff` sorry |
| Steiner double-count surface | `lean/K22/MOG/SteinerDoubleCount.lean` — **S1–S6 + `golay_octads_form_steiner` lake-green, 0 sorry** |
| Hexacode / Golay spine | `lean/K22/HexacodeGolay.lean` — `octad_count=759` **A-native** (`native_decide`) |
| Steiner whitepaper / handoff | `docs/formal/STEINER-DISCHARGE-STRATEGY-WHITEPAPER-20260730.md` · `docs/componentry/ATOMS/ATOM-STEINER-LANE-CHECKPOINT-20260730.md` |
| NS packages | `lean/Ns/**` |
| Name bridge | `lean/AgdaLeanBridge.md` |

**Build (prefer pin via lake):**

```powershell
logos-lean
# or
cd lean
lake build K22.MOG.MonomialWitness
lake build K22.MOG.SteinerDoubleCount
# full: lake build
```

**NotebookLM formal pack:** `docs/notebooklm/CRITICAL-MONOM-STEINER-LANE-A-20260730.txt`

Heisenforge-era bounds table: `docs/formal/key-bounds-collapsed-regime.tex`  
Collapsed-regime Lipschitz story: return map ≈ 0.8306, steps ~150–230 for 1000× shrink (see `ops/HANDOVER-HEISENGROK-BUILD-OS-2026-06-21.md`).

---

## 5. Executable theorem companions

| Piece | Path | Role |
|-------|------|------|
| cutile | `cutiles/cutile/` | HITs mirror, SRAC, TDA hooks, R-matrix tests |
| Blackwell kernels | `kernels/blackwell-*.cu` | entropy, FFT, PCR, Rips |
| Fundamental R | `kernels/fundamental_r_matrix.*` | device + WGSL |
| HUP M1 Mirage | `hup/unikernel/`, flake instance1 | BbBR surface |
| HUP M2 Redox | `hup/instance2-redox/` | ownership rails |
| HUP M3 RVM | `hup/instance3-rvm/` + vendor ruvnet | agentic domains |
| Dimensional collapse | `hup/python/dimensional_collapse.py` | 768→…→2 monitor |
| Existence certificates | `docs/encyclopedia-equilibria/certificates/` | JSON witnesses |
| Formal copies | `docs/encyclopedia-equilibria/formal-agda|formal-lean/` | portable excerpts |

**Smoke:**

```powershell
cargo test -p cutile r_matrix --manifest-path cutiles/cutile/Cargo.toml
```

CUDA optional: `cutiles/cutile` features + `scripts/build_ptx.ps1` when `nvcc` present. CPU host path is valid offline.

---

## 6. QPH / QTDA research (theorem-adjacent)

Quantum Persistent Homology / topological AI research directive:

- `docs/theory/RESEARCH-DIRECTIVE-QPH-20260403.md`  
- `docs/theory/QDI-FORMAL-ALGEBRAIC-FRAMEWORK.md`  
- `docs/theory/HOPF-PRINCIPAL-BUNDLE.md`  
- `docs/theory/FIXED-POINTS.md`  
- Companion: `05-HEISENPUP-QTDA-COMPANION.md` (operational uncertainty + homology metaphor)

**Novel territory claimed (research, Category B until sealed):**

1. Persistent homology for hallucination / H₂ void mapping  
2. Braid / Jones provenance beyond hash signatures  
3. Cross-strand topological consensus under conservation tag  
4. Browser-side TDA (Ripser→WASM path)

---

## 7. Cold-start protocol (theorems only)

1. Open this file + LAYER-CASCADE-MAP.  
2. Run cutile R-matrix tests (fastest position truth).  
3. If proofs: `lake build` (Lean pin) or Agda check (WSL OK).  
4. If receipts: verification orchestrator notebook.  
5. Stamp result via `atom_track` type `VERIFY`.  
6. **Do not** re-ingest entire User_Dropfiles — index via maps.

---

## 8. Open gaps (honest, tracked)

| ID | Gap | Note |
|----|-----|------|
| ML-1 | R-matrix params not fully injected into Qiskit conservation angles | open |
| ML-2 | Property tests partial | cutile units exist |
| ML-4 | CUDA needs toolkit; CPU host offline OK | partial |
| G2 | Mehler plateau not in MCP stdio | Rust only (`CERTIFIED_ERROR_TOL=5e-7` in cutile) |
| MW-1 | `mogOctadsFormSteinerSystem_via_transport` residual | CB-1 open; SDC S1–S6 green; handoff `ATOMS/ATOM-STEINER-LANE-CHECKPOINT-20260730.md` |
| MW-2 | `mem_conway_packed_iff` | optional SlowStep |
| MW-3 | `MiracleOctadGenerator.mogOctadsFormSteinerSystem` | separate keystone sorry — do not import to fake monom |
| MW-4 | MW compile debt (maskOf / weight8 ok path) | finish before claiming CB-1 green |
| SR-1 | SubRiemannian OB4/OB5 + live plant | literature A-lit; in-repo B/open |
| npm coherence-mcp TS | historical build pain | prefer live MCP + Rust tests |

---

## 9. Comments for newer theorem workers

- **Category A** = machine-checked or test-green; **B** = bounded approximation; **C** = convention; **D** = open. Label claims.  
- Do not “prove” with prose alone when cutile already has a failing test.  
- Lean global version ≠ project pin — always use `lake` from `lean/`.  
- Agda vendor tree is huge; use project scripts, not ad-hoc path surgery.  
- Conservation **tag** failure is not a physics violation — check your numbers.

*Music conserved. Proofs when green; amber when honest.*
