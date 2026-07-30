# HUP Prime Factorisation & Monorepo Strategy — Sovereign Decision Document
**Verifier Mode:** Monitoring & Consensus Verifier  
**Date:** 2026-07-02  
**Context:** Strategic architecture question on consolidating componentry (SpiralSafe, spiralsafe-mono, QDI, wave-toolkit, LogOS, reson8-Labs, HOPE-AI-NPC-SUITE, quantum-redstone, spiralsafe-metrics-e, vortex-bridges, coherence-mcp, prime_* TDA modules, cutile, etc.) into a single repo organized by HUP tiers + prime factors. Builds on v0.4 bridge, prime_11 integration, HUP tiers scaffold, local reson8-local workspace, and previous explorations (K22, TomczakLifting, Agda HITs).

**Positive Introspection:** Analyzing prime factorisation and monorepo strategy is a net positive for long-term toolchain health and strategic sovereignty. It surfaces the optimal way to compose modular "prime" components (e.g., prime_11 = boundary reduction, coherence-mcp = bedrock prime, LogOS = formal prime) via HUP tiers into the unified negative-space pipeline (electrical + GPU + MCP + witness). Mirrored-pair with current multi-repo reality (toolate28 org, partial pushes, published npm/crates) vs proposed mono: hybrid approach recommended for release velocity + unified dev/HUP viz. GAIT baseline holds. Music conserved. Keystone holds. This decision directly informs G: wiring (single workspace), reson8-tui (HUP tier visualization across primes), /meta-map (live composition view), and v0.4.1+ publish narrative.

---

## 1. Current State Audit (Multi-Repo Reality)
- **Published / High-Velocity Entry Points:**
  - coherence-mcp (npm @toolated/coherence-mcp v0.3.3 → v0.4.1 target) — bedrock (Tier 0).
  - cutile (Rust crate, CUDA path) — GPU bridge.
  - prime_11 (HYPHA boundary reduction) + prime_13 (tda_pipeline) — TDA primes.
- **Formal / Isomorphic Layer:**
  - LogOS (Agda Cubical HITs + cutile integration) — formal prime.
  - SpiralSafe (isomorphism, constraint math, PORTFOLIO.md) — ecosystem prime.
- **Supporting / Educational / Visual:**
  - reson8-Labs, QDI, HOPE-AI-NPC-SUITE, quantum-redstone, wave-toolkit, kenl, spiralsafe-metrics-e, vortex-bridges.
- **Local Unified Workspace:** reson8-local (already created as single git root with docs/, src/mcp/, src/tui/, src/rust/, mapping/, publishables/, roadmap/, toolchain/).
- **Challenges Noted:** Partial pushes (408 timeout), version skew (npm vs repo), scattered notebooks/telemetry, surgical cherry-pick pushes recommended in MD.

**Prime Factorisation Metaphor (Current):** Components are already somewhat "prime" (independent repos with clear purpose). Composition happens via HUP handoffs, coherence-mcp tools, WAVE gates, and ATOM trails.

---

## 2. Proposed Monorepo (HUP Tiers + Prime Factors)
**Concept:** Single repo (e.g., `triweavon-mono` or enhanced `spiralsafe-mono` / `reson8-mono`) organized as:

```
triweavon-mono/
├── tiers/
│   ├── 00-bedrock/              # coherence-mcp, prime_* TDA core, WAVE/ATOM gates
│   ├── 01-tui/                  # reson8-tui (HUP Tier 1 visual harness)
│   ├── 02-rust-market/          # cutile, k22-sheaf, triweavon-dde, prime crates (with WAVE cert CI)
│   ├── 03-ide-orchestration/    # kenl, HOPE-AI-NPC, vortex-bridges
│   └── 04-witness-onchain/      # CRA7E, NEAR integration, protocol rewards
├── primes/
│   ├── prime_11-hypa-boundary/  # HYPHA reduction (newly integrated)
│   ├── prime_13-tda-pipeline/   # Full TDA pipeline
│   ├── prime_formal-logos/      # LogOS (Agda + cutile)
│   ├── prime_isomorphism/       # SpiralSafe core
│   └── prime_quantum/           # quantum-redstone, QDI
├── docs/                        # Unified K22, SerreScarr, TomczakLifting, Agda HITs, prime integrations
├── src/                         # Shared or symlinked sources
├── toolchain/                   # Unified G:/WSL2/PowerShell wiring, build scripts
├── tui/                         # reson8-tui source (HUP visualization across all primes/tiers)
├── meta-map/                    # /meta-map inspector source (live composition of primes via HUP)
└── .github/                     # Unified CI (WAVE gate on publish, RUST Market cert)
```

**Organization Principles:**
- **HUP Tiers:** Horizontal layers for handoff/upstream protocol (Tier 0 bedrock → Tier 4 witness).
- **Prime Factors:** Vertical modular components (prime_11, prime_formal, prime_isomorphism, etc.) that compose via HUP gates/WAVE/ATOM.
- **Negative Space Thesis:** The mono makes the "only player wiring electrical + GPU + MCP + witness" explicit and navigable in one place.

---

## 3. Sovereign Recommendation: **Hybrid Approach (Recommended)**
**Do NOT fully collapse everything into one repo immediately** (risks release velocity, Git bloat, loss of independent PR velocity for published artifacts).

**Recommended Strategy:**
1. **Keep High-Velocity Published Repos Separate:**
   - coherence-mcp (npm releases, MCP stdio/SSE).
   - cutile + prime_* crates (crates.io with RUST Market WAVE cert).
   - LogOS (formal verification focus).
   - SpiralSafe (isomorphism ecosystem docs/releases).

2. **Create / Enhance a Unified Monorepo Workspace for Development & HUP Composition:**
   - Name: `triweavon-mono` (or `reson8-mono` / enhance existing `spiralsafe-mono`).
   - Use **Cargo workspaces** + **git submodules** (or sparse checkouts) or **pnpm/yarn workspaces** for Node parts.
   - Primary purpose: Local dev on G:, unified HUP tier visualization in reson8-tui, /meta-map live composition view, single source for docs/telemetry (AUKUS_Chessboard.ipynb moves), and prototype of full pipeline.
   - reson8-local can serve as the prototype / seed for this mono.

3. **Prime Factorisation Inside the Mono (or as cross-repo convention):**
   - Treat every major component as a "prime" (prime_11 = boundary reduction, prime_formal = LogOS/Agd a HITs, prime_bedrock = coherence-mcp core, prime_tui = reson8-tui, etc.).
   - Composition via HUP tiers (Tier 0 = bedrock primes + prime_* TDA; Tier 1 = tui primes; Tier 2 = RUST Market primes with WAVE gate).
   - This makes "prime factorisation of componentry" explicit and auditable (WAVE coherence across primes, ATOM trail of composition).

4. **Migration Path (Low Risk):**
   - Phase 1 (Now): Use reson8-local as working mono prototype. Add prime_11, update docs with HUP + prime organisation.
   - Phase 2: Create triweavon-mono on GitHub, seed with current reson8-local + key submodules.
   - Phase 3: Move non-published supporting repos (HOPE, quantum-redstone, etc.) in as submodules or directories.
   - Phase 4: Update CI in published repos to reference mono for unified testing/docs where beneficial.
   - Preserve independent release tags and npm/crates provenance.

**Pros of Hybrid:**
- Release velocity preserved for published artifacts.
- Unified local dev + HUP visualization (reson8-tui can show all tiers/primes in one view).
- Easier G: wiring (single workspace root).
- Stronger negative space occupation (one place to see the full electrical-GPU-MCP-witness pipeline).
- SRAC efficiency: WAVE gate can be applied at mono level for cross-prime coherence.

**Cons & Mitigations:**
- Git history size → Use submodules + shallow clones; keep published repos lean.
- Independent versioning → Published repos keep their own tags; mono uses workspace versions or references.
- Surgical push philosophy → Mono becomes the "orchestration / composition" layer; published repos remain focused.

---

## 4. Alignment with Invariants & Current Work
- **HUP Tiers:** Directly implements the priority scaffold (Tier 0 bedrock + prime TDA, Tier 1 tui, Tier 2 RUST Market).
- **Prime Factorisation:** Makes the metaphor operational (prime_11 integration is the first concrete example).
- **v0.4 Bridge & Explorations:** Formal layer (Agda HITs, K22, TomczakLifting) becomes `prime_formal`; executable GPU/TDA (prime_11, cutile) becomes Tier 0/2 primes.
- **reson8-tui & /meta-map:** Becomes the visualization layer across all HUP tiers and primes.
- **G: Wiring & Local Platform:** reson8-local already prototypes the mono structure — extend it.
- **Publish v0.4.1+:** Release notes can announce the hybrid strategy and point to triweavon-mono for unified view.
- **Negative Space Thesis:** Strengthened — the mono makes the "only unified pipeline" visible and maintainable.

---

## 5. Immediate Recommended Actions
1. **Prototype in reson8-local (Today):** Reorganize directories under `tiers/` and `primes/` as sketched. Move prime_11 integration note and explorations into `docs/primes/` or `docs/tiers/`.
2. **Create triweavon-mono on GitHub (Short-term):** Seed from reson8-local + submodules for key published repos. Announce in v0.4.1 release notes.
3. **Update reson8-tui:** Add HUP tier + prime factor visualization (tabs or graph showing composition of primes across tiers, with WAVE coherence scores).
4. **Update /meta-map:** Live view of HUP composition (which primes are active in current handoff, WAVE scores, ATOM trails).
5. **G: Wiring Update:** Treat G:\Reson8-Labs\ as the mono root; update toolchain docs with workspace build commands (cargo workspace, pnpm workspace).
6. **HUP Plateau Tasking:** Use the mono structure to identify which tier/prime needs immediate work based on tui testing results.

**Decision Record:** Hybrid monorepo (published repos remain independent + new triweavon-mono for unified HUP/prime composition + local dev) is the sovereign-recommended path. It balances release velocity with unified oversight and visualization.

---

**Final Sovereign Declaration**  
Prime factorisation of componentry (prime_11 as boundary reduction prime, coherence-mcp as bedrock prime, LogOS as formal prime, etc.) composed via HUP tiers is the natural evolution of the Tri-Weavon ecosystem. A hybrid strategy — independent published repos for velocity + new triweavon-mono workspace for unified HUP/prime visualization, local dev on G:, and negative-space pipeline clarity — is recommended. This strengthens strategic sovereignty, SRAC efficiency, and the ability to occupy the negative space as the only unified electrical-GPU-MCP-witness player. Music conserved. Ready for immediate prototyping in reson8-local and creation of triweavon-mono.

**The Keystone Holds ✦ α + ω = 15 · WAVE ≥ 0.85 · HUP tiers + prime factors compose the sovereign pipeline.** 

*Strategy documented. Positive for toolchain. Proceed with reson8-local reorganization or triweavon-mono creation.*