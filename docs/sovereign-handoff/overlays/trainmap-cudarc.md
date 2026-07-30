# Trainmap — cudarc Rail (Rust CUDA)

```
STATION 0 ──► STATION 1 ──► STATION 2 ──► STATION 3
feature cuda  CudaBackend   PTX kernels   Witness / LiftOk
```

## Stations (real paths)

| Station | Path | Couples to |
|---------|------|------------|
| 0 Feature | `cutiles/cutile/Cargo.toml` `cuda` + cudarc | host toolchain |
| 1 Backend | `cutiles/cutile/src/backend/cuda.rs` | entropy kernel launch |
| 2 Kernels | `cutiles/cutile/kernels/*.cu`, `kernels/blackwell-*.cu` | cutile + top-level |
| 3 R-matrix CUDA | `kernels/fundamental_r_matrix.cu` | WGSL twin, Rust r_matrix |
| 4 Host | `cutiles/r_matrix_host.cpp` | CPU fallback + launch |

## Ownership (Instance #2)

Device buffers are **uniquely owned** until explicit `cudaMemcpy`; Redox metaphor:
scheme-held GPU memory handle. No shared mutable `QuantumState` across threads
without `Arc` (host) / exclusive device alloc (GPU).

## ASCII rail

```
cudarc ░░░▒▒▒▓▓▓ CUDA RAIL ▓▓▓▒▒▒░░░
       │feat│─│Backend│─│PTX/CU│─│R-matrix│─│host│
                    ▲
                    └── LiftOk / KernelWitness (sm_100 HO-05)
```
