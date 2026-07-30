================================================================================
NOTEBOOKLM CRITICAL SOURCE (snapshot)
================================================================================
File role       : AdHealth product surface
Snapshot date   : 2026-07-25
Source path     : F:\Users\Matthew Ruhnau\LogOS\adhealth-meaningseed\README.md
NotebookLM note : Upload with NOTEBOOKLM-SINGLE-IMPORT-LOADBEARING-20260725.txt
                  as hub. This is a point-in-time copy; prefer tree if conflict.
================================================================================
# AdHealth — Creative Fatigue Intelligence

**It's Today Media · $5,000 Build Challenge · Marketing Development Engineer**

**Live demo:** [https://coherence.toolated.online/portal/](https://coherence.toolated.online/portal/)

---

## What does this tool do?

AdHealth models your creative portfolio as a **graph** (creatives = nodes, shared ad sets / platforms / performance similarity = edges), then runs **continuous-time quantum walk dynamics** to extract signals media buyers can act on today:

- **Return probability** — early fatigue indicator (walker returns to the same creative state)
- **Participation ratio** — how spread-out attention is across the portfolio
- **Portfolio concentration** — over-weighted spend risk
- **Per-creative fatigue flags** with plain-language recommendations

Outputs are gated through **coherence-mcp** invariants (α + ω = 15, WAVE ≥ 0.85) and packaged as **Rezk/Yoneda witnesses** for auditability.

---

## Why did you build THIS one?

Media buyers at scale (Meta, Google, Taboola, TikTok) see CPA degradation **after** budget is already wasted. Standard dashboards report lagging conversions; they do not surface **structural fatigue** across the creative mix.

AdHealth answers: *which creatives are structurally exhausted, and where is the portfolio over-concentrated?* — using math that runs classically on CPU today and scales to GPU via the LogOS `cutile` bridge tomorrow.

I built this because it maps directly to money: pause the right creative, diversify the right cluster, test the right variant — **before** CPA spikes.

---

## What would you build next (full-time)?

1. **Platform MCP connectors** — live pulls from Meta/Google/Taboola reporting APIs into the graph builder
2. **Automated weekly brief** — Slack/email digest with fatigue flags + recommended actions
3. **Landing-page variant lab** — tie creative nodes to LP performance subgraphs
4. **Agentic workflow** — coherence-mcp gated agent that drafts refresh briefs and pushes pause/scale rules to ad platforms

---

## Quick start

```bash
cd adhealth-meaningseed
pip install -e ".[dev]"
adhealth analyze --demo
adhealth analyze --demo --json --export portal/data/demo_report.json
```

### Deploy portal (Cloudflare Pages)

```bash
npx wrangler pages deploy portal --project-name adhealth-portal
# Route: coherence.toolated.online/portal → Pages custom path
```

---

## Architecture

```
Campaign CSV / sample data
        ↓
  build_campaign_graph()
        ↓
  QuantumWalkDynamics (CTQW, scipy expm)
        ↓
  analyze_portfolio() → fatigue + concentration
        ↓
  coherence gates + Rezk/Yoneda witness
        ↓
  CLI JSON / portal dashboard
```

| Module | Role |
|--------|------|
| `core/campaign.py` | Graph from creatives |
| `core/dynamics.py` | CTQW evolution |
| `core/signals.py` | Media-buying signals |
| `core/witness.py` | Rezk/Yoneda completion |
| `core/coherence.py` | MCP gate map |
| `portal/` | Static dashboard |
| `docs/COHERENCE_MCP.md` | MCP integration |

---

## CSV format

| Column | Type |
|--------|------|
| creative_id | string |
| ad_set | string |
| platform | meta/google/taboola/tiktok |
| impressions | int |
| clicks | int |
| spend | float |
| conversions | int |
| days_live | int |

Sample: `data/sample_campaign.csv`

---

## Tests

```bash
pip install pytest
pytest tests/ -q
```

---

## Judging alignment

| Criterion | Evidence |
|-----------|----------|
| Real problem | Creative fatigue + concentration for media buyers |
| Works | Live portal + CLI + JSON export |
| Code quality | Modular `core/`, typed dataclasses, tests |
| README | What / Why / Next above |

---

**License:** MIT · **Author:** Matthew Ruhnau / LogOS MeaningSeed