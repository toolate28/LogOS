// fundamental_r_matrix.cu
// Mirrors Rust nalgebra / cutile::core::r_matrix fundamental R-matrix on CUDA.
// Invariant chain: α+ω=15 (caller-side conservation); R entries use q, 1/q, 1-q².
// Cascade layer: CUDA (L3)

#include "fundamental_r_matrix.cuh"

__device__ __forceinline__ cuDoubleComplex make_complex(double re, double im) {
    return make_cuDoubleComplex(re, im);
}

// Fundamental R-matrix for U_q(sl_2)-style two-rail braiding (row-major 4×4)
__device__ void fundamental_r_matrix(double q, CMatrix4* out) {
    double q_inv = 1.0 / q;
    double off   = 1.0 - q * q;

    // Row 0
    (*out)[0]  = make_complex(q,     0.0);
    (*out)[1]  = make_complex(0.0,   0.0);
    (*out)[2]  = make_complex(0.0,   0.0);
    (*out)[3]  = make_complex(0.0,   0.0);

    // Row 1
    (*out)[4]  = make_complex(0.0,   0.0);
    (*out)[5]  = make_complex(q_inv, 0.0);
    (*out)[6]  = make_complex(off,   0.0);
    (*out)[7]  = make_complex(0.0,   0.0);

    // Row 2
    (*out)[8]  = make_complex(0.0,   0.0);
    (*out)[9]  = make_complex(0.0,   0.0);
    (*out)[10] = make_complex(q,     0.0);
    (*out)[11] = make_complex(0.0,   0.0);

    // Row 3
    (*out)[12] = make_complex(0.0,   0.0);
    (*out)[13] = make_complex(0.0,   0.0);
    (*out)[14] = make_complex(0.0,   0.0);
    (*out)[15] = make_complex(q_inv, 0.0);
}

__global__ void apply_r_matrix_kernel(double q, CMatrix4* matrices, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        fundamental_r_matrix(q, &matrices[idx]);
    }
}
