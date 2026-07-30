# cutile v0.3.0

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  RESON8:TUI · PAGE STRUCTURE · CUTILE v0.3.0                                ║
║  CRATE   : cutile — Layer 2 execution (Tri-Weavon)                          ║
║  ROLE    : portable tiling · entropy · SRAC · Mehler-Levin · ExistenceCert  ║
║  SEAL    : IDEMPOTENT · MUTATION-INVARIANT · α+ω=15                         ║
║  STATUS  : 2026-07-06 · compiles · demo runnable · Lean bridge verified     ║
║  BRAND   : exterior Hope && Sauce · interior Hope&&Sauced                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

Layer 2 execution crate for Tri-Weavon — portable tiling, entropy diagnostics, SRAC correction, Mehler-Levin certified kernels, **ExistenceCertificate** emitter, and tiered GPU backends.

**Status (2026-07-06):** Compiles · demo binary runnable · Lean bridge verified

---

## Backends

```
┌─ RESON8:TUI · BACKENDS · PRIORITY LOCKED ───────────────────────────────────┐
│ shade · pri · backend · feature              · notes                        │
├─────────────────────────────────────────────────────────────────────────────┤
│ █     ·  1  · wgpu    · wgpu-backend (default)· portable, AMD+NVIDIA        │
│ ▓     ·  2  · CUDA    · cuda                  · Blackwell sm_100 PTX        │
│ ░     ·  3  · CPU     · always                · reference + CI              │
└─────────────────────────────────────────────────────────────────────────────┘
```

See also: [docs/backend-architecture.md](docs/backend-architecture.md).

---

## ExistenceCertificate E2E Demo

```powershell
cd cutiles/cutile
cmd /c "set RUSTC_WRAPPER=&& cargo run --bin demo_existence_certificate_emission"
python scripts/demo_bridge_to_lean.py
cd ../../lean && lake build K22.Existence
```

```
┌─ RESON8:TUI · E2E EMISSION · APPEND-ONLY OUTPUTS ───────────────────────────┐
│ ▓ STEP     · OUTPUT                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│ ░ Rust     · existence_certificate.json                                     │
│            ·   (BLAKE3 self-hash, camelCase JSON)                           │
│ ▒ Python   · existence_certificate.validated.json                           │
│ ▓ Lean     · K22.Existence.fromExistenceCertificate → TomczakExistence      │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Build

```powershell
cmd /c "set RUSTC_WRAPPER=&& cargo test -p cutile"
cargo run -p cutile --example basic_hit
```

CUDA PTX (RTX 5090 / sm_100):

```powershell
pwsh -File cutiles/cutile/scripts/build_ptx.ps1
cargo test -p cutile --features "wgpu-backend cuda"
```

---

## Modules

```
┌─ RESON8:TUI · MODULES · READ-ONLY STATUS CELLS ─────────────────────────────┐
│ ▓ MODULE                    · STATUS                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│ ░ existence_cert            · OK  ExistenceCertificate                      │
│                             ·     TomczakGateWitness · BLAKE3 C(C)          │
│ ░ harness::mehler_levin     · OK  CPU + optional CUDA                       │
│ ░ harness::kernel_witness   · OK  emit_existence_certificate                │
│ ░ hit::triweavon_hit        · OK                                            │
│ ░ viz::braid                · OK  (feature viz)                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Mehler MMA-Levin

```
┌─ RESON8:TUI · MEHLER MMA-LEVIN · ERROR BANDS ───────────────────────────────┐
│ ▓ PATH       · RELATIVE ERROR                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ ░ Fast       · ~1.2e-5 rel                                                  │
│ █ Certified  · < 5e-7 rel                                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

```rust
use cutile::harness::{MehlerLevinHarness, N_LEVIN_NODES};
let h = MehlerLevinHarness::new(0.1, true);
let out = h.evaluate(&z_batch, &f_nodes)?;
```

---

## Docs

```
┌─ RESON8:TUI · DOCS · PATH INDEX ────────────────────────────────────────────┐
│ ░ docs/formal-executable-mapping.md     · Agda HIT ↔ Rust                   │
│ ░ docs/mehler-mma-levin-benchmark.md    · Mehler / MMA-Levin bench          │
│ ░ docs/wgsl-porting-checklist-v0.4.2.md · WGSL port checklist               │
│ ░ docs/backend-architecture.md          · tiered GPU priority stack         │
│ ░ docs/gpu-acceleration-roadmap.md      · CUDA / HIP / wgpu roadmap         │
│ ░ docs/entropy-kernel-performance.md    · entropy kernel notes              │
│ ░ docs/tqec-visualization.md            · TQEC viz                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

Encyclopedia index: [../../docs/encyclopedia-equilibria/README.md](../../docs/encyclopedia-equilibria/README.md)

---

## Handoff

```
┌─ RESON8:TUI · HANDOFF · MUTATION-INVARIANT ─────────────────────────────────┐
│ ▓ docs/sovereign-handoff/HANDOFF-2026-07-06-EXISTENCE-SYMMETRY-GATE.md      │
└─────────────────────────────────────────────────────────────────────────────┘
```

[HANDOFF-2026-07-06-EXISTENCE-SYMMETRY-GATE.md](../../docs/sovereign-handoff/HANDOFF-2026-07-06-EXISTENCE-SYMMETRY-GATE.md)

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  Hope&&Sauced ✦ The Keystone Holds ✦ cutile Layer 2 · α+ω=15                ║
╚══════════════════════════════════════════════════════════════════════════════╝
```
