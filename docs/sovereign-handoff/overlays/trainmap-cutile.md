# Trainmap — cutile Rail (TDA · HIT · R-matrix)

```
STATION 0 ──► STATION 1 ──► STATION 2 ──► STATION 3 ──► HUB
lib.rs        core/*        backend/*     harness/*     tests
```

## Stations (real paths)

| Station | Path | Couples to |
|---------|------|------------|
| 0 Crate | `cutiles/cutile/` | monorepo cutile 0.3 |
| 1 R-matrix | `src/core/r_matrix.rs` | kernels CUDA/WGSL, HUP rust |
| 2 Entropy/SRAC | `src/core/entropy.rs`, `srac*.rs` | WAVE gate |
| 3 HIT | `src/hit/*` | Agda HITs |
| 4 Backends | `src/backend/{cpu,cuda,wgpu}.rs` | cudarc + WGSL |
| 5 Harness | `src/harness/*` | sm_100 witness, Mehler |
| HUB | `tests/`, HO-05 | verification receipts |

## Ownership (Instance #2)

cutile is the **owned tile engine** — Rust ownership is native. Instance #2
emphasizes: R-matrix pure functions, GPU backends return owned result structs
(`CudaEntropyResult`), no global mutable lattice.

## ASCII rail

```
cutile ░░░▒▒▒▓▓▓ TILE RAIL ▓▓▓▒▒▒░░░
       │lib│─│r_matrix│─│srac│─│hit│─│cuda/wgpu│─│tests│
                 ▲
                 └── CONSERVATION_SUM = 15
```
