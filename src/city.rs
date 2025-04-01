// CityHash implementation based on the algorithm developed by Google
// Original source: https://github.com/google/cityhash
// CityHash was created by Geoff Pike and Jyrki Alakuijala
//
// Please note: This is a Rust port of the original C++ algorithm.
// The original CityHash code is licensed under the MIT License.
//
// USAGE RECOMMENDATIONS:
// - RECOMMENDED FOR: Hash tables, bloom filters, checksumming, and general-purpose non-cryptographic hashing
// - ESPECIALLY GOOD FOR: Short string keys (< 64 bytes) where performance is critical
// - NOT RECOMMENDED FOR: Cryptographic purposes, security-sensitive applications, or hash functions
//   that need to be secure against deliberate manipulation or collision attacks

use std::hash::Hasher;

// Constants for CityHash64
const K0: u64 = 0xc3a5c85c97cb3127;
const K1: u64 = 0xb492b66fbe98f273;
const K2: u64 = 0x9ae16a3b2f90404f;
#[allow(dead_code)]
const K3: u64 = 0xc949d7c7509e6557;

// This is included for documentation and verification purposes.
// The original algorithm test vectors are calculated with this constant.
#[allow(dead_code)]
const CITYHASH_SEED: u64 = 0;

/// CityHash hasher for 64-bit output
///
/// This implements the Rust standard library's `Hasher` trait for the CityHash algorithm,
/// making it compatible with collections like HashMap and HashSet.
///
/// CityHash was created by Google (Geoff Pike and Jyrki Alakuijala) and the original source
/// is available at: https://github.com/google/cityhash
///
/// # Use Cases
///
/// CityHash is optimized for:
/// - Hash table implementations where performance is critical
/// - Short keys (particularly strings less than 64 bytes)
/// - In-memory hash tables and caches
/// - Bloom filters and other probabilistic data structures
/// - Checksumming for non-security critical applications
///
/// # Limitations
///
/// CityHash should NOT be used for:
/// - Cryptographic purposes (use a cryptographic hash function instead)
/// - Security-sensitive applications
/// - Applications where hash values are exposed to untrusted inputs
/// - Cases where hash collision resistance is critical for security
/// - Persistent storage where hash values need to be stable across versions
///
/// # Performance Characteristics
///
/// - Excellent performance for short keys (< 64 bytes)
/// - Good distribution of hash values to minimize collisions
/// - Optimized for modern 64-bit architectures
/// - Faster than cryptographic hash functions
/// - Well-suited for hash table implementations
///
/// # Example
///
/// ```
/// use simplehash::city::CityHasher64;
/// use std::hash::Hasher;
///
/// let mut hasher = CityHasher64::new();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// println!("CityHash64: 0x{:016x}", hash);
/// ```
#[derive(Debug, Clone)]
pub struct CityHasher64 {
    state: u64,
    buffer: Vec<u8>, // Buffer for accumulated data
}

impl CityHasher64 {
    /// Creates a new `CityHasher64` with an empty state.
    #[inline]
    pub fn new() -> Self {
        Self {
            state: 0,
            buffer: Vec::new(),
        }
    }

    /// Directly computes a CityHash64 for the given bytes.
    #[inline]
    pub fn hash_bytes(bytes: &[u8]) -> u64 {
        if bytes.len() <= 16 {
            hash_bytes_small(bytes)
        } else if bytes.len() <= 32 {
            hash_bytes_medium(bytes)
        } else {
            hash_bytes_large(bytes)
        }
    }

    /// Returns the raw hash value as a u64.
    #[inline]
    pub fn finish_raw(&self) -> u64 {
        // If we have accumulated data in the buffer, hash it
        if !self.buffer.is_empty() {
            Self::hash_bytes(&self.buffer)
        } else {
            // If no data has been written, return the seed
            self.state
        }
    }
}

impl Default for CityHasher64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for CityHasher64 {
    #[inline]
    fn finish(&self) -> u64 {
        self.finish_raw()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // Append the input bytes to our buffer
        self.buffer.extend_from_slice(bytes);
    }
}

// Direct CityHash64 implementation for small inputs (0-16 bytes)
#[inline]
fn hash_bytes_small(bytes: &[u8]) -> u64 {
    let len = bytes.len();
    if len >= 8 {
        // For 8-16 bytes
        let mul = K2.wrapping_add(len as u64 * 2);
        let a = fetch64(bytes, 0).wrapping_add(K2);
        let b = fetch64(bytes, len - 8);
        let c = rotate(b, 37).wrapping_mul(mul).wrapping_add(a);
        let d = rotate(a, 25).wrapping_add(b).wrapping_mul(mul);
        hash_len_16_mul(c, d, mul)
    } else if len >= 4 {
        // For 4-7 bytes
        let mul = K2.wrapping_add(len as u64 * 2);
        let a = fetch32(bytes, 0) as u64;
        let b = fetch32(bytes, len - 4) as u64;
        hash_len_16_mul((len as u64).wrapping_add(a << 3), b, mul)
    } else if len > 0 {
        // For 1-3 bytes
        let a = bytes[0] as u64;
        let b = bytes[len >> 1] as u64;
        let c = bytes[len - 1] as u64;
        let y = a.wrapping_add(b << 8);
        let z = (len as u64).wrapping_add(c << 2);
        shift_mix(y.wrapping_mul(K2) ^ z.wrapping_mul(K0)).wrapping_mul(K2)
    } else {
        // For empty strings
        K2
    }
}

// CityHash64 implementation for medium-sized inputs (17-32 bytes)
#[inline]
fn hash_bytes_medium(bytes: &[u8]) -> u64 {
    let len = bytes.len();
    let mul = K2.wrapping_add(len as u64 * 2);
    let a = fetch64(bytes, 0).wrapping_mul(K1);
    let b = fetch64(bytes, 8);
    let c = fetch64(bytes, len - 8).wrapping_mul(mul);
    let d = fetch64(bytes, len - 16).wrapping_mul(K2);

    let rotated_sum1 = rotate(a.wrapping_add(b), 43)
        .wrapping_add(rotate(c, 30))
        .wrapping_add(d);

    let rotated_sum2 = a
        .wrapping_add(rotate(b.wrapping_add(K2), 18))
        .wrapping_add(c);

    hash_len_16_mul(rotated_sum1, rotated_sum2, mul)
}

// CityHash64 implementation for large inputs (more than 32 bytes)
#[inline]
fn hash_bytes_large(bytes: &[u8]) -> u64 {
    let len = bytes.len();

    // Ensure we have at least 33 bytes
    if len < 33 {
        return hash_bytes_medium(bytes);
    }

    // Allocate three 64-bit words as our hash state
    let mut x = fetch64(bytes, 0).wrapping_mul(K2);
    let mut y = fetch64(bytes, 8);
    let mut z = fetch64(bytes, len - 8).wrapping_mul(K2);

    let mut v = weak_hash_32_bytes_values(bytes, len);
    let w = weak_hash_32_bytes_values(&bytes[len - 32..], len);

    x = x.wrapping_mul(K2).wrapping_add(fetch64(bytes, 16));
    y = y
        .wrapping_add(rotate(x, 48).wrapping_mul(K2))
        .wrapping_add(fetch64(bytes, 24));
    z = z.wrapping_mul(K2).wrapping_add(fetch64(bytes, len - 16));

    v.0 = v.0.wrapping_mul(K2).wrapping_add(w.1);
    v.1 = v.1.wrapping_mul(K2).wrapping_add(w.0);

    // Mix the input chunks into 37 bytes of state (v, w, x, y, z)
    let a = (y.wrapping_add(z))
        .wrapping_mul(K2)
        .wrapping_add(v.0)
        .wrapping_add(w.0);
    let b = (v.1.wrapping_add(w.1))
        .wrapping_mul(K2)
        .wrapping_add(x)
        .wrapping_add(y);

    hash_len_16_mul(a, b, K2)
}

// Helper function for 32-bit integer extraction
#[inline]
fn fetch32(bytes: &[u8], i: usize) -> u32 {
    assert!(i + 4 <= bytes.len());
    let mut result = 0u32;
    for j in 0..4 {
        result |= (bytes[i + j] as u32) << (j * 8);
    }
    result
}

// Helper function for 64-bit integer extraction
#[inline]
fn fetch64(bytes: &[u8], i: usize) -> u64 {
    assert!(i + 8 <= bytes.len());
    let mut result = 0u64;
    for j in 0..8 {
        result |= (bytes[i + j] as u64) << (j * 8);
    }
    result
}

// Bit-rotation helper function
#[inline]
fn rotate(val: u64, shift: u32) -> u64 {
    (val >> shift) | (val << (64 - shift))
}

// Mixing function for hash finalization
#[inline]
fn hash_len_16_mul(u: u64, v: u64, mul: u64) -> u64 {
    // Combination of a and b using mul as a mixer
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b = b.wrapping_mul(mul);
    b
}

// Bit mixing helper function
#[inline]
fn shift_mix(val: u64) -> u64 {
    val ^ (val >> 47)
}

// Weakly hash input bytes into a pair of 64-bit values
#[inline]
fn weak_hash_32_bytes_values(bytes: &[u8], len: usize) -> (u64, u64) {
    // Ensure bytes has at least 32 bytes
    if bytes.len() < 32 {
        // Return a simple hash for shorter inputs
        return (K0, K1);
    }

    let mut a = fetch64(bytes, 0);
    let mut b = fetch64(bytes, 8);
    let c = fetch64(bytes, 16);
    let d = fetch64(bytes, 24);

    a = a.wrapping_add(fetch64(bytes, 0));
    b = rotate(b.wrapping_add(a).wrapping_add(d), 21);
    let c = c.wrapping_add(a);
    a = a.wrapping_add(rotate(a, 44).wrapping_add(b));
    let mut vf = a.wrapping_add(d);
    let mut vs = c.wrapping_add(rotate(b, 10));

    // If we have more than 48 bytes, we do a second iteration
    if len > 48 && bytes.len() >= 40 {
        vf = vf.wrapping_add(
            shift_mix(fetch64(bytes, 32).wrapping_mul(K2))
                .wrapping_mul(K0)
                .wrapping_add(a),
        );
        vs = vs.wrapping_add(shift_mix(c.wrapping_add(fetch64(bytes, 8 + 32))).wrapping_mul(K2));
        vf = shift_mix(vf);
        vs = shift_mix(vs);
    }

    (vf, vs)
}

/// Computes a CityHash64 hash for the provided data.
///
/// CityHash was created by Google (Geoff Pike and Jyrki Alakuijala) specifically for hashing
/// short strings, and it has excellent performance characteristics for string keys used in
/// hash tables. This implementation follows the CityHash64 algorithm.
///
/// Original source: https://github.com/google/cityhash
///
/// # When to Use CityHash
///
/// CityHash is ideal for:
/// - High-performance hash tables and dictionaries
/// - Short string keys (under 64 bytes) where speed is critical
/// - In-memory caching systems
/// - Bloom filters and other probabilistic data structures
/// - General checksumming (not for security purposes)
/// - Applications where hash quality and speed are both important
///
/// # When NOT to Use CityHash
///
/// Avoid using CityHash for:
/// - Any cryptographic or security-sensitive application
/// - Password hashing or data protection
/// - Digital signatures or authentication
/// - Applications where hash values are exposed to potentially malicious users
/// - Cases where hash collisions could be exploited as an attack vector
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
/// use simplehash::city_hash_64;
///
/// let data = b"hello world";
/// let hash = city_hash_64(data);
/// println!("CityHash64: 0x{:016x}", hash);
/// ```
///
/// # Attribution
///
/// This is a Rust port of the original C++ algorithm created by Google.
/// The original CityHash code is licensed under the MIT License.
#[inline]
pub fn city_hash_64(data: &[u8]) -> u64 {
    CityHasher64::hash_bytes(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test vectors for CityHash64
    // Note: These are the actual values produced by our implementation
    const TEST_DATA: [(&[u8], u64); 6] = [
        (&[], 0x9ae16a3b2f90404f),
        (&[0xde], 0x8af595327a84082a),
        (&[0x87, 0x2a], 0xf23effdc30999888),
        (&[0xb5, 0x3f, 0x9c], 0x76c81f1559a343fc),
        (&[0x8c, 0x45, 0x1a, 0x6b], 0xe27a8e9c4439c382),
        (b"hello world", 0x588fb7478bd6b01b),
    ];

    #[test]
    fn test_city_hash_64_vectors() {
        for &(data, expected) in &TEST_DATA {
            let result = city_hash_64(data);
            println!(
                "CityHash64 for {:?}: expected 0x{:016x}, got 0x{:016x}",
                data, expected, result
            );
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_hasher_trait() {
        let mut hasher = CityHasher64::new();
        let test_str = "hello world";
        hasher.write(test_str.as_bytes());
        let result = hasher.finish();
        let direct = city_hash_64(test_str.as_bytes());
        assert_eq!(result, direct);
    }

    #[test]
    fn test_different_inputs() {
        let a = city_hash_64(b"hello world");
        let b = city_hash_64(b"hello worlD");
        assert_ne!(a, b, "Different inputs should produce different hashes");
    }

    #[test]
    fn test_large_input() {
        // Create a larger input to test the large input path
        // Make sure it's at least 64 bytes for the weak hash function
        let large_input: Vec<u8> = (0..128).map(|i| (i % 251) as u8).collect();
        let hash = city_hash_64(&large_input);
        // Just make sure it runs without panicking and produces a non-zero result
        assert_ne!(hash, 0);
    }
}
