/**
 * blackwell-entropy.cu — Microlocal entropy W[ω̃] reduction kernel
 *
 * Target: NVIDIA RTX 5090 Blackwell (sm_100)
 * Budget: ≤ 15 TFLOPs FP32, 0.6 TB/s memory BW
 * Cadence: EVERY DNS tick — this is the Theorem-4 witness
 *
 * THE CRITICAL KERNEL for the Navier-Stokes singularity hunt.
 *
 * Computes:
 *   W = Σ_{x,ξ} (τ ‖d_⊥ρ‖² + ρ)
 *   visc_term = −ν Σ ‖d_⊥ρ‖²
 *   stretch_term = −τ Σ ‖S‖ ‖ω̃‖
 *
 * Uses warp-level parallel reduction for maximum throughput.
 *
 * Reference: Brief §1.2, §3.2, §4.2
 * Epistemic status: ⟦H-NS1⟧ braid-provisional
 */

#include <cuda_runtime.h>

#define WARP_SIZE 32
#define BLOCK_SIZE 256

/**
 * Warp-level reduction: sum of float within a warp.
 */
__device__ __forceinline__ float warp_reduce_sum(float val) {
    for (int offset = WARP_SIZE / 2; offset > 0; offset >>= 1) {
        val += __shfl_down_sync(0xffffffff, val, offset);
    }
    return val;
}

/**
 * Block-level reduction using shared memory + warp reduction.
 */
__device__ float block_reduce_sum(float val) {
    __shared__ float shared[BLOCK_SIZE / WARP_SIZE];

    int lane = threadIdx.x % WARP_SIZE;
    int warp_id = threadIdx.x / WARP_SIZE;

    val = warp_reduce_sum(val);

    if (lane == 0) {
        shared[warp_id] = val;
    }
    __syncthreads();

    val = (threadIdx.x < BLOCK_SIZE / WARP_SIZE) ? shared[lane] : 0.0f;
    if (warp_id == 0) {
        val = warp_reduce_sum(val);
    }
    return val;
}

/**
 * Compute W[ω̃], visc_term, and stretch_term in a single kernel launch.
 *
 * Input:
 *   omega_tilde[np * N] — lifted vorticity magnitudes
 *   d_perp_rho_sq[np]   — ‖d_⊥ρ‖² per mesh point (precomputed)
 *   rho[np * N]          — log-amplitudes
 *   strain_norms[np]     — ‖S‖ Frobenius norms
 *   np, N, nu, tau       — parameters
 *
 * Output:
 *   partial_w[gridDim.x]       — partial sums for W
 *   partial_visc[gridDim.x]    — partial sums for visc_term
 *   partial_stretch[gridDim.x] — partial sums for stretch_term
 */
__global__ void entropy_reduction(
    const float* __restrict__ omega_tilde,
    const float* __restrict__ d_perp_rho_sq,
    const float* __restrict__ rho,
    const float* __restrict__ strain_norms,
    float* __restrict__ partial_w,
    float* __restrict__ partial_visc,
    float* __restrict__ partial_stretch,
    const int np,
    const int N,
    const float nu,
    const float tau
) {
    const int tid = blockIdx.x * blockDim.x + threadIdx.x;
    const int total_dof = np * N;

    float local_w = 0.0f;
    float local_visc = 0.0f;
    float local_stretch = 0.0f;

    // Grid-stride loop
    for (int idx = tid; idx < total_dof; idx += blockDim.x * gridDim.x) {
        int point = idx / N;
        float r = rho[idx];

        if (isfinite(r)) {
            float grad_sq = d_perp_rho_sq[point];
            local_w += tau * grad_sq + r;
            local_visc += grad_sq;
            local_stretch += strain_norms[point] * omega_tilde[idx];
        }
    }

    // Block reduce
    local_w = block_reduce_sum(local_w);
    local_visc = block_reduce_sum(local_visc);
    local_stretch = block_reduce_sum(local_stretch);

    if (threadIdx.x == 0) {
        partial_w[blockIdx.x] = local_w;
        partial_visc[blockIdx.x] = -nu * local_visc;
        partial_stretch[blockIdx.x] = -tau * local_stretch;
    }
}

// TODO(Week +1): Final reduction pass + Rust cudarc binding
// TODO(Week +1): Taylor-Green vortex baseline test (analytic energy decay)
