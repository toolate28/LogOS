# Entangle slot `ci-verify`

- **Title:** Local verify scaffolding
- **Priority:** A
- **Paths:** ops/ci/
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id ci-verify
```

Then apply the slice locally or commit it onto `entangle/ci-verify`.
