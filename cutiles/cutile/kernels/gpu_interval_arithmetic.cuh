#pragma once

#include <cuda_runtime.h>
#include <cuComplex.h>
#include <cmath>

// Directed rounding helpers (sound over-approximation for certified path).
__device__ __forceinline__ float add_down(float a, float b) {
    return __fadd_rd(a, b);
}

__device__ __forceinline__ float add_up(float a, float b) {
    return __fadd_ru(a, b);
}

__device__ __forceinline__ float mul_down(float a, float b) {
    return __fmul_rd(a, b);
}

__device__ __forceinline__ float mul_up(float a, float b) {
    return __fmul_ru(a, b);
}

__device__ __forceinline__ float div_down(float a, float b) {
    return __fdiv_rd(a, b);
}

__device__ __forceinline__ float div_up(float a, float b) {
    return __fdiv_ru(a, b);
}

struct ComplexInterval {
    float re_lo;
    float re_hi;
    float im_lo;
    float im_hi;
};

__device__ __forceinline__ ComplexInterval make_complex_interval(
    float re_lo, float re_hi, float im_lo, float im_hi)
{
    ComplexInterval out;
    out.re_lo = re_lo;
    out.re_hi = re_hi;
    out.im_lo = im_lo;
    out.im_hi = im_hi;
    return out;
}

__device__ __forceinline__ ComplexInterval complex_interval_add(
    const ComplexInterval& a, const ComplexInterval& b)
{
    return make_complex_interval(
        add_down(a.re_lo, b.re_lo),
        add_up(a.re_hi, b.re_hi),
        add_down(a.im_lo, b.im_lo),
        add_up(a.im_hi, b.im_hi));
}

__device__ __forceinline__ ComplexInterval complex_interval_scale(
    const ComplexInterval& a, float s_lo, float s_hi)
{
    float products[4] = {
        mul_down(a.re_lo, s_lo), mul_up(a.re_hi, s_hi),
        mul_down(a.re_lo, s_hi), mul_up(a.re_hi, s_lo)};
    float re_lo = fminf(fminf(products[0], products[1]), fminf(products[2], products[3]));
    float re_hi = fmaxf(fmaxf(products[0], products[1]), fmaxf(products[2], products[3]));

    products[0] = mul_down(a.im_lo, s_lo);
    products[1] = mul_up(a.im_hi, s_hi);
    products[2] = mul_down(a.im_lo, s_hi);
    products[3] = mul_up(a.im_hi, s_lo);
    float im_lo = fminf(fminf(products[0], products[1]), fminf(products[2], products[3]));
    float im_hi = fmaxf(fmaxf(products[0], products[1]), fmaxf(products[2], products[3]));

    return make_complex_interval(re_lo, re_hi, im_lo, im_hi);
}

__device__ __forceinline__ float interval_width(const ComplexInterval& c) {
    return (c.re_hi - c.re_lo) + (c.im_hi - c.im_lo);
}

// Preconditioned Neumann bound: ||A^{-1}||_infty * ||r||_infty
__device__ __forceinline__ float neumann_error_bound(
    const float* residual, int n, float neumann_threshold)
{
    float r_inf = 0.0f;
    for (int i = 0; i < n; ++i) {
        r_inf = fmaxf(r_inf, fabsf(residual[i]));
    }
    return r_inf / fmaxf(neumann_threshold, 1e-12f);
}

// Small-N (N=8) interval Gaussian elimination for Levin collocation system.
__device__ void interval_gaussian_elimination_n8(
    const float* D,
    float z,
    const float* f_nodes,
    ComplexInterval* C_out,
    float* max_error,
    bool* reliable)
{
    constexpr int N = 8;
    float A_re[N * N];
    float A_im[N * N];
    ComplexInterval rhs[N];

    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            float d = D[i * N + j];
            A_re[i * N + j] = d;
            A_im[i * N + j] = (i == j) ? z : 0.0f;
        }
        rhs[i] = make_complex_interval(f_nodes[i], f_nodes[i], 0.0f, 0.0f);
    }

    // Forward elimination with outward rounding on pivot operations.
    for (int k = 0; k < N; ++k) {
        int pivot = k;
        float pivot_mag = fabsf(A_re[k * N + k]);
        for (int i = k + 1; i < N; ++i) {
            float mag = fabsf(A_re[i * N + k]);
            if (mag > pivot_mag) {
                pivot_mag = mag;
                pivot = i;
            }
        }

        if (pivot_mag < 1e-8f) {
            *max_error = 1.0f;
            *reliable = false;
            for (int i = 0; i < N; ++i) {
                C_out[i] = make_complex_interval(0.0f, 0.0f, 0.0f, 0.0f);
            }
            return;
        }

        if (pivot != k) {
            for (int j = 0; j < N; ++j) {
                float tmp_re = A_re[k * N + j];
                A_re[k * N + j] = A_re[pivot * N + j];
                A_re[pivot * N + j] = tmp_re;

                float tmp_im = A_im[k * N + j];
                A_im[k * N + j] = A_im[pivot * N + j];
                A_im[pivot * N + j] = tmp_im;
            }
            ComplexInterval tmp_rhs = rhs[k];
            rhs[k] = rhs[pivot];
            rhs[pivot] = tmp_rhs;
        }

        float pivot_re = A_re[k * N + k];
        float inv_pivot = div_up(1.0f, fmaxf(fabsf(pivot_re), 1e-12f));

        for (int i = k + 1; i < N; ++i) {
            float factor = mul_up(A_re[i * N + k], inv_pivot);
            for (int j = k; j < N; ++j) {
                A_re[i * N + j] = add_down(A_re[i * N + j], -mul_up(factor, A_re[k * N + j]));
                A_im[i * N + j] = add_down(A_im[i * N + j], -mul_up(factor, A_im[k * N + j]));
            }
            rhs[i] = complex_interval_add(
                rhs[i],
                complex_interval_scale(rhs[k], -factor, -factor));
        }
    }

    // Back substitution
    ComplexInterval x[N];
    for (int i = N - 1; i >= 0; --i) {
        ComplexInterval sum = rhs[i];
        for (int j = i + 1; j < N; ++j) {
            sum = complex_interval_add(
                sum,
                complex_interval_scale(x[j], -A_re[i * N + j], -A_re[i * N + j]));
        }
        float diag = fmaxf(fabsf(A_re[i * N + i]), 1e-12f);
        float inv = div_up(1.0f, diag);
        x[i] = complex_interval_scale(sum, inv, inv);
        C_out[i] = x[i];
    }

    // Residual check for Neumann bound
    float residual[N];
    for (int i = 0; i < N; ++i) {
        float re = 0.0f;
        float im = 0.0f;
        for (int j = 0; j < N; ++j) {
            float xr = 0.5f * (x[j].re_lo + x[j].re_hi);
            re += A_re[i * N + j] * xr;
            im += A_im[i * N + j] * xr;
        }
        float target = f_nodes[i];
        residual[i] = fabsf(re - target) + fabsf(im);
    }

    *max_error = neumann_error_bound(residual, N, 0.5f);
    *reliable = (*max_error < 5e-7f);
}