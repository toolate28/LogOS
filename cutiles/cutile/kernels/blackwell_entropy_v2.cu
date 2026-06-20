/**
 * blackwell_entropy_v2.cu — Microlocal entropy + surge/Betti proxy (v2)
 * Target: NVIDIA Blackwell sm_100 (fallback build: sm_90)
 */

#include <cuda_runtime.h>
#include <math.h>

#define WARP_SIZE 32
#define BLOCK_SIZE 256

__device__ __forceinline__ float warp_reduce_sum(float val) {
    for (int offset = WARP_SIZE / 2; offset > 0; offset >>= 1) {
        val += __shfl_down_sync(0xffffffff, val, offset);
    }
    return val;
}

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

extern "C" __global__ void entropy_reduction_v2(
    const float* __restrict__ omega_tilde,
    const float* __restrict__ d_perp_rho_sq,
    const float* __restrict__ rho,
    const float* __restrict__ strain_norms,
    float* __restrict__ out_w,
    float* __restrict__ out_visc,
    float* __restrict__ out_stretch,
    float* __restrict__ out_surge,
    float* __restrict__ out_betti_proxy,
    const int total_dof,
    const int np,
    const float nu,
    const float tau,
    const float surge_threshold,
    const float prev_w_avg
) {
    const int tid = blockIdx.x * blockDim.x + threadIdx.x;
    const int grad_len = max(np, 1);
    const int strain_len = max(np, 1);

    float local_w = 0.0f;
    float local_visc = 0.0f;
    float local_stretch = 0.0f;
    float local_betti = 0.0f;

    for (int idx = tid; idx < total_dof; idx += blockDim.x * gridDim.x) {
        int point = (np > 0) ? (idx / max(total_dof / np, 1)) : (idx % grad_len);
        if (point >= grad_len) point = idx % grad_len;
        if (point >= strain_len) point = idx % strain_len;

        float r = rho[idx];
        if (!isfinite(r)) continue;

        float grad_sq = d_perp_rho_sq[point % grad_len];
        float omega = omega_tilde[idx];
        float strain = strain_norms[point % strain_len];

        local_w += tau * grad_sq + r;
        local_visc += grad_sq;
        local_stretch += strain * omega;
        if (grad_sq > surge_threshold) {
            local_betti += 1.0f;
        }
    }

    local_w = block_reduce_sum(local_w);
    local_visc = block_reduce_sum(local_visc);
    local_stretch = block_reduce_sum(local_stretch);
    local_betti = block_reduce_sum(local_betti);

    __shared__ float block_w;
    __shared__ float block_visc;
    __shared__ float block_stretch;
    __shared__ float block_betti;

    if (threadIdx.x == 0) {
        block_w = local_w;
        block_visc = -nu * local_visc;
        block_stretch = -tau * local_stretch;
        block_betti = local_betti;
        atomicAdd(out_w, block_w);
        atomicAdd(out_visc, block_visc);
        atomicAdd(out_stretch, block_stretch);
        atomicAdd(out_betti_proxy, block_betti);
    }
    __syncthreads();

    if (threadIdx.x == 0 && blockIdx.x == 0) {
        float base = fmaxf(fabsf(prev_w_avg), 1e-12f);
        float rel = fabsf((*out_w) - prev_w_avg) / base;
        *out_surge = (rel > surge_threshold) ? 1.0f : 0.0f;
    }
}