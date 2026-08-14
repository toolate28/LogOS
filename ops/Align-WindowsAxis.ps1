#Requires -Version 5.1
<#
.SYNOPSIS
    One-shot: lock git / wrangler / $PROFILE / dynamic terminal on the Windows axis.
.EXAMPLE
    pwsh -File ops\Align-WindowsAxis.ps1
    pwsh -File ops\Align-WindowsAxis.ps1 -InstallWrangler
#>
[CmdletBinding()]
param(
    [switch]$InstallWrangler,
    [switch]$SkipProfile,
    [switch]$Quiet
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'LogOS.Root.ps1')
$Root = Resolve-LogOSRootPortable -ScriptRoot $PSScriptRoot -ThrowIfMissing
$Module = Join-Path $Root 'ops\LogOS.Windows.psm1'
if (-not (Test-Path -LiteralPath $Module)) {
    throw "Missing $Module"
}

Import-Module $Module -Force
# Also load shell surface for logos-* if present
$shell = Join-Path $Root 'ops\LogOS.Shell.psm1'
if (Test-Path $shell) {
    Import-Module $shell -Force
    Initialize-LogOSShell -Quiet -ImportTriWeavon -ErrorAction SilentlyContinue | Out-Null
}

$code = Set-LogOSWindowsAxis -InstallWrangler:$InstallWrangler -SkipProfile:$SkipProfile -Quiet:$Quiet
Write-Host ''
Write-Host '=== Windows axis checkpoint ===' -ForegroundColor Cyan
Write-Host "  LOGOS_ROOT=$env:LOGOS_ROOT"
Write-Host '  preflight .ps1: adhealth-meaningseed\bin\Preflight.ps1, Run.ps1'
Write-Host '  hup:            hup\instance3-rvm\Preflight-Guest.ps1'
Write-Host '  modules:        ops\LogOS.Windows.psm1, ops\GB.Deploy.psm1'
Write-Host '  wrangler:       coherence-mcp\coherence-site\wrangler.toml'
Write-Host '  terminal:       coherence-mcp\coherence-site\public\terminal\index.html'
Write-Host '  cmds: logos-preflight | logos-align | logos-wrangler | logos-terminal | tw'
Write-Host ''
exit $code
