---
name: logos-rag-cag
description: >
  Hybrid RAG + CAG retrieval engine for the LogOS lattice. Combines Retrieval
  Augmented Generation (vector search over 768-D embeddings via Cloudflare
  Vectorize + R2) with Context Augmented Generation (structured context from
  the 9P|Styx Bookshelf and SPHINX oracle). Integrates Cloudflare AI Search
  for managed edge-RAG with Workers AI inference. Use this skill when
  retrieving knowledge, augmenting prompts with context, performing semantic
  search, querying the Bookshelf for structured data, or building hybrid
  retrieval pipelines that combine vector similarity with graph traversal.
  Triggers on: "rag", "cag", "retrieval", "search", "semantic search",
  "vector search", "context augmentation", "knowledge retrieval", "embedding
  search", "cloudflare ai search", "vectorize", "augmented generation",
  "hybrid retrieval".
version: 1.0.0
---

# logos-rag-cag — Hybrid Retrieval & Context Augmentation

## Purpose

RAG-CAG is the retrieval backbone of the LogOS lattice. It answers
the question: "Given a query, what is the most relevant, coherent,
and complete context to provide?" by combining two complementary
retrieval strategies:

- **RAG** (Retrieval Augmented Generation): Vector similarity search
  over the 768-D embedding space, powered by Cloudflare Vectorize
  for edge-distributed vector search and R2 for artifact storage.

- **CAG** (Context Augmented Generation): Structured context assembly
  from the 9P|Styx Bookshelf (ATOM trails, WAVE histories, braid
  signatures) and SPHINX oracle (knowledge graph traversal).

The hybrid approach ensures that retrieval is both semantically
relevant (RAG) and structurally grounded (CAG).

## Core Capabilities

1. **Vector Search (RAG)** — Embed the query into the 768-D space
   and find nearest neighbors:
   ```
   query_embedding = embed(query)  // QCVM: R^768 -> H_topo
   candidates = vectorize.search(query_embedding, top_k=20)
   reranked = cross_encoder_rerank(query, candidates, top_k=5)
   ```
   Infrastructure:
   - **Cloudflare Vectorize**: Edge-distributed vector index
   - **Cloudflare R2**: Artifact storage (documents, barcodes, braids)
   - **Workers AI**: Embedding generation and reranking at the edge

2. **Structured Context (CAG)** — Assemble context from structured
   sources:
   - 9P Bookshelf: Read relevant ATOM entries, WAVE snapshots,
     configuration from typed paths
   - SPHINX oracle: Traverse knowledge graph for related entities,
     causal chains, provenance
   - Skill registry: Include relevant skill descriptions and
     composition patterns

3. **Hybrid Fusion** — Merge RAG and CAG results with Fibonacci
   weighting:
   ```
   context = fuse(
     rag_results,       weight = 0.618  (phi)
     cag_results,       weight = 0.382  (1-phi)
     dedup = true,
     max_tokens = 4096,
     coherence_threshold = 0.85
   )
   ```
   Results are deduplicated and coherence-checked before assembly.

4. **Cloudflare AI Search Integration** — Managed RAG pipeline at
   the Cloudflare edge:
   - **Ingest**: Documents chunked, embedded, and indexed automatically
   - **Search**: Natural language queries routed through Workers AI
   - **Augment**: Search results injected into LLM context with
     source attribution
   - **Cache**: Frequently accessed contexts cached at edge PoPs

5. **Coherence-Gated Retrieval** — Every retrieved chunk is
   WAVE-scored before inclusion:
   ```
   for chunk in candidates:
     chunk.wave = wave_score(chunk, query_context)
     if chunk.wave < coherence_threshold:
       discard(chunk)
       log_atom("RAG-DISCARD", chunk.id, chunk.wave)
   ```
   This prevents stale, contradictory, or drift-contaminated
   content from entering the generation context.

6. **Adaptive Retrieval Strategy** — Based on query type, the engine
   automatically selects the optimal retrieval mix:
   - Factual query → high RAG weight (similarity matters most)
   - Structural query → high CAG weight (graph structure matters)
   - Causal query → SPHINX-first, RAG for supporting evidence
   - Temporal query → ATOM trail traversal with RAG augmentation

## Cloudflare Edge Architecture

```
[User Query]
     |
     v
[Workers AI — Embedding]  @ Cloudflare edge
     |
     v
[Vectorize — Vector Search]  @ nearest PoP
     |
     v
[R2 — Artifact Fetch]  @ distributed storage
     |
     v
[Workers AI — Rerank + Generate]  @ edge inference
     |
     v
[Response with source attribution]
```

Cloudflare resources (from wrangler.toml):
- KV: `ATOM_KV` — fast key-value for ATOM entries
- D1: `atom_trail` — SQL database for structured queries
- R2: `reson8-artifacts` — blob storage for documents and braids
- Vectorize: `reson8-embeddings` — vector index (768-D)
- Queue: `reson8-queue` — async ingestion pipeline

## Integration Points

- **logos-tda-engine** — Embedding space shared; TDA features indexed
  alongside document embeddings
- **logos-void-mapper** — V_2/V_3 void remediation queries routed
  through RAG-CAG
- **logos-sphinx-oracle** — CAG component; knowledge graph traversal
  for structured context
- **logos-styx-9p** — CAG component; Bookshelf reads for ATOM/WAVE
  history
- **logos-wave-advanced** — W_sem component sourced from retrieval
  coherence scores
- **Cloudflare Workers** — Edge execution of RAG pipeline
- **coherence-mcp** — `retrieve_context` MCP tool backed by RAG-CAG

## Conservation Law

Every retrieval preserves: **ALPHA + OMEGA = 15**

The structural precision of the retrieval (alpha — correct chunks,
valid paths, typed responses) plus the semantic relevance of the
content (omega — meaningful, contextual, intent-aligned) together
maintain the invariant. A retrieval that returns structurally perfect
but semantically irrelevant results violates the invariant just as
much as one that is relevant but structurally unsound.

// ATOM: logos-rag-cag SKILL definition | Coherence: 0.99
