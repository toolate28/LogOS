/**
 * r_matrix_interface.h — language-agnostic contract for fundamental R-matrix
 * Cascade layer: unified kernel interface (L11)
 *
 * Structural identity across Rust / CUDA / WGSL / Python:
 *   R = diag-block form with entries q, 1/q, and off-diagonal (1 - q²)
 *   Layout: row-major 4×4 over complex doubles (re, im interleaved or paired)
 *
 * Conservation law (orthogonal but coupled): α + ω = 15
 */

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

/** Flattened row-major complex matrix: 16 pairs (re, im) → 32 doubles. */
typedef struct RMatrixC64 {
    double re_im[32];
} RMatrixC64;

/**
 * Host-callable pure fill (CPU reference). Implementations may also expose
 * GPU kernels (fundamental_r_matrix.cu) with the same algebra.
 *
 * @param q  deformation parameter (must be != 0)
 * @param out filled R matrix
 * @return 0 on success, non-zero on invalid q
 */
int fundamental_r_matrix_host(double q, RMatrixC64* out);

#ifdef __cplusplus
}
#endif
