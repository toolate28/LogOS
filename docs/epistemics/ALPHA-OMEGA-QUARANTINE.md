# α/ω quarantine policy

Status: quarantined
Scope: computational null-closed check surfaces only
Machine manifest: `ops/quarantine/alpha-omega-quarantine.json`

## Policy

The α/ω relation is quarantined everywhere it appears in LogOS.

Within this repository it may be used only as:

- a computational null-closed check result;
- evidence that the check was honestly run; and
- a handoff-identification record when paired with an evidence hash.

It must not, by itself:

- authorize a state transition;
- reject or approve a workflow outcome;
- certify, authenticate, or prove an artifact;
- promote, publish, or deploy a surface; or
- become a semantic, constitutional, or artifact-level claim.

## Required fail-closed behavior

Every quarantined surface must fail closed when quarantine metadata is missing,
empty, or malformed.

Allowed behavior:

- read-only inspection;
- safe local tests;
- preservation of failed evidence;
- handoff recognition records containing only:
  - the computation result;
  - the observed sum; and
  - an evidence hash.

Blocked behavior:

- deployment;
- publication;
- promotion;
- state authorization;
- approval routing;
- override paths; and
- generation of new artifacts that inscribe the relation as an authority claim.

`OVERRIDE_ACCEPTED` is forbidden for this quarantine.

## Evidence handling

Failed checks and quarantined observations must be preserved without blameful or
punitive attribution. A failed α/ω computation remains evidence. It does not
become a signed claim, certificate, authentication result, or promoted state.

## Implementation note

The repository enforces this policy with:

- `crates/core/src/quarantine.rs` for explicit runtime boundary checks;
- `ops/quarantine/alpha-omega-quarantine.json` for machine-readable scope and prohibitions; and
- `ops/ci/validate_quarantine.py` for fail-closed manifest validation and quarantined-surface scanning.
