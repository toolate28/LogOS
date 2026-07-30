/**
 * blackwell-fft.cu — 3D Spectral Poisson Solve (pressure projection)
 *
 * Target: NVIDIA RTX 5090 Blackwell (sm_100)
 * Budget: ≤ 40 TFLOPs FP32, 1.5 TB/s memory BW
 * Cadence: per-tick pressure projection step
 *
 * Solves ∇²p = ∇·(u·∇u) in Fourier space to enforce ∇·u = 0.
 * Uses cuFFT for forward/inverse transforms; the Poisson solve in
 * Fourier space is a trivial diagonal division by -|k|².
 *
 * Reference: Brief §4.2
 */

#include <cuda_runtime.h>
#include <cufft.h>

/**
 * Poisson solve in Fourier space: p_hat[k] = rhs_hat[k] / (-|k|²)
 * Handles k=0 mode (sets p_hat[0] = 0 for zero-mean pressure).
 */
__global__ void poisson_solve_spectral(
    cufftComplex* __restrict__ rhs_hat,  // in-place: RHS → solution
    const int nx, const int ny, const int nz,
    const float dx, const float dy, const float dz
) {
    const int idx = blockIdx.x * blockDim.x + threadIdx.x;
    const int total = nx * ny * (nz / 2 + 1);  // R2C output size
    if (idx >= total) return;

    const int kz = idx % (nz / 2 + 1);
    const int ky = (idx / (nz / 2 + 1)) % ny;
    const int kx = idx / ((nz / 2 + 1) * ny);

    // Wavenumber magnitudes (assuming periodic BC)
    const float pi = 3.14159265358979323846f;
    float wx = 2.0f * pi * (kx <= nx / 2 ? kx : kx - nx) / (nx * dx);
    float wy = 2.0f * pi * (ky <= ny / 2 ? ky : ky - ny) / (ny * dy);
    float wz = 2.0f * pi * kz / (nz * dz);

    float k_sq = wx * wx + wy * wy + wz * wz;

    if (k_sq < 1e-12f) {
        // Zero mode: enforce zero-mean pressure
        rhs_hat[idx].x = 0.0f;
        rhs_hat[idx].y = 0.0f;
    } else {
        float inv_k_sq = -1.0f / k_sq;
        rhs_hat[idx].x *= inv_k_sq;
        rhs_hat[idx].y *= inv_k_sq;
    }
}

// TODO(Week +1): Full pipeline: R2C FFT → poisson_solve_spectral → C2R IFFT
// Rust binding via cudarc cufft wrapper
