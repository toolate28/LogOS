# Ephemeral Zero-Trust Shared Context

## Status

This document extends the existing LogOS alpha/omega quarantine posture with an **ephemeral zero-trust shared-context layer**.

The alpha/omega relation remains a **read-only computational check** and **handoff-identification signal** only. It is **never** authority, proof, authentication, promotion, deployment authorization, or a punitive gate.

## Core policy

GitHub Actions events, workflow conclusions, artifacts, Discussions, pull request comments, labels, approvals, and external narrative are **untrusted observations**.

They may be transported as ephemeral context only when they are:

1. schema-validated,
2. hash-bound,
3. freshness-checked,
4. repository-bound, and
5. prevented from directly causing a state transition.

No shared-context envelope may directly merge, deploy, publish, approve, override, force, or promote state.
`authority_effect` must remain `none`.

## Evidence classes

The narrow waist may transport evidence, but it does not certify truth.

| Evidence class | Meaning | Trust posture |
|---|---|---|
| narrative | Human or model-authored claims, comments, explanations, apologies, summaries | Unverified observation unless independently bound and checked |
| reference | Identifiers, URLs, run IDs, issue IDs, labels, refs | Pointer only; never authority |
| formal | Proof objects, theorem status, typecheck results | Bounded to the exact checked target; transport does not promote scope |
| executable | Deterministic program output and validator results | Trusted only for the exact input hash, binary/source revision, and execution boundary |
| functional | Derived operating signals such as uncertainty bands | Restrictive guidance only; never identity, moral, or consciousness inference |
| replayed | Reconstructed evidence re-derived from recorded inputs and hashes | Useful for drift detection; still subject to freshness and repository binding |
| operational | CI observations, workflow states, artifact metadata, runtime receipts | Transport evidence only; never certifies truth or authority |

## Repository binding and freshness

Every ephemeral envelope must bind at minimum:

- source repository,
- event and workflow-run identifiers,
- commit SHA and ref,
- observations,
- generated timestamp,
- expiry timestamp,
- parent context hash,
- context hash,
- evidence hashes.

Malformed, mismatched, replayed, stale, or expired envelopes fail closed.

## Human narrative handling

Human narrative may be captured only as an observation with:

- `truth_status = unverified`
- `authority_effect = none`

Narrative is retained as evidence transport, not as certification.

## Uncertainty integration

Operational signals such as `stress`, `strain`, `provenance_confidence`, `crosscheck_confidence`, and `rollback_available` may be summarized into deterministic execution bands:

- `analysis`
- `reversible`
- `constrained`
- `quarantine`
- `fully_restricted`

Missing signals must increase restriction.
These signals describe **execution uncertainty**, not mental state, intent, moral worth, or consciousness.

## Security posture

These controls are designed to **reduce and expose poisoning, grafting, and decay risk** across shared context.
They **cannot guarantee absolute elimination** of those risks.

## Non-authority boundary

The following remain prohibited as direct consequences of ephemeral shared context:

- merge authorization,
- deployment authorization,
- publication authorization,
- permission changes,
- promotion,
- punitive user judgments,
- approval-based bypasses,
- `OVERRIDE_ACCEPTED`-style exceptions.

Shared context is for observation, validation, replay, and quarantine-aware handoff only.
