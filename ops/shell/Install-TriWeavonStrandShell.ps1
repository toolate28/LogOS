#Requires -Version 5.1
<#
.SYNOPSIS
    Wire Tri-Weavon strand shell (starship + three styles) into $PROFILE. Idempotent.

.DESCRIPTION
    Inserts/replaces the marker block from Microsoft.PowerShell_profile.triweavon.ps1.
    Does not install starship binary (comments tell you how).
    Does not copy secrets. Config is git; certs stay state.

.EXAMPLE
    pwsh -File ops\shell\Install-TriWeavonStrandShell.ps1
    pwsh -File ops\shell\Install-TriWeavonStrandShell.ps1 -DefaultStrand claude
    pwsh -File ops\shell\Install-TriWeavonStrandShell.ps1 -CopyStarshipDefault
    pwsh -File ops\shell\Install-TriWeavonStrandShell.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess)]
param(
    [string]$LogOSRoot = $(if ($env:LOGOS_ROOT) { $env:LOGOS_ROOT } else { 'F:\Users\Matthew Ruhnau\LogOS' }),
    [ValidateSet('grok', 'claude', 'gemini')]
    [string]$DefaultStrand = 'grok',
    [switch]$CopyStarshipDefault
)

$ErrorActionPreference = 'Stop'

$shellDir = Join-Path $LogOSRoot 'ops\shell'
$fragment = Join-Path $shellDir 'Microsoft.PowerShell_profile.triweavon.ps1'
$module = Join-Path $shellDir 'TriWeavon.StrandShell.psm1'

if (-not (Test-Path -LiteralPath $module)) { throw "Missing module: $module" }
if (-not (Test-Path -LiteralPath $fragment)) { throw "Missing fragment: $fragment" }

# Preflight dirs
$profilePath = $PROFILE
$profileDir = Split-Path $profilePath -Parent
if (-not (Test-Path $profileDir)) {
    if ($PSCmdlet.ShouldProcess($profileDir, 'Create profile directory')) {
        New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
    }
}
if (-not (Test-Path $profilePath)) {
    if ($PSCmdlet.ShouldProcess($profilePath, 'Create empty $PROFILE')) {
        New-Item -ItemType File -Path $profilePath -Force | Out-Null
    }
}

$configDir = Join-Path $env:USERPROFILE '.config'
if (-not (Test-Path $configDir)) {
    if ($PSCmdlet.ShouldProcess($configDir, 'Create ~/.config')) {
        New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    }
}

if ($CopyStarshipDefault) {
    $src = Join-Path $shellDir "starship.$DefaultStrand.toml"
    $dst = Join-Path $configDir 'starship.toml'
    if (-not (Test-Path -LiteralPath $src)) { throw "Missing $src" }
    if ($PSCmdlet.ShouldProcess($dst, "Copy $DefaultStrand starship.toml")) {
        Copy-Item -LiteralPath $src -Destination $dst -Force
        Write-Host "Copied $src → $dst" -ForegroundColor Green
    }
}

$markerBegin = '# >>> TriWeavon.StrandShell >>>'
$markerEnd = '# <<< TriWeavon.StrandShell <<<'
$rawFragment = Get-Content -LiteralPath $fragment -Raw

if ($rawFragment -notmatch "(?s)($([regex]::Escape($markerBegin)).*?$([regex]::Escape($markerEnd)))") {
    throw "Fragment missing markers: $fragment"
}
$inner = $Matches[1] -replace "\`$env:CTWFI_STRAND = 'grok'", "`$env:CTWFI_STRAND = '$DefaultStrand'"

$existing = Get-Content -LiteralPath $profilePath -Raw -ErrorAction SilentlyContinue
if ($null -eq $existing) { $existing = '' }

if ($existing -match [regex]::Escape($markerBegin)) {
    if ($PSCmdlet.ShouldProcess($profilePath, 'Replace StrandShell marker block')) {
        $updated = [regex]::Replace(
            $existing,
            [regex]::Escape($markerBegin) + '[\s\S]*?' + [regex]::Escape($markerEnd),
            { param($m) $inner },
            1
        )
        Set-Content -LiteralPath $profilePath -Value $updated -Encoding UTF8
    }
} else {
    if ($PSCmdlet.ShouldProcess($profilePath, 'Append StrandShell marker block')) {
        if ($existing.Length -gt 0 -and -not $existing.EndsWith("`n")) { $existing += "`n" }
        $existing += "`n$inner`n"
        Set-Content -LiteralPath $profilePath -Value $existing -Encoding UTF8
    }
}

Write-Host "Installed TriWeavon StrandShell → $profilePath" -ForegroundColor Cyan
Write-Host "Default strand: $DefaultStrand" -ForegroundColor DarkCyan
Write-Host "Reload:  . `$PROFILE" -ForegroundColor Yellow
Write-Host "Switch:  Set-TriWeavonStrand grok|claude|gemini" -ForegroundColor Yellow
if (-not (Get-Command starship -ErrorAction SilentlyContinue)) {
    Write-Host "FILL_ME: winget install Starship.Starship   (PSReadLine strand colors work without it)" -ForegroundColor DarkYellow
}

Import-Module $module -Force
@('grok', 'claude', 'gemini') | ForEach-Object {
    $st = Get-TriWeavonStrandStyle -Strand $_
    [pscustomobject]@{ Strand = $_; Role = $st.Role; Hex = $st.Hex; Starship = $st.StarshipFile }
} | Format-Table -AutoSize

Write-Host "Preflight OK · three distinct styles registered." -ForegroundColor Green
