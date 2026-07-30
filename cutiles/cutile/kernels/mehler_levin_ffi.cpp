#include "mehler_levin_ffi.h"

#include <cuda_runtime.h>
#include <cuComplex.h>

#include <cstdint>
#include <cstring>
#include <memory>
#include <string>
#include <vector>

namespace {

constexpr int N_LEVIN = 8;
constexpr int BLOCK_SIZE = 256;

thread_local std::string last_error_msg;

void set_error(const std::string& msg) {
    last_error_msg = msg;
}

MehlerError check_cuda(cudaError_t err, const char* msg) {
    if (err != cudaSuccess) {
        set_error(std::string(msg) + ": " + cudaGetErrorString(err));
        return MEHLER_CUDA_ERROR;
    }
    return MEHLER_SUCCESS;
}

// Host-side default Chebyshev differentiation matrix + uniform Levin weights.
void init_default_constants() {
    static bool initialized = false;
    if (initialized) {
        return;
    }

    float D[N_LEVIN * N_LEVIN] = {};
    float weights[N_LEVIN] = {};

    for (int i = 0; i < N_LEVIN; ++i) {
        weights[i] = 2.0f / static_cast<float>(N_LEVIN);
        for (int j = 0; j < N_LEVIN; ++j) {
            if (i == j) {
                D[i * N_LEVIN + j] = 0.0f;
            } else {
                float xi = -1.0f + 2.0f * static_cast<float>(i) / static_cast<float>(N_LEVIN - 1);
                float xj = -1.0f + 2.0f * static_cast<float>(j) / static_cast<float>(N_LEVIN - 1);
                D[i * N_LEVIN + j] = 1.0f / (xi - xj);
            }
        }
    }

    for (int i = 0; i < N_LEVIN; ++i) {
        float row_sum = 0.0f;
        for (int j = 0; j < N_LEVIN; ++j) {
            if (i != j) {
                row_sum += D[i * N_LEVIN + j];
            }
        }
        D[i * N_LEVIN + i] = -row_sum;
    }

    if (check_cuda(mehler_init_constants(D, weights, N_LEVIN), "Constant init") == MEHLER_SUCCESS) {
        initialized = true;
    }
}

}  // namespace

struct MehlerLevinContext {
    float t;
    bool certified_mode;
    cudaStream_t stream;
    float* d_z;
    float* d_f_nodes;
    cuComplex* d_point;
    float* d_max_error;
    bool* d_reliable;
    int allocated_batch_size;
};

extern "C" cudaError_t mehler_init_constants(
    const float* D_host,
    const float* weights_host,
    int n);

extern "C" void mehler_mma_levin_batched(
    float t,
    const float* z_batch,
    const float* f_nodes_batch,
    cuComplex* point_out,
    void* interval_out,
    float* max_error_out,
    bool* reliable_out,
    int batch_size,
    bool certified_mode);

const char* mehler_levin_get_last_error(void) {
    return last_error_msg.c_str();
}

MehlerLevinContext* mehler_levin_create(float t, bool enable_certified_mode) {
    init_default_constants();

    auto ctx = std::make_unique<MehlerLevinContext>();
    ctx->t = t;
    ctx->certified_mode = enable_certified_mode;
    ctx->allocated_batch_size = 0;
    ctx->d_z = nullptr;
    ctx->d_f_nodes = nullptr;
    ctx->d_point = nullptr;
    ctx->d_max_error = nullptr;
    ctx->d_reliable = nullptr;

    if (check_cuda(cudaStreamCreate(&ctx->stream), "Stream creation") != MEHLER_SUCCESS) {
        return nullptr;
    }

    return ctx.release();
}

void mehler_levin_destroy(MehlerLevinContext* ctx) {
    if (!ctx) {
        return;
    }

    cudaStreamSynchronize(ctx->stream);
    cudaFree(ctx->d_z);
    cudaFree(ctx->d_f_nodes);
    cudaFree(ctx->d_point);
    cudaFree(ctx->d_max_error);
    cudaFree(ctx->d_reliable);
    cudaStreamDestroy(ctx->stream);
    delete ctx;
}

static MehlerError ensure_device_buffers(MehlerLevinContext* ctx, int batch_size) {
    if (batch_size <= ctx->allocated_batch_size) {
        return MEHLER_SUCCESS;
    }

    cudaFree(ctx->d_z);
    cudaFree(ctx->d_f_nodes);
    cudaFree(ctx->d_point);
    cudaFree(ctx->d_max_error);
    cudaFree(ctx->d_reliable);

    ctx->d_z = nullptr;
    ctx->d_f_nodes = nullptr;
    ctx->d_point = nullptr;
    ctx->d_max_error = nullptr;
    ctx->d_reliable = nullptr;

    MehlerError err;
    err = check_cuda(
        cudaMalloc(&ctx->d_z, static_cast<size_t>(batch_size) * sizeof(float)),
        "d_z allocation");
    if (err != MEHLER_SUCCESS) {
        return MEHLER_OUT_OF_MEMORY;
    }

    err = check_cuda(
        cudaMalloc(
            &ctx->d_f_nodes,
            static_cast<size_t>(batch_size) * N_LEVIN * sizeof(float)),
        "d_f_nodes allocation");
    if (err != MEHLER_SUCCESS) {
        return MEHLER_OUT_OF_MEMORY;
    }

    err = check_cuda(
        cudaMalloc(&ctx->d_point, static_cast<size_t>(batch_size) * sizeof(cuComplex)),
        "d_point allocation");
    if (err != MEHLER_SUCCESS) {
        return MEHLER_OUT_OF_MEMORY;
    }

    if (ctx->certified_mode) {
        err = check_cuda(
            cudaMalloc(&ctx->d_max_error, static_cast<size_t>(batch_size) * sizeof(float)),
            "d_max_error allocation");
        if (err != MEHLER_SUCCESS) {
            return MEHLER_OUT_OF_MEMORY;
        }

        err = check_cuda(
            cudaMalloc(&ctx->d_reliable, static_cast<size_t>(batch_size) * sizeof(bool)),
            "d_reliable allocation");
        if (err != MEHLER_SUCCESS) {
            return MEHLER_OUT_OF_MEMORY;
        }
    }

    ctx->allocated_batch_size = batch_size;
    return MEHLER_SUCCESS;
}

MehlerError mehler_levin_evaluate(
    MehlerLevinContext* ctx,
    const float* z_host,
    const float* f_nodes_host,
    float* point_real_host,
    float* point_imag_host,
    float* max_error_host,
    bool* reliable_host,
    int batch_size)
{
    if (!ctx || !z_host || !f_nodes_host || !point_real_host || !point_imag_host || batch_size <= 0) {
        set_error("Invalid arguments");
        return MEHLER_INVALID_ARGUMENT;
    }

    if (ctx->certified_mode && (!max_error_host || !reliable_host)) {
        set_error("Certified mode requires max_error_host and reliable_host");
        return MEHLER_INVALID_ARGUMENT;
    }

    MehlerError alloc_err = ensure_device_buffers(ctx, batch_size);
    if (alloc_err != MEHLER_SUCCESS) {
        return alloc_err;
    }

    size_t z_bytes = static_cast<size_t>(batch_size) * sizeof(float);
    size_t f_bytes = static_cast<size_t>(batch_size) * N_LEVIN * sizeof(float);

    MehlerError err = check_cuda(
        cudaMemcpyAsync(ctx->d_z, z_host, z_bytes, cudaMemcpyHostToDevice, ctx->stream),
        "z H2D copy");
    if (err != MEHLER_SUCCESS) {
        return err;
    }

    err = check_cuda(
        cudaMemcpyAsync(ctx->d_f_nodes, f_nodes_host, f_bytes, cudaMemcpyHostToDevice, ctx->stream),
        "f_nodes H2D copy");
    if (err != MEHLER_SUCCESS) {
        return err;
    }

    dim3 block(BLOCK_SIZE);
    dim3 grid((batch_size + block.x - 1) / block.x);

    mehler_mma_levin_batched<<<grid, block, 0, ctx->stream>>>(
        ctx->t,
        ctx->d_z,
        ctx->d_f_nodes,
        ctx->d_point,
        nullptr,
        ctx->certified_mode ? ctx->d_max_error : nullptr,
        ctx->certified_mode ? ctx->d_reliable : nullptr,
        batch_size,
        ctx->certified_mode);

    err = check_cuda(cudaGetLastError(), "Kernel launch");
    if (err != MEHLER_SUCCESS) {
        return err;
    }

    std::vector<cuComplex> host_point(static_cast<size_t>(batch_size));
    err = check_cuda(
        cudaMemcpyAsync(
            host_point.data(),
            ctx->d_point,
            static_cast<size_t>(batch_size) * sizeof(cuComplex),
            cudaMemcpyDeviceToHost,
            ctx->stream),
        "point D2H copy");
    if (err != MEHLER_SUCCESS) {
        return err;
    }

    if (ctx->certified_mode) {
        std::vector<float> host_max_error(static_cast<size_t>(batch_size));
        std::vector<bool> host_reliable(static_cast<size_t>(batch_size));

        err = check_cuda(
            cudaMemcpyAsync(
                host_max_error.data(),
                ctx->d_max_error,
                static_cast<size_t>(batch_size) * sizeof(float),
                cudaMemcpyDeviceToHost,
                ctx->stream),
            "max_error D2H copy");
        if (err != MEHLER_SUCCESS) {
            return err;
        }

        err = check_cuda(
            cudaMemcpyAsync(
                host_reliable.data(),
                ctx->d_reliable,
                static_cast<size_t>(batch_size) * sizeof(bool),
                cudaMemcpyDeviceToHost,
                ctx->stream),
            "reliable D2H copy");
        if (err != MEHLER_SUCCESS) {
            return err;
        }

        err = check_cuda(cudaStreamSynchronize(ctx->stream), "Stream sync");
        if (err != MEHLER_SUCCESS) {
            return err;
        }

        for (int i = 0; i < batch_size; ++i) {
            point_real_host[i] = host_point[static_cast<size_t>(i)].x;
            point_imag_host[i] = host_point[static_cast<size_t>(i)].y;
            max_error_host[i] = host_max_error[static_cast<size_t>(i)];
            reliable_host[i] = host_reliable[static_cast<size_t>(i)];
            if (!reliable_host[i]) {
                set_error("Certified mode reliability check failed for one or more batch elements");
                return MEHLER_CERTIFIED_MODE_FAILURE;
            }
        }
    } else {
        err = check_cuda(cudaStreamSynchronize(ctx->stream), "Stream sync");
        if (err != MEHLER_SUCCESS) {
            return err;
        }

        for (int i = 0; i < batch_size; ++i) {
            point_real_host[i] = host_point[static_cast<size_t>(i)].x;
            point_imag_host[i] = host_point[static_cast<size_t>(i)].y;
        }
    }

    return MEHLER_SUCCESS;
}