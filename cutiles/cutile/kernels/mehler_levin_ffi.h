#pragma once

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum MehlerError {
    MEHLER_SUCCESS = 0,
    MEHLER_INVALID_ARGUMENT = 1,
    MEHLER_CUDA_ERROR = 2,
    MEHLER_CERTIFIED_MODE_FAILURE = 3,
    MEHLER_OUT_OF_MEMORY = 4,
    MEHLER_UNKNOWN = 99
} MehlerError;

typedef struct MehlerLevinContext MehlerLevinContext;

MehlerLevinContext* mehler_levin_create(float t, bool enable_certified_mode);
void mehler_levin_destroy(MehlerLevinContext* ctx);

const char* mehler_levin_get_last_error(void);

MehlerError mehler_levin_evaluate(
    MehlerLevinContext* ctx,
    const float* z_host,
    const float* f_nodes_host,
    float* point_real_host,
    float* point_imag_host,
    float* max_error_host,
    bool* reliable_host,
    int batch_size);

#ifdef __cplusplus
}
#endif