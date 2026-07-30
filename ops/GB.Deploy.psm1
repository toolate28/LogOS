#Requires -Version 5.1
<#
.SYNOPSIS
    PowerShell wrappers for ops/gb0x-*.sh (WSL-first; host gcloud when available).
.DESCRIPTION
    GB-05/06 scripts are Nix/Docker/kind heavy and run in WSL ~/LogOS by design.
    This module provides HITL-gated entry points from Windows without re-implementing
    the full kind pipeline.

    gb05-smoke / gb05-finish → WSL only
    gb06-deploy            → WSL or host gcloud (-Force required)
#>

Set-StrictMode -Version Latest

function Get-GBWslDistro {
    if (-not (Get-Command wsl.exe -ErrorAction SilentlyContinue)) { return $null }
    $list = & wsl.exe -l -q 2>$null | ForEach-Object { $_.ToString().Trim([char]0).Trim() } | Where-Object { $_ }
    foreach ($n in @('Kali', 'kali-linux', 'Ubuntu', 'Debian')) {
        if ($list -contains $n) { return $n }
    }
    if ($list) { return $list[0] }
    $null
}

function Invoke-GBWslScript {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$ScriptRel,
        [hashtable]$Env = @{}
    )
    $distro = Get-GBWslDistro
    if (-not $distro) { throw 'WSL required for GB-05 scripts (kind/docker/nix on ext4 ~/LogOS)' }
    $exports = ($Env.GetEnumerator() | ForEach-Object { "export $($_.Key)='$($_.Value)'" }) -join '; '
    $bash = "cd ~/LogOS && $exports && bash $ScriptRel"
    Write-Host "WSL/$distro → $ScriptRel" -ForegroundColor Cyan
    & wsl.exe -d $distro -- bash -lc $bash
}

function Invoke-GB05Smoke {
    [CmdletBinding()]
    param([switch]$Force)
    if (-not $Force) {
        Write-Host 'HITL: GB-05 smoke mutates compose/kind. Re-run: Invoke-GB05Smoke -Force' -ForegroundColor Yellow
        return
    }
    Invoke-GBWslScript -ScriptRel 'ops/gb05-smoke.sh'
}

function Invoke-GB05Finish {
    [CmdletBinding()]
    param([switch]$Force)
    if (-not $Force) {
        Write-Host 'HITL: GB-05 finish pins digests + kind. Re-run: Invoke-GB05Finish -Force' -ForegroundColor Yellow
        return
    }
    Invoke-GBWslScript -ScriptRel 'ops/gb05-finish.sh'
}

function Invoke-GB06DeployFromModule {
    [CmdletBinding()]
    param(
        [string]$Project = $env:GOOGLE_CLOUD_PROJECT,
        [switch]$Force
    )
    $win = Join-Path (if ($env:LOGOS_ROOT) { $env:LOGOS_ROOT } else { 'F:\Users\Matthew Ruhnau\LogOS' }) 'ops\LogOS.Windows.psm1'
    if (Test-Path $win) {
        Import-Module $win -Force
        Invoke-GB06Deploy -Project $Project -Force:$Force -UseWsl
    } else {
        if (-not $Force) {
            Write-Host 'HITL: require -Force' -ForegroundColor Yellow
            return
        }
        $envMap = @{}
        if ($Project) { $envMap['PROJECT'] = $Project }
        Invoke-GBWslScript -ScriptRel 'ops/gb06-deploy.sh' -Env $envMap
    }
}

Export-ModuleMember -Function @(
    'Get-GBWslDistro',
    'Invoke-GBWslScript',
    'Invoke-GB05Smoke',
    'Invoke-GB05Finish',
    'Invoke-GB06DeployFromModule'
)
