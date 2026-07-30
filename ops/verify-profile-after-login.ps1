# LogOS / profile post-login verification
# Run in PowerShell AFTER a full Sign out + Sign in

Write-Host "=== Identity ===" -ForegroundColor Cyan
$id = [System.Security.Principal.WindowsIdentity]::GetCurrent()
"User: $($id.Name)"
"IsGuest: $($id.IsGuest)   (expect False)"
"Auth: $($id.AuthenticationType)"

Write-Host "`n=== Guests group in token? ===" -ForegroundColor Cyan
$g = whoami /groups | Select-String "S-1-5-32-546"
if ($g) { Write-Host "STILL HAS GUESTS IN TOKEN — sign out again or reboot" -ForegroundColor Red; $g }
else { Write-Host "OK: Guests not in token" -ForegroundColor Green }

Write-Host "`n=== ProfileList State ===" -ForegroundColor Cyan
$sid = ([System.Security.Principal.WindowsIdentity]::GetCurrent()).User.Value
$state = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList\$sid" -ErrorAction SilentlyContinue).State
"SID=$sid State=$state  (expect 0; 128=guest profile flag)"

Write-Host "`n=== Local groups ===" -ForegroundColor Cyan
Get-LocalGroup | ForEach-Object {
  try {
    if (Get-LocalGroupMember -Group $_.Name -ErrorAction Stop | Where-Object { $_.SID -eq $sid -or $_.Name -match $env:USERNAME }) {
      $_.Name
    }
  } catch {}
}

Write-Host "`n=== OneDrive ===" -ForegroundColor Cyan
$od = Get-Process OneDrive -ErrorAction SilentlyContinue
if ($od) { "Running: PIDs $($od.Id -join ',')" } else {
  $exe = "${env:ProgramFiles}\Microsoft OneDrive\OneDrive.exe"
  if (Test-Path $exe) {
    Start-Process $exe
    "Launched OneDrive — check tray / errors"
  } else { "OneDrive.exe missing" }
}

Write-Host "`n=== LogOS paths ===" -ForegroundColor Cyan
@(
  "F:\Users\Matthew Ruhnau\LogOS",
  "F:\Users\Matthew Ruhnau\LogOS.worktrees\master",
  "F:\Users\Matthew Ruhnau\LogOS.worktrees\master\9P2000.L\strands\User_Dropfiles\dump"
) | ForEach-Object {
  if (Test-Path $_) { "OK  $_" } else { "MISS $_" }
}

Write-Host "`nDone. If IsGuest=False and State=0, OneDrive + special-profile blocks should be gone." -ForegroundColor Green
