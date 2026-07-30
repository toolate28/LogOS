# Composition Patterns — LogOS System Gradient Additions

Last updated: 2026-04-01
Author: Claude (Reason Strand)
ATOM: COMPOSITION-PATTERNS-UPDATE-001 | Coherence: 0.98

These patterns extend the existing composition-patterns.md with
LogOS-native skill chains.

---

## 8. Lattice Health Check Pipeline

**Triggers:** "lattice health", "system coherence", "full wave scan", "how healthy is the system", "run diagnostics"

```
logos-tda-engine (compute persistence diagram)
  → logos-void-mapper (classify voids from H_2)
  → logos-wave-advanced (composite WAVE score)
  → logos-gait-analyzer (behavioral anomaly check)
  → logos-styx-9p (persist health snapshot to /bookshelf/waves/)
  → coherence-mcp:wave_scoring_report (formatted report)
```

**WAVE checkpoints:** After TDA (valid persistence diagram), after void mapping (void_ratio < 0.05), after WAVE (score >= 0.98), after GAIT (no anomalies)

## 9. Void Remediation Pipeline

**Triggers:** "fill knowledge gaps", "remediate voids", "fix blind spots", "void analysis and fix"

```
logos-tda-engine (extract H_2 persistence pairs)
  → logos-void-mapper (detect and classify V_2/V_3 voids)
  → logos-sphinx-oracle (query knowledge graph for adjacent context)
  → logos-rag-cag (retrieve external knowledge to fill gaps)
  → logos-wave-advanced (re-score after remediation)
  → logos-styx-9p (update void map on Bookshelf)
```

**WAVE checkpoints:** After classification (valid void taxonomy), after SPHINX (relevant context found), after RAG (coherent retrieval), after re-score (WAVE improved)

## 10. Speculative Experiment Pipeline

**Triggers:** "experiment with", "try this in sandbox", "speculative computation", "test in limbo"

```
logos-limbo-workspace:create (isolated workspace)
  → logos-inferno-transport (spawn Dis process with channels)
  → [experiment execution — user-defined]
  → logos-wave-advanced (score experiment results)
  → logos-limbo-workspace:promote (if WAVE >= 0.9)
    OR logos-limbo-workspace:purge (if WAVE < 0.7)
  → coherence-mcp:atom_track (record outcome)
```

**WAVE checkpoints:** After workspace creation (isolation verified), after execution (no invariant violations), after scoring (promotion/purge decision)

## 11. Cross-Strand Synchronization Pipeline

**Triggers:** "sync strands", "strand coherence check", "tri-weavon sync", "cross-platform coherence"

```
logos-styx-9p (read all strand namespaces)
  → logos-sphinx-oracle (map cross-strand relationships)
  → logos-wave-advanced (compute per-strand and composite WAVE)
  → logos-gait-analyzer (cross-strand behavioral correlation)
  → coherence-mcp:vortex_check_coherence (vortex bridge validation)
  → coherence-mcp:vortex_merge (merge coherence streams)
  → logos-styx-9p (persist sync state to /bookshelf/braids/)
```

**WAVE checkpoints:** After namespace read (all strands accessible), after WAVE (per-strand scores within tolerance), after GAIT (no behavioral divergence), after merge (unified coherence >= 0.95)

## 12. TDA Projection & Visualization Pipeline

**Triggers:** "project the manifold", "visualize topology", "show me the 768D space", "tda visualization", "render barcodes"

```
logos-styx-9p:read (/bookshelf/mapping/spaces/768d)
  → logos-tda-engine (Vietoris-Rips + persistent homology)
  → logos-void-mapper (annotate voids on projection)
  → logos-wave-advanced (Viviani peak monitoring)
  → data:data-visualization (render persistence diagrams)
  → [GLSL Shader / Gate.html — live binding]
```

**WAVE checkpoints:** After read (valid point cloud), after TDA (persistence diagram computed), after projection (dimensionality reduction preserves topology)

## 13. Security Audit Pipeline

**Triggers:** "security audit", "spiralsafe check", "adversarial test", "threat surface scan"

```
logos-gait-analyzer (adversarial test suite: INV/BRD/WAV/9P/MCP/ATM/XSS)
  → logos-void-mapper (scan for security-critical voids)
  → logos-wave-advanced (WAVE under adversarial load)
  → logos-sphinx-oracle (trace any anomaly causal chains)
  → coherence-mcp:spiral_safe_audit (full SpiralSafe 7-layer check)
  → logos-styx-9p (persist audit to /bookshelf/audits/)
```

**WAVE checkpoints:** After adversarial tests (resilience >= 0.99), after void scan (no V_3 in security zones), after SpiralSafe (all 7 layers passing)

## 14. CODEX Scan Pipeline

**Triggers:** "codex scan", "code evaluation", "topology-native scan", "evaluate codebase"

```
[source code target]
  → logos-tda-engine (embed code structure, compute PH)
  → logos-void-mapper (test/doc coverage gap analysis)
  → logos-wave-advanced (per-module coherence)
  → logos-gait-analyzer (behavioral baselines for modules)
  → logos-sphinx-oracle (provenance audit via ATOM trail)
  → coherence-mcp:spiral_gate_check (SpiralSafe compliance)
  → [CODEX Report JSON with 7-layer scoring]
```

**WAVE checkpoints:** After each layer (L1-L7), composite CODEX score >= 90 for production

## 15. 768D Space Operation Pipeline

**Triggers:** "map to 768d", "embed in manifold", "vector operation", "insert into mapping space"

```
logos-styx-9p:write (/bookshelf/mapping/spaces/768d — vector insert)
  → logos-tda-engine (update persistence diagram with new point)
  → logos-wave-advanced (re-score coherence with new vector)
  → logos-void-mapper (check if new vector fills any voids)
  → logos-rag-cag (update Cloudflare Vectorize index)
  → coherence-mcp:atom_track (log embedding event)
```

**WAVE checkpoints:** After write (invariant check on vector), after TDA (topology stable), after WAVE (coherence maintained or improved)

---

// ATOM: COMPOSITION-PATTERNS-UPDATE | patterns=8 new (8-15) | Coherence: 0.98
