# =============================================================================
# Tri-Weavon · $PROFILE fragment (WITH-BLANKS)
# ATOM: ATOM-TRIWEAVON-STRAND-SHELL-20260722
# =============================================================================
#
# HOW TO INSTALL (pick one):
#
#   A) Automated:
#        pwsh -File "$env:LOGOS_ROOT\ops\shell\Install-TriWeavonStrandShell.ps1"
#
#   B) Manual: paste everything between the markers into:
#        $PROFILE
#      which on this machine is often:
#        C:\Users\toolated\OneDrive\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
#      or Documents\PowerShell\Microsoft.PowerShell_profile.ps1
#
# ORDERING vs existing modules:
#   1. Chris Titus / personal utilities (optional)
#   2. ops\TriWeavon.Profile.psm1          (metrics / bridge)
#   3. ops\TriWeavon.Unitary.Profile.psm1  (tw cockpit)
#   4. THIS block — strand shell + starship style
#   5. ops\LogOS.Shell.psm1                (optional logos-* commands)
#
# FILL_ME blanks below before first use.
# =============================================================================

# >>> TriWeavon.StrandShell >>>
# Portable LOGOS_ROOT: env · %USERPROFILE%\LogOS · %USERNAME% drive scan (prefer non-C:)
if (-not $env:LOGOS_ROOT -or -not (Test-Path -LiteralPath $env:LOGOS_ROOT)) {
    $__twCands = [System.Collections.Generic.List[string]]::new()
    if ($env:USERPROFILE) { [void]$__twCands.Add((Join-Path $env:USERPROFILE 'LogOS')) }
    if ($HOME) { [void]$__twCands.Add((Join-Path $HOME 'LogOS')) }
    if ($env:USERNAME) {
        foreach ($__drv in @('F:', 'D:', 'G:', 'E:', 'H:', 'C:')) {
            [void]$__twCands.Add("$__drv\Users\$env:USERNAME\LogOS")
        }
    }
    $__twPick = $null
    foreach ($__c in $__twCands) {
        if ($__c -and (Test-Path -LiteralPath (Join-Path $__c 'Cargo.toml'))) {
            if ($__c -notmatch '^[Cc]:\\') { $__twPick = $__c; break }
            if (-not $__twPick) { $__twPick = $__c }
        }
    }
    if ($__twPick) { $env:LOGOS_ROOT = $__twPick }
    Remove-Variable __twCands, __twPick, __c, __drv -ErrorAction SilentlyContinue
}

# FILL_ME_DEFAULT_STRAND: grok | claude | gemini
if (-not $env:CTWFI_STRAND) {
    $env:CTWFI_STRAND = 'grok'
}

$env:CTWFI_INVARIANT = 'alpha+omega=15'  # Category C label only

$__twStrandMod = Join-Path $env:LOGOS_ROOT 'ops\shell\TriWeavon.StrandShell.psm1'
if ($env:LOGOS_ROOT -and (Test-Path -LiteralPath $__twStrandMod)) {
    Import-Module $__twStrandMod -Force

    # Apply distinct style for this session (Quiet on auto-load; drop -Quiet for banner)
    Set-TriWeavonStrand -Strand $env:CTWFI_STRAND -Quiet

    # Starship: only if installed. FILL_ME: winget install Starship.Starship
    if (Get-Command starship -ErrorAction SilentlyContinue) {
        # STARSHIP_CONFIG already set by Set-TriWeavonStrand
        Invoke-Expression (& starship init powershell | Out-String)
    } else {
        # Fallback: minimal strand-colored prompt if starship absent
        function global:prompt {
            $s = $env:CTWFI_STRAND
            $glyph = switch ($s) {
                'claude' { 'α' }
                'gemini' { '∇' }
                default  { '⚡' }
            }
            $loc = $executionContext.SessionState.Path.CurrentLocation
            "PS $glyph [$s] $loc$('>' * ($nestedPromptLevel + 1)) "
        }
    }
} else {
    Write-Warning "TriWeavon.StrandShell.psm1 not found. Set LOGOS_ROOT. FILL_ME."
}
Remove-Variable __twStrandMod -ErrorAction SilentlyContinue

# Optional: load unitary + classic profile without clobbering strand style
# FILL_ME: uncomment if desired
# Import-Module (Join-Path $env:LOGOS_ROOT 'ops\TriWeavon.Profile.psm1') -Force -ErrorAction SilentlyContinue
# Import-Module (Join-Path $env:LOGOS_ROOT 'ops\TriWeavon.Unitary.Profile.psm1') -Force -ErrorAction SilentlyContinue
# Import-Module (Join-Path $env:LOGOS_ROOT 'ops\LogOS.Shell.psm1') -Force -ErrorAction SilentlyContinue

# Windows Terminal tip (FILL_ME — manual WT JSON):
#   Create 3 profiles with tab colors:
#     Grok   #c8a04a  commandline: pwsh -NoExit -Command "Set-TriWeavonStrand grok"
#     Claude #67e8f9  commandline: pwsh -NoExit -Command "Set-TriWeavonStrand claude"
#     Gemini #4ade80  commandline: pwsh -NoExit -Command "Set-TriWeavonStrand gemini"
# <<< TriWeavon.StrandShell <<<
