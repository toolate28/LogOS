# Clone agda/cubical and agda-stdlib if missing
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$Vendor = Join-Path $Root "vendor"

function Ensure-Clone($Name, $Url) {
    $dest = Join-Path $Vendor $Name
    if (Test-Path (Join-Path $dest ".git")) {
        Write-Host "vendor/$Name already present"
        return
    }
    Write-Host "Cloning $Url -> vendor/$Name ..."
    git clone --depth 1 $Url $dest
}

Ensure-Clone "agda-cubical" "https://github.com/agda/cubical.git"
Ensure-Clone "agda-stdlib" "https://github.com/agda/agda-stdlib.git"