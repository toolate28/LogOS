#Requires -Version 5.1
<#
.SYNOPSIS
    Emit Claude Code cert JSON to the state path (keyless).

.DESCRIPTION
    Writes:
      .atom-trail/certs/claude-code/latest.json
      .atom-trail/certs/claude-code/YYYYMMDD-HHMMSS.json  (optional copy)

    Config is git (this script + schema). Cert files are state/backup (gitignored).
    Does NOT set pass:true unless -AsInitRun and all three surfaces probe ok.
    Never stores keys. Deploy must refuse pass:false or keys_present:true.

.EXAMPLE
    pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1
    pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1 -AsInitRun
    pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess)]
param(
    [string]$LogOSRoot = $(if ($env:LOGOS_ROOT) { $env:LOGOS_ROOT } else { 'F:\Users\Matthew Ruhnau\LogOS' }),
    # When set, run live probes and allow pass:true if all ok. Without it, honest B placeholder.
    [switch]$AsInitRun,
    [switch]$SkipTimestampCopy
)

$ErrorActionPreference = 'Continue'
Set-StrictMode -Version Latest

function Test-Cmd([string]$Name) {
    [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Probe-Wrangler {
    $result = [ordered]@{
        ok      = $false
        version = $null
        path    = $null
        detail  = ''
    }
    $cmd = Get-Command wrangler -ErrorAction SilentlyContinue
    if (-not $cmd) {
        $result.detail = 'wrangler not on PATH'
        return [pscustomobject]$result
    }
    $result.path = $cmd.Source
    $binJs = Join-Path (Split-Path $cmd.Source -Parent) 'node_modules\wrangler\bin\wrangler.js'
    if (-not (Test-Path -LiteralPath $binJs)) {
        # Also try Roaming npm layout
        $alt = Join-Path $env:APPDATA 'npm\node_modules\wrangler\bin\wrangler.js'
        if (-not (Test-Path -LiteralPath $alt)) {
            $result.detail = "stub present but bin/wrangler.js missing (corrupt install). Repair: npm install -g wrangler"
            return [pscustomobject]$result
        }
    }
    try {
        $verOut = & wrangler --version 2>&1 | Out-String
        if ($LASTEXITCODE -ne 0 -and $verOut -match 'Cannot find module|MODULE_NOT_FOUND') {
            $result.detail = "wrangler broken: $verOut"
            return [pscustomobject]$result
        }
        $result.version = ($verOut -replace '\s', '').Trim()
        if (-not $result.version) { $result.version = $verOut.Trim() }
        $result.ok = $true
        $result.detail = "wrangler $($result.version)"
    } catch {
        $result.detail = "wrangler --version threw: $_"
    }
    return [pscustomobject]$result
}

function Probe-SpiralSafeWrangler([string]$Root) {
    $candidates = @(
        'F:\Users\Matthew Ruhnau\SpiralSafe\ops\wrangler.toml',
        (Join-Path $Root 'wrangler.toml'),
        (Join-Path $Root 'adhealth-meaningseed\wrangler.toml')
    )
    $found = @()
    foreach ($c in $candidates) {
        if (Test-Path -LiteralPath $c) { $found += $c }
    }
    $w = Probe-Wrangler
    $ok = ($found.Count -gt 0) -and $w.ok
    $detail = if ($found.Count -eq 0) {
        'no wrangler.toml candidates found'
    } elseif (-not $w.ok) {
        "configs present ($($found.Count)) but CLI: $($w.detail)"
    } else {
        "configs=$($found.Count); $($w.detail)"
    }
    [pscustomobject]@{
        ok      = $ok
        detail  = $detail
        wrangler = $w
        configs = $found
    }
}

function Probe-Vcs([string]$Root) {
    Push-Location $Root
    try {
        $sb = git status -sb 2>&1 | Out-String
        $remote = git remote -v 2>&1 | Out-String
        $ok = $LASTEXITCODE -eq 0 -or ($sb -match '##')
        # Presence of git is enough for surface reachability; dirty is noted not failed
        $dirty = $sb -match '\n [MADRCU?]|\?\?'
        $detail = ($sb.Trim() -split "`n")[0]
        if ($dirty) { $detail += ' (dirty worktree — not a fail)' }
        if ($remote -notmatch 'github.com') { $detail += ' remotes present' }
        [pscustomobject]@{ ok = [bool]$ok; detail = $detail.Trim() }
    } catch {
        [pscustomobject]@{ ok = $false; detail = "git probe failed: $_" }
    } finally {
        Pop-Location
    }
}

function Probe-Formal([string]$Root) {
    $leanTc = Join-Path $Root 'lean\lean-toolchain'
    $agda = Join-Path $Root 'agda'
    $hasLeanDir = Test-Path (Join-Path $Root 'lean')
    $hasAgdaDir = Test-Path $agda
    $leanCmd = Test-Cmd 'lean'
    $lakeCmd = Test-Cmd 'lake'
    $alsCmd = Test-Cmd 'als'
    $agdaCmd = Test-Cmd 'agda'

    $notes = @()
    if ($hasLeanDir) { $notes += 'lean/ present' } else { $notes += 'lean/ missing' }
    if ($hasAgdaDir) { $notes += 'agda/ present' } else { $notes += 'agda/ missing' }
    if ($leanCmd) {
        try { $notes += "lean=$(lean --version 2>&1 | Select-Object -First 1)" } catch { $notes += 'lean on PATH' }
    } else { $notes += 'lean binary missing' }
    if ($lakeCmd) { $notes += 'lake on PATH' } else { $notes += 'lake missing' }
    if ($alsCmd) { $notes += 'als on PATH' } else { $notes += 'als MISSING [B]' }
    if ($agdaCmd) { $notes += 'agda on PATH' } else { $notes += 'agda MISSING [B]' }
    if (Test-Path $leanTc) {
        $pin = (Get-Content $leanTc -Raw).Trim()
        $notes += "toolchain_file=$pin"
    }

    # Surface "reachable" if sources exist; live LSP not required for path existence
    $ok = $hasLeanDir -and $hasAgdaDir
    [pscustomobject]@{
        ok            = $ok
        detail        = ($notes -join '; ')
        lean_attached = $false  # not attaching LSP from this script
        als_attached  = $false
    }
}

# ── main ────────────────────────────────────────────────────────────────────
if (-not (Test-Path -LiteralPath $LogOSRoot)) {
    throw "LogOSRoot not found: $LogOSRoot"
}

$certDir = Join-Path $LogOSRoot '.atom-trail\certs\claude-code'
if (-not (Test-Path $certDir)) {
    New-Item -ItemType Directory -Path $certDir -Force | Out-Null
}

$now = Get-Date
$stamp = $now.ToString('yyyyMMdd-HHmmss')
$atom = if ($AsInitRun) { "ATOM-CC-CERT-$stamp" } else { 'ATOM-CC-CERT-PLACEHOLDER' }

if ($AsInitRun) {
    $surfA = Probe-SpiralSafeWrangler $LogOSRoot
    $surfB = Probe-Vcs $LogOSRoot
    $surfC = Probe-Formal $LogOSRoot
    $wDetail = $surfA.wrangler
    # Reachability of three surfaces is necessary but not sufficient for pass.
    # Failure-promoter: no false-green — LSP not attached + als missing = B, not pass.
    $surfacesReachable = [bool]($surfA.ok -and $surfB.ok -and $surfC.ok)
    $noFalseGreen = -not (
        ($surfC.detail -match 'als MISSING') -or
        ($surfC.detail -match 'agda MISSING') -or
        ($surfC.detail -match 'lean binary missing')
    )
    # Emit script never attaches LSP; full pass requires Claude Code + TUI live diags.
    # -ForceFullPass is intentionally absent: only Claude Code may claim LSP after verify.
    $pass = $false
    $notes = @(
        'Emitted by Emit-ClaudeCodeCert.ps1 -AsInitRun',
        "surfaces_reachable=$surfacesReachable wrangler.ok=$($wDetail.ok)",
        'pass stays false until Claude Code cold-start confirms no false-green formal/LSP',
        'als/agda binaries and TUI LSP attach are Category B until proven live',
        'α+ω=15 is Category C label only',
        'pass:false — deploy must refuse (this script does not self-certify full pass)'
    )
    if ($surfacesReachable -and $wDetail.ok -and $noFalseGreen) {
        $notes = @(
            'Emitted by Emit-ClaudeCodeCert.ps1 -AsInitRun',
            'surfaces_reachable=true and formal binaries present',
            'pass still false: LSP attach must be confirmed by Claude Code / TUI Formal pane',
            'α+ω=15 is Category C label only',
            'pass:false — deploy must refuse until Claude Code sets pass after live verify'
        )
    }
} else {
    $wDetail = Probe-Wrangler
    $surfA = [pscustomobject]@{
        ok     = $false
        detail = "[CATEGORY B: PLANNED, NOT BUILT] awaiting Claude Code init (wrangler probe: $($wDetail.detail))"
        wrangler = $wDetail
    }
    $surfB = [pscustomobject]@{
        ok     = $false
        detail = '[CATEGORY B: PLANNED, NOT BUILT] awaiting Claude Code init'
    }
    $surfC = [pscustomobject]@{
        ok            = $false
        detail        = '[CATEGORY B: PLANNED, NOT BUILT] awaiting Claude Code init'
        lean_attached = $false
        als_attached  = $false
    }
    $pass = $false
    $notes = @(
        'Scaffold / preflight emit — not a pass. Deploy must refuse.',
        'Run with -AsInitRun after Claude Code cold-start survey.'
    )
}

# Tree-state binding: cert describes THIS head; countersign parent must match (D6).
$headSha = 'UNKNOWN'
$noteList = [System.Collections.Generic.List[string]]::new()
foreach ($n in @($notes)) { [void]$noteList.Add([string]$n) }
Push-Location $LogOSRoot
try {
    $resolved = git rev-parse HEAD 2>$null
    if ($resolved) { $headSha = [string]$resolved }
    $sbLine = (git status -sb 2>$null | Select-Object -First 1)
    if ($sbLine) { [void]$noteList.Add("vcs_status=$sbLine") }
} finally {
    Pop-Location
}
[void]$noteList.Add("head_sha=$headSha — copy to Mark-Cert-Head on countersign; parent of countersign commit must equal this")
[void]$noteList.Add('category A/B/C/D is epistemic kind; verification is build-asserted|countersigned — countersign never promotes C→A')

$cert = [ordered]@{
    atom         = $atom
    init_packet  = 'CLAUDECODE-INIT-v0_1'
    pass         = $pass
    wave_label   = 'Category C tag only; not a gate number'
    head_sha     = [string]$headSha
    mark_id      = $null
    surfaces     = [ordered]@{
        spiralsafe_wrangler = [ordered]@{ ok = [bool]$surfA.ok; detail = [string]$surfA.detail }
        vcs                 = [ordered]@{ ok = [bool]$surfB.ok; detail = [string]$surfB.detail }
        formal_core         = [ordered]@{ ok = [bool]$surfC.ok; detail = [string]$surfC.detail }
    }
    tui_needle   = 'crates/tui (reson8-forge)'
    lsp          = [ordered]@{
        lean_attached = [bool]$surfC.lean_attached
        als_attached  = [bool]$surfC.als_attached
        note          = if ($pass) {
            'LSP attach not claimed by emit script — TUI must show live diags'
        } else {
            '[CATEGORY B: PLANNED, NOT BUILT] until servers publish to TUI'
        }
    }
    keys_present = $false
    wrangler     = [ordered]@{
        ok      = [bool]$wDetail.ok
        version = $wDetail.version
        path    = $wDetail.path
        detail  = [string]$wDetail.detail
    }
    emitted_at   = $now.ToUniversalTime().ToString('o')
    schema_ref   = 'ops/claude-code/cert.schema.json'
    cert_path    = '.atom-trail/certs/claude-code/latest.json'
    notes        = @($noteList.ToArray())
}

$json = $cert | ConvertTo-Json -Depth 8
$latest = Join-Path $certDir 'latest.json'

if ($PSCmdlet.ShouldProcess($latest, 'Write Claude Code cert')) {
    Set-Content -LiteralPath $latest -Value $json -Encoding UTF8
    if (-not $SkipTimestampCopy) {
        $copy = Join-Path $certDir "$stamp.json"
        Set-Content -LiteralPath $copy -Value $json -Encoding UTF8
    }
    # Keep local README present
    $readme = Join-Path $certDir 'README.md'
    if (-not (Test-Path $readme)) {
        @"
# Claude Code cert emit directory

**State / backup — not secrets. No keys.**

- ``latest.json`` — written by Claude Code cold-start or ``ops/claude-code/Emit-ClaudeCodeCert.ps1``
- Timestamped copies optional
- Schema (git): ``ops/claude-code/cert.schema.json``
- Path doc (git): ``ops/claude-code/CERT-PATH.md``

Until first real init with ``pass:true``, treat as **B**.
"@ | Set-Content -LiteralPath $readme -Encoding UTF8
    }
}

Write-Host "cert → $latest" -ForegroundColor Cyan
Write-Host "pass=$pass  keys_present=false  wrangler.ok=$($wDetail.ok)  wrangler=$($wDetail.version)" -ForegroundColor $(if ($pass) { 'Green' } else { 'DarkYellow' })
if (-not $pass) {
    Write-Host '[CATEGORY B] Deploy must refuse.' -ForegroundColor DarkYellow
}

# Return object for pipelines
[pscustomobject]@{
    Path   = $latest
    Pass   = $pass
    Cert   = $cert
}
