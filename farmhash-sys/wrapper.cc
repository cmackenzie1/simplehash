#include "wrapper.h"
#include "src/farmhash/farmhash.h"

extern "C" {

// 32-bit hash functions
uint32_t farmhash_hash32(const char* s, size_t len) {
    return util::Hash32(s, len);
}

uint32_t farmhash_hash32_with_seed(const char* s, size_t len, uint32_t seed) {
    return util::Hash32WithSeed(s, len, seed);
}

uint32_t farmhash_fingerprint32(const char* s, size_t len) {
    return util::Fingerprint32(s, len);
}

// 64-bit hash functions
uint64_t farmhash_hash64(const char* s, size_t len) {
    return util::Hash64(s, len);
}

uint64_t farmhash_hash64_with_seed(const char* s, size_t len, uint64_t seed) {
    return util::Hash64WithSeed(s, len, seed);
}

uint64_t farmhash_hash64_with_seeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1) {
    return util::Hash64WithSeeds(s, len, seed0, seed1);
}

uint64_t farmhash_fingerprint64(const char* s, size_t len) {
    return util::Fingerprint64(s, len);
}

uint64_t farmhash_fingerprint64_with_seed(const char* s, size_t len, uint64_t seed) {
    // Implement our own version since FarmHash doesn't provide this directly
    // We'll just use hash64_with_seed for now
    return util::Hash64WithSeed(s, len, seed);
}

// 128-bit hash functions
uint128_t farmhash_fingerprint128(const char* s, size_t len) {
    util::uint128_t result = util::Fingerprint128(s, len);
    uint128_t output;
    output.low = util::Uint128Low64(result);
    output.high = util::Uint128High64(result);
    return output;
}

uint128_t farmhash_fingerprint128_with_seed(const char* s, size_t len, uint64_t seed_low, uint64_t seed_high) {
    // Implement our own version since FarmHash doesn't provide this directly
    // We'll use Hash128WithSeed for now
    util::uint128_t seed = util::Uint128(seed_high, seed_low);
    util::uint128_t result = util::Hash128WithSeed(s, len, seed);
    uint128_t output;
    output.low = util::Uint128Low64(result);
    output.high = util::Uint128High64(result);
    return output;
}

}
