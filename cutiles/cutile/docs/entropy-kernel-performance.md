# Performance Model — `entropy_reduction_v2`

Kernel: `blackwell_entropy_v2.cu` / `entropy_reduce.wgsl`  
Workload: grid-stride reduction of `W[ω̃]`, viscous + stretch terms, surge + Betti proxy  
Target: NVIDIA RTX 5090 Blackwell (sm_100), budget ≤15 TFLOPs FP32 / 0.6 TB/s

## Executive summary

The kernel is **memory-bandwidth bound** (arithmetic intensity ~0.8–1.2 FLOPs/byte).  
In SRAC self-healing loops, **kernel launch overhead** dominates at moderate batch sizes.

## Relative performance (native CUDA = 100%)

| Backend | Estimated perf | Main overhead |
|---------|----------------|---------------|
| Native CUDA | **100%** | memory BW + launch |
| wgpu | **82–90%** | command encoding, launch (~20–60 µs) |
| ZLUDA | **72–85%** | API translation + memory model |
| Native HIP (est.) | **95–102%** | ROCm stack only |

## Overhead breakdown

| Source | CUDA | wgpu | ZLUDA | Impact |
|--------|------|------|-------|--------|
| Memory bandwidth | baseline | +5–10% | +8–15% | **Dominant** |
| Kernel launch | ~5–15 µs | ~20–60 µs | medium–high | **High in SRAC loops** |
| API translation | none | low | high | medium |
| Surge/Betti logic | negligible | negligible | negligible | very low |

## Relative execution time model

| Backend | Memory | Launch/encode | Translation | **Total** |
|---------|--------|---------------|-------------|-----------|
| CUDA | 1.00 | 1.00 | 0.00 | **1.00** |
| wgpu | 1.05–1.10 | 1.8–3.0 | 0.00 | **1.11–1.22** |
| ZLUDA | 1.08–1.15 | 1.6–2.5 | 1.10–1.25 | **1.18–1.39** |

## Optimization priority

| # | Optimization | Expected gain | Status |
|---|--------------|---------------|--------|
| 1 | **Batch entropy evaluations** | Very high | `Backend::compute_entropy_batch` ✓ |
| 2 | Fuse surge + SRAC in one pass | High | planned |
| 3 | Persistent threads / grid-stride | Medium | CUDA kernel already grid-stride |
| 4 | `__ldg` read-only cache (CUDA) | Medium | planned |
| 5 | FP16/BF16 tensor cores | Medium–high | future |
| 6 | Nsight Compute profiling | diagnostic | recommended before tuning |

## Nsight Compute guidance

1. Profile `entropy_reduction_v2` with representative `np × N` from TriWeavon meshes.
2. Check **Memory Throughput** vs **Compute Throughput** — expect memory-bound.
3. Inspect **Achieved Occupancy** and register usage (surge/Betti adds minor register pressure).
4. Compare **Kernel Duration** vs **API overhead** when launched in tight SRAC loops.
5. Validate `__ldg` on `omega_tilde`, `d_perp_rho_sq`, `rho`, `strain_norms` inputs.

## Recommendation

- **Production hot path**: native CUDA on NVIDIA.
- **Default development / AMD / Nix**: wgpu (`entropy_reduce.wgsl`).
- **Avoid**: ZLUDA for first-party cutile kernels.
- **Next**: fused batch kernel (single launch, multiple parameter sets) for SRAC loops.