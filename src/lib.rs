/// FNV-1 hash algorithm implementation (32-bit)
///
/// This is the original FNV-1 algorithm. For most purposes, you should
/// prefer FNV-1a instead, which generally has better dispersion properties.
pub fn fnv1_32(data: &[u8]) -> u32 {
    // FNV constants for 32-bit
    const FNV_PRIME: u32 = 16777619;
    const FNV_OFFSET_BASIS: u32 = 2166136261;

    let mut hash = FNV_OFFSET_BASIS;

    for &byte in data {
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= byte as u32;
    }

    hash
}

/// FNV-1a hash algorithm implementation (32-bit)
///
/// This is an improved version of the FNV-1 algorithm with better dispersion.
pub fn fnv1a_32(data: &[u8]) -> u32 {
    // FNV constants for 32-bit
    const FNV_PRIME: u32 = 16777619;
    const FNV_OFFSET_BASIS: u32 = 2166136261;

    let mut hash = FNV_OFFSET_BASIS;

    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

/// FNV-1 hash algorithm implementation (64-bit)
pub fn fnv1_64(data: &[u8]) -> u64 {
    // FNV constants for 64-bit
    const FNV_PRIME: u64 = 1099511628211;
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;

    let mut hash = FNV_OFFSET_BASIS;

    for &byte in data {
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= byte as u64;
    }

    hash
}

/// FNV-1a hash algorithm implementation (64-bit)
pub fn fnv1a_64(data: &[u8]) -> u64 {
    // FNV constants for 64-bit
    const FNV_PRIME: u64 = 1099511628211;
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;

    let mut hash = FNV_OFFSET_BASIS;

    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

/// MurmurHash3 32-bit implementation
/// Based on the reference implementation by Austin Appleby
pub fn murmurhash3_32(data: &[u8], seed: u32) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let len = data.len();
    let nblocks = len / 4;

    let mut h1 = seed;

    // Body
    for i in 0..nblocks {
        let block_index = i * 4;
        let mut k1 = (data[block_index] as u32)
            | ((data[block_index + 1] as u32) << 8)
            | ((data[block_index + 2] as u32) << 16)
            | ((data[block_index + 3] as u32) << 24);

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    // Tail
    let mut k1: u32 = 0;
    let tail = &data[nblocks * 4..];

    match tail.len() {
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        _ => {}
    }

    // Finalization
    h1 ^= len as u32;
    h1 = h1 ^ (h1 >> 16);
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 = h1 ^ (h1 >> 13);
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 = h1 ^ (h1 >> 16);

    h1
}

/// MurmurHash3 128-bit implementation
pub fn murmurhash3_128(data: &[u8], seed: u32) -> u128 {
    const C1: u32 = 0x239b961b;
    const C2: u32 = 0xab0e9789;
    const C3: u32 = 0x38b34ae5;
    const C4: u32 = 0xa1e38b93;

    let len = data.len();
    let nblocks = len / 16;

    let mut h1 = seed;
    let mut h2 = seed;
    let mut h3 = seed;
    let mut h4 = seed;

    // Body
    for i in 0..nblocks {
        let block_index = i * 16;

        let mut k1 = (data[block_index] as u32)
            | ((data[block_index + 1] as u32) << 8)
            | ((data[block_index + 2] as u32) << 16)
            | ((data[block_index + 3] as u32) << 24);

        let mut k2 = (data[block_index + 4] as u32)
            | ((data[block_index + 5] as u32) << 8)
            | ((data[block_index + 6] as u32) << 16)
            | ((data[block_index + 7] as u32) << 24);

        let mut k3 = (data[block_index + 8] as u32)
            | ((data[block_index + 9] as u32) << 8)
            | ((data[block_index + 10] as u32) << 16)
            | ((data[block_index + 11] as u32) << 24);

        let mut k4 = (data[block_index + 12] as u32)
            | ((data[block_index + 13] as u32) << 8)
            | ((data[block_index + 14] as u32) << 16)
            | ((data[block_index + 15] as u32) << 24);

        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;

        h1 = h1.rotate_left(19);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x561ccd1b);

        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(16);
        k2 = k2.wrapping_mul(C3);
        h2 ^= k2;

        h2 = h2.rotate_left(17);
        h2 = h2.wrapping_add(h3);
        h2 = h2.wrapping_mul(5).wrapping_add(0x0bcaa747);

        k3 = k3.wrapping_mul(C3);
        k3 = k3.rotate_left(17);
        k3 = k3.wrapping_mul(C4);
        h3 ^= k3;

        h3 = h3.rotate_left(15);
        h3 = h3.wrapping_add(h4);
        h3 = h3.wrapping_mul(5).wrapping_add(0x96cd1c35);

        k4 = k4.wrapping_mul(C4);
        k4 = k4.rotate_left(18);
        k4 = k4.wrapping_mul(C1);
        h4 ^= k4;

        h4 = h4.rotate_left(13);
        h4 = h4.wrapping_add(h1);
        h4 = h4.wrapping_mul(5).wrapping_add(0x32ac3b17);
    }

    // Tail
    let tail = &data[nblocks * 16..];
    let mut k1: u32 = 0;
    let mut k2: u32 = 0;
    let mut k3: u32 = 0;
    let mut k4: u32 = 0;

    match tail.len() {
        15 => {
            k4 ^= (tail[14] as u32) << 16;
            k4 ^= (tail[13] as u32) << 8;
            k4 ^= tail[12] as u32;
            k4 = k4.wrapping_mul(C4);
            k4 = k4.rotate_left(18);
            k4 = k4.wrapping_mul(C1);
            h4 ^= k4;

            k3 ^= (tail[11] as u32) << 24;
            k3 ^= (tail[10] as u32) << 16;
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        14 => {
            k4 ^= (tail[13] as u32) << 8;
            k4 ^= tail[12] as u32;
            k4 = k4.wrapping_mul(C4);
            k4 = k4.rotate_left(18);
            k4 = k4.wrapping_mul(C1);
            h4 ^= k4;

            k3 ^= (tail[11] as u32) << 24;
            k3 ^= (tail[10] as u32) << 16;
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        13 => {
            k4 ^= tail[12] as u32;
            k4 = k4.wrapping_mul(C4);
            k4 = k4.rotate_left(18);
            k4 = k4.wrapping_mul(C1);
            h4 ^= k4;

            k3 ^= (tail[11] as u32) << 24;
            k3 ^= (tail[10] as u32) << 16;
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        12 => {
            k3 ^= (tail[11] as u32) << 24;
            k3 ^= (tail[10] as u32) << 16;
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        11 => {
            k3 ^= (tail[10] as u32) << 16;
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        10 => {
            k3 ^= (tail[9] as u32) << 8;
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        9 => {
            k3 ^= tail[8] as u32;
            k3 = k3.wrapping_mul(C3);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4);
            h3 ^= k3;

            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        8 => {
            k2 ^= (tail[7] as u32) << 24;
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        7 => {
            k2 ^= (tail[6] as u32) << 16;
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        6 => {
            k2 ^= (tail[5] as u32) << 8;
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        5 => {
            k2 ^= tail[4] as u32;
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3);
            h2 ^= k2;

            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        4 => {
            k1 ^= (tail[3] as u32) << 24;
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        _ => {}
    }

    // Finalization
    h1 ^= len as u32;
    h2 ^= len as u32;
    h3 ^= len as u32;
    h4 ^= len as u32;

    h1 = h1.wrapping_add(h2);
    h1 = h1.wrapping_add(h3);
    h1 = h1.wrapping_add(h4);
    h2 = h2.wrapping_add(h1);
    h3 = h3.wrapping_add(h1);
    h4 = h4.wrapping_add(h1);

    h1 = h1 ^ (h1 >> 16);
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 = h1 ^ (h1 >> 13);
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 = h1 ^ (h1 >> 16);

    h2 = h2 ^ (h2 >> 16);
    h2 = h2.wrapping_mul(0x85ebca6b);
    h2 = h2 ^ (h2 >> 13);
    h2 = h2.wrapping_mul(0xc2b2ae35);
    h2 = h2 ^ (h2 >> 16);

    h3 = h3 ^ (h3 >> 16);
    h3 = h3.wrapping_mul(0x85ebca6b);
    h3 = h3 ^ (h3 >> 13);
    h3 = h3.wrapping_mul(0xc2b2ae35);
    h3 = h3 ^ (h3 >> 16);

    h4 = h4 ^ (h4 >> 16);
    h4 = h4.wrapping_mul(0x85ebca6b);
    h4 = h4 ^ (h4 >> 13);
    h4 = h4.wrapping_mul(0xc2b2ae35);
    h4 = h4 ^ (h4 >> 16);

    // Combine the four 32-bit values into one 128-bit value
    ((h4 as u128) << 96) | ((h3 as u128) << 64) | ((h2 as u128) << 32) | (h1 as u128)
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
