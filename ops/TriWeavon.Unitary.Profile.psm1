#Requires -Version 5.1
<#
.SYNOPSIS
    Tri-Weavon Unitary Profile — actionable cockpit (not just a vanity board).
.DESCRIPTION
  Sensors + **next actions** + one-shot service starters via WSL.
  Designed for daily use: quiet by default, loud when something is broken.

  Install:  ops/Install-TriWeavonUnitaryProfile.ps1
  Commands: tw                 # compact status + next steps
            tw sensors         # full board
            tw fix             # attempt auto-remediation
            tw up              # start waist+bbbr+styx in WSL
            tw verify          # run vector smokes
            tw help
#>

Set-StrictMode -Version Latest

$script:UnitaryVersion = '1.1.0-unitary'
$script:SensorCache = $null
$script:SensorCacheAt = [datetime]::MinValue
$script:SensorTtlSec = 6
$script:TelemetryDir = $null
$script:WslDistro = $null

# ─── Roots / WSL ─────────────────────────────────────────────────────────────

function Get-UnitaryLogOSRoot {
    [CmdletBinding()]
    param()
    foreach ($c in @(
            $env:LOGOS_ROOT,
            'F:\Users\Matthew Ruhnau\LogOS',
            (Join-Path $HOME 'LogOS')
        )) {
        if ($c -and (Test-Path -LiteralPath $c)) {
            if ($c -like 'C:\*' -and (Test-Path 'F:\Users\Matthew Ruhnau\LogOS')) {
                return (Resolve-Path 'F:\Users\Matthew Ruhnau\LogOS').Path
            }
            return (Resolve-Path -LiteralPath $c).Path
        }
    }
    $wsl = '\\wsl$\Kali\home\toolated\LogOS'
    if (Test-Path $wsl) { return $wsl }
    return $null
}

function Get-UnitaryWslDistro {
    if ($script:WslDistro) { return $script:WslDistro }
    if (-not (Get-Command wsl.exe -ErrorAction SilentlyContinue)) { return $null }
    try {
        # wsl -l emits UTF-16; strip NULs and normalize
        $list = @(
            & wsl.exe -l -q 2>$null |
                ForEach-Object { ($_ -replace "`0", '').Trim() } |
                Where-Object { $_ -and $_ -notmatch '^(Windows Subsystem|docker-desktop)' }
        )
        foreach ($name in @('kali-linux', 'Kali', 'Ubuntu', 'Debian', 'Ubuntu-24.04', 'Ubuntu-22.04')) {
            $hit = $list | Where-Object { $_ -ieq $name } | Select-Object -First 1
            if ($hit) { $script:WslDistro = $hit; return $hit }
        }
        # Prefer any name containing kali
        $kali = $list | Where-Object { $_ -match 'kali' } | Select-Object -First 1
        if ($kali) { $script:WslDistro = $kali; return $kali }
        if ($list) { $script:WslDistro = $list[0]; return $list[0] }
    } catch { }
    return $null
}

function Invoke-UnitaryWsl {
    <#
    .SYNOPSIS
        Run a bash command in the LogOS WSL distro (ext4 ~/LogOS).
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Bash,
        [switch]$Interactive
    )
    $distro = Get-UnitaryWslDistro
    if (-not $distro) { throw 'WSL distro not found (need Kali/Ubuntu for deploy waist).' }
    $cmd = "cd ~/LogOS && $Bash"
    if ($Interactive) {
        & wsl.exe -d $distro -- bash -lc $cmd
    } else {
        & wsl.exe -d $distro -- bash -lc $cmd 2>&1
    }
}

function Initialize-UnitaryEnv {
    [CmdletBinding()]
    param()
    $root = Get-UnitaryLogOSRoot
    if ($root) {
        $env:LOGOS_ROOT = $root
        $env:CTWFI_INVARIANT = 'alpha+omega=15'
        $env:TRIWEAVON_UNITARY = $script:UnitaryVersion
        $script:TelemetryDir = Join-Path $root 'ops\telemetry'
        if (-not (Test-Path $script:TelemetryDir)) {
            New-Item -ItemType Directory -Path $script:TelemetryDir -Force | Out-Null
        }
    }
    $root
}

# ─── Probes ──────────────────────────────────────────────────────────────────

function Test-TcpPortFast {
    param([string]$TargetHost = '127.0.0.1', [int]$Port, [int]$TimeoutMs = 350)
    try {
        $c = [System.Net.Sockets.TcpClient]::new()
        $iar = $c.BeginConnect($TargetHost, $Port, $null, $null)
        if (-not $iar.AsyncWaitHandle.WaitOne($TimeoutMs, $false)) {
            try { $c.Close() } catch { }
            return $false
        }
        $c.EndConnect($iar)
        $ok = $c.Connected
        $c.Close()
        return [bool]$ok
    } catch { return $false }
}

function Invoke-HttpJsonFast {
    param([string]$Uri, [int]$TimeoutSec = 2)
    try {
        $resp = Invoke-WebRequest -Uri $Uri -UseBasicParsing -TimeoutSec $TimeoutSec -ErrorAction Stop
        $json = $null
        try { $json = $resp.Content | ConvertFrom-Json } catch { }
        return @{ ok = ($resp.StatusCode -ge 200 -and $resp.StatusCode -lt 400); status = [int]$resp.StatusCode; json = $json }
    } catch {
        return @{ ok = $false; status = 0; error = $_.Exception.Message }
    }
}

function Get-ShadeBar {
    param([double]$Frac, [int]$Width = 10)
    $Frac = [math]::Max(0, [math]::Min(1, $Frac))
    $n = [int][math]::Round($Frac * $Width)
    return ('█' * $n) + ('░' * ($Width - $n))
}

# ─── Sensor board ────────────────────────────────────────────────────────────

function Get-TriWeavonSensors {
    [CmdletBinding()]
    param([switch]$Force)

    $now = Get-Date
    if (-not $Force -and $script:SensorCache -and
        ($now - $script:SensorCacheAt).TotalSeconds -lt $script:SensorTtlSec) {
        return $script:SensorCache
    }

    $root = Initialize-UnitaryEnv
    $s = [ordered]@{}

    $s.conservation = [pscustomobject]@{
        name = 'conservation'; ok = $true; critical = $true
        detail = 'α=7 ω=8 Σ=15'; fix = $null
        shade = Get-ShadeBar 1.0
    }

    $s.logos_root = [pscustomobject]@{
        name = 'logos_root'; ok = [bool]$root; critical = $true
        detail = if ($root) { $root } else { 'MISSING — set LOGOS_ROOT' }
        fix = if (-not $root) { 'Set LOGOS_ROOT to F:\Users\Matthew Ruhnau\LogOS' } else { $null }
        shade = Get-ShadeBar $(if ($root) { 1 } else { 0 })
    }

    $wsl = Get-UnitaryWslDistro
    $s.wsl = [pscustomobject]@{
        name = 'wsl'; ok = [bool]$wsl; critical = $true
        detail = if ($wsl) { $wsl } else { 'no distro' }
        fix = if (-not $wsl) { 'Install/start WSL Kali (build substrate)' } else { $null }
        shade = Get-ShadeBar $(if ($wsl) { 1 } else { 0 })
    }

    # Waist
    $wTcp = Test-TcpPortFast -Port 8080
    $wHttp = if ($wTcp) { Invoke-HttpJsonFast 'http://127.0.0.1:8080/health' } else { @{ ok = $false; status = 0 } }
    $s.waist = [pscustomobject]@{
        name = 'waist'; ok = [bool]$wHttp.ok; critical = $false
        detail = if ($wHttp.ok) { "http://127.0.0.1:8080  ✓ $($wHttp.status)" } else { 'down — verify/validate API' }
        fix = if (-not $wHttp.ok) { 'tw up waist   # or: wsl -d kali-linux -- bash -lc "cd /mnt/f/Users/Matthew\ Ruhnau/LogOS && docker compose up -d"' } else { $null }
        shade = Get-ShadeBar $(if ($wHttp.ok) { 1 } elseif ($wTcp) { 0.4 } else { 0.05 })
        url = 'http://127.0.0.1:8080/health'
    }

    # BbBR
    $bTcp = Test-TcpPortFast -Port 8081
    $bHttp = if ($bTcp) { Invoke-HttpJsonFast 'http://127.0.0.1:8081/health' } else { @{ ok = $false; status = 0 } }
    $s.bbbr = [pscustomobject]@{
        name = 'bbbr'; ok = [bool]$bHttp.ok; critical = $false
        detail = if ($bHttp.ok) { 'http://127.0.0.1:8081  linkage auditor' } else { 'down — hermetic chain verify' }
        fix = if (-not $bHttp.ok) { 'tw up bbbr    # nix build .#bbbr-verifier && result-bbbr/bin/bbbr-verifier' } else { $null }
        shade = Get-ShadeBar $(if ($bHttp.ok) { 1 } else { 0.05 })
        url = 'http://127.0.0.1:8081/verify'
    }

    # Styx 9P
    $styx = Test-TcpPortFast -Port 5640
    $s.styx = [pscustomobject]@{
        name = 'styx'; ok = $styx; critical = $false
        detail = if ($styx) { '9P2000.L 127.0.0.1:5640' } else { 'down — Bookshelf daemon' }
        fix = if (-not $styx) { 'tw up styx    # cargo run -p styx-vfs-layer --bin styx-bookshelf' } else { $null }
        shade = Get-ShadeBar $(if ($styx) { 0.95 } else { 0.05 })
    }

    # Bridge
    $br = Test-TcpPortFast -Port 8088
    $s.bridge = [pscustomobject]@{
        name = 'bridge'; ok = $br; critical = $false
        detail = if ($br) { 'ws://127.0.0.1:8088' } else { 'down — optional SPHINX WS' }
        fix = if (-not $br) { 'Optional: start reson8 styx WS bridge on :8088' } else { $null }
        shade = Get-ShadeBar $(if ($br) { 0.9 } else { 0.15 })
    }

    # CLIs — prefer WSL for docker/kubectl/nix (host often lacks them)
    $dockerHost = [bool](Get-Command docker -EA SilentlyContinue)
    $dockerWsl = $false
    if (-not $dockerHost -and $wsl) {
        try {
            $out = & wsl.exe -d $wsl -- bash -lc 'docker info >/dev/null 2>&1 && echo OK' 2>$null
            $dockerWsl = ($out -match 'OK')
        } catch { }
    }
    $s.docker = [pscustomobject]@{
        name = 'docker'; ok = ($dockerHost -or $dockerWsl); critical = $false
        detail = if ($dockerHost) { 'host CLI' } elseif ($dockerWsl) { "WSL/$wsl" } else { 'absent' }
        fix = if (-not ($dockerHost -or $dockerWsl)) { 'Start Docker Desktop or dockerd in WSL' } else { $null }
        shade = Get-ShadeBar $(if ($dockerHost -or $dockerWsl) { 0.9 } else { 0.1 })
    }

    $s.cargo = [pscustomobject]@{
        name = 'cargo'; ok = [bool](Get-Command cargo -EA SilentlyContinue); critical = $false
        detail = if (Get-Command cargo -EA SilentlyContinue) { 'host' } else { 'use WSL nix develop .#rust' }
        fix = $null
        shade = Get-ShadeBar $(if (Get-Command cargo -EA SilentlyContinue) { 0.9 } else { 0.3 })
    }

    $schemaOk = $root -and (Test-Path (Join-Path $root 'docs\schemas\v0.1\validate.py'))
    $s.schemas = [pscustomobject]@{
        name = 'schemas'; ok = $schemaOk; critical = $true
        detail = if ($schemaOk) { 'v0.1 filed' } else { 'schemas missing' }
        fix = if (-not $schemaOk) { 'Pull LogOS; ensure docs/schemas/v0.1 exists' } else { $null }
        shade = Get-ShadeBar $(if ($schemaOk) { 1 } else { 0 })
    }

    # WAVE: weight critical sensors higher
    $weights = @{
        conservation = 2; logos_root = 2; wsl = 1.5; schemas = 2
        waist = 1.5; bbbr = 1; styx = 1; bridge = 0.5; docker = 0.8; cargo = 0.5
    }
    $num = 0.0; $den = 0.0
    foreach ($k in $s.Keys) {
        $w = if ($weights.ContainsKey($k)) { $weights[$k] } else { 1.0 }
        $den += $w
        if ($s[$k].ok) { $num += $w }
    }
    $wave = if ($den -gt 0) { [math]::Round(100.0 * $num / $den, 1) } else { 0 }

    $down = @($s.Values | Where-Object { -not $_.ok })
    $actions = @($down | Where-Object { $_.fix } | ForEach-Object { $_.fix } | Select-Object -First 5)

    $board = [pscustomobject]@{
        timestamp       = $now.ToString('o')
        unitary_version = $script:UnitaryVersion
        wave_score      = $wave
        wave_shade      = Get-ShadeBar ($wave / 100.0) 14
        sensors         = $s
        down            = $down
        next_actions    = $actions
        logos_root      = $root
        wsl_distro      = $wsl
        keystone        = 'Hope&&Sauced · The Keystone Holds'
    }

    $script:SensorCache = $board
    $script:SensorCacheAt = $now
    if ($script:TelemetryDir) {
        try {
            $line = (@{ t = $board.timestamp; wave = $wave; down = @($down | ForEach-Object name) } | ConvertTo-Json -Compress)
            Add-Content (Join-Path $script:TelemetryDir 'unitary-sensors.jsonl') $line -Encoding utf8
        } catch { }
    }
    $board
}

function Show-TriWeavonCompact {
    <#
    .SYNOPSIS
        One-screen useful status: WAVE + what's down + exact next commands.
    #>
    [CmdletBinding()]
    param([switch]$Force)
    $b = Get-TriWeavonSensors -Force:$Force
    $wc = if ($b.wave_score -ge 80) { 'Green' } elseif ($b.wave_score -ge 50) { 'Yellow' } else { 'Red' }

    Write-Host ''
    Write-Host ("  TRI-WEAVON  {0} {1,5}%  α+ω=15" -f $b.wave_shade, $b.wave_score) -ForegroundColor $wc

    $up = @($b.sensors.Values | Where-Object ok | ForEach-Object name)
    $dn = @($b.down | ForEach-Object name)
    if ($up) { Write-Host ("  up:   {0}" -f ($up -join ', ')) -ForegroundColor Green }
    if ($dn) { Write-Host ("  down: {0}" -f ($dn -join ', ')) -ForegroundColor Yellow }
    else { Write-Host '  down: (none)' -ForegroundColor DarkGreen }

    if ($b.next_actions -and $b.next_actions.Count -gt 0) {
        Write-Host '  next:' -ForegroundColor Cyan
        $i = 1
        foreach ($a in $b.next_actions) {
            Write-Host ("    {0}. {1}" -f $i, $a) -ForegroundColor White
            $i++
        }
    } else {
        Write-Host '  next: tw verify   # smoke vectors when green' -ForegroundColor DarkGray
    }

    Write-Host '  cmds: tw | tw confidence | tw sensors | tw up | tw fix | tw verify | tw help' -ForegroundColor DarkGray
    Write-Host ''
    $b
}

function Show-TriWeavonSensorBoard {
    [CmdletBinding()]
    param([switch]$Force)
    $b = Get-TriWeavonSensors -Force:$Force
    $wc = if ($b.wave_score -ge 80) { 'Green' } elseif ($b.wave_score -ge 50) { 'Yellow' } else { 'Red' }
    Write-Host ''
    Write-Host '  ╭─ SENSOR BOARD ────────────────────────────────────────────────╮' -ForegroundColor Cyan
    Write-Host ("  │  WAVE {0} {1,5}%" -f $b.wave_shade, $b.wave_score) -ForegroundColor $wc
    Write-Host '  ├────────────────────────────────────────────────────────────────┤' -ForegroundColor Cyan
    foreach ($x in $b.sensors.Values) {
        $m = if ($x.ok) { '●' } else { '○' }
        $c = if ($x.ok) { 'Green' } else { 'Yellow' }
        $line = ('  │  {0} {1,-14} {2}  {3}' -f $m, $x.name, $x.shade, $x.detail)
        if ($line.Length -gt 72) { $line = $line.Substring(0, 69) + '…' }
        Write-Host $line -ForegroundColor $c
    }
    Write-Host '  ╰────────────────────────────────────────────────────────────────╯' -ForegroundColor Cyan
    if ($b.next_actions) {
        Write-Host '  FIX:' -ForegroundColor Cyan
        $b.next_actions | ForEach-Object { Write-Host "    → $_" -ForegroundColor White }
    }
    Write-Host ''
    $b
}

# ─── Service control (the helpful part) ──────────────────────────────────────

function Start-TriWeavonServices {
    <#
    .SYNOPSIS
        Bring up deploy-waist services inside WSL (ext4 ~/LogOS).
    .PARAMETER Service
        waist | bbbr | styx | all
    #>
    [CmdletBinding()]
    param(
        [ValidateSet('waist', 'bbbr', 'styx', 'all')]
        [string]$Service = 'all'
    )
    $distro = Get-UnitaryWslDistro
    if (-not $distro) { Write-Error 'WSL required'; return }

    Write-Host "  Starting via WSL ($distro) ~/LogOS …" -ForegroundColor Cyan

    if ($Service -eq 'waist' -or $Service -eq 'all') {
        Write-Host '  → waist (docker compose)' -ForegroundColor DarkCyan
        Invoke-UnitaryWsl -Bash 'docker compose up -d 2>&1 | tail -5' | ForEach-Object { Write-Host "    $_" }
    }
    if ($Service -eq 'bbbr' -or $Service -eq 'all') {
        Write-Host '  → bbbr-verifier :8081' -ForegroundColor DarkCyan
        $bash = @'
if [ -f /tmp/bbbr.pid ] && kill -0 $(cat /tmp/bbbr.pid) 2>/dev/null; then echo already_up; exit 0; fi
if [ ! -x result-bbbr/bin/bbbr-verifier ]; then
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh 2>/dev/null
  nix build .#bbbr-verifier --out-link result-bbbr 2>&1 | tail -3
fi
nohup ./result-bbbr/bin/bbbr-verifier >/tmp/bbbr.log 2>&1 & echo $! >/tmp/bbbr.pid
sleep 0.5
curl -sf http://127.0.0.1:8081/health && echo OK || (echo FAIL; tail -5 /tmp/bbbr.log)
'@
        Invoke-UnitaryWsl -Bash $bash | ForEach-Object { Write-Host "    $_" }
    }
    if ($Service -eq 'styx' -or $Service -eq 'all') {
        Write-Host '  → styx-bookshelf :5640' -ForegroundColor DarkCyan
        $bash = @'
if [ -f /tmp/styx-bookshelf.pid ] && kill -0 $(cat /tmp/styx-bookshelf.pid) 2>/dev/null; then echo already_up; exit 0; fi
if [ ! -x target/debug/styx-bookshelf ]; then cargo build -p styx-vfs-layer 2>&1 | tail -5; fi
LOGOS_ROOT=$HOME/LogOS nohup ./target/debug/styx-bookshelf >/tmp/styx-bookshelf.log 2>&1 &
echo $! >/tmp/styx-bookshelf.pid
sleep 0.4
ss -ltn | grep 5640 && echo OK || echo FAIL
'@
        Invoke-UnitaryWsl -Bash $bash | ForEach-Object { Write-Host "    $_" }
    }

    Start-Sleep -Milliseconds 600
    $script:SensorCache = $null
    Show-TriWeavonCompact -Force | Out-Null
}

function Repair-TriWeavonStack {
    <#
    .SYNOPSIS
        Auto-remediate: start whatever sensors say is down (non-interactive).
    #>
    [CmdletBinding()]
    param()
    $b = Get-TriWeavonSensors -Force
    $need = @()
    if (-not $b.sensors.waist.ok) { $need += 'waist' }
    if (-not $b.sensors.bbbr.ok) { $need += 'bbbr' }
    if (-not $b.sensors.styx.ok) { $need += 'styx' }
    if (-not $need) {
        Write-Host '  Nothing to fix — stack looks fine. Try: tw verify' -ForegroundColor Green
        return $b
    }
    Write-Host ("  Auto-fix: {0}" -f ($need -join ', ')) -ForegroundColor Yellow
    foreach ($svc in $need) {
        Start-TriWeavonServices -Service $svc
    }
    Get-TriWeavonSensors -Force
}

function Invoke-TriWeavonVerify {
    <#
    .SYNOPSIS
        Run lightweight verify smokes against live services.
    #>
    [CmdletBinding()]
    param()
    $b = Get-TriWeavonSensors -Force
    $pass = 0; $fail = 0
    function Hit($name, $uri) {
        try {
            $r = Invoke-WebRequest -Uri $uri -UseBasicParsing -TimeoutSec 3
            if ($r.StatusCode -ge 200 -and $r.StatusCode -lt 300) {
                Write-Host "  PASS  $name  $uri" -ForegroundColor Green
                $script:pass++
            } else {
                Write-Host "  FAIL  $name  status=$($r.StatusCode)" -ForegroundColor Red
                $script:fail++
            }
        } catch {
            Write-Host "  FAIL  $name  $($_.Exception.Message)" -ForegroundColor Red
            $script:fail++
        }
    }
    $script:pass = 0; $script:fail = 0
    if ($b.sensors.waist.ok) {
        Hit 'waist/health' 'http://127.0.0.1:8080/health'
        Hit 'waist/manifest' 'http://127.0.0.1:8080/manifest'
    } else { Write-Host '  SKIP  waist (down) — tw up waist' -ForegroundColor DarkGray }
    if ($b.sensors.bbbr.ok) {
        Hit 'bbbr/health' 'http://127.0.0.1:8081/health'
        Hit 'bbbr/verify' 'http://127.0.0.1:8081/verify'
    } else { Write-Host '  SKIP  bbbr (down) — tw up bbbr' -ForegroundColor DarkGray }
    if ($b.sensors.styx.ok) {
        Write-Host '  INFO  styx up — full smoke: wsl -d Kali -- bash -lc "cd ~/LogOS && python3 ops/styx-9p-client-smoke.py"' -ForegroundColor DarkCyan
    }
    Write-Host ("  result: {0} pass / {1} fail" -f $script:pass, $script:fail) -ForegroundColor $(if ($script:fail) { 'Yellow' } else { 'Green' })
}

function Test-TriWeavonUnitaryHealth {
    [CmdletBinding()]
    param([double]$MinWave = 45, [switch]$RequireWaist)
    $b = Get-TriWeavonSensors -Force
    if (-not $b.sensors.conservation.ok) { return 2 }
    if (-not $b.sensors.logos_root.ok) { return 2 }
    if ($RequireWaist -and -not $b.sensors.waist.ok) { return 2 }
    if ($b.wave_score -lt $MinWave) { return 1 }
    return 0
}

function Show-TriWeavonHelp {
    Write-Host @'

  TRI-WEAVON UNITARY — useful commands
  ────────────────────────────────────
  tw                 Compact status + next actions (default)
  tw confidence      1-click confidence board (cert · 3 surfaces · needle)
  tw confidence -Refresh   Re-emit cert probe then board
  tw sensors         Full sensor board
  tw up [all|waist|bbbr|styx]   Start services in WSL
  tw fix             Auto-start whatever is down
  tw verify          HTTP smokes against live ports
  tw health          Exit code 0/1/2 (for scripts)
  tw wsl <bash>      Run bash in ~/LogOS on WSL
  tw open            Open health URLs in browser
  tw help            This text

  Pop-out confidence
  ──────────────────
  logos-pop -Command "logos-confidence"
  logos-confidence -Refresh

  Manual verifies
  ───────────────
  curl http://127.0.0.1:8080/health
  curl http://127.0.0.1:8081/verify
  wsl -d Kali -- bash -lc "cd ~/LogOS && python3 ops/styx-9p-client-smoke.py"

  Docs
  ────
  SAIF-Docs/UNITARY-RELEASE-v1.0.md
  ops/GB06-worklog.md   (Cloud Run ⚑)
  ops/LogOS.Confidence.psm1
  ops/claude-code/CERT-PATH.md

'@ -ForegroundColor Cyan
}

function Show-TriWeavonConfidence {
    <#
    .SYNOPSIS
        1-click confidence — thin wrapper over LogOS.Confidence.psm1
    #>
    [CmdletBinding()]
    param([switch]$Refresh, [switch]$Json)
    $root = Initialize-UnitaryEnv
    if (-not $root) { $root = $env:LOGOS_ROOT }
    $mod = Join-Path $root 'ops\LogOS.Confidence.psm1'
    if (-not (Test-Path -LiteralPath $mod)) {
        Write-Error "Missing $mod"
        return
    }
    Import-Module $mod -Force
    Show-LogOSConfidence -Refresh:$Refresh -Json:$Json -LogOSRoot $root
}

function Open-TriWeavonDash {
    $b = Get-TriWeavonSensors
    if ($b.sensors.waist.ok) { Start-Process 'http://127.0.0.1:8080/health' }
    if ($b.sensors.bbbr.ok) { Start-Process 'http://127.0.0.1:8081/verify' }
}

function Invoke-TriWeavonTw {
    <#
    .SYNOPSIS
        Dispatcher: tw [confidence|sensors|up|fix|verify|health|help|wsl|open]
    #>
    [CmdletBinding()]
    param(
        [Parameter(Position = 0)][string]$Command = 'status',
        [Parameter(Position = 1, ValueFromRemainingArguments = $true)]$Rest
    )
    switch -Regex ($Command.ToLower()) {
        '^(s|status)?$' { Show-TriWeavonCompact; break }
        '^(confidence|cert|cc)$' {
            $refresh = $false
            $json = $false
            if ($Rest) {
                foreach ($r in $Rest) {
                    if ("$r" -match '^-?Refresh$') { $refresh = $true }
                    if ("$r" -match '^-?Json$') { $json = $true }
                }
            }
            Show-TriWeavonConfidence -Refresh:$refresh -Json:$json
            break
        }
        '^(sensors|board)$' { Show-TriWeavonSensorBoard -Force; break }
        '^(up|start)$' {
            $svc = if ($Rest -and $Rest[0]) { $Rest[0] } else { 'all' }
            Start-TriWeavonServices -Service $svc
            break
        }
        '^(fix|repair|heal)$' { Repair-TriWeavonStack | Out-Null; break }
        '^(verify|smoke)$' { Invoke-TriWeavonVerify; break }
        '^(health|hc)$' {
            $c = Test-TriWeavonUnitaryHealth
            Write-Host "health_exit=$c  (0=ok 1=degraded 2=fail)"
            return $c
        }
        '^(help|\?)$' { Show-TriWeavonHelp; break }
        '^(wsl)$' {
            $bash = if ($Rest) { ($Rest -join ' ') } else { 'pwd && ls' }
            Invoke-UnitaryWsl -Bash $bash
            break
        }
        '^(open|dash)$' { Open-TriWeavonDash; break }
        default {
            Write-Host "Unknown: $Command — try tw help" -ForegroundColor Yellow
            Show-TriWeavonHelp
        }
    }
}

function Start-TriWeavonUnitary {
    [CmdletBinding()]
    param(
        [switch]$Banner,
        [switch]$Sensors,
        [switch]$Quiet
    )
    $null = Initialize-UnitaryEnv
    $root = $env:LOGOS_ROOT
    if ($root) {
        $shell = Join-Path $root 'ops\LogOS.Shell.psm1'
        $classic = Join-Path $root 'ops\TriWeavon.Profile.psm1'
        $winAxis = Join-Path $root 'ops\LogOS.Windows.psm1'
        if (Test-Path $shell) {
            Import-Module $shell -Force -ErrorAction SilentlyContinue
            Initialize-LogOSShell -ErrorAction SilentlyContinue | Out-Null
        }
        if (Test-Path $classic) {
            Import-Module $classic -Force -ErrorAction SilentlyContinue
            Set-TriWeavonEnv -ErrorAction SilentlyContinue | Out-Null
        }
        if (Test-Path $winAxis) {
            Import-Module $winAxis -Force -ErrorAction SilentlyContinue
        }
        $conf = Join-Path $root 'ops\LogOS.Confidence.psm1'
        if (Test-Path $conf) {
            Import-Module $conf -Force -ErrorAction SilentlyContinue
        }
    }
    if ($Quiet) { return }
    # Default: compact useful status (not a wall of glyphs)
    if ($Sensors) {
        Show-TriWeavonSensorBoard | Out-Null
    } else {
        Show-TriWeavonCompact | Out-Null
    }
}

# Dispatcher aliases
Set-Alias -Name tw -Value Invoke-TriWeavonTw -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-sensors -Value Show-TriWeavonSensorBoard -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-health -Value Test-TriWeavonUnitaryHealth -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-fix -Value Repair-TriWeavonStack -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-up -Value Start-TriWeavonServices -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-verify -Value Invoke-TriWeavonVerify -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-help -Value Show-TriWeavonHelp -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-confidence -Value Show-TriWeavonConfidence -Scope Global -Force -ErrorAction SilentlyContinue

Export-ModuleMember -Function @(
    'Get-UnitaryLogOSRoot',
    'Get-UnitaryWslDistro',
    'Invoke-UnitaryWsl',
    'Initialize-UnitaryEnv',
    'Get-TriWeavonSensors',
    'Show-TriWeavonCompact',
    'Show-TriWeavonSensorBoard',
    'Show-TriWeavonConfidence',
    'Start-TriWeavonServices',
    'Repair-TriWeavonStack',
    'Invoke-TriWeavonVerify',
    'Test-TriWeavonUnitaryHealth',
    'Show-TriWeavonHelp',
    'Open-TriWeavonDash',
    'Invoke-TriWeavonTw',
    'Start-TriWeavonUnitary',
    'Get-ShadeBar',
    'Test-TcpPortFast'
) -Alias @('tw', 'tw-sensors', 'tw-health', 'tw-fix', 'tw-up', 'tw-verify', 'tw-help', 'tw-confidence')
