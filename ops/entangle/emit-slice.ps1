<#
.SYNOPSIS
  Emit a path-slice packet for an entangle component (local → remote payload).

.DESCRIPTION
  Reads ops/entangle/manifest.yaml, packs listed paths into a zip under
  ops/entangle/out/<id>-<stamp>.zip excluding target/, .git, binaries.
  Also writes a sidecar JSON receipt for Actions ingest.

.PARAMETER Id
  Component id from the manifest (e.g. reson8-tui, barcode-tui).

.PARAMETER OutDir
  Output directory (default ops/entangle/out).

.EXAMPLE
  pwsh -File ops/entangle/emit-slice.ps1 -Id reson8-tui
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Id,
    [string]$OutDir = ""
)

$ErrorActionPreference = 'Stop'
$Root = git rev-parse --show-toplevel 2>$null
if (-not $Root) { throw 'not inside a git worktree' }
Set-Location $Root

$manifestPath = Join-Path $Root 'ops/entangle/manifest.yaml'
if (-not (Test-Path $manifestPath)) { throw "missing $manifestPath" }

# Minimal YAML id/paths parse (no PyYAML required)
$raw = Get-Content $manifestPath -Raw
if ($raw -notmatch "(?ms)-\s+id:\s*$([regex]::Escape($Id))\b") {
    throw "component id '$Id' not found in manifest"
}

# Extract block for this id until next "  - id:" or EOF
$m = [regex]::Match($raw, "(?ms)-\s+id:\s*$([regex]::Escape($Id))\b.*?(?=\n  - id:|\z)")
if (-not $m.Success) { throw "failed to parse component block for $Id" }
$block = $m.Value

$paths = @()
foreach ($line in ($block -split "`n")) {
    if ($line -match '^\s+-\s+([A-Za-z0-9_./-]+/?)\s*$') {
        $p = $Matches[1]
        # skip verify bullets that look like commands
        if ($p -match '^(cargo|python|bash|pwsh)\b') { continue }
        if ($p -match '^(crates|apps|docs|ops|lean|tools|cutiles|\.github|coherence-mcp)/') {
            $paths += $p.TrimEnd('/')
        }
    }
}
# Prefer explicit paths: section only
$pathSection = [regex]::Match($block, '(?ms)paths:\s*\n((?:\s+-\s+.+\n)+)')
if ($pathSection.Success) {
    $paths = @()
    foreach ($line in ($pathSection.Groups[1].Value -split "`n")) {
        if ($line -match '^\s+-\s+(.+?)\s*$') {
            $paths += $Matches[1].Trim().TrimEnd('/')
        }
    }
}

if ($paths.Count -eq 0) { throw "no paths resolved for $Id" }

if (-not $OutDir) { $OutDir = Join-Path $Root 'ops/entangle/out' }
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$stamp = Get-Date -Format 'yyyyMMddTHHmmssZ'
$stage = Join-Path $env:TEMP "logos-entangle-$Id-$stamp"
if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
New-Item -ItemType Directory -Force -Path $stage | Out-Null

$excludeDirs = @('target', 'crates\target', '.git', 'node_modules', '.lake', '__pycache__')
$excludeExt = @('.pdb', '.rlib', '.rmeta', '.exe', '.dll', '.so', '.dylib')

function Copy-Filtered {
    param([string]$Rel)
    $src = Join-Path $Root $Rel
    if (-not (Test-Path $src)) {
        Write-Warning "missing path (skip): $Rel"
        return
    }
    if (Test-Path $src -PathType Leaf) {
        $dest = Join-Path $stage $Rel
        New-Item -ItemType Directory -Force -Path (Split-Path $dest) | Out-Null
        Copy-Item $src $dest -Force
        return
    }
    Get-ChildItem -Path $src -Recurse -File -Force | ForEach-Object {
        $full = $_.FullName
        $relFromRoot = $full.Substring($Root.Length).TrimStart('\', '/')
        foreach ($d in $excludeDirs) {
            if ($relFromRoot -match [regex]::Escape($d.Replace('\', '/')) -or
                $relFromRoot -match [regex]::Escape($d.Replace('/', '\'))) {
                return
            }
        }
        if ($excludeExt -contains $_.Extension.ToLowerInvariant()) { return }
        # skip huge binaries > 8 MiB unless under docs/
        if ($_.Length -gt 8MB -and $relFromRoot -notmatch '^docs[\\/]') {
            Write-Warning "skip large file ($([math]::Round($_.Length/1MB,1)) MB): $relFromRoot"
            return
        }
        $dest = Join-Path $stage $relFromRoot
        New-Item -ItemType Directory -Force -Path (Split-Path $dest) | Out-Null
        Copy-Item $full $dest -Force
    }
}

foreach ($p in $paths) { Copy-Filtered $p }

$zip = Join-Path $OutDir "$Id-$stamp.zip"
if (Test-Path $zip) { Remove-Item -Force $zip }
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force

$receipt = @{
    atom      = 'ATOM-ENTANGLE-MANIFEST-20260809'
    id        = $Id
    stamp     = $stamp
    paths     = $paths
    zip       = $zip
    head      = (git rev-parse HEAD)
    branch    = (git rev-parse --abbrev-ref HEAD)
    generated = (Get-Date).ToString('o')
} | ConvertTo-Json -Depth 4
$receiptPath = Join-Path $OutDir "$Id-$stamp.receipt.json"
Set-Content -Path $receiptPath -Value $receipt -Encoding utf8

Remove-Item -Recurse -Force $stage
Write-Host "emit-slice OK id=$Id"
Write-Host "  zip:     $zip"
Write-Host "  receipt: $receiptPath"
Write-Host "Next: upload zip as workflow_dispatch artifact or attach to entangle/$Id PR"
