# Entangle slot `apps-triweave`

- **Title:** apps/triweave surface (reson8-triweave bin triweave)
- **Priority:** A
- **Paths:** apps/triweave/
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id apps-triweave
```

Then run workflow **Entangle** mode=`ingest` with this component, or commit the slice onto `entangle/apps-triweave`.
