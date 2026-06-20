# Compile blackwell_entropy_v2.cu → PTX (requires CUDA toolkit nvcc)
param(
    [string]$Arch = "sm_100",
    [string]$FallbackArch = "sm_90"
)

$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$Cu = Join-Path $Root "kernels\blackwell_entropy_v2.cu"
$Ptx = Join-Path $Root "kernels\blackwell_entropy_v2.ptx"

if (-not (Get-Command nvcc -ErrorAction SilentlyContinue)) {
    Write-Host "nvcc not found — install CUDA Toolkit or add to PATH" -ForegroundColor Red
    exit 1
}

Write-Host "Building PTX arch=$Arch" -ForegroundColor Cyan
& nvcc -ptx -arch=$Arch $Cu -o $Ptx
if ($LASTEXITCODE -ne 0) {
    Write-Host "Retry arch=$FallbackArch" -ForegroundColor Yellow
    & nvcc -ptx -arch=$FallbackArch $Cu -o $Ptx
}
if ($LASTEXITCODE -eq 0) {
    Write-Host "Wrote $Ptx — rebuild cutile with: cargo build -p cutile --features cuda" -ForegroundColor Green
} else {
    exit $LASTEXITCODE
}