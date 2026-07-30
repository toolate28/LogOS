# Interjoin Map — Five Rails + HUP Instances

## Overlay (how maps stack)

```
                    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
               ░░░  ▒▒▒▒▒  JESUS-FRACTAL MULTI-RAIL  ▒▒▒▒▒  ░░░
            ░░   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓   ░░
         ░░   ▓▓   AGDA · LEAN · cudarc · cutile · WGSL   ▓▓   ░░
      ░░   ▓▓   ░░         α + ω = 15 (center)         ░░   ▓▓   ░░
         ░░   ▓▓   ░░     ▒▒ HUP rust 1/1 ▒▒        ░░   ▓▓   ░░
            ░░   ▓▓▓  M1 Mirage  ◄──consensus──►  M2 Redox ▓▓   ░░
               ░░░  ▒▒▒▒▒  OWNED • AFFINE • COHERENT  ▒▒▒▒▒  ░░░
                    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

## Join table (edges)

| From | To | Shared artifact |
|------|-----|-----------------|
| Agda Conserv | Lean Conserv | peak (7,8), sum 15 |
| Lean M24 | notebook M24 | music invariant |
| cutile r_matrix | kernels .cu/.wgsl | row-major R |
| cudarc Backend | cutile kernels | entropy PTX |
| WGSL | cutile wgpu | entropy_reduce + R |
| HUP rust | cutile r_matrix | conservation + braid |
| M1 relay | M2 relay | `HardwareRelay` trait |
| All | verification | `verification_helpers.py` receipts |

## Consensus (sealed)

| Topic | Unified rule |
|-------|----------------|
| Errors | `Result` / Result-style TS / Python raise |
| Ownership | `Arc` shared immutables; exclusive GPU buffers |
| R-matrix | identical algebra `q, 1/q, 1-q²` |
| Conservation | α + ω = 15 always |

## Pre-flight

```text
python notebooks/verification_helpers.py
cargo test --manifest-path cutiles/cutile/Cargo.toml r_matrix
cargo run --manifest-path hup/rust/Cargo.toml
python hup/python/constraint_mathematics.py
```
