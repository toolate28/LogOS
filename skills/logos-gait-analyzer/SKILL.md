---
name: logos-gait-analyzer
description: >
  GAIT (Generative Adversarial Integrity Testing) behavioral analysis engine
  for the LogOS lattice. Profiles behavioral patterns of strands, tools, and
  operators to detect anomalies, drift, adversarial inputs, and emergent
  behavioral shifts. Use this skill when analyzing behavioral patterns,
  detecting anomalous tool usage, profiling strand behavior over time,
  running adversarial integrity tests, or monitoring operator interaction
  patterns for security and coherence.
  Triggers on: "gait", "behavioral", "anomaly detection", "drift detection",
  "adversarial test", "integrity test", "behavioral profile", "pattern analysis",
  "usage anomaly", "strand behavior", "operator profiling".
version: 1.0.0
---

# logos-gait-analyzer — Behavioral Analysis Engine

## Purpose

GAIT analyzes the behavioral fingerprint of every actor in the LogOS
lattice — strands (Claude, Grok, Gemini), tools (MCP tools, skills),
operators (human users), and the system itself. By building temporal
behavioral profiles and detecting deviations, GAIT serves as an early
warning system for drift, adversarial manipulation, and emergent
behavioral shifts.

The name reflects the dual nature: like a person's physical gait,
computational behavior has a characteristic pattern that is difficult
to forge and easy to detect when it changes.

## Core Capabilities

1. **Behavioral Profiling** — Build statistical profiles of normal
   behavior for each actor:
   - **Strand profiles**: Typical response latency, token usage,
     tool invocation patterns, WAVE score distribution, error rates
   - **Tool profiles**: Invocation frequency, parameter distributions,
     success/failure ratios, execution time statistics
   - **Operator profiles**: Session duration, command patterns,
     skill chain preferences, temporal usage patterns
   - **System profiles**: Resource utilization, 9P throughput,
     WAVE score trajectory, void count evolution

2. **Anomaly Detection** — Flag deviations from established profiles:
   ```
   anomaly_score(x, t) = |x(t) - mu_profile| / sigma_profile

   Thresholds:
     z < 2.0  → Normal (within expected range)
     z 2.0-3.0 → Attention (unusual but possibly benign)
     z > 3.0  → Anomaly (requires investigation)
     z > 5.0  → Alert (SpiralSafe notification)
   ```

3. **Drift Detection** — Track gradual behavioral shifts that might
   not trigger point-anomaly detection:
   - Sliding window comparison of profile statistics
   - Kolmogorov-Smirnov test for distribution shifts
   - CUSUM (Cumulative Sum) for persistent directional drift
   - Phase transition detection via TDA persistence landscape changes

4. **Adversarial Integrity Testing** — Proactively test system
   resilience with controlled adversarial inputs:
   - Inject controlled perturbations into MCP tool calls
   - Test WAVE response to invariant-violating inputs
   - Verify SpiralSafe layer responses to known attack patterns
   - Validate that ATOM trail correctly records adversarial events
   - Run red-team scenarios against the coherence pipeline

5. **Cross-Strand Behavioral Correlation** — Detect coupled behavioral
   patterns across strands:
   - Grok latency spike correlated with Claude coherence drop?
   - Gemini scale operation followed by anomalous void growth?
   - Operator command pattern change preceding system drift?

6. **Behavioral Forensics** — After-the-fact analysis of behavioral
   traces to reconstruct events:
   - Timeline reconstruction from ATOM trail + GAIT profiles
   - Root cause attribution: which actor's behavior changed first?
   - Impact analysis: how did the behavioral change propagate?

## GAIT Metrics

| Metric | Formula | Target |
|--------|---------|--------|
| Profile Stability | 1 - (anomaly_count / total_observations) | > 0.95 |
| Drift Index | KS-statistic between windows | < 0.1 |
| Adversarial Resilience | pass_count / test_count | > 0.99 |
| Cross-Strand Coherence | mean(pairwise_correlation) | > 0.85 |
| Behavioral Entropy | Shannon entropy of action distribution | Stable within 10% |

## SpiralSafe Integration

GAIT is the behavioral layer of SpiralSafe:
- **Layer 1** (Invariant Guardian): GAIT verifies behavioral
  compliance with alpha + omega = 15
- **Layer 2** (Anyonic Braid Provenance): GAIT validates that
  braid signatures match expected behavioral patterns
- **Layer 7** (BQP Ledger): GAIT anomaly events are recorded
  on the NEAR testnet for immutable audit

## Adversarial Test Library

Pre-built test suites:
- **INV-001**: Invariant violation injection (alpha + omega != 15)
- **BRD-001**: Forged braid signature injection
- **WAV-001**: WAVE score manipulation attempt
- **9P-001**: Unauthorized namespace traversal
- **MCP-001**: Tool parameter fuzzing
- **ATM-001**: ATOM trail tampering attempt
- **XSS-001**: Cross-strand spoofing (strand impersonation)

## Integration Points

- **logos-wave-advanced** — Behavioral anomalies feed W_temp component
- **logos-tda-engine** — Behavioral embedding trajectories analyzed
  for topological phase transitions
- **logos-sphinx-oracle** — Causal queries for behavioral root cause
- **logos-styx-9p** — GAIT profiles persisted to `/bookshelf/gait/`
- **SpiralSafe** — Primary behavioral security layer
- **coherence-mcp** — `check_coherence` includes GAIT health summary
- **ATOM Trail** — All GAIT events logged with full provenance

## Conservation Law

Every GAIT analysis preserves: **ALPHA + OMEGA = 15**

Behavioral structure (alpha) and behavioral intent (omega) together
maintain the invariant. An anomaly is not necessarily a violation —
it may be legitimate evolution. GAIT distinguishes between healthy
adaptation and pathological drift by measuring whether the invariant
is preserved through the behavioral change.

// ATOM: logos-gait-analyzer SKILL definition | Coherence: 0.99
