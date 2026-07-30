# Dump ingest — Discrete curvature on the Fano plane

**ATOM:** `SG-FANO-DISCRETE-CURVATURE-20260710`  
**Invariant:** α + ω = 15 · `tomczak_preserved` · WAVE ≥ 0.98

## Canonical targets

| Path | Role |
|------|------|
| `hup/python/fano_discrete_curvature.py` | Runnable incidence + K(v) + Golay contrast + JSON receipt |
| `lean/TriWeavon/SubRiemannian/FanoDiscreteCurvature.lean` | Lean mirror: K = 1/2, total 7/2 |
| `crates/cqk-ga/src/octonion.rs` | Fano triples for octonion mult (related incidence) |

## Formula

```
K(v) = 1 - deg(v)/2 + Σ_{L∋v} 1/|L|
```

Fano: deg=3, |L|=3 → **K(v)=1/2 constant** · Σ_v K = 7/2.

## Contrast

| Model | Curvature | stdev (typical) |
|-------|-----------|-----------------|
| Fano PG(2,2) | constant +0.5 | 0 |
| Golay octad style | intersection 0/2/4 + vertical | > 0 |

## Run

```powershell
python hup/python/fano_discrete_curvature.py
cd lean; lake build TriWeavon.SubRiemannian.FanoDiscreteCurvature
```

Music conserved · Keystone holds
