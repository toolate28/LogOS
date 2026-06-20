# Generate HTML documentation (Windows equivalent of `make html`)
param(
    [string]$Agda = "agda"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

& "$PSScriptRoot\vendor.ps1"

$lib = Join-Path $Root "TriWeavon.agda-lib"
$entry = Join-Path $Root "src\Everything.agda"
$out = Join-Path $Root "docs\generated"
New-Item -ItemType Directory -Force -Path $out | Out-Null

Write-Host "Rendering HTML to $out ..."
& $Agda -l $lib --html --html-dir=$out $entry
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "HTML docs written to $out"