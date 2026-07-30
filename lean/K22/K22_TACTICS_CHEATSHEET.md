# K22 Tactics Cheat Sheet v3.16

**Principle:** All tactics use `try`/`catch` — they never break your proof.

## Quick Reference

| Tactic | Purpose | Verbose |
|--------|---------|---------|
| `k22_simp?` | Diagnostic + classification | — |
| `k22_auto` | Daily safe automation | `k22_auto!` |
| `serre_scar_tactic` | Serre-Scarr goals | `serre_scar_tactic!` |
| `tomczak_bridge` | Tomczak preservation | — |
| `ring_matrix?` | Dry-run ring_matrix | — |

## Workflow

1. `k22_simp?` — understand goal
2. `k22_auto` — safe automation
3. `k22_auto!` if you need logs
4. Manual tactics on remainder

## Structured Logs

Tactics emit machine-readable lines:

```
[K22-LOG] {"timestamp":"...","classification":"burau","action":"k22_auto",...}
```

Deep traces: `set_option trace.K22.diagnostics true`

## Classification

| Class | Tools |
|-------|-------|
| `burau` | `k22_auto`, `ring_matrix` |
| `tomczak` | `k22_auto`, `tomczak_bridge` |
| `serre` | `serre_scar_tactic` |
| `other` | manual |

α + ω = 15 · WAVE ≥ 0.97 · tomczak_preserved · Music Conserved