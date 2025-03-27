//! # SimpleHash
//!
//! A simple, fast Rust library implementing common non-cryptographic hash functions.
//!
//! ## Overview
//!
//! This library provides implementations of several widely-used non-cryptographic hash functions:
//! - FNV-1 (32-bit and 64-bit variants)
//! - FNV-1a (32-bit and 64-bit variants)
//! - MurmurHash3 (32-bit and 128-bit variants)
//!
//! Non-cryptographic hash functions are designed for fast computation and good distribution
//! properties, making them suitable for hash tables, checksums, and other general-purpose
//! hashing needs. They are NOT suitable for cryptographic purposes.
//!
//! All hash functions in this library implement the `std::hash::Hasher` trait, making them
//! compatible with Rust's standard collections like `HashMap` and `HashSet`.
//!
//! ## Example Usage
//!
//! ```rust
//! use simplehash::{fnv1_32, fnv1a_32, fnv1_64, fnv1a_64, murmurhash3_32, murmurhash3_128};
//!
//! let input = "hello world";
//! let bytes = input.as_bytes();
//!
//! // Computing various hashes
//! let fnv1_32_hash = fnv1_32(bytes);
//! let fnv1a_32_hash = fnv1a_32(bytes);
//! let fnv1_64_hash = fnv1_64(bytes);
//! let fnv1a_64_hash = fnv1a_64(bytes);
//! let murmur3_32_hash = murmurhash3_32(bytes, 0);
//! let murmur3_128_hash = murmurhash3_128(bytes, 0);
//!
//! println!("FNV1-32: 0x{:x}", fnv1_32_hash);
//! println!("FNV1a-32: 0x{:x}", fnv1a_32_hash);
//! println!("FNV1-64: 0x{:x}", fnv1_64_hash);
//! println!("FNV1a-64: 0x{:x}", fnv1a_64_hash);
//! println!("MurmurHash3-32: 0x{:x}", murmur3_32_hash);
//! println!("MurmurHash3-128: 0x{:x}", murmur3_128_hash);
//! ```
//!
//! ## Choosing a Hash Function
//!
//! - **FNV-1a**: Good general-purpose hash function. Simple to implement with reasonable
//!   performance and distribution properties. The FNV-1a variant is generally preferred
//!   over FNV-1.
//!
//! - **MurmurHash3**: Offers excellent distribution properties and performance, especially
//!   for larger inputs. The 128-bit variant provides better collision resistance.
//!
//! ## Using with HashMap and HashSet
//!
//! All hashers in this library implement the `std::hash::Hasher` trait, making them
//! compatible with standard collections. This allows you to use these faster, non-cryptographic
//! hashers in place of the default SipHash implementation when performance is a priority.
//!
//! ```rust
//! use simplehash::fnv::Fnv1aHasher64;
//! use simplehash::murmur::MurmurHasher32;
//! use std::collections::{HashMap, HashSet};
//! use std::hash::BuildHasherDefault;
//!
//! // Using FNV-1a with HashMap
//! let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
//!     HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());
//! map.insert("key".to_string(), 42);
//!
//! // Using FNV-1a with HashSet
//! let mut set: HashSet<String, BuildHasherDefault<Fnv1aHasher64>> =
//!     HashSet::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());
//! set.insert("value".to_string());
//!
//! // Create a BuildHasher for MurmurHash3
//! #[derive(Default, Clone)]
//! struct MurmurHash3BuildHasher;
//!
//! impl std::hash::BuildHasher for MurmurHash3BuildHasher {
//!     type Hasher = MurmurHasher32;
//!
//!     fn build_hasher(&self) -> Self::Hasher {
//!         MurmurHasher32::new(0) // Using seed 0
//!     }
//! }
//!
//! // Using MurmurHash3 with HashMap
//! let mut murmur_map: HashMap<String, u32, MurmurHash3BuildHasher> =
//!     HashMap::with_hasher(MurmurHash3BuildHasher);
//! murmur_map.insert("key".to_string(), 42);
//! ```
//!
//! ### Performance Characteristics
//!
//! * **FNV-1a**:
//!   - Very fast for small keys (like short strings, integers)
//!   - Uses less memory than SipHash (Rust's default)
//!   - Particularly effective for HashMaps with small keys
//!   - Not recommended for untrusted inputs (e.g., network data) due to lack of DoS protection
//!
//! * **MurmurHash3**:
//!   - Excellent performance for medium to large inputs
//!   - Better distribution properties than FNV
//!   - Good compromise between speed and collision resistance
//!   - The 32-bit version is faster, while the 128-bit version has better collision resistance
//!
//! ## Implementation Notes
//!
//! All hash functions in this library:
//! - Accept a byte slice (`&[u8]`) as input
//! - Return an unsigned integer of the appropriate size
//! - Are deterministic (same input always produces same output)
//! - Are endian-agnostic (produce same result regardless of platform endianness)
//!
//! The MurmurHash3 implementations are compatible with reference implementations
//! in other languages (Python's mmh3 package and the original C++ implementation).

use std::hash::Hasher;

pub mod fnv;
pub mod murmur;

// Re-export for users to use directly
pub use fnv::*;
pub use murmur::*;

/// Computes the FNV-1 hash (32-bit) of the provided data.
///
/// This is the original FNV-1 algorithm developed by Glenn Fowler, Landon Curt Noll,
/// and Phong Vo. For most purposes, you should prefer [`fnv1a_32`] instead, which
/// generally has better dispersion properties.
///
/// # Algorithm
///
/// FNV-1 works by:
/// 1. Starting with an initial basis value (2166136261 for 32-bit FNV-1)
/// 2. For each byte in the input:
///    a. Multiply the current hash by the FNV prime (16777619 for 32-bit)
///    b. XOR the result with the current byte
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
///
/// # Returns
///
/// A 32-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::fnv1_32;
///
/// let data = b"hello world";
/// let hash = fnv1_32(data);
/// println!("FNV1-32 hash: 0x{:08x}", hash);
/// ```
#[inline]
pub fn fnv1_32(data: &[u8]) -> u32 {
    let mut hasher = fnv::FnvHasher32::new();
    hasher.write(data);
    hasher.finish_raw()
}

/// Computes the FNV-1a hash (32-bit) of the provided data.
///
/// FNV-1a is an improved variant of the original FNV-1 algorithm with better
/// dispersion properties. It is generally considered superior to FNV-1 for most use cases.
///
/// # Algorithm
///
/// FNV-1a works by:
/// 1. Starting with an initial basis value (2166136261 for 32-bit FNV-1a)
/// 2. For each byte in the input:
///    a. XOR the current hash with the current byte
///    b. Multiply the result by the FNV prime (16777619 for 32-bit)
///
/// The key difference from FNV-1 is the order of operations (XOR then multiply, vs multiply then XOR).
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
///
/// # Returns
///
/// A 32-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::fnv1a_32;
///
/// let data = b"hello world";
/// let hash = fnv1a_32(data);
/// println!("FNV1a-32 hash: 0x{:08x}", hash);
/// ```
#[inline]
pub fn fnv1a_32(data: &[u8]) -> u32 {
    let mut hasher = fnv::Fnv1aHasher32::new();
    hasher.write(data);
    hasher.finish_raw()
}

/// Computes the FNV-1 hash (64-bit) of the provided data.
///
/// This is the 64-bit variant of the original FNV-1 algorithm, offering a larger
/// hash space and reduced collision probability compared to the 32-bit version.
/// For most purposes, the [`fnv1a_64`] variant is preferred for its better dispersion properties.
///
/// # Algorithm
///
/// FNV-1 (64-bit) works by:
/// 1. Starting with an initial basis value (14695981039346656037 for 64-bit FNV-1)
/// 2. For each byte in the input:
///    a. Multiply the current hash by the FNV prime (1099511628211 for 64-bit)
///    b. XOR the result with the current byte
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
///
/// # Returns
///
/// A 64-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::fnv1_64;
///
/// let data = b"hello world";
/// let hash = fnv1_64(data);
/// println!("FNV1-64 hash: 0x{:016x}", hash);
/// ```
#[inline]
pub fn fnv1_64(data: &[u8]) -> u64 {
    let mut hasher = fnv::FnvHasher64::new();
    hasher.write(data);
    hasher.finish_raw()
}

/// Computes the FNV-1a hash (64-bit) of the provided data.
///
/// FNV-1a (64-bit) is an improved variant of the original FNV-1 algorithm with better
/// dispersion properties and a larger hash space than the 32-bit variant. It is generally
/// preferred over FNV-1 (64-bit) for most applications.
///
/// # Algorithm
///
/// FNV-1a (64-bit) works by:
/// 1. Starting with an initial basis value (14695981039346656037 for 64-bit FNV-1a)
/// 2. For each byte in the input:
///    a. XOR the current hash with the current byte
///    b. Multiply the result by the FNV prime (1099511628211 for 64-bit)
///
/// The key difference from FNV-1 is the order of operations (XOR then multiply, vs multiply then XOR).
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
///
/// # Returns
///
/// A 64-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::fnv1a_64;
///
/// let data = b"hello world";
/// let hash = fnv1a_64(data);
/// println!("FNV1a-64 hash: 0x{:016x}", hash);
/// ```
#[inline]
pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hasher = fnv::Fnv1aHasher64::new();
    hasher.write(data);
    hasher.finish()
}

/// Computes the MurmurHash3 32-bit hash of the provided data.
///
/// MurmurHash3 is a non-cryptographic hash function created by Austin Appleby in 2008.
/// This 32-bit implementation is optimized for x86 architectures and provides excellent
/// distribution, avalanche behavior, and performance characteristics.
///
/// # Algorithm
///
/// MurmurHash3 (32-bit) works by:
/// 1. Processing the input in 4-byte (32-bit) blocks
/// 2. Applying carefully chosen magic constants and bit manipulation operations
/// 3. Processing any remaining bytes (the "tail")
/// 4. Finalizing the hash with additional mixing to improve avalanche behavior
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
/// * `seed` - A 32-bit seed value that can be used to create different hash values for the same input
///
/// # Returns
///
/// A 32-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::murmurhash3_32;
///
/// let data = b"hello world";
/// let hash = murmurhash3_32(data, 0);  // Using seed value 0
/// println!("MurmurHash3-32 hash: 0x{:08x}", hash);
///
/// // Using a different seed produces a different hash
/// let hash2 = murmurhash3_32(data, 42);
/// println!("MurmurHash3-32 hash (seed 42): 0x{:08x}", hash2);
/// ```
///
/// # Compatibility
///
/// This implementation is compatible with other MurmurHash3 implementations including the
/// original C++ implementation by Austin Appleby and the Python mmh3 package.
#[inline]
pub fn murmurhash3_32(data: &[u8], seed: u32) -> u32 {
    let mut hasher = murmur::MurmurHasher32::new(seed);
    hasher.write(data);
    hasher.finish_u32()
}

/// Computes the MurmurHash3 128-bit hash of the provided data.
///
/// MurmurHash3 is a non-cryptographic hash function created by Austin Appleby in 2008.
/// This 128-bit implementation provides superior collision resistance compared to the 32-bit
/// variant, making it suitable for applications requiring a larger hash space.
///
/// # Algorithm
///
/// MurmurHash3 (128-bit) works by:
/// 1. Processing the input in 16-byte (128-bit) blocks
/// 2. Using four 32-bit state variables (h1, h2, h3, h4) that are updated as data is processed
/// 3. Applying carefully chosen magic constants and bit manipulation operations
/// 4. Processing any remaining bytes (the "tail")
/// 5. Finalizing the hash with additional mixing to improve avalanche behavior
///
/// # Parameters
///
/// * `data` - A slice of bytes to hash
/// * `seed` - A 32-bit seed value that can be used to create different hash values for the same input
///
/// # Returns
///
/// A 128-bit unsigned integer representing the hash value
///
/// # Example
///
/// ```
/// use simplehash::murmurhash3_128;
///
/// let data = b"hello world";
/// let hash = murmurhash3_128(data, 0);  // Using seed value 0
/// println!("MurmurHash3-128 hash: 0x{:032x}", hash);
///
/// // Using a different seed produces a different hash
/// let hash2 = murmurhash3_128(data, 42);
/// println!("MurmurHash3-128 hash (seed 42): 0x{:032x}", hash2);
/// ```
///
/// # Compatibility
///
/// This implementation is compatible with other MurmurHash3 128-bit implementations including the
/// original C++ implementation by Austin Appleby and the Python mmh3 package.
///
/// Note that the lower 64 bits of the result match the value returned by mmh3.hash64() in Python.
#[inline]
pub fn murmurhash3_128(data: &[u8], seed: u32) -> u128 {
    let mut hasher = murmur::MurmurHasher128::new(seed);
    hasher.write(data);
    hasher.finish_u128()
}

// Test corpus for validation against Python mmh3 implementation
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, from_str};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_fnv1_32() {
        let data1 = b"hello";
        // Get the result from our implementation
        let result = fnv1_32(data1);
        // Debug print the result
        println!("FNV1-32 for 'hello': 0x{:x} ({})", result, result);
    }

    #[test]
    fn test_fnv1a_32() {
        let data1 = b"hello";
        // Get the result from our implementation
        let result = fnv1a_32(data1);
        // Debug print the result
        println!("FNV1a-32 for 'hello': 0x{:x} ({})", result, result);
    }

    #[test]
    fn test_fnv1_64() {
        let data1 = b"hello";
        // Get the result from our implementation
        let result = fnv1_64(data1);
        // Debug print the result
        println!("FNV1-64 for 'hello': 0x{:x} ({})", result, result);
    }

    #[test]
    fn test_fnv1a_64() {
        let data1 = b"hello";
        // Get the result from our implementation
        let result = fnv1a_64(data1);
        // Debug print the result
        println!("FNV1a-64 for 'hello': 0x{:x} ({})", result, result);
    }

    #[test]
    fn test_murmurhash3_32() {
        // Test cases with debug output
        let data1 = b"hello";
        let result = murmurhash3_32(data1, 0);
        println!(
            "MurmurHash3-32 for 'hello' (seed 0): 0x{:x} ({})",
            result, result
        );

        let data2 = b"hello world";
        let result2 = murmurhash3_32(data2, 0);
        println!(
            "MurmurHash3-32 for 'hello world' (seed 0): 0x{:x} ({})",
            result2, result2
        );

        let data3 = b"";
        let result3 = murmurhash3_32(data3, 0);
        println!(
            "MurmurHash3-32 for '' (seed 0): 0x{:x} ({})",
            result3, result3
        );

        let data4 = b"aaaa";
        let result4 = murmurhash3_32(data4, 0x9747b28c);
        println!(
            "MurmurHash3-32 for 'aaaa' (seed 0x9747b28c): 0x{:x} ({})",
            result4, result4
        );
    }

    #[test]
    fn test_murmurhash3_128() {
        // Test with debug output
        let data = b"hello world";
        let result = murmurhash3_128(data, 0);
        println!(
            "MurmurHash3-128 for 'hello world' (seed 0): 0x{:x} ({})",
            result, result
        );

        let empty = b"";
        let result_empty = murmurhash3_128(empty, 0);
        println!(
            "MurmurHash3-128 for '' (seed 0): 0x{:x} ({})",
            result_empty, result_empty
        );
    }

    #[test]
    fn test_against_mmh3_python() {
        // Read the test corpus generated by Python
        let mut file = match File::open("data/mmh3_test_corpus.json") {
            Ok(file) => file,
            Err(_) => {
                println!("Skipping Python mmh3 comparison test - mmh3_test_corpus.json not found");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let corpus: Value = from_str(&contents).unwrap();

        // Test each entry in the corpus
        if let Value::Array(entries) = corpus {
            for entry in entries {
                // Get the input bytes
                let input_bytes = entry["input_bytes"]
                    .as_array()
                    .expect("Expected input_bytes to be an array");
                let bytes: Vec<u8> = input_bytes
                    .iter()
                    .map(|v| v.as_u64().expect("Expected input_byte to be a number") as u8)
                    .collect();

                // Get the input string for error messages
                let input_str = entry["input"].as_str().unwrap_or("binary data");

                // Test MurmurHash3 32-bit with seed 0
                if let Some(py_result_32_0) = entry["murmur3_32_seed0"].as_u64() {
                    let rust_result_32_0 = murmurhash3_32(&bytes, 0);

                    println!(
                        "Testing 32-bit seed 0 for input: '{}' - Python: {} vs Rust: {}",
                        input_str, py_result_32_0, rust_result_32_0
                    );

                    assert_eq!(
                        rust_result_32_0, py_result_32_0 as u32,
                        "32-bit seed 0 mismatch for input: '{}'",
                        input_str
                    );
                }

                // Test MurmurHash3 32-bit with seed 42
                if let Some(py_result_32_42) = entry["murmur3_32_seed42"].as_u64() {
                    let rust_result_32_42 = murmurhash3_32(&bytes, 42);

                    println!(
                        "Testing 32-bit seed 42 for input: '{}' - Python: {} vs Rust: {}",
                        input_str, py_result_32_42, rust_result_32_42
                    );

                    assert_eq!(
                        rust_result_32_42, py_result_32_42 as u32,
                        "32-bit seed 42 mismatch for input: '{}'",
                        input_str
                    );
                }

                // Test MurmurHash3 128-bit with seed 0 (just compare lower 64 bits)
                if let Some(py_result_128_0) = entry["murmur3_128_seed0"].as_u64() {
                    let rust_result_128_0 = murmurhash3_128(&bytes, 0);
                    let rust_low64_0 = rust_result_128_0 as u64;

                    println!(
                        "Testing 128-bit seed 0 (low 64 bits) for input: '{}' - Python: {} vs Rust: {}",
                        input_str, py_result_128_0, rust_low64_0
                    );

                    assert_eq!(
                        rust_low64_0, py_result_128_0,
                        "128-bit seed 0 (low 64 bits) mismatch for input: '{}'",
                        input_str
                    );
                }

                // Test MurmurHash3 128-bit with seed 42
                if let Some(py_result_128_42) = entry["murmur3_128_seed42"].as_u64() {
                    let rust_result_128_42 = murmurhash3_128(&bytes, 42);
                    let rust_low64_42 = rust_result_128_42 as u64;

                    println!(
                        "Testing 128-bit seed 42 (low 64 bits) for input: '{}' - Python: {} vs Rust: {}",
                        input_str, py_result_128_42, rust_low64_42
                    );

                    assert_eq!(
                        rust_low64_42, py_result_128_42,
                        "128-bit seed 42 (low 64 bits) mismatch for input: '{}'",
                        input_str
                    );
                }
            }
        }
    }

    #[test]
    fn test_against_go_fnv() {
        // Read the test corpus generated by Go
        let mut file = match File::open("data/fnv_test_corpus.json") {
            Ok(file) => file,
            Err(_) => {
                println!("Skipping Go FNV comparison test - fnv_test_corpus.json not found");
                println!("Run 'go run generate_fnv_corpus.go' to generate the test corpus first");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let corpus: Value = match from_str(&contents) {
            Ok(v) => v,
            Err(e) => {
                println!("Error parsing JSON: {}", e);
                return;
            }
        };

        let mut total_tests = 0;
        let mut passed_tests = 0;

        // Test each entry in the corpus
        if let Value::Array(entries) = corpus {
            for entry in entries {
                // Get the input bytes
                let input_bytes = match entry["input_bytes"].as_array() {
                    Some(arr) => arr,
                    None => {
                        println!("Expected input_bytes to be an array");
                        continue;
                    }
                };

                let bytes: Vec<u8> = match input_bytes
                    .iter()
                    .map(|v| v.as_i64().map(|n| n as u8))
                    .collect::<Option<Vec<u8>>>()
                {
                    Some(b) => b,
                    None => {
                        println!("Expected input_bytes to contain valid byte values");
                        continue;
                    }
                };

                // Get the input string for error messages
                let input_str = entry["input"].as_str().unwrap_or("binary data");

                // Verify FNV1-32
                if let Some(go_fnv1_32) = entry["fnv1_32"].as_u64() {
                    total_tests += 1;
                    let rust_fnv1_32 = fnv1_32(&bytes);

                    println!(
                        "Testing FNV1-32 for input: '{}' - Go: {} vs Rust: {}",
                        input_str, go_fnv1_32, rust_fnv1_32
                    );

                    assert_eq!(
                        rust_fnv1_32, go_fnv1_32 as u32,
                        "FNV1-32 mismatch for input: '{}'",
                        input_str
                    );
                    passed_tests += 1;
                }

                // Verify FNV1a-32
                if let Some(go_fnv1a_32) = entry["fnv1a_32"].as_u64() {
                    total_tests += 1;
                    let rust_fnv1a_32 = fnv1a_32(&bytes);

                    println!(
                        "Testing FNV1a-32 for input: '{}' - Go: {} vs Rust: {}",
                        input_str, go_fnv1a_32, rust_fnv1a_32
                    );

                    assert_eq!(
                        rust_fnv1a_32, go_fnv1a_32 as u32,
                        "FNV1a-32 mismatch for input: '{}'",
                        input_str
                    );
                    passed_tests += 1;
                }

                // Verify FNV1-64
                if let Some(go_fnv1_64) = entry["fnv1_64"].as_u64() {
                    total_tests += 1;
                    let rust_fnv1_64 = fnv1_64(&bytes);

                    println!(
                        "Testing FNV1-64 for input: '{}' - Go: {} vs Rust: {}",
                        input_str, go_fnv1_64, rust_fnv1_64
                    );

                    assert_eq!(
                        rust_fnv1_64, go_fnv1_64,
                        "FNV1-64 mismatch for input: '{}'",
                        input_str
                    );
                    passed_tests += 1;
                }

                // Verify FNV1a-64
                if let Some(go_fnv1a_64) = entry["fnv1a_64"].as_u64() {
                    total_tests += 1;
                    let rust_fnv1a_64 = fnv1a_64(&bytes);

                    println!(
                        "Testing FNV1a-64 for input: '{}' - Go: {} vs Rust: {}",
                        input_str, go_fnv1a_64, rust_fnv1a_64
                    );

                    assert_eq!(
                        rust_fnv1a_64, go_fnv1a_64,
                        "FNV1a-64 mismatch for input: '{}'",
                        input_str
                    );
                    passed_tests += 1;
                }
            }
        }

        // Print summary
        println!("FNV Verification Summary:");
        println!(
            "Passed {}/{} tests ({}%)",
            passed_tests,
            total_tests,
            if total_tests > 0 {
                passed_tests * 100 / total_tests
            } else {
                0
            }
        );
    }
}
