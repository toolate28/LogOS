# Tri-Weavon Unitary Release Notes

**Version:** 1.0.0-unitary  
**Date:** 2026-07-15  
**Invariant:** α + ω = 15

## Ship contents

1. **Deployment Waist** — Nix flake, waist OCI, compose, kind, styx TCP, bbbr unix  
2. **SAIF unitary doc** — `SAIF-Docs/UNITARY-RELEASE-v1.0.md`  
3. **Crease tables** — `ops/ascii_crease_table.py` (miura/kresling/waterbomb/reidemeister/etch)  
4. **TUI etch shading** — multi-level `ascii_bar` + `etch_wave_field` in `crates/tui`  
5. **$PROFILE unitary** — `ops/TriWeavon.Unitary.Profile.psm1` + installer  

## Install profile

```powershell
pwsh -File 'F:\Users\Matthew Ruhnau\LogOS\ops\Install-TriWeavonUnitaryProfile.ps1'
. $PROFILE
tw-sensors
```

## Convert a markdown table

```bash
python3 ops/ascii_crease_table.py --file SAIF-Docs/UNITARY-RELEASE-v1.0.md --style miura
# or in-place:
python3 ops/ascii_crease_table.py --file path/md --in-place --style kresling
```

## Health

| Check | Command |
|-------|---------|
| sensors | `tw-sensors` |
| exit code | `tw-health; echo $LASTEXITCODE` |
| waist | `curl -s localhost:8080/health` |
| bbbr | `curl -s localhost:8081/verify` |
| styx | `python3 ops/styx-9p-client-smoke.py` |

(Note: remaining GFM tables in legacy docs may still exist; **new** SAIF/deploy
boards use crease fences. Full monorepo table rewrite is iterative via the generator.)
