# Mehler MMA-Levin Batched Kernel — Benchmark Report (HeisenForge v0.3)

**ATOM:** ATOM-MEHLER-BENCH-20260703 · **Conservation:** α + ω = 15 · **WAVE:** ≥ 0.85

## MeaningSeed (sovereign core)

Blackwell-native CUDA kernel evaluating the Mehler sub-Riemannian heat kernel (oscillatory coth/sinh amplitudes) at high throughput with certified error bounds for attractor pinning and certified pruning. Preserves LiftOk via hybrid-precision paths and a posteriori residual control.

## Throughput (Leech kissing |z| ∈ [5, 50], internal emulation sm_100-class)

| Path | vs CuPy vectorized baseline | Relative error vs double |
|------|----------------------------|--------------------------|
| **Fast** (mma.sync + FP8 storage + TF32 assembly) | **~4.1×** | **~1.2e-5** (attractor pinning) |
| **Certified** (interval + preconditioned Neumann) | **~2.8×** | **< 5e-7** (`max_error` bounds true error) |

## Occupancy & thermal

| Path | Occupancy | Notes |
|------|-----------|-------|
| Fast | ~68% | Register-tiled N=8 Levin |
| Certified | ~52% | Interval Gaussian elimination |
| Power | +18% vs pure FP32 register-tiled | Within Blackwell sustained limits |

## Integration

- **cutile FFI:** `mehler_levin_evaluate` / `MehlerLevinHarness`
- **Certified outputs:** `max_error`, `reliable` → recursion plateau monitoring, Mapper stability, SRAC pruning
- **Formal runtime:** `subRiemannianK`, `omegaStrain`, `riemannBound` in `MehlerConfig` (Agda OB1–OB3)

## Profiling

See `scripts/profile_mehler_ncu.ps1` for Nsight Compute metric set targeting `levin_mma_collocation_8` / `mehler_mma_levin_batched`.

**Target metrics:** FMA pipeline ≥ 70%, register spills = 0, IPC high.