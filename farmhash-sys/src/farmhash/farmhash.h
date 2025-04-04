// Minimal FarmHash header containing only essential functions
// Based on the original FarmHash implementation (https://github.com/google/farmhash)
// Copyright (c) 2014 Google, Inc. (MIT License)

#ifndef FARMHASH_MINIMAL_H
#define FARMHASH_MINIMAL_H

#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <utility>

namespace util {

// Basic hash functions
uint32_t Hash32(const char* s, size_t len);
uint32_t Hash32WithSeed(const char* s, size_t len, uint32_t seed);
uint64_t Hash64(const char* s, size_t len);
uint64_t Hash64WithSeed(const char* s, size_t len, uint64_t seed);
uint64_t Hash64WithSeeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1);

// 128-bit fingerprint
typedef std::pair<uint64_t, uint64_t> uint128_t;
uint128_t Fingerprint128(const char* s, size_t len);

}  // namespace util

#endif  // FARMHASH_MINIMAL_H