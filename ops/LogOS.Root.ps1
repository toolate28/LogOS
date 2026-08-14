#Requires -Version 5.1
<#
.SYNOPSIS
  Portable LogOS root resolution — no hard-coded person names or home folders.

.DESCRIPTION
  Order:
    1. $env:LOGOS_ROOT (if valid Cargo.toml tree)
    2. Parent of this script when living under <root>/ops/
    3. $env:USERPROFILE\LogOS · $HOME\LogOS · %SystemDrive%\Users\%USERNAME%\LogOS
    4. Other drive letters: X:\Users\%USERNAME%\LogOS
    5. Prefer non-C: when multiple match (Beelink-style secondary disk)

  WSL callers should use ops/wsl/logos-env.sh (globs /mnt/*/Users/*/LogOS).

  Markers: Cargo.toml required for acceptance.
#>

$script:LogOSReadyDrives = $null

function Get-LogOSReadyDriveLetters {
    # Local ready volumes only. Get-PSDrive / Test-Path on disconnected
    # network maps can hang for minutes.
    if ($null -ne $script:LogOSReadyDrives) { return $script:LogOSReadyDrives }
    $ready = [System.Collections.Generic.HashSet[string]]::new(
        [System.StringComparer]::OrdinalIgnoreCase
    )
    try {
        foreach ($di in [System.IO.DriveInfo]::GetDrives()) {
            if (-not $di.IsReady) { continue }
            if ($di.DriveType -notin @([System.IO.DriveType]::Fixed, [System.IO.DriveType]::Removable)) { continue }
            $letter = $di.Name.TrimEnd('\').TrimEnd(':')
            if ($letter.Length -eq 1) { [void]$ready.Add($letter) }
        }
    } catch { }
    $script:LogOSReadyDrives = $ready
    return $ready
}

function Test-LogOSRootCandidate {
    [CmdletBinding()]
    param([string]$Path)
    if (-not $Path) { return $false }
    if ($Path -match '^([A-Za-z]):\\') {
        $ready = Get-LogOSReadyDriveLetters
        if (-not $ready.Contains($Matches[1])) { return $false }
    }
    if (-not (Test-Path -LiteralPath $Path -PathType Container -ErrorAction SilentlyContinue)) { return $false }
    return (Test-Path -LiteralPath (Join-Path $Path 'Cargo.toml') -ErrorAction SilentlyContinue)
}

function Get-LogOSPortableCandidates {
    [CmdletBinding()]
    param(
        # Optional: directory of the calling module/script (usually $PSScriptRoot)
        [string]$ScriptRoot
    )
    $list = [System.Collections.Generic.List[string]]::new()
    $add = {
        param($p)
        if ($p -and -not $list.Contains($p)) { [void]$list.Add($p) }
    }

    & $add $env:LOGOS_ROOT

    $opsDir = $null
    if ($ScriptRoot -and (Test-Path -LiteralPath $ScriptRoot)) {
        $opsDir = $ScriptRoot
    } elseif ($PSScriptRoot) {
        $opsDir = $PSScriptRoot
    }
    if ($opsDir) {
        # Walk up from the calling script (ops/, ops/wsl/, ops/claude-code/, ops/shell/)
        $walk = $opsDir
        for ($i = 0; $i -lt 6 -and $walk; $i++) {
            if (Test-LogOSRootCandidate $walk) {
                & $add $walk
                break
            }
            $parent = Split-Path -Parent $walk
            if (-not $parent -or $parent -eq $walk) { break }
            $walk = $parent
        }
    }

    if ($env:USERPROFILE) { & $add (Join-Path $env:USERPROFILE 'LogOS') }
    if ($HOME) { & $add (Join-Path $HOME 'LogOS') }

    $user = $env:USERNAME
    if (-not $user) { $user = $env:USER }
    $sys = $env:SystemDrive
    if ($user -and $sys) {
        & $add (Join-Path $sys "Users\$user\LogOS")
    }
    $ready = Get-LogOSReadyDriveLetters
    if ($user) {
        foreach ($letter in @('F', 'D', 'G', 'E', 'H')) {
            if ($ready.Contains($letter)) {
                & $add "${letter}:\Users\$user\LogOS"
            }
        }
    }

    # Optional lab roots without personal names (only if G: is live)
    if ($ready.Contains('G')) {
        & $add 'G:\Reson8-Labs\LogOS'
        & $add 'G:\LogOS'
    }

    return @($list)
}

function Resolve-LogOSRootPortable {
    <#
    .SYNOPSIS
      Resolve LogOS monorepo root without embedding a named home directory.
    #>
    [CmdletBinding()]
    param(
        [string]$ScriptRoot,
        [switch]$ThrowIfMissing
    )
    $valid = [System.Collections.Generic.List[string]]::new()
    $candidates = @(Get-LogOSPortableCandidates -ScriptRoot $ScriptRoot)
    # Calling tree (walk-up) is first non-env candidate — accept it immediately
    # so we never scan hung volumes when already inside a valid clone.
    foreach ($c in $candidates) {
        if ($env:LOGOS_ROOT -and $c -eq $env:LOGOS_ROOT) { continue }
        if (Test-LogOSRootCandidate $c) {
            try { return (Resolve-Path -LiteralPath $c).Path } catch { }
        }
        break
    }
    foreach ($c in $candidates) {
        if (Test-LogOSRootCandidate $c) {
            try {
                $resolved = (Resolve-Path -LiteralPath $c).Path
                if (-not $valid.Contains($resolved)) { [void]$valid.Add($resolved) }
            } catch { }
        }
    }

    if ($valid.Count -eq 0) {
        if ($ThrowIfMissing) {
            throw 'LogOS root not found. Set LOGOS_ROOT to your clone, or place the repo at %USERPROFILE%\LogOS'
        }
        return $null
    }

    # Prefer non-C: when several valid trees exist (secondary disk / Beelink pattern)
    $nonC = $valid | Where-Object { $_ -notmatch '^[Cc]:\\' } | Select-Object -First 1
    if ($nonC) { return $nonC }
    return $valid[0]
}
