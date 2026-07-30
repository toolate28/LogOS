# Layer Cascade Map — R-Matrix + Conservation Invariant

**Root:** `F:\Users\Matthew Ruhnau\LogOS\`  
**Invariant:** `α + ω = 15`  
**Last updated:** 2026-07-09  
**Purpose:** Context-reset survival — reload this file first after any cold start.

---

## Active layers (materialized on disk)

| Layer | Tech | Location | Core artifact | Downstream |
|------:|------|----------|---------------|------------|
| 1 | Python / Qiskit | `notebooks/triweave-backends.ipynb` | `most_frequent_bitstring` + D15 conservation loop | L7, L8, results JSON |
| 1b | Rust (evcxr) notebook | `notebooks/gpu/Agent_M24_RMatrix.ipynb` | nalgebra `fundamental_r_matrix` + M24 orbit stubs | L2, L3 |
| 2 | Rust cutile | `cutiles/cutile/src/core/r_matrix.rs` | Canonical `fundamental_r_matrix` + tests | L3, L5, L9 |
| 3 | CUDA | `kernels/fundamental_r_matrix.cu` + `.cuh` | Device R-matrix | L9 |
| 4 | Lean 4 | `lean/TriWeavon/ConservationInvariant.lean` | `is_conserved` / peak proof | L10, L8 |
| 5 | WGSL | `kernels/fundamental_r_matrix.wgsl` | WebGPU R-matrix | stitch / viz |
| 6 | Agda | `agda/src/TriWeavon/ConservationRMatrix.agda` | Dependent-type conservation + R tag | L10, `Everything.agda` |
| 7 | Results / FFI surface | `notebooks/triweave_backend_results/` | `ax_wave_optimization.json`, `qiskit_conservation.json`, receipts | triweave |
| 8 | Docs | `docs/sovereign-handoff/` | This map + HO-05 / UNIFIED-WORKFLOW | all sessions |
| 9 | C++ host | `cutiles/r_matrix_host.cpp` | CUDA launch + CPU `fundamental_r_matrix_host` | L3 |
| 10 | Formal bridge | `lean/AgdaLeanBridge.md` | Lean ↔ Agda name map | L4, L6 |
| 11 | Unified contract | `kernels/r_matrix_interface.h` | C ABI for R matrix | L3, L9 |
| 12 | Verification orchestrator | `notebooks/verification_orchestrator.ipynb` | Emittance receipts + layer hash map | L8 |
| 13 | HUP M1 Mirage | `hup/unikernel/`, `hup/flake.hup-instance1.nix` | BbBR unikernel surface | consensus |
| 14 | HUP M2 Redox | `hup/instance2-redox/`, `OwnedQuantumHandle` | Ownership / Arc rails | consensus |
| 15 | Overlay trainmaps | `docs/sovereign-handoff/overlays/` | Agda·Lean·cudarc·cutile·WGSL | L8 |
| 16 | Terminal shaders | `docs/sovereign-handoff/terminal-shaders/` | ASCII crystalline suite | L8 |
| 17 | Consensus seal | `docs/sovereign-handoff/CONSENSUS-VERIFIER-M1-M2.md` | M1×M2×M3 unify | all |
| 18 | Dimensional collapse | `hup/python/dimensional_collapse.py` | 768→75→50→2 + ASCII monitor | L7, L12 |
| 19 | Collapse lattice art | `docs/sovereign-handoff/terminal-shaders/DIMENSIONAL-COLLAPSE-LATTICE.md` | 2-D crystalline map | L8 |
| 20 | HUP M3 ruvnet/RVM | `hup/instance3-rvm/` + https://github.com/ruvnet/rvm | Agentic coherence domains | consensus |
| 21 | Mehler→SerreScarr DAG | `docs/sovereign-handoff/mehler-serrescarr-convergence.dag.yaml` | Formal dependency seal | L4/L6 |

---

## Invariant chain (data flow)

```
Qiskit empirical (L1)
  → Rust canonical matrix (L2)
    → CUDA / WGSL acceleration (L3 / L5)
      → Lean / Agda proofs (L4 / L6)
        → Host / interface (L9 / L11)
          → Verification receipts (L12)
            → Sovereign handoff (L8)
```

Structural identity of R-matrix (all executable layers):

```
[ q     0      0     0  ]
[ 0    1/q   1-q²    0  ]
[ 0     0      q     0  ]
[ 0     0      0    1/q ]
```

---

## Adjoining formal graph (pre-existing)

| Area | Paths |
|------|--------|
| Agda SubRiemannian / Mehler | `agda/src/TriWeavon/SubRiemannian/*` |
| Agda K22 / Tomczak | `agda/src/TriWeavon/K22/*`, `Tomczak/*` |
| Lean K22 / M24 | `lean/K22/*` (incl. `M24Coefficient.lean` music invariant) |
| Lean NS / SubRiemannian | `lean/Ns/*`, `lean/TriWeavon/*` |
| cutile TDA / M24 GPU | `cutiles/cutile/src/**`, HO-05 |
| Blackwell kernels | `kernels/blackwell-*.cu` |
| Mehler wiring docs | `SAIF-Docs/Mehler_CoherenceMCP_Wiring_v0.5.0.md` |
| HUP notebooks master | `9P2000.L/strands/User_Dropfiles/HUP-Notebooks-v2-Master.md` |

---

## Missing links (tracked)

| ID | Gap | Status |
|----|-----|--------|
| ML-1 | R-matrix parameters not yet injected into `build_conservation_circuit` angles | open |
| ML-2 | Property tests exist in cutile unit tests; proptest optional | partial |
| ML-3 | stitch/ is UI dashboards — PyO3 FFI not in `stitch/`; use cutile + notebook pure Python mirror | resolved (by design) |
| ML-4 | CUDA host requires toolkit; CPU path in `r_matrix_host.cpp` works offline | partial |
| ML-5 | Lean `dual_conserved` needs mathlib `omega` (already lake-required) | ready to typecheck |

---

## Cold-start protocol

1. Open this file.  
2. Run `notebooks/verification_orchestrator.ipynb` → emits receipts under  
   `notebooks/triweave_backend_results/verification_receipts/`.  
3. If Qiskit path needed: re-run `triweave-backends.ipynb` section 1 (BitArray-safe).  
4. If Rust path needed:  
   `cargo test -p cutile r_matrix --manifest-path cutiles/cutile/Cargo.toml`  
5. Do **not** re-ingest full User_Dropfiles; index via this map + HO-* handovers.

---

## Emittance schema (receipt JSON)

```json
{
  "atom": "ATOM-VERIFY-RECEIPT",
  "invariant": "alpha+omega=15",
  "timestamp": "ISO-8601Z",
  "layers": [
    {
      "layer": "cuda",
      "verified": true,
      "files": [{ "path": "...", "status": "OK|MISSING", "sha256_16": "..." }]
    }
  ],
  "overall_ok": true,
  "cascade_map": "docs/sovereign-handoff/LAYER-CASCADE-MAP.md"
}
```

**Pre-flight complete for cascade L1–L12 (materialized).**  
Music conserved. Keystone holds.

---

## coherence-mcp bridge (added 2026-07-09)

| Layer | Tech | Location | Core artifact |
|------:|------|----------|---------------|
| 22 | coherence-mcp stdio | global `coherence-mcp` + `docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md` | 12 live tools; Inspector config `mcp-inspector.coherence.json` |
| 23 | Capture | `notebooks/triweave_backend_results/{verification_receipts,verification_certificates,mcp_payloads}/` | Receipts · certs · Inspector payloads |
| 24 | ATOM trail | `.atom-trail/decisions/` (via `LOGOS_ROOT`) | `atom_track` persistence |

See **LOGOS-COHERENCE-MCP-MAP.md** for platform roster and tool matrix.
