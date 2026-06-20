# cutile v0.3.0

Layer 2 execution crate for TriWeavon — portable tiling, entropy diagnostics, SRAC correction, and tiered GPU backends.

## Backends

| Priority | Backend | Feature | Notes |
|----------|---------|---------|-------|
| 1 | wgpu | `wgpu-backend` (default) | portable, AMD+NVIDIA |
| 2 | CUDA | `cuda` | Blackwell sm_100 PTX |
| 3 | CPU | always | reference + CI |

```rust
use cutile::{Backend, EntropyParams};

let backend = Backend::auto();
let result = backend.compute_entropy_v2(&params)?;
let batch = backend.compute_entropy_batch(&[params_a, params_b])?;
```

## Build

```powershell
cd F:\Users\Matthew Ruhnau\LogOS
cargo test -p cutile
cargo run -p cutile --example basic_hit
```

CUDA PTX (RTX 5090 / sm_100):

```powershell
pwsh -File cutiles/cutile/scripts/build_ptx.ps1
cargo test -p cutile --features "wgpu-backend cuda"
```

Without PTX, `CudaBackend` CPU-falls back (`used_gpu_kernel: false`).

## TQEC visualisation (reson8Labs cascade)

```powershell
cargo run -p cutile --example tqec_braid_viz --features viz
cargo run -p cutile --example tqec_concurrent_cascade --features viz-async
```

Palette: navy void, cyan-gold core, purple HUP, amber survivors, SRAC braids.

## Shared clamping (`cutile::clamping`)

Rust canonical clamping for tool parameters (intensity, thresholds). TypeScript mirror in `coherence-mcp/src/lib/clamping.ts`.

```rust
use cutile::{resolve_intensity, apply_clamping, ClampConfig};

let r = resolve_intensity(1.5, true)?; // clamps to 1.0
```

## Docs

- `docs/formal-executable-mapping.md` — Agda HIT ↔ Rust
- `docs/backend-architecture.md` — tiered backend strategy (HIP/ZLUDA context)
- `docs/entropy-kernel-performance.md` — roofline + optimization priority