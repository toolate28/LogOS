# triweave — Tri-Weavon Unified Deployer

Single binary that deploys, manages, and monitors any combination of Claude/Grok/Gemini strands with SPHINX-gated ATOM-AUTH, Styx/9P-Virtio transport, and Coherence City Minecraft integration.

## Install

```bash
cargo install --path .
# or from workspace root:
cargo build -p triweave --release
# binary at: target/release/triweave.exe
```

## Commands

### `triweave init` — SAIF Onboarding

Setup with INTENT. Interactive 5-step flow:

1. **Strand selection** — choose Claude, Grok, Gemini, or all
2. **Vault passphrase** — creates SPHINX-gated encrypted key store
3. **API keys** — collected per strand, encrypted with Jones polynomial derived AES-256-GCM
4. **Health checks** — verifies API reachability and Styx bridge
5. **Config write** — saves `~/.triweave/config.toml`

ATOM trail entries at each gate: KENL (intention) -> AWI (awareness) -> ATOM (operational).

```
$ triweave init

╔══════════════════════════════════════════════════╗
║       TRIWEAVE — Setup with INTENT (SAIF)        ║
║           Conservation: alpha + omega = 15       ║
╚══════════════════════════════════════════════════╝

Step 1/5 — Select strands to enable:
  Available: claude (Windows), grok (NixOS/GLF), gemini (WSL2/Kali)
  Enable (comma-separated, or 'all'): all

Step 2/5 — Create vault passphrase:
  Passphrase: ********
  Confirm: ********

Step 3/5 — API keys (stored in SPHINX-gated vault):
  Anthropic API key (sk-ant-...): ****
  anthropic — stored
  xAI API key (xai-...): ****
  xai — stored
  Google AI API key: ****
  google — stored

Step 4/5 — Health checks:
  claude — reachable
  grok — reachable
  gemini — reachable
  styx bridge — not running (started by `triweave up`)

Step 5/5 — Writing configuration:
  ~/.triweave/config.toml written

SAIF complete — 3 keys encrypted, strands: claude, grok, gemini
Next: `triweave up` to start strands
```

### `triweave up [strand|all]` — Start Strands

Boots selected strands and transport layer:

```bash
triweave up           # all enabled strands
triweave up claude    # just Claude
triweave up grok      # just Grok
triweave up gemini    # just Gemini
triweave up all       # explicit all
```

Sequence:
1. Load config + decrypt keys from SPHINX vault
2. Start Styx bridge (ws://127.0.0.1:8088) if not running
3. Boot strands in parallel
4. Verify API connectivity
5. Conservation check: alpha + omega = 15

### `triweave down [strand|all]` — Stop Strands

Graceful shutdown with conservation law preservation.

### `triweave status` — TUI Dashboard

Live ratatui terminal dashboard:

```
+-- TRIWEAVE v0.1.0 -----------------------------------+
| STRANDS        | WAVE -- alpha + omega = 15           |
|  * Claude 0.95 | XXXXXXXXXXXXXXXXXXXX 0.937           |
|  * Grok   0.93 |                                      |
|  * Gemini 0.92 | a 8  w 7  S 15                       |
|                |                                      |
| FORGE          | ATOM TRAIL                           |
|  Transport: styx| KENL -> AWI -> ATOM -> SAIF         |
|  Styx: ws://...|                                      |
|  NEAR: testnet | Waiting for bridge events...         |
+------------------------------------------------------+
```

Keybindings: `q` quit, `Esc` quit.

### `triweave doctor` — Diagnostics

Storyboard error frames with auto-fix:

```
Checking SPHINX vault... (3 keys)
Checking Styx bridge (ws://127.0.0.1:8088)... X

+= ERROR FRAME 1/1 ========================================+
| X [styx] Styx bridge unreachable                         |
|                                                          |
| WHY: WebSocket server not running at ws://127.0.0.1:8088 |
| WAVE: 0.93 -> 0.41 (below 0.85 threshold)                |
+==========================================================+
  AUTO-FIX: Restarting Styx bridge...
  Result: FIXED
```

Checks: vault integrity, Styx bridge, strand APIs, Minecraft RCON.

Auto-fixes: RemountVFS, RestartStyx, RotateKey, PingMinecraft, RetryApi.

### `triweave deploy <target>` — Minecraft Deployment

```bash
triweave deploy amazon-room   # Zone 5: Vectorize search holograms
triweave deploy city           # All 5 zones
triweave deploy npc-suite      # Claude/Grok/Gemini NPCs
```

**Amazon Room** (Zone 5, x:100 z:0):
- 32x12x32 obsidian/glass structure
- Central search podium (lectern)
- 3x3 hologram pedestal grid (armor stands with floating text)
- Amazon Merchant NPC
- Vectorize embedding search via Cloudflare Worker

### `triweave vault` — Key Management

```bash
triweave vault list     # Show key names (never values)
triweave vault rotate anthropic   # Re-encrypt with new SPHINX braid
triweave vault audit    # Show fingerprints, verify integrity
```

## ATOM-AUTH Vault

Keys are encrypted at rest in `~/.triweave/vault.sphinx` using:

1. **Per-key SPHINX braid** — deterministic braid word from `payload_to_braid_word(key_name:passphrase)`
2. **Jones polynomial fingerprint** — evaluated at t = e^{2*pi*i/5} (Fibonacci anyon point)
3. **Argon2id KDF** — derives AES-256-GCM key from passphrase + fingerprint salt
4. **AES-256-GCM** — authenticated encryption with random 12-byte nonce

Every decrypt operation:
- Verifies SPHINX fingerprint matches stored braid (gate check)
- Derives decryption key via argon2id
- Decrypts AES-256-GCM
- Logs ATOM trail entry (key access event)
- Returns plaintext (held in memory only, never on disk)

## Configuration

`~/.triweave/config.toml`:

```toml
[theme]
name = "coherence-dark"
accent = "cyan"
border = "rounded"

[strands]
enabled = ["claude", "grok", "gemini"]
transport = "styx"

[near]
network = "testnet"
account = "reson8-test.testnet"

[minecraft]
rcon_host = "127.0.0.1"
rcon_port = 25575
world = "WORLD_CONVERGENCE"

[styx]
ws_url = "ws://127.0.0.1:8088"
```

## Environment Variables

API keys can be provided via env vars (SAIF will auto-detect and store in vault):

| Variable               | Strand | Purpose                       |
|------------------------|--------|-------------------------------|
| `ANTHROPIC_API_KEY`    | Claude | Anthropic API access          |
| `XAI_API_KEY`          | Grok   | xAI API access                |
| `GOOGLE_AI_KEY`        | Gemini | Google AI API access          |
| `NEAR_ACCOUNT_ID`      | All    | NEAR testnet account          |
| `CLOUDFLARE_API_TOKEN` | All    | Cloudflare Workers/D1/KV/R2   |
| `RCON_PASSWORD`        | All    | Minecraft RCON authentication |
| `FORGE_WS_URL`         | All    | Override Styx bridge URL      |

## Dependencies

From workspace:
- `reson8-forge-core` — protocol types, bridge, superskill engine
- `sphinx` — Jones polynomial SPHINX gating
- `styx` — 9P2000.L WebSocket bridge
- `coherence-activator` — meta-skill routing

Added:
- `clap 4` — CLI framework
- `aes-gcm 0.10` — authenticated encryption
- `argon2 0.5` — key derivation
- `toml 0.8` — config serialization
- `ratatui 0.29` + `crossterm 0.28` — TUI
- `reqwest 0.12` — HTTP health checks

## Tests

```bash
cargo test -p triweave

# 4 tests:
# vault::tests::vault_roundtrip — encrypt/decrypt cycle
# vault::tests::wrong_passphrase_fails — SPHINX rejection
# vault::tests::sphinx_fingerprint_integrity — Jones polynomial verification
# amazon::hologram::tests::test_render_holograms — hologram RCON commands
```

## Roadmap

- [x] Phase 1: Skeleton + SAIF + vault + strands + doctor + TUI + amazon room
- [ ] Phase 2: TUI dashboard with live WebSocket bridge events
- [ ] Phase 3: NEAR conservation-verifier real contract (replace stub)
- [ ] Phase 4: Amazon Room RCON execution + Vectorize search endpoint
- [ ] Phase 5: Global themes + full storyboard TUI doctor
- [ ] Phase 6: forge_tda_ml (Betti telemetry, Ridge score, Negative Space Addendum)

## Conservation Law

Every operation in triweave respects: **alpha + omega = 15**

This means: the sum of intention (alpha) and outcome (omega) is conserved. No information is created or destroyed — only transformed across substrates. The WAVE score measures conservation at each transition. The ATOM trail records the full transformation history.

```
alpha = 8 (Claude, structure)
omega = 5 (Grok, pulse) + 3 (Gemini, scale) = 7 (runtime) + 1 (forge) = 7
alpha + omega = 8 + 7 = 15
```
