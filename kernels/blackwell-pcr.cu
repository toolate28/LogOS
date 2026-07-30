/**
 * blackwell-pcr.cu — Parallel Cyclic Reduction for implicit 1D diffusion
 *
 * Target: NVIDIA RTX 5090 Blackwell (sm_100)
 * Budget: ≤ 20 TFLOPs FP32, 1.2 TB/s memory BW
 * Cadence: per-tick viscous substep
 *
 * Solves tridiagonal systems arising from the implicit treatment of
 * the viscous term νΔu in the Navier-Stokes DNS loop.
 * Each z-pencil (nx × ny independent systems) is solved in parallel.
 *
 * Reference: Brief §4.2
 */

#include <cuda_runtime.h>

// Shared memory tile for PCR forward/backward sweep
#define TILE_SIZE 256

/**
 * Parallel Cyclic Reduction kernel for tridiagonal systems.
 * a[i]*x[i-1] + b[i]*x[i] + c[i]*x[i+1] = d[i]
 *
 * Each thread block handles one pencil of length n.
 */
__global__ void pcr_tridiagonal(
    const float* __restrict__ a,  // sub-diagonal
    const float* __restrict__ b,  // main diagonal
    const float* __restrict__ c,  // super-diagonal
    float* __restrict__ d,        // RHS (overwritten with solution)
    const int n,                  // system size
    const int num_systems         // total independent systems
) {
    const int sys_id = blockIdx.x;
    if (sys_id >= num_systems) return;

    const int offset = sys_id * n;
    const int tid = threadIdx.x;

    __shared__ float sa[TILE_SIZE];
    __shared__ float sb[TILE_SIZE];
    __shared__ float sc[TILE_SIZE];
    __shared__ float sd[TILE_SIZE];

    // Load into shared memory
    if (tid < n) {
        sa[tid] = a[offset + tid];
        sb[tid] = b[offset + tid];
        sc[tid] = c[offset + tid];
        sd[tid] = d[offset + tid];
    }
    __syncthreads();

    // PCR forward reduction
    for (int stride = 1; stride < n; stride <<= 1) {
        float a_new = sa[tid], b_new = sb[tid], c_new = sc[tid], d_new = sd[tid];

        if (tid >= stride && tid < n) {
            float k1 = sa[tid] / sb[tid - stride];
            a_new = -k1 * sa[tid - stride];
            b_new = sb[tid] - k1 * sc[tid - stride];
            d_new = sd[tid] - k1 * sd[tid - stride];
        }
        if (tid + stride < n && tid < n) {
            float k2 = sc[tid] / sb[tid + stride];
            c_new = -k2 * sc[tid + stride];
            b_new = b_new - k2 * sa[tid + stride];
            d_new = d_new - k2 * sd[tid + stride];
        }

        __syncthreads();
        sa[tid] = a_new;
        sb[tid] = b_new;
        sc[tid] = c_new;
        sd[tid] = d_new;
        __syncthreads();
    }

    // Write solution back
    if (tid < n) {
        d[offset + tid] = sd[tid] / sb[tid];
    }
}

// TODO(Week +1): Launch wrapper with Rust cudarc binding
// void launch_pcr(float* a, float* b, float* c, float* d, int n, int num_systems, cudaStream_t stream);
