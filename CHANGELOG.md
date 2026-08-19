# Changelog

All notable lattice-level changes are recorded here. Crate-local notes may also
live under `crates/*/RELEASE-*.md` and `ops/UNITARY-RELEASE-NOTES.md`.

Format: keep entries honest with Category tags where quantitative.

---

## [0.2.2-tui] — constraint 20260818.hitl-gate

See `crates/tui/RELEASE-0.2.2.md` — HITL board · request/escalate/approve · first_need.

reson8-tui **0.2.2** loads `ops/human-actions.json` (SAIF ⚑ twin) and latches
operator decisions as receipts. Approve does not deploy.

---

## [0.3.0] — 2026-08-09

### Added

- **Operator journey + recovery** — `docs/ops/OPERATOR-JOURNEY-AND-RECOVERY-20260809.md` with stages and graceful error loops R1–R8 → remediation commands.
- **Release verifier** — `ops/release/verify-release.ps1` (Priority A smokes + remediation hints; `-Remediate`, `-Quick`).
- **Release notes** — `ops/release/RELEASE-v0.3.0.md` · root `VERSION`.
- **Hopf / pirouette investigation** — `docs/theory/HOPF-PIROUETTE-INVESTIGATION-20260809.md` (residual-zero **observe only**; no Category-D lift).
- **ArrivalDetector continuous** — path-typed `baseFix` / reverse-Hopf fragment (`agda/src/ArrivalDetector/ArrivalDetectorContinuous.agda`).
- **Entangle protocol** — path-slot ship path under receive-pack 408 constraint (`ops/entangle/*`, lattice assessment).
- **Epistemics / claim gate packet** — Lean Category consumers, cutile `claim_gate`, `tools/claim_lint.py`, Stage A/B/C SRAC docs.

### Changed

- **reson8-tui** documented at **0.2.1** (QDI drain budget, codes lab, surface layout, net panel).
- Lattice assessment 2026-08-09 orders threads by impact × (1 − uncertainty).

### Explicit non-goals

- Geometric Hopf \(S^3 \to S^2\) remains open **[B]**.
- residual-zero promotion remains blocked.
- GCP / multi-host git / cubical pin remain human ⚑.

---

## [0.2.1-tui] — 2026-08-07

See `crates/tui/RELEASE-0.2.1.md` — QDI hygiene · codes lab · residual-C.

---

## [1.0.0-unitary] — 2026-07-15

See `ops/UNITARY-RELEASE-NOTES.md` — deployment waist · unitary profile · crease tables.
