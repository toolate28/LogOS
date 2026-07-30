# Sovereign WGSL Porting Checklist — TriWeavon GPU Layer v0.4.2

**Status:** Ready for cutile maintainers · **WAVE:** ≥ 0.97 · **ATOM:** WGSL-PORTING-CHECKLIST-v0.4.2

## Kernel matrix

| # | Component | Priority | WGSL | Status |
|---|-----------|----------|------|--------|
| 1 | `blackwell_entropy_v2.cu` | Critical | High | Partial |
| 2 | `entropy_reduce.wgsl` | Critical | — | In progress (reference) |
| 3 | `blackwell-pcr.cu` | High | High | Not started |
| 4 | `blackwell-fft.cu` | Medium | Medium | Not started |
| 5 | `hypa_boundary_reduction.cu` | Medium | Low–Med | CUDA-primary |
| 6 | `mehler_levin_fp8.cu` / `mehler_mma_levin_batched.cu` | Low | **Very Low** | **CUDA-only** |
| 7 | `bch_syndrome_v1.cu` | Low | Medium | Not started |
| 8–10 | Atomics / subgroups / Tensor Core FP8 | — | — | Design `AtomicHelper`, `SubgroupOps`; CUDA-only for mma.sync |
| 11 | Backend divergence lemma | High | — | Add to tomczak-lift-gate-formalizer |
| 12 | WAVE gate / certified pruning | High | High | Partial — unify CUDA + WGSL |
| 13 | ATOM trail | Critical | High | Partial |
| 14 | Parity harness | Critical | — | Missing — required before production |

## Mandatory process

1. **Design** — backend contract (precision, atomics, subgroups)
2. **Abstraction** — `TilingStrategy`, workgroup reduce in cutile
3. **Port** — preserve semantics; hook `certified_mode` / `anomaly_count`
4. **Verify** — bitwise + statistical parity; LiftOk identical within tolerance
5. **Document** — Crystalline Kernel Compendium + formal-executable bridge

## Recommendations

- `entropy_reduce.wgsl` = wgpu reference for entropy work
- Mehler-FP8 / HYPHA / Tensor Core = **CUDA/HIP primary** indefinitely
- Invest in cutile abstractions first (highest leverage)