// cutiles/r_matrix_host.cpp
// C++ host wrapper for fundamental_r_matrix CUDA kernel + CPU reference.
// Cascade layer: C++ host (L9)

#include "../kernels/fundamental_r_matrix.cuh"
#include "../kernels/r_matrix_interface.h"

#include <cuda_runtime.h>
#include <iostream>
#include <cmath>

// CPU reference matching device algebra (no GPU required for unit checks)
int fundamental_r_matrix_host(double q, RMatrixC64* out) {
    if (out == nullptr || q == 0.0 || !std::isfinite(q)) {
        return 1;
    }
    double q_inv = 1.0 / q;
    double off = 1.0 - q * q;
    // zero
    for (int i = 0; i < 32; ++i) out->re_im[i] = 0.0;
    // (0,0)=q  (1,1)=1/q  (1,2)=off  (2,2)=q  (3,3)=1/q
    auto set = [&](int row, int col, double re, double im) {
        int i = (row * 4 + col) * 2;
        out->re_im[i] = re;
        out->re_im[i + 1] = im;
    };
    set(0, 0, q, 0.0);
    set(1, 1, q_inv, 0.0);
    set(1, 2, off, 0.0);
    set(2, 2, q, 0.0);
    set(3, 3, q_inv, 0.0);
    return 0;
}

void launch_r_matrix(double q, int n_matrices) {
    CMatrix4* d_matrices = nullptr;
    cudaError_t err = cudaMalloc(&d_matrices, n_matrices * sizeof(CMatrix4));
    if (err != cudaSuccess) {
        std::cerr << "cudaMalloc failed: " << cudaGetErrorString(err) << std::endl;
        return;
    }

    int threads = 64;
    int blocks = (n_matrices + threads - 1) / threads;
    apply_r_matrix_kernel<<<blocks, threads>>>(q, d_matrices, n_matrices);
    cudaDeviceSynchronize();

    CMatrix4 h_result{};
    cudaMemcpy(&h_result, d_matrices, sizeof(CMatrix4), cudaMemcpyDeviceToHost);
    std::cout << "R-matrix computed on GPU for q=" << q
              << " R[0]=" << cuCreal(h_result[0]) << std::endl;

    cudaFree(d_matrices);
}
