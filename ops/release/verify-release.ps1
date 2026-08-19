#requires -Version 7.0
<#
.SYNOPSIS
  LogOS lattice release verifier with graceful error → remediation hints.

.DESCRIPTION
  Runs Priority-A smokes for v0.3.0 surfaces. On failure prints a remediation
  command (see docs/ops/OPERATOR-JOURNEY-AND-RECOVERY-20260809.md) and continues
  so the operator gets a full board, not a single hard stop mid-loop.

  Exit codes:
    0 = all required checks passed
    1 = one or more required checks failed (amber/red board printed)
    2 = environment/setup fault (wrong root, missing python/cargo)

.PARAMETER Remediate
  Attempt safe auto-remediation only (never force-push, never cloud deploy).

.PARAMETER SkipCargo
  Skip rust tests (docs/tools only) — useful when iterating docs.

.PARAMETER Quick
  claim_lint help + entangle validate + cutile claim_gate only.
#>
[CmdletBinding()]
param(
    [switch]$Remediate,
    [switch]$SkipCargo,
    [switch]$Quick
)

$ErrorActionPreference = 'Continue'
$env:RUSTC_WRAPPER = ''

function Resolve-LogOSRoot {
    if ($env:LOGOS_ROOT -and (Test-Path -LiteralPath $env:LOGOS_ROOT)) {
        return (Resolve-Path -LiteralPath $env:LOGOS_ROOT).Path
    }
    $here = $PSScriptRoot
    # ops/release → repo root
    $cand = (Resolve-Path (Join-Path $here '..\..')).Path
    if (Test-Path (Join-Path $cand 'Cargo.toml')) { return $cand }
    return $null
}

$root = Resolve-LogOSRoot
if (-not $root) {
    Write-Host 'FAIL  LOGOS_ROOT unresolved — install shell: pwsh -File ops/Install-LogOSShell.ps1' -ForegroundColor Red
    exit 2
}
Set-Location -LiteralPath $root
Write-Host "LogOS release verify · root=$root" -ForegroundColor Cyan
Write-Host 'ATOM-OPERATOR-JOURNEY-RECOVERY-20260809 · lattice v0.3.0' -ForegroundColor DarkGray

$pass = 0
$fail = 0
$skip = 0
$rows = [System.Collections.Generic.List[object]]::new()

function Add-Result {
    param(
        [string]$Name,
        [ValidateSet('PASS','FAIL','SKIP')]$Status,
        [string]$Detail = '',
        [string]$Remediation = ''
    )
    $script:rows.Add([pscustomobject]@{
        Name = $Name
        Status = $Status
        Detail = $Detail
        Remediation = $Remediation
    })
    switch ($Status) {
        'PASS' { $script:pass++; Write-Host "  PASS  $Name  $Detail" -ForegroundColor Green }
        'FAIL' { $script:fail++; Write-Host "  FAIL  $Name  $Detail" -ForegroundColor Red
                 if ($Remediation) { Write-Host "        → remediate: $Remediation" -ForegroundColor Yellow } }
        'SKIP' { $script:skip++; Write-Host "  SKIP  $Name  $Detail" -ForegroundColor DarkGray }
    }
}

function Invoke-Check {
    param(
        [string]$Name,
        [scriptblock]$Body,
        [string]$Remediation,
        [switch]$Required
    )
    # Bodies must NOT call `exit` — that kills the whole verifier.
    # They should run tools and leave $LASTEXITCODE set, or `return <int>`.
    try {
        $global:LASTEXITCODE = 0
        $out = & $Body 2>&1
        $returned = $out | Where-Object { $_ -is [int] } | Select-Object -Last 1
        $textLines = @($out | Where-Object { $_ -isnot [int] })
        $text = ($textLines | Out-String).Trim()
        $code = if ($null -ne $returned) { [int]$returned }
                elseif ($null -ne $LASTEXITCODE) { [int]$LASTEXITCODE }
                else { 0 }
        if ($code -eq 0) {
            $detail = if ($text.Length -gt 120) { $text.Substring(0, 120) + '…' } else { $text }
            if (-not $detail) { $detail = 'ok' }
            Add-Result -Name $Name -Status PASS -Detail $detail
        } else {
            Add-Result -Name $Name -Status FAIL -Detail "exit=$code $(if ($text) { $text.Substring(0, [Math]::Min(80, $text.Length)) })" -Remediation $Remediation
        }
    } catch {
        Add-Result -Name $Name -Status FAIL -Detail $_.Exception.Message -Remediation $Remediation
    }
}

# ── R2/R3 setup probes ──────────────────────────────────────────────────────
if (-not (Get-Command python -ErrorAction SilentlyContinue) -and -not (Get-Command python3 -ErrorAction SilentlyContinue)) {
    Add-Result -Name 'python' -Status FAIL -Detail 'not on PATH' -Remediation 'install Python 3.11+ or activate .venv'
    Write-Host "`nresult: environment fault" -ForegroundColor Red
    exit 2
}
$py = if (Get-Command python -ErrorAction SilentlyContinue) { 'python' } else { 'python3' }

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    if (-not $SkipCargo) {
        Add-Result -Name 'cargo' -Status FAIL -Detail 'not on PATH' -Remediation 'install rustup; re-open shell after logos-align'
        Write-Host "`nresult: environment fault" -ForegroundColor Red
        exit 2
    }
}

# ── Safe remediate (stack only) ─────────────────────────────────────────────
if ($Remediate) {
    Write-Host 'Remediate: safe auto-fixes only' -ForegroundColor Yellow
    if (Get-Command Repair-TriWeavonStack -ErrorAction SilentlyContinue) {
        try { Repair-TriWeavonStack | Out-Null; Add-Result -Name 'tw-fix' -Status PASS -Detail 'Repair-TriWeavonStack ran' }
        catch { Add-Result -Name 'tw-fix' -Status FAIL -Detail $_.Exception.Message -Remediation 'tw fix · tw up' }
    } else {
        Add-Result -Name 'tw-fix' -Status SKIP -Detail 'unitary profile not loaded' -Remediation '. $PROFILE after Install-TriWeavonUnitaryProfile.ps1'
    }
}

# ── Tools / entangle (always) ───────────────────────────────────────────────
Invoke-Check -Name 'claim_lint' -Remediation 'python tools/claim_lint.py docs/formal/  # tag untagged claims [A]/[B]/[C]' -Body {
    & $py tools/claim_lint.py --help | Out-Null
    if ($LASTEXITCODE -ne 0 -and $null -ne $LASTEXITCODE) { return [int]$LASTEXITCODE }
    if (Test-Path 'docs/formal') {
        & $py tools/claim_lint.py docs/formal/
        return [int]$LASTEXITCODE
    }
    return 0
}

Invoke-Check -Name 'entangle-manifest' -Remediation 'python ops/entangle/validate_manifest.py · fix ops/entangle/manifest.yaml' -Body {
    & $py ops/entangle/validate_manifest.py
    return [int]$LASTEXITCODE
}

# Version / release notes present
if (Test-Path 'VERSION') {
    $ver = (Get-Content VERSION -Raw).Trim()
    Add-Result -Name 'VERSION' -Status PASS -Detail $ver
} else {
    Add-Result -Name 'VERSION' -Status FAIL -Detail 'missing' -Remediation 'restore VERSION file for lattice tag'
}

if (Test-Path 'ops/release/RELEASE-v0.3.0.md') {
    Add-Result -Name 'release-notes' -Status PASS -Detail 'ops/release/RELEASE-v0.3.0.md'
} else {
    Add-Result -Name 'release-notes' -Status FAIL -Detail 'missing' -Remediation 'restore ops/release/RELEASE-v0.3.0.md'
}

# Hopf investigation observe-only stamp (docs present, not a promote)
if (Test-Path 'docs/theory/HOPF-PIROUETTE-INVESTIGATION-20260809.md') {
    Add-Result -Name 'hopf-observe' -Status PASS -Detail 'residual-zero observe-only doc present'
} else {
    Add-Result -Name 'hopf-observe' -Status FAIL -Detail 'missing investigation stamp' -Remediation 'restore docs/theory/HOPF-PIROUETTE-INVESTIGATION-20260809.md'
}

if ($Quick) {
    Write-Host ("`nresult: {0} pass / {1} fail / {2} skip  (quick)" -f $pass, $fail, $skip) -ForegroundColor $(if ($fail) { 'Yellow' } else { 'Green' })
    exit $(if ($fail) { 1 } else { 0 })
}

# ── Cargo Priority A ────────────────────────────────────────────────────────
if ($SkipCargo) {
    Add-Result -Name 'cargo-suite' -Status SKIP -Detail '-SkipCargo'
} else {
    Invoke-Check -Name 'cutile-claim_gate' -Remediation 'cd cutiles/cutile; cargo test --no-default-features --lib claim_gate  # wgpu drift → keep no-default-features' -Body {
        cargo test -p cutile --no-default-features --lib claim_gate -- --quiet
        return [int]$LASTEXITCODE
    }
    Invoke-Check -Name 'cutile-drift_guard' -Remediation 'cargo test -p cutile --no-default-features --lib drift_guard' -Body {
        cargo test -p cutile --no-default-features --lib drift_guard -- --quiet
        return [int]$LASTEXITCODE
    }
    Invoke-Check -Name 'reson8-tui' -Remediation 'cargo test -p reson8-tui --bin reson8-forge · see crates/tui/RELEASE-0.2.1.md' -Body {
        cargo test -p reson8-tui --bin reson8-forge -- --quiet
        return [int]$LASTEXITCODE
    }
    Invoke-Check -Name 'barcode-tui' -Remediation 'cargo test -p barcode-tui · entangle slot barcode-tui' -Body {
        cargo test -p barcode-tui -- --quiet
        return [int]$LASTEXITCODE
    }
}

# ── Optional live stack (never required for tag) ────────────────────────────
if (Get-Command Test-TriWeavonUnitaryHealth -ErrorAction SilentlyContinue) {
    try {
        $hc = Test-TriWeavonUnitaryHealth
        if ($hc -eq 0) { Add-Result -Name 'tw-health' -Status PASS -Detail 'exit 0' }
        else { Add-Result -Name 'tw-health' -Status SKIP -Detail "exit $hc (live stack optional for tag)" -Remediation 'tw fix · tw verify' }
    } catch {
        Add-Result -Name 'tw-health' -Status SKIP -Detail $_.Exception.Message
    }
} else {
    Add-Result -Name 'tw-health' -Status SKIP -Detail 'unitary profile not loaded'
}

Write-Host ''
Write-Host '── board ──' -ForegroundColor Cyan
$rows | Format-Table -AutoSize | Out-String | Write-Host
Write-Host ("result: {0} pass / {1} fail / {2} skip" -f $pass, $fail, $skip) -ForegroundColor $(if ($fail) { 'Yellow' } else { 'Green' })
if ($fail -gt 0) {
    Write-Host 'Recovery: docs/ops/OPERATOR-JOURNEY-AND-RECOVERY-20260809.md · re-run with -Remediate for safe fixes' -ForegroundColor Yellow
    exit 1
}
Write-Host 'Lattice verify green · residual-zero still observe-only · Music conserved' -ForegroundColor Green
exit 0
