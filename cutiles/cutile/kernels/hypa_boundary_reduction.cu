#include <cooperative_groups.h>
#include <cuda_runtime.h>

namespace cg = cooperative_groups;

#define WARP_LOCAL_PIVOT_SIZE 256
#define MAX_COL_NONZEROS      128
#define MAX_REDUCTION_ITER    2048

// Kernel error codes
#define TDA_SUCCESS                    0
#define TDA_ERROR_MAX_ITER_EXCEEDED    1

__device__ __forceinline__ int findLowestNonZeroWarp(const int* __restrict__ col_data, int col_nz) {
    unsigned mask = __ballot_sync(0xffffffff, col_data[threadIdx.x % 32] < col_nz);
    if (mask == 0) return -1;
    return __ffs(mask) - 1;
}

__device__ __forceinline__ void reduce_column_xor(
    int* __restrict__ matrix,
    int* __restrict__ col_nz_count,
    int target_col,
    int pivot_col)
{
    int target_nz = col_nz_count[target_col];
    int pivot_nz  = col_nz_count[pivot_col];

    for (int i = 0; i < pivot_nz; ++i) {
        int row = matrix[pivot_col * MAX_COL_NONZEROS + i];
        bool exists = false;
        for (int j = 0; j < target_nz; ++j) {
            if (matrix[target_col * MAX_COL_NONZEROS + j] == row) {
                matrix[target_col * MAX_COL_NONZEROS + j] = matrix[target_col * MAX_COL_NONZEROS + target_nz - 1];
                target_nz--;
                exists = true;
                break;
            }
        }
        if (!exists && target_nz < MAX_COL_NONZEROS) {
            matrix[target_col * MAX_COL_NONZEROS + target_nz] = row;
            target_nz++;
        }
    }
    col_nz_count[target_col] = target_nz;
}

__device__ __forceinline__ bool try_claim_pivot_hierarchical(
    int pivot_row, int col,
    int* __restrict__ global_pivots,
    int* __restrict__ warp_local_pivots)
{
    if (pivot_row == -1) return false;
    int local_idx = pivot_row % WARP_LOCAL_PIVOT_SIZE;
    int expected = -1;

    int old = atomicCAS(&warp_local_pivots[local_idx], expected, col);
    if (old != -1 && old != col) return false;

    int global_old = atomicCAS(&global_pivots[pivot_row], expected, col);
    if (global_old == -1 || global_old == col) return true;

    atomicExch(&warp_local_pivots[local_idx], -1);
    return false;
}

__device__ __forceinline__ void report_kernel_error(
    int* __restrict__ error_flag,
    int* __restrict__ error_details,
    int error_code, int pivot_row, int iteration)
{
    if (atomicCAS(error_flag, TDA_SUCCESS, error_code) == TDA_SUCCESS) {
        int tid = blockIdx.x * blockDim.x + threadIdx.x;
        error_details[0] = error_code;
        error_details[1] = blockIdx.x;
        error_details[2] = threadIdx.x;
        error_details[3] = pivot_row;
        error_details[4] = iteration;
    }
}

__global__ void lockFreeReductionKernel_Optimized(
    int* __restrict__ matrix,
    int* __restrict__ col_nz_count,
    int* __restrict__ global_pivots,
    int* __restrict__ reduced,
    int num_columns,
    int* __restrict__ error_flag,
    int* __restrict__ error_details)
{
    extern __shared__ int shared_mem[];
    int warp_id = threadIdx.x / 32;
    int* warp_local_pivots = shared_mem + (warp_id * WARP_LOCAL_PIVOT_SIZE);

    if ((threadIdx.x % 32) == 0) {
        for (int i = 0; i < WARP_LOCAL_PIVOT_SIZE; ++i) warp_local_pivots[i] = -1;
    }
    __syncwarp();

    cg::thread_block block = cg::this_thread_block();
    cg::thread_block_tile<32> warp = cg::tiled_partition<32>(block);

    int col = blockIdx.x * blockDim.x + threadIdx.x;
    if (col >= num_columns) return;

    bool done = false;
    int iterations = 0;

    while (!done && iterations < MAX_REDUCTION_ITER) {
        int pivot_row = findLowestNonZeroWarp(matrix + col * MAX_COL_NONZEROS, col_nz_count[col]);
        bool owns_pivot = false;

        if (pivot_row != -1) {
            owns_pivot = try_claim_pivot_hierarchical(pivot_row, col, global_pivots, warp_local_pivots);
        }
        owns_pivot = warp.any(owns_pivot);

        if (owns_pivot && pivot_row != -1) {
            reduce_column_xor(matrix, col_nz_count, col, global_pivots[pivot_row]);
        }

        int new_pivot = findLowestNonZeroWarp(matrix + col * MAX_COL_NONZEROS, col_nz_count[col]);
        done = (new_pivot == -1);
        iterations++;

        if (iterations >= MAX_REDUCTION_ITER) {
            report_kernel_error(error_flag, error_details, TDA_ERROR_MAX_ITER_EXCEEDED, pivot_row, iterations);
            return;
        }
        warp.sync();
    }

    if (col < num_columns) reduced[col] = col;
}
