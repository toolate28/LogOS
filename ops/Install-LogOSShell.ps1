#Requires -Version 5.1
<#
.SYNOPSIS
    One-shot: wire LogOS into PowerShell + optional WSL2 so crates/Agda/Lean/kernels load on shell open.
.EXAMPLE
    pwsh -File ops\Install-LogOSShell.ps1
.EXAMPLE
    pwsh -File ops\Install-LogOSShell.ps1 -Wsl -Distro Ubuntu
#>
[CmdletBinding()]
param(
    [switch]$Wsl,
    [string]$Distro = 'Ubuntu',
    [switch]$SkipUserEnv,
    [switch]$SkipProfile,
    [switch]$Quiet
)

$ErrorActionPreference = 'Stop'
$OpsDir = $PSScriptRoot
$Module = Join-Path $OpsDir 'LogOS.Shell.psm1'
if (-not (Test-Path -LiteralPath $Module)) {
    throw "Missing module: $Module"
}

Import-Module $Module -Force
Initialize-LogOSShell -Quiet:$Quiet -ImportTriWeavon | Out-Null

if (-not $SkipUserEnv) {
    Set-LogOSUserEnvironment
}

if (-not $SkipProfile) {
    $profileTargets = [System.Collections.Generic.List[string]]::new()
    if ($PROFILE) { [void]$profileTargets.Add($PROFILE) }

    # Prefer local Documents over OneDrive when cloud provider is offline
    foreach ($p in @(
            (Join-Path $env:USERPROFILE 'Documents\PowerShell\Microsoft.PowerShell_profile.ps1')
            (Join-Path $env:USERPROFILE 'Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1')
            (Join-Path $env:USERPROFILE 'OneDrive\Documents\PowerShell\Microsoft.PowerShell_profile.ps1')
            (Join-Path $env:USERPROFILE 'OneDrive\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1')
        )) {
        if ($profileTargets -notcontains $p) { [void]$profileTargets.Add($p) }
    }

    foreach ($p in $profileTargets) {
        try {
            $parent = Split-Path $p -Parent
            if (-not (Test-Path -LiteralPath $parent)) {
                New-Item -ItemType Directory -Path $parent -Force -ErrorAction Stop | Out-Null
            }
            # Probe writeability (OneDrive offline throws on write)
            $probe = Join-Path $parent '.logos-write-probe'
            Set-Content -LiteralPath $probe -Value 'ok' -ErrorAction Stop
            Remove-Item -LiteralPath $probe -Force -ErrorAction SilentlyContinue
            Install-LogOSShellHook -ProfilePath $p
        } catch {
            Write-Warning "Skip profile $p — $($_.Exception.Message)"
        }
    }

    # Intentionally skip CurrentUserAllHosts (profile.ps1): host profile is enough
    # and double-import of the module resets script-scoped init flags.
}

function Install-LogOSWslHook {
    param([string]$TargetDistro, [string]$EnvShWsl, [string]$RootWsl)

    $bash = @"
set -e
ENV_SH='$EnvShWsl'
ROOT='$RootWsl'
MARKER='# >>> LogOS.Shell >>>'
ENDM='# <<< LogOS.Shell <<<'
BLOCK=`"`${MARKER}
if [ -f `"`${ENV_SH}`" ]; then
  . `"`${ENV_SH}`"
fi
`${ENDM}`"
for rc in "`$HOME/.bashrc" "`$HOME/.zshrc" "`$HOME/.profile"; do
  touch "`$rc"
  if grep -q 'LogOS.Shell' "`$rc" 2>/dev/null; then
    tmp=`$(mktemp)
    awk '
      /# >>> LogOS\.Shell >>>/ { print ENVIRON["BLOCK"]; skip=1; next }
      /# <<< LogOS\.Shell <</ { skip=0; next }
      !skip { print }
    ' BLOCK="`$BLOCK" "`$rc" > "`$tmp" && mv "`$tmp" "`$rc"
  else
    printf '\n%s\n' "`$BLOCK" >> "`$rc"
  fi
done
echo "WSL LogOS hook OK  LOGOS_ROOT=`$ROOT  env=`$ENV_SH"
"@

    # Simpler: write a tiny installer script via stdin to avoid quoting hell
    $script = @'
set -e
ENV_SH="$1"
ROOT="$2"
MARKER="# >>> LogOS.Shell >>>"
ENDM="# <<< LogOS.Shell <<<"
BLOCK="${MARKER}
if [ -f \"${ENV_SH}\" ]; then
  . \"${ENV_SH}\"
fi
${ENDM}"
for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
  touch "$rc"
  if grep -q 'LogOS.Shell' "$rc" 2>/dev/null; then
    tmp=$(mktemp)
    # strip old block then append
    sed '/# >>> LogOS\.Shell >>>/,/# <<< LogOS\.Shell <<</d' "$rc" > "$tmp"
    printf '\n%s\n' "$BLOCK" >> "$tmp"
    mv "$tmp" "$rc"
  else
    printf '\n%s\n' "$BLOCK" >> "$rc"
  fi
done
echo "WSL LogOS hook OK  LOGOS_ROOT=$ROOT"
'@

    $b64 = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($script))
    wsl -d $TargetDistro -- bash -lc "echo $b64 | base64 -d > /tmp/logos-install-hook.sh && bash /tmp/logos-install-hook.sh '$EnvShWsl' '$RootWsl'"
    return $LASTEXITCODE
}

if ($Wsl) {
    $envSh = Join-Path $OpsDir 'wsl\logos-env.sh'
    if (-not (Test-Path -LiteralPath $envSh)) {
        Write-Warning "Missing $envSh — skip WSL"
    } else {
        $envShWsl = ConvertTo-LogOSWslPath $envSh
        $rootWsl = ConvertTo-LogOSWslPath $env:LOGOS_ROOT
        Write-Host "Installing WSL hook on $Distro ..." -ForegroundColor Cyan
        $code = Install-LogOSWslHook -TargetDistro $Distro -EnvShWsl $envShWsl -RootWsl $rootWsl
        if ($code -ne 0) {
            Write-Warning "WSL ($Distro) install exit $code — distro may be cold. Re-run with -Wsl after: wsl -d $Distro -- echo ready"
        } else {
            Write-Host "WSL ($Distro) hooked to logos-env.sh" -ForegroundColor Green
        }

        $listed = @(wsl -l -q 2>$null | ForEach-Object { $_.ToString().Trim() -replace '\x00', '' } | Where-Object { $_ })
        if ($listed -contains 'kali-linux' -and $Distro -ne 'kali-linux') {
            Write-Host 'Also installing on kali-linux ...' -ForegroundColor Cyan
            $null = Install-LogOSWslHook -TargetDistro 'kali-linux' -EnvShWsl $envShWsl -RootWsl $rootWsl
        }
    }
}

Write-Host ''
Write-Host '=== LogOS shell install complete ===' -ForegroundColor Green
Write-Host "  LOGOS_ROOT = $env:LOGOS_ROOT"
Write-Host '  Open a NEW pwsh window (or: . $PROFILE)'
Write-Host '  Then: logos-status | logos-agda | logos-lean | logos-kernels | logos-cargo'
Write-Host ''
Get-LogOSToolchain
