#Requires -Version 5.1
<#
.SYNOPSIS
    LogOS shell bootstrap — Rust/Python/Agda/Lean/CUDA kernels on every shell open.
.DESCRIPTION
    Resolve F: Beelink LogOS as canonical root, prepend toolchain PATH segments,
    export formal-layer env vars, and surface convenience commands.

    Load via Install-LogOSShell.ps1 (writes $PROFILE hook) or:
      Import-Module 'F:\Users\Matthew Ruhnau\LogOS\ops\LogOS.Shell.psm1' -Force
      Initialize-LogOSShell
#>

Set-StrictMode -Version Latest

$script:LogOSShellVersion = '1.1.0'
$script:LogOSInitialized = $false
$script:LogOSCommandSurfacePath = $null

function Get-LogOSCandidateRoots {
    [CmdletBinding()]
    param()

    @(
        $env:LOGOS_ROOT
        'F:\Users\Matthew Ruhnau\LogOS'
        'C:\Users\Matthew Ruhnau\LogOS'
        (Join-Path $env:USERPROFILE 'LogOS')
        'G:\Reson8-Labs\LogOS'
        'G:\LogOS'
    ) | Where-Object { $_ -and (Test-Path -LiteralPath $_) } | Select-Object -Unique
}

function Resolve-LogOSRoot {
    <#
    .SYNOPSIS
        Prefer F:\ Beelink tree; ignore stale C: copy unless F: is missing.
    #>
    [CmdletBinding()]
    param()

    $candidates = @(Get-LogOSCandidateRoots)
    if (-not $candidates) {
        throw 'LogOS root not found. Set LOGOS_ROOT or clone to F:\Users\Matthew Ruhnau\LogOS'
    }

    $fPreferred = $candidates | Where-Object { $_ -like 'F:\*' } | Select-Object -First 1
    if ($fPreferred) { return (Resolve-Path -LiteralPath $fPreferred).Path }

    return (Resolve-Path -LiteralPath $candidates[0]).Path
}

function Add-LogOSPathEntry {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Directory,
        [switch]$Prepend
    )

    if (-not $Directory -or -not (Test-Path -LiteralPath $Directory)) { return $false }

    $parts = @($env:PATH -split ';' | Where-Object { $_ -and $_.Trim() })
    $norm = (Resolve-Path -LiteralPath $Directory).Path.TrimEnd('\')
    $exists = $parts | Where-Object { $_.TrimEnd('\') -ieq $norm }
    if ($exists) { return $true }

    if ($Prepend) {
        $env:PATH = "$norm;$env:PATH"
    } else {
        $env:PATH = "$env:PATH;$norm"
    }
    $true
}

function Find-LogOSCudaHome {
    [CmdletBinding()]
    param()

    if ($env:CUDA_PATH -and (Test-Path -LiteralPath $env:CUDA_PATH)) {
        return $env:CUDA_PATH
    }
    if ($env:CUDA_HOME -and (Test-Path -LiteralPath $env:CUDA_HOME)) {
        return $env:CUDA_HOME
    }

    $roots = @(
        'C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA',
        'F:\Program Files\NVIDIA GPU Computing Toolkit\CUDA',
        'C:\Program Files\NVIDIA Corporation\CUDA',
        'F:\CUDA',
        'C:\CUDA'
    )
    foreach ($root in $roots) {
        if (-not (Test-Path -LiteralPath $root)) { continue }
        $ver = Get-ChildItem -LiteralPath $root -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -match '^v?\d' } |
            Sort-Object Name -Descending |
            Select-Object -First 1
        if ($ver) { return $ver.FullName }
    }
    $null
}

function Find-LogOSAgda {
    [CmdletBinding()]
    param()

    $cmd = Get-Command agda -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }

    $hints = @(
        (Join-Path $env:USERPROFILE 'AppData\Roaming\cabal\bin\agda.exe')
        (Join-Path $env:USERPROFILE 'AppData\Roaming\local\bin\agda.exe')
        (Join-Path $env:USERPROFILE 'scoop\shims\agda.exe')
        'C:\ghcup\bin\agda.exe'
        (Join-Path $env:USERPROFILE 'AppData\Roaming\ghcup\bin\agda.exe')
        'C:\ProgramData\chocolatey\bin\agda.exe'
    )
    foreach ($h in $hints) {
        if (Test-Path -LiteralPath $h) { return $h }
    }
    $null
}

function Initialize-LogOSShell {
    <#
    .SYNOPSIS
        Wire PATH + env so crates, Agda, kernels, Lean, Python are ready on shell open.
    .PARAMETER Quiet
        Suppress banner.
    .PARAMETER SkipVenv
        Do not prepend LogOS .venv\Scripts.
    .PARAMETER SkipMsvc
        Do not import MSVC LIB/INCLUDE (slower; only needed for some Rust/CUDA builds).
    .PARAMETER ImportTriWeavon
        Also load ops\TriWeavon.Profile.psm1 (bridge/metrics).
    #>
    [CmdletBinding()]
    param(
        [switch]$Quiet,
        [switch]$SkipVenv,
        [switch]$SkipMsvc,
        [switch]$ImportTriWeavon,
        [switch]$Force
    )

    # Process-level guard: AllHosts + CurrentHost profiles both import this module.
    if (-not $Force -and ($script:LogOSInitialized -or $env:LOGOS_SHELL_INIT -eq '1')) {
        if (-not $Quiet -and -not $script:LogOSInitialized) {
            # Module re-imported (-Force) but process already wired — stay silent.
        }
        $script:LogOSInitialized = $true
        return Get-LogOSRoots
    }

    $root = Resolve-LogOSRoot
    $env:LOGOS_ROOT = $root
    $env:RESON8_LOGOS_ROOT = $root
    $env:CUTILE_ROOT = Join-Path $root 'cutiles\cutile'
    $env:AGDA_ROOT = Join-Path $root 'agda'
    $env:LEAN_ROOT = Join-Path $root 'lean'
    $env:KERNELS_ROOT = Join-Path $root 'kernels'
    $env:CRATES_ROOT = Join-Path $root 'crates'
    $env:LOGOS_OPS = Join-Path $root 'ops'
    $env:LOGOS_SURFACES = Join-Path $root 'docs\surfaces'
    $env:LOGOS_SITE_PUBLIC = Join-Path $root 'coherence-mcp\coherence-site\public'
    $env:LOGOS_COMMAND_SURFACE = Join-Path $root 'ops\command-surface.json'
    $script:LogOSCommandSurfacePath = $env:LOGOS_COMMAND_SURFACE
    $env:CTWFI_INVARIANT = 'alpha+omega=15'
    if (-not $env:CTWFI_STRAND) { $env:CTWFI_STRAND = 'reason' }
    if (-not $env:ATOM_LOG) {
        $env:ATOM_LOG = Join-Path $root 'ATOM\ATOM-trail.log'
    }
    if (-not $env:FORGE_WS_URL) { $env:FORGE_WS_URL = 'ws://127.0.0.1:8088' }

    $base = Split-Path $root -Parent
    if (-not $env:COHERENCE_MCP_ROOT) {
        # Prefer sibling repo with built server (F:\Users\...\coherence-mcp), else in-tree site.
        $sibling = Join-Path $base 'coherence-mcp'
        $nested = Join-Path $root 'coherence-mcp'
        if (Test-Path -LiteralPath (Join-Path $sibling 'build\index.js')) {
            $env:COHERENCE_MCP_ROOT = $sibling
        } elseif (Test-Path -LiteralPath $sibling) {
            $env:COHERENCE_MCP_ROOT = $sibling
        } elseif (Test-Path -LiteralPath $nested) {
            $env:COHERENCE_MCP_ROOT = $nested
        }
    }
    if (-not $env:RESON8_LABS_ROOT -and (Test-Path -LiteralPath (Join-Path $base 'reson8-Labs'))) {
        $env:RESON8_LABS_ROOT = Join-Path $base 'reson8-Labs'
    }

    # --- PATH: toolchains first ---
    $pathAdds = @(
        (Join-Path $env:USERPROFILE '.cargo\bin')
        (Join-Path $env:USERPROFILE '.elan\bin')
        'C:\Users\toolated\.cargo\bin'
        'C:\Users\toolated\.elan\bin'
        'F:\Users\Matthew Ruhnau\.cargo\bin'
        'F:\Users\Matthew Ruhnau\.elan\bin'
        'C:\Program Files\Rust stable MSVC 1.96\bin'
        (Join-Path $env:USERPROFILE 'AppData\Roaming\cabal\bin')
        (Join-Path $env:USERPROFILE 'AppData\Roaming\local\bin')
        (Join-Path $env:USERPROFILE 'AppData\Roaming\ghcup\bin')
        'C:\ghcup\bin'
        (Join-Path $root 'ops')
        (Join-Path $root 'agda\scripts')
        (Join-Path $root 'cutiles\cutile\scripts')
        (Join-Path $root 'tools')
    )

    if (-not $SkipVenv) {
        $venvScripts = Join-Path $root '.venv\Scripts'
        if (Test-Path -LiteralPath $venvScripts) {
            $pathAdds = @($venvScripts) + $pathAdds
            $env:VIRTUAL_ENV = Join-Path $root '.venv'
            $env:VIRTUAL_ENV_PROMPT = 'LogOS'
        }
    }

    $cudaHome = Find-LogOSCudaHome
    if ($cudaHome) {
        $env:CUDA_PATH = $cudaHome
        $env:CUDA_HOME = $cudaHome
        $pathAdds += @(
            (Join-Path $cudaHome 'bin')
            (Join-Path $cudaHome 'libnvvp')
        )
        $cudaLib = Join-Path $cudaHome 'lib\x64'
        if (Test-Path -LiteralPath $cudaLib) {
            if ($env:LIB) { $env:LIB = "$cudaLib;$env:LIB" } else { $env:LIB = $cudaLib }
        }
    }

    foreach ($p in $pathAdds) { Add-LogOSPathEntry -Directory $p -Prepend | Out-Null }

    $agdaExe = Find-LogOSAgda
    if ($agdaExe) {
        $env:AGDA = $agdaExe
        Add-LogOSPathEntry -Directory (Split-Path $agdaExe -Parent) -Prepend | Out-Null
    }

    # Agda libraries file for TriWeavon
    $agdaLib = Join-Path $root 'agda\TriWeavon.agda-lib'
    if (Test-Path -LiteralPath $agdaLib) {
        $env:AGDA_DIR = Join-Path $root 'agda'
        # libraries file discovery (Agda reads AGDA_DIR / --library-file)
        $libs = Join-Path $env:USERPROFILE 'AppData\Roaming\agda\libraries'
        if (-not (Test-Path -LiteralPath (Split-Path $libs -Parent))) {
            New-Item -ItemType Directory -Path (Split-Path $libs -Parent) -Force -ErrorAction SilentlyContinue | Out-Null
        }
    }

    if (-not $SkipMsvc) {
        $tri = Join-Path $root 'ops\TriWeavon.Profile.psm1'
        if (Test-Path -LiteralPath $tri) {
            Import-Module $tri -Force -ErrorAction SilentlyContinue
            if (Get-Command Import-MsvcBuildEnv -ErrorAction SilentlyContinue) {
                Import-MsvcBuildEnv | Out-Null
            }
        }
    }

    if ($ImportTriWeavon) {
        $tri = Join-Path $root 'ops\TriWeavon.Profile.psm1'
        if (Test-Path -LiteralPath $tri) {
            Import-Module $tri -Force -ErrorAction SilentlyContinue
            if (Get-Command Set-TriWeavonEnv -ErrorAction SilentlyContinue) {
                Set-TriWeavonEnv | Out-Null
            }
        }
    }

    # Windows preflight / wrangler / dynamic-terminal axis (idempotent soft load)
    $winAxis = Join-Path $root 'ops\LogOS.Windows.psm1'
    if (Test-Path -LiteralPath $winAxis) {
        Import-Module $winAxis -Force -ErrorAction SilentlyContinue
    }

    $script:LogOSInitialized = $true
    $env:LOGOS_SHELL_INIT = '1'
    if (-not $Quiet) { Show-LogOSBanner }
    Get-LogOSRoots
}

function Get-LogOSRoots {
    [CmdletBinding()]
    param()

    $root = if ($env:LOGOS_ROOT) { $env:LOGOS_ROOT } else { Resolve-LogOSRoot }
    [ordered]@{
        LogOS      = $root
        Crates     = Join-Path $root 'crates'
        Agda       = Join-Path $root 'agda'
        Lean       = Join-Path $root 'lean'
        Kernels    = Join-Path $root 'kernels'
        Cutile     = Join-Path $root 'cutiles\cutile'
        Ops        = Join-Path $root 'ops'
        Venv       = Join-Path $root '.venv'
        CudaHome   = $env:CUDA_PATH
        AgdaExe    = if ($env:AGDA) { $env:AGDA } else { Find-LogOSAgda }
    }
}

function Show-LogOSBanner {
    [CmdletBinding()]
    param()

    $t = Get-LogOSToolchain -AsObject
    $root = $env:LOGOS_ROOT
    $mcpOk = if ($env:COHERENCE_MCP_ROOT -and (Test-Path -LiteralPath (Join-Path $env:COHERENCE_MCP_ROOT 'build\index.js'))) {
        'OK'
    } elseif ($env:COHERENCE_MCP_ROOT) {
        'path'
    } else {
        '--'
    }
    $meta = Join-Path $root 'coherence-mcp\coherence-site\public\meta-map\index.html'
    $metaOk = if (Test-Path -LiteralPath $meta) { 'OK' } else { '--' }
    $nodeOk = if ($t.node) { 'OK' } else { '--' }

    Write-Host ''
    Write-Host "  LogOS shell v$script:LogOSShellVersion  |  α+ω=15  |  $root" -ForegroundColor Cyan
    Write-Host ("  cargo={0}  python={1}  lean={2}  lake={3}  agda={4}  nvcc={5}  wsl={6}" -f `
        $(if ($t.cargo) { 'OK' } else { '--' }),
        $(if ($t.python) { 'OK' } else { '--' }),
        $(if ($t.lean) { 'OK' } else { '--' }),
        $(if ($t.lake) { 'OK' } else { '--' }),
        $(if ($t.agda) { 'OK' } else { 'WSL?' }),
        $(if ($t.nvcc) { 'OK' } else { '--' }),
        $(if ($t.wsl) { 'OK' } else { '--' })
    ) -ForegroundColor DarkGray
    Write-Host ("  mcp={0}  node={1}  meta-map={2}  bridge={3}" -f `
        $mcpOk, $nodeOk, $metaOk,
        $(if ($env:FORGE_WS_URL) { $env:FORGE_WS_URL } else { 'ws://127.0.0.1:8088' })
    ) -ForegroundColor DarkGray
    Write-Host '  cmds: logos-status  logos-mcp  logos-tui  logos-site  logos-surfaces  logos-bridge' -ForegroundColor DarkGray
    Write-Host '        logos-agda  logos-lean  logos-kernels  logos-cargo  logos-barcode  logos-wsl' -ForegroundColor DarkGray
    Write-Host '        logos-preflight  logos-align  logos-wrangler  logos-terminal  logos-pop' -ForegroundColor DarkGray
    Write-Host '        logos-confidence  tw confidence  tw  (1-click confidence board)' -ForegroundColor DarkGray
    Write-Host ''
}

function Get-LogOSToolchain {
    <#
    .SYNOPSIS
        Probe whether formal + systems tools resolve on PATH.
    #>
    [CmdletBinding()]
    param([switch]$AsObject)

    $names = @('cargo', 'rustc', 'python', 'agda', 'lean', 'lake', 'elan', 'nvcc', 'wsl', 'node', 'npm')
    $map = [ordered]@{}
    foreach ($n in $names) {
        $c = Get-Command $n -ErrorAction SilentlyContinue
        $map[$n] = if ($c) { $c.Source } else { $null }
    }
    $map['LOGOS_ROOT'] = $env:LOGOS_ROOT
    $map['CUDA_PATH'] = $env:CUDA_PATH
    $map['VIRTUAL_ENV'] = $env:VIRTUAL_ENV

    if ($AsObject) { return [pscustomobject]$map }

    $map.GetEnumerator() | ForEach-Object {
        $mark = if ($_.Value) { '[OK]' } else { '[--]' }
        '{0} {1,-12} {2}' -f $mark, $_.Key, $_.Value
    }
}

function Enter-LogOS {
    param([string]$Sub = '')
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $target = if ($Sub) { Join-Path $env:LOGOS_ROOT $Sub } else { $env:LOGOS_ROOT }
    Set-Location -LiteralPath $target
}

function Enter-LogOSCrates { Enter-LogOS 'crates' }
function Enter-LogOSAgda { Enter-LogOS 'agda' }
function Enter-LogOSLean { Enter-LogOS 'lean' }
function Enter-LogOSKernels { Enter-LogOS 'kernels' }
function Enter-LogOSCutile { Enter-LogOS 'cutiles\cutile' }

function Invoke-LogOSCargo {
    <#
    .SYNOPSIS
        Run cargo from LOGOS_ROOT (workspace root).
    #>
    [CmdletBinding()]
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$CargoArgs
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error 'cargo not on PATH — install Rust or run Initialize-LogOSShell'
        return
    }
    Push-Location -LiteralPath $env:LOGOS_ROOT
    try {
        if ($CargoArgs -and $CargoArgs.Count -gt 0) {
            & cargo @CargoArgs
        } else {
            & cargo metadata --no-deps --format-version 1 2>$null | Out-Null
            Write-Host "LogOS workspace: $env:LOGOS_ROOT" -ForegroundColor Cyan
            & cargo metadata --no-deps --format-version 1 2>$null |
                ConvertFrom-Json |
                Select-Object -ExpandProperty packages |
                Select-Object -ExpandProperty name |
                Sort-Object
        }
    } finally {
        Pop-Location
    }
}

function Invoke-LogOSAgda {
    <#
    .SYNOPSIS
        Typecheck TriWeavon Agda (scripts/check.ps1). Falls back to WSL if no Windows agda.
    #>
    [CmdletBinding()]
    param(
        [switch]$Html,
        [switch]$Vendor,
        [string]$Distro = 'Ubuntu'
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $agdaRoot = $env:AGDA_ROOT
    $check = Join-Path $agdaRoot 'scripts\check.ps1'
    $html = Join-Path $agdaRoot 'scripts\html.ps1'
    $vendor = Join-Path $agdaRoot 'scripts\vendor.ps1'

    $agda = Find-LogOSAgda
    if ($Vendor -and (Test-Path -LiteralPath $vendor)) {
        & $vendor
    }

    if ($agda) {
        if ($Html -and (Test-Path -LiteralPath $html)) {
            & $html
            return
        }
        if (Test-Path -LiteralPath $check) {
            & $check -Agda $agda
            return
        }
        Push-Location $agdaRoot
        try { & $agda -l (Join-Path $agdaRoot 'TriWeavon.agda-lib') (Join-Path $agdaRoot 'src\Everything.agda') }
        finally { Pop-Location }
        return
    }

    # WSL fallback (Linux Agda preferred for Cubical)
    if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) {
        Write-Error 'agda not installed on Windows and wsl missing. Install Agda (cabal/ghcup) or apt install agda in WSL.'
        return
    }
    $wslRoot = ConvertTo-LogOSWslPath $agdaRoot
    Write-Host "agda not on Windows PATH — using WSL ($Distro) at $wslRoot" -ForegroundColor Yellow
    $cmd = "cd `"$wslRoot`" && (command -v agda >/dev/null || { echo 'install agda in WSL: sudo apt install agda'; exit 127; }) && agda -l TriWeavon.agda-lib src/Everything.agda"
    wsl -d $Distro -- bash -lc $cmd
}

function Invoke-LogOSLean {
    <#
    .SYNOPSIS
        lake build in LOGOS lean/ (toolchain leanprover/lean4:v4.8.0).
    #>
    [CmdletBinding()]
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$LakeArgs
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $leanRoot = $env:LEAN_ROOT
    if (-not (Test-Path -LiteralPath (Join-Path $leanRoot 'lakefile.lean'))) {
        Write-Error "Lean project missing: $leanRoot"
        return
    }
    if (-not (Get-Command lake -ErrorAction SilentlyContinue)) {
        Write-Error 'lake not on PATH — install elan (https://github.com/leanprover/elan)'
        return
    }
    # Prefer project toolchain over global stable
    $tc = Join-Path $leanRoot 'lean-toolchain'
    if ((Test-Path -LiteralPath $tc) -and (Get-Command elan -ErrorAction SilentlyContinue)) {
        $want = (Get-Content -LiteralPath $tc -Raw).Trim()
        Write-Host "Lean project toolchain: $want" -ForegroundColor DarkGray
    }
    Push-Location -LiteralPath $leanRoot
    try {
        if ($LakeArgs -and $LakeArgs.Count -gt 0) {
            & lake @LakeArgs
        } else {
            & lake build
        }
    } finally {
        Pop-Location
    }
}

function Invoke-LogOSKernels {
    <#
    .SYNOPSIS
        Build CUDA PTX from cutile scripts and/or list LOGOS kernels/*.cu.
    .PARAMETER Build
        Run cutiles/cutile/scripts/build_ptx.ps1 (needs nvcc).
    .PARAMETER List
        List kernel sources under kernels/ and cutile.
    #>
    [CmdletBinding()]
    param(
        [switch]$Build,
        [switch]$List,
        [string]$Arch = 'sm_100'
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }

    if ($List -or -not $Build) {
        Write-Host "LOGOS kernels: $env:KERNELS_ROOT" -ForegroundColor Cyan
        Get-ChildItem -LiteralPath $env:KERNELS_ROOT -Include *.cu,*.cuh,*.wgsl -File -ErrorAction SilentlyContinue |
            ForEach-Object { '  {0}' -f $_.Name }
        $cutileK = Join-Path $env:CUTILE_ROOT 'kernels'
        if (Test-Path -LiteralPath $cutileK) {
            Write-Host "cutile kernels: $cutileK" -ForegroundColor Cyan
            Get-ChildItem -LiteralPath $cutileK -Include *.cu,*.ptx -File -Recurse -ErrorAction SilentlyContinue |
                ForEach-Object { '  {0}' -f $_.FullName.Substring($env:LOGOS_ROOT.Length + 1) }
        }
    }

    if ($Build) {
        $ptx = Join-Path $env:CUTILE_ROOT 'scripts\build_ptx.ps1'
        if (-not (Get-Command nvcc -ErrorAction SilentlyContinue)) {
            Write-Error 'nvcc not found. Install CUDA Toolkit and re-open shell (Initialize-LogOSShell picks CUDA_PATH).'
            return
        }
        if (Test-Path -LiteralPath $ptx) {
            & $ptx -Arch $Arch
        } else {
            Write-Error "Missing $ptx"
        }
    }
}

function ConvertTo-LogOSWslPath {
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$WindowsPath)

    $full = $WindowsPath
    if (Test-Path -LiteralPath $WindowsPath) {
        $full = (Resolve-Path -LiteralPath $WindowsPath).Path
    }
    if ($full -match '^([A-Za-z]):\\(.*)$') {
        $drive = $Matches[1].ToLower()
        $rest = $Matches[2] -replace '\\', '/'
        return "/mnt/$drive/$rest"
    }
    $full -replace '\\', '/'
}

function Enter-LogOSWsl {
    <#
    .SYNOPSIS
        Open WSL in LogOS tree with logos-env.sh sourced.
    #>
    [CmdletBinding()]
    param(
        [string]$Distro = 'Ubuntu',
        [string]$Command = ''
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $wslRoot = ConvertTo-LogOSWslPath $env:LOGOS_ROOT
    $envSh = ConvertTo-LogOSWslPath (Join-Path $env:LOGOS_ROOT 'ops\wsl\logos-env.sh')
    $inner = "source `"$envSh`" 2>/dev/null; cd `"$wslRoot`"; pwd; export LOGOS_ROOT=`"$wslRoot`""
    if ($Command) {
        wsl -d $Distro -- bash -lc "$inner; $Command"
    } else {
        wsl -d $Distro -- bash -lc "$inner; exec bash"
    }
}

# ---------------------------------------------------------------------------
# Command surface — MCP / TUI / HTML (registry: ops/command-surface.json)
# ---------------------------------------------------------------------------

function Get-LogOSCommandSurface {
    <#
    .SYNOPSIS
        Load ops/command-surface.json (unified shell↔MCP↔TUI↔HTML registry).
    #>
    [CmdletBinding()]
    param()

    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $path = if ($env:LOGOS_COMMAND_SURFACE) {
        $env:LOGOS_COMMAND_SURFACE
    } else {
        Join-Path $env:LOGOS_ROOT 'ops\command-surface.json'
    }
    if (-not (Test-Path -LiteralPath $path)) {
        Write-Error "command-surface registry missing: $path"
        return $null
    }
    Get-Content -LiteralPath $path -Raw -Encoding UTF8 | ConvertFrom-Json
}

function Get-LogOSSurfaces {
    <#
    .SYNOPSIS
        List registered surfaces with resolved paths and existence flags.
    #>
    [CmdletBinding()]
    param()

    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $reg = Get-LogOSCommandSurface
    if (-not $reg) { return }

    $rows = foreach ($s in $reg.surfaces) {
        $full = $null
        $exists = $false
        $pathProp = $s.PSObject.Properties['path']
        $urlProp = $s.PSObject.Properties['url_path']
        $pkgProp = $s.PSObject.Properties['package']
        $kind = [string]$s.kind
        if ($pathProp -and $pathProp.Value) {
            $full = Join-Path $env:LOGOS_ROOT (($pathProp.Value -replace '/', '\'))
            $exists = Test-Path -LiteralPath $full
        } elseif ($kind -eq 'tui' -and $pkgProp -and $pkgProp.Value) {
            $full = "cargo run -p $($pkgProp.Value)"
            $exists = $true
        } elseif ($kind -eq 'mcp') {
            $full = $env:COHERENCE_MCP_ROOT
            $exists = [bool]$full -and (Test-Path -LiteralPath $full)
        }
        [pscustomobject]@{
            Id      = $s.id
            Kind    = $kind
            Exists  = $exists
            Role    = $s.role
            Path    = $full
            UrlPath = if ($urlProp) { $urlProp.Value } else { $null }
        }
    }
    $rows | Format-Table -AutoSize Id, Kind, Exists, Role, Path
}

function Resolve-LogOSSurfacePath {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Name
    )

    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $key = $Name.Trim().ToLowerInvariant()
    $aliases = @{
        'meta'         = 'meta-map'
        'metamap'      = 'meta-map'
        'rust'         = 'rust-market'
        'rust_market'  = 'rust-market'
        'rust-market'  = 'rust-market'
        'market'       = 'rust-market'
        'orch'         = 'orchestrator'
        'forge'        = 'reforge'
        'tui'          = 'tui'
        'barcode'      = 'barcode'
        'mcp'          = 'mcp'
        'term'         = 'terminal'
        'terminal'     = 'terminal'
        'dynterm'      = 'terminal'
    }
    if ($aliases.ContainsKey($key)) { $key = $aliases[$key] }

    $reg = Get-LogOSCommandSurface
    $hit = $reg.surfaces | Where-Object { $_.id -eq $key } | Select-Object -First 1
    if (-not $hit) {
        throw "Unknown surface '$Name'. Run logos-surfaces for the registry."
    }
    $pathProp = $hit.PSObject.Properties['path']
    $pkgProp = $hit.PSObject.Properties['package']
    $binProp = $hit.PSObject.Properties['bin']
    if ($pathProp -and $pathProp.Value) {
        return [pscustomobject]@{
            Id   = $hit.id
            Kind = $hit.kind
            Path = (Join-Path $env:LOGOS_ROOT (($pathProp.Value -replace '/', '\')))
            Role = $hit.role
        }
    }
    [pscustomobject]@{
        Id      = $hit.id
        Kind    = $hit.kind
        Path    = $null
        Role    = $hit.role
        Package = if ($pkgProp) { $pkgProp.Value } else { $null }
        Bin     = if ($binProp) { $binProp.Value } else { $null }
    }
}

function Open-LogOSSurface {
    <#
    .SYNOPSIS
        Open an HTML command surface in the default browser (or return path).
    .PARAMETER Name
        Surface id: meta-map (default), rust-market, orchestrator, reforge, evenstar, cockpit, gate, flow
    .PARAMETER PassThru
        Return resolved path without opening.
    #>
    [CmdletBinding()]
    param(
        [string]$Name = 'meta-map',
        [switch]$PassThru
    )

    $s = Resolve-LogOSSurfacePath -Name $Name
    if ($s.Kind -eq 'tui') {
        Write-Host "Surface '$($s.Id)' is a TUI — use logos-tui or logos-barcode" -ForegroundColor Yellow
        return $s
    }
    if ($s.Kind -eq 'mcp') {
        Write-Host "Surface 'mcp' — use logos-mcp list | gauge | rust" -ForegroundColor Yellow
        if ($env:COHERENCE_MCP_ROOT) { Write-Host "  COHERENCE_MCP_ROOT=$env:COHERENCE_MCP_ROOT" }
        return $s
    }
    if (-not $s.Path -or -not (Test-Path -LiteralPath $s.Path)) {
        Write-Error "Surface file missing: $($s.Path)"
        return $s
    }
    if ($PassThru) { return $s }
    $uri = (Resolve-Path -LiteralPath $s.Path).Path
    Write-Host "Opening $($s.Id): $uri" -ForegroundColor Cyan
    Start-Process $uri
    $s
}

function Invoke-LogOSMcp {
    <#
    .SYNOPSIS
        Call coherence-mcp via ops/logos-mcp.mjs (stdio JSON-RPC).
    .EXAMPLE
        logos-mcp list
        logos-mcp gauge
        logos-mcp rust
        logos-mcp wave --content "α+ω=15"
    #>
    [CmdletBinding()]
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$McpArgs
    )

    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $cli = Join-Path $env:LOGOS_ROOT 'ops\logos-mcp.mjs'
    if (-not (Test-Path -LiteralPath $cli)) {
        Write-Error "Missing $cli"
        return
    }
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-Error 'node not on PATH — required for logos-mcp'
        return
    }
    if (-not $McpArgs -or $McpArgs.Count -eq 0) {
        $McpArgs = @('help')
    }
    & node $cli @McpArgs
}

function Start-LogOSBridge {
    <#
    .SYNOPSIS
        Start triweave WebSocket bridge (default 127.0.0.1:8088) for reson8-tui.
    #>
    [CmdletBinding()]
    param(
        [string]$Addr = '127.0.0.1:8088'
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    $env:FORGE_WS_URL = "ws://$Addr"
    $serve = Join-Path $env:LOGOS_ROOT 'ops\serve.ps1'
    if (Test-Path -LiteralPath $serve) {
        & $serve -Addr $Addr -LogOSRoot $env:LOGOS_ROOT
    } else {
        Push-Location $env:LOGOS_ROOT
        try { cargo run -p reson8-triweave -- serve --addr $Addr }
        finally { Pop-Location }
    }
}

function Start-LogOSTui {
    <#
    .SYNOPSIS
        Launch reson8-forge (package reson8-tui). Requires bridge on FORGE_WS_URL.
    #>
    [CmdletBinding()]
    param(
        [switch]$Release,
        [string]$WsUrl
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    if ($WsUrl) { $env:FORGE_WS_URL = $WsUrl }
    if (-not $env:FORGE_WS_URL) { $env:FORGE_WS_URL = 'ws://127.0.0.1:8088' }
    Write-Host "reson8-tui → FORGE_WS_URL=$env:FORGE_WS_URL" -ForegroundColor Cyan
    Push-Location $env:LOGOS_ROOT
    try {
        if ($Release) {
            cargo run -p reson8-tui --release
        } else {
            cargo run -p reson8-tui
        }
    } finally {
        Pop-Location
    }
}

function Start-LogOSBarcode {
    <#
    .SYNOPSIS
        Launch barcode-tui (persistent-homology barcode viewer).
    #>
    [CmdletBinding()]
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$BarcodeArgs
    )
    if (-not $env:LOGOS_ROOT) { Initialize-LogOSShell -Quiet | Out-Null }
    Push-Location $env:LOGOS_ROOT
    try {
        if ($BarcodeArgs -and $BarcodeArgs.Count -gt 0) {
            cargo run -p barcode-tui -- @BarcodeArgs
        } else {
            cargo run -p barcode-tui
        }
    } finally {
        Pop-Location
    }
}

function Show-LogOSCommandSurfaceHelp {
    <#
    .SYNOPSIS
        Print command-surface map (shell / mcp / tui / html).
    #>
    [CmdletBinding()]
    param()

    Write-Host ''
    Write-Host '  LogOS command surface  ·  α+ω=15' -ForegroundColor Cyan
    Write-Host '  ────────────────────────────────────────────' -ForegroundColor DarkGray
    Write-Host '  SHELL   logos-status   toolchain + surface roots'
    Write-Host '          logos-surfaces registry paths'
    Write-Host '  MCP     logos-mcp      list | gauge | wave | rust | workspace | store'
    Write-Host '  TUI     logos-bridge   triweave WS :8088'
    Write-Host '          logos-tui      reson8-forge dashboard'
    Write-Host '          logos-barcode  PH barcode viewer'
    Write-Host '  HTML    logos-site     open /meta-map (default)'
    Write-Host '          logos-site rust-market | orchestrator | reforge | gate | terminal'
    Write-Host '  WIN     logos-preflight | logos-align | logos-wrangler | logos-terminal | logos-pop'
    Write-Host '  CONF    logos-confidence | tw confidence | logos-pop -Command logos-confidence'
    Write-Host '  FORMAL  logos-agda | logos-lean | logos-kernels | logos-cargo'
    Write-Host '  registry: ops/command-surface.json' -ForegroundColor DarkGray
    Write-Host ''
}

function Install-LogOSShellHook {
    <#
    .SYNOPSIS
        Append Initialize-LogOSShell to a PowerShell profile file (idempotent).
    .NOTES
        Uses line-based strip/append — never [regex]::Replace with $variables
        ( .NET replacement treats $__foo as a substitution group and corrupts profiles).
    #>
    [CmdletBinding()]
    param(
        [string]$ProfilePath = $PROFILE,
        [switch]$AllHosts
    )

    if ($AllHosts) { $ProfilePath = $PROFILE.CurrentUserAllHosts }
    $dir = Split-Path $ProfilePath -Parent
    if ($dir -and -not (Test-Path -LiteralPath $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
    if (-not (Test-Path -LiteralPath $ProfilePath)) {
        Set-Content -LiteralPath $ProfilePath -Value "# PowerShell profile`n" -Encoding UTF8
    }

    $marker = '# >>> LogOS.Shell >>>'
    $endMarker = '# <<< LogOS.Shell <<<'
    $modulePath = Join-Path (Resolve-LogOSRoot) 'ops\LogOS.Shell.psm1'
    $modulePathLiteral = $modulePath -replace "'", "''"
    $blockLines = @(
        $marker
        '# Auto-wired by ops/Install-LogOSShell.ps1 — crates / Agda / Lean / kernels on shell open'
        "`$__logosShell = '$modulePathLiteral'"
        'if (Test-Path -LiteralPath $__logosShell) {'
        '    Import-Module $__logosShell -Force'
        '    Initialize-LogOSShell -ImportTriWeavon -ErrorAction SilentlyContinue | Out-Null'
        '}'
        'Remove-Variable __logosShell -ErrorAction SilentlyContinue'
        $endMarker
    )

    $lines = @(Get-Content -LiteralPath $ProfilePath -ErrorAction Stop)
    $out = [System.Collections.Generic.List[string]]::new()
    $skip = $false
    $hadBlock = $false
    foreach ($line in $lines) {
        if ($line -eq $marker -or $line.Trim() -eq $marker) {
            $skip = $true
            $hadBlock = $true
            continue
        }
        if ($line -eq $endMarker -or $line.Trim() -eq $endMarker) {
            $skip = $false
            continue
        }
        if (-not $skip) { [void]$out.Add($line) }
    }

    # Drop trailing blank lines then append one blank + block
    while ($out.Count -gt 0 -and [string]::IsNullOrWhiteSpace($out[$out.Count - 1])) {
        $out.RemoveAt($out.Count - 1)
    }
    [void]$out.Add('')
    foreach ($bl in $blockLines) { [void]$out.Add($bl) }

    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllLines($ProfilePath, $out.ToArray(), $utf8NoBom)

    if ($hadBlock) {
        Write-Host "Updated LogOS hook in $ProfilePath" -ForegroundColor Green
    } else {
        Write-Host "Installed LogOS hook in $ProfilePath" -ForegroundColor Green
    }
}

function Set-LogOSUserEnvironment {
    <#
    .SYNOPSIS
        Persist LOGOS_ROOT + toolchain bins on User PATH (new shells / GUI apps).
    #>
    [CmdletBinding()]
    param()

    $root = Resolve-LogOSRoot
    [Environment]::SetEnvironmentVariable('LOGOS_ROOT', $root, 'User')
    [Environment]::SetEnvironmentVariable('CUTILE_ROOT', (Join-Path $root 'cutiles\cutile'), 'User')
    [Environment]::SetEnvironmentVariable('AGDA_ROOT', (Join-Path $root 'agda'), 'User')
    [Environment]::SetEnvironmentVariable('LEAN_ROOT', (Join-Path $root 'lean'), 'User')
    [Environment]::SetEnvironmentVariable('KERNELS_ROOT', (Join-Path $root 'kernels'), 'User')
    [Environment]::SetEnvironmentVariable('CTWFI_INVARIANT', 'alpha+omega=15', 'User')

    $userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    if (-not $userPath) { $userPath = '' }
    $parts = [System.Collections.Generic.List[string]]::new()
    foreach ($p in ($userPath -split ';')) {
        if ($p -and $p.Trim()) { [void]$parts.Add($p.Trim()) }
    }

    $want = @(
        (Join-Path $env:USERPROFILE '.cargo\bin')
        (Join-Path $env:USERPROFILE '.elan\bin')
        'C:\Users\toolated\.cargo\bin'
        'C:\Users\toolated\.elan\bin'
        (Join-Path $root '.venv\Scripts')
        (Join-Path $root 'ops')
        (Join-Path $root 'agda\scripts')
        (Join-Path $root 'cutiles\cutile\scripts')
    )
    $cuda = Find-LogOSCudaHome
    if ($cuda) {
        [Environment]::SetEnvironmentVariable('CUDA_PATH', $cuda, 'User')
        $want += (Join-Path $cuda 'bin')
    }

    $changed = $false
    foreach ($w in $want) {
        if (-not (Test-Path -LiteralPath $w)) { continue }
        $norm = (Resolve-Path -LiteralPath $w).Path.TrimEnd('\')
        $hit = $false
        foreach ($p in $parts) {
            if ($p.TrimEnd('\') -ieq $norm) { $hit = $true; break }
        }
        if (-not $hit) {
            $parts.Insert(0, $norm)
            $changed = $true
        }
    }
    if ($changed) {
        [Environment]::SetEnvironmentVariable('PATH', ($parts -join ';'), 'User')
        Write-Host 'User PATH updated (cargo/elan/venv/ops/cuda)' -ForegroundColor Green
    } else {
        Write-Host 'User PATH already contains LogOS toolchain entries' -ForegroundColor DarkGray
    }
    Write-Host "User LOGOS_ROOT=$root" -ForegroundColor Green
}

# Friendly aliases (exported via Export-ModuleMember -Alias)
Set-Alias -Name logos -Value Enter-LogOS -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-status -Value Get-LogOSToolchain -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-agda -Value Invoke-LogOSAgda -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-lean -Value Invoke-LogOSLean -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-kernels -Value Invoke-LogOSKernels -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-cargo -Value Invoke-LogOSCargo -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-wsl -Value Enter-LogOSWsl -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-mcp -Value Invoke-LogOSMcp -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-tui -Value Start-LogOSTui -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-barcode -Value Start-LogOSBarcode -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-bridge -Value Start-LogOSBridge -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-site -Value Open-LogOSSurface -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-surfaces -Value Get-LogOSSurfaces -Force -ErrorAction SilentlyContinue
Set-Alias -Name logos-help -Value Show-LogOSCommandSurfaceHelp -Force -ErrorAction SilentlyContinue
Set-Alias -Name cd-logos -Value Enter-LogOS -Force -ErrorAction SilentlyContinue
Set-Alias -Name cd-crates -Value Enter-LogOSCrates -Force -ErrorAction SilentlyContinue
Set-Alias -Name cd-agda -Value Enter-LogOSAgda -Force -ErrorAction SilentlyContinue
Set-Alias -Name cd-lean -Value Enter-LogOSLean -Force -ErrorAction SilentlyContinue
Set-Alias -Name cd-kernels -Value Enter-LogOSKernels -Force -ErrorAction SilentlyContinue

Export-ModuleMember -Function @(
    'Get-LogOSCandidateRoots', 'Resolve-LogOSRoot', 'Initialize-LogOSShell',
    'Get-LogOSRoots', 'Show-LogOSBanner', 'Get-LogOSToolchain',
    'Enter-LogOS', 'Enter-LogOSCrates', 'Enter-LogOSAgda', 'Enter-LogOSLean',
    'Enter-LogOSKernels', 'Enter-LogOSCutile',
    'Invoke-LogOSCargo', 'Invoke-LogOSAgda', 'Invoke-LogOSLean', 'Invoke-LogOSKernels',
    'ConvertTo-LogOSWslPath', 'Enter-LogOSWsl',
    'Install-LogOSShellHook', 'Set-LogOSUserEnvironment',
    'Find-LogOSCudaHome', 'Find-LogOSAgda', 'Add-LogOSPathEntry',
    'Get-LogOSCommandSurface', 'Get-LogOSSurfaces', 'Resolve-LogOSSurfacePath',
    'Open-LogOSSurface', 'Invoke-LogOSMcp', 'Start-LogOSBridge',
    'Start-LogOSTui', 'Start-LogOSBarcode', 'Show-LogOSCommandSurfaceHelp'
) -Alias @(
    'logos', 'logos-status', 'logos-agda', 'logos-lean', 'logos-kernels',
    'logos-cargo', 'logos-wsl', 'logos-mcp', 'logos-tui', 'logos-barcode',
    'logos-bridge', 'logos-site', 'logos-surfaces', 'logos-help',
    'cd-logos', 'cd-crates', 'cd-agda', 'cd-lean', 'cd-kernels'
)
