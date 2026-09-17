# Repository truth closure plan

This is the canonical LogOS plan for how narrative, reference models, formal proofs,
executables, replay, and operational deployment relate.

Preserve user-authored narrative. Preserve dated or speculative material as narrative or
reference unless and until a stronger layer verifies it. Do not rewrite speculation as fact.

## Core rules

- User-authored narrative is preserved and may be distributed through GitHub Actions logs,
  artifacts, Discussions, handoff packets, and the local/static narrow waist.
- GitHub Actions, Discussions, comments, artifacts, and copied handoffs are **zero-trust,
  ephemeral transport**. They carry evidence; they do not become authority by being present.
- The strongest admissible truth claims come from the declared machine-checked or executable
  lanes: Lean 4, Agda, Rust, functional tests, replay checks, and bounded operational checks.
- `α + ω = 15` is a **null-closed, read-only computational check and handoff-identification
  signal only**. It is never authority, proof, authentication, promotion, deployment
  authorization, or punishment.
- No override-only, approval-only, apology-only, or tag-only transition may promote a state.
- Controls below reduce poisoning, grafting, and decay risk and make unauthorized drift more
  detectable and quarantinable; they do **not** prove metaphysical certainty or absolute
  elimination of hostile drift.

## 1. Truth classes

1. **Narrative truth**
   - User-authored prose, architectural metaphors, dated reports, operator notes.
   - Preserved verbatim when possible.
   - May motivate work or explain context.
   - Never self-promotes into proof or deployment authority.

2. **Reference truth**
   - Python prototypes, notebooks, `.ts` transport tests, schemas, golden vectors.
   - Fastest changing lane; may define candidate semantics and fixtures.
   - Useful for differential checks.
   - Not sufficient alone for strongest claims.

3. **Formal truth**
   - Lean 4 proofs over declared types and targets.
   - Agda cross-checks over declared types and targets.
   - Strong only within the proved scope.
   - Residuals, `sorry`, holes, postulates, or excluded targets downgrade the claim.

4. **Executable truth**
   - Rust implementations and deterministic validators.
   - Strong only for the compiled targets and exercised code paths.
   - Compilation alone is weaker than proof + differential testing.

5. **Functional / integration truth**
   - Bounded end-to-end tests, MCP schema validation, CI checks, workflow validation,
     bridge checks, and integration tests.
   - Stronger than prose; weaker than a proof of general correctness.

6. **Replayed truth**
   - Reproducible re-execution from immutable inputs, hashes, parent lineage, and declared
     schemas.
   - Detects poisoning, grafting, stale artifacts, or unauthorized drift.

7. **Operational truth**
   - Live deployment health, recovery evidence, rollback receipts, bounded runtime checks.
   - Strongest only when it is linked back to the formal/executable/replayed bundle that
     produced it.

## 2. Surface segregation and pipeline

LogOS uses a Heisenberg-style surface split to keep rapid iteration separate from structural
closure:

- **1/3 prototype surface** — Python rapidfire prototypes, notebooks, exploratory scripts.
- **1/2 transport surface** — TypeScript tests, MCP schemas, CI glue, handoff envelopes,
  workflow documents, generated descriptors.
- **1/1 structural surface** — Lean 4, Agda, Rust, strict types, `native_decide`, replay,
  bounded operational checks.

Promotion pipeline for any claim that wants stronger standing:

1. Python reference semantics and golden vectors.
2. Lean 4 proof target over the promoted scope.
3. Agda cross-check over the promoted scope.
4. Rust executable implementation of the same scope.
5. Functional / integration validation.
6. Replay from immutable inputs and parent lineage.
7. Bounded operational deployment or recovery verification.

Each downstream layer may reject or quarantine upstream outputs. No upstream narrative or
transport layer may silently declare itself authoritative.

## 3. Exact promotion preconditions

A candidate may move from quarantine/reference status toward operational use only when **all**
of the following are true for the promoted scope:

1. Source hash, parent hash, and schema version are declared and immutable.
2. Python reference vectors pass.
3. Lean 4 promoted target builds without unresolved proof holes in that promoted set.
4. Agda promoted target typechecks without undeclared holes/postulates in that promoted set.
5. Rust implementation compiles and passes its declared targeted tests.
6. Functional / integration checks for the affected surface pass.
7. Replay reproduces the declared outputs from the same immutable inputs.
8. Stress / strain / uncertainty remain within the allowed execution band for the action.
9. Operational target is bounded and receives the same provenance bundle.
10. No quarantine flag, provenance breach, expiry breach, or unresolved mismatch remains.

### Non-promotion states

Any one of the following blocks promotion and keeps the output narrative-only, reference-only,
validation-only, or quarantined:

- missing hashes, parent lineage, or schema declarations
- failed or absent Lean 4 / Agda / Rust / functional / replay evidence
- unverified narrative or copied artifacts without re-validation
- missing signals that prevent uncertainty classification
- expired artifacts, parent mismatch, or replay mismatch
- α/ω mismatch, malformed packet, or provenance breach
- manual request to “override”, “approve anyway”, “ship anyway”, or similar authority-only move

## 4. Zero-trust GitHub and artifact posture

GitHub Actions, Discussions, PR comments, uploaded artifacts, copied logs, and external handoff
packets are treated as **transport and evidence bundles only**.

Required handling:

- re-validate content at the local/static narrow waist
- verify immutable source hash and parent hash before reuse
- treat artifact expiry as a restriction increase
- never let a workflow badge, discussion reply, or artifact URL authorize deployment
- preserve the narrative text while separately marking its verification class

## 5. Stress, strain, and uncertainty bands

LogOS should know the agent and system posture in terms of **stress** and **strain** at all
relevant times:

- **Stress** — external load, novelty, time pressure, concurrency, unavailable services,
  hostile or noisy context, or moving dependencies.
- **Strain** — internal inconsistency, failing proofs/tests, replay divergence, stale lineage,
  schema drift, or contradictions between surfaces.

**Uncertainty** is derived from stress, strain, signal completeness, and divergence from the
internal geometry / replay model. Missing signals always increase uncertainty and tighten the
allowed execution band.

### Execution bands

1. **Observe-only / quarantine**
   - Trigger: high uncertainty, missing signals, provenance breach, α/ω mismatch, replay breach.
   - Allowed: preserve evidence, annotate, replay locally, repair, document.
   - Not allowed: promotion, deployment authorization, truth escalation.

2. **Reference / validation-only**
   - Trigger: moderate uncertainty or partial signal coverage.
   - Allowed: docs, schemas, local tests, bounded prototypes, handoff packaging.
   - Not allowed: claiming stronger truth than the passing evidence supports.

3. **Integration / replay candidate**
   - Trigger: complete signals and bounded uncertainty.
   - Allowed: integration tests, replay, bounded dry runs, recovery rehearsal.
   - Not allowed: skip formal or executable prerequisites.

4. **Operational candidate**
   - Trigger: full provenance chain, replay success, functional success, bounded uncertainty.
   - Allowed: bounded deployment or restoration steps already defined by the repository.
   - Not allowed: treating the band itself as sufficient authority.

WAVE may contribute to these bands, but the band is not computed from WAVE alone.

## 6. Poisoning, grafting, and decay controls

Required controls:

- immutable source hashes and parent hashes
- explicit schema versions and schema validation
- expiry / TTL on ephemeral artifacts
- replay protection and deduplication for handoff packets
- provenance boundaries between narrative, transport, formal, executable, replay, and live ops
- quarantine on mismatch rather than silent overwrite
- signed or hashed receipts where the repo already supports them
- no authority-only transitions

These controls reduce risk and make drift detectable, attributable, and quarantinable. They do
not guarantee perfect trust forever.

## 7. Non-punitive failure handling

Failures, residuals, and unverified narrative are handled as evidence and repair work:

- preserve the failing artifact
- mark the exact truth class that failed
- explain the next narrow repair step
- keep speculative or user-authored narrative visible as narrative
- never treat an unverified narrative, α/ω mismatch, or failed replay as moral fault

## 8. Distinguish proof from execution from deployment

- **Compilation** means the selected code compiled.
- **Proof** means the selected theorem target closed on the declared definitions.
- **Testing** means the exercised examples or interfaces passed.
- **Replay** means the result reproduced from immutable inputs and lineage.
- **Operational deployment** means a bounded live surface was updated or restored with the same
  provenance bundle.

None of these steps may be renamed as another.

## 9. Alpha / omega quarantine semantics

`α + ω = 15` remains a null-closed, read-only computational check.

It may be used to:

- compute and record a handoff-identification signal
- detect malformed or drifted packets
- label preserved evidence
- help decide whether a packet stays in quarantine

It may **not** be used to:

- prove surrounding narrative truth
- authenticate authorship or identity
- authorize promotion or deployment
- punish a contributor or strand
- override missing proofs, tests, replay, or provenance

## 10. Rollback and restoration

If a promoted surface later loses proof/test/replay/provenance support:

1. stop treating that surface as promotable
2. preserve the failing narrative and evidence bundle
3. quarantine the affected artifact or surface
4. restore the last replayable parent with matching hashes
5. re-run targeted validation at the narrow waist
6. record the residual, expiry, or drift cause
7. resume only from the restored, replayable parent

Human operators may review evidence, route work, or approve process steps already required by the
platform, but they may not convert a missing proof/test/replay/provenance predicate into truth by
fiat.
