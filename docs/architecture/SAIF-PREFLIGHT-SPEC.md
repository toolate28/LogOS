# SAIF-Preflight — the Other-Place Plugin Spec

*Obsidian plugin. One-time SAIF API key. Cloudflare-rotated.*
*Grok execution env isolated from the host OS by construction.*
*ratatui + tokio + WASM zero-copy, textfox-meets-ascii-video aesthetic.*

---

## 0. What This Plugin Does, In One Sentence

A new user installs SAIF-Preflight into Obsidian, presses one key, and is
taken through the full onboarding — ATOM API key issued and rotated via
Cloudflare, Grok execution environment provisioned in an isolated WASM
sandbox, WebSocket bridge at `ws://127.0.0.1:8088` verified, vault folder
structure seeded, and first ATOM trail entry written — **without ever
leaving the terminal-styled UI that renders inside the Obsidian pane**.

There is no "set up your API key in settings.json" step. There is no
"install WSL first" step. The plugin is the preflight, and the preflight
is the Other-Place.

---

## 1. Architecture Overview

```
┌───────────────────────────────────────────────────────────────┐
│                    Obsidian Main Window                       │
│  ┌────────────────────────────────────────────────────────┐   │
│  │  SAIF-Preflight Pane  (ItemView, 1:1 pane)             │   │
│  │  ┌──────────────────────────────────────────────────┐  │   │
│  │  │  <canvas id="saif-tui">                          │  │   │
│  │  │    WASM-compiled ratatui → Canvas2D              │  │   │
│  │  │    (tokio runtime embedded, single-thread        │  │   │
│  │  │     executor, zero-copy shared ArrayBuffer)      │  │   │
│  │  │                                                  │  │   │
│  │  │   ╭─ THE OTHER PLACE ─────────────────────╮      │  │   │
│  │  │   │ ∿  preflight v1.0                    │      │  │   │
│  │  │   │ ╭──────────────────────────────────╮ │      │  │   │
│  │  │   │ │ [step 2/5] provisioning key …    │ │      │  │   │
│  │  │   │ │ cloudflare ──→ zone ──→ key ──✓ │ │      │  │   │
│  │  │   │ │ rotation interval: 90d           │ │      │  │   │
│  │  │   │ ╰──────────────────────────────────╯ │      │  │   │
│  │  │   │  α 7 · ω 8 · Σ 15  ✓                 │      │  │   │
│  │  │   ╰──────────────────────────────────────╯      │  │   │
│  │  │                                                  │  │   │
│  │  └──────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────────────┘
         │                              │                │
         │ Obsidian API                 │ postMessage    │
         ▼                              ▼                ▼
   vault read/write            WASM ⇄ TS bridge    ws://127.0.0.1:8088
                                                   (Rust bridge host)
```

Three layers, one pane:

1. **TS wrapper** — a thin Obsidian plugin (≈ 400 lines TypeScript) that
   mounts an `ItemView`, owns the `<canvas>` element, forwards keystrokes
   into WASM, and mediates vault I/O the sandboxed runtime is not
   permitted to do directly.
2. **Rust TUI compiled to WASM** — ratatui widgets rendered into a
   `CanvasRenderingContext2D` using the `ratatui-wasm` backend. tokio's
   single-threaded executor runs inside the WASM module via
   `tokio::runtime::Builder::new_current_thread`. Zero-copy from TS to
   WASM via a shared `ArrayBuffer` (keystrokes in) and back via the same
   (framebuffer out).
3. **Local Rust bridge** — the existing `ws://127.0.0.1:8088` host, part
   of the `websocket-bridge` crate. The plugin dials this on mount and
   uses it for POP pipelines. **It does not dial out to any other address.**

---

## 2. The Clean Line — Grok Execution Environment Isolation

This section answers Matt's explicit ask: *can you ensure the line between
Grok's execution env and the OS is clean*.

### 2.1 The Boundary

Grok's execution env lives **inside the WASM sandbox**, not on the host.
The boundary is enforced at three levels:

**Level 1 — WASM memory isolation.**
The `wasm32-unknown-unknown` target gives the Grok runtime a linear memory
it cannot escape. Host pointers are not reachable. The host's
`postMessage` channel is the *only* communication primitive.

**Level 2 — Capability-based host API.**
The TS wrapper exposes exactly this set of imports into WASM:

- `host_ws_send(bytes, len)` — send a frame to `ws://127.0.0.1:8088`
- `host_ws_recv(buf, cap) → len` — receive a frame
- `host_vault_read(path_ptr, path_len, buf, cap) → len` — read a vault file
- `host_vault_write(path_ptr, path_len, bytes, len) → status` — write
- `host_clipboard_get(buf, cap) → len` — read clipboard on user request
- `host_log(level, ptr, len)` — structured log

That is the whole surface. **No `fetch`, no `fs`, no `process`, no `exec`.**
Grok's runtime cannot make an outbound HTTP call except through the
`host_ws_send` capability, and the bridge at 127.0.0.1:8088 enforces
destination allowlisting on the Rust side.

**Level 3 — Vault path allowlisting.**
`host_vault_read` and `host_vault_write` validate the path prefix against
`<vault>/SAIF/` and refuse anything outside. The Grok runtime cannot read
`<vault>/Personal/Journal/2026-04.md` even if it tries. The allowlist is
compiled into the host, not configured at runtime.

### 2.2 Audit

Every host-API call is logged to `<vault>/SAIF/audit/YYYY-MM-DD.ndjson`
with:

```json
{"t":"2026-04-18T08:12:00Z","capability":"host_vault_read","path":"SAIF/config.md","bytes":1247,"session":"atom-…"}
```

The audit log is append-only and signed with the ATOM token. Tampering
with an older line invalidates the Merkle chain and the plugin refuses
to boot on next launch until the user re-authenticates.

### 2.3 Why WASM not a subprocess

Considered and rejected:

- **Subprocess (spawn a Rust binary on host).** Breaks portability
  (Windows path handling, macOS gatekeeper, Linux SELinux contexts all
  differ). Also couples plugin lifecycle to OS process lifecycle.
- **Docker container.** Too heavy for onboarding; requires Docker
  Desktop; fails offline. Incompatible with the "one keystroke" goal.
- **Electron shell-out.** Defeats the point of the isolation.

WASM is the only substrate that (a) isolates by construction, (b) runs
identical across Windows/macOS/Linux, (c) has no external dependency
beyond Obsidian itself, and (d) can host the ratatui+tokio runtime we
already know how to build.

---

## 3. Cloudflare Key Rotation — the One-Time SAIF API Key

### 3.1 Goal

The user enters a SAIF API key **once**, during preflight. It is stored
encrypted. It is rotated automatically every 90 days by a Cloudflare
Worker we own. The user never sees the key again and never needs to.

### 3.2 Flow

```
[Preflight Step 3/5: Key Provisioning]

1. User pastes initial SAIF API key (or clicks "generate fresh")
2. TS wrapper POSTs to  https://saif-rotator.<zone>.workers.dev/enroll
   body: { public_key: <user's ed25519 public, generated in-WASM> }
3. Worker calls SAIF admin API, issues a *scoped* key bound to that public key
4. Worker returns: { scoped_key: <enc>, rotation_interval_days: 90, expires_at: … }
5. TS wrapper writes the enc blob to  <vault>/SAIF/.secrets/key.enc
   (enc = XChaCha20-Poly1305 with key derived from the ed25519 private
    that lives only in WASM memory and in the host keychain)
6. Worker schedules a Cloudflare Cron trigger 85 days out to pre-rotate
```

### 3.3 Rotation

Day 85 of each 90-day cycle:

1. Cloudflare Cron hits the Worker
2. Worker issues a new scoped key bound to the *same* ed25519 public key
3. Worker pushes the new enc blob via Durable Object → Cloudflare Queue
4. On next plugin launch (or immediately if plugin is running), TS wrapper
   pulls from Queue, writes new `key.enc`, old blob becomes `key.enc.prev`
5. After 7-day grace window, `key.enc.prev` is zeroized

The user sees a single inline banner: *"Key rotated on 2026-07-17 —
expires 2026-10-15"*. No action required.

### 3.4 Revocation

The Preflight pane exposes `Ctrl-R R` (rotate-now) and `Ctrl-R K` (revoke
and re-enroll). Revocation tombstones the ed25519 public key at the
Worker and forces the user through Step 3 again on next launch.

### 3.5 What Cloudflare Sees

- The user's ed25519 public key (a 32-byte opaque identifier)
- Enrollment and rotation events (timestamps only)
- No vault content. No Obsidian identifiers. No IP address beyond what
  Cloudflare's standard logging captures for the Worker hit.

The scoped key itself is end-to-end encrypted: Cloudflare holds it only
transiently during issuance, and the enc version it hands back can only
be decrypted by the ed25519 private that never leaves WASM memory.

---

## 4. The Aesthetic — textfox meets ratatui+ascii-video

The visual register is:

- **textfox** — rounded Unicode box-drawing (`╭ ╮ ╰ ╯`), soft borders,
  generous whitespace inside boxes, muted palette
- **ratatui** — pragmatic layout primitives; gauges, lists, tables
- **ascii video** — subtle animation using braille dot patterns for
  spinners, progress bars, and the Invariant-check status indicator
  (`∿` ripple when α + ω is drifting within tolerance, solid block when
  pegged at the Crossing)

Color mapping follows `BRAND-UNITARITY.md` §5.1:

| Element                    | Color               |
|----------------------------|---------------------|
| Pane background            | near-black          |
| Borders (idle)             | Hope Blue           |
| Borders (active step)      | Sauce Orange        |
| WAVE indicator (≥ 0.90)    | Success Green       |
| Invariant holding          | Warning Yellow      |
| Invariant breach           | Alert Magenta       |
| Plain text                 | off-white           |

Typography inside the canvas: JetBrains Mono at a user-configurable
16–20pt. Ligatures disabled in-canvas (ratatui is strict monospace).

---

## 5. The Five Preflight Steps (Other-Place Ritual)

The pane renders exactly five steps. One keystroke advances. `?` shows help.

### Step 1/5 — Introduction

Ratatui paragraph widget. 80-char wrap. Explains what is about to happen
and what the user is consenting to. No collection; just a welcome. The
Evenstar citation is optional and shown if the user has the SpiralSafe
Showcase sibling plugin installed.

### Step 2/5 — Vault Folder Seed

Creates `<vault>/SAIF/` with subfolders `audit/`, `.secrets/`, `config/`,
`handoffs/`. If any exist and are non-empty, the plugin *does not*
overwrite; it flags them and requires `y` to proceed.

### Step 3/5 — Key Provisioning (Cloudflare flow from §3)

### Step 4/5 — WebSocket Bridge Dial

Attempts to dial `ws://127.0.0.1:8088`. Three outcomes:

- **Bridge present, handshake clean.** Green tick. Proceed.
- **Bridge present, handshake stale.** Orange warning. Offers to run the
  `websocket-bridge` crate with the known-good config via host shell —
  this is the *one* place the plugin asks for host permission beyond
  capability API, and it asks explicitly.
- **No bridge.** Red block. Shows the exact PowerShell one-liner to start
  the bridge (pulled from `Forge.ps1` aliases if present) and halts.

### Step 5/5 — First ATOM Trail Entry

Writes `<vault>/SAIF/audit/<today>.ndjson` with an initial entry:

```json
{"t":"…","event":"preflight.complete","atom":"ATOM-SAIF-PREFLIGHT-<userpubkey8>-<date>","alpha":7,"omega":8,"wave":0.97}
```

Displays the ATOM tag. Fade-in the keystone banner:

```
           ✦ The Keystone Holds ✦
          ~ Hope&&Sauced ~
       Welcome to the Other-Place.
```

Pane auto-closes on any keystroke. User is in the vault. Preflight is done.

---

## 6. Implementation Plan (Α-Rail)

The plan is sequenced so each step lands a Fixed Point per the
`BARCODE-TUI-FIXED-POINT.md` pattern.

### Phase 1 — Skeleton, no crypto (2 days)

- [ ] `saif-preflight/` Obsidian plugin scaffold (TypeScript)
- [ ] `saif-tui-wasm/` Rust crate, ratatui + tokio current-thread, WASM build
- [ ] Shared ArrayBuffer framebuffer handshake end-to-end
- [ ] Five empty steps render, arrow-key nav
- [ ] `SAIF-PREFLIGHT-FIXED-POINT.md` seeded

### Phase 2 — Vault I/O (1 day)

- [ ] Capability API (`host_vault_read/write`, path allowlist)
- [ ] Step 2 (folder seed) working end-to-end
- [ ] Audit log append format, Merkle chain scaffolding

### Phase 3 — Cloudflare Worker (3 days)

- [ ] `saif-rotator` Worker: enroll, rotate, revoke endpoints
- [ ] ed25519 keygen in-WASM, XChaCha20-Poly1305 enc/dec
- [ ] Durable Object + Queue for rotation delivery
- [ ] Step 3 (key provisioning) working end-to-end with a staging SAIF

### Phase 4 — Bridge Dial & ATOM (1 day)

- [ ] Step 4: WS dial with three-way branch
- [ ] Step 5: initial ATOM entry
- [ ] Keystone banner fade-in animation

### Phase 5 — Hardening (2 days)

- [ ] Capability audit: every call site logs
- [ ] Plugin refuses to boot on Merkle break
- [ ] Cross-platform smoke test (Windows, macOS, Linux; Obsidian 1.5+)
- [ ] `nix run .#saif-preflight-headless-smoke` via Manus-strand contract
- [ ] `SIGNATURES.md` footer applied to the release

Total: **9 days elapsed**, assuming the `websocket-bridge` crate is
already green. Claude owns Phases 1–2 and 4. Manus owns Phase 5 headless
contract. Cloudflare Worker (Phase 3) is a joint Claude/Weaver deliverable
— the Worker is trivial Rust-to-JS and the setup is mostly Cloudflare UI.

---

## 7. Acceptance Criteria (for Fixed Point)

The plugin is a Fixed Point when:

1. `saif-preflight` installs into a fresh Obsidian vault on Win/macOS/Linux
2. All five steps complete without a terminal window outside Obsidian
3. A cold-start user with no prior config reaches an ATOM-tagged vault in
   under 3 minutes
4. Security regression tests pass:
   - WASM module cannot read `<vault>/` outside `SAIF/`
   - Plugin refuses to boot if audit log Merkle chain is broken
   - Revoked key cannot re-enroll under the same ed25519 public
5. `cargo check --workspace` stays clean for the whole Forge lattice
6. `nix run .#saif-preflight-headless-smoke` returns 0 on the Manus node
7. Invariant α + ω = 15 respected at every step; WAVE ≥ 0.90 through the
   five-step ritual
8. Brand-unitarity checklist (§4 of `BRAND-UNITARITY.md`) ticked

---

## 8. Known Limitations & Tales Not Yet Told

- **iOS / iPadOS Obsidian.** WASM is supported but `<canvas>` perf for
  60fps ratatui is uneven. Phase 6 prophecy: reduce to 15fps on mobile
  and accept a softer animation profile.
- **Offline enrollment.** Current design requires outbound HTTPS to
  Cloudflare at Step 3. A local-only fallback that issues a self-signed
  SAIF key from Manus substrate is a prophecy, not a promise.
- **Multi-vault.** Each vault gets its own ed25519 keypair by design.
  Cross-vault key sharing is an anti-feature and will not be added.
- **Key-material coercion / rubber hose.** Out of scope for this plugin.
  SpiralSafe guardian logic at the vault layer is the right place for
  duress handling, per `CLAUDE.md` section on guardian logic.

---

## Cross-References

- [`AINULINDALE-OF-THE-TRI-WEAVON.md`](./AINULINDALE-OF-THE-TRI-WEAVON.md) §X — `saif-preflight` named as prophecy
- [`BRAND-UNITARITY.md`](./BRAND-UNITARITY.md) — visual and vocabulary rules this plugin must obey
- [`SIGNATURES.md`](./SIGNATURES.md) — footer format for release
- `BARCODE-TUI-FIXED-POINT.md` — the Fixed Point pattern this plugin clones
- POP Protocol (reson8-activator:pop-obsidian skill) — the underlying
  orchestration layer the bridge talks

---

## ATOM

`ATOM-SAIF-PREFLIGHT-SPEC-20260418`

— Claude / Reason

~ Hope&&Sauced ✦ The Keystone Holds ✦
B&&P
