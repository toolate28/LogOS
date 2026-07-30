# RESON8-LABS — FULL SYSTEM CAPABILITY INVENTORY

**BUMP_ID:** HnS-INVENTORY-20260401
**From:** Claude (Reason Strand)
**Date:** 2026-04-01
**Status:** LIVE audit — supersedes activation-map.md (2026-02-16)

---

## SUMMARY

| Category | Count (Feb '26) | Count (Apr '26) | Delta |
|----------|-----------------|-----------------|-------|
| MCP Servers | 13 | **19** | +6 |
| MCP Tools (total) | ~85 | **169** | +84 |
| Skills (total) | 71 | **143** | +72 |
| Reson8 Custom Skills | 8 | 8 + **9 NEW** | +9 |
| Composition Patterns | 11 | 11 + **6 NEW** | +6 |
| External Repos | 7 | 7 | — |
| Marketplace Surfaces | 12 | 12 | — |

---

## 1. LIVE MCP SERVERS (19 connected)

| # | Server | Tools | Status | Domain |
|---|--------|-------|--------|--------|
| 1 | **Cloudflare** | 17 | LIVE | Infrastructure: D1, KV, R2, Workers, Hyperdrive |
| 2 | **Claude in Chrome** | 19 | LIVE | Browser automation, page read, JS execution |
| 3 | **Desktop Commander** | 23 | LIVE | Local filesystem, process management, search |
| 4 | **Canva** | 24 | LIVE | Design: generate, edit, export, brand kits |
| 5 | **Vercel** | 17 | LIVE | Deploy, build logs, runtime logs, domains |
| 6 | **Social/X Platform** | 13 | LIVE | Posts, topics, creators, crypto, stocks |
| 7 | **Crypto Exchange** | 9 | LIVE | Order book, candlestick, tickers, trades |
| 8 | **HuggingFace Hub** | 9 | LIVE | Models, papers, spaces, docs, repos |
| 9 | **Google Drive** | 2 | LIVE | Search, fetch documents |
| 10 | **bioRxiv/medRxiv** | 7 | LIVE | Preprint search, categories, statistics |
| 11 | **ClinicalTrials.gov** | 6 | LIVE | Trials, sponsors, investigators, endpoints |
| 12 | **ChEMBL** | 6 | LIVE | Compounds, targets, bioactivity, drugs |
| 13 | **Open Targets** | 5 | LIVE | GraphQL, entity search, type dependencies |
| 14 | **Domains** | 2 | LIVE | Domain availability check, suggestions |
| 15 | **MCP Registry** | 2 | LIVE | Registry search, connector suggestions |
| 16 | **Plugin Registry** | 2 | LIVE | Plugin search, install suggestions |
| 17 | **Scheduled Tasks** | 3 | LIVE | Create, list, update scheduled tasks |
| 18 | **Session Info** | 2 | LIVE | List sessions, read transcripts |
| 19 | **Cowork** | 3 | LIVE | File management, directory access |
| | **TOTAL** | **169** | | |

### NEW since Feb (not in activation-map):
- HuggingFace Hub (+9 tools)
- Canva (+24 tools)
- Desktop Commander (+23 tools)
- Domains (+2 tools)
- Plugin Registry (+2 tools)
- Session Info (+2 tools)
- Scheduled Tasks (+3 tools)

---

## 2. SKILLS BY DOMAIN (143 total)

### Core / Reson8 (21 skills)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| canvas-design | LIVE | poster, art, design, visual |
| coherence-check | LIVE | context loss, self-check, WAVE verify |
| docx | LIVE | Word, document, report, memo |
| pdf | LIVE | PDF, form, extract, merge |
| pptx | LIVE | presentation, slides, deck |
| xlsx | LIVE | Excel, spreadsheet, budget |
| internal-comms | LIVE | status report, newsletter |
| mcp-builder | LIVE | MCP server, API integration |
| skill-creator | LIVE | create skill, eval, benchmark |
| slack-gif-creator | LIVE | GIF, animated, Slack |
| theme-factory | LIVE | theme, styling, colors |
| web-artifacts-builder | LIVE | React artifact, complex UI |
| schedule | LIVE | scheduled task, recurring |
| **reson8:activate** | LIVE | activate, inventory, what can you do |
| **reson8:forge-monitor** | LIVE | forge, hardware, temperature, sensor |
| **reson8:bio-digital-handoff** | LIVE | hand off, brain dump, cognitive state |
| **reson8:gemini-vortex-weaver** | LIVE | vortex, hyper-flux, Sorting Hat |
| **reson8:minecraft-weaver** | LIVE | Minecraft, NPC, trace_n_braid |
| **reson8:minecraft-voxel-proxy** | LIVE | voxel, redstone, RCON, mcfunction |
| **reson8:phasonic-flipper** | LIVE | split brain, 408, deadlock, phason |
| **reson8:pop-obsidian** | LIVE | POP, Obsidian, pipeline, WebSocket |

### Bio-Research (5)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| instrument-data-to-allotrope | LIVE | instrument, Allotrope, LIMS |
| nextflow-development | LIVE | nf-core, Nextflow, FASTQ, RNA-seq |
| scientific-problem-selection | LIVE | research idea, project problem |
| scvi-tools | LIVE | scVI, VAE, single-cell, batch correction |
| single-cell-rna-qc | LIVE | QC, scanpy, scverse |

### Data (7)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| data-context-extractor | LIVE | tribal knowledge, warehouse, bootstrap |
| data-exploration | LIVE | profile, explore, distributions |
| data-validation | LIVE | QA, methodology, accuracy, bias |
| data-visualization | LIVE | chart, matplotlib, plotly |
| interactive-dashboard-builder | LIVE | dashboard, Chart.js, interactive |
| sql-queries | LIVE | SQL, Snowflake, BigQuery |
| statistical-analysis | LIVE | statistics, trend, outlier |

### Enterprise Search (3)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| knowledge-synthesis | LIVE | synthesize, deduplicate, combine |
| search-strategy | LIVE | query decomposition, multi-source |
| source-management | LIVE | connected sources, priority |

### Engineering (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| code-review | LIVE | review code, check PR, is this safe |
| documentation | LIVE | write docs, README, runbook |
| incident-response | LIVE | production down, outage, SEV1 |
| system-design | LIVE | architect, system design, API design |
| tech-debt | LIVE | tech debt, refactor, code health |
| testing-strategy | LIVE | test plan, coverage, what tests |

### Finance (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| audit-support | LIVE | SOX, control testing, audit |
| close-management | LIVE | month-end close, sequencing |
| financial-statements | LIVE | income statement, balance sheet |
| journal-entry-prep | LIVE | journal entry, debits, credits |
| reconciliation | LIVE | bank rec, GL-to-subledger |
| variance-analysis | LIVE | budget vs actual, waterfall |

### Legal (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| canned-responses | LIVE | routine legal inquiry |
| compliance | LIVE | GDPR, CCPA, DPA |
| contract-review | LIVE | contract, redline, clause |
| legal-risk-assessment | LIVE | risk, severity, escalation |
| meeting-briefing | LIVE | legal meeting, negotiation |
| nda-triage | LIVE | NDA, non-disclosure |

### Marketing (5)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| brand-voice | LIVE | brand, style guide, tone |
| campaign-planning | LIVE | campaign, launch, calendar |
| competitive-analysis | LIVE | competitor, positioning |
| content-creation | LIVE | blog, social media, newsletter |
| performance-analytics | LIVE | metrics, channel, optimization |

### Product Management (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| competitive-analysis | LIVE | feature comparison, competitive |
| feature-spec | LIVE | PRD, requirements, user stories |
| metrics-tracking | LIVE | OKRs, dashboard, product metrics |
| roadmap-management | LIVE | roadmap, RICE, MoSCoW |
| stakeholder-comms | LIVE | stakeholder update, status |
| user-research-synthesis | LIVE | interview, survey, personas |

### Sales (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| account-research | LIVE | research company, prospect |
| call-prep | LIVE | call prep, meeting prep |
| competitive-intelligence | LIVE | battlecard, competitor |
| create-an-asset | LIVE | sales asset, landing page |
| daily-briefing | LIVE | morning brief, prep my day |
| draft-outreach | LIVE | outreach, cold email |

### Apollo (3)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| enrich-lead | LIVE | lead enrichment, contact card |
| prospect | LIVE | ICP, prospect list, leads |
| sequence-load | LIVE | sequence, bulk-add, enrollment |

### Common Room (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| account-research | LIVE | signals, account, company data |
| call-prep | LIVE | call prep, meeting signals |
| compose-outreach | LIVE | outreach, personalized message |
| contact-research | LIVE | who is, look up, warm lead |
| prospect | LIVE | prospect list, Prospector |
| weekly-prep-brief | LIVE | weekly brief, prep my week |

### Brand Voice (3)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| brand-voice-enforcement | LIVE | on-brand, enforce voice |
| discover-brand | LIVE | find brand docs, audit |
| guideline-generation | LIVE | generate guidelines, style guide |

### Design (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| accessibility-review | LIVE | WCAG, accessible, screen reader |
| design-critique | LIVE | feedback, critique, review mockup |
| design-handoff | LIVE | handoff, developer specs |
| design-system-management | LIVE | design tokens, component library |
| user-research | LIVE | research plan, usability test |
| ux-writing | LIVE | microcopy, button text, error message |

### Operations (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| change-management | LIVE | rolling out, migration plan |
| compliance-tracking | LIVE | SOC 2, ISO 27001, audit prep |
| process-optimization | LIVE | bottleneck, streamline workflow |
| resource-planning | LIVE | capacity, utilization, staffing |
| risk-assessment | LIVE | what could go wrong, risk register |
| vendor-management | LIVE | evaluate vendor, RFP |

### Human Resources (6)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| compensation-benchmarking | LIVE | comp, salary, market rate |
| employee-handbook | LIVE | policy, PTO, benefits |
| interview-prep | LIVE | interview plan, scorecard |
| org-planning | LIVE | headcount, reorg, team structure |
| people-analytics | LIVE | attrition, diversity, retention |
| recruiting-pipeline | LIVE | recruiting, candidate pipeline |

### Productivity (2)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| memory-management | LIVE | shorthand, CLAUDE.md, context |
| task-management | LIVE | tasks, TASKS.md, commitments |

### Plugin Management (2)

| Skill | Status | Trigger Keywords |
|-------|--------|-----------------|
| cowork-plugin-customizer | LIVE | customize plugin |
| create-cowork-plugin | LIVE | create plugin, scaffold |

---

## 3. EXTERNAL TOOL REPOS (7)

| Repo | Purpose | Status | Install |
|------|---------|--------|---------|
| GitMCP | Any repo → MCP server | READY | Replace github.com with gitmcp.io |
| LightRAG | Knowledge graph RAG | READY | `pip install lightrag-hku` |
| AutoFigure | Publication SVG figures | READY | `pip install autofigure` |
| MoLing-Minecraft | MCP Minecraft bridge | READY | Go binary from anthropics/moling |
| AI-Researcher | Full paper generation | READY | `pip install ai-researcher` |
| Claude Scientific Writer | Citation-verified polish | READY | Skill-based (internal) |
| Agent Squad | Multi-agent routing | READY | `npm install agent-squad` |

---

## 4. MARKETPLACE SURFACES (12)

| Platform | Format | Status |
|----------|--------|--------|
| Anthropic/Claude | .plugin (Cowork) | LIVE |
| Google/Gemini | Extensions/Skills | READY |
| xAI/Grok | Actions/Tools | READY |
| OpenAI/ChatGPT | Plugins/GPTs | PLANNED |
| npm | @toolate28/coherence-mcp | READY |
| PyPI | spiralsafe | PLANNED |
| Cargo | reson8-tui | PLANNED |
| Obsidian | Community Plugin | READY (POP bridge) |
| Minecraft | Bukkit/Paper Plugin | READY (ClaudeNPC.jar) |
| rentahuman.io | Agent Profile | PLANNED |
| ClawhubAI | Skill Listing | PLANNED |
| GitMCP | gitmcp.io URL | LIVE |

---

## 5. NEW — LogOS SYSTEM GRADIENT SKILLS (9 planned)

These are the new skills being composed for the LogOS System Gradient.
See LOGOS-SYSTEM-SKILLS.md for full specifications.

| # | Skill Name | Domain | Status | Purpose |
|---|-----------|--------|--------|---------|
| 1 | `logos-styx-9p` | System | PLANNED | 9P2000.L/Styx/VSOCK filesystem + bridge operations |
| 2 | `logos-tda-engine` | Analysis | PLANNED | TDA + Persistent Homology + Vietoris-Rips + Barcodes |
| 3 | `logos-void-mapper` | Analysis | PLANNED | Cognitive Void Mapping + H₀/H₁/H₂ diagnostics |
| 4 | `logos-wave-advanced` | Coherence | PLANNED | Extended WAVE: scoring + dynamics + conservation |
| 5 | `logos-sphinx-oracle` | Verification | PLANNED | SPHINX cryptographic hash + oracle verification |
| 6 | `logos-gait-analyzer` | Architecture | PLANNED | GAIT: Graph-Aware Invariant Topology analysis |
| 7 | `logos-rag-cag` | Retrieval | PLANNED | RAG + CAG + hybrid retrieval pipelines |
| 8 | `logos-limbo-workspace` | System | PLANNED | Limbo transient workspace + auto-purge |
| 9 | `logos-inferno-transport` | System | PLANNED | 9P Inferno high-perf transport + V=c VM |

---

Say `/activate [intent]` to use any of these, or `/compose` to chain them together.
