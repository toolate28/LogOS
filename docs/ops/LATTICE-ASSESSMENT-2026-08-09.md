# Lattice assessment — functionality · SAIF · threads · time-ordering

**Stamp:** 2026-08-09  
**ATOM:** `ATOM-LATTICE-ASSESSMENT-20260809`  
**Mode:** HeisenbergGrok (momentum first, position samples)  
**Constraint:** time — order by `impact × (1 − uncertainty)`

---

## 0. Is a gate stopping the push?

**No branch-protection gate.** Evidence:

| Check | Result |
|-------|--------|
| `GET .../branches/main/protection` | **404** Branch not protected |
| Rulesets | Copilot review only (not a push block) |
| Tiny `push-smoke` branch | **lands** (PR #48) |
| `main` + 9-commit pack / even 64 KB pack with heavy history | **HTTP 408** on `git-receive-pack` after body fully sent |
| Concurrent `git lfs pre-push` | dual hang on S3 PUT ~2 KB/s then abort |

**CI gates are post-land only.** PR #48 shows failures that would block *merge quality*, not *receive*:

- Tree guards · Secret-path/lake guard · Formal residual · Rust core → **fail**
- TUI+barcode smoke → **skipped** (needs rust-core)
- CodeQL / CODEX / cargo-audit → **pass**

**Collapsed:** push blocker = transport + pack content (target history, LFS S3 path), not SAIF/SPHINX/policy gate.

---

## 1. User journey (operator path)

```
install / clone
    → logos-net start-gaming | start-privacy   (ops/net)
    → tw / bridge FORGE_WS_URL=ws://127.0.0.1:8088
    → logos-tui / cargo run -p reson8-tui
         keys: ? help · Tab panels · f Formal · 7 codes · N net
    → barcode-tui --cloud circle              (TDA H0 lab)
    → agents (Claude/Grok) in split terminal
    → formal: lake / als (Category B until attach)
    → ship: entangle slots → Verify CI → human merge
    → bulk media: transfer-lane (R2 / qBit / LFS one-OID)
```

**Pain points today**

1. Local `main` ahead 9 with history that 408s on push.  
2. Showcase MP4s on LFS (~290 MB) pathologically slow to us-east-1 S3.  
3. `crates/target/` was committed mid-history (stripped locally; not on origin).  
4. Formal LSP empty → amber honest, not false green.  
5. SAIF human queue still holds GCP / multi-host git reconcile.

---

## 2. Functionality map (Rust apps focus)

| Surface | Path | State | Category | Next |
|---------|------|-------|----------|------|
| **reson8-tui** | `crates/tui` | v0.2.1 local: surface layout, codes lab, QDI drain, net_proxy, share_publish, ctqw example | B runtime | Entangle slot → CI smoke |
| **barcode-tui** | `crates/barcode-tui` | H0 + clouds + tests; braille/H1 TODO | B | Slot + strand TODOs |
| **apps/triweave** | `apps/triweave` | deploy scripts + TUI dashboard | B | Slot B |
| **apps/mc-bridge** | `apps/mc-bridge` | RCON bridge | B | Slot C |
| **apps/nexus-pulse** | `apps/nexus-pulse-bot` | voice + invariant | B | Slot C |
| **net stack** | `ops/net` | local untracked on origin; TUI wired | B | Commit via entangle/net-proxy |
| **formal SRAC** | lean + cutile + docs/packets | local 9-commit work | A/B mix | Slot formal-srac |
| **coherence-mcp** | site + tools | live hub; LFS media | B | Media → R2 |

---

## 3. SAIF along the path

| Stage | SAIF question | Doc / control | Honesty |
|-------|---------------|---------------|---------|
| Setup | INTENT clear? | CLAUDE.md · AGENTS.md · conservation C-only | C label not gate |
| Onboard | Can operator run TUI? | RELEASE-0.2.1.md · logos-net | B |
| Act | With-Intent only? | Operating doctrine | process |
| Invariant | Claims labeled A/B/C/D? | claim_lint · DriftGuard · EPISTEMICS-GATE | A tooling local |
| Human | What only Matthew does? | `ops/SAIF-OUTSTANDING-HUMAN-ACTIONS.md` | queue stale (git push item still open) |
| Ship | Who has authority? | capability ≠ authority · human merge | A |

**Gap:** SAIF queue A2 (git push surfaces) is the active bottleneck — this assessment + entangle protocol **is** the remediation path.

---

## 4. Skills → autofire pipeline (on demand)

| Trigger | Skill / workflow | Fires | Output |
|---------|------------------|-------|--------|
| `/heisenberg-grok` | uncertainty map | session boot | YAML map + next_sharpen |
| `/subagent-driven-development` | plan → implement → dual review | when plan exists | commits per task |
| `workflow_dispatch` Entangle | `.github/workflows/entangle.yml` | scaffold / ingest | PR slots |
| push/PR main | Verify · CI Policy · CodeQL · CODEX | automatic | status checks |
| `/axiom-alerting` | monitors/notifiers | **blocked** no `.axiom.toml` token | scaffold only |
| net optimize | `LogOS.NetProxy.ps1` | operator / TUI `M` | clearnet vs privacy |
| claim change | `tools/claim_lint.py` | local/CI later | gate report |

**Proposed autofire chain (time-ordered)**

1. **On demand:** Entangle scaffold Priority A.  
2. **On slice emit:** ingest workflow or direct PR commit.  
3. **On PR:** Verify + rust-tui-smoke must pass.  
4. **On merge main:** badges + residual report.  
5. **Optional:** Axiom monitor on workflow failure webhooks once token present.  
6. **Never autofire:** force-push, secret deploy, GCP billable — human ⚑.

---

## 5. Loose threads (sourced · scoped · ordered)

| # | Thread | Source | Scope | Order |
|---|--------|--------|-------|-------|
| T1 | Land code without 408 | this session · receive-pack logs | entangle + slim packs | **now** |
| T2 | reson8-tui 0.2.1 remote | local commits a83b69b… | entangle/reson8-tui | now |
| T3 | barcode-tui TODOs | README TODO(gemini/grok/…) | crate only | after T2 |
| T4 | Strip/prevent target/ | filter-branch done local | .gitignore already | done local |
| T5 | LFS 5 MP4s | git lfs status | transfer-lane R2 | after clearnet |
| T6 | PR #48 smoke | github | close after real land | cleanup |
| T7 | Formal residual CI red | PR48 check | formal_residual_report | fix in formal slot |
| T8 | Rust core CI red | PR48 | workspace on origin | fix when core lands |
| T9 | ops/net not on origin | untracked | entangle/net-proxy | B |
| T10 | Axiom auth | MCP failed · no toml | ops/axiom scaffolds | when token |
| T11 | SSH pubkey denied | git@github.com | optional | later |
| T12 | SAIF GCP A1 | SAIF queue | deploy waist | human ⚑ |

---

## 6. Proxy + qBittorrent + Cloudflare (prime optimization)

| Asset | Role |
|-------|------|
| `logos-net start-gaming` | clear proxies, maximize TCP for git/LFS/R2 |
| DNSCrypt `:53553` | encrypted DNS without routing bulk through Tor |
| qBittorrent | private seed of `ops/entangle/out/*.zip` or LFS object export across machines |
| Cloudflare R2 / Stream | permanent home for showcase MP4s; git keeps pointers/URLs |
| Privacy lane | Tor/i2pd/Privoxy — **never** for bulk git/LFS |

See `ops/entangle/transfer-lane.md`.

---

## 7. Axiom alerting (skill invoked)

**Status:** no project `.axiom.toml` / `~/.axiom.toml` token in this environment; Axiom MCP handshake previously required auth.

**Scaffold only** under `ops/axiom/monitors/` — do not create live monitors until `scripts/setup` + token.

Suggested first monitors (after auth):

1. MatchEvent: GitHub Actions failure webhook ingest (if logs → Axiom).  
2. Threshold: elevated error rate on coherence MCP edge dataset.  
3. Anomaly: LFS/transfer job duration if instrumented.

---

## 8. Time-as-constraint ordering (do next)

1. Land **entangle** PR (this branch) — tiny, structure only.  
2. `workflow_dispatch` scaffold A slots.  
3. Emit slices from full local main worktree for reson8-tui / barcode / formal.  
4. Ingest or manual PR fill → merge.  
5. Gaming-lane LFS or R2 for videos.  
6. Close PR #48; refresh SAIF A2 as done.  
7. Axiom when token exists.

Music conserved · Keystone holds · capability ≠ authority
