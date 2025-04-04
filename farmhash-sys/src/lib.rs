#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_char;

/// Compute FarmHash32 for a byte slice
pub fn farm_hash_32(bytes: &[u8]) -> u32 {
    unsafe { farmhash_hash32(bytes.as_ptr() as *const c_char, bytes.len()) }
}

/// Compute FarmHash32 for a byte slice with a seed
pub fn farm_hash_32_with_seed(bytes: &[u8], seed: u32) -> u32 {
    unsafe { farmhash_hash32_with_seed(bytes.as_ptr() as *const c_char, bytes.len(), seed) }
}

/// Compute FarmHash64 for a byte slice
pub fn farm_hash_64(bytes: &[u8]) -> u64 {
    unsafe { farmhash_hash64(bytes.as_ptr() as *const c_char, bytes.len()) }
}

/// Compute FarmHash64 for a byte slice with a seed
pub fn farm_hash_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    unsafe { farmhash_hash64_with_seed(bytes.as_ptr() as *const c_char, bytes.len(), seed) }
}

/// Compute FarmHash64 for a byte slice with two seeds
pub fn farm_hash_64_with_seeds(bytes: &[u8], seed0: u64, seed1: u64) -> u64 {
    unsafe {
        farmhash_hash64_with_seeds(bytes.as_ptr() as *const c_char, bytes.len(), seed0, seed1)
    }
}

/// Compute FarmHash128 (fingerprint) for a byte slice
pub fn farm_hash_128(bytes: &[u8]) -> (u64, u64) {
    unsafe {
        let result = farmhash_fingerprint128(bytes.as_ptr() as *const c_char, bytes.len());
        (result.low, result.high)
    }
}

/// Compute FarmHash128 (fingerprint) for a byte slice with a seed
pub fn farm_hash_128_with_seed(bytes: &[u8], seed: u64) -> (u64, u64) {
    // Implement using hash64_with_seed for each half
    let low = farm_hash_64_with_seed(bytes, seed);
    let high = farm_hash_64_with_seed(bytes, seed.wrapping_add(1));
    (low, high)
}

use std::hash::Hasher;

/// Hasher implementation for FarmHash
pub struct FarmHashHasher {
    buffer: Vec<u8>,
    seed: Option<u64>,
}

impl FarmHashHasher {
    /// Create a new FarmHashHasher with no seed
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            seed: None,
        }
    }

    /// Create a new FarmHashHasher with the specified seed
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
        if self.seed.is_some() {
            farm_hash_64_with_seed(&self.buffer, self.seed.unwrap())
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
/// Default hasher for FarmHash
type FarmHashHasherDefault = std::hash::BuildHasherDefault<FarmHashHasher>;

// We also provide the original module for backward compatibility or for users who prefer that style
pub mod farmhash {
    use super::*;

    /// Hash function for a byte array, returning a 32-bit hash
    pub fn hash32(bytes: &[u8]) -> u32 {
        farm_hash_32(bytes)
    }

    /// Hash function for a byte array with a 32-bit seed, returning a 32-bit hash
    pub fn hash32_with_seed(bytes: &[u8], seed: u32) -> u32 {
        farm_hash_32_with_seed(bytes, seed)
    }

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

    /// Fingerprint function for a byte array, returning a 128-bit fingerprint
    pub fn fingerprint128(bytes: &[u8]) -> (u64, u64) {
        farm_hash_128(bytes)
    }

    /// Fingerprint function for a byte array with a seed, returning a 128-bit fingerprint
    pub fn fingerprint128_with_seed(bytes: &[u8], seed: u64) -> (u64, u64) {
        farm_hash_128_with_seed(bytes, seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_farm_hash_128() {
        let data = b"hello world";
        let (low, high) = farm_hash_128(data);
        // Test consistency
        let (low2, high2) = farm_hash_128(data);
        assert_eq!((low, high), (low2, high2));
        // Simple verification that we get non-zero values
        assert!(low != 0 || high != 0);
    }

    #[test]
    fn test_farm_hash_128_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let (low1, high1) = farm_hash_128_with_seed(data, seed);
        // Test consistency
        assert_eq!(farm_hash_128_with_seed(data, seed), (low1, high1));
        // Different seeds should produce different fingerprints
        let (low2, high2) = farm_hash_128_with_seed(data, seed + 1);
        assert!(low1 != low2 || high1 != high2);
    }

    #[test]
    fn test_farmhash_module_compatibility() {
        let data = b"hello world";
        // Verify that the farmhash module functions match the top-level functions
        assert_eq!(farmhash::hash32(data), farm_hash_32(data));
        assert_eq!(farmhash::hash64(data), farm_hash_64(data));
        assert_eq!(farmhash::fingerprint128(data), farm_hash_128(data));
    }

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
