#Requires -Version 5.1
<#
.SYNOPSIS
    Repair global npm wrangler when shim exists but bin/wrangler.js is missing.

.DESCRIPTION
    Observed failure (2026-07-22):
      Cannot find module ...\npm\node_modules\wrangler\bin\wrangler.js
    Package partially installed (no bin/). Fix: reinstall global wrangler.

.EXAMPLE
    pwsh -File ops/claude-code/Repair-Wrangler.ps1
    pwsh -File ops/claude-code/Repair-Wrangler.ps1 -Version 4.111.0
#>
[CmdletBinding()]
param(
    [string]$Version = '4.111.0'
)

$ErrorActionPreference = 'Stop'

function Test-WranglerHealthy {
    $cmd = Get-Command wrangler -ErrorAction SilentlyContinue
    if (-not $cmd) { return $false }
    try {
        $out = & wrangler --version 2>&1 | Out-String
        if ($out -match 'Cannot find module|MODULE_NOT_FOUND') { return $false }
        if ($LASTEXITCODE -ne 0 -and -not ($out -match '^\s*[\d.]+')) {
            # wrangler often prints version to stdout with 0
        }
        return ($out -match '\d+\.\d+')
    } catch {
        return $false
    }
}

Write-Host '=== Wrangler repair ===' -ForegroundColor Cyan
if (Test-WranglerHealthy) {
    $v = & wrangler --version 2>&1 | Out-String
    Write-Host "Already healthy: $($v.Trim())" -ForegroundColor Green
    Write-Host "path: $((Get-Command wrangler).Source)"
    exit 0
}

Write-Host "Reinstalling wrangler@$Version globally..." -ForegroundColor Yellow
# Note: if allow-scripts blocked, workerd postinstall may warn; --version still works for CLI repair.
npm install -g "wrangler@$Version"
if ($LASTEXITCODE -ne 0) {
    Write-Host 'npm install failed. Try: npm install -g wrangler --force' -ForegroundColor Red
    exit $LASTEXITCODE
}

$bin = Join-Path $env:APPDATA 'npm\node_modules\wrangler\bin\wrangler.js'
if (-not (Test-Path $bin)) {
    Write-Error "Still missing $bin after install"
    exit 1
}

if (-not (Test-WranglerHealthy)) {
    Write-Host 'bin present but --version still fails. Try allow-scripts for workerd/esbuild:' -ForegroundColor Red
    Write-Host '  npm install -g --allow-scripts=esbuild,workerd,sharp wrangler' -ForegroundColor DarkYellow
    exit 1
}

$v = & wrangler --version 2>&1 | Out-String
Write-Host "Repaired: $($v.Trim())" -ForegroundColor Green
Write-Host "path: $((Get-Command wrangler).Source)"
Write-Host 'FILL_ME deploy: do not wrangler deploy until Claude Code cert pass:true' -ForegroundColor DarkYellow
exit 0
