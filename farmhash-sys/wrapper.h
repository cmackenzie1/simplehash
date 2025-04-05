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

// 32-bit hash functions
uint32_t farmhash_hash32(const char* s, size_t len);
uint32_t farmhash_hash32_with_seed(const char* s, size_t len, uint32_t seed);
uint32_t farmhash_fingerprint32(const char* s, size_t len);

// 64-bit hash functions
uint64_t farmhash_hash64(const char* s, size_t len);
uint64_t farmhash_hash64_with_seed(const char* s, size_t len, uint64_t seed);
uint64_t farmhash_hash64_with_seeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1);
uint64_t farmhash_fingerprint64(const char* s, size_t len);
uint64_t farmhash_fingerprint64_with_seed(const char* s, size_t len, uint64_t seed);

// 128-bit hash functions
uint128_t farmhash_fingerprint128(const char* s, size_t len);
uint128_t farmhash_fingerprint128_with_seed(const char* s, size_t len, uint64_t seed_low, uint64_t seed_high);

#ifdef __cplusplus
}
#endif

#endif // FARMHASH_SYS_WRAPPER_H