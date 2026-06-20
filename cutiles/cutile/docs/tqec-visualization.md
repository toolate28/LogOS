# TQEC Visualisation Cascade — cutile v0.3

Minimal prototypes aligned with reson8Labs Museum of Computation visual language.

## Visual grammar

| Element | Colour | Source |
|---------|--------|--------|
| Void background | navy-black `(6,10,22)` | `viz::palette::tqec::VOID` |
| 0-cells / core | cyan `(0,200,255)` | `TriWeavonHIT::add_point` |
| Weaves / SRAC braids | amber-orange `(255,140,60)` | `weave`, `hcomp_edge` |
| HUP entropy field | purple `(140,60,220)` | `EntropyResult.w` normalized |
| Survivors | amber `(255,180,0)` | `surge == 0` |
| Syndrome spikes | cyan `(80,220,240)` | `betti_proxy > 1` |
| Mirrored observer frame | violet `(180,120,255)` | UI chrome |

## Prototype map

| # | File | Fidelity | Data sources |
|---|------|----------|--------------|
| 1 | `examples/tqec_braid_viz.rs` | Runnable egui | `TriWeavonHIT`, `Backend::compute_entropy_v2`, SRAC |
| 2 | `src/viz/tqec_syndrome.rs` | CPU syndrome map | `SyndromeField`, embedded in prototype 1 |
| 3 | `examples/tqec_concurrent_cascade.rs` | Thread skeleton | `thread::spawn` + `Backend` |

## Cascade layers

```
Agda HIT (formal)  →  TriWeavonHIT (executable graph)
                    →  Backend entropy (numerical witness)
                    →  viz palette + braid layout (rendering)
```

## Run

```powershell
cargo run -p cutile --example tqec_braid_viz --features viz
```

**Step SRAC Correction** triggers `srac_correct_if_needed`, adds weave cells on correction, refreshes entropy field.

## Future (Phase 1–3)

- wgpu render pass for syndrome field (replace CPU pixel loop)
- `hcomp` 2-cell rendering for error-correction cycles
- tokio async bridge to MCP / triweave WS