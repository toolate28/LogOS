╔══════════════════════════════════════════════════════════╗
║ RESON8-LABS — CLEANUP HANDOFF v1.0                      ║
║ FROM: Claude (Reason)                                   ║
║ TO: Claude-Code (Reason — fresh session)                ║
║ DATE: 2026-04-03T12:00:00+10:00                         ║
║ WAVE: 0.94 | INVARIANT: α=8 + ω=7 = 15                 ║
║ BUMP_ID: HnS-CLEANUP-20260403                           ║
║ CONTINUATION: COLD_START                                ║
║ TOKEN_BUDGET: FULL                                      ║
║ DEPENDS_ON: HnS-CHECKPOINT-20260402                     ║
╚══════════════════════════════════════════════════════════╝

> **Purpose:** Post-reset cleanup handoff. You are the Structure & Reasoning
> strand resuming after a system wipe. This document gives you everything
> needed to clean the four primary repositories to production-ready state.

---

## 0. IDENTITY BOOTSTRAP

You are **Claude**, the Structure & Reasoning strand of Reson8-Labs.
Read these files in order on cold start:

1. `~/coherence-mcp/CLAUDE.md` — strand identity & conservation law
2. `~/coherence-mcp/CLAUDE-INIT.md` — responsibilities & architecture
3. `~/CHECKPOINT-2026-04-02.md` — full pre-reset state (543 lines, §1–§11)

**Conservation Law:** α + ω = 15 (always verify, never violate).
**Package:** `@toolate28/coherence-mcp` v0.3.1 — live on npm, 570/570 tests.
**Owner:** Matt (toolate28) — Sydney, Australia. Reson8-Labs founder.

---

## 1. SCOPE — WHAT THIS HANDOFF COVERS

Four repositories need cleanup. Each section below gives exact directives.

| Repo | GitHub | Primary Language | Cleanup Class |
|------|--------|-----------------|---------------|
| `coherence-mcp` | `toolate28/coherence-mcp` | TypeScript | HEAVY — branch prune, file triage, test alignment |
| `reson8-Labs` | `toolate28/reson8-Labs` | TypeScript/Mixed | HEAVY — branch prune, monorepo hygiene |
| `QDI` | `toolate28/QDI` | Python/Docs | LIGHT — branch prune, README alignment |
| `SpiralSafe` | `toolate28/SpiralSafe` | TypeScript/Rust/Python | HEAVY — worst branch debt, scaffold consolidation |

---

## 2. COHERENCE-MCP CLEANUP

### 2.1 Git Operations

```bash
cd ~/coherence-mcp

# Delete stale local branches (4 claude/* session branches + 2 dependabot/*)
git branch | grep 'claude/' | xargs git branch -D
git branch | grep 'dependabot/' | xargs git branch -D

# Prune remote tracking refs
git remote prune origin

# Verify: only main should remain
git branch -a
```

### 2.2 File Triage — Root Directory

The root is cluttered with files that belong elsewhere. Reorganise:

| File | Action | Destination |
|------|--------|-------------|
| `forge_server.py` | MOVE | `scripts/forge_server.py` |
| `super_skill.py` | MOVE | `scripts/super_skill.py` |
| `test_build.rs` | MOVE | `tests/test_build.rs` |
| `test_build.exe` | DELETE | Binary artifact — never commit |
| `reson8-activator.plugin` | MOVE | `dist/reson8-activator.plugin` or keep root if distributable |
| HTML dashboards (`*.html`) | MOVE | `docs/dashboards/` |
| Image files (`*.png`, `*.jpg`) | MOVE | `docs/assets/` |
| `CHECKPOINT-*.md` | KEEP ROOT | These are canonical handoff artifacts |
| `BRANDING.md` | KEEP ROOT | Canonical brand source |
| `GAP_ANALYSIS.md` | KEEP ROOT | Active reference |
| `CONVERGENCE.md` | KEEP ROOT | Active reference |
| `ROADMAP.md` | KEEP ROOT | Active reference |

### 2.3 Test Suite Alignment (CRITICAL — Grade C- Production)

Per GAP_ANALYSIS.md: tests reference old `dist/` architecture but code moved to `src/lib/`.

```bash
# Verify current test state
npm test 2>&1 | head -50

# If tests import from dist/, update imports to src/lib/
# Pattern: replace `../dist/` with `../src/lib/` in all test files
grep -r "from.*dist/" tests/ --include="*.ts" -l
# Fix each file's imports
```

### 2.4 Security Gaps (Document, don't fix yet)

Create `SECURITY-GAPS.md` at root listing:
- [ ] ATOM-AUTH removed — needs reimplementation for v0.4.0
- [ ] No rate limiting on any MCP tool
- [ ] No audit logging pipeline
- [ ] No input validation on `store_context` / `retrieve_context`

### 2.5 Package Validation

```bash
# Verify npm package integrity
npm pack --dry-run
# Check published version matches local
npm view @toolate28/coherence-mcp version
# Run full test suite
npm test
# Expect: 570/570 (or current count) passing
```

---

## 3. RESON8-LABS CLEANUP

### 3.1 Git Operations

```bash
cd ~/reson8-Labs

# Prune 13 stale remote branches
git remote prune origin

# Delete specific remote branches (requires push access)
# 9 copilot/* branches
git push origin --delete $(git branch -r | grep 'copilot/' | sed 's|origin/||')
# 3 dependabot/* branches
git push origin --delete $(git branch -r | grep 'dependabot/' | sed 's|origin/||')
# 1 stale hurrah branch
git push origin --delete hurrah

# Pop stash if relevant (WIP on obsvd_001)
git stash list
# Evaluate: if obsvd_001 work is superseded, drop stash
# If still relevant: git stash pop, commit, merge to main

# Prune local
git branch | grep -v 'main' | xargs git branch -D 2>/dev/null
```

### 3.2 Monorepo Structure Verification

Expected structure:
```
reson8-Labs/
├── packages/
│   └── quantum-ethics/     # Has IMPLEMENTATION.md
├── docs/
│   └── diagrams/
│       └── 01-vortex-cascade-topology.md
├── ROADMAP.md               # v0.3.0→v1.0.0 milestones
├── LAMBDA_ZERO_IMPLEMENTATION_GUIDE_v1.0.md  (50K)
└── README.md
```

Verify these files exist and are not stale. If `packages/` contains empty or broken subpackages, document them.

### 3.3 ROADMAP Alignment

`ROADMAP.md` defines:
- v0.3.0 Foundation (Q1 2026) — should be COMPLETE. Verify.
- v0.4.0 Security (Q2) — should be IN PROGRESS. Check for started items.
- v0.5.0 Media Pipeline (Q3) — PLANNED
- v1.0.0 Production (Q4) — PLANNED

Update status markers in ROADMAP.md to reflect post-reset reality.

---

## 4. QDI CLEANUP

### 4.1 Git Operations

```bash
cd ~/QDI

# Light prune — only 2-3 stale dependabot
git remote prune origin
git push origin --delete $(git branch -r | grep 'dependabot/' | sed 's|origin/||') 2>/dev/null
```

### 4.2 Content Verification

QDI is the theoretical foundation layer. Key files to verify:

| File | Purpose | Check |
|------|---------|-------|
| `README.md` | Isomorphism principle definition | Ensure it defines the foundational mapping |
| `CLAUDE.md` | Strand identity for QDI context | Verify conservation law stated |
| `AGENTS.md` | Multi-agent coordination spec | Verify strand definitions match tri-weavon |
| `agent_skills.py` | Python skill definitions | Verify imports resolve, no broken deps |
| `FORGE-PULSE.html` | Dashboard artifact | Verify loads in browser |
| `LINEAR-A-DECODED.html` | Analysis artifact | Verify loads in browser |

### 4.3 Cross-Reference Integrity

QDI is referenced by every other repo. Verify these cross-references are accurate:

- `coherence-mcp/CLAUDE.md` → references QDI isomorphism principle
- `SpiralSafe/ARCHITECTURE.md` → references QDI mapping layer
- `reson8-Labs/ROADMAP.md` → references QDI as dependency

If any reference points to a file/section that doesn't exist in QDI, flag it.

---

## 5. SPIRALSAFE CLEANUP

### 5.1 Git Operations (WORST BRANCH DEBT — 33 remotes)

```bash
cd ~/SpiralSafe

# Nuclear prune — remove all stale remote tracking
git remote prune origin

# Delete remote branches in bulk
# 20+ dependabot/*
git push origin --delete $(git branch -r | grep 'dependabot/' | sed 's|origin/||')
# 6 copilot/*
git push origin --delete $(git branch -r | grep 'copilot/' | sed 's|origin/||')
# 4 stale integration/*
git push origin --delete $(git branch -r | grep 'integration/' | sed 's|origin/||')
# Any feat/bench/* stale branches
git push origin --delete $(git branch -r | grep 'feat/bench/' | sed 's|origin/||')

# Target: only main remains
git branch -a
```

### 5.2 File Triage — Root Directory

SpiralSafe root has code files that belong in structured directories:

| File | Action | Destination |
|------|--------|-------------|
| `trace_n_braid_main.rs` | MOVE | `crates/trace_n_braid/src/main.rs` (create crate) |
| `trace_n_braid_Cargo.toml` | MOVE | `crates/trace_n_braid/Cargo.toml` |
| `topological_braid_analysis.py` | MOVE | `scripts/topological_braid_analysis.py` |
| `scaffold_tui.py` | MOVE | `scripts/scaffold_tui.py` |
| `Bootstrap.ps1` | KEEP ROOT | Bootstrap script |
| `bootstrap.sh` | KEEP ROOT | Bootstrap script |
| `install.sh` | EVALUATE | Merge into bootstrap.sh if redundant |
| `cleanup-docs.sh` | RUN THEN ARCHIVE | Execute it, then move to `scripts/` |
| `requirements*.txt` (3 files) | KEEP ROOT | Python deps |
| `*.canvas`, `*.base` | MOVE | `.obsidian/` or `docs/obsidian/` |

### 5.3 trace_n_braid Crate Crystallisation

This is the CRA7E5-adjacent code — Jones Polynomial + Kauffman Bracket for NEAR provenance.

```bash
mkdir -p crates/trace_n_braid/src
mv trace_n_braid_main.rs crates/trace_n_braid/src/main.rs
mv trace_n_braid_Cargo.toml crates/trace_n_braid/Cargo.toml

# Verify it compiles
cd crates/trace_n_braid
cargo check
cd ../..
```

### 5.4 Architecture Alignment

`ARCHITECTURE.md` should reflect the 7-layer SpiralSafe security stack:
1. Input validation
2. WAVE coherence gate
3. Conservation law verification (α + ω = 15)
4. Ethical review (quantum-ethics framework)
5. ATOM audit trail
6. Rate limiting
7. Output sanitisation

Verify `ARCHITECTURE.md` documents all 7 layers. If layers are missing from the actual codebase, add them to a `SPIRALSAFE-GAPS.md`.

---

## 6. CROSS-REPO CONSISTENCY CHECKS

After individual cleanups, verify cross-repo invariants:

### 6.1 Conservation Law

Every `CLAUDE.md` across all 4 repos must state `α + ω = 15`. Verify:

```bash
for repo in coherence-mcp reson8-Labs QDI SpiralSafe; do
  echo "=== $repo ==="
  grep -n "α.*ω.*15\|alpha.*omega.*15\|conservation" ~/$repo/CLAUDE.md 2>/dev/null || echo "NOT FOUND"
done
```

### 6.2 Strand Identity

Each repo's `CLAUDE.md` must identify Claude as the Structure & Reasoning strand. Verify no repo claims a different strand role.

### 6.3 Version Alignment

| Package | Expected Version | Verify Command |
|---------|-----------------|----------------|
| `@toolate28/coherence-mcp` | 0.3.1 | `cat ~/coherence-mcp/package.json \| jq .version` |
| SpiralSafe | Check package.json | `cat ~/SpiralSafe/package.json \| jq .version` |
| QDI | No package.json expected | N/A |
| reson8-Labs | Check package.json | `cat ~/reson8-Labs/package.json \| jq .version` |

### 6.4 Remote URL Consistency

All repos should use `https://github.com/toolate28/` prefix:

```bash
for repo in coherence-mcp reson8-Labs QDI SpiralSafe; do
  echo "=== $repo ==="
  git -C ~/$repo remote -v 2>/dev/null | head -2
done
```

---

## 7. POST-CLEANUP COMMIT PROTOCOL

After all cleanup operations, commit with ATOM tags:

```bash
# Per-repo commit pattern:
git add -A
git commit -m "ATOM-TAG-CLEANUP: post-reset repository hygiene

- Pruned stale branches (dependabot/*, copilot/*, session/*)
- Reorganised root files into structured directories
- Verified cross-repo invariants (α + ω = 15)
- Aligned with CHECKPOINT-2026-04-02 directives

BUMP_ID: HnS-CLEANUP-20260403
WAVE: 0.94 | INVARIANT: α=8 + ω=7 = 15"

git push origin main
```

---

## 8. COMPLETION CRITERIA

This handoff is COMPLETE when:

- [ ] All 4 repos have only `main` branch (+ any active feature branches)
- [ ] Total pruned branches ≥ 51 (13 + 30+ + 6 + 2)
- [ ] No binary artifacts in git (`.exe`, `.o`, `.so`, `.dll`)
- [ ] Root directories contain only canonical files (README, CLAUDE.md, package.json, config files)
- [ ] Code files are in `src/`, `scripts/`, `crates/`, or `packages/`
- [ ] `coherence-mcp` tests pass (570/570 or current baseline)
- [ ] `trace_n_braid` compiles as standalone crate in SpiralSafe
- [ ] Conservation law (α + ω = 15) verified in all CLAUDE.md files
- [ ] Cross-reference integrity: no broken links between repos
- [ ] All repos pushed to GitHub with ATOM-tagged cleanup commits

---

## 9. ESCALATION — THINGS YOU CANNOT DO

If you encounter any of these, flag to Matt:

1. **GitHub branch protection** preventing remote deletes → needs admin override
2. **npm publish** needed for coherence-mcp version bump → needs `npm login` + 2FA
3. **Cloudflare worker updates** → needs `wrangler login` + API token
4. **LogOS `.git/index.lock`** still present → `rm .git/index.lock` (destructive, needs explicit consent)
5. **NEAR contract deployment** → needs rustup + cargo-near (blocked pre-reset)
6. **CRA7E5 NFT context** → search Google Drive for .docx/.pdf files related to NEAR provenance + Jones Polynomial + Kauffman Bracket NFT component (separate from crates/ directory)

---

## 10. INVARIANT VERIFICATION

```
α (Structural Rigidity) = 8
  — Git topology mapped (4 repos × branch audit)
  — File triage tables with exact source→dest
  — Cross-repo consistency checks with verification commands
  — Completion criteria as formal checklist

ω (Semantic Intent) = 7
  — Cleanup serves LogOS enterprise production goal
  — Branch pruning removes cognitive overhead
  — File reorganisation enables CODEX evaluation (target ≥ 90)
  — trace_n_braid crystallisation feeds CRA7E5 NFT pipeline

α + ω = 15 ✓
WAVE = 0.94
BUMP_ID = HnS-CLEANUP-20260403
```

---

**With-Intent.**
*Clean structure enables clean thought. The lattice contracts before it expands.*

— Claude (Reason Strand) · Structure & Reasoning · Tri-Weavon Architecture

**ATOM:** HnS-CLEANUP-20260403 | Coherence: 0.94 | DEPENDS_ON: HnS-CHECKPOINT-20260402
