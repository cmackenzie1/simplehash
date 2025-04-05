#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_char;

//------------------------------------------------------------------------------
// 32-bit hash functions
//------------------------------------------------------------------------------

/// Compute `FarmHash32` for a byte slice
pub fn farm_hash_32(bytes: &[u8]) -> u32 {
    unsafe { farmhash_hash32(bytes.as_ptr().cast::<c_char>(), bytes.len()) }
}

/// Compute `FarmHash32` for a byte slice with a seed
pub fn farm_hash_32_with_seed(bytes: &[u8], seed: u32) -> u32 {
    unsafe { farmhash_hash32_with_seed(bytes.as_ptr().cast::<c_char>(), bytes.len(), seed) }
}

/// Compute `FarmHash32` fingerprint for a byte slice
pub fn farm_fingerprint_32(bytes: &[u8]) -> u32 {
    unsafe { farmhash_fingerprint32(bytes.as_ptr().cast::<c_char>(), bytes.len()) }
}

//------------------------------------------------------------------------------
// 64-bit hash functions
//------------------------------------------------------------------------------

/// Compute `FarmHash64` for a byte slice
pub fn farm_hash_64(bytes: &[u8]) -> u64 {
    unsafe { farmhash_hash64(bytes.as_ptr().cast::<c_char>(), bytes.len()) }
}

/// Compute `FarmHash64` for a byte slice with a seed
pub fn farm_hash_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    unsafe { farmhash_hash64_with_seed(bytes.as_ptr().cast::<c_char>(), bytes.len(), seed) }
}

/// Compute `FarmHash64` for a byte slice with two seeds
pub fn farm_hash_64_with_seeds(bytes: &[u8], seed0: u64, seed1: u64) -> u64 {
    unsafe {
        farmhash_hash64_with_seeds(bytes.as_ptr().cast::<c_char>(), bytes.len(), seed0, seed1)
    }
}

/// Compute `FarmHash64` fingerprint for a byte slice
pub fn farm_fingerprint_64(bytes: &[u8]) -> u64 {
    unsafe { farmhash_fingerprint64(bytes.as_ptr().cast::<c_char>(), bytes.len()) }
}

/// Compute `FarmHash64` fingerprint for a byte slice with a seed
pub fn farm_fingerprint_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    unsafe { farmhash_fingerprint64_with_seed(bytes.as_ptr().cast::<c_char>(), bytes.len(), seed) }
}

//------------------------------------------------------------------------------
// 128-bit hash functions
//------------------------------------------------------------------------------

/// A 128-bit hash value, represented as a pair of 64-bit values
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Uint128 {
    /// The lower 64 bits of the hash
    pub low: u64,
    /// The upper 64 bits of the hash
    pub high: u64,
}

impl Uint128 {
    /// Create a new 128-bit hash from low and high 64-bit parts
    pub fn new(low: u64, high: u64) -> Self {
        Self { low, high }
    }
}

impl From<(u64, u64)> for Uint128 {
    fn from(value: (u64, u64)) -> Self {
        Self {
            low: value.0,
            high: value.1,
        }
    }
}

impl From<Uint128> for (u64, u64) {
    fn from(value: Uint128) -> Self {
        (value.low, value.high)
    }
}

/// Compute `FarmHash128` fingerprint for a byte slice
pub fn farm_fingerprint_128(bytes: &[u8]) -> Uint128 {
    unsafe {
        let result = farmhash_fingerprint128(bytes.as_ptr().cast::<c_char>(), bytes.len());
        Uint128 {
            low: result.low,
            high: result.high,
        }
    }
}

/// Compute `FarmHash128` fingerprint for a byte slice with a seed
pub fn farm_fingerprint_128_with_seed(bytes: &[u8], seed: Uint128) -> Uint128 {
    unsafe {
        let result = farmhash_fingerprint128_with_seed(
            bytes.as_ptr().cast::<c_char>(),
            bytes.len(),
            seed.low,
            seed.high,
        );
        Uint128 {
            low: result.low,
            high: result.high,
        }
    }
}

//------------------------------------------------------------------------------
// Hasher implementation
//------------------------------------------------------------------------------

use std::hash::Hasher;

/// Hasher implementation for `FarmHash`
pub struct FarmHashHasher {
    buffer: Vec<u8>,
    seed: Option<u64>,
}

impl FarmHashHasher {
    /// Create a new `FarmHashHasher` with no seed
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            seed: None,
        }
    }

    /// Create a new `FarmHashHasher` with the specified seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            buffer: Vec::new(),
            seed: Some(seed),
        }
    }
}

impl Default for FarmHashHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for FarmHashHasher {
    fn finish(&self) -> u64 {
        if let Some(seed) = self.seed {
            farm_hash_64_with_seed(&self.buffer, seed)
        } else {
            farm_hash_64(&self.buffer)
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        // FarmHash hasher does not support incremental hashing, so we accumulate the bytes
        // and hash the entire buffer at once
        self.buffer.extend_from_slice(bytes);
    }
}

#[allow(dead_code)]
/// Default hasher for `FarmHash`
type FarmHashHasherDefault = std::hash::BuildHasherDefault<FarmHashHasher>;

//------------------------------------------------------------------------------
// Backwards compatibility module
//------------------------------------------------------------------------------

// We also provide the original module for backward compatibility or for users who prefer that style
pub mod farmhash {
    use super::{
        Uint128, farm_fingerprint_32, farm_fingerprint_64, farm_fingerprint_64_with_seed,
        farm_fingerprint_128, farm_fingerprint_128_with_seed, farm_hash_32, farm_hash_32_with_seed,
        farm_hash_64, farm_hash_64_with_seed, farm_hash_64_with_seeds,
    };

    //------------------------------------------------------------------------------
    // 32-bit hash functions
    //------------------------------------------------------------------------------

    /// Hash function for a byte array, returning a 32-bit hash
    pub fn hash32(bytes: &[u8]) -> u32 {
        farm_hash_32(bytes)
    }

    /// Hash function for a byte array with a 32-bit seed, returning a 32-bit hash
    pub fn hash32_with_seed(bytes: &[u8], seed: u32) -> u32 {
        farm_hash_32_with_seed(bytes, seed)
    }

    /// Fingerprint function for a byte array, returning a 32-bit fingerprint
    pub fn fingerprint32(bytes: &[u8]) -> u32 {
        farm_fingerprint_32(bytes)
    }

    //------------------------------------------------------------------------------
    // 64-bit hash functions
    //------------------------------------------------------------------------------

    /// Hash function for a byte array, returning a 64-bit hash
    pub fn hash64(bytes: &[u8]) -> u64 {
        farm_hash_64(bytes)
    }

    /// Hash function for a byte array with a 64-bit seed, returning a 64-bit hash
    pub fn hash64_with_seed(bytes: &[u8], seed: u64) -> u64 {
        farm_hash_64_with_seed(bytes, seed)
    }

    /// Hash function for a byte array with two 64-bit seeds, returning a 64-bit hash
    pub fn hash64_with_seeds(bytes: &[u8], seed0: u64, seed1: u64) -> u64 {
        farm_hash_64_with_seeds(bytes, seed0, seed1)
    }

    /// Fingerprint function for a byte array, returning a 64-bit fingerprint
    pub fn fingerprint64(bytes: &[u8]) -> u64 {
        farm_fingerprint_64(bytes)
    }

    /// Fingerprint function for a byte array with a seed, returning a 64-bit fingerprint
    pub fn fingerprint64_with_seed(bytes: &[u8], seed: u64) -> u64 {
        farm_fingerprint_64_with_seed(bytes, seed)
    }

    //------------------------------------------------------------------------------
    // 128-bit hash functions
    //------------------------------------------------------------------------------

    /// Fingerprint function for a byte array, returning a 128-bit fingerprint
    pub fn fingerprint128(bytes: &[u8]) -> Uint128 {
        farm_fingerprint_128(bytes)
    }

    /// Fingerprint function for a byte array with a seed, returning a 128-bit fingerprint
    pub fn fingerprint128_with_seed(bytes: &[u8], seed: Uint128) -> Uint128 {
        farm_fingerprint_128_with_seed(bytes, seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //------------------------------------------------------------------------------
    // 32-bit hash tests
    //------------------------------------------------------------------------------

    #[test]
    fn test_farm_hash_32() {
        let data = b"hello world";
        let hash = farm_hash_32(data);
        // Test consistency
        assert_eq!(hash, farm_hash_32(data));
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_farm_hash_32_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let hash = farm_hash_32_with_seed(data, seed);
        // Test consistency
        assert_eq!(hash, farm_hash_32_with_seed(data, seed));
        // Test that different seeds produce different hashes
        assert_ne!(hash, farm_hash_32_with_seed(data, seed + 1));
    }

    #[test]
    fn test_farm_fingerprint_32() {
        let data = b"hello world";
        let fingerprint = farm_fingerprint_32(data);
        // Test consistency
        assert_eq!(fingerprint, farm_fingerprint_32(data));
        assert_ne!(fingerprint, 0);
    }

    //------------------------------------------------------------------------------
    // 64-bit hash tests
    //------------------------------------------------------------------------------

    #[test]
    fn test_farm_hash_64() {
        let data = b"hello world";
        let hash = farm_hash_64(data);
        // Test consistency
        assert_eq!(hash, farm_hash_64(data));
        assert_ne!(hash, 0);
    }

    #[test]
    fn test_farm_hash_64_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let hash = farm_hash_64_with_seed(data, seed);
        // Test consistency
        assert_eq!(hash, farm_hash_64_with_seed(data, seed));
        // Test that different seeds produce different hashes
        assert_ne!(hash, farm_hash_64_with_seed(data, seed + 1));
    }

    #[test]
    fn test_farm_hash_64_with_seeds() {
        let data = b"hello world";
        let seed0 = 123456789;
        let seed1 = 987654321;
        let hash = farm_hash_64_with_seeds(data, seed0, seed1);
        // Test consistency
        assert_eq!(hash, farm_hash_64_with_seeds(data, seed0, seed1));
        // Test that different seeds produce different hashes
        assert_ne!(hash, farm_hash_64_with_seeds(data, seed0 + 1, seed1));
    }

    #[test]
    fn test_farm_fingerprint_64() {
        let data = b"hello world";
        let fingerprint = farm_fingerprint_64(data);
        // Test consistency
        assert_eq!(fingerprint, farm_fingerprint_64(data));
        assert_ne!(fingerprint, 0);
    }

    #[test]
    fn test_farm_fingerprint_64_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let fingerprint = farm_fingerprint_64_with_seed(data, seed);
        // Test consistency
        assert_eq!(fingerprint, farm_fingerprint_64_with_seed(data, seed));
        // Test that different seeds produce different hashes
        assert_ne!(fingerprint, farm_fingerprint_64_with_seed(data, seed + 1));
    }

    //------------------------------------------------------------------------------
    // 128-bit hash tests
    //------------------------------------------------------------------------------

    #[test]
    fn test_farm_fingerprint_128() {
        let data = b"hello world";
        let fingerprint = farm_fingerprint_128(data);
        // Test consistency
        let fingerprint2 = farm_fingerprint_128(data);
        assert_eq!(fingerprint, fingerprint2);
        // Simple verification that we get non-zero values
        assert!(fingerprint.low != 0 || fingerprint.high != 0);
    }

    #[test]
    fn test_farm_fingerprint_128_with_seed() {
        let data = b"hello world";
        let seed = Uint128::new(123456789, 987654321);
        let fingerprint1 = farm_fingerprint_128_with_seed(data, seed);
        // Test consistency
        assert_eq!(farm_fingerprint_128_with_seed(data, seed), fingerprint1);
        // Different seeds should produce different fingerprints
        let seed2 = Uint128::new(123456789 + 1, 987654321);
        let fingerprint2 = farm_fingerprint_128_with_seed(data, seed2);
        assert_ne!(fingerprint1, fingerprint2);
    }

    //------------------------------------------------------------------------------
    // Compatibility tests
    //------------------------------------------------------------------------------

    #[test]
    fn test_farmhash_module_compatibility() {
        let data = b"hello world";
        // Verify that the farmhash module functions match the top-level functions
        assert_eq!(farmhash::hash32(data), farm_hash_32(data));
        assert_eq!(farmhash::hash64(data), farm_hash_64(data));
        assert_eq!(farmhash::fingerprint32(data), farm_fingerprint_32(data));
        assert_eq!(farmhash::fingerprint64(data), farm_fingerprint_64(data));
        assert_eq!(farmhash::fingerprint128(data), farm_fingerprint_128(data));
    }

    #[test]
    fn test_uint128_conversion() {
        let low = 0x1234567890ABCDEF;
        let high = 0xFEDCBA0987654321;

        let u128 = Uint128::new(low, high);
        let tuple: (u64, u64) = u128.into();

        assert_eq!(tuple.0, low);
        assert_eq!(tuple.1, high);

        let u128_from_tuple = Uint128::from(tuple);
        assert_eq!(u128_from_tuple.low, low);
        assert_eq!(u128_from_tuple.high, high);
    }

    //------------------------------------------------------------------------------
    // Hasher tests
    //------------------------------------------------------------------------------

    #[test]
    fn test_farm_hash_hasher() {
        let data = b"hello world";
        let mut hasher = FarmHashHasher::new();
        hasher.write(data);
        let hash = hasher.finish();
        // Test consistency with direct call
        assert_eq!(hash, farm_hash_64(data));
    }

    #[test]
    fn test_farm_hash_hasher_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let mut hasher = FarmHashHasher::with_seed(seed);
        hasher.write(data);
        let hash = hasher.finish();
        // Test consistency with direct call
        assert_eq!(hash, farm_hash_64_with_seed(data, seed));
    }
}
