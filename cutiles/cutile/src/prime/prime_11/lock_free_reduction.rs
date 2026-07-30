#[cfg(feature = "cuda")]
use cudarc::driver::{CudaDevice, CudaStream, DriverError};
#[cfg(feature = "cuda")]
use std::sync::Arc;

use crate::prime::prime_17::provenance::{TdaLaunchProvenance, TdaTilingPhase};

#[derive(Debug, thiserror::Error)]
pub enum TdaError {
    #[cfg(feature = "cuda")]
    #[error("CUDA driver error: {0}")]
    Cuda(#[from] DriverError),
    #[error("Kernel error: code={code} block={block} thread={thread} pivot_row={pivot_row} iteration={iteration}")]
    KernelError {
        code: i32,
        block: i32,
        thread: i32,
        pivot_row: i32,
        iteration: i32,
    },
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Shared memory too large")]
    SharedMemoryTooLarge,
    #[error("CUDA feature not enabled")]
    CudaDisabled,
}

#[cfg(feature = "cuda")]
pub fn launch_hypa_boundary_reduction(
    device: &Arc<CudaDevice>,
    stream: &CudaStream,
    matrix: &mut cudarc::driver::CudaSlice<i32>,
    col_nz_count: &mut cudarc::driver::CudaSlice<i32>,
    global_pivots: &mut cudarc::driver::CudaSlice<i32>,
    reduced: &mut cudarc::driver::CudaSlice<i32>,
    num_columns: usize,
    block_size: u32,
    error_flag: &mut cudarc::driver::CudaSlice<i32>,
    error_details: &mut cudarc::driver::CudaSlice<i32>,
) -> Result<TdaLaunchProvenance, TdaError> {
    if num_columns == 0 {
        return Err(TdaError::InvalidInput("num_columns cannot be zero".into()));
    }

    let warps_per_block = (block_size / 32) as usize;
    let shared_mem_bytes = 256 * std::mem::size_of::<i32>() * warps_per_block;

    let max_shared = device
        .attribute(
            cudarc::driver::sys::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK,
        )
        .map_err(TdaError::from)? as usize;

    if shared_mem_bytes > max_shared {
        return Err(TdaError::SharedMemoryTooLarge);
    }

    let grid_size = ((num_columns as u32) + block_size - 1) / block_size;

    unsafe {
        device.launch_kernel(
            "lockFreeReductionKernel_Optimized",
            (grid_size, 1, 1),
            (block_size, 1, 1),
            shared_mem_bytes,
            stream,
            matrix,
            col_nz_count,
            global_pivots,
            reduced,
            num_columns as i32,
            error_flag,
            error_details,
        )?;
    }

    stream.synchronize()?;

    let mut host_error = [0i32; 1];
    device.dtoh_sync_copy_into(error_flag, &mut host_error)?;

    if host_error[0] != 0 {
        let mut details = [0i32; 5];
        device.dtoh_sync_copy_into(error_details, &mut details)?;
        return Err(TdaError::KernelError {
            code: details[0],
            block: details[1],
            thread: details[2],
            pivot_row: details[3],
            iteration: details[4],
        });
    }

    Ok(TdaLaunchProvenance {
        phase: TdaTilingPhase::BoundaryReduction,
        launch_config_hash: "hypa_v1".to_string(),
        algorithm: "HYPHA".to_string(),
        reduction_mode: "HierarchicalLockFree+ApparentPairs".to_string(),
        apparent_pairs_count: 0,
        reduction_iterations: 0,
        execution_time_ms: None,
        shared_memory_bytes: shared_mem_bytes,
        block_size,
        grid_size,
    })
}

#[cfg(not(feature = "cuda"))]
pub fn launch_hypa_boundary_reduction() -> Result<TdaLaunchProvenance, TdaError> {
    Err(TdaError::CudaDisabled)
}