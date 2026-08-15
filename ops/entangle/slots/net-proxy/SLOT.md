# Entangle slot `net-proxy`

- **Title:** LogOS net proxy stack (TUI Net panel)
- **Priority:** B
- **Paths:** ops/net/;crates/tui/src/net_proxy.rs
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id net-proxy
```

Then run workflow **Entangle** mode=`ingest` with this component, or commit the slice onto `entangle/net-proxy`.
