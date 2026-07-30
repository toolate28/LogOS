#Requires -Version 5.1
<#
.SYNOPSIS
    1-click confidence board — endemic LogOS / Claude Code / wrangler honesty.

.DESCRIPTION
    Aggregates three Claude Code surfaces + cert state + LSP/TUI needle into one
    readable board. Never self-certifies full pass. Category B stays amber.

    Commands (after import):
      logos-confidence          # board
      logos-confidence -Refresh # re-run Emit-ClaudeCodeCert -AsInitRun then board
      tw confidence             # via unitary dispatcher (if wired)

    Pop-out:
      logos-pop -Command "Import-Module `$env:LOGOS_ROOT\ops\LogOS.Confidence.psm1 -Force; Show-LogOSConfidence"

.NOTES
    ATOM: ATOM-LOGOS-CONFIDENCE-1CLICK-20260725
    Config is git · cert JSON is state under .atom-trail/certs/
    α+ω=15 is Category C label only — never a confidence threshold.
#>

Set-StrictMode -Version Latest

$script:ConfidenceVersion = '0.1.0'

function Resolve-LogOSConfidenceRoot {
    foreach ($c in @(
            $env:LOGOS_ROOT,
            'F:\Users\Matthew Ruhnau\LogOS',
            (Join-Path $HOME 'LogOS')
        )) {
        if ($c -and (Test-Path -LiteralPath $c)) {
            if ($c -like 'C:\Users\Matthew Ruhnau\LogOS*' -and (Test-Path 'F:\Users\Matthew Ruhnau\LogOS')) {
                return 'F:\Users\Matthew Ruhnau\LogOS'
            }
            return (Resolve-Path -LiteralPath $c).Path
        }
    }
    return $null
}

function Test-LogOSCmd([string]$Name) {
    [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Get-LogOSCertSnapshot {
    [CmdletBinding()]
    param([string]$Root)
    $latest = Join-Path $Root '.atom-trail\certs\claude-code\latest.json'
    if (-not (Test-Path -LiteralPath $latest)) {
        return [pscustomobject]@{
            present    = $false
            path       = $latest
            pass       = $false
            atom       = $null
            emitted_at = $null
            raw        = $null
            detail     = '[CATEGORY B] no latest.json — run Emit-ClaudeCodeCert.ps1'
        }
    }
    try {
        $j = Get-Content -LiteralPath $latest -Raw -Encoding UTF8 | ConvertFrom-Json
        return [pscustomobject]@{
            present    = $true
            path       = $latest
            pass       = [bool]$j.pass
            atom       = $j.atom
            emitted_at = $j.emitted_at
            keys       = [bool]$j.keys_present
            surfaces   = $j.surfaces
            wrangler   = $j.wrangler
            lsp        = $j.lsp
            notes      = $j.notes
            raw        = $j
            detail     = if ($j.pass) { 'pass:true (deploy gate open — still verify TUI)' } else { 'pass:false — deploy must refuse' }
        }
    } catch {
        return [pscustomobject]@{
            present = $true
            path    = $latest
            pass    = $false
            detail  = "cert unreadable: $_"
            raw     = $null
        }
    }
}

function Get-LogOSConfidence {
    <#
    .SYNOPSIS
        Structured confidence object (no false-green pass).
    #>
    [CmdletBinding()]
    param(
        [string]$LogOSRoot,
        [switch]$Refresh
    )

    $root = if ($LogOSRoot) { $LogOSRoot } else { Resolve-LogOSConfidenceRoot }
    if (-not $root) { throw 'LOGOS_ROOT not found' }
    $env:LOGOS_ROOT = $root

    if ($Refresh) {
        $emit = Join-Path $root 'ops\claude-code\Emit-ClaudeCodeCert.ps1'
        if (Test-Path -LiteralPath $emit) {
            & pwsh -NoProfile -File $emit -AsInitRun -SkipTimestampCopy 2>&1 | Out-Null
        }
    }

    $rows = [System.Collections.Generic.List[object]]::new()
    $next = [System.Collections.Generic.List[string]]::new()

    function Add-Row([string]$Id, [string]$Label, [string]$State, [string]$Detail, [string]$Edge = 'grep') {
        # State: green | amber | red | void
        # Edge:  grep = live-probed · assert = scheme/doc only · void = missing connector
        $rows.Add([pscustomobject]@{
                id     = $Id
                label  = $Label
                state  = $State
                detail = $Detail
                edge   = $Edge
            }) | Out-Null
    }

    # ── cert (state) ─────────────────────────────────────────────────────────
    $cert = Get-LogOSCertSnapshot -Root $root
    if (-not $cert.present) {
        Add-Row 'cert' 'Claude Code cert' 'amber' $cert.detail 'grep'
        [void]$next.Add('pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1 -AsInitRun')
    } elseif ($cert.pass) {
        # Still amber until TUI LSP proven — failure-promoter
        Add-Row 'cert' 'Claude Code cert' 'amber' "$($cert.detail) · atom=$($cert.atom) · re-verify Formal pane" 'grep'
        [void]$next.Add('logos-tui   # confirm Formal pane live diags before deploy')
    } else {
        Add-Row 'cert' 'Claude Code cert' 'amber' "$($cert.detail) · atom=$($cert.atom)" 'grep'
    }

    # ── Surface A wrangler ───────────────────────────────────────────────────
    $wCmd = Get-Command wrangler -ErrorAction SilentlyContinue
    $wVer = $null
    $wOk = $false
    if ($wCmd) {
        try {
            $wOut = & wrangler --version 2>&1 | Out-String
            if ($wOut -notmatch 'Cannot find module|MODULE_NOT_FOUND' -and $wOut -match '\d+\.\d+') {
                $wOk = $true
                $wVer = ($wOut -replace '\s', '').Trim()
                if ($wVer.Length -gt 40) { $wVer = $wOut.Trim().Split("`n")[0] }
            }
        } catch { $wOk = $false }
    }
    $wTomls = @(
        (Join-Path $root 'wrangler.toml'),
        (Join-Path $root 'adhealth-meaningseed\wrangler.toml'),
        'F:\Users\Matthew Ruhnau\SpiralSafe\ops\wrangler.toml'
    ) | Where-Object { Test-Path -LiteralPath $_ }
    if ($wOk) {
        Add-Row 'surf_a' 'Surface A · wrangler' 'green' "CLI $wVer · configs=$($wTomls.Count)" 'grep'
    } else {
        Add-Row 'surf_a' 'Surface A · wrangler' 'red' "CLI broken/missing · configs=$($wTomls.Count) · Repair-Wrangler.ps1" 'grep'
        [void]$next.Add('pwsh -File ops/claude-code/Repair-Wrangler.ps1')
    }

    # whoami is assert until grepped this run — probe lightly
    $authState = 'void'
    $authDetail = 'not probed (wrangler whoami needs network/auth)'
    if ($wOk -and $env:LOGOS_CONFIDENCE_WHOAMI -eq '1') {
        try {
            $who = & wrangler whoami 2>&1 | Out-String
            if ($who -match 'not authenticated|Please run') {
                $authState = 'amber'
                $authDetail = 'CLI ok · not logged in (FILL_ME: wrangler login)'
            } elseif ($who -match 'You are logged|Account') {
                $authState = 'green'
                $authDetail = 'authenticated'
            } else {
                $authState = 'amber'
                $authDetail = ($who.Trim() -split "`n")[0]
            }
        } catch {
            $authState = 'amber'
            $authDetail = "whoami failed: $_"
        }
    }
    Add-Row 'cf_auth' 'Cloudflare auth' $authState $authDetail $(if ($env:LOGOS_CONFIDENCE_WHOAMI -eq '1') { 'grep' } else { 'void' })
    if ($authState -eq 'amber' -and $authDetail -match 'not logged') {
        [void]$next.Add('wrangler login   # only when deploy is actually needed')
    }

    # ── Surface B VCS ────────────────────────────────────────────────────────
    Push-Location $root
    try {
        $branch = (git rev-parse --abbrev-ref HEAD 2>$null)
        $dirtyN = @((git status --porcelain 2>$null)).Count
        $head = (git rev-parse --short HEAD 2>$null)
        $okGit = [bool]$branch
        $d = "$branch @$head dirty=$dirtyN"
        Add-Row 'surf_b' 'Surface B · VCS' $(if ($okGit) { 'green' } else { 'red' }) $d 'grep'
    } catch {
        Add-Row 'surf_b' 'Surface B · VCS' 'red' "git probe failed: $_" 'grep'
    } finally { Pop-Location }

    # ── Surface C formal ─────────────────────────────────────────────────────
    $hasLean = Test-Path (Join-Path $root 'lean')
    $hasAgda = Test-Path (Join-Path $root 'agda')
    $leanBin = Test-LogOSCmd 'lean'
    $lakeBin = Test-LogOSCmd 'lake'
    $alsBin = Test-LogOSCmd 'als'
    $agdaBin = Test-LogOSCmd 'agda'
    $tcFile = Join-Path $root 'lean\lean-toolchain'
    $tc = if (Test-Path $tcFile) { (Get-Content $tcFile -Raw).Trim() } else { '?' }
    $leanVer = $null
    if ($leanBin) {
        try { $leanVer = (lean --version 2>&1 | Select-Object -First 1).ToString() } catch { }
    }
    $formalBits = @()
    if ($hasLean) { $formalBits += 'lean/' } else { $formalBits += 'lean/MISSING' }
    if ($hasAgda) { $formalBits += 'agda/' } else { $formalBits += 'agda/MISSING' }
    $formalBits += $(if ($leanBin) { 'lean=OK' } else { 'lean=NO' })
    $formalBits += $(if ($lakeBin) { 'lake=OK' } else { 'lake=NO' })
    $formalBits += $(if ($alsBin) { 'als=OK' } else { 'als=MISSING[B]' })
    $formalBits += $(if ($agdaBin) { 'agda=OK' } else { 'agda=MISSING[B]' })
    $formalBits += "pin=$tc"
    if ($leanVer) { $formalBits += "host=$leanVer" }
    $formalReach = $hasLean -and $hasAgda
    $formalLive = $leanBin -and $lakeBin -and $alsBin -and $agdaBin
    $fState = if ($formalLive) { 'green' } elseif ($formalReach) { 'amber' } else { 'red' }
    Add-Row 'surf_c' 'Surface C · formal' $fState ($formalBits -join ' · ') 'grep'
    if (-not $alsBin -or -not $agdaBin) {
        [void]$next.Add('Install agda + als (or WSL path) — cubical pin GB-01 still open')
    }
    if ($tc -match 'v4\.8' -and $leanVer -match '4\.3') {
        [void]$next.Add('Resolve lean-toolchain v4.8 vs host elan drift before claiming Lean LSP A')
    }

    # ── Eye of the needle ────────────────────────────────────────────────────
    $lspRs = Test-Path (Join-Path $root 'crates\tui\src\lsp.rs')
    $tuiPkg = Test-Path (Join-Path $root 'crates\tui\Cargo.toml')
    if ($lspRs -and $tuiPkg) {
        Add-Row 'needle' 'Eye · reson8-tui Formal' 'amber' 'lsp.rs present · attach/live diags unproven this session [B]' 'grep'
        [void]$next.Add('logos-tui   # or: cargo run -p reson8-tui  · focus Formal with f')
    } else {
        Add-Row 'needle' 'Eye · reson8-tui Formal' 'red' 'crates/tui or lsp.rs missing' 'grep'
    }

    # ── Mark-is-sensor scheme (assert until hit telemetry exists) ─────────────
    $markDoc = Test-Path (Join-Path $root 'docs\ops\MARKERS-SENSORS-v0_1.md')
    if ($markDoc) {
        Add-Row 'markers' 'Mark-is-sensor scheme' 'amber' 'docs present · runtime hit bus not grepped live [assert→B]' 'assert'
    } else {
        Add-Row 'markers' 'Mark-is-sensor scheme' 'void' 'MARKERS-SENSORS doc missing' 'void'
    }

    # ── Strand shell / pop ───────────────────────────────────────────────────
    $popFn = $false
    try {
        Import-Module (Join-Path $root 'ops\LogOS.Windows.psm1') -Force -ErrorAction SilentlyContinue
        $popFn = [bool](Get-Command Open-LogOSConsole -ErrorAction SilentlyContinue)
    } catch { }
    Add-Row 'pop' 'logos-pop console' $(if ($popFn) { 'green' } else { 'amber' }) $(if ($popFn) { 'Open-LogOSConsole available' } else { 'import LogOS.Windows.psm1' }) 'grep'

    # ── Confidence score (operational fraction — NOT α+ω=15) ─────────────────
    $weights = @{ green = 1.0; amber = 0.45; red = 0.0; void = 0.15 }
    $scoreSum = 0.0
    foreach ($r in $rows) { $scoreSum += $weights[$r.state] }
    $confidence = [math]::Round(100.0 * $scoreSum / [math]::Max(1, $rows.Count), 1)

    # Deploy gate: only green if cert.pass AND no red AND formal not amber-only blockers
    $deploy = 'REFUSE'
    $deployWhy = 'default refuse'
    if ($cert.pass -and $wOk -and $formalLive -and $cert.keys -ne $true) {
        $deploy = 'CONDITIONAL'
        $deployWhy = 'cert pass + formal binaries — still need TUI Formal live (not auto-granted)'
    } elseif ($cert.pass) {
        $deploy = 'REFUSE'
        $deployWhy = 'cert claims pass but formal/LSP not fully live — failure-promoter'
    } else {
        $deploy = 'REFUSE'
        $deployWhy = 'cert.pass=false or missing'
    }

    if ($deploy -eq 'REFUSE' -and $next.Count -eq 0) {
        [void]$next.Add('tw confidence -Refresh   # refresh cert + board')
    }

    [pscustomobject]@{
        version       = $script:ConfidenceVersion
        logos_root    = $root
        confidence_pct = $confidence
        deploy_gate   = $deploy
        deploy_why    = $deployWhy
        cert          = $cert
        rows          = @($rows)
        next_actions  = @($next)
        invariant_tag = 'α+ω=15 (Category C label only)'
        timestamp     = (Get-Date).ToUniversalTime().ToString('o')
    }
}

function Show-LogOSConfidence {
    <#
    .SYNOPSIS
        Print 1-click confidence board (endemic dash surface).
    #>
    [CmdletBinding()]
    param(
        [switch]$Refresh,
        [switch]$Json,
        [string]$LogOSRoot
    )

    $c = Get-LogOSConfidence -Refresh:$Refresh -LogOSRoot $LogOSRoot
    if ($Json) {
        $c | ConvertTo-Json -Depth 8
        return $c
    }

    $pctColor = if ($c.confidence_pct -ge 75) { 'Green' } elseif ($c.confidence_pct -ge 45) { 'Yellow' } else { 'Red' }
    $gateColor = if ($c.deploy_gate -eq 'CONDITIONAL') { 'Yellow' } elseif ($c.deploy_gate -eq 'ALLOW') { 'Green' } else { 'DarkYellow' }

    # ASCII-only chrome — Windows consoles often mangle box-drawing / Unicode
    Write-Host ''
    Write-Host '  +-- CONFIDENCE  (1-click · endemic) -----------------------------+' -ForegroundColor Cyan
    Write-Host ("  |  score {0,5}%   deploy={1,-12}  v{2}" -f $c.confidence_pct, $c.deploy_gate, $c.version) -ForegroundColor $pctColor
    Write-Host ("  |  {0}" -f $c.deploy_why) -ForegroundColor $gateColor
    Write-Host ("  |  {0}" -f $c.invariant_tag) -ForegroundColor DarkGray
    Write-Host '  +----------------------------------------------------------------+' -ForegroundColor Cyan

    foreach ($r in $c.rows) {
        $glyph = switch ($r.state) {
            'green' { '[*]' }
            'amber' { '[~]' }
            'red'   { '[ ]' }
            'void'  { '[?]' }
            default { '[?]' }
        }
        $col = switch ($r.state) {
            'green' { 'Green' }
            'amber' { 'DarkYellow' }
            'red'   { 'Red' }
            'void'  { 'DarkMagenta' }
            default { 'Gray' }
        }
        $edge = switch ($r.edge) {
            'grep'   { 'G' }
            'assert' { 'A' }
            'void'   { 'V' }
            default  { '?' }
        }
        $line = '  |  {0} [{1}] {2,-26} {3}' -f $glyph, $edge, $r.label, $r.detail
        if ($line.Length -gt 78) { $line = $line.Substring(0, 75) + '...' }
        Write-Host $line -ForegroundColor $col
    }

    Write-Host '  +----------------------------------------------------------------+' -ForegroundColor Cyan
    Write-Host '  |  legend: [*] green  [~] amber/B  [ ] red  [?] void' -ForegroundColor DarkGray
    Write-Host '  |  edge [G]rep [A]ssert [V]oid · B never paints deploy-green' -ForegroundColor DarkGray
    Write-Host '  |  Claude Code owns pass:true · cascade LB-4' -ForegroundColor DarkGray

    if ($c.next_actions -and $c.next_actions.Count -gt 0) {
        Write-Host '  |  NEXT:' -ForegroundColor Cyan
        $i = 1
        foreach ($a in $c.next_actions) {
            Write-Host ("  |    {0}. {1}" -f $i, $a) -ForegroundColor White
            $i++
        }
    }

    Write-Host '  |  cmds: tw confidence | logos-confidence [-Refresh]' -ForegroundColor DarkGray
    Write-Host '  |        logos-pop -Command logos-confidence | logos-tui (Formal f)' -ForegroundColor DarkGray
    Write-Host '  +----------------------------------------------------------------+' -ForegroundColor Cyan
    Write-Host ''
    $c
}

Set-Alias -Name logos-confidence -Value Show-LogOSConfidence -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-confidence -Value Show-LogOSConfidence -Scope Global -Force -ErrorAction SilentlyContinue

Export-ModuleMember -Function @(
    'Resolve-LogOSConfidenceRoot',
    'Get-LogOSCertSnapshot',
    'Get-LogOSConfidence',
    'Show-LogOSConfidence'
) -Alias @('logos-confidence', 'tw-confidence')
