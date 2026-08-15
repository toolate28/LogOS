# Entangle slot `ci-verify`

- **Title:** CI policy + verify pipeline
- **Priority:** A
- **Paths:** .github/workflows/;ops/ci/
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id ci-verify
```

Then run workflow **Entangle** mode=`ingest` with this component, or commit the slice onto `entangle/ci-verify`.
