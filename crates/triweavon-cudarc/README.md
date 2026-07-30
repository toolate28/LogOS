# triweavon-cudarc

Sovereign-named high-level CUDA layer for TriWeavon (avoids colliding with the `cudarc` dependency crate).

**Relationship to `cutile` v0.3**: `cutile` owns tiling, backends (wgpu/CUDA/CPU), entropy kernels, and HIT execution. `triweavon-cudarc` will own domain GPU types: `GpuManifold`, Jones polynomial ops, K22_SHEAF traversals, SRAC metric buffers.

## Status

Skeleton v0.1.0 — invariant enforcement + CPU fallback wired; GPU kernels TBD.

## Features

- `cpu-fallback` (default) — no GPU required
- `cuda` — NVIDIA Blackwell via `cudarc` + `cutile`