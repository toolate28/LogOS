# Entangle slot `barcode-tui`

- **Title:** barcode-tui H0 viewer
- **Priority:** A
- **Paths:** crates/barcode-tui/
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id barcode-tui
```

Then run workflow **Entangle** mode=`ingest` with this component, or commit the slice onto `entangle/barcode-tui`.
