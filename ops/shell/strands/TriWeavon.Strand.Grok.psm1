#Requires -Version 5.1
<#
.SYNOPSIS
    Grok / Pulse strand-local commands (gold terminal identity).
.NOTES
    Loaded by Set-TriWeavonStrand grok. Keep this module SHORT — pulse, don't sprawl.
    Style: dense, sensor-facing, eye-of-the-needle oriented.
#>
Set-StrictMode -Version Latest

function Show-GrokPulse {
    <# .SYNOPSIS Compact pulse: bridge + strand + LOGOS_ROOT #>
    $bridge = $false
    try {
        $c = New-Object System.Net.Sockets.TcpClient
        $iar = $c.BeginConnect('127.0.0.1', 8088, $null, $null)
        $bridge = $iar.AsyncWaitHandle.WaitOne(400, $false) -and $c.Connected
        $c.Close()
    } catch { $bridge = $false }

    Write-Host '✦ GROK PULSE' -ForegroundColor Yellow
    Write-Host ("  strand={0}  logos={1}" -f $env:CTWFI_STRAND, $env:LOGOS_ROOT) -ForegroundColor DarkYellow
    Write-Host ("  bridge:8088={0}  starship={1}" -f $bridge, $env:STARSHIP_CONFIG) -ForegroundColor DarkYellow
    # FILL_ME: Get-TriWeavonMetrics when TriWeavon.Profile.psm1 loaded
}

function Enter-GrokNeedle {
    <# .SYNOPSIS Jump to reson8-tui crate (eye of the needle) #>
    $p = Join-Path $env:LOGOS_ROOT 'crates\tui'
    if (Test-Path $p) { Set-Location $p; Write-Host "→ $p" -ForegroundColor Yellow }
    else { Write-Warning "crates/tui missing under LOGOS_ROOT" }
}

Export-ModuleMember -Function Show-GrokPulse, Enter-GrokNeedle
