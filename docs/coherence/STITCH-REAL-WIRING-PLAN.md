# Stitch · Real-Wiring Plan

Companion to `COHERENCE-TOOLATED-SUBDOMAIN-MAP.md` and `STITCH-COHERENCE-MCP-BINDING.md`. This doc names the four concrete wiring tracks that turn the cockpit and its sibling surfaces from mocked prototypes into a live suit: **real hooks**, **real terminals**, **real bytecode**, **real telemetry**. Every track names its edge entry point, its client-side seam, its server-side owner, and the Universal Invariant enforcement that wraps it. Nothing here asks for rediscovery — each attractor already exists in the map; this doc is the infrastructure we leave between them.

## Invariant contract (applies to all four tracks)

Every request and every response passes through two gates. The outer gate is the Cloudflare Worker running the subdomain Middleware Stack from `COHERENCE-TOOLATED-SUBDOMAIN-MAP.md` — invariant check, sigil gate, OTLP injection, response hardening. The inner gate is the coherence-mcp server itself, which re-runs `check_coherence` on the response payload before it leaves the origin. If either gate returns Σ ≠ 15 the call is converted to a 503 with `{"error":"invariant_violation","alpha":<a>,"omega":<o>}` and a Magenta Alert hits the SpiralSafe channel. The cockpit's α+ω=15 gauge binds directly to the `X-Invariant-Sigma` response header, so a violation is visible in the UI one paint after it happens.

The signature `~ Hope&&Sauced ✦ The Keystone Holds ✦` is treated as an α-rail invariant. Any long-lived document emitted by any track (checkpoint, ratification, postmortem) closes with it; its presence is verified by the self-test in `skills/internal-handoff/SKILL.md`.

## Track 1 · Real hooks (mocked TOOLS → live MCP)

The cockpit currently carries a JavaScript `TOOLS` object whose five entries (`store_context`, `retrieve_context`, `map_isomorphism`, `check_coherence`, `bridge_translate`) are shape-identical to the live server contract but serve canned responses. Real-hook wiring replaces that object with a small client module that issues real calls against `mcp.coherence.toolated.online`. The module exposes the same five functions with the same signatures, so no UI code changes — only the fetch backend swaps.

Transport splits by call shape. `check_coherence` and `store_context` run over HTTPS POST to `mcp.coherence.toolated.online/v1/<tool>` with a JSON body matching the current mocked `req`. `retrieve_context`, `map_isomorphism`, and `bridge_translate` open a single persistent WebSocket on page load to `mcp.coherence.toolated.online/ws`, multiplex requests by `context_id`, and receive streamed fragments for large payloads. The WebSocket also carries server-pushed `invariant_drift` frames which re-render the α+ω gauge in real time rather than on a polling tick.

The self-referential fixed-point call runs on page load before any UI interaction is enabled. The cockpit issues `store_context({subject: "coherence_mcp_cockpit", atom: <SHA-256 of the rendered DOM at load time>, strand: "claude", fib: 8})`. The returned `atom_sig` — the CBOR canonical commitment produced by the Rust `atom-sig` crate — is rendered directly into the top-nav BRAID slot, replacing the placeholder `#C0-FF-EE`. This closes the loop: the cockpit's first stored context is its own specification, and the hash displayed is the hash of the DOM that displayed it. Any subsequent reload whose `atom_sig` differs indicates DOM drift; the gauge needle should pick the drift up one frame later.

Error handling is stratified. Invariant violations (Σ ≠ 15) are terminal — the tool card displays the full diagnostic and the cockpit enters α-deadlock (UI dims, Magenta Alert pulse). Transport errors (timeout, 5xx non-invariant) are retried with exponential backoff up to three attempts, then surfaced as amber badges on the tool button. Semantic errors (4xx from coherence-mcp validation) display the field-level violation without blocking other tools.

## Track 2 · Real terminals (xterm.js + WebSocket + Durable Object PTY)

The cockpit's bottom terminal cluster is currently a static HTML grid with fake VCS state strings. Real-terminal wiring replaces each cell with an xterm.js instance backed by a per-session Durable Object hosting a constrained PTY. The four cells bind to four distinct roles: (cell 1) `vcs` — live `git status`, `git log -1 --oneline`, `git submodule status` streamed every 15 s; (cell 2) `sink` — tailing the live sink feed from `sink.coherence.toolated.online/stream`; (cell 3) `env` — `uname -a`, `free -m`, `nvidia-smi --query-gpu=utilization.gpu --format=csv` on the RTX 5090; (cell 4) `weaver` — the Weaver sigil animation overlaid on the current `atom_sig` truncated to 12 hex digits.

The PTY is not a general shell. Every command that reaches the Durable Object is gated by the Invariant Gate middleware (pending task #6): the middleware holds an allowlist table keyed by cell role, and anything not on the allowlist is rejected with `ERR_NOT_ALLOWED` before hitting exec. The allowlist is small, read-only, versioned in Git under `coherence-mcp/config/terminal-allowlist.toml`, and re-validated by `check_coherence` on every PTY spawn. This means the terminal is useful for live debugging but cannot escalate into a shell from the outside — the α-rail holds at the gate.

Transport is WebSocket to `term.coherence.toolated.online/cell/<role>`. The Durable Object name matches the session id (derived from the sigil JWT `sub` claim) so all four cells in one browser share one Durable Object, amortizing PTY spawn cost. The Durable Object persists exactly the last 1,024 lines of scrollback per cell; anything older rolls off. No PII is ever written to persistent storage — SpiralSafe's regex denylist runs on the output stream inline.

## Track 3 · Real bytecode (Rust atom-sig + fib-braid-core)

Two Rust crates produce the bytecode that the cockpit displays. `atom-sig` (pending task #5, Phase 1 stub landed) takes a context payload, canonicalises it via CBOR (RFC 8949 §4.2.1 deterministic encoding — shortest-length, sorted keys, deterministic floats, no indefinite-length items), hashes the canonical bytes with BLAKE3, and returns a 256-bit commitment plus a small header identifying the hash function, the context subject, and the strand of origin. The resulting byte string is what the cockpit renders into the BRAID slot and what `store_context` writes to the 9P|Styx Bookshelf.

`fib-braid-core` (pending task #12, Phase 1 stub landed) produces the trace_n_braid bytes. For a braid word like σ₁σ₂σ₁⁻¹σ₃ it computes the reduced Burau matrix at `t = e^(2πi/5)` (the Fibonacci root of unity that anchors the five-strand closure), evaluates the Jones polynomial V(q) at `q = e^(2πi/5)` via the Kauffman bracket, evaluates HOMFLY P(a, z) on the closure, and emits a canonical 4-tuple `(burau_hash, jones_bytes, homfly_bytes, closure_bytes)` that the cockpit's four-cell metadata grid binds to directly. The double-cover (4π rotation) property is verified by evaluating V(q) and V(q⁻¹) and asserting their product equals the unit scalar.

Delivery from server to browser is hex over SSE from `mcp.coherence.toolated.online/bytes/<context_id>`. Hex rather than binary so that the bytes render directly in the monospace DESIGN.md typography without a decoder step — the bytecode is the UI, not a proxy for it. The cockpit's active-tool card can optionally `track_v2` a context, which subscribes to the SSE stream for that `context_id` and updates the braid SVG crossings and metadata grid as the backend mutates the braid word. This is the hook atom_track_v2 (pending task #13) needs.

Verifiability is important: any third party can re-execute the Rust canonicalisation against the same input and reproduce the exact byte string, because CBOR deterministic encoding is total and BLAKE3 is deterministic. There is no hidden state between the raw context and the displayed hash.

## Track 4 · Real telemetry (OTLP-HTTP → Collector → Prometheus → SSE)

Every browser surface emits OpenTelemetry spans and metrics over OTLP-HTTP to `otel.coherence.toolated.online/v1/traces` and `/v1/metrics`. The browser SDK is initialised once at the edge (injected by the Worker into the HTML response) so each surface inherits the same tag set without per-page config. Required tags on every span: `reson8.strand` (claude | grok | gemini | manus), `reson8.subdomain` (the specific surface), `reson8.context_id` (the UUID bound to the current sigil session), `reson8.alpha`, `reson8.omega`, `reson8.coherence` (the WAVE score at span-open). W3C `traceparent` propagates through every fetch and WebSocket frame so a single user action can be stitched end-to-end across cockpit → Worker → coherence-mcp origin → Durable Object.

The Collector is a single Cloudflare-hosted OTEL Collector with the OTLP receiver, the batch processor, and three exporters: Prometheus (for the `reson8-prometheus` scrape endpoint), Loki (for logs), and a small custom SSE exporter that pushes every span that crosses a coherence or invariant threshold to `otel.coherence.toolated.online/live`. The cockpit's bottom terminal cluster subscribes to `/live` and renders the last ten threshold-crossing spans as a live tail in the right-most cell, effectively turning the cockpit into its own dashboard.

Metrics that matter are enumerated in Prometheus rules. `reson8_wave_coherence` (per-strand, per-surface, gauge, target ≥ 0.98); `reson8_invariant_sigma` (per-surface, gauge, target = 15, alert on drift); `reson8_tool_latency_seconds` (histogram per tool, 50/95/99p); `reson8_store_context_rate` (counter, rate/minute); `reson8_magenta_alerts_total` (counter, any nonzero increment = page). Grafana dashboards import directly from `coherence-mcp/ops/grafana/` which is committed alongside the server code.

Budget is explicit: no surface emits more than 100 spans per minute per user session. If the rate is exceeded the browser SDK switches to head-based sampling at 1:10. This keeps the telemetry subsystem from inflating α while still giving enough coverage to catch drift.

## Milestone ordering

The four tracks land in a strict order because each depends on the last. Track 1 (real hooks) lands first — it requires the coherence-mcp server running, the `mcp.*` subdomain cut over, and the sigil JWT flow live. Track 3 (real bytecode) lands second, because it completes the self-referential `store_context` call that Track 1 initiates — the cockpit can use Track 1 with a stub `atom_sig` for a short window, but the fixed point is not closed until `atom-sig` and `fib-braid-core` are wired. Track 4 (real telemetry) lands third, so the first two weeks of production traffic produce a baseline for the alerting thresholds. Track 2 (real terminals) lands last — it is the highest-surface-area track and benefits from having Tracks 1, 3, 4 already emitting clean signal.

Each track has a standalone acceptance test checked into `coherence-mcp/tests/wiring/`. The tests run against a live staging stack (staging subdomain `staging.coherence.toolated.online` with its own wildcard) and report green/amber/red to the main Grafana dashboard. A track is considered "landed" only when its acceptance test has run green for 72 hours of staging traffic including at least one simulated invariant drift event and at least one planned Magenta Alert.

## What this is not

This plan names the wires. It does not duplicate the work of `COHERENCE-TOOLATED-SUBDOMAIN-MAP.md` (subdomain topology), `STITCH-COHERENCE-MCP-BINDING.md` (why the cockpit is the right first binding surface), `CHECKPOINT-20260419.md` (session state at Phase 1 close), or `skills/internal-handoff/SKILL.md` (how to hand off to a smaller model for mechanical stretches of this work). Read those four first; this doc is the fifth.

This plan also does not prescribe the internals of the Rust crates beyond the interface each exposes to the cockpit. The gold-pan-to-Rust conversation is where the internals get distilled into a golden substrate. That conversation starts after this plan's Track 1 lands green.

## Return-trip framing

Every one of the four tracks is a return trip. The cockpit existed before the real hooks did, but the hooks were always implied by the five-tool contract. The terminal cluster existed as HTML before the Durable Object did, but the PTY was always implied by the DESIGN.md call for "real technical data in monospace". The `atom_sig` placeholder existed before the Rust crate did, but the commitment was always implied by the self-referential fixed point. The OTLP tags existed as a CLAUDE.md constraint before the Collector did, but the telemetry was always implied by WAVE ≥ 0.98.

What this doc leaves in the attractor is the wiring diagram: the named edge entry points, the named client seams, the named server owners, the named invariant enforcement. Future work iterates on the wires, not on whether the wires exist.

---

~ Hope&&Sauced ✦ The Keystone Holds ✦
