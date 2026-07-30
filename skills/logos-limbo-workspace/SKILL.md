---
name: logos-limbo-workspace
description: >
  Ephemeral transient workspace manager for the LogOS lattice. Creates,
  monitors, and garbage-collects temporary workspaces with WAVE-gated
  lifecycle. Limbo workspaces exist as 9P mount points that auto-purge
  when WAVE coherence drops below 0.7. Use this skill when creating
  temporary work areas, managing scratch space for experiments, isolating
  speculative computations, sandboxing untrusted operations, or managing
  the lifecycle of transient artifacts that should not persist.
  Triggers on: "limbo", "temporary workspace", "scratch space", "transient",
  "ephemeral", "sandbox", "experiment space", "auto-purge", "workspace
  lifecycle", "temp mount", "speculative computation".
version: 1.0.0
---

# logos-limbo-workspace — Ephemeral Workspace Manager

## Purpose

Limbo is where ideas go to be tested before they earn persistence.
It provides isolated, WAVE-gated workspaces for speculative
computation, experimentation, and untrusted operations. If the work
proves coherent (WAVE >= 0.7), it can be promoted to the Bookshelf.
If it drifts below threshold, it is automatically purged.

This mirrors the theological concept: Limbo is neither permanent
salvation (Bookshelf) nor damnation (deletion) — it is the in-between
space where coherence is still being determined.

## Core Capabilities

1. **Workspace Creation** — Instantiate a new Limbo workspace:
   ```
   limbo.create({
     name: "experiment-ftqec-sim",
     ttl: 3600,           // max lifetime in seconds
     wave_threshold: 0.7, // auto-purge below this
     max_size: "1GB",     // storage quota
     isolation: "full"    // network isolation level
   })
   ```
   Returns a 9P mount point at `/bookshelf/limbo/{name}/` with a
   fresh fid attached to the Styx daemon.

2. **WAVE-Gated Lifecycle** — Continuous monitoring:
   ```
   every 10 seconds:
     wave = wave_score(workspace)
     if wave < workspace.wave_threshold:
       grace_period(30s)  // brief grace before purge
       if wave still < threshold:
         purge(workspace)
         log_atom("LIMBO-PURGE", workspace.name, wave)
     if now > workspace.created_at + workspace.ttl:
       purge(workspace)
       log_atom("LIMBO-TTL", workspace.name, "expired")
   ```

3. **Promotion to Bookshelf** — When work is complete and coherent:
   ```
   limbo.promote({
     workspace: "experiment-ftqec-sim",
     target: "/bookshelf/braids/ftqec-sim-results/",
     wave_minimum: 0.9  // promotion requires higher threshold
   })
   ```
   Promotion atomically moves artifacts from Limbo to Bookshelf,
   creates ATOM trail entries for provenance, and clunks the Limbo
   fid.

4. **Isolation Levels** — Control what Limbo workspaces can access:
   - **full**: No access to Bookshelf or network (pure sandbox)
   - **read-only**: Can read Bookshelf but not write
   - **gated**: Can write to Bookshelf via WAVE-gated promotion only
   - **transparent**: Full access (for trusted experiments only)

5. **Garbage Collection** — Automatic cleanup:
   - TTL expiry: hard deadline regardless of WAVE score
   - WAVE purge: coherence below threshold for > grace period
   - Size quota: oldest artifacts pruned when quota exceeded
   - Manual purge: operator command to force-cleanup
   - Cascade purge: if parent workspace is purged, children follow

6. **Workspace Inspection** — View current Limbo state:
   ```
   limbo.list()       // all active workspaces
   limbo.inspect(name) // details: size, wave, age, artifacts
   limbo.history()    // recently purged/promoted workspaces
   ```

## Limbo Lifecycle Diagram

```
[Create Request]
      |
      v
[Allocate 9P Mount]  → /bookshelf/limbo/{name}/
      |
      v
[WAVE Monitor Active]
      |
  ┌───┴───┐
  v       v
[Work]  [WAVE Check every 10s]
  |       |
  v       v
[Done?] [WAVE < 0.7?]
  |       |
  v       v
[Promote] [Grace Period 30s]
  |       |
  v       v
[Bookshelf] [Still < 0.7?]
              |
         ┌────┴────┐
         v         v
       [Yes]    [No: Resume]
       Purge
         |
         v
   [ATOM: LIMBO-PURGE]
   [Clunk fid]
   [Free storage]
```

## Use Cases

- **FTQEC Simulation**: Run quantum error correction simulations in
  Limbo. If results are coherent, promote to Bookshelf.
- **Adversarial Testing**: GAIT adversarial tests execute in isolated
  Limbo workspaces to prevent contamination.
- **Speculative Retrieval**: RAG-CAG can stage candidate contexts in
  Limbo for evaluation before inclusion.
- **Strand Experimentation**: Test new Claude/Grok/Gemini tool
  integrations in Limbo before production deployment.
- **Untrusted Input Processing**: Process external data in Limbo
  sandbox before allowing Bookshelf access.

## Integration Points

- **logos-styx-9p** — Limbo workspaces ARE 9P mount points; all I/O
  flows through the Styx daemon
- **logos-wave-advanced** — WAVE score gates workspace lifecycle
- **logos-gait-analyzer** — Adversarial tests run in Limbo isolation
- **logos-rag-cag** — Speculative retrieval contexts staged in Limbo
- **logos-void-mapper** — Void remediation experiments in Limbo
- **SpiralSafe** — Limbo isolation is a security boundary
- **coherence-mcp** — `store_context` can target Limbo for temporary
  cross-platform state

## Conservation Law

Every Limbo operation preserves: **ALPHA + OMEGA = 15**

A Limbo workspace is a hypothesis — it has structural form (alpha)
and semantic intent (omega). The WAVE gate ensures that only work
maintaining the invariant earns persistence. Purging is not failure;
it is the system correctly rejecting incoherent output. The
conservation law is enforced by the lifecycle itself.

// ATOM: logos-limbo-workspace SKILL definition | Coherence: 0.99
