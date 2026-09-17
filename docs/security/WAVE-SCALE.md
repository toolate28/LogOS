# WAVE scale — posture bands, not authority

**Canonical scale: 0–100 integer.**  
WAVE is an operational posture signal that feeds execution bands; it does **not** by itself
authorize proof, promotion, authentication, or deployment.

## Inputs

- **Stress** — external load, novelty, outages, concurrency, hostile/noisy context.
- **Strain** — internal inconsistency, failing proofs/tests, replay drift, stale lineage.
- **Uncertainty** — derived from stress, strain, signal completeness, and divergence.

Missing signals always increase uncertainty and tighten the allowed execution band.

## Bands

| Band | WAVE guidance | Meaning | Allowed execution |
|------|---------------|---------|-------------------|
| Observe-only | < 60 or unknown | high uncertainty / missing signals | preserve evidence, quarantine, repair, replay locally |
| Reference / validation | 60–84 | bounded but incomplete confidence | docs, schemas, prototypes, local validation, targeted tests |
| Integration / replay candidate | 85–97 | bounded operational confidence | integration tests, replay, dry-runs, recovery rehearsal |
| High-coherence target | 98–100 | preferred operating target when measurable | same as above, with tighter confidence; still not promotion authority |

## Notes

- **85 / 0.85** remains the historical floor for integration/publish posture in existing docs.
- **0.98** remains the preferred high-coherence operating target in strand guidance.
- Neither value can substitute for the proof/test/replay/provenance predicates defined in
  `docs/epistemics/REPOSYSTEM-TRUTH-CLOSURE-PLAN.md`.
- `α + ω = 15` remains a Category C read-only computational check and handoff-identification
  signal only.
