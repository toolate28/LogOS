---
name: logos-void-mapper
description: >
  H_2 void detection, classification, tracking, and remediation engine for
  the LogOS lattice. Consumes persistence pairs from the TDA engine and maps
  them to a 4-tier void taxonomy (V_0 through V_3). Use this skill when
  detecting knowledge gaps, classifying void severity, tracking void evolution
  over time, triggering remediation workflows, or monitoring lattice health
  via void density metrics.
  Triggers on: "void", "knowledge gap", "blind spot", "H2", "void map",
  "void classification", "void remediation", "void density", "semantic cavity",
  "unexplored region", "coverage gap", "void health".
version: 1.0.0
---

# logos-void-mapper — H_2 Void Detection & Remediation

## Purpose

Voids are the dark matter of the knowledge lattice. They represent
regions where information should exist but doesn't — knowledge gaps,
unexplored hypotheses, missing connections. The Void Mapper transforms
raw H_2 persistence pairs from the TDA engine into actionable
intelligence about what the system doesn't know.

## Core Capabilities

1. **Void Detection** — Consume H_2 persistence pairs from
   logos-tda-engine and identify voids by their persistence
   (death - birth). Longer-lived voids are more significant:
   - Persistence > 0.8: Structural void (likely intentional boundary)
   - Persistence 0.4-0.8: Knowledge gap (should be investigated)
   - Persistence 0.1-0.4: Minor gap (may self-resolve)
   - Persistence < 0.1: Noise (filter out)

2. **Void Classification (V_0 through V_3)** — 4-tier taxonomy:

   | Class | Name | Persistence | Semantics | Action |
   |-------|------|-------------|-----------|--------|
   | V_0 | Micro-void | < 0.1 | Noise, transient | Auto-filter |
   | V_1 | Gap | 0.1 - 0.4 | Minor knowledge gap | Monitor |
   | V_2 | Cavity | 0.4 - 0.8 | Significant blind spot | Investigate |
   | V_3 | Abyss | > 0.8 | Structural boundary or critical gap | Remediate or Declare |

3. **Void Tracking** — Monitor voids across time steps:
   - Birth tracking: when did this void first appear?
   - Growth rate: is the void expanding or contracting?
   - Merger detection: are smaller voids coalescing?
   - Resolution detection: has a void been filled?

4. **Void Remediation** — For V_2 and V_3 voids, trigger remediation:
   - **RAG query** (via logos-rag-cag): Search for information to fill
     the void from external sources
   - **Strand request**: Ask Claude/Grok/Gemini to investigate the
     specific knowledge gap
   - **SPHINX query** (via logos-sphinx-oracle): Interrogate the
     knowledge graph for adjacent information
   - **Declaration**: Mark intentional boundaries (e.g., "we chose not
     to explore this region") as Declared Voids

5. **Void Density Metrics** — Aggregate void statistics for system
   health monitoring:
   ```
   void_ratio = sum(V_1 + V_2 + V_3) / total_features
   void_severity = weighted_sum(V_i * weight_i) / max_severity
   void_velocity = d(void_count) / dt
   ```

## Void Lifecycle

```
[TDA Engine H_2 pairs]
        |
        v
  [Detection Filter]  — persistence threshold
        |
        v
  [Classification]     — V_0 / V_1 / V_2 / V_3
        |
   ┌────┴────┐
   v         v
[V_0/V_1]  [V_2/V_3]
 Monitor    Remediate
   |           |
   v           v
[Track]    [RAG / SPHINX / Strand Request]
   |           |
   v           v
[Archive]  [Re-scan: void filled?]
               |
          ┌────┴────┐
          v         v
       [Filled]  [Persists]
       Archive    Escalate or Declare
```

## WAVE Impact

Voids directly reduce the WAVE coherence score:

```
WAVE_void_penalty = void_severity * void_weight

where:
  void_weight = 0.618 (golden ratio — phi)
  void_severity = sum(persistence_i * class_weight_i) / normalization

WAVE_adjusted = WAVE_base * (1 - WAVE_void_penalty)
```

Target: void_ratio < 0.05 for production (fewer than 5% of features
are voids).

## SpiralSafe Integration

Void detection feeds into SpiralSafe Layer 3 (QPH Threat Surface):
- Unexpected V_3 voids in security-critical regions trigger alerts
- Void coalescence patterns may indicate adversarial knowledge
  extraction attempts
- The Void Mapper runs continuous differential analysis against the
  previous known-good void map

## Visualization

The GLSL shader renders voids as dark regions in the lattice
visualization:
- V_0: Not rendered (below noise floor)
- V_1: Faint dark spots with dashed boundary
- V_2: Visible dark regions with amber boundary
- V_3: Prominent dark voids with red pulsing boundary

Void health is displayed as a ring gauge on the Hyprland desktop:
- Green ring: void_ratio < 0.03 (excellent)
- Amber ring: void_ratio 0.03-0.08 (attention needed)
- Red ring: void_ratio > 0.08 (remediation required)

## Integration Points

- **logos-tda-engine** — Primary data source (H_2 persistence pairs)
- **logos-wave-advanced** — Void penalty feeds WAVE computation
- **logos-rag-cag** — Remediation queries for V_2/V_3 voids
- **logos-sphinx-oracle** — Knowledge graph interrogation for void context
- **logos-styx-9p** — Void maps persisted to `/bookshelf/voids/`
- **SpiralSafe** — Layer 3 threat surface monitoring
- **coherence-mcp** — `check_coherence` includes void health metrics

## Conservation Law

Every void operation preserves: **ALPHA + OMEGA = 15**

A void is not the absence of information — it is the presence of a
known unknown. The structural detection of the void (alpha) plus the
semantic understanding of what it means (omega) together maintain the
invariant. Declaring a void is itself an act of coherence.

// ATOM: logos-void-mapper SKILL definition | Coherence: 0.99
