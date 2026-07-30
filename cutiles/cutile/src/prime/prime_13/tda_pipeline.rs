//! Integration point: prime_13 TDA pipeline → prime_11 boundary reduction.

use crate::prime::prime_11::launch_hypa_boundary_reduction;
use crate::prime::prime_17::provenance::TdaLaunchProvenance;

/// Run the boundary-reduction phase and return launch provenance.
#[cfg(feature = "cuda")]
pub fn run_boundary_reduction_phase(
    device: &std::sync::Arc<cudarc::driver::CudaDevice>,
    stream: &cudarc::driver::CudaStream,
    matrix: &mut cudarc::driver::CudaSlice<i32>,
    col_nz_count: &mut cudarc::driver::CudaSlice<i32>,
    global_pivots: &mut cudarc::driver::CudaSlice<i32>,
    reduced: &mut cudarc::driver::CudaSlice<i32>,
    num_columns: usize,
) -> Result<TdaLaunchProvenance, crate::prime::prime_11::TdaError> {
    let mut error_flag = device.alloc_zeros::<i32>(1)?;
    let mut error_details = device.alloc_zeros::<i32>(5)?;
    launch_hypa_boundary_reduction(
        device,
        stream,
        matrix,
        col_nz_count,
        global_pivots,
        reduced,
        num_columns,
        256,
        &mut error_flag,
        &mut error_details,
    )
}

#[cfg(not(feature = "cuda"))]
pub fn run_boundary_reduction_phase() -> Result<TdaLaunchProvenance, String> {
    Err("cuda feature required for TDA boundary reduction".to_string())
}