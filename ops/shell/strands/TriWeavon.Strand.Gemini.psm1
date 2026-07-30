#Requires -Version 5.1
<#
.SYNOPSIS
    Gemini / Scale strand-local commands (green terminal identity).
.NOTES
    Loaded by Set-TriWeavonStrand gemini. Embed / filter / research width.
    Style: long paths, dump/stitch/index awareness — populate, don't bound.
#>
Set-StrictMode -Version Latest

function Show-GeminiScale {
    <# .SYNOPSIS Scale-rail status: stitch / dump / wrangler config presence #>
    $root = $env:LOGOS_ROOT
    $stitch = Join-Path $root 'stitch'
    $wrangler = Join-Path $root 'wrangler.toml'
    $dump = 'F:\Users\Matthew Ruhnau\LogOS.worktrees\master\9P2000.L\strands\User_Dropfiles\dump'
    # FILL_ME_DUMP_PATH if worktree moves

    Write-Host '▽ GEMINI SCALE' -ForegroundColor Green
    Write-Host ("  stitch={0}" -f (Test-Path $stitch)) -ForegroundColor DarkGreen
    Write-Host ("  wrangler.toml={0}" -f (Test-Path $wrangler)) -ForegroundColor DarkGreen
    Write-Host ("  dump={0}" -f (Test-Path $dump)) -ForegroundColor DarkGreen
    $w = Get-Command wrangler -ErrorAction SilentlyContinue
    if ($w) {
        Write-Host ("  wrangler PATH={0}" -f $w.Source) -ForegroundColor DarkGreen
        # FILL_ME: do not wrangler deploy from this helper — survey only
    } else {
        Write-Host '  wrangler: not on PATH or broken install' -ForegroundColor DarkYellow
    }
}

function Enter-GeminiStitch {
    <# .SYNOPSIS Jump to stitch design surface #>
    $p = Join-Path $env:LOGOS_ROOT 'stitch'
    if (Test-Path $p) { Set-Location $p; Write-Host "→ $p" -ForegroundColor Green }
    else { Write-Warning 'stitch/ missing' }
}

function Enter-GeminiDump {
    <# .SYNOPSIS Jump to User_Dropfiles dump (read-only discipline) #>
    $dump = 'F:\Users\Matthew Ruhnau\LogOS.worktrees\master\9P2000.L\strands\User_Dropfiles\dump'
    # FILL_ME_DUMP_PATH
    if (Test-Path $dump) { Set-Location $dump; Write-Host "→ $dump" -ForegroundColor Green }
    else { Write-Warning "dump path missing — FILL_ME_DUMP_PATH in TriWeavon.Strand.Gemini.psm1" }
}

Export-ModuleMember -Function Show-GeminiScale, Enter-GeminiStitch, Enter-GeminiDump
