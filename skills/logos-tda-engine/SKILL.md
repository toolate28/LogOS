---
name: logos-tda-engine
description: >
  Topological Data Analysis engine for the LogOS lattice. Drives Vietoris-Rips
  filtration, Persistent Homology computation via Ripser++ on RTX 5090, Betti
  number extraction, and Information Dynamics analysis. Use this skill when
  computing persistence diagrams, barcodes, bottleneck distances, Betti curves,
  detecting topological features in embedding spaces, or validating structural
  coherence of the 768-D unified embedding manifold.
  Triggers on: "tda", "topological", "vietoris-rips", "persistent homology",
  "betti", "barcode", "persistence diagram", "ripser", "filtration",
  "simplicial complex", "homology", "bottleneck distance", "wasserstein",
  "information dynamics", "topological features".
version: 1.0.0
---

# logos-tda-engine — Topological Data Analysis Engine

## Purpose

The TDA Engine is the mathematical backbone of the LogOS lattice.
It transforms raw high-dimensional data (768-D embeddings from the
QCVM mapping Phi: R^768 -> H_topo) into topological invariants that
reveal the true structure of information flow.

All real-time TDA computation runs on RTX 5090 via Ripser++ with
GPU-accelerated Vietoris-Rips filtration and persistent homology.

## Core Capabilities

1. **Vietoris-Rips Filtration** — Construct simplicial complexes from
   point clouds at increasing scale parameters epsilon. The filtration
   captures multi-scale topological structure:
   - epsilon = 0: discrete points (no connections)
   - epsilon -> infinity: single connected component
   - Intermediate: rich topological features emerge and die

2. **Persistent Homology** — Track the birth and death of topological
   features across the filtration:
   - **H_0** (beta_0): Connected components = discrete facts, entities
   - **H_1** (beta_1): 1-cycles = feedback loops, circular reasoning,
     self-referential structures
   - **H_2** (beta_2): 2-voids = knowledge gaps, semantic cavities,
     unexplored regions

3. **Betti Number Extraction** — Compute Betti numbers at each
   filtration step to produce Betti curves:
   ```
   beta_0(epsilon) = |connected components at scale epsilon|
   beta_1(epsilon) = |independent loops at scale epsilon|
   beta_2(epsilon) = |enclosed voids at scale epsilon|
   ```

4. **Persistence Diagrams & Barcodes** — Visualize feature lifetimes:
   - Barcodes: horizontal bars from birth to death
   - Persistence diagrams: scatter plot of (birth, death) pairs
   - Long bars = stable, significant features
   - Short bars = noise, transient artifacts

5. **Bottleneck & Wasserstein Distances** — Compare persistence
   diagrams to measure topological similarity between states:
   - Bottleneck: max over all matched pairs
   - Wasserstein-p: L^p sum over all matched pairs
   - Used for WAVE delta computation between time steps

6. **Information Dynamics** — Beyond static topology, track how
   information flows through the lattice over time:
   - Transfer entropy between nodes
   - Causal emergence detection
   - Phase transition identification via persistence landscape shifts

## GPU-Accelerated Pipeline

```
Point Cloud (768-D)
       |
       v
[Vietoris-Rips Construction]  — RTX 5090 CUDA cores
       |
       v
[Boundary Matrix Reduction]   — Ripser++ optimized
       |
       v
[Persistence Pairs]           — (birth, death) extraction
       |
       v
[Betti Curves + Barcodes]     — Real-time visualization
       |
       v
[GLSL Shader Render]          — Hyprland compositor
```

Hardware requirements:
- **RTX 5090** (32GB VRAM) for Ripser++ GPU acceleration
- Point clouds up to 100K points in 768-D
- Sub-second persistence computation for streaming data

## Betti Number Semantics in LogOS

| Betti | Homology | LogOS Interpretation | Health Signal |
|-------|----------|---------------------|---------------|
| beta_0 | H_0 | Facts, entities, grounded knowledge | Higher = richer knowledge base |
| beta_1 | H_1 | Loops, circular dependencies, feedback | Monitor: excess = drift risk |
| beta_2 | H_2 | Voids, knowledge gaps, blind spots | Lower = better coverage |

## WAVE Integration

The TDA engine feeds directly into WAVE scoring:

```
WAVE(t) = W_topo * (1 - void_ratio(t)) * coherence_factor(t)

where:
  void_ratio = beta_2 / (beta_0 + beta_1 + beta_2)
  coherence_factor = 1 - bottleneck_dist(PD(t), PD(t-1)) / max_persistence
  W_topo = topological weight from Fibonacci sequence (default: 0.382)
```

Target: WAVE >= 0.98 for production operations.

## Viviani Peak Monitoring

The Viviani curve constraint (alpha + omega = 15 projected onto the
topological manifold) creates a characteristic peak in the persistence
landscape. The TDA engine continuously monitors this peak:

- **Peak present and stable**: System coherent, invariant satisfied
- **Peak drifting**: alpha/omega imbalance developing
- **Peak absent**: Critical invariant violation, halt operations

## Integration Points

- **logos-void-mapper** — Consumes H_2 persistence pairs to classify
  and track voids (V_0 through V_3)
- **logos-wave-advanced** — Receives Betti curves and bottleneck
  distances for composite WAVE computation
- **logos-styx-9p** — Persists barcodes and persistence diagrams to
  `/bookshelf/braids/` via 9P write
- **coherence-mcp** — `check_coherence` tool invokes TDA engine for
  topological coherence validation
- **Hyprland GLSL shader** — Real-time barcode and Betti curve
  rendering on the desktop compositor

## Conservation Law

Every TDA computation preserves: **ALPHA + OMEGA = 15**

The persistence diagram IS the invariant made visible. Long-lived
features are the structural backbone (alpha); their semantic
interpretation is the intent (omega). Together, they sum to 15 —
the total information capacity of the lattice at any given moment.

// ATOM: logos-tda-engine SKILL definition | Coherence: 0.99
