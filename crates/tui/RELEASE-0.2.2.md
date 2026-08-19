# reson8-tui v0.2.2 — HITL gate · first_need

**Stamp:** constraint `20260818.hitl-gate` (ATOM order, not wall clock)  
**Bin:** `reson8-forge` · package `reson8-tui` **0.2.2**  
**ATOMs:** `ATOM-GROK-TUI-HITL-GATE-20260818`  
depends on `ATOM-SAIF-HUMAN-QUEUE-20260723` · `ATOM-CHECKPOINT-CHOKE-20260814` · `ATOM-LATTICE-ACTIVATE-20260815` · `ATOM-GROK-TUI-QR-META-20260806`

**Tagline:** Where the last thing you've done becomes the first thing you need

## What shipped

| Surface | Detail |
|---------|--------|
| **HITL board** | `FocusPanel::Actions` · layout `8` / `hitl` · also on ops + agent |
| **Approval subroutine** | request → escalate → approve / defer / deny / next ⚑ |
| **Queue** | `ops/human-actions.json` first · MD parse fallback |
| **Receipts** | `ops/marks/hitl-receipts.jsonl` — constraint + seq (not wall time) |
| **first_need** | toast + idle strip show the next ⚑ A/B item |
| **Git observe** | `git_lab` wired — `[G]` refresh · `[V]` fetch · no commit/push |
| **Activator kit** | `reson8-activator::ops_caps` have/don't-have on the HITL board |

Approve is a **receipt**. It does not run GCP, sudo, wrangler, or git push.

## Not a sidecar

A separate sidecar process would be another node on the choke spine. The queue is a file the cockpit already knows how to H-probe (same pattern as lattice `[A]`). GitNexus eval/MCP/HTTP stay the impact plane (`F:\Users\Matthew Ruhnau\GitNexus`); they are not required to latch a ⚑ decision.

## Operator path

```text
cargo run -p reson8-tui
# or: RESON8_LAYOUT=hitl cargo run -p reson8-tui
```

| Key | Action |
|-----|--------|
| `8` / `o` | HITL layout / focus |
| `O` | Reload JSON/MD queue |
| `u` | Open gate on first_need |
| `j` / `k` | Select item (or popup option) |
| Enter | Confirm decision |
| Esc | Dismiss without deciding |

## Honest labels (NOVIKOV)

| Claim | Category |
|-------|----------|
| Queue file present / parse | **A** observation |
| In-session approve/escalate latch | **B** |
| ATOM constraint clock | **C** ordering convention |
| Approve ⇒ host action done | **false** — capability ≠ authority |

## Tests

```bash
cargo test -p reson8-tui --lib
```

Expect: markdown parse, first_need A1, approve advances to A2, receipt seq 1 then 2, apply_intent HITL gate.

Hope&&Sauced · Keystone holds · last becomes first
