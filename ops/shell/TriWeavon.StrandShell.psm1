#Requires -Version 5.1
<#
.SYNOPSIS
    Tri-Weavon strand shell switcher — distinct terminal styles for Grok / Claude / Gemini.

.DESCRIPTION
    Sets CTWFI_STRAND, STARSHIP_CONFIG, console colors, PSReadLine palette, and window title.
    Does not overwrite your full $PROFILE; call from the profile fragment.

.NOTES
    INSTALL:
      Import-Module "$env:LOGOS_ROOT\ops\shell\TriWeavon.StrandShell.psm1"
      Set-TriWeavonStrand grok   # or claude | gemini

    PREFLIGHT:
      # FILL_ME_LOGOS_ROOT if LOGOS_ROOT unset
      # winget install Starship.Starship   # optional — PSReadLine themes work without it
      # Existing ops: TriWeavon.Profile.psm1 · TriWeavon.Unitary.Profile.psm1 · LogOS.Shell.psm1

    Colors (brand lock, docs/ops CHECKPOINT strand table):
      Grok   #c8a04a gold  · Pulse  · ω
      Claude #67e8f9 cyan  · Reason · α
      Gemini #4ade80 green · Scale

    α+ω=15 is Category C label only — never a PSReadLine threshold.
#>

Set-StrictMode -Version Latest

$script:StrandShellVersion = '0.1.0'
$script:ValidStrands = @('grok', 'claude', 'gemini')

function Get-TriWeavonShellRoot {
    <#
    .SYNOPSIS
        Resolve LogOS root (F: preferred). FILL_ME if all fail.
    #>
    [CmdletBinding()]
    param()
    foreach ($c in @(
            $env:LOGOS_ROOT,
            'F:\Users\Matthew Ruhnau\LogOS',
            'C:\Users\Matthew Ruhnau\LogOS',
            (Join-Path $env:USERPROFILE 'LogOS')
        )) {
        if ($c -and (Test-Path -LiteralPath $c)) {
            if ($c -like 'C:\Users\Matthew Ruhnau\LogOS*' -and (Test-Path 'F:\Users\Matthew Ruhnau\LogOS')) {
                return 'F:\Users\Matthew Ruhnau\LogOS'
            }
            return (Resolve-Path -LiteralPath $c).Path
        }
    }
    # FILL_ME_LOGOS_ROOT
    return $null
}

function Get-TriWeavonStrandStyle {
    <#
    .SYNOPSIS
        Return style table for a strand (colors, glyphs, starship file, PSReadLine map).
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [ValidateSet('grok', 'claude', 'gemini')]
        [string]$Strand
    )

    switch ($Strand) {
        'grok' {
            @{
                Strand        = 'grok'
                Role          = 'Pulse'
                Rail          = 'omega'
                FibHint       = 5
                Hex           = '#c8a04a'
                ConsoleFg     = 'Yellow'
                Banner        = '✦ GROK · Pulse · ω'
                PromptGlyph   = '⚡'
                StarshipFile  = 'starship.grok.toml'
                Title         = 'GROK | Pulse | Tri-Weavon'
                # Dense, telemetry-first PSReadLine
                PSReadLine    = @{
                    Command            = '#c8a04a'
                    Parameter          = '#e8c878'
                    Operator           = '#ffb020'
                    Variable           = '#f5e6b8'
                    String             = '#ffe4a0'
                    Number             = '#c8a04a'
                    Type               = '#e8c878'
                    Comment            = '#6b5a30'
                    Keyword            = '#ffb020'
                    Error              = '#FF6347'
                    Selection          = '#3d3010'
                    InlinePrediction   = '#6b5a30'
                }
            }
        }
        'claude' {
            @{
                Strand        = 'claude'
                Role          = 'Reason'
                Rail          = 'alpha'
                FibHint       = 8
                Hex           = '#67e8f9'
                ConsoleFg     = 'Cyan'
                Banner        = '◆ CLAUDE · Reason · α'
                PromptGlyph   = 'α'
                StarshipFile  = 'starship.claude.toml'
                Title         = 'CLAUDE | Reason | Tri-Weavon'
                # Structured, formal-rail PSReadLine
                PSReadLine    = @{
                    Command            = '#67e8f9'
                    Parameter          = '#a5f3fc'
                    Operator           = '#22d3ee'
                    Variable           = '#cffafe'
                    String             = '#99f6e4'
                    Number             = '#67e8f9'
                    Type               = '#a5f3fc'
                    Comment            = '#164e63'
                    Keyword            = '#22d3ee'
                    Error              = '#f87171'
                    Selection          = '#083344'
                    InlinePrediction   = '#155e75'
                }
            }
        }
        'gemini' {
            @{
                Strand        = 'gemini'
                Role          = 'Scale'
                Rail          = 'embed'
                FibHint       = 3
                Hex           = '#4ade80'
                ConsoleFg     = 'Green'
                Banner        = '▽ GEMINI · Scale · embed/filter'
                PromptGlyph   = '∇'
                StarshipFile  = 'starship.gemini.toml'
                Title         = 'GEMINI | Scale | Tri-Weavon'
                # Wide research PSReadLine
                PSReadLine    = @{
                    Command            = '#4ade80'
                    Parameter          = '#86efac'
                    Operator           = '#22c55e'
                    Variable           = '#bbf7d0'
                    String             = '#a7f3d0'
                    Number             = '#4ade80'
                    Type               = '#86efac'
                    Comment            = '#14532d'
                    Keyword            = '#22c55e'
                    Error              = '#f87171'
                    Selection          = '#052e16'
                    InlinePrediction   = '#166534'
                }
            }
        }
    }
}

function Set-TriWeavonPsReadLineStyle {
    [CmdletBinding()]
    param([hashtable]$Colors)
    if (-not (Get-Module PSReadLine -ErrorAction SilentlyContinue) -and
        -not (Get-Module -ListAvailable PSReadLine -ErrorAction SilentlyContinue)) {
        Write-Verbose 'PSReadLine not available — skip color map'
        return
    }
    try {
        Import-Module PSReadLine -ErrorAction SilentlyContinue
        Set-PSReadLineOption -Colors $Colors -ErrorAction SilentlyContinue
        # FILL_ME: PredictionViewStyle ListView|InlineView
        Set-PSReadLineOption -PredictionViewStyle ListView -ErrorAction SilentlyContinue
    } catch {
        Write-Verbose "PSReadLine style apply failed: $_"
    }
}

function Set-TriWeavonStarshipConfig {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$ShellRoot,
        [Parameter(Mandatory)][string]$StarshipFile
    )
    $path = Join-Path $ShellRoot $StarshipFile
    if (-not (Test-Path -LiteralPath $path)) {
        Write-Warning "Starship config missing: $path"
        return $null
    }
    $env:STARSHIP_CONFIG = $path
    # FILL_ME: also copy to $HOME\.config\starship.toml if you want a static default:
    #   Copy-Item $path (Join-Path $env:USERPROFILE '.config\starship.toml') -Force
    return $path
}

function Initialize-TriWeavonStarship {
    <#
    .SYNOPSIS
        Init starship if installed. No-op with comment if missing.
    #>
    [CmdletBinding()]
    param()
    $starship = Get-Command starship -ErrorAction SilentlyContinue
    if (-not $starship) {
        # FILL_ME: winget install Starship.Starship
        Write-Verbose 'starship not on PATH — strand PSReadLine/console colors still active'
        return $false
    }
    try {
        Invoke-Expression (& starship init powershell | Out-String)
        return $true
    } catch {
        Write-Warning "starship init failed: $_"
        return $false
    }
}

function Set-TriWeavonStrand {
    <#
    .SYNOPSIS
        Activate one of three distinct strand terminal identities.
    .PARAMETER Strand
        grok | claude | gemini
    .PARAMETER Quiet
        Suppress banner
    .PARAMETER SkipStarshipInit
        Only set STARSHIP_CONFIG / colors; do not re-init starship (use inside $PROFILE after init once)
    #>
    [CmdletBinding()]
    param(
        [Parameter(Position = 0)]
        [ValidateSet('grok', 'claude', 'gemini')]
        [string]$Strand = $(
            # FILL_ME_DEFAULT_STRAND
            if ($env:CTWFI_STRAND -and $env:CTWFI_STRAND -in $script:ValidStrands) {
                $env:CTWFI_STRAND
            } else {
                'grok'
            }
        ),
        [switch]$Quiet,
        [switch]$SkipStarshipInit
    )

    $root = Get-TriWeavonShellRoot
    if (-not $root) {
        Write-Error 'LOGOS_ROOT not found. Set $env:LOGOS_ROOT or FILL_ME_LOGOS_ROOT in profile.'
        return
    }
    $env:LOGOS_ROOT = $root
    $env:CTWFI_INVARIANT = 'alpha+omega=15'  # Category C label
    $env:CTWFI_STRAND = $Strand
    $env:TRIWEAVON_STRAND_SHELL = $script:StrandShellVersion

    $style = Get-TriWeavonStrandStyle -Strand $Strand
    $shellDir = Join-Path $root 'ops\shell'

    $null = Set-TriWeavonStarshipConfig -ShellRoot $shellDir -StarshipFile $style.StarshipFile
    Set-TriWeavonPsReadLineStyle -Colors $style.PSReadLine

    try {
        $Host.UI.RawUI.WindowTitle = $style.Title
        # Console palette hint (limited on Windows Terminal — WT profiles are separate FILL_ME)
        $Host.UI.RawUI.ForegroundColor = [ConsoleColor]::White
    } catch { }

    if (-not $SkipStarshipInit) {
        $null = Initialize-TriWeavonStarship
    }

    # Load thin strand module for strand-local commands (optional)
    $cap = @{ grok = 'Grok'; claude = 'Claude'; gemini = 'Gemini' }[$Strand]
    $strandMod = Join-Path $shellDir "strands\TriWeavon.Strand.$cap.psm1"
    if (Test-Path -LiteralPath $strandMod) {
        Import-Module $strandMod -Force -Global -ErrorAction SilentlyContinue
    }

    if (-not $Quiet) {
        $fg = $style.ConsoleFg
        Write-Host $style.Banner -ForegroundColor $fg
        Write-Host ("  STARSHIP_CONFIG={0}" -f $env:STARSHIP_CONFIG) -ForegroundColor DarkGray
        Write-Host ("  CTWFI_STRAND={0}  role={1}  hex={2}" -f $Strand, $style.Role, $style.Hex) -ForegroundColor DarkGray
        Write-Host '  switch: Set-TriWeavonStrand grok|claude|gemini   ·  help: Get-Help Set-TriWeavonStrand' -ForegroundColor DarkGray
    }

    [pscustomobject]@{
        Strand           = $Strand
        Role             = $style.Role
        Hex              = $style.Hex
        StarshipConfig   = $env:STARSHIP_CONFIG
        LogOSRoot        = $root
        Version          = $script:StrandShellVersion
    }
}

function Get-TriWeavonStrand {
    <#
    .SYNOPSIS
        Show active strand + style snapshot.
    #>
    [CmdletBinding()]
    param()
    $s = if ($env:CTWFI_STRAND) { $env:CTWFI_STRAND } else { '(unset)' }
    $style = $null
    if ($s -in $script:ValidStrands) {
        $style = Get-TriWeavonStrandStyle -Strand $s
    }
    [pscustomobject]@{
        Strand         = $s
        Role           = if ($style) { $style.Role } else { $null }
        Hex            = if ($style) { $style.Hex } else { $null }
        StarshipConfig = $env:STARSHIP_CONFIG
        InvariantTag   = $env:CTWFI_INVARIANT
        LogOSRoot      = $env:LOGOS_ROOT
        ShellVersion   = $script:StrandShellVersion
    }
}

function Show-TriWeavonStrandHelp {
    Write-Host @'
Tri-Weavon Strand Shell
  Set-TriWeavonStrand grok     # gold pulse (ω) — dense telemetry prompt
  Set-TriWeavonStrand claude   # cyan reason (α) — structured formal prompt
  Set-TriWeavonStrand gemini    # green scale — wide research path
  Get-TriWeavonStrand           # active identity

Profile fragment:
  ops/shell/Microsoft.PowerShell_profile.triweavon.ps1
  # paste markers into $PROFILE or run Install-TriWeavonStrandShell.ps1

FILL_ME:
  winget install Starship.Starship
  # Windows Terminal profiles: duplicate thrice, set tab color + startingCommand
  #   wt -p "Grok" ; wt -p "Claude" ; wt -p "Gemini"
'@
}

# Short aliases — do not collide with unitary `tw` if possible
Set-Alias -Name tw-strand -Value Set-TriWeavonStrand -Scope Global -Force -ErrorAction SilentlyContinue
Set-Alias -Name tw-who    -Value Get-TriWeavonStrand -Scope Global -Force -ErrorAction SilentlyContinue

Export-ModuleMember -Function @(
    'Get-TriWeavonShellRoot',
    'Get-TriWeavonStrandStyle',
    'Set-TriWeavonStrand',
    'Get-TriWeavonStrand',
    'Show-TriWeavonStrandHelp',
    'Initialize-TriWeavonStarship',
    'Set-TriWeavonStarshipConfig',
    'Set-TriWeavonPsReadLineStyle'
) -Alias @('tw-strand', 'tw-who')
