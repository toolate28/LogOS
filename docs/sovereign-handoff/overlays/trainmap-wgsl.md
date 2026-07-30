# Trainmap — WGSL Rail (WebGPU Shader)

```
STATION 0 ──► STATION 1 ──► STATION 2 ──► STATION 3
checklist     entropy_reduce  fundamental_r  wgpu backend
```

## Stations (real paths)

| Station | Path | Couples to |
|---------|------|------------|
| 0 Checklist | `cutiles/cutile/docs/wgsl-porting-checklist-v0.4.2.md` | parity plan |
| 1 Entropy | `cutiles/cutile/kernels/entropy_reduce.wgsl` | CUDA entropy twin |
| 2 R-matrix | `kernels/fundamental_r_matrix.wgsl` | CUDA/Rust R-matrix |
| 3 Backend | `cutiles/cutile/src/backend/wgpu.rs` | browser / desktop GPU |
| 4 Stitch UI | `stitch/*` dashboards | visualization only |

## Ownership (Instance #2)

Shader storage buffers are **bind-group owned**; host writes, GPU exclusive
during dispatch. Redox parallel: scheme resource → single writer.

## ASCII rail

```
WGSL ░░░▒▒▒▓▓▓ SHADER RAIL ▓▓▓▒▒▒░░░
     │check│─│entropy│─│R-matrix│─│wgpu│─│stitch│
                  ▲
                  └── structural identity with CUDA R rows
```
