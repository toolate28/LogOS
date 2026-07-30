# Formal core set — Lean · Agda · docs waist

**ATOM:** `ATOM-CORE-SET-STANDARDISE-20260730`  
**Invariant:** α + ω = 15 (Category C label only)  
**WAVE publish gate:** **85 / 100** (= 0.85). SAIF 0.98 superseded as default.  
**Pin:** Lean 4.8.0 + mathlib4 v4.8.0 · Agda cubical (vendor, not committed)

This is the **standard map** of what counts as formal *core* for push, CI, and handoff.
Everything else is SlowStep / A-literature until receipt.

---

## 1. Lean core (`lean/`)

| Path | Role | Status target |
|------|------|----------------|
| `lean/lean-toolchain` · `lakefile.lean` · `lake-manifest.json` | Pin | always |
| `lean/K22/HexacodeGolay.lean` | GF(4) · G₂₄ · 759 octads · ∩≤4 | A / green |
| `lean/K22/MOG/SteinerDoubleCount.lean` | **S1–S6** packing → Steiner on Golay pin | A / green |
| `lean/K22/MOG/MonomialWitness.lean` | π transport · CB-1 residual | B until lake-green |
| `lean/K22/MiracleOctadGenerator.lean` | Conway MOG recognition | B (`sorry` on direct Steiner) |
| `lean/K22/Existence.lean` · `K22Log.lean` · tactics | bridges / telemetry | core scaffold |
| `lean/TriWeavon/ConservationInvariant.lean` | α+ω software constant | core |
| `lean/AgdaLeanBridge.md` | name map Lean ↔ Agda | core |
| `lean/K22/README.md` · `lean/CORE.md` | module status | core |

**Do not commit:** `lean/.lake/**` (build products).

### S5 success (receipt)

```text
lake build K22.MOG.SteinerDoubleCount   # green, 0 sorry
# golayOctadBlocks_card : card = 759
# golayOctadBlocks_pack : packing
# golay_octads_form_steiner : S(5,8,24) on Golay blocks
```

---

## 2. Agda core (`agda/`)

| Path | Role | Status target |
|------|------|----------------|
| `agda/TriWeavon.agda-lib` · `Makefile` · `.gitignore` | project shell | always |
| `agda/src/Everything.agda` | aggregate typecheck entry | core |
| `agda/src/TriWeavon/Core.agda` | foundation | core |
| `agda/src/TriWeavon/ConservationRMatrix.agda` | conservation twin of Lean | core |
| `agda/src/TriWeavon/K22/SerreScarr.agda` · `SerrePage.agda` | Serre / page | core |
| `agda/src/TriWeavon/Tomczak/**` (via Everything) | Tomczak lift | core / B holes ok |
| `agda/src/TriWeavon/SubRiemannian/**` | Mehler / geometry | core scaffold |
| `agda/docs/CORE.md` | this waist for Agda readers | core |
| `agda/scripts/{check,html,vendor}.ps1` | local tooling | core |

**Do not commit:** `agda/vendor/**`, `*.zip`, large `MAlonzo/**` dumps.

---

## 3. Docs core (`docs/`)

| Path | Role |
|------|------|
| `docs/formal/CORE-SET.md` | this file |
| `docs/formal/K22-TELEMETRY-MCP-TUI-BRIDGE.md` | Stage 0–4 telemetry |
| `docs/formal/*STEINER*` · `*GOLAY*` · `*CB1*` | Steiner discharge lane |
| `docs/security/**` | SECURITY companions · WAVE · MCP |
| `docs/componentry/ATOMS/ATOM-STEINER-LANE-CHECKPOINT-*.md` | formal handoff ATOM |
| `docs/encyclopedia-equilibria/certificates/cert_latest.json` | **last emittance certificate only** |
| `SECURITY.md` (repo root) | supported surfaces · scanning |

**Do not commit as core:** mood JPGs, encyclopedia misc dumps, duplicate `(1).agda` archives unless explicitly promoted.

---

## 4. Executable / verify companions (core for CI)

| Path | Role |
|------|------|
| `.github/workflows/*` · `codeql/` · `dependabot.yml` | verification pipeline |
| `ops/ci/*` · `ops/githooks/*` · `ops/mcp/*` | local mirrors · registry · manifold API |
| `mcps/coherence-mcp/tools/*.json` | tool schemas (snake_case 0.4.x) |
| `coherence-mcp/coherence-site/public/manifold/` | 2D projection UI |

---

## 5. Emission certificate policy

- Keep **one** rolling latest: `docs/encyclopedia-equilibria/certificates/cert_latest.json`
- Optional domain cert: `lean/K22/MOG/existence_certificate_mog.json` (MOG preflight)
- Claude Code certs under `.atom-trail/certs/` stay **local** (gitignored) — not push surface
- No silent promotion: `tomczak_preserved` · capability ≠ authority

---

## 6. Category labels

| Cat | Meaning |
|-----|---------|
| A | Verified on pin (`native_decide` / closed proof) |
| B | Bounded approx / residual / `sorry` honesty |
| C | Convention (α+ω, Fibonacci WAVE weights) |
| D | Open research |

---

Hope&&Sauced · Keystone holds · Music conserved
