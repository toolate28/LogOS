---
name: logos-sphinx-oracle
description: >
  SPHINX oracle for knowledge graph interrogation, causal inference, and
  hypothesis generation within the LogOS lattice. Operates over the unified
  768-D embedding space to answer structured queries about relationships,
  provenance chains, causal paths, and counterfactuals. Use this skill when
  querying the knowledge graph, tracing causal chains, generating hypotheses
  from topological features, performing abductive reasoning, or answering
  "why" and "what-if" questions about the lattice state.
  Triggers on: "sphinx", "oracle", "knowledge graph", "causal", "why",
  "what if", "hypothesis", "inference", "provenance chain", "abductive",
  "reasoning query", "knowledge query", "graph interrogation".
version: 1.0.0
---

# logos-sphinx-oracle — Knowledge Graph Oracle

## Purpose

The SPHINX (Structured Persistent Hierarchical Inference and
eXploration) oracle is the query engine for the LogOS knowledge graph.
It answers questions that require traversing relationships, tracing
causality, and reasoning about the structure of what the system knows.

Where the TDA engine reveals topology and the Void Mapper finds gaps,
SPHINX navigates the semantic space to explain why things are the way
they are and what might happen if they changed.

## Core Capabilities

1. **Structured Query** — Ask precise questions about the knowledge
   graph using a typed query language:
   ```
   SPHINX.query({
     subject: "coherence-mcp",
     relation: "depends_on",
     object: "?",
     depth: 3,
     filter: { wave_score: ">= 0.9" }
   })
   ```
   Returns all entities within 3 hops that coherence-mcp depends on,
   filtered by WAVE score.

2. **Causal Inference** — Trace causal chains through the lattice:
   - Forward causation: "If X changes, what is affected?"
   - Backward causation: "What caused Y to be in this state?"
   - Causal strength: weighted by edge confidence and WAVE score
   - Intervention modeling: "What if we set X to value V?"

3. **Hypothesis Generation** — From topological features detected by
   the TDA engine, generate explanatory hypotheses:
   - H_1 loops suggest circular dependencies — SPHINX traces the cycle
   - H_2 voids suggest missing knowledge — SPHINX proposes what might
     fill them based on adjacent graph structure
   - Persistence anomalies suggest phase transitions — SPHINX
     identifies the likely trigger

4. **Abductive Reasoning** — Given an observation, infer the most
   likely explanation:
   ```
   SPHINX.abduce({
     observation: "WAVE dropped from 0.97 to 0.82 at t=1042",
     context: "deployment of coherence-mcp v2.1",
     candidates: 5
   })
   ```
   Returns ranked explanations with confidence scores.

5. **Provenance Tracing** — Follow the ATOM trail to determine the
   complete history of any artifact:
   - Who created it? (strand attribution)
   - When? (temporal provenance)
   - What was the WAVE score at creation?
   - What braid signature was it part of?
   - Has it been modified? By whom?

6. **Counterfactual Analysis** — Model alternative histories:
   - "What would the WAVE score be if we hadn't added the Cloudflare
     edge layer?"
   - "What if Grok's pulse data was delayed by 500ms?"
   - Uses causal graph structure to estimate counterfactual outcomes

## Query Types

| Query Type | Input | Output | Use Case |
|-----------|-------|--------|----------|
| TRAVERSE | subject, relation, depth | Entity subgraph | Dependency mapping |
| CAUSAL | source, target | Causal path + strength | Root cause analysis |
| ABDUCE | observation, context | Ranked explanations | Debugging, diagnostics |
| HYPOTHESIZE | topological feature | Semantic interpretation | Void filling, loop breaking |
| PROVENANCE | artifact ID | Full ATOM history | Audit, attribution |
| COUNTERFACTUAL | intervention, target | Estimated outcome | Planning, risk assessment |

## Knowledge Graph Structure

The SPHINX operates over a typed property graph stored in the 768-D
embedding space:

```
Nodes:
  - Entity (skills, tools, services, artifacts, strands)
  - Event (ATOM trail entries, WAVE snapshots, deployments)
  - Concept (architectural principles, invariants, patterns)

Edges:
  - depends_on (structural dependency)
  - causes (causal relationship, with strength)
  - contains (hierarchical composition)
  - transforms_to (isomorphic mapping)
  - authored_by (strand attribution)
  - temporal (before/after/concurrent)
```

## Integration Points

- **logos-tda-engine** — Provides topological features for hypothesis
  generation; SPHINX provides semantic interpretation
- **logos-void-mapper** — SPHINX generates remediation hypotheses for
  V_2/V_3 voids
- **logos-wave-advanced** — W_sem component sourced from SPHINX
  query consistency
- **logos-rag-cag** — SPHINX queries can trigger RAG retrieval for
  external knowledge augmentation
- **logos-styx-9p** — Knowledge graph persisted to
  `/bookshelf/graph/` via 9P
- **coherence-mcp** — `map_isomorphism` and `bridge_translate` tools
  use SPHINX for cross-platform relationship mapping
- **ATOM Trail** — Every SPHINX query is logged as an ATOM entry

## Conservation Law

Every SPHINX query preserves: **ALPHA + OMEGA = 15**

The structure of the query (alpha) and the meaning of the answer
(omega) together maintain the invariant. A query that returns no
results is not a failure — it is the detection of a void, which
is itself information that preserves coherence.

// ATOM: logos-sphinx-oracle SKILL definition | Coherence: 0.99
