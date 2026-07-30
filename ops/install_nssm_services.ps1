<#
NSSM installer and service registration script for Windows
- Installs NSSM if not present (manual download step required if not found)
- Registers three services using NSSM to run: styx-bridge, inferno watcher, mc-to-atom watcher
#>
param()

$Base = "F:\Users\Matthew Ruhnau\LogOS\9P2000.L"
$LogOS = "F:\Users\Matthew Ruhnau\LogOS"
$Watchers = @(
    @{ Name = "styx-bridge"; Path = "$Base\styx\styx-bridge.py"; Args = "" },
    @{ Name = "inferno-watcher"; Path = "$Base\inferno\watch-waveai.py"; Args = "" },
    @{ Name = "mc-to-atom"; Path = "$LogOS\ops\watchers\mc-to-atom.ps1"; Args = "-NoProfile -ExecutionPolicy Bypass -File `"$LogOS\ops\watchers\mc-to-atom.ps1`"" }
)

# locate nssm
$nssm = (Get-Command nssm -ErrorAction SilentlyContinue | Select-Object -First 1).Source
if (-not $nssm) {
    Write-Host "nssm.exe not found in PATH. Please download NSSM and place nssm.exe in C:\\Windows\\System32 or add to PATH." -ForegroundColor Yellow
    Write-Host "Download: https://nssm.cc/download" -ForegroundColor Cyan
    exit 1
}

foreach ($w in $Watchers) {
    $svc = "reson8-$($w.Name)"
    if ((Get-Service -Name $svc -ErrorAction SilentlyContinue)) {
        Write-Host "Service $svc already exists, skipping." -ForegroundColor Gray
        continue
    }

    if ($w.Name -eq 'mc-to-atom') {
        # register as powershell
        & $nssm install $svc "C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe" $w.Args
    } else {
        # python script
        $python = (Get-Command python -ErrorAction SilentlyContinue).Source
        if (-not $python) { Write-Host "python not found in PATH" -ForegroundColor Red; exit 1 }
        & $nssm install $svc $python "$($w.Path)" $w.Args
    }

    & $nssm set $svc AppDirectory "$Base"
    & $nssm set $svc Start SERVICE_AUTO_START
    Write-Host "Installed service $svc" -ForegroundColor Green
}

Write-Host "All NSSM service installation attempts complete. Start services with: nssm start reson8-styx-bridge" -ForegroundColor Cyan
