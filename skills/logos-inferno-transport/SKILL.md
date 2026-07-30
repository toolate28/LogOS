---
name: logos-inferno-transport
description: >
  Inferno/Dis VM transport layer and concurrent channel operations for the
  LogOS lattice. Implements the Limbo programming language runtime (Dis VM),
  9P-multiplexed concurrent channels, and the Inferno operating system
  semantics that underpin cross-strand communication. Use this skill when
  managing concurrent channel operations, spawning Dis VM processes,
  multiplexing 9P connections, implementing CSP-style (Communicating
  Sequential Processes) concurrency patterns, or bridging Inferno semantics
  into the modern Rust/Tokio runtime.
  Triggers on: "inferno", "dis vm", "limbo language", "channel", "concurrent",
  "csp", "alt", "spawn", "multiplex", "9p transport", "inferno channel",
  "communicating sequential", "dis bytecode", "channel operation".
version: 1.0.0
---

# logos-inferno-transport — Inferno/Dis VM Transport Layer

## Purpose

Inferno is the concurrency substrate of the LogOS lattice. Where
9P|Styx provides the filesystem abstraction and VSOCK provides the
transport, Inferno/Dis provides the computational model: CSP-style
concurrent channels multiplexed over 9P connections.

The Inferno operating system (Bell Labs, 1995) pioneered the idea
that all resources — local and remote — are accessed through the same
filesystem protocol (Styx/9P), and that concurrency is managed through
typed channels with explicit send/receive semantics. LogOS inherits
this model and implements it on modern Rust/Tokio.

## Core Capabilities

1. **Dis VM Process Management** — Spawn and manage lightweight
   processes (dis modules) that communicate through typed channels:
   ```rust
   // ATOM: Dis Process Spawn | Coherence: 0.98
   let proc = dis::spawn(module, args, channel_set)?;
   proc.set_wave_threshold(0.8);  // auto-terminate below WAVE 0.8
   ```
   Each Dis process has:
   - A unique process ID (pid)
   - A set of typed channels for I/O
   - A WAVE score inherited from parent context
   - An ATOM trail entry recording its lifecycle

2. **Typed Channel Operations** — CSP-style channels with type safety:
   ```rust
   // Channel types
   let cmd_ch: Channel<Command> = channel::new();
   let data_ch: Channel<Vec<u8>> = channel::buffered(1024);
   let wave_ch: Channel<WaveScore> = channel::new();

   // Send (blocks until receiver ready, or buffered)
   cmd_ch.send(Command::GateCheck)?;

   // Receive (blocks until sender ready)
   let score = wave_ch.recv()?;

   // Alt (select first ready channel — CSP alt statement)
   alt! {
     cmd_ch.recv() => |cmd| handle_command(cmd),
     wave_ch.recv() => |score| update_wave(score),
     timeout(Duration::from_secs(5)) => handle_timeout(),
   }
   ```

3. **9P Channel Multiplexing** — Multiple logical channels over a
   single 9P connection:
   ```
   Physical:  [VSOCK Socket] ──── 1 connection ────→ [Styx Daemon]

   Logical:   Channel A (commands)  ──┐
              Channel B (data)      ──┼── multiplexed over 9P
              Channel C (waves)     ──┘

   Each channel maps to a fid in the 9P namespace:
     /mnt/channels/{pid}/cmd
     /mnt/channels/{pid}/data
     /mnt/channels/{pid}/wave
   ```

4. **Cross-Strand Channel Bridge** — Connect channels across AI
   strands (Claude <-> Grok <-> Gemini):
   ```
   [Claude Process] --ch_a--> [Vortex Bridge] --ch_b--> [Grok Process]
                                    |
                                    v
                              [9P/VSOCK Transport]
   ```
   The Vortex Bridge translates channel semantics across strand
   boundaries while preserving type safety and WAVE coherence.

5. **Channel Patterns** — Pre-built concurrency patterns:
   - **Pipeline**: ch_a -> process_1 -> ch_b -> process_2 -> ch_c
   - **Fan-out**: ch_in -> [process_1, process_2, ..., process_n]
   - **Fan-in**: [ch_1, ch_2, ..., ch_n] -> merger -> ch_out
   - **Request-Reply**: ch_req -> process -> ch_rep (with timeout)
   - **Pub-Sub**: publisher -> topic -> [subscriber_1, subscriber_2]
   - **Scatter-Gather**: query -> [shard_1..n] -> gather -> response

6. **Anyonic Channel Braiding** — Channels can be braided using the
   SU(2)_3 anyonic algebra to create topologically protected
   communication:
   ```
   // Two channels braided = exchange that preserves topology
   let braided = braid(ch_a, ch_b, BraidType::Sigma1)?;
   // The braid signature is recorded in the ATOM trail
   // and verified by SpiralSafe Layer 2
   ```
   This is not metaphorical — the channel exchange follows the
   Temperley-Lieb algebra with Jones polynomial verification.

## Concurrency Model

```
                    ┌─────────────────┐
                    │   Dis VM Pool    │
                    │ (Tokio Runtime)  │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
         ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
         │ Process │   │ Process │   │ Process │
         │  (pid1) │   │  (pid2) │   │  (pid3) │
         └────┬────┘   └────┬────┘   └────┬────┘
              │              │              │
         [channels]    [channels]    [channels]
              │              │              │
              └──────────────┼──────────────┘
                             │
                    ┌────────┴────────┐
                    │  9P Multiplexer  │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │  VSOCK Transport │
                    └─────────────────┘
```

## Rust/Tokio Implementation

The Inferno semantics are implemented on Tokio:
- Dis processes → Tokio tasks (lightweight, cooperative)
- Channels → tokio::sync::mpsc / oneshot / broadcast
- Alt statement → tokio::select! macro
- 9P multiplexing → tokio::io framed codec
- WAVE monitoring → background Tokio task per process

## Integration Points

- **logos-styx-9p** — All channel I/O flows through 9P; channels
  are fids in the namespace
- **logos-limbo-workspace** — Limbo workspaces use Inferno channels
  for isolated computation
- **logos-wave-advanced** — Each process carries a WAVE score;
  processes below threshold are terminated
- **logos-gait-analyzer** — Process behavioral patterns profiled
  for anomaly detection
- **coherence-mcp** — `bridge_translate` uses cross-strand channels
  for platform translation
- **vortex-bridges** — Cross-strand channel bridging via Vortex
- **SpiralSafe** — Layer 2 (Anyonic Braid Provenance) validates
  braided channel exchanges

## Conservation Law

Every channel operation preserves: **ALPHA + OMEGA = 15**

A send is structural (alpha) — it places data into a typed channel
with defined semantics. A receive is semantic (omega) — it
interprets the data in context. The channel itself is the bridge
between structure and meaning, and every message that crosses it
maintains the invariant.

// ATOM: logos-inferno-transport SKILL definition | Coherence: 0.99
