#Requires -Version 5.1
<#
.SYNOPSIS
    Claude / Reason strand-local commands (cyan terminal identity).
.NOTES
    Loaded by Set-TriWeavonStrand claude. Formal / LABEL / VERIFY oriented.
    Style: structured checklist surfaces — never paint B as green.
#>
Set-StrictMode -Version Latest

function Show-ClaudeReason {
    <# .SYNOPSIS Reason-rail status: cert path + init packet + formal roots #>
    $root = $env:LOGOS_ROOT
    $init = Join-Path $root 'CLAUDECODE-INIT-v0_1.md'
    $cert = Join-Path $root '.atom-trail\certs\claude-code\latest.json'
    $settings = Join-Path $root '.claude\settings.json'

    Write-Host '◆ CLAUDE REASON' -ForegroundColor Cyan
    Write-Host ("  init={0}" -f (Test-Path $init)) -ForegroundColor DarkCyan
    Write-Host ("  settings={0}" -f (Test-Path $settings)) -ForegroundColor DarkCyan
    Write-Host ("  cert={0}" -f (Test-Path $cert)) -ForegroundColor DarkCyan
    if (Test-Path $cert) {
        try {
            $j = Get-Content -LiteralPath $cert -Raw | ConvertFrom-Json
            $pass = $j.pass
            $color = if ($pass) { 'Green' } else { 'DarkYellow' }  # B stays amber-ish when false
            Write-Host ("  cert.pass={0}  (deploy only if true)" -f $pass) -ForegroundColor $color
        } catch {
            Write-Host '  cert: unreadable JSON' -ForegroundColor DarkYellow
        }
    } else {
        Write-Host '  cert: [CATEGORY B: PLANNED, NOT BUILT] no latest.json' -ForegroundColor DarkYellow
    }
}

function Enter-ClaudeFormal {
    <# .SYNOPSIS Jump to formal core (lean + agda) #>
    $lean = Join-Path $env:LOGOS_ROOT 'lean'
    if (Test-Path $lean) { Set-Location $lean; Write-Host "→ $lean" -ForegroundColor Cyan }
    else { Write-Warning 'lean/ missing' }
}

function Show-ClaudeSurfaces {
    <# .SYNOPSIS Three surfaces from CLAUDECODE-INIT (paths only — no deploy) #>
    Write-Host 'Surfaces (survey):' -ForegroundColor Cyan
    Write-Host '  A SpiralSafe wrangler · B VCS · C Formal core' -ForegroundColor DarkCyan
    Write-Host '  packet: CLAUDECODE-INIT-v0_1.md' -ForegroundColor DarkCyan
    # FILL_ME: wire Test-TriWeavonStack when profile module loaded
}

Export-ModuleMember -Function Show-ClaudeReason, Enter-ClaudeFormal, Show-ClaudeSurfaces
