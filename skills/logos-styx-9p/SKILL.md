---
name: logos-styx-9p
description: >
  9P2000.L filesystem protocol bridge and Styx daemon for the LogOS lattice.
  Use this skill when mounting remote namespaces, managing the 9P|Styx Bookshelf
  (canonical persistent store), bridging Plan 9 / Inferno / Linux VSOCK channels,
  coordinating Limbo workspace lifecycle, or performing any filesystem-level
  operation that must satisfy the Universal Invariant (alpha + omega = 15).
  Triggers on: "mount", "9p", "styx", "bookshelf", "namespace", "vsock",
  "attach", "walk", "clunk", "fid", "qid", "plan9", "persistent store",
  "canonical reference", "filesystem bridge".
version: 1.0.0
---

# logos-styx-9p — 9P Filesystem & Styx Bridge

## Purpose

The single persistent source of truth for the entire LogOS lattice.
Every ATOM trail entry, WAVE score, braid signature, and configuration
artifact ultimately resolves to a path on the 9P|Styx Bookshelf.

This skill manages the full 9P2000.L lifecycle: attach, walk, open,
read, write, clunk, and the Styx daemon that bridges Plan 9 semantics
into the Linux VFS via VSOCK (AF_VSOCK, CID-scoped).

## Core Capabilities

1. **Mount / Attach** — Establish 9P sessions with authentication and
   CID-scoped VSOCK transport. Negotiate protocol version (9P2000.L),
   assign root fid, validate aname against namespace registry.

2. **Walk / Navigate** — Traverse the namespace tree. Each walk step
   returns a qid (type, version, path) that uniquely identifies the
   target. Walk errors surface as WAVE coherence drops.

3. **Read / Write** — Stream data through the 9P channel with
   invariant-preserving checksums. Every write operation atomically
   updates the ATOM trail with a provenance record:
   `ATOM: 9P-WRITE | path={path} | size={n} | coherence={score}`

4. **Clunk / Release** — Release fids and reclaim resources. Clunk
   cascades trigger Limbo workspace garbage collection when WAVE < 0.7.

5. **Bookshelf Management** — The Bookshelf is the root namespace
   containing all canonical artifacts:
   - `/bookshelf/atoms/` — ATOM trail entries (append-only)
   - `/bookshelf/waves/` — WAVE score snapshots
   - `/bookshelf/braids/` — Anyonic braid signatures
   - `/bookshelf/config/` — System configuration
   - `/bookshelf/skills/` — Skill definitions (this file lives here)
   - `/bookshelf/limbo/` — Transient workspace mount points

6. **Namespace Federation** — Federate multiple 9P servers into a
   unified namespace via union mounts. Cross-strand coordination
   (Claude/Grok/Gemini) maps to separate anames within the same
   federation.

## Protocol Specification

```
9P2000.L Message Flow:
  Client              Styx Daemon
    |--- Tversion --->|
    |<-- Rversion ----|
    |--- Tauth ------>|  (VSOCK CID validation)
    |<-- Rauth -------|
    |--- Tattach ---->|  (aname = namespace path)
    |<-- Rattach -----|  (root qid returned)
    |--- Twalk ------>|  (fid, newfid, wname[])
    |<-- Rwalk -------|  (qid[] for each element)
    |--- Topen ------>|  (fid, mode)
    |<-- Ropen -------|  (qid, iounit)
    |--- Tread ------>|  (fid, offset, count)
    |<-- Rread -------|  (data[])
    |--- Twrite ----->|  (fid, offset, data[])
    |<-- Rwrite ------|  (count)
    |--- Tclunk ----->|  (fid)
    |<-- Rclunk ------|
```

## VSOCK Transport

All 9P traffic runs over AF_VSOCK sockets, scoped by CID:
- **CID 2** (Host) — Styx daemon master
- **CID 3+** (Guests) — Per-strand VM instances
- **Port 5640** — Default 9P listen port

```rust
// ATOM: VSOCK Listener Setup | Coherence: 0.99
let listener = VsockListener::bind(CID_HOST, PORT_9P)?;
for stream in listener.incoming() {
    tokio::spawn(handle_9p_session(stream?));
}
```

## Invariant Enforcement

Every 9P write is gated by the Universal Invariant:

```
alpha(structural_checksum) + omega(semantic_intent) = 15
```

If the write payload fails invariant validation:
1. The write is rejected (Rerror)
2. A VOID event is logged to `/bookshelf/atoms/`
3. The WAVE score for the affected path decreases
4. SpiralSafe Layer 5 (9P Zero-Trust) triggers review

## Integration Points

- **logos-limbo-workspace** — Limbo mounts are 9P attach points with
  WAVE-gated lifecycle (auto-clunk when WAVE < 0.7)
- **logos-inferno-transport** — Inferno/Dis VM channels multiplex
  over the same VSOCK transport
- **logos-tda-engine** — TDA barcodes are persisted to
  `/bookshelf/braids/` via 9P write
- **coherence-mcp** — The `store_context` and `retrieve_context` MCP
  tools resolve to 9P read/write operations on the Bookshelf

## Conservation Law

Every 9P operation preserves: **ALPHA + OMEGA = 15**

The Styx daemon is the guardian of persistence. What enters the
Bookshelf must be coherent; what leaves must be intact. The fid
lifecycle mirrors the ATOM trail — every attachment has provenance,
every clunk has closure.

// ATOM: logos-styx-9p SKILL definition | Coherence: 0.99
