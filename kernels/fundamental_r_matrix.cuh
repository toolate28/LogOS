#pragma once
// fundamental_r_matrix.cuh
// Device contract mirroring Rust cutile::core::r_matrix / nalgebra Matrix4<Complex>
// Cascade layer: CUDA header (L3)

#include <cuComplex.h>

// 4×4 complex matrix, row-major
using CMatrix4 = cuDoubleComplex[16];

__device__ void fundamental_r_matrix(double q, CMatrix4* out);

__global__ void apply_r_matrix_kernel(double q, CMatrix4* matrices, int n);
