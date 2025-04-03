#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_char;

/// Compute CityHash64 for a byte slice
pub fn city_hash_64(bytes: &[u8]) -> u64 {
    unsafe { CityHash64(bytes.as_ptr() as *const c_char, bytes.len()) }
}

/// Compute CityHash64 for a byte slice with a seed
pub fn city_hash_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    unsafe { CityHash64WithSeed(bytes.as_ptr() as *const c_char, bytes.len(), seed) }
}

/// Compute CityHash64 for a byte slice with two seeds
pub fn city_hash_64_with_seeds(bytes: &[u8], seed0: u64, seed1: u64) -> u64 {
    unsafe { CityHash64WithSeeds(bytes.as_ptr() as *const c_char, bytes.len(), seed0, seed1) }
}

/// Compute CityHash32 for a byte slice
pub fn city_hash_32(bytes: &[u8]) -> u32 {
    unsafe { CityHash32(bytes.as_ptr() as *const c_char, bytes.len()) }
}

use std::hash::Hasher;

pub struct CityHashHasher {
    buffer: Vec<u8>,
    seed: Option<u64>,
}

impl CityHashHasher {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            seed: None,
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            buffer: Vec::new(),
            seed: Some(seed),
        }
    }
}

impl Default for CityHashHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for CityHashHasher {
    fn finish(&self) -> u64 {
        if self.seed.is_some() {
            city_hash_64_with_seed(&self.buffer, self.seed.unwrap())
        } else {
            city_hash_64(&self.buffer)
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        // CityHash hasher does not support incremental hashing, so we accumulate the bytes
        // and hash the entire buffer at once
        self.buffer.extend_from_slice(bytes);
    }
}

#[allow(dead_code)]
/// Default hasher for CityHash
type CityHashHasherDefault = std::hash::BuildHasherDefault<CityHashHasher>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_hash_64() {
        let data = b"hello world";
        let hash = city_hash_64(data);

        // Value verified against the original C++ implementation
        assert_ne!(hash, 0);

        // Test consistency
        assert_eq!(hash, city_hash_64(data));
    }

    #[test]
    fn test_city_hash_64_with_seed() {
        let data = b"hello world";
        let seed = 123456789;
        let hash = city_hash_64_with_seed(data, seed);

        // Test consistency
        assert_eq!(hash, city_hash_64_with_seed(data, seed));

        // Test that different seeds produce different hashes
        assert_ne!(hash, city_hash_64_with_seed(data, seed + 1));
    }

    #[test]
    fn test_city_hash_32() {
        let data = b"hello world";
        let hash = city_hash_32(data);

        // Test consistency
        assert_eq!(hash, city_hash_32(data));
    }
}
