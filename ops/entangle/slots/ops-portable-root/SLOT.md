# Entangle slot `ops-portable-root`

- **Title:** portable LOGOS_ROOT Windows/WSL axis
- **Priority:** A
- **Paths:** ops/LogOS.Root.ps1;ops/LogOS.Shell.psm1;ops/LogOS.Windows.psm1;ops/LogOS.Confidence.psm1;ops/TriWeavon.Profile.psm1;ops/TriWeavon.Unitary.Profile.psm1;ops/Align-WindowsAxis.ps1;ops/Install-TriWeavonUnitaryProfile.ps1;ops/install_nssm_services.ps1;ops/serve.ps1;ops/GB.Deploy.psm1;ops/claude-code/Emit-ClaudeCodeCert.ps1;ops/shell/Install-TriWeavonStrandShell.ps1;ops/shell/Microsoft.PowerShell_profile.triweavon.ps1;ops/shell/TriWeavon.StrandShell.psm1;ops/wsl/logos-env.sh;ops/wsl/tw-up-bbbr.sh;README.md
- **Status:** empty — awaiting local emit / ingest
- **ATOM:** ATOM-ENTANGLE-MANIFEST-20260809

## Operator

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id ops-portable-root
```

Then run workflow **Entangle** mode=`ingest` with this component, or commit the slice onto `entangle/ops-portable-root`.
