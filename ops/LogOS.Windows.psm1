#Requires -Version 5.1
<#
.SYNOPSIS
    Windows-native preflight, bootstrap, wrangler, clean, and dynamic-terminal axis for LogOS.
.DESCRIPTION
    PowerShell equivalents for bash preflight/bootstrap scripts so Windows hosts
    stay on the same toral focii as WSL (α+ω=15).

    Load:
      Import-Module "$env:LOGOS_ROOT\ops\LogOS.Windows.psm1" -Force
      Invoke-LogOSPreflight
      logos-align
      logos-clean

    Commands:
      logos-preflight   Full Windows preflight board
      logos-align       Idempotent profile + PATH + wrangler axis (Set-LogOSWindowsAxis)
      logos-clean       Repo hygiene scan / deep reclaim (HITL — Apply needs -Force)
      logos-wrangler    Deploy / whoami / pages for coherence-site
      logos-terminal    HTML dynamic terminal · tui · sensors · pop/window/tab
      logos-pop         Pop out a new OS console (Windows Terminal / pwsh)
      adhealth-preflight / adhealth-run
      hup-preflight
      gb-deploy         GB-06 Cloud Run (HITL — requires -Confirm or -Force)
#>

Set-StrictMode -Version Latest

$script:WindowsAxisVersion = '1.0.1-win-axis'
# Portable default: %USERPROFILE%\LogOS (override with LOGOS_ROOT)
$script:CanonicalRoot = if ($env:USERPROFILE) { Join-Path $env:USERPROFILE 'LogOS' } else { $null }

. (Join-Path $PSScriptRoot 'LogOS.Root.ps1')

# ─── Roots ───────────────────────────────────────────────────────────────────

function Resolve-LogOSWindowsRoot {
    [CmdletBinding()]
    param()
    $root = Resolve-LogOSRootPortable -ScriptRoot $PSScriptRoot
    if ($root) { return $root }
    throw 'LogOS root not found. Set LOGOS_ROOT=%USERPROFILE%\LogOS (or your clone path)'
}

function Get-LogOSPython {
    [CmdletBinding()]
    param([string]$Root)
    if (-not $Root) { $Root = Resolve-LogOSWindowsRoot }
    $raw = @(
        (Join-Path $Root '.venv\Scripts\python.exe')
        (Join-Path $Root 'venv2\Scripts\python.exe')
        (Get-Command python -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source)
        (Get-Command py -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source)
    ) | Where-Object { $_ }
    foreach ($c in $raw) {
        try {
            if (Test-Path -LiteralPath $c) {
                return (Resolve-Path -LiteralPath $c).Path
            }
        } catch { }
    }
    return $null
}

# ─── Preflight probes ────────────────────────────────────────────────────────

function Invoke-LogOSPreflight {
    <#
    .SYNOPSIS
        Windows preflight — git, shell, wrangler, sensors, formal roots.
    .OUTPUTS
        Exit code 0=PASS, 1=degraded, 2=fail
    #>
    [CmdletBinding()]
    param(
        [switch]$Json,
        [switch]$Quiet
    )

    $root = Resolve-LogOSWindowsRoot
    $env:LOGOS_ROOT = $root
    $env:CTWFI_INVARIANT = 'alpha+omega=15'
    $rows = [System.Collections.Generic.List[object]]::new()

    function Add-Probe([string]$Name, [bool]$Ok, [string]$Detail, [bool]$Critical = $false) {
        $rows.Add([pscustomobject]@{
                name     = $Name
                ok       = $Ok
                critical = $Critical
                detail   = $Detail
            }) | Out-Null
    }

    Add-Probe 'logos_root' $true $root $true
    Add-Probe 'conservation' $true 'α=7 ω=8 Σ=15' $true

    $git = Get-Command git -ErrorAction SilentlyContinue
    if ($git) {
        Push-Location $root
        try {
            $branch = (git rev-parse --abbrev-ref HEAD 2>$null)
            $dirty = @(git status --porcelain 2>$null).Count
            $ahead = git rev-list --left-right --count 'LogOS/master...HEAD' 2>$null
            Add-Probe 'git' $true ("$branch dirty=$dirty ahead=$ahead") $true
        } finally { Pop-Location }
    } else {
        Add-Probe 'git' $false 'git not on PATH' $true
    }

    $pwshOk = $PSVersionTable.PSVersion.Major -ge 5
    Add-Probe 'pwsh' $pwshOk ("v$($PSVersionTable.PSVersion)") $true

    $profileHost = $PROFILE.CurrentUserCurrentHost
    $hasLogos = $false
    if ($profileHost -and (Test-Path -LiteralPath $profileHost)) {
        $txt = Get-Content -LiteralPath $profileHost -Raw -ErrorAction SilentlyContinue
        $hasLogos = $txt -match 'LogOS\.Shell'
    }
    Add-Probe 'profile_hook' $hasLogos $(if ($hasLogos) { $profileHost } else { 'missing LogOS.Shell block' }) $false

    $unitaryMod = Join-Path $root 'ops\TriWeavon.Unitary.Profile.psm1'
    Add-Probe 'unitary_module' (Test-Path $unitaryMod) $unitaryMod $false

    $sitePublic = Join-Path $root 'coherence-mcp\coherence-site\public'
    $meta = Join-Path $sitePublic 'meta-map\index.html'
    Add-Probe 'coherence_site' (Test-Path $meta) $sitePublic $false

    $siteWrangler = Join-Path $root 'coherence-mcp\coherence-site\wrangler.toml'
    Add-Probe 'site_wrangler' (Test-Path $siteWrangler) $siteWrangler $false

    $wranglerCmd = Get-Command wrangler -ErrorAction SilentlyContinue
    if (-not $wranglerCmd) {
        $npx = Get-Command npx -ErrorAction SilentlyContinue
        Add-Probe 'wrangler' ([bool]$npx) $(if ($npx) { 'via npx wrangler' } else { 'install: npm i -g wrangler' }) $false
    } else {
        Add-Probe 'wrangler' $true $wranglerCmd.Source $false
    }

    $py = Get-LogOSPython -Root $root
    Add-Probe 'python' ([bool]$py) $(if ($py) { $py } else { 'missing' }) $false

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    Add-Probe 'cargo' ([bool]$cargo) $(if ($cargo) { $cargo.Source } else { 'missing' }) $false

    $wsl = Get-Command wsl -ErrorAction SilentlyContinue
    Add-Probe 'wsl' ([bool]$wsl) $(if ($wsl) { 'OK' } else { 'optional for GB/nix' }) $false

    $schemas = Test-Path (Join-Path $root 'docs\schemas\v0.1\validate.py')
    Add-Probe 'schemas_v0.1' $schemas 'docs/schemas/v0.1' $true

    $adBin = Join-Path $root 'adhealth-meaningseed\bin'
    Add-Probe 'adhealth_bin' (Test-Path (Join-Path $adBin 'Preflight.ps1')) $adBin $false

    $term = Join-Path $sitePublic 'terminal\index.html'
    Add-Probe 'dynamic_terminal' (Test-Path $term) $term $false

    $critFail = @($rows | Where-Object { $_.critical -and -not $_.ok }).Count
    $softFail = @($rows | Where-Object { -not $_.critical -and -not $_.ok }).Count
    $pass = @($rows | Where-Object ok).Count
    $total = $rows.Count
    $wave = if ($total) { [math]::Round(100.0 * $pass / $total, 1) } else { 0 }
    $code = if ($critFail -gt 0) { 2 } elseif ($softFail -gt 0) { 1 } else { 0 }

    $board = [pscustomobject]@{
        version   = $script:WindowsAxisVersion
        timestamp = (Get-Date).ToString('o')
        logos_root = $root
        wave      = $wave
        exit_code = $code
        probes    = $rows
        status    = switch ($code) { 0 { 'PASS' } 1 { 'DEGRADED' } default { 'FAIL' } }
    }

    if ($Json) {
        $board | ConvertTo-Json -Depth 6
        return $code
    }

    if (-not $Quiet) {
        $wc = if ($code -eq 0) { 'Green' } elseif ($code -eq 1) { 'Yellow' } else { 'Red' }
        Write-Host ''
        Write-Host ("  LogOS Windows Preflight  {0}  WAVE={1}%  α+ω=15" -f $board.status, $wave) -ForegroundColor $wc
        Write-Host ("  root: {0}" -f $root) -ForegroundColor DarkGray
        foreach ($r in $rows) {
            $m = if ($r.ok) { '●' } else { '○' }
            $c = if ($r.ok) { 'Green' } elseif ($r.critical) { 'Red' } else { 'Yellow' }
            Write-Host ("  {0} {1,-18} {2}" -f $m, $r.name, $r.detail) -ForegroundColor $c
        }
        Write-Host ("  exit={0}  (0=PASS 1=DEGRADED 2=FAIL)" -f $code) -ForegroundColor DarkGray
        Write-Host ''
    }
    return $code
}

# ─── AdHealth (bin/preflight.sh + run.sh) ────────────────────────────────────

function Invoke-AdHealthPreflight {
    <#
    .SYNOPSIS
        Windows equivalent of adhealth-meaningseed/bin/preflight.sh
    .NOTES
        Tree may ship without src/ (scaffold only). Report DEGRADED, do not hard-crash.
    #>
    [CmdletBinding()]
    param()
    $root = Resolve-LogOSWindowsRoot
    $proj = Join-Path $root 'adhealth-meaningseed'
    if (-not (Test-Path $proj)) { throw "Missing $proj" }
    $py = Get-LogOSPython -Root $root
    if (-not $py) { throw 'python not found' }

    $src = Join-Path $proj 'src'
    $pkg = Join-Path $src 'adhealth'
    if (-not (Test-Path $pkg)) {
        Write-Host 'adhealth: src/adhealth missing (scaffold only — package not filed)' -ForegroundColor Yellow
        Write-Host 'preflight: DEGRADED  (awaiting src restore / pip install -e .)' -ForegroundColor Yellow
        Write-Host "  expected: $pkg" -ForegroundColor DarkGray
        Write-Host "  tests:    $(Join-Path $proj 'tests')" -ForegroundColor DarkGray
        return 1
    }

    $env:PYTHONPATH = $src
    Push-Location $proj
    try {
        & $py -c "from adhealth.core.dynamics import QuantumWalkDynamics; print('dynamics OK')"
        if ($LASTEXITCODE -ne 0) {
            Write-Host 'preflight: DEGRADED (import failed)' -ForegroundColor Yellow
            return 1
        }
        & $py -m adhealth.cli analyze --demo --json 1>$null 2>$null
        if ($LASTEXITCODE -ne 0) {
            Write-Host 'adhealth cli analyze --demo: SKIP/FAIL (check install)' -ForegroundColor Yellow
            Write-Host 'preflight: DEGRADED' -ForegroundColor Yellow
            return 1
        }
        Write-Host 'preflight: PASS' -ForegroundColor Green
        return 0
    } finally {
        Pop-Location
        Remove-Item Env:PYTHONPATH -ErrorAction SilentlyContinue
    }
}

function Invoke-AdHealthRun {
    <#
    .SYNOPSIS
        Windows equivalent of adhealth-meaningseed/bin/run.sh
    #>
    [CmdletBinding()]
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$CliArgs
    )
    $root = Resolve-LogOSWindowsRoot
    $proj = Join-Path $root 'adhealth-meaningseed'
    $py = Get-LogOSPython -Root $root
    if (-not $py) { throw 'python not found' }
    $env:PYTHONPATH = (Join-Path $proj 'src')
    Push-Location $proj
    try {
        if (-not $CliArgs -or $CliArgs.Count -eq 0) {
            & $py -m adhealth.cli analyze --demo
        } else {
            & $py -m adhealth.cli @CliArgs
        }
    } finally {
        Pop-Location
        Remove-Item Env:PYTHONPATH -ErrorAction SilentlyContinue
    }
}

# ─── HUP guest preflight ─────────────────────────────────────────────────────

function Invoke-HupGuestPreflight {
    <#
    .SYNOPSIS
        Windows equivalent of hup/instance3-rvm/preflight-guest.sh
    #>
    [CmdletBinding()]
    param(
        [string]$HupRoot
    )
    $root = Resolve-LogOSWindowsRoot
    if (-not $HupRoot) { $HupRoot = Join-Path $root 'hup' }
    Write-Host '=== HUP M3 RVM guest preflight (Windows) ===' -ForegroundColor Cyan
    $py = Get-LogOSPython -Root $root
    $pyDir = Join-Path $HupRoot 'python'
    if ((Test-Path $pyDir) -and $py) {
        $cm = Join-Path $pyDir 'constraint_mathematics.py'
        $dc = Join-Path $pyDir 'dimensional_collapse.py'
        if (Test-Path $cm) { & $py $cm }
        if (Test-Path $dc) { & $py $dc }
    } else {
        Write-Host '  python probes skipped (no hup/python or python)' -ForegroundColor DarkGray
    }
    $manifest = Join-Path $HupRoot 'rust\Cargo.toml'
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        if (Get-Command hup-rust -ErrorAction SilentlyContinue) {
            & hup-rust
        } elseif (Test-Path $manifest) {
            cargo run --manifest-path $manifest
        }
    }
    Write-Host '=== M3 RVM preflight complete · α+ω=15 ===' -ForegroundColor Green
}

# ─── Wrangler / coherence-site ───────────────────────────────────────────────

function Get-CoherenceSiteRoot {
    Join-Path (Resolve-LogOSWindowsRoot) 'coherence-mcp\coherence-site'
}

function Invoke-LogOSWrangler {
    <#
    .SYNOPSIS
        Wrangler helper for coherence-site (and optional adhealth portal).
    .PARAMETER Action
        whoami | pages-dev | pages-deploy | adhealth-deploy | list
    .PARAMETER DryRun
        Print commands only (HITL safe).
    #>
    [CmdletBinding()]
    param(
        [ValidateSet('whoami', 'pages-dev', 'pages-deploy', 'adhealth-deploy', 'list', 'status')]
        [string]$Action = 'status',
        [switch]$DryRun,
        [switch]$Force
    )

    $site = Get-CoherenceSiteRoot
    $npx = Get-Command npx -ErrorAction SilentlyContinue
    $wr = Get-Command wrangler -ErrorAction SilentlyContinue
    # Force [string[]] — single-element @() unrolls to [string]; StrictMode then
    # throws PropertyNotFoundException on .Count (user-facing logos-wrangler status).
    if ($wr) {
        [string[]]$runner = @('wrangler')
    } elseif ($npx) {
        [string[]]$runner = @('npx', '--yes', 'wrangler')
    } else {
        throw 'wrangler not found. Run: npm i -g wrangler   or ensure npx is on PATH'
    }

    function Invoke-Wrangler {
        # Do not name param $Args — collides with automatic $args and drops CLI tokens.
        param(
            [Parameter(Mandatory)]
            [string[]]$CliArgs,
            [Parameter(Mandatory)]
            [string]$WorkDir
        )
        $cmd = [string[]]@($runner)
        if ($DryRun) {
            Write-Host ("  DRY  cd {0}; {1} {2}" -f $WorkDir, ($cmd -join ' '), ($CliArgs -join ' ')) -ForegroundColor Yellow
            return
        }
        Push-Location $WorkDir
        try {
            if ($cmd.Length -eq 1) {
                & $cmd[0] @CliArgs
            } else {
                $prefix = [string[]]@($cmd[1..($cmd.Length - 1)])
                & $cmd[0] @prefix @CliArgs
            }
        } finally { Pop-Location }
    }

    switch ($Action) {
        'list' {
            Write-Host "coherence-site: $site"
            Write-Host "wrangler.toml:  $(Test-Path (Join-Path $site 'wrangler.toml'))"
            Write-Host "public/:        $(Test-Path (Join-Path $site 'public'))"
            Write-Host "ops-shell:      $(Test-Path (Join-Path $site 'public\stitch\_shared\ops-shell.js'))"
            Write-Host "mcp-client:     $(Test-Path (Join-Path $site 'public\stitch\_shared\mcp-client.js'))"
            Write-Host "lattice OS:     $(Test-Path (Join-Path $site 'public\os\lattice\index.html'))"
            Write-Host "runner:         $($runner -join ' ')"
        }
        'status' {
            Write-Host "=== logos-wrangler status · SpiralSafe lattice ===" -ForegroundColor Cyan
            Write-Host "site: $site"
            $proj = if ($env:LOGOS_PAGES_PROJECT) { $env:LOGOS_PAGES_PROJECT } else { 'spiralsafe' }
            Write-Host "pages project: $proj"
            Write-Host "SITE_BASE default: https://spiralsafe-ey9.pages.dev"
            Write-Host "API_BASE:          https://api.spiralsafe.org"
            foreach ($rel in @(
                'public\index.html',
                'public\os\lattice\index.html',
                'public\cockpit\index.html',
                'public\meta-map\index.html',
                'public\terminal\index.html',
                'public\site-config.js',
                'public\stitch\_shared\mcp-client.js',
                'public\stitch\_shared\ops-shell.js'
            )) {
                $p = Join-Path $site $rel
                $ok = Test-Path $p
                Write-Host ("  {0}  {1}" -f ($(if ($ok) { 'OK ' } else { 'MISS' })), $rel)
            }
            try {
                $h = Invoke-RestMethod -Uri 'https://api.spiralsafe.org/api/health' -TimeoutSec 8
                Write-Host ("API health: {0} v{1}" -f $h.status, $h.version) -ForegroundColor Green
            } catch {
                Write-Host ("API health: amber / unreachable ({0})" -f $_.Exception.Message) -ForegroundColor Yellow
            }
            Write-Host '--- wrangler whoami ---' -ForegroundColor DarkGray
            Invoke-Wrangler -WorkDir $site -CliArgs @('whoami')
        }
        'whoami' {
            Invoke-Wrangler -WorkDir $site -CliArgs @('whoami')
        }
        'pages-dev' {
            Invoke-Wrangler -WorkDir $site -CliArgs @('pages', 'dev', 'public', '--compatibility-date=2024-11-01')
        }
        'pages-deploy' {
            if (-not $Force -and -not $DryRun) {
                Write-Host 'HITL: pages deploy publishes to Cloudflare. Re-run with -Force or -DryRun.' -ForegroundColor Yellow
                Write-Host "  logos-wrangler pages-deploy -Force" -ForegroundColor Cyan
                return
            }
            # Prefer SpiralSafe Pages project (toolated lattice host retired).
            $proj = if ($env:LOGOS_PAGES_PROJECT) { $env:LOGOS_PAGES_PROJECT } else { 'spiralsafe' }
            Invoke-Wrangler -WorkDir $site -CliArgs @('pages', 'deploy', 'public', "--project-name=$proj")
        }
        'adhealth-deploy' {
            $ad = Join-Path (Resolve-LogOSWindowsRoot) 'adhealth-meaningseed'
            if (-not $Force -and -not $DryRun) {
                Write-Host 'HITL: adhealth portal deploy. Re-run with -Force or -DryRun.' -ForegroundColor Yellow
                return
            }
            Invoke-Wrangler -WorkDir $ad -CliArgs @('pages', 'deploy')
        }
    }
}

# ─── Pop-out OS console (Windows Terminal / pwsh) ────────────────────────────

function Open-LogOSConsole {
    <#
    .SYNOPSIS
        Pop out a new real terminal window or tab (not the HTML surface).

    .DESCRIPTION
        Prefer Windows Terminal (`wt`):
          -window  → new WT window  (default)
          -tab     → new tab in the current WT window
        Fallback: Start-Process pwsh (new console host).

        Optional Tri-Weavon strand style: grok | claude | gemini
        (loads ops/shell/TriWeavon.StrandShell.psm1 when present).

    .EXAMPLE
        logos-pop
        logos-pop -Strand grok
        logos-pop -Layout tab -Strand claude
        logos-pop -Command "tw" -Title "Unitary"
        logos-terminal pop
        logos-terminal window -Strand gemini
    .NOTES
        GNU `screen` / `tmux` are not native on this host; use WSL if you need them:
          wsl -d kali-linux -- tmux new -s logos
    #>
    [CmdletBinding()]
    param(
        [ValidateSet('window', 'tab')]
        [string]$Layout = 'window',

        [ValidateSet('none', 'grok', 'claude', 'gemini')]
        [string]$Strand = 'none',

        [string]$WorkingDirectory,

        # Extra PowerShell to run after profile/env (e.g. "tw", "logos-status")
        [string]$Command,

        [string]$Title,

        # Skip loading $PROFILE in the child (faster, thinner)
        [switch]$NoProfile,

        [switch]$WhatIf
    )

    $root = Resolve-LogOSWindowsRoot
    $wd = if ($WorkingDirectory -and (Test-Path -LiteralPath $WorkingDirectory)) {
        (Resolve-Path -LiteralPath $WorkingDirectory).Path
    } else {
        $root
    }

    $pwshCmd = Get-Command pwsh -ErrorAction SilentlyContinue
    $pwsh = if ($pwshCmd) { $pwshCmd.Source } else { 'pwsh.exe' }

    if (-not $Title) {
        $Title = switch ($Strand) {
            'grok'   { 'GROK | Pulse | LogOS' }
            'claude' { 'CLAUDE | Reason | LogOS' }
            'gemini' { 'GEMINI | Scale | LogOS' }
            default  { 'LogOS shell' }
        }
    }

    $esc = { param([string]$s) if ([string]::IsNullOrEmpty($s)) { '' } else { $s.Replace("'", "''") } }

    $bootParts = [System.Collections.Generic.List[string]]::new()
    [void]$bootParts.Add("`$env:LOGOS_ROOT = '$(& $esc $root)'")
    [void]$bootParts.Add("Set-Location -LiteralPath '$(& $esc $wd)'")

    if ($Strand -ne 'none') {
        $strandMod = Join-Path $root 'ops\shell\TriWeavon.StrandShell.psm1'
        if (Test-Path -LiteralPath $strandMod) {
            [void]$bootParts.Add("Import-Module '$(& $esc $strandMod)' -Force")
            [void]$bootParts.Add("Set-TriWeavonStrand -Strand $Strand")
        } else {
            [void]$bootParts.Add("`$env:CTWFI_STRAND = '$Strand'")
            [void]$bootParts.Add("Write-Host 'StrandShell.psm1 missing — CTWFI_STRAND=$Strand only' -ForegroundColor DarkYellow")
        }
    }

    if ($Command) {
        [void]$bootParts.Add($Command)
    }

    $boot = ($bootParts -join '; ')

    $pwshArgs = [System.Collections.Generic.List[string]]::new()
    if ($NoProfile) { [void]$pwshArgs.Add('-NoProfile') }
    [void]$pwshArgs.Add('-NoExit')
    if ($boot) {
        [void]$pwshArgs.Add('-Command')
        [void]$pwshArgs.Add($boot)
    }

    $wt = Get-Command wt -ErrorAction SilentlyContinue
    if ($wt) {
        # wt CLI: -w -1 = new window; -w 0 = current window; nt = new-tab
        $wtArgs = [System.Collections.Generic.List[string]]::new()
        if ($Layout -eq 'window') {
            foreach ($x in @('-w', '-1')) { [void]$wtArgs.Add($x) }
        } else {
            foreach ($x in @('-w', '0')) { [void]$wtArgs.Add($x) }
        }
        [void]$wtArgs.Add('nt')
        foreach ($x in @('--title', $Title, '-d', $wd, $pwsh)) { [void]$wtArgs.Add($x) }
        foreach ($a in $pwshArgs) { [void]$wtArgs.Add($a) }

        Write-Host ("Open-LogOSConsole  layout={0}  strand={1}  via=wt" -f $Layout, $Strand) -ForegroundColor Cyan
        Write-Host ("  title={0}" -f $Title) -ForegroundColor DarkGray
        Write-Host ("  cwd={0}" -f $wd) -ForegroundColor DarkGray
        if ($WhatIf) {
            Write-Host ("  WhatIf: wt {0}" -f ($wtArgs -join ' ')) -ForegroundColor DarkYellow
            return [pscustomobject]@{ Engine = 'wt'; Args = @($wtArgs); WorkingDirectory = $wd; Strand = $Strand }
        }
        # Start-Process so this shell is not replaced
        Start-Process -FilePath $wt.Source -ArgumentList @($wtArgs)
        return [pscustomobject]@{ Engine = 'wt'; Layout = $Layout; WorkingDirectory = $wd; Strand = $Strand; Title = $Title }
    }

    # Fallback: new pwsh process (console subsystem)
    Write-Host ("Open-LogOSConsole  layout={0}  strand={1}  via=Start-Process pwsh (wt not found)" -f $Layout, $Strand) -ForegroundColor Cyan
    if ($WhatIf) {
        Write-Host ("  WhatIf: {0} {1}" -f $pwsh, ($pwshArgs -join ' ')) -ForegroundColor DarkYellow
        return [pscustomobject]@{ Engine = 'pwsh'; Args = @($pwshArgs); WorkingDirectory = $wd; Strand = $Strand }
    }
    Start-Process -FilePath $pwsh -WorkingDirectory $wd -ArgumentList @($pwshArgs)
    return [pscustomobject]@{ Engine = 'pwsh'; Layout = 'window'; WorkingDirectory = $wd; Strand = $Strand; Title = $Title }
}

# ─── Dynamic terminal display ────────────────────────────────────────────────

function Start-LogOSDynamicTerminal {
    <#
    .SYNOPSIS
        Open HTML dynamic terminal, logos-tui / sensors, or pop a real OS console.
    .PARAMETER Mode
        html | tui | sensors | serve | pop | window | tab
        pop/window → new OS window; tab → new WT tab (same as logos-pop)
    #>
    [CmdletBinding()]
    param(
        [ValidateSet('html', 'tui', 'sensors', 'serve', 'pop', 'window', 'tab')]
        [string]$Mode = 'html',
        [int]$Port = 8788,
        [ValidateSet('none', 'grok', 'claude', 'gemini')]
        [string]$Strand = 'none',
        [string]$Command,
        [string]$Title
    )
    $root = Resolve-LogOSWindowsRoot
    $term = Join-Path $root 'coherence-mcp\coherence-site\public\terminal\index.html'

    switch ($Mode) {
        'html' {
            if (-not (Test-Path $term)) {
                throw "Dynamic terminal missing: $term — run Align-LogOSWindowsAxis first"
            }
            Start-Process (Resolve-Path $term).Path
            Write-Host "Opened $term" -ForegroundColor Cyan
            Write-Host 'Tip: logos-pop  |  logos-terminal pop  → real OS console' -ForegroundColor DarkGray
        }
        'sensors' {
            $mod = Join-Path $root 'ops\TriWeavon.Unitary.Profile.psm1'
            if (Test-Path $mod) {
                Import-Module $mod -Force
                Show-TriWeavonSensorBoard -Force
            } else {
                Invoke-LogOSPreflight
            }
        }
        'tui' {
            # Broken sccache RUSTC_WRAPPER is a known host trap (os error 2).
            if (Get-Command Repair-LogOSRustcWrapper -ErrorAction SilentlyContinue) {
                Repair-LogOSRustcWrapper | Out-Null
            } elseif ($env:RUSTC_WRAPPER -and -not (Test-Path -LiteralPath ($env:RUSTC_WRAPPER.Trim().Trim('"')))) {
                $env:RUSTC_WRAPPER = ''
            }
            if (Get-Command Start-LogOSTui -ErrorAction SilentlyContinue) {
                Start-LogOSTui
            } else {
                Push-Location $root
                try { cargo run -p reson8-tui } finally { Pop-Location }
            }
        }
        'serve' {
            $pub = Join-Path $root 'coherence-mcp\coherence-site\public'
            Write-Host "Serving $pub on http://127.0.0.1:$Port/terminal/" -ForegroundColor Cyan
            if (Get-Command npx -ErrorAction SilentlyContinue) {
                Push-Location $pub
                try { npx --yes serve -l $Port } finally { Pop-Location }
            } else {
                # Fallback: static file open
                Start-Process (Resolve-Path $term).Path
            }
        }
        { $_ -in @('pop', 'window') } {
            Open-LogOSConsole -Layout window -Strand $Strand -Command $Command -Title $Title
        }
        'tab' {
            Open-LogOSConsole -Layout tab -Strand $Strand -Command $Command -Title $Title
        }
    }
}

# ─── GB-06 deploy (HITL) ─────────────────────────────────────────────────────

function Invoke-GB06Deploy {
    <#
    .SYNOPSIS
        Windows wrapper for ops/gb06-deploy.sh via WSL or gcloud on host.
    .NOTES
        HITL auth gate: requires -Force. Never embeds secrets.
    #>
    [CmdletBinding(SupportsShouldProcess)]
    param(
        [string]$Project = $env:GOOGLE_CLOUD_PROJECT,
        [string]$Region = 'australia-southeast1',
        [string]$Service = 'reson8-waist',
        [string]$ImageDigest = 'sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b',
        [string]$ArRepo = 'reson8',
        [switch]$Force,
        [switch]$UseWsl
    )

    if (-not $Force) {
        Write-Host 'HITL GATE: Cloud Run deploy requires explicit -Force' -ForegroundColor Yellow
        Write-Host '  Example: gb-deploy -Project YOUR_PROJECT -Force' -ForegroundColor Cyan
        Write-Host '  Never pass keys here; use gcloud auth login / ADC.' -ForegroundColor DarkGray
        return
    }
    if (-not $Project) {
        throw 'Set -Project or $env:GOOGLE_CLOUD_PROJECT'
    }

    $hostName = "$Region-docker.pkg.dev"
    $image = "$hostName/$Project/$ArRepo/reson8-waist@$ImageDigest"

    Write-Host 'GB-06 deploy (Windows axis)' -ForegroundColor Cyan
    Write-Host "  project=$Project region=$Region"
    Write-Host "  image=$image"
    Write-Host '  allow-unauthenticated=false (IAM invoker only)'

    if ($UseWsl -or -not (Get-Command gcloud -ErrorAction SilentlyContinue)) {
        $bash = "cd ~/LogOS && PROJECT='$Project' REGION='$Region' SERVICE='$Service' IMAGE_DIGEST='$ImageDigest' AR_REPO='$ArRepo' bash ops/gb06-deploy.sh"
        if ($PSCmdlet.ShouldProcess($image, 'WSL gb06-deploy')) {
            wsl -- bash -lc $bash
        }
        return
    }

    if ($PSCmdlet.ShouldProcess($image, 'gcloud run deploy')) {
        gcloud auth configure-docker $hostName --quiet
        gcloud run deploy $Service `
            --project=$Project `
            --region=$Region `
            --image=$image `
            --no-allow-unauthenticated `
            --port=8080 `
            --min-instances=0 `
            --cpu=1 `
            --memory=512Mi `
            --set-env-vars='PORT=8080,SCHEMAS_DIR=/schemas'
    }
    Write-Host 'Smoke with identity token:' -ForegroundColor DarkGray
    Write-Host '  $TOKEN = gcloud auth print-identity-token'
    Write-Host '  curl -H "Authorization: Bearer $TOKEN" https://SERVICE_URL/health'
}

# ─── Axis align (profiles + env + optional packages) ─────────────────────────

function Set-LogOSWindowsAxis {
    <#
    .SYNOPSIS
        Idempotent: fix LOGOS_ROOT, profile hooks, install wrangler if missing.
    .NOTES
        Alias: logos-align, Align-LogOSWindowsAxis (compat)
    #>
    [CmdletBinding()]
    param(
        [switch]$InstallWrangler,
        [switch]$SkipProfile,
        [switch]$Quiet
    )

    $root = Resolve-LogOSWindowsRoot
    $env:LOGOS_ROOT = $root
    [Environment]::SetEnvironmentVariable('LOGOS_ROOT', $root, 'User')
    [Environment]::SetEnvironmentVariable('CTWFI_INVARIANT', 'alpha+omega=15', 'User')

    # Fix AllHosts profile stale C: LOGOS_ROOT (prefer F: Beelink)
    $allHosts = $PROFILE.CurrentUserAllHosts
    if ($allHosts -and (Test-Path -LiteralPath $allHosts)) {
        $raw = Get-Content -LiteralPath $allHosts -Raw
        # Stale same-user C: tree when a preferred non-C: root exists
        $stale = if ($env:USERNAME -and $env:SystemDrive) {
            Join-Path $env:SystemDrive "Users\$env:USERNAME\LogOS"
        } else { $null }
        if ($raw.Contains($stale)) {
            $fixed = $raw.Replace($stale, $root)
            $bak = "$allHosts.bak-axis-$(Get-Date -Format 'yyyyMMddHHmmss')"
            Copy-Item -LiteralPath $allHosts -Destination $bak -Force
            $utf8 = New-Object System.Text.UTF8Encoding $false
            [System.IO.File]::WriteAllText($allHosts, $fixed, $utf8)
            if (-not $Quiet) { Write-Host "Fixed stale LOGOS_ROOT in $allHosts (backup $bak)" -ForegroundColor Green }
        }
    }

    if (-not $SkipProfile) {
        $installShell = Join-Path $root 'ops\Install-LogOSShell.ps1'
        $installUnitary = Join-Path $root 'ops\Install-TriWeavonUnitaryProfile.ps1'
        if (Test-Path $installShell) {
            & $installShell -SkipUserEnv -Quiet
        }
        if (Test-Path $installUnitary) {
            # Install unitary into active $PROFILE and local Documents profile
            $targets = @(
                $PROFILE
                (Join-Path $env:USERPROFILE 'Documents\PowerShell\Microsoft.PowerShell_profile.ps1')
                (Join-Path $env:USERPROFILE 'OneDrive\Documents\PowerShell\Microsoft.PowerShell_profile.ps1')
            ) | Select-Object -Unique
            foreach ($tp in $targets) {
                if (-not $tp) { continue }
                try {
                    $parent = Split-Path $tp -Parent
                    if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }
                    $env:LOGOS_ROOT = $root
                    # Temporarily set PROFILE for installer semantics
                    & pwsh -NoProfile -File $installUnitary -LogOSRoot $root 2>$null
                    break
                } catch {
                    Write-Warning "Unitary install skip $tp : $_"
                }
            }
            # Ensure local Documents host profile also has unitary (OneDrive may be primary)
            foreach ($tp in $targets) {
                if (-not $tp -or -not (Test-Path (Split-Path $tp -Parent))) { continue }
                try {
                    if (-not (Test-Path $tp)) {
                        Set-Content -LiteralPath $tp -Value "# PowerShell profile`n" -Encoding utf8
                    }
                    $markerB = '# >>> TriWeavon.Unitary >>>'
                    $markerE = '# <<< TriWeavon.Unitary <<<'
                    $mod = (Join-Path $root 'ops\TriWeavon.Unitary.Profile.psm1').Replace("'", "''")
                    $rEsc = $root.Replace("'", "''")
                    $block = @"
$markerB
# Auto-wired by Set-LogOSWindowsAxis — unitary cockpit
`$env:LOGOS_ROOT = '$rEsc'
Import-Module '$mod' -Force
Start-TriWeavonUnitary
$markerE
"@
                    $existing = Get-Content -LiteralPath $tp -Raw -ErrorAction SilentlyContinue
                    if ($null -eq $existing) { $existing = '' }
                    if ($existing -match [regex]::Escape($markerB)) {
                        $existing = [regex]::Replace(
                            $existing,
                            [regex]::Escape($markerB) + '[\s\S]*?' + [regex]::Escape($markerE),
                            $block.TrimEnd(),
                            1
                        )
                    } else {
                        if ($existing.Length -gt 0 -and -not $existing.EndsWith("`n")) { $existing += "`n" }
                        $existing += "`n$block`n"
                    }
                    $utf8 = New-Object System.Text.UTF8Encoding $false
                    [System.IO.File]::WriteAllText($tp, $existing, $utf8)
                    if (-not $Quiet) { Write-Host "Unitary hook → $tp" -ForegroundColor Green }
                } catch {
                    Write-Warning "Could not write $tp : $($_.Exception.Message)"
                }
            }
        }
    }

    if ($InstallWrangler) {
        if (-not (Get-Command wrangler -ErrorAction SilentlyContinue)) {
            if (Get-Command npm -ErrorAction SilentlyContinue) {
                Write-Host 'Installing wrangler globally via npm…' -ForegroundColor Cyan
                npm install -g wrangler
            } else {
                Write-Warning 'npm not found — skip wrangler install'
            }
        } else {
            if (-not $Quiet) { Write-Host 'wrangler already on PATH' -ForegroundColor DarkGray }
        }
    }

    # Ensure dynamic terminal page exists
    $termDir = Join-Path $root 'coherence-mcp\coherence-site\public\terminal'
    $termHtml = Join-Path $termDir 'index.html'
    if (-not (Test-Path $termHtml)) {
        if (-not (Test-Path $termDir)) { New-Item -ItemType Directory -Path $termDir -Force | Out-Null }
        Write-Warning "terminal/index.html missing — expected Align artifacts"
    }

    if (-not $Quiet) {
        Write-Host ''
        Write-Host "  Axis aligned  LOGOS_ROOT=$root  v$script:WindowsAxisVersion" -ForegroundColor Green
        Write-Host '  Next: . $PROFILE   |   logos-preflight   |   logos-clean   |   tw   |   logos-pop' -ForegroundColor Cyan
        Write-Host ''
    }
    Invoke-LogOSPreflight -Quiet:$Quiet
}

# ─── Repo hygiene (logos-clean) ───────────────────────────────────────────────
# Design: docs/ops/LOGOS-CLEAN-DESIGN-2026-08-07.md
# Default = scan only. Apply always requires -Force (deep hygiene HITL).

function Get-LogOSPathBytes {
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) { return [int64]0 }
    try {
        $item = Get-Item -LiteralPath $Path -Force -ErrorAction Stop
        if ($item.PSIsContainer) {
            $fso = New-Object -ComObject Scripting.FileSystemObject
            return [int64]$fso.GetFolder($item.FullName).Size
        }
        return [int64]$item.Length
    } catch {
        $sum = [int64]0
        Get-ChildItem -LiteralPath $Path -Recurse -Force -File -ErrorAction SilentlyContinue |
            ForEach-Object { $sum += $_.Length }
        return $sum
    }
}

function Get-LogOSVolumeFreeBytes {
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Path)
    try {
        $full = (Resolve-Path -LiteralPath $Path -ErrorAction Stop).Path
        $rootPath = [System.IO.Path]::GetPathRoot($full)
        $drive = Get-PSDrive -PSProvider FileSystem -ErrorAction SilentlyContinue |
            Where-Object { $_.Root -eq $rootPath -or ($_.Name + ':\') -eq $rootPath } |
            Select-Object -First 1
        if ($drive) { return [int64]$drive.Free }
    } catch { }
    return [int64]0
}

function Get-LogOSPythonCacheTargets {
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$Root)
    $skipNames = [System.Collections.Generic.HashSet[string]]::new(
        [string[]]@('.git', 'target', 'node_modules', '.venv', 'venv2', 'venv-ctfwi',
            '.tmp.driveupload', '.gitnexus', 'vendor', 'agda'),
        [StringComparer]::OrdinalIgnoreCase
    )
    $found = [System.Collections.Generic.List[string]]::new()
    $stack = [System.Collections.Generic.Stack[string]]::new()
    $stack.Push($Root)
    $maxHits = 400
    while ($stack.Count -gt 0 -and $found.Count -lt $maxHits) {
        $dir = $stack.Pop()
        try {
            foreach ($child in [System.IO.Directory]::EnumerateDirectories($dir)) {
                $name = [System.IO.Path]::GetFileName($child)
                if ($skipNames.Contains($name)) { continue }
                if ($name -eq '__pycache__') {
                    $found.Add($child)
                    continue
                }
                $stack.Push($child)
            }
        } catch { }
    }
    return @($found)
}

function Invoke-LogOSClean {
    <#
    .SYNOPSIS
        Scan (default) or reclaim LogOS regenerable / temp / orphan bulk.
    .DESCRIPTION
        Deep-hygiene classes under LOGOS_ROOT. Scan is default.
        -Apply requires -Force. Never removes .venv. See docs/ops/LOGOS-CLEAN-DESIGN-2026-08-07.md.
    .EXAMPLE
        logos-clean
    .EXAMPLE
        logos-clean -Class DriveTmp,Cargo -Apply -Force
    .EXAMPLE
        logos-clean -Class Deep -Apply -Force -Json
    #>
    [CmdletBinding()]
    param(
        [ValidateSet(
            'DriveTmp', 'Cargo', 'PythonCache', 'NodeModules', 'BuildCaches',
            'GitPrune', 'VenvOrphan', 'Deep', 'All'
        )]
        [string[]]$Class = @('All'),
        [switch]$Apply,
        [switch]$Force,
        [switch]$Json,
        [switch]$Quiet
    )

    $root = Resolve-LogOSWindowsRoot
    $env:LOGOS_ROOT = $root

    $allClasses = @(
        'DriveTmp', 'Cargo', 'PythonCache', 'NodeModules',
        'BuildCaches', 'GitPrune', 'VenvOrphan'
    )
    $deepClasses = @('GitPrune', 'VenvOrphan')
    $selected = [System.Collections.Generic.List[string]]::new()
    foreach ($c in $Class) {
        if ($c -eq 'All' -or $c -eq 'Deep') {
            foreach ($a in $allClasses) { if (-not $selected.Contains($a)) { $selected.Add($a) } }
        } else {
            if (-not $selected.Contains($c)) { $selected.Add($c) }
        }
    }

    if ($Apply -and -not $Force) {
        throw 'logos-clean -Apply requires -Force (HITL). Run without -Apply for scan-only board.'
    }

    $freeBefore = Get-LogOSVolumeFreeBytes -Path $root
    $dirty = 0
    if (Get-Command git -ErrorAction SilentlyContinue) {
        Push-Location $root
        try { $dirty = @((git status --porcelain 2>$null)).Count } finally { Pop-Location }
    }

    $rows = [System.Collections.Generic.List[object]]::new()
    # script-scoped accumulators (StrictMode-safe across class blocks)
    $script:reclaimableScan = [int64]0
    $script:reclaimedTotal = [int64]0

    # ── DriveTmp ────────────────────────────────────────────────────────────
    if ($selected -contains 'DriveTmp') {
        $p = Join-Path $root '.tmp.driveupload'
        $bytes = Get-LogOSPathBytes -Path $p
        $exists = Test-Path -LiteralPath $p
        $status = 'scanned'
        $notes = if ($exists) { '.tmp.driveupload (Drive staging)' } else { 'absent' }
        if ($Apply -and $exists -and $bytes -ge 0) {
            try {
                Remove-Item -LiteralPath $p -Recurse -Force -ErrorAction Stop
                $status = 'reclaimed'
                $script:reclaimedTotal += $bytes
                $notes = 'removed'
            } catch {
                $status = 'failed'
                $notes = $_.Exception.Message
            }
        } elseif ($Apply -and -not $exists) {
            $status = 'skipped'
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        $rows.Add([pscustomobject]@{
                class = 'DriveTmp'; paths = @($p); bytes = $bytes
                depth = 'safe'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── Cargo ───────────────────────────────────────────────────────────────
    if ($selected -contains 'Cargo') {
        $cargoPaths = @(
            (Join-Path $root 'target'),
            (Join-Path $root 'crates\target'),
            (Join-Path $root 'cutiles\target')
        ) | Where-Object { Test-Path -LiteralPath $_ }
        $cargoPaths = @($cargoPaths)
        $bytes = [int64]0
        foreach ($cp in $cargoPaths) { $bytes += Get-LogOSPathBytes -Path $cp }
        $status = 'scanned'
        $notes = if (@($cargoPaths).Count -gt 0) { ($cargoPaths | ForEach-Object { $_.Replace($root + '\', '') }) -join ', ' } else { 'no target dirs' }
        if ($Apply -and @($cargoPaths).Count -gt 0) {
            try {
                $cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
                if ($cargoCmd -and (Test-Path -LiteralPath (Join-Path $root 'Cargo.toml'))) {
                    Push-Location $root
                    try {
                        & cargo clean 2>&1 | Out-Null
                    } finally { Pop-Location }
                }
                foreach ($cp in $cargoPaths) {
                    if (Test-Path -LiteralPath $cp) {
                        Remove-Item -LiteralPath $cp -Recurse -Force -ErrorAction SilentlyContinue
                    }
                }
                # residual nested targets + root target (cargo clean can leave stubs)
                foreach ($extra in @('target', 'crates\target', 'cutiles\target')) {
                    $ep = Join-Path $root $extra
                    if (Test-Path -LiteralPath $ep) {
                        cmd /c "rmdir /s /q `"$ep`"" 2>$null | Out-Null
                        if (Test-Path -LiteralPath $ep) {
                            Remove-Item -LiteralPath $ep -Recurse -Force -ErrorAction SilentlyContinue
                        }
                    }
                }
                $still = @(@('target', 'crates\target', 'cutiles\target') | ForEach-Object { Join-Path $root $_ } | Where-Object { Test-Path -LiteralPath $_ })
                if (@($still).Count -gt 0) {
                    $status = 'failed'
                    $notes = "partial; still present: $($still -join ', ')"
                } else {
                    $status = 'reclaimed'
                    $script:reclaimedTotal += $bytes
                    $notes = 'cargo clean + target dirs removed'
                }
            } catch {
                $status = 'failed'
                $notes = $_.Exception.Message
            }
        } elseif ($Apply -and @($cargoPaths).Count -eq 0) {
            $status = 'skipped'
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        $rows.Add([pscustomobject]@{
                class = 'Cargo'; paths = @($cargoPaths); bytes = $bytes
                depth = 'safe'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── PythonCache ─────────────────────────────────────────────────────────
    if ($selected -contains 'PythonCache') {
        $pcPaths = @(Get-LogOSPythonCacheTargets -Root $root)
        $bytes = [int64]0
        foreach ($pp in $pcPaths) { $bytes += Get-LogOSPathBytes -Path $pp }
        $status = 'scanned'
        $notes = "$(@($pcPaths).Count) __pycache__ dirs (capped walk)"
        if ($Apply -and @($pcPaths).Count -gt 0) {
            $failed = 0
            foreach ($pp in $pcPaths) {
                try {
                    Remove-Item -LiteralPath $pp -Recurse -Force -ErrorAction Stop
                } catch { $failed++ }
            }
            if ($failed -eq 0) {
                $status = 'reclaimed'
                $script:reclaimedTotal += $bytes
                $notes = "removed $(@($pcPaths).Count) dirs"
            } else {
                $status = 'failed'
                $notes = "removed with $failed failures"
            }
        } elseif ($Apply) {
            $status = 'skipped'
            $notes = 'none found'
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        $rows.Add([pscustomobject]@{
                class = 'PythonCache'; paths = @($pcPaths | Select-Object -First 12); bytes = $bytes
                depth = 'safe'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── NodeModules (allowlist) ─────────────────────────────────────────────
    if ($selected -contains 'NodeModules') {
        $allow = @(
            'ops\mcp\logos-residual-zero\node_modules'
        )
        $nmPaths = @()
        $bytes = [int64]0
        foreach ($rel in $allow) {
            $fp = Join-Path $root $rel
            if (Test-Path -LiteralPath $fp) {
                $nmPaths += $fp
                $bytes += Get-LogOSPathBytes -Path $fp
            }
        }
        $status = 'scanned'
        $nmPaths = @($nmPaths)
        $notes = if (@($nmPaths).Count -gt 0) { 'allowlisted only' } else { 'no allowlisted node_modules' }
        if ($Apply -and @($nmPaths).Count -gt 0) {
            try {
                foreach ($np in $nmPaths) {
                    Remove-Item -LiteralPath $np -Recurse -Force -ErrorAction Stop
                }
                $status = 'reclaimed'
                $script:reclaimedTotal += $bytes
                $notes = 'allowlist removed'
            } catch {
                $status = 'failed'
                $notes = $_.Exception.Message
            }
        } elseif ($Apply) {
            $status = 'skipped'
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        $rows.Add([pscustomobject]@{
                class = 'NodeModules'; paths = @($nmPaths); bytes = $bytes
                depth = 'safe'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── BuildCaches ─────────────────────────────────────────────────────────
    if ($selected -contains 'BuildCaches') {
        $bcPaths = [System.Collections.Generic.List[string]]::new()
        $lake = Join-Path $root 'lean\.lake'
        if (Test-Path -LiteralPath $lake) { $bcPaths.Add($lake) }
        $agdaRoot = Join-Path $root 'agda'
        if (Test-Path -LiteralPath $agdaRoot) {
            Get-ChildItem -LiteralPath $agdaRoot -Filter '*.agdai' -Recurse -File -Force -ErrorAction SilentlyContinue |
                ForEach-Object { $bcPaths.Add($_.FullName) }
            Get-ChildItem -LiteralPath $agdaRoot -Directory -Filter 'MAlonzo' -Recurse -Force -ErrorAction SilentlyContinue |
                ForEach-Object { $bcPaths.Add($_.FullName) }
        }
        $bytes = [int64]0
        foreach ($bp in $bcPaths) { $bytes += Get-LogOSPathBytes -Path $bp }
        $status = 'scanned'
        $notes = "$(@($bcPaths).Count) cache paths (lean .lake / agda)"
        if ($Apply -and @($bcPaths).Count -gt 0) {
            try {
                foreach ($bp in $bcPaths) {
                    if (Test-Path -LiteralPath $bp) {
                        Remove-Item -LiteralPath $bp -Recurse -Force -ErrorAction SilentlyContinue
                    }
                }
                $status = 'reclaimed'
                $script:reclaimedTotal += $bytes
                $notes = 'build caches removed'
            } catch {
                $status = 'failed'
                $notes = $_.Exception.Message
            }
        } elseif ($Apply) {
            $status = 'skipped'
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        $rows.Add([pscustomobject]@{
                class = 'BuildCaches'; paths = @($bcPaths | Select-Object -First 8); bytes = $bytes
                depth = 'safe'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── GitPrune (deep) ─────────────────────────────────────────────────────
    if ($selected -contains 'GitPrune') {
        $status = 'scanned'
        $notes = 'git worktree prune + git gc --prune=now (no rewrite)'
        $bytes = [int64]0
        $gitDir = Join-Path $root '.git'
        if (Test-Path -LiteralPath $gitDir) {
            # estimate only: not reclaimable until apply; report 0 for scan estimate
            $notes = 'deep: run -Apply -Force to prune worktrees + gc'
        } else {
            $status = 'skipped'
            $notes = 'no .git'
        }
        if ($Apply -and (Test-Path -LiteralPath $gitDir)) {
            try {
                Push-Location $root
                try {
                    $beforeGit = Get-LogOSPathBytes -Path $gitDir
                    git worktree prune 2>&1 | Out-Null
                    git gc --prune=now 2>&1 | Out-Null
                    $afterGit = Get-LogOSPathBytes -Path $gitDir
                    $delta = [Math]::Max([int64]0, $beforeGit - $afterGit)
                    $bytes = $delta
                    $script:reclaimedTotal += $delta
                    $status = 'reclaimed'
                    $notes = "worktree prune + gc; freed ~$([math]::Round($delta/1MB,1)) MB"
                } finally { Pop-Location }
            } catch {
                $status = 'failed'
                $notes = $_.Exception.Message
            }
        }
        $rows.Add([pscustomobject]@{
                class = 'GitPrune'; paths = @((Join-Path $root '.git')); bytes = $bytes
                depth = 'deep'; status = $status; notes = $notes
            }) | Out-Null
    }

    # ── VenvOrphan (deep) ───────────────────────────────────────────────────
    if ($selected -contains 'VenvOrphan') {
        $preferred = Get-LogOSPython -Root $root
        $candidates = @('venv2', 'venv-ctfwi')
        $voPaths = [System.Collections.Generic.List[string]]::new()
        $bytes = [int64]0
        $blocked = [System.Collections.Generic.List[string]]::new()
        foreach ($name in $candidates) {
            $vp = Join-Path $root $name
            if (-not (Test-Path -LiteralPath $vp)) { continue }
            $sz = Get-LogOSPathBytes -Path $vp
            if ($preferred -and $preferred.StartsWith($vp, [System.StringComparison]::OrdinalIgnoreCase)) {
                $blocked.Add($name)
                continue
            }
            $voPaths.Add($vp)
            $bytes += $sz
        }
        $canonicalVenv = Join-Path $root '.venv'
        $status = 'scanned'
        $notes = "orphan candidates; .venv protected$(if (@($blocked).Count -gt 0) { '; blocked: ' + ($blocked -join ',') })"
        if ($Apply) {
            if (@($voPaths).Count -eq 0) {
                $status = 'skipped'
                $notes = 'no orphan venvs (or all protected)'
            } else {
                try {
                    foreach ($vp in $voPaths) {
                        Remove-Item -LiteralPath $vp -Recurse -Force -ErrorAction Stop
                    }
                    $status = 'reclaimed'
                    $script:reclaimedTotal += $bytes
                    $notes = 'removed orphan venvs; .venv kept'
                } catch {
                    $status = 'failed'
                    $notes = $_.Exception.Message
                }
            }
        }
        if ($status -eq 'scanned') { $script:reclaimableScan += $bytes }
        # ensure we never list .venv as a target path
        $rows.Add([pscustomobject]@{
                class = 'VenvOrphan'
                paths = @($voPaths + @("PROTECTED:$canonicalVenv"))
                bytes = $bytes
                depth = 'deep'
                status = $status
                notes = $notes
            }) | Out-Null
    }

    $reclaimable = $script:reclaimableScan
    $reclaimed = $script:reclaimedTotal
    $freeAfter = Get-LogOSVolumeFreeBytes -Path $root

    $report = [pscustomobject]@{
        root               = $root
        mode               = $(if ($Apply) { 'apply' } else { 'scan' })
        force              = [bool]$Force
        classes            = @($rows)
        reclaimable_bytes  = $reclaimable
        reclaimed_bytes    = $reclaimed
        free_before_bytes  = $freeBefore
        free_after_bytes   = $freeAfter
        git_dirty          = $dirty
        axis_version       = $script:WindowsAxisVersion
        design             = 'docs/ops/LOGOS-CLEAN-DESIGN-2026-08-07.md'
    }

    if ($Json) {
        $report | ConvertTo-Json -Depth 6
    } elseif (-not $Quiet) {
        Write-Host ''
        Write-Host "  logos-clean  mode=$($report.mode)  LOGOS_ROOT=$root" -ForegroundColor Cyan
        if ($dirty -gt 0) {
            Write-Host "  WARN git dirty files: $dirty (clean does not touch tracked source)" -ForegroundColor Yellow
        }
        Write-Host ('  {0,-14} {1,10} {2,-10} {3,-10} {4}' -f 'CLASS', 'SIZE_MB', 'DEPTH', 'STATUS', 'NOTES') -ForegroundColor DarkGray
        foreach ($r in $rows) {
            $mb = [math]::Round($r.bytes / 1MB, 1)
            $color = switch ($r.status) {
                'reclaimed' { 'Green' }
                'failed' { 'Red' }
                'skipped' { 'DarkGray' }
                default { 'White' }
            }
            Write-Host ('  {0,-14} {1,10:N1} {2,-10} {3,-10} {4}' -f $r.class, $mb, $r.depth, $r.status, $r.notes) -ForegroundColor $color
        }
        Write-Host ''
        Write-Host ("  Reclaimable: {0:N1} MB   Reclaimed: {1:N1} MB" -f ($reclaimable / 1MB), ($reclaimed / 1MB)) -ForegroundColor Cyan
        Write-Host ("  Free on volume: {0:N1} GB → {1:N1} GB" -f ($freeBefore / 1GB), ($freeAfter / 1GB)) -ForegroundColor DarkGray
        if (-not $Apply) {
            Write-Host '  Apply: logos-clean -Class DriveTmp,Cargo -Apply -Force' -ForegroundColor Yellow
            Write-Host '         logos-clean -Class Deep -Apply -Force   # all classes' -ForegroundColor Yellow
        }
        Write-Host ''
    }

    return $report
}

# ─── Aliases ─────────────────────────────────────────────────────────────────
# Module-scoped aliases (no -Scope Global): Export-ModuleMember can export them.
# Global-scoped aliases previously survived module unload → orphan logos-wrangler
# pointing at missing Invoke-LogOSWrangler.

# Compat alias for older docs / scripts
Set-Alias -Name Align-LogOSWindowsAxis -Value Set-LogOSWindowsAxis -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-preflight -Value Invoke-LogOSPreflight -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-align -Value Set-LogOSWindowsAxis -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-clean -Value Invoke-LogOSClean -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-wrangler -Value Invoke-LogOSWrangler -Force -ErrorAction SilentlyContinue

function Invoke-LogOSNet {
    <#
    .SYNOPSIS
        Selective net proxy stack (gaming clearnet vs privacy Tor/i2pd).
    .EXAMPLE
        logos-net
        logos-net -Action start-gaming
        logos-net -Action start-privacy
        logos-net -Action optimize
        logos-net -Action activate
    #>
    [CmdletBinding()]
    param(
        [ValidateSet('status', 'start-privacy', 'start-gaming', 'stop', 'install', 'optimize', 'activate')]
        [string]$Action = 'status',
        [switch]$Json
    )
    $root = Resolve-LogOSWindowsRoot
    $script = Join-Path $root 'ops\net\LogOS.NetProxy.ps1'
    if (-not (Test-Path -LiteralPath $script)) {
        throw "Net proxy controller missing: $script"
    }
    $argList = @('-NoProfile', '-File', $script, '-Action', $Action)
    if ($Json) { $argList += '-Json' }
    & pwsh @argList

    # Apply lane proxy env to *this* session (child process env does not stick).
    $laneFile = Join-Path $root 'ops\net\data\active_lane.txt'
    $lane = if (Test-Path -LiteralPath $laneFile) {
        (Get-Content -LiteralPath $laneFile -Raw).Trim()
    } else {
        'unknown'
    }
    switch ($lane) {
        'privacy' {
            $env:ALL_PROXY = 'socks5://127.0.0.1:9050'
            $env:HTTP_PROXY = 'http://127.0.0.1:8118'
            $env:HTTPS_PROXY = 'http://127.0.0.1:8118'
            $env:NO_PROXY = 'localhost,127.0.0.1'
            Write-Host "  session env: privacy proxies set (ALL_PROXY/HTTP_PROXY)" -ForegroundColor DarkCyan
        }
        { $_ -in @('gaming', 'idle') } {
            foreach ($k in @('ALL_PROXY', 'HTTP_PROXY', 'HTTPS_PROXY', 'all_proxy', 'http_proxy', 'https_proxy')) {
                Remove-Item -Path "Env:$k" -ErrorAction SilentlyContinue
            }
            $env:NO_PROXY = '*'
            Write-Host "  session env: proxies cleared (lane=$lane)" -ForegroundColor DarkCyan
        }
    }
}
Set-Alias -Name logos-net -Value Invoke-LogOSNet -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-terminal -Value Start-LogOSDynamicTerminal -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-pop -Value Open-LogOSConsole -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-console -Value Open-LogOSConsole -Force -ErrorAction SilentlyContinue
Set-Alias -Name adhealth-preflight -Value Invoke-AdHealthPreflight -Force -ErrorAction SilentlyContinue
Set-Alias -Name adhealth-run -Value Invoke-AdHealthRun -Force -ErrorAction SilentlyContinue
Set-Alias -Name hup-preflight -Value Invoke-HupGuestPreflight -Force -ErrorAction SilentlyContinue
Set-Alias -Name gb-deploy -Value Invoke-GB06Deploy -Force -ErrorAction SilentlyContinue

Export-ModuleMember -Function @(
    'Resolve-LogOSWindowsRoot',
    'Get-LogOSPython',
    'Invoke-LogOSPreflight',
    'Invoke-LogOSClean',
    'Invoke-LogOSNet',
    'Invoke-AdHealthPreflight',
    'Invoke-AdHealthRun',
    'Invoke-HupGuestPreflight',
    'Get-CoherenceSiteRoot',
    'Invoke-LogOSWrangler',
    'Open-LogOSConsole',
    'Start-LogOSDynamicTerminal',
    'Invoke-GB06Deploy',
    'Set-LogOSWindowsAxis'
) -Alias @(
    'logos-preflight', 'logos-align', 'logos-clean', 'logos-net', 'logos-wrangler', 'logos-terminal',
    'logos-pop', 'logos-console',
    'adhealth-preflight', 'adhealth-run', 'hup-preflight', 'gb-deploy',
    'Align-LogOSWindowsAxis'
)
