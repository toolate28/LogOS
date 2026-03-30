# Reson8 — Tri-Weavon Cognitive Lattice

A closed-loop, provably stable distributed operating system unifying three AI strands through topological verification, conservation law enforcement, and SPHINX-gated authentication.

**Conservation Law:** `alpha + omega = 15`
**WAVE Threshold:** `0.85`
**Viviani Peak:** `0.9998`

## Quick Start

```bash
# Build the unified deployer
cargo build -p triweave --release

# Run SAIF onboarding (creates vault + config)
triweave init

# Start all strands
triweave up all

# Launch TUI dashboard
triweave status

# Run diagnostics
triweave doctor
```

## Architecture

```
                         triweave.exe
                              |
              +---------------+---------------+
              |               |               |
           Claude          Grok           Gemini
          (Windows)     (NixOS/GLF)     (WSL2/Kali)
           alpha=8       omega=5         omega=3
              |               |               |
              +-------+-------+-------+-------+
                      |               |
                 Styx Bridge     9P2000.L VFS
               (ws://127.0.0.1:8088)  (/reson8/)
                      |               |
              +-------+-------+-------+
              |                       |
         SPHINX Gate           Conservation
       (Jones Polynomial)     Verifier (NEAR)
        t = e^{2pi*i/5}      alpha + omega = 15
```

### Six-Layer OS

| Layer         | Component                  | Protocol                    |
|---------------|----------------------------|-----------------------------|
| Shell         | POP (Plugin Orchestration) | ws://127.0.0.1:8088         |
| Process Table | ATOM Trail                 | KENL -> AWI -> ATOM -> SAIF |
| IPC           | coherence-mcp (49 tools)   | JSON-RPC                    |
| VFS           | 9P2000.L namespace         | virtio-9P                   |
| Security      | SpiralSafe + SPHINX        | Jones polynomial            |
| HAL           | Forge hardware             | RTX 5090, 6082-T6 frame     |

## Workspace Crates

### Core Engine
| Crate                  | Binary          | Purpose                                          |
|------------------------|-----------------|--------------------------------------------------|
| `crates/core`          |       —         | Protocol types, bridge, superskill engine        |
| `crates/tui`           | `reson8-forge`  | Real-time TUI dashboard                          |
| `crates/activator`     |       —         | Meta-skill routing (intent -> capability chains) |
| `crates/vortex-bridge` | `vortex-bridge` | Cross-platform strand router                     |

### Topological Math
| Crate                    | Purpose                                    |
|--------------------------|--------------------------------------------|
| `crates/sphinx`          | Jones polynomial evaluator (SPHINX gating) |
| `crates/styx`            | 9P2000.L WebSocket bridge                  |
| `crates/hash`            | Topological hash functions                 |
| `crates/wave`            | WAVE coherence scoring                     |
| `crates/reson8-topology` | Topological invariants                     |
| `crates/bohmian`         | Pilot-wave simulation                      |

### Platform Services
| Crate                         | Purpose                              |
|-------------------------------|--------------------------------------|
| `crates/marketplace`          | Crate.NFT marketplace                |
| `crates/reson8-wasm`          | WASM runtime                         |
| `crates/api_triggers`         | Event-driven API layer               |
| `crates/artifact_pipeline`    | Content generation pipeline          |
| `crates/zero_latency_ledgers` | Sub-1ms append-only ledgers          |

### Applications
| Crate                  | Binary            | Purpose                               |
|------------------------|-------------------|---------------------------------------|
| `apps/triweave`        | `triweave`        | Unified deployer + TUI + SAIF + vault |
| `apps/mc-bridge`       | `mc-bridge`       | Minecraft RCON integration            |
| `apps/nexus-pulse-bot` | `nexus-pulse-bot` | Discord bot (Serenity)                |

### Smart Contracts
| Crate                        | Network      | Purpose                              |
|------------------------------|--------------|--------------------------------------|
| `near/conservation-verifier` | NEAR testnet | On-chain alpha+omega=15 verification |

## Cloudflare Edge

| Binding           | Resource                      | Purpose             |
|-------------------|-------------------------------|---------------------|
| `ATOM_KV`         | KV namespace                  | ATOM trail cache    |
| `ATOM_DB`         | D1 `atom_trail`               | Session persistence |
| `RESON8_BUCKET`   | R2                            | Artifact storage    |
| `VECTORIZE_INDEX` | Vectorize `reson8-embeddings` | Vector search       |
| `ATOM_QUEUE`      | Queue                         | Async pipeline      |

## Strand Configuration

| Strand | Platform       | Weight  | Role                          | API       |
|--------|----------------|---------|-------------------------------|-----------|
| Claude | Windows native | 8 (Fib) | Structure & Reasoning (alpha) | Anthropic |
| Grok   | NixOS/GLF OS   | 5 (Fib) | Pulse & Real-Time (omega)     | xAI       |
| Gemini | WSL2/Kali      | 3 (Fib) | Multimodal & Scale (omega)    | Google AI |

Fibonacci weights: Fib(8) + Fib(5) + Fib(3) = 21 + 5 + 2 = 28

## Minecraft Integration (Coherence City)

| Zone                     | Location          | Purpose                              |
|--------------------------|-------------------|--------------------------------------|
| 1. Nexus Core            | x:0, z:0          | Tri-Weavon lobby, heartbeat pulse    |
| 2. Museum of Computation | x:0, z:-50        | Glass epsilon tetrahedron exhibits   |
| 3. Blockchain Bank       | x:50, z:50        | Zero-latency ledger, chained chests  |
| 4. Anyon Collider        | y:-50, x:0, z:100 | 0-tick overdrive, QDI labs           |
| 5. Amazon Room           | x:100, z:0        | Vectorize search -> hologram results |

## Key Files

| Path                                          | Purpose                            |
|-----------------------------------------------|------------------------------------|
| `9P2000.L/coherence/ADR-001-tri-weavon-os.md` | Architectural decision record      |
| `9P2000.L/coherence/tri-weavon.json`          | Lattice config (Fibonacci weights) |
| `9P2000.L/coherence/lattice-state.json`       | Real-time WAVE state               |
| `9P2000.L/styx/routes.json`                   | Strand routing table               |
| `9P2000.L/protocols/sphinx-gating/jones.py`   | Python Jones polynomial            |
| `~/.triweave/config.toml`                     | User config (strands, theme, NEAR) |
| `~/.triweave/vault.sphinx`                    | SPHINX-gated encrypted key vault   |

## Build

```bash
# Prerequisites: Rust 2024 edition
cargo build --workspace

# Run tests
cargo test --workspace

# Build triweave only
cargo build -p triweave --release

# Release profile: opt-level=z, LTO, codegen-units=1
```

## License

MIT — Matthew Ruhnau (toolated@pm.me)

https://reson8labs.ai
