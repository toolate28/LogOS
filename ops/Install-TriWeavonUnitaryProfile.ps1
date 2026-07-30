#Requires -Version 5.1
<#
.SYNOPSIS
    Wire Tri-Weavon Unitary module into the current user's $PROFILE (idempotent).
#>
[CmdletBinding()]
param(
    [string]$LogOSRoot = $(if ($env:LOGOS_ROOT) { $env:LOGOS_ROOT } else { 'F:\Users\Matthew Ruhnau\LogOS' })
)

$ErrorActionPreference = 'Stop'
$module = Join-Path $LogOSRoot 'ops\TriWeavon.Unitary.Profile.psm1'
if (-not (Test-Path -LiteralPath $module)) {
    throw "Unitary module not found: $module"
}

$profilePath = $PROFILE
$profileDir = Split-Path $profilePath -Parent
if (-not (Test-Path $profileDir)) {
    New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
}
if (-not (Test-Path $profilePath)) {
    New-Item -ItemType File -Path $profilePath -Force | Out-Null
}

$markerBegin = '# >>> TriWeavon.Unitary >>>'
$markerEnd = '# <<< TriWeavon.Unitary <<<'
$block = @"
$markerBegin
# Auto-wired by ops/Install-TriWeavonUnitaryProfile.ps1 — unitary cockpit
`$env:LOGOS_ROOT = '$($LogOSRoot.Replace("'", "''"))'
Import-Module '$($module.Replace("'", "''"))' -Force
# Compact status + next actions (not a glyph wall). Use: tw help
Start-TriWeavonUnitary
$markerEnd
"@

$existing = Get-Content -LiteralPath $profilePath -Raw -ErrorAction SilentlyContinue
if ($null -eq $existing) { $existing = '' }

if ($existing -match [regex]::Escape($markerBegin)) {
    $existing = [regex]::Replace(
        $existing,
        [regex]::Escape($markerBegin) + '[\s\S]*?' + [regex]::Escape($markerEnd),
        $block.TrimEnd(),
        1
    )
} else {
    if ($existing.Length -gt 0 -and -not $existing.EndsWith("`n")) {
        $existing += "`n"
    }
    $existing += "`n$block`n"
}

Set-Content -LiteralPath $profilePath -Value $existing -Encoding UTF8
Write-Host "Installed Unitary profile hook → $profilePath" -ForegroundColor Green
Write-Host "Reload: . `$PROFILE   |   Sensors: tw-sensors   |   Health: tw-health" -ForegroundColor Cyan
