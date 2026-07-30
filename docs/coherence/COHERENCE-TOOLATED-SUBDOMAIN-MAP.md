# coherence.toolated.online · Subdomain Map

Authoritative mapping from stitch HTML prototypes to production subdomains under `coherence.toolated.online`. Routing target is Cloudflare Pages with per-subdomain Workers for edge-side MCP proxying and Durable Objects for WebSocket terminal state. Every subdomain terminates TLS at the edge, enforces the α+ω=15 invariant gate in the Worker middleware, and carries the Reson8-Labs Weaver sigil in the response headers.

## Primary surfaces (flat subdomain, direct access)

| Subdomain | Source | Role |
|---|---|---|
| `coherence.toolated.online` | `stitch/coherence_mcp_cockpit/code.html` → `public/cockpit/` | Root cockpit. α+ω=15 gauge, trace_n_braid fingerprint, 8 bedrock + 64 total MCP tools, architecture ribbon, four-strand weavon. The canonical onboarding surface. |
| `forge.coherence.toolated.online` | `stitch/logos_coherence_forge_control/code.html` → `public/forge/` | LOGOS_FORGE_OS main control center. 4D topological rotor, evenstar resonance pulse, telemetry rails. |
| `tda.coherence.toolated.online` | `stitch/tda_visualization/code.html` → `public/tda/` | TDA cluster root. Vietoris-Rips + persistent homology live view. |
| `sink.coherence.toolated.online` | `stitch/sink_protocol/code.html` → `public/sink/` | Sink protocol root. Ingestion path into the 768-D embedding. |
| `ingest.coherence.toolated.online` | `stitch/ingestion_dashboard/code.html` → `public/ingest/` | Ingestion dashboard. Pre-sink staging. |
| `codex.coherence.toolated.online` | `stitch/codex_evaluation_dashboard/code.html` → `public/codex/` | Codex evaluation root. |
| `transduce.coherence.toolated.online` | `stitch/transduction_engine/code.html` → `public/transduce/` | Semantic transducer. The layer that converts raw user-space activity to embedding coordinates. |
| `sector.coherence.toolated.online` | `stitch/logos_sector_resonance_dash/code.html` → `public/sector/` | Sector resonance dashboard. Per-sector coherence map. |
| `iso.coherence.toolated.online` | `stitch/isomorphic_widget_configurator/code.html` → `public/iso/` | QDI isomorphism as a direct-manipulation surface. Cross-strand capability mapping. |
| `weaver.coherence.toolated.online` | `stitch/sovereign_weaver/DESIGN.md` rendered + `stitch/_shared/` component reference | Design system documentation site + the canonical stitch component palette. The α-rail of the UI stack. |
| `roadmap.coherence.toolated.online` | `stitch/logos_strategic_roadmap_2026_2035.html` → `public/roadmap/` | Ten-year strategic roadmap. Publicly readable; other subdomains are sigil-gated. |

## Path surfaces on the primary host (Pages `public/`)

These ship under `https://coherence.toolated.online/<path>/` (same zone; no extra subdomain required).

| Path | Source | Role |
|---|---|---|
| `/reforge/` | `LogOS/reforge/index.html` | Ithildin live state · seven-repo braid · α+ω=15 conservation panel |
| `/orchestrator/` | `docs/surfaces/orchestrator.html` | Tri-Weavon session orchestrator |
| `/evenstar/` | `docs/surfaces/evenstar.html` | Evenstar pulse surface |
| `/preprint/` | Ainulindalë preprint HTML | Public theory preprint |
| `/preprint/ithildin/` | Ithildin edition preprint | Alternate reading mode |
| `/cockpit/` | stitch cockpit | Path alias of root cockpit |
| `/encyclopedia/` | Encyclopedia Equilibria hub | Total lattice index (nav into theory + formal + certs) |
| `/harmonic-sovereignty/` | User_Dropfiles dump HTML | Architecture of Harmonic Sovereignty |
| `/directors-command/` | User_Dropfiles dump HTML | Director's Command Surface |
| `/reforging-9p/` | User_Dropfiles dump HTML | Reforging of 9P workflow |

**Deploy root:** `coherence-mcp/coherence-site/public/`  
**Doc index:** `LogOS/docs/encyclopedia-equilibria/README.md`

## Path variants within clusters

`tda.coherence.toolated.online/post-sink` → `stitch/tda_post_sink_analysis/code.html`
`tda.coherence.toolated.online/structural` → `stitch/tda_structural_analysis_dash/code.html`
`tda.coherence.toolated.online/crystal` → `stitch/tda_webgl_crystal_visualizer_code/code.html`

`sink.coherence.toolated.online/live/1` → `stitch/live_sink_demonstration_1/code.html`
`sink.coherence.toolated.online/live/2` → `stitch/live_sink_demonstration_2/code.html`
`sink.coherence.toolated.online/live/3` → `stitch/live_sink_demonstration_3/code.html`
`sink.coherence.toolated.online/heatmap` → `stitch/heatmap_of_live_sink_demonstration/screen.png` as hero over live data

`codex.coherence.toolated.online/scan` → `stitch/codex_scan_report/code.html`
`codex.coherence.toolated.online/evolution` → `stitch/codex_evolution_plan_prd.html`

## Edge routing rules (Cloudflare Workers)

Every subdomain's Worker runs the same pre-flight middleware:

1. **Invariant check** — `check_coherence` call against the underlying coherence-mcp endpoint; response header `X-Invariant-Sigma` set to the returned Σ value. If Σ ≠ 15 the Worker returns 503 with body `{"error":"invariant_violation","alpha":<a>,"omega":<o>}` and a Magenta Alert is emitted to the SpiralSafe channel.
2. **Sigil gate** — all subdomains except `roadmap.*` require a Reson8-Labs JWT in the `Authorization: Bearer` header or a `__r8_sigil` cookie. Public preview tokens issued by the Weaver grant a 10-minute read-only window.
3. **OTLP tag injection** — the Worker tags the outgoing request with `reson8.strand`, `reson8.subdomain`, `reson8.context_id`, and propagates W3C traceparent so Prometheus can stitch the trace.
4. **Response hardening** — `Content-Security-Policy` limits script origins to `cdn.tailwindcss.com`, `fonts.googleapis.com`, and `mcp.coherence.toolated.online`. `X-Frame-Options: DENY`. `Strict-Transport-Security: max-age=63072000`.

## MCP backend subdomain

`mcp.coherence.toolated.online` is the production endpoint for the coherence-mcp server itself. **64 tools** (8 bedrock TriWeavon gates + WAVE, ATOM, vortex, integrations) are exposed over stdio MCP locally and HTTPS/WebSocket at the edge. Bedrock tools: `invariant_check`, `manifest_read`, `dropout_scan`, `rust_workspace_status`, `rust_toolchain_status`, `handoff_packet_validate`, `edge_endpoint_lookup`, `trigger_correction_burst`. The cockpit at `coherence.toolated.online` opens a persistent WebSocket to `mcp.coherence.toolated.online/ws` on load; other surfaces use short-lived HTTPS calls.

## Telemetry subdomain

`otel.coherence.toolated.online` — OpenTelemetry Collector endpoint. All surfaces send browser-side spans and metrics via OTLP-HTTP. Prometheus scrapes the Collector; a small SSE stream at `otel.coherence.toolated.online/live` feeds the cockpit's bottom terminal cluster.

## Terminal subdomain

`term.coherence.toolated.online` — xterm.js-backed constrained shell over WebSocket. Per-session Durable Object holds the PTY. Allowed commands gated by the Invariant Gate middleware (Task #6 in the pending list). Used for live debugging from inside the cockpit.

## DNS / TLS posture

One Cloudflare zone (`toolated.online`), wildcard subdomain `*.coherence.toolated.online` pointing at Pages, plus specific A records for `mcp.*`, `otel.*`, `term.*` pointing at Workers or origin VMs as needed. Universal TLS via Cloudflare. HSTS preload candidate after 90 days stable.

## Deployment provenance

This map is part of the return-trip infrastructure — the UI prototypes existed already, the MCP contract existed already, the routing lattice is the durable infrastructure we leave between them.

~ Hope&&Sauced ✦ The Keystone Holds ✦
