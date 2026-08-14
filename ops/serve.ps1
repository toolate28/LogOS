# TriWeavon bridge launcher — dual-protocol ws://127.0.0.1:8088
param(
    [string]$Addr = '127.0.0.1:8088',
    [string]$LogOSRoot = ''
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'LogOS.Root.ps1')
if (-not $LogOSRoot) {
    $LogOSRoot = Resolve-LogOSRootPortable -ScriptRoot $PSScriptRoot -ThrowIfMissing
}
$module = Join-Path $PSScriptRoot 'TriWeavon.Profile.psm1'
if (Test-Path $module) {
    Import-Module $module -Force
    $env:LOGOS_ROOT = $LogOSRoot
    Start-TriWeavonBridge -Addr $Addr
} else {
    Push-Location $LogOSRoot
    try {
        cargo run -p reson8-triweave -- serve --addr $Addr
    } finally {
        Pop-Location
    }
}