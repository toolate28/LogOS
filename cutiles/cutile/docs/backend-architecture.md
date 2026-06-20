# cutile Backend Architecture — v0.3

Tiered GPU strategy for TriWeavon entropy, tiling, and SRAC workloads.

## Priority stack

| Priority | Backend | Use when | ZLUDA / HIP notes |
|----------|---------|----------|-------------------|
| 1 | **wgpu** | Default portable path, AMD+NVIDIA, Nix reproducibility | Preferred over ZLUDA |
| 2 | **CUDA** | Peak performance on NVIDIA Blackwell (sm_100) | Native hot path |
| 3 | **HIP** (future) | Peak performance on AMD Instinct | Native preferred over ZLUDA |
| 4 | **ZLUDA** | External unmodified CUDA binaries only | 10–30% overhead; not for cutile kernels |
| 5 | **CPU** | No GPU, CI, correctness reference | Always available |

## Rust API

```rust
use cutile::{Backend, EntropyParams, EntropyResult};

let backend = Backend::auto(); // cuda → wgpu → cpu
let params = EntropyParams { /* ... */ };
let result: EntropyResult = backend.compute_entropy_v2(&params)?;

// SRAC loop: batch to amortize launch overhead
let batch = vec![params_a, params_b, params_c];
let results = backend.compute_entropy_batch(&batch)?;
```

### `Backend` enum

| Variant | Feature flag | Implements |
|---------|--------------|------------|
| `Backend::Cpu` | always | `compute_entropy_v2`, `compute_entropy_batch` |
| `Backend::Cuda` | `cuda` | wraps `CudaBackend` + PTX `entropy_reduction_v2` |
| `Backend::Wgpu` | `wgpu-backend` (default) | WGSL `kernels/entropy_reduce.wgsl` |

### Trait separation

| Trait | Layer | Backends |
|-------|-------|----------|
| `ManifoldCompute` | numerical entropy | CPU, CUDA, wgpu |
| `CubicalHIT` | HIT cell graph (`TriWeavonHIT`) | CPU only (structure, not GPU) |
| `TilingStrategy` | tile sizing | `DefaultTiler` (shared) |
| `SurgeDetector` | surge predicate | `DefaultSurgeDetector` (core) |

`CubicalHIT` stays on the CPU graph — GPU backends handle field reductions, not path constructors.

## HIP comparison (2026)

| Approach | Portability | AMD perf | Port from CUDA | GrokOS.nix |
|----------|-------------|----------|----------------|------------|
| `hip-sys` direct | Low | Excellent | High | Medium |
| wgpu (Vulkan/Metal/DX12) | Excellent | Very good | Low–medium | Excellent |
| ZLUDA | Medium | Good (70–90%) | None (drop-in) | Good |
| SYCL | Low (Rust) | Good | Medium | Medium |

**Verdict**: implement HIP as priority 3 after wgpu stabilizes; never use ZLUDA for first-party kernels.

## Build matrix

```powershell
# Default (wgpu + CPU)
cargo test -p cutile

# NVIDIA peak
pwsh -File cutiles/cutile/scripts/build_ptx.ps1
cargo test -p cutile --features "wgpu-backend cuda"

# CPU-only CI
cargo test -p cutile --no-default-features
```

## File map

| Path | Role |
|------|------|
| `src/backend/mod.rs` | `Backend` enum + batch dispatch |
| `src/backend/cpu.rs` | CPU reference + batch |
| `src/backend/cuda.rs` | PTX kernel + CPU fallback |
| `src/backend/wgpu.rs` | WGSL compute pipeline |
| `src/backend/types.rs` | `EntropyParams`, `EntropyResult` |
| `kernels/blackwell_entropy_v2.cu` | NVIDIA hot kernel |
| `kernels/entropy_reduce.wgsl` | portable compute shader |