# Compile CUDA kernels → PTX (requires CUDA toolkit nvcc)
param(
    [string]$Arch = "sm_100",
    [string]$FallbackArch = "sm_90"
)

$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)

$Kernels = @(
    @{ Cu = "kernels\blackwell_entropy_v2.cu"; Ptx = "kernels\blackwell_entropy_v2.ptx" },
    @{ Cu = "kernels\mehler_mma_levin_batched.cu"; Ptx = "kernels\mehler_mma_levin_batched.ptx" }
)

if (-not (Get-Command nvcc -ErrorAction SilentlyContinue)) {
    Write-Host "nvcc not found — install CUDA Toolkit or add to PATH" -ForegroundColor Red
    exit 1
}

function Build-Ptx([string]$CuRel, [string]$PtxRel, [string]$Arch, [string]$FallbackArch) {
    $Cu = Join-Path $Root $CuRel
    $Ptx = Join-Path $Root $PtxRel
    Write-Host "Building $CuRel arch=$Arch" -ForegroundColor Cyan
    & nvcc -ptx -arch=$Arch -O3 --use_fast_math $Cu -o $Ptx
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Retry $CuRel arch=$FallbackArch" -ForegroundColor Yellow
        & nvcc -ptx -arch=$FallbackArch -O3 --use_fast_math $Cu -o $Ptx
    }
    return $LASTEXITCODE
}

$failed = $false
foreach ($k in $Kernels) {
    if (Build-Ptx $k.Cu $k.Ptx $Arch $FallbackArch -ne 0) {
        $failed = $true
    }
}

if (-not $failed) {
    Write-Host "PTX build complete — rebuild cutile with: cargo build -p cutile --features cuda" -ForegroundColor Green
    exit 0
}
exit 1