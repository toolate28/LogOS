# GPU Acceleration Roadmap — TriWeavon Layer 2

**Date**: 2026-06-20  
**Status**: `cutile` v0.3 implemented; `triweavon-cudarc` v0.1 skeleton added.

## Crate split (sovereign naming)

| Crate | Role | Status |
|-------|------|--------|
| **`cutile`** | Tiling, backends (wgpu/CUDA/CPU), entropy kernel, HIT bridge, clamping | **v0.3** on disk |
| **`triweavon-cudarc`** | Domain GPU: `GpuManifold`, Jones, K22_SHEAF, SRAC buffers | **v0.1 skeleton** |
| upstream `cudarc` | Low-level CUDA driver — dependency only, not renamed |

Avoid naming a crate `cudarc` — collides with the dependency. Production integration uses `triweavon-cudarc`.

## Backend priority

1. **wgpu** — portable default (AMD+NVIDIA)
2. **CUDA** — Blackwell sm_100 hot path (`blackwell_entropy_v2.cu`)
3. **HIP** (future) — AMD Instinct native
4. **ZLUDA** — never for first-party kernels
5. **CPU** — always available

## Integration targets

| Consumer | Connection | Priority |
|----------|------------|----------|
| `triweave` | Optional GPU backend for heavy ops | Medium |
| `coherence-mcp` | `trigger_correction_burst` + SRAC metrics | High |
| Stitch cockpit | WebSocket `invariant_drift` + OTLP `reson8.*` | High |
| Lean4/Agda | FFI bounds on kernel drift | Long-term |

## Next actions (recommended order)

1. Benchmark `entropy_reduction_v2` on RTX 5090 (Coherence Forge)
2. Wire `triweavon-cudarc::compute_srac_metrics` into MCP + Stitch WS frames
3. Jones / K22_SHEAF GPU kernels (narrow scope)
4. GrokOS.nix CUDA pin (`cuda-12040`)