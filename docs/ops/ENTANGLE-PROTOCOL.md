# Entangle protocol — remote structure · local payload · human collapse

**ATOM:** `ATOM-ENTANGLE-PROTOCOL-20260809`  
**Related:** `ops/entangle/*` · `.github/workflows/entangle.yml` · lattice assessment 2026-08-09

## One-sentence law

**Remote opens the empty particle (PR slot); local fills the state (path slice); human measurement (approve) collapses both into `main`.**

## Why not `git push main`?

See transfer-lane diagnosis: 408 receive-pack + LFS S3 stall + historical `target/` bloat.  
Entangle is the structural workaround that preserves intent without requiring a single monolithic pack.

## States

| State | Remote | Local | Next |
|-------|--------|-------|------|
| Vacuum | no slot | work exists | scaffold |
| Entangled empty | PR open, SLOT.md only | emit-slice ready | populate |
| Superposed payload | branch has slice | receipt zip | CI Verify |
| Collapsed | merged | origin synced | next component |

## Authority

- Actions may open branches/PRs and apply slices **only** when dispatched.  
- Actions may **not** merge without human (no auto-merge in workflow).  
- Secrets / deploy / GCP remain SAIF ⚑ human.

## Reproduce

```powershell
python ops/entangle/validate_manifest.py
python ops/ci/assert_action_pins.py
gh workflow run entangle.yml -f mode=validate
gh workflow run entangle.yml -f mode=scaffold -f components=reson8-tui,barcode-tui,formal-srac
```
