#ifndef FARMHASH_SYS_WRAPPER_H
#define FARMHASH_SYS_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

// Simple uint128_t implementation for C API
typedef struct {
    uint64_t low;
    uint64_t high;
} uint128_t;

// C API for FarmHash functions
uint32_t farmhash_hash32(const char* s, size_t len);
uint32_t farmhash_hash32_with_seed(const char* s, size_t len, uint32_t seed);
uint64_t farmhash_hash64(const char* s, size_t len);
uint64_t farmhash_hash64_with_seed(const char* s, size_t len, uint64_t seed);
uint64_t farmhash_hash64_with_seeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1);
uint128_t farmhash_fingerprint128(const char* s, size_t len);

#ifdef __cplusplus
}
#endif

#endif // FARMHASH_SYS_WRAPPER_H