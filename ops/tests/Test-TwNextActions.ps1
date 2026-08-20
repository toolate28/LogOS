# ATOM-CLAUDE-LABEL-TW-NEXTLINE-20260820
# Fail the build if tw next: would print prose in a command slot.
# Checked by claude/reason. Owner: grok/build.

$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$mod = Join-Path (Split-Path -Parent $here) 'TriWeavon.Unitary.Profile.psm1'
if (-not (Test-Path -LiteralPath $mod)) {
    $mod = Join-Path $PSScriptRoot '../TriWeavon.Unitary.Profile.psm1'
}
Import-Module $mod -Force

$pass = 0; $fail = 0
function Expect($line, $ok) {
    $got = Test-NextActionRunnable -Line $line
    if ($got -eq $ok) {
        Write-Host "  PASS  ok=$ok  $line" -ForegroundColor Green
        $script:pass++
    } else {
        Write-Host "  FAIL  expected ok=$ok got=$got  $line" -ForegroundColor Red
        $script:fail++
    }
}

Expect 'logos-bridge' $true
Expect 'tw up waist   # or WSL: cd "$LOGOS_ROOT" && docker compose up -d' $true
Expect 'tw up bbbr    # python hup/unikernel/bbbr-verifier/bbbr_unix.py (:8081)' $true
Expect 'tw up styx    # cargo run -p styx-vfs-layer --bin styx-bookshelf' $true
Expect '# Set LOGOS_ROOT=%USERPROFILE%\LogOS (or your clone path)' $true
Expect '# Install/start WSL Kali (build substrate)' $true
Expect '# Start Docker Desktop or dockerd in WSL' $true
Expect '# Pull LogOS; ensure docs/schemas/v0.1 exists' $true
Expect 'Optional: start reson8 styx WS bridge on :8088' $false
Expect 'Set LOGOS_ROOT=%USERPROFILE%\LogOS (or your clone path)' $false
Expect '' $true
Expect '# comment only' $true

Write-Host ("  result: {0} pass / {1} fail" -f $pass, $fail)
if ($fail -gt 0) { exit 1 }
exit 0
