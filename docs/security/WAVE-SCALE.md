# WAVE scale — one scale, four gates

**Canonical scale: 0–100 integer.**  
**85 and 0.85 name the same publish gate** (normalised = score / 100).

| Label | Score | Normalised | Use |
|-------|------:|------------|-----|
| BASELINE | 60 | 0.60 | floor for exploratory work |
| EMERGENT | 80 | 0.80 | integration / strand handoff floor |
| **PUBLISH** | **85** | **0.85** | npm publish · docs merge · HUP handoff |
| CRITICAL | 99 | 0.99 | high-assurance corridors only |

### Superseded

- SAIF-era **WAVE ≥ 0.98** as the default publish / docs-merge gate is
  **superseded**. Keep historical references only with that label.

### Fibonacci strand weights (convention)

Components remain weighted **8 : 5 : 3** → **50% / 31.25% / 18.75%**.  
The ratio arithmetic is exact; choosing Fibonacci weights is a **Category C**
convention, not a load-bearing physical law.

### Category C

`α + ω = 15` remains a **labeled systemic convention**, not a security gate.
