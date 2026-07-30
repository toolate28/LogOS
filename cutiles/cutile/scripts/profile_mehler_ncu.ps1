# Nsight Compute profiling for mehler_mma_levin_batched / levin_mma_collocation_8
# Usage: pwsh -File scripts/profile_mehler_ncu.ps1 -Binary .\target\release\examples\mehler_bench

param(
    [string]$Binary = ".\target\release\examples\mehler_bench",
    [string]$Kernel = "mehler_mma_levin_batched",
    [int]$LaunchCount = 3
)

$metrics = @(
    "sm__pipe_fma_cycles_active.avg.pct_of_peak_sustained_active",
    "sm__sass_thread_inst_executed_op_fmaf_pred_on.sum",
    "sm__sass_thread_inst_executed_op_fadd_pred_on.sum",
    "sm__sass_thread_inst_executed_op_fmul_pred_on.sum",
    "smsp__sass_average_register_spills_per_thread",
    "sm__inst_executed.avg.per_cycle_active",
    "sm__warps_active.avg.pct_of_peak_sustained_active"
) -join ","

ncu --target-processes all `
    --kernel-name $Kernel `
    --launch-skip 0 `
    --launch-count $LaunchCount `
    --metrics $metrics `
    --csv `
    $Binary @args | Out-File -Encoding utf8 "mehler_full_profile.csv"

Write-Host "Profile written to mehler_full_profile.csv"
Write-Host "GUI report: ncu --export levin_profile.ncu-rep --kernel-name levin_mma_collocation_8 $Binary"