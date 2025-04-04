#include "wrapper.h"
#include "src/farmhash/farmhash.h"

extern "C" {

uint32_t farmhash_hash32(const char* s, size_t len) {
    return util::Hash32(s, len);
}

uint32_t farmhash_hash32_with_seed(const char* s, size_t len, uint32_t seed) {
    return util::Hash32WithSeed(s, len, seed);
}

uint64_t farmhash_hash64(const char* s, size_t len) {
    return util::Hash64(s, len);
}

uint64_t farmhash_hash64_with_seed(const char* s, size_t len, uint64_t seed) {
    return util::Hash64WithSeed(s, len, seed);
}

uint64_t farmhash_hash64_with_seeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1) {
    return util::Hash64WithSeeds(s, len, seed0, seed1);
}

uint128_t farmhash_fingerprint128(const char* s, size_t len) {
    util::uint128_t result = util::Fingerprint128(s, len);
    uint128_t ret;
    ret.low = result.first;
    ret.high = result.second;
    return ret;
}

}