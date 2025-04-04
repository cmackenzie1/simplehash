#include "farmhash.h"
#include <utility>
#include <string.h>
#include <stdio.h>

namespace util {

// Simple implementation of FarmHash functions using the xxHash algorithm
// These are simplified implementations for binding purposes

// Implementation based on xxHash32 algorithm (simplified)
uint32_t Hash32(const char* s, size_t len) {
    const uint32_t PRIME1 = 2654435761U;
    const uint32_t PRIME2 = 2246822519U;
    const uint32_t PRIME3 = 3266489917U;
    const uint32_t PRIME4 = 668265263U;
    const uint32_t PRIME5 = 374761393U;

    uint32_t h = PRIME5;
    if (len > 0) {
        h += len * PRIME3;
        
        // Process 4 bytes at a time
        const uint32_t* p = (const uint32_t*)s;
        const uint32_t* end = p + (len / 4);
        
        while (p < end) {
            h = ((h + (*p * PRIME2)) << 13) ^ (h << 1);
            p++;
        }
        
        // Process remaining bytes
        const uint8_t* p8 = (const uint8_t*)p;
        const uint8_t* end8 = (const uint8_t*)s + len;
        
        while (p8 < end8) {
            h = ((h + (*p8 * PRIME1)) << 11) ^ (h << 1);
            p8++;
        }
    }
    
    h ^= h >> 15;
    h *= PRIME2;
    h ^= h >> 13;
    h *= PRIME3;
    h ^= h >> 16;
    
    return h;
}

uint32_t Hash32WithSeed(const char* s, size_t len, uint32_t seed) {
    return Hash32(s, len) ^ seed;
}

// Implementation based on xxHash64 algorithm (simplified)
uint64_t Hash64(const char* s, size_t len) {
    const uint64_t PRIME1 = 11400714785074694791ULL;
    const uint64_t PRIME2 = 14029467366897019727ULL;
    const uint64_t PRIME3 = 1609587929392839161ULL;
    const uint64_t PRIME4 = 9650029242287828579ULL;
    const uint64_t PRIME5 = 2870177450012600261ULL;
    
    uint64_t h = PRIME5;
    if (len > 0) {
        h += len * PRIME3;
        
        // Process 8 bytes at a time
        const uint64_t* p = (const uint64_t*)s;
        const uint64_t* end = p + (len / 8);
        
        while (p < end) {
            uint64_t k = *p++;
            k *= PRIME2;
            k = (k << 31) | (k >> 33);
            k *= PRIME1;
            h ^= k;
            h = ((h << 27) | (h >> 37)) * PRIME1 + PRIME4;
        }
        
        // Process remaining bytes
        const uint8_t* p8 = (const uint8_t*)p;
        const uint8_t* end8 = (const uint8_t*)s + len;
        
        while (p8 < end8) {
            h ^= (*p8) * PRIME5;
            h = ((h << 11) | (h >> 53)) * PRIME1;
            p8++;
        }
    }
    
    h ^= h >> 33;
    h *= PRIME2;
    h ^= h >> 29;
    h *= PRIME3;
    h ^= h >> 32;
    
    return h;
}

uint64_t Hash64WithSeed(const char* s, size_t len, uint64_t seed) {
    return Hash64(s, len) ^ seed;
}

uint64_t Hash64WithSeeds(const char* s, size_t len, uint64_t seed0, uint64_t seed1) {
    return Hash64WithSeed(s, len, seed0) ^ seed1;
}

// Implementation of Fingerprint128 using two Hash64 calls
uint128_t Fingerprint128(const char* s, size_t len) {
    uint64_t h1 = Hash64(s, len);
    
    // Modify seed for second hash to ensure different result
    uint64_t h2 = Hash64WithSeed(s, len, 0x9E3779B97F4A7C15ULL);
    
    return std::make_pair(h1, h2);
}

}  // namespace util