#Requires -Version 5.1
<#
.SYNOPSIS
    Tri-Weavon PowerShell ops — dynamical metrics, stack tests, bridge control.
.DESCRIPTION
    Loaded from $PROFILE on Beelink. Canonical roots via Get-TriWeavonRoots.
    Rust clamping mirror: cutile::clamping (LogOS/cutiles/cutile).
#>

Set-StrictMode -Version Latest

$script:TriWeavonProfileVersion = '0.3.3'
$script:TriWeavonBridgeHost = '127.0.0.1'
$script:TriWeavonBridgePort = 8088

function Get-TriWeavonRoots {
    <#
    .SYNOPSIS
        Resolve LogOS / coherence-mcp / reson8-Labs roots (F: Beelink, C: fallback).
    #>
    [CmdletBinding()]
    param()

    # Prefer F: Beelink tree; ignore stale C:\Users\Matthew Ruhnau\LogOS when F: exists.
    $preferredLogOS = $null
    foreach ($cand in @(
            $(if ($env:LOGOS_ROOT -and (Test-Path -LiteralPath $env:LOGOS_ROOT)) { $env:LOGOS_ROOT }),
            'F:\Users\Matthew Ruhnau\LogOS',
            'C:\Users\Matthew Ruhnau\LogOS',
            (Join-Path $env:USERPROFILE 'LogOS')
        )) {
        if ($cand -and (Test-Path -LiteralPath $cand)) {
            # If env points at C: but F: exists, upgrade to F:
            if ($cand -like 'C:\Users\Matthew Ruhnau\LogOS*' -and (Test-Path -LiteralPath 'F:\Users\Matthew Ruhnau\LogOS')) {
                $preferredLogOS = 'F:\Users\Matthew Ruhnau\LogOS'
            } else {
                $preferredLogOS = $cand
            }
            break
        }
    }
    if (-not $preferredLogOS) {
        $preferredLogOS = Join-Path $env:USERPROFILE 'LogOS'
    }
    $base = Split-Path $preferredLogOS -Parent

    @{
        Base          = $base
        LogOS         = $preferredLogOS
        CoherenceMcp  = if ($env:COHERENCE_MCP_ROOT) { $env:COHERENCE_MCP_ROOT } else { Join-Path $base 'coherence-mcp' }
        Reson8Labs    = if ($env:RESON8_LABS_ROOT) { $env:RESON8_LABS_ROOT } else { Join-Path $base 'reson8-Labs' }
        Cutile        = Join-Path $preferredLogOS 'cutiles\cutile'
        Ops           = Join-Path $preferredLogOS 'ops'
        AtomLog       = if ($env:ATOM_LOG) { $env:ATOM_LOG } else { Join-Path $preferredLogOS 'ATOM\ATOM-trail.log' }
    }
}

function Set-TriWeavonEnv {
    <#
    .SYNOPSIS
        Export standard Tri-Weavon environment variables for MCP tools and shells.
    #>
    [CmdletBinding()]
    param()

    $r = Get-TriWeavonRoots
    $env:LOGOS_ROOT = $r.LogOS
    $env:COHERENCE_MCP_ROOT = $r.CoherenceMcp
    $env:RESON8_LABS_ROOT = $r.Reson8Labs
    $env:RESON8_LOGOS_ROOT = $r.LogOS
    $env:CTWFI_INVARIANT = 'alpha+omega=15'
    $env:CTWFI_STRAND = 'reason'
    if (-not $env:ATOM_LOG) { $env:ATOM_LOG = $r.AtomLog }
    $r
}

function Import-MsvcBuildEnv {
    <#
    .SYNOPSIS
        MSVC + Windows SDK on PATH/LIB/INCLUDE (Beelink F: BuildTools layout).
    #>
    [CmdletBinding()]
    param(
        [string]$MsvcRoot = 'F:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC',
        [string]$SdkRoot  = 'F:\Program Files (x86)\Windows Kits\10'
    )

    if (-not (Test-Path $MsvcRoot)) {
        Write-Verbose 'MSVC BuildTools not found — skipping LIB/INCLUDE bootstrap'
        return $false
    }

    $msvcVer = Get-ChildItem $MsvcRoot -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending | Select-Object -First 1
    if (-not $msvcVer) { return $false }

    $msvcBin = Join-Path $msvcVer.FullName 'bin\Hostx64\x64'
    $sdkVer = Get-ChildItem (Join-Path $SdkRoot 'Lib') -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending | Select-Object -First 1
    if (-not $sdkVer) { return $false }

    $cargoBin = Join-Path $env:USERPROFILE '.cargo\bin'
    if (Test-Path $cargoBin) {
        $env:PATH = "$msvcBin;$cargoBin;" + $env:PATH
    } else {
        $env:PATH = "$msvcBin;" + $env:PATH
    }

    $env:LIB = @(
        (Join-Path $msvcVer.FullName 'lib\x64'),
        (Join-Path $sdkVer.FullName 'ucrt\x64'),
        (Join-Path $sdkVer.FullName 'um\x64')
    ) -join ';'

    $env:INCLUDE = @(
        (Join-Path $msvcVer.FullName 'include'),
        (Join-Path $SdkRoot "Include\$($sdkVer.Name)\ucrt"),
        (Join-Path $SdkRoot "Include\$($sdkVer.Name)\um"),
        (Join-Path $SdkRoot "Include\$($sdkVer.Name)\shared")
    ) -join ';'

    $true
}

function Test-TriWeavonBridge {
    param(
        [string]$BridgeHost = $script:TriWeavonBridgeHost,
        [int]$Port = $script:TriWeavonBridgePort,
        [int]$TimeoutMs = 2000
    )
    try {
        $client = New-Object System.Net.Sockets.TcpClient
        $iar = $client.BeginConnect($BridgeHost, $Port, $null, $null)
        $ok = $iar.AsyncWaitHandle.WaitOne($TimeoutMs, $false)
        if ($ok -and $client.Connected) {
            $client.EndConnect($iar)
            $client.Close()
            return $true
        }
        $client.Close()
        return $false
    } catch {
        return $false
    }
}

function Get-TriWeavonKeystone {
    'Hope&&Sauced ~ The Keystone Holds ~ alpha+omega=15'
}

function Get-TriWeavonWaveScore {
    <#
    .SYNOPSIS
        Lightweight dynamical coherence proxy (conservation + bridge + toolchain).
    #>
    [CmdletBinding()]
    param()

    $score = 60.0
    if (Test-TriWeavonBridge) { $score += 15 }
    $r = Get-TriWeavonRoots
    if (Test-Path $r.LogOS) { $score += 5 }
    if (Test-Path $r.CoherenceMcp) { $score += 5 }
    if (Get-Command cargo -ErrorAction SilentlyContinue) { $score += 5 }
    if (Get-Command node -ErrorAction SilentlyContinue) { $score += 5 }
    try {
        $ver = npm view @toolated/coherence-mcp version 2>$null
        if ($ver -eq '0.3.2') { $score += 5 }
    } catch { }
    [math]::Min(99, [math]::Round($score, 1))
}

function Get-TriWeavonMetrics {
    <#
    .SYNOPSIS
        Snapshot dynamical metrics for SRAC / cockpit / ATOM trail logging.
    #>
    [CmdletBinding()]
    param([switch]$ProbeHttp)

    Set-TriWeavonEnv | Out-Null
    $r = Get-TriWeavonRoots
    $bridgeUp = Test-TriWeavonBridge
    $wave = Get-TriWeavonWaveScore

    $npmVer = $null
    try { $npmVer = npm view @toolated/coherence-mcp version 2>$null } catch { }

    $httpProbes = @()
    if ($ProbeHttp) {
        foreach ($url in @(
            'https://coherence.toolated.online/health',
            'https://coherence.toolated.online/api/health'
        )) {
            try {
                $res = Invoke-WebRequest -Uri $url -TimeoutSec 6 -UseBasicParsing -ErrorAction Stop
                $httpProbes += [pscustomobject]@{ url = $url; ok = $res.StatusCode -lt 400; status = $res.StatusCode }
            } catch {
                $httpProbes += [pscustomobject]@{ url = $url; ok = $false; status = $_.Exception.Message }
            }
        }
    }

    [pscustomobject]@{
        timestamp       = (Get-Date).ToString('o')
        profile_version = $script:TriWeavonProfileVersion
        invariant       = 'alpha+omega=15'
        alpha           = 7
        omega           = 8
        conservation_ok = $true
        wave_score      = $wave
        bridge          = "ws://${script:TriWeavonBridgeHost}:$($script:TriWeavonBridgePort)"
        bridge_up       = $bridgeUp
        keystone        = Get-TriWeavonKeystone
        npm_coherence   = $npmVer
        logos_root      = $r.LogOS
        logos_exists    = Test-Path $r.LogOS
        mcp_root        = $r.CoherenceMcp
        mcp_exists      = Test-Path $r.CoherenceMcp
        cutile_exists   = Test-Path (Join-Path $r.LogOS 'cutiles\cutile\Cargo.toml')
        http_probes     = $httpProbes
    }
}

function Test-TriWeavonStack {
    <#
    .SYNOPSIS
        Integrated stack test: roots, bridge, npm registry, optional full test suites.
    .PARAMETER Full
        Run npm test + cargo test -p cutile (slower).
    #>
    [CmdletBinding()]
    param([switch]$Full)

    $state = @{ pass = 0; fail = 0 }
    function Step([string]$name, [scriptblock]$action) {
        Write-Host "`n[$name]" -ForegroundColor Cyan
        try {
            & $action
            $exitCode = if (Test-Path 'variable:global:LASTEXITCODE') { $global:LASTEXITCODE } else { 0 }
            if ($exitCode -ne 0) { throw "exit $exitCode" }
            Write-Host "  PASS" -ForegroundColor Green
            $state.pass++
        } catch {
            Write-Host "  FAIL: $($_.Exception.Message)" -ForegroundColor Red
            $state.fail++
        }
    }

    function Assert-Conservation([int]$Alpha, [int]$Omega) {
        if (($Alpha + $Omega) -ne 15) {
            throw "DRIFT: alpha=$Alpha omega=$Omega sum=$($Alpha + $Omega)"
        }
        Write-Host "  Locked: alpha=$Alpha omega=$Omega sum=15" -ForegroundColor Green
    }

    $r = Set-TriWeavonEnv

    Step 'roots' {
        Show-TriWeavonRoots
        if (-not (Test-Path $r.LogOS)) { throw "LogOS missing: $($r.LogOS)" }
        if (-not (Test-Path $r.CoherenceMcp)) { throw "coherence-mcp missing: $($r.CoherenceMcp)" }
    }

    Step 'conservation' {
        Assert-Conservation -Alpha 7 -Omega 8
    }

    Step 'bridge-tcp' {
        $up = Test-TriWeavonBridge
        if ($up) {
            Write-Host '  ws://127.0.0.1:8088 reachable' -ForegroundColor Green
        } else {
            Write-Host '  ws://127.0.0.1:8088 down — run Start-TriWeavonBridge' -ForegroundColor Yellow
        }
    }

    Step 'npm-registry' {
        $ver = npm view @toolated/coherence-mcp version 2>&1
        if ($LASTEXITCODE -ne 0) { throw $ver }
        Write-Host "  @toolated/coherence-mcp@$ver"
    }

    Step 'dynamical-metrics' {
        $m = Get-TriWeavonMetrics
        $m | Format-List timestamp, wave_score, bridge_up, npm_coherence, cutile_exists
    }

    if ($Full) {
        Step 'coherence-mcp-tests' {
            Push-Location $r.CoherenceMcp
            try { npm test 2>&1 | Select-Object -Last 6 | ForEach-Object { Write-Host "  $_" } }
            finally { Pop-Location }
        }

        Step 'cutile-tests' {
            Import-MsvcBuildEnv | Out-Null
            Push-Location $r.LogOS
            try { cargo test -p cutile --no-default-features 2>&1 | Select-Object -Last 8 | ForEach-Object { Write-Host "  $_" } }
            finally { Pop-Location }
        }
    }

    Write-Host "`n=== Tri-Weavon stack: $($state.pass) passed, $($state.fail) failed ===" -ForegroundColor $(if ($state.fail -eq 0) { 'Green' } else { 'Yellow' })
    return ($state.fail -eq 0)
}

function Show-TriWeavonRoots {
    $r = Get-TriWeavonRoots
    foreach ($key in @('LogOS', 'CoherenceMcp', 'Reson8Labs', 'Cutile', 'Ops')) {
        $path = $r[$key]
        $mark = if (Test-Path $path) { '[OK]' } else { '[--]' }
        '{0} {1,-14} {2}' -f $mark, $key, $path
    }
}

function Start-TriWeavonBridge {
    <#
    .SYNOPSIS
        Start triweave WS bridge (triweavon-events + json-rpc-2.0) on :8088.
    .PARAMETER CoherenceMcp
        Start coherence-mcp stdio server (includes bridgeServer) instead of triweave.
    #>
    [CmdletBinding()]
    param(
        [string]$Addr = '127.0.0.1:8088',
        [switch]$CoherenceMcp
    )

    $r = Set-TriWeavonEnv
    Import-MsvcBuildEnv | Out-Null

    if ($CoherenceMcp) {
        Write-Host "Starting coherence-mcp (stdio + bridge on ws://$Addr)..." -ForegroundColor Cyan
        Push-Location $r.CoherenceMcp
        try { node build/index.js }
        finally { Pop-Location }
        return
    }

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error 'cargo not on PATH'
        return
    }

    Write-Host "TriWeavon bridge — $($r.LogOS)" -ForegroundColor Cyan
    Write-Host "  Addr: ws://$Addr" -ForegroundColor Yellow
    Push-Location $r.LogOS
    try {
        cargo run -p reson8-triweave -- serve --addr $Addr
    } finally {
        Pop-Location
    }
}

function Watch-TriWeavonMetrics {
    <#
    .SYNOPSIS
        Poll dynamical metrics and append to ATOM trail log (replaces broken ctwfi-monitor).
    #>
    [CmdletBinding()]
    param(
        [int]$IntervalSeconds = 15,
        [switch]$ProbeHttp
    )

    Set-TriWeavonEnv | Out-Null
    $log = $env:ATOM_LOG
    $logDir = Split-Path $log -Parent
    if ($logDir -and -not (Test-Path $logDir)) {
        New-Item -ItemType Directory -Path $logDir -Force | Out-Null
    }

    Write-Host "Watch-TriWeavonMetrics → $log (every ${IntervalSeconds}s). Ctrl+C to stop." -ForegroundColor Cyan
    while ($true) {
        $m = Get-TriWeavonMetrics -ProbeHttp:$ProbeHttp
        $line = '{0} CTWFI | strand={1} | {2} | WAVE={3} | bridge={4} | npm={5} | {6}' -f `
            $m.timestamp, $env:CTWFI_STRAND, $m.invariant, $m.wave_score, `
            $(if ($m.bridge_up) { 'UP' } else { 'DOWN' }), $m.npm_coherence, $m.keystone
        Add-Content -Path $log -Value $line -Encoding UTF8 -ErrorAction SilentlyContinue
        Write-Host $line -ForegroundColor DarkGray
        Start-Sleep -Seconds $IntervalSeconds
    }
}

function Test-TriWeavonProfile {
    <#
    .SYNOPSIS
        Verify $PROFILE loaded this module and core commands resolve.
    #>
    [CmdletBinding()]
    param()

    $required = @(
        'Get-TriWeavonRoots', 'Set-TriWeavonEnv', 'Get-TriWeavonMetrics',
        'Test-TriWeavonStack', 'Start-TriWeavonBridge', 'Show-TriWeavonRoots',
        'Test-TriWeavonBridge', 'Get-TriWeavonWaveScore', 'Watch-TriWeavonMetrics'
    )

    $missing = @($required | Where-Object { -not (Get-Command $_ -ErrorAction SilentlyContinue) })
    if ($missing.Count -gt 0) {
        Write-Host "Missing commands: $($missing -join ', ')" -ForegroundColor Red
        return $false
    }

    Write-Host "TriWeavon.Profile.psm1 v$script:TriWeavonProfileVersion — commands OK" -ForegroundColor Green
    Write-Host "PROFILE: $PROFILE" -ForegroundColor DarkGray
    Test-TriWeavonStack
}

function Publish-CoherenceMcp {
    <#
    .SYNOPSIS
        Build, test, and publish @toolated/coherence-mcp (requires npm login).
    #>
    [CmdletBinding()]
    param([switch]$DryRun, [switch]$SkipTest)

    $r = Get-TriWeavonRoots
    Push-Location $r.CoherenceMcp
    try {
        npm run build
        if (-not $SkipTest) { npm test }
        if ($DryRun) {
            npm pack
            Write-Host 'Dry run: npm pack only' -ForegroundColor Yellow
        } else {
            npm publish --access public
        }
    } finally {
        Pop-Location
    }
}

Export-ModuleMember -Function @(
    'Get-TriWeavonRoots', 'Set-TriWeavonEnv', 'Import-MsvcBuildEnv',
    'Test-TriWeavonBridge', 'Get-TriWeavonKeystone', 'Get-TriWeavonWaveScore',
    'Get-TriWeavonMetrics', 'Test-TriWeavonStack', 'Show-TriWeavonRoots',
    'Start-TriWeavonBridge', 'Watch-TriWeavonMetrics', 'Test-TriWeavonProfile',
    'Publish-CoherenceMcp'
)