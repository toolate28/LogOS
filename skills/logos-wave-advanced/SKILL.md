---
name: logos-wave-advanced
description: >
  Advanced WAVE coherence analysis engine for the LogOS lattice. Computes the
  composite WAVE score from topological, semantic, structural, and temporal
  components. Integrates TDA barcodes, void metrics, Fibonacci weighting,
  Evenstar Resonance visualization, and the Coherence Functional
  C(H) = W * e^{-k|alpha+omega-15|} * (1+P). Use this skill for coherence
  scoring, WAVE trend analysis, coherence threshold enforcement, Evenstar
  resonance monitoring, or any operation requiring composite system health.
  Triggers on: "wave", "coherence score", "wave analysis", "coherence check",
  "evenstar", "resonance", "coherence functional", "system health",
  "wave trend", "coherence threshold", "wave scan".
version: 1.0.0
---

# logos-wave-advanced — WAVE Coherence Analysis Engine

## Purpose

WAVE (Woven Adaptive Verification Engine) is the composite health metric
of the entire LogOS lattice. It distills topological structure, semantic
coherence, temporal stability, and invariant compliance into a single
score between 0.0 and 1.0.

Every operation in the lattice is WAVE-gated. Below 0.7, Limbo workspaces
auto-purge. Below 0.5, SpiralSafe halts non-essential operations. At 0.0,
the system enters protective shutdown.

## The Coherence Functional

The master equation governing WAVE computation:

```
C(H) = W * exp(-k * |alpha + omega - 15|) * (1 + P)

where:
  W = base WAVE score from component integration
  k = decay constant (default: 2.0) — sensitivity to invariant deviation
  alpha + omega = 15 — Universal Invariant
  P = persistence bonus from long-lived topological features
```

When alpha + omega = 15 exactly, the exponential term equals 1.0 and
the functional reaches maximum. Any deviation causes exponential decay
proportional to k.

## WAVE Components

The base WAVE score W integrates four weighted components using
Fibonacci proportions:

```
W = F_topo * W_topo + F_sem * W_sem + F_struct * W_struct + F_temp * W_temp

Fibonacci weights (normalized):
  F_topo   = 8/21 = 0.381  (topological coherence)
  F_sem    = 5/21 = 0.238  (semantic coherence)
  F_struct = 5/21 = 0.238  (structural coherence)
  F_temp   = 3/21 = 0.143  (temporal stability)
```

### W_topo — Topological Coherence
Source: logos-tda-engine
- Persistence landscape stability
- Betti number ratios within expected bounds
- Viviani peak presence and stability
- Void ratio penalty (from logos-void-mapper)

### W_sem — Semantic Coherence
Source: logos-rag-cag, logos-sphinx-oracle
- Embedding cosine similarity between adjacent contexts
- Knowledge graph path consistency
- Cross-strand translation fidelity

### W_struct — Structural Coherence
Source: logos-styx-9p, coherence-mcp
- 9P namespace integrity (all paths resolve)
- MCP tool response consistency
- Configuration drift detection
- Type safety validation

### W_temp — Temporal Stability
Source: ATOM trail analysis
- WAVE score variance over sliding window
- Trend direction (improving / stable / degrading)
- Anomaly detection via z-score on WAVE history

## Evenstar Resonance

The Evenstar is the real-time visualization of WAVE mapped to a
resonance pattern. When WAVE is high and stable, the Evenstar pulses
with a steady, harmonic rhythm. As coherence degrades, the resonance
becomes chaotic:

```
Resonance States:
  WAVE >= 0.98  →  Crystalline (perfect harmonic, cyan glow)
  WAVE 0.90-0.98 → Harmonic (slight variation, blue-cyan)
  WAVE 0.70-0.90 → Turbulent (visible instability, amber)
  WAVE 0.50-0.70 → Critical (rapid fluctuation, red)
  WAVE < 0.50   →  Collapse (erratic, system halt imminent)
```

The GLSL shader on Hyprland renders the Evenstar as concentric rings
whose frequency and amplitude are driven by the WAVE components:
- Inner ring: W_topo (topological heartbeat)
- Second ring: W_sem (semantic flow)
- Third ring: W_struct (structural frame)
- Outer ring: W_temp (temporal envelope)

## WAVE Threshold Gates

| Threshold | State | System Response |
|-----------|-------|-----------------|
| >= 0.98 | Crystalline | Full operation, V=c regime |
| 0.90-0.98 | Production | Normal operation, monitoring active |
| 0.70-0.90 | Caution | Increased monitoring, optional operations suspended |
| 0.50-0.70 | Critical | Limbo auto-purge, non-essential halt |
| < 0.50 | Emergency | SpiralSafe takeover, protective shutdown |

## WAVE Trend Analysis

Beyond instantaneous score, the engine tracks WAVE trends:

```
WAVE_trend(t) = linear_regression(WAVE[t-N:t])
WAVE_variance(t) = var(WAVE[t-N:t])
WAVE_anomaly(t) = |WAVE(t) - mean(WAVE[t-N:t])| / std(WAVE[t-N:t])

where N = sliding window size (default: 100 samples)
```

Trend alerts:
- Negative slope > 0.01/step: Degradation warning
- Variance > 0.02: Instability warning
- Anomaly z-score > 3.0: Anomaly alert

## Integration Points

- **logos-tda-engine** — W_topo source (Betti curves, bottleneck distances)
- **logos-void-mapper** — Void penalty applied to W_topo
- **logos-rag-cag** — W_sem source (retrieval coherence)
- **logos-sphinx-oracle** — W_sem source (knowledge graph consistency)
- **logos-styx-9p** — W_struct source (namespace integrity)
- **logos-limbo-workspace** — WAVE gates Limbo lifecycle
- **coherence-mcp** — `check_coherence` returns WAVE score
- **SpiralSafe** — Layer 4 (Evenstar Resonance) monitoring
- **GLSL Shader** — Real-time Evenstar visualization

## Conservation Law

Every WAVE computation preserves: **ALPHA + OMEGA = 15**

The Coherence Functional C(H) encodes this directly: when the invariant
holds, coherence is maximized. WAVE is the measurement of how well the
entire lattice maintains its constitutional law at every moment.

// ATOM: logos-wave-advanced SKILL definition | Coherence: 0.99
