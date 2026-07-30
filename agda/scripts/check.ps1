# Typecheck TriWeavon formal modules (Windows equivalent of `make check`)
param(
    [string]$Agda = "agda"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

& "$PSScriptRoot\vendor.ps1"

$lib = Join-Path $Root "TriWeavon.agda-lib"
$entry = Join-Path $Root "src\Everything.agda"

Write-Host "Checking $entry with library $lib ..."
& $Agda -l $lib $entry
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "TriWeavon formal layer: OK"