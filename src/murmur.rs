use std::hash::Hasher;

// Constants for MurmurHash3 32-bit
const C1_32: u32 = 0xcc9e2d51;
const C2_32: u32 = 0x1b873593;

// Constants for MurmurHash3 128-bit
const C1_128: u32 = 0x239b961b;
const C2_128: u32 = 0xab0e9789;
const C3_128: u32 = 0x38b34ae5;
const C4_128: u32 = 0xa1e38b93;

// Finalization constants
const FMIX_32_1: u32 = 0x85ebca6b;
const FMIX_32_2: u32 = 0xc2b2ae35;

// MurmurHash3 32-bit hasher
#[derive(Debug, Copy, Clone)]
pub struct MurmurHasher32 {
    state: u32,
    length: usize,
}

impl MurmurHasher32 {
    #[inline(always)]
    pub fn new(seed: u32) -> Self {
        Self {
            state: seed,
            length: 0,
        }
    }

    #[inline(always)]
    pub fn finish_u32(&self) -> u32 {
        let mut h1 = self.state;

        // Finalization
        h1 ^= self.length as u32;
        h1 = fmix32(h1);

        h1
    }
}

// Helper function for 32-bit finalization mix
#[inline(always)]
fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(FMIX_32_1);
    h ^= h >> 13;
    h = h.wrapping_mul(FMIX_32_2);
    h ^= h >> 16;
    h
}

impl Default for MurmurHasher32 {
    #[inline(always)]
    fn default() -> Self {
        Self::new(0)
    }
}

impl Hasher for MurmurHasher32 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.finish_u32() as u64
    }

    #[inline(always)]
    fn write(&mut self, data: &[u8]) {
        let len = data.len();
        self.length += len;

        // Local state for better optimization
        let mut h1 = self.state;

        // Process 4-byte blocks
        let nblocks = len / 4;
        let blocks_end = nblocks * 4;

        for i in (0..blocks_end).step_by(4) {
            // Use endian-agnostic byte loading (same as original algorithm)
            let k1 = (data[i] as u32)
                | ((data[i + 1] as u32) << 8)
                | ((data[i + 2] as u32) << 16)
                | ((data[i + 3] as u32) << 24);

            let mut k = k1.wrapping_mul(C1_32);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2_32);

            h1 ^= k;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
        }

        // Process tail (remaining bytes)
        let mut k1: u32 = 0;
        let tail = &data[blocks_end..];

        match tail.len() {
            3 => {
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                h1 ^= k1;
            }
            2 => {
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                h1 ^= k1;
            }
            1 => {
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                h1 ^= k1;
            }
            _ => {}
        }

        // Save state
        self.state = h1;
    }
}

// MurmurHash3 128-bit hasher
#[derive(Debug, Copy, Clone)]
pub struct MurmurHasher128 {
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
    length: usize,
}

impl MurmurHasher128 {
    #[inline(always)]
    pub fn new(seed: u32) -> Self {
        Self {
            h1: seed,
            h2: seed,
            h3: seed,
            h4: seed,
            length: 0,
        }
    }

    #[inline(always)]
    pub fn finish_u128(&self) -> u128 {
        let mut h1 = self.h1;
        let mut h2 = self.h2;
        let mut h3 = self.h3;
        let mut h4 = self.h4;

        // Finalization
        h1 ^= self.length as u32;
        h2 ^= self.length as u32;
        h3 ^= self.length as u32;
        h4 ^= self.length as u32;

        // Mix the state values together
        h1 = h1.wrapping_add(h2).wrapping_add(h3).wrapping_add(h4);
        h2 = h2.wrapping_add(h1);
        h3 = h3.wrapping_add(h1);
        h4 = h4.wrapping_add(h1);

        // Apply the finalization mix to each part
        h1 = fmix32(h1);
        h2 = fmix32(h2);
        h3 = fmix32(h3);
        h4 = fmix32(h4);

        // Combine the four 32-bit values into one 128-bit value
        ((h4 as u128) << 96) | ((h3 as u128) << 64) | ((h2 as u128) << 32) | (h1 as u128)
    }

    #[inline(always)]
    pub fn write(&mut self, data: &[u8]) {
        let len = data.len();
        self.length += len;

        // Local state for better optimization
        let mut h1 = self.h1;
        let mut h2 = self.h2;
        let mut h3 = self.h3;
        let mut h4 = self.h4;

        // Process 16-byte blocks
        let nblocks = len / 16;
        let blocks_end = nblocks * 16;

        for i in (0..blocks_end).step_by(16) {
            // Use endian-agnostic byte loading (same as original algorithm)
            let k1 = (data[i] as u32)
                | ((data[i + 1] as u32) << 8)
                | ((data[i + 2] as u32) << 16)
                | ((data[i + 3] as u32) << 24);

            let k2 = (data[i + 4] as u32)
                | ((data[i + 5] as u32) << 8)
                | ((data[i + 6] as u32) << 16)
                | ((data[i + 7] as u32) << 24);

            let k3 = (data[i + 8] as u32)
                | ((data[i + 9] as u32) << 8)
                | ((data[i + 10] as u32) << 16)
                | ((data[i + 11] as u32) << 24);

            let k4 = (data[i + 12] as u32)
                | ((data[i + 13] as u32) << 8)
                | ((data[i + 14] as u32) << 16)
                | ((data[i + 15] as u32) << 24);

            // Process k1
            let mut k = k1.wrapping_mul(C1_128);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2_128);
            h1 ^= k;
            h1 = h1.rotate_left(19);
            h1 = h1.wrapping_add(h2);
            h1 = h1.wrapping_mul(5).wrapping_add(0x561ccd1b);

            // Process k2
            let mut k = k2.wrapping_mul(C2_128);
            k = k.rotate_left(16);
            k = k.wrapping_mul(C3_128);
            h2 ^= k;
            h2 = h2.rotate_left(17);
            h2 = h2.wrapping_add(h3);
            h2 = h2.wrapping_mul(5).wrapping_add(0x0bcaa747);

            // Process k3
            let mut k = k3.wrapping_mul(C3_128);
            k = k.rotate_left(17);
            k = k.wrapping_mul(C4_128);
            h3 ^= k;
            h3 = h3.rotate_left(15);
            h3 = h3.wrapping_add(h4);
            h3 = h3.wrapping_mul(5).wrapping_add(0x96cd1c35);

            // Process k4
            let mut k = k4.wrapping_mul(C4_128);
            k = k.rotate_left(18);
            k = k.wrapping_mul(C1_128);
            h4 ^= k;
            h4 = h4.rotate_left(13);
            h4 = h4.wrapping_add(h1);
            h4 = h4.wrapping_mul(5).wrapping_add(0x32ac3b17);
        }

        // Process tail bytes
        let tail = &data[blocks_end..];
        self.process_tail(tail, &mut h1, &mut h2, &mut h3, &mut h4);

        // Save state
        self.h1 = h1;
        self.h2 = h2;
        self.h3 = h3;
        self.h4 = h4;
    }

    // Helper function to process tail bytes
    #[inline(always)]
    fn process_tail(&self, tail: &[u8], h1: &mut u32, h2: &mut u32, h3: &mut u32, h4: &mut u32) {
        // Process the remaining bytes that didn't fit in a complete block
        if tail.is_empty() {
            return;
        }

        // Use a more efficient approach for tail processing
        match tail.len() {
            1..=4 => {
                let mut k1: u32 = 0;

                if !tail.is_empty() {
                    k1 ^= tail[0] as u32;
                }
                if tail.len() >= 2 {
                    k1 ^= (tail[1] as u32) << 8;
                }
                if tail.len() >= 3 {
                    k1 ^= (tail[2] as u32) << 16;
                }
                if tail.len() >= 4 {
                    k1 ^= (tail[3] as u32) << 24;
                }

                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                *h1 ^= k1;
            }
            5..=8 => {
                let mut k1: u32 = 0;
                let mut k2: u32 = 0;

                // Process bytes for k1
                if !tail.is_empty() {
                    k1 ^= tail[0] as u32;
                }
                if tail.len() >= 2 {
                    k1 ^= (tail[1] as u32) << 8;
                }
                if tail.len() >= 3 {
                    k1 ^= (tail[2] as u32) << 16;
                }
                if tail.len() >= 4 {
                    k1 ^= (tail[3] as u32) << 24;
                }

                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                *h1 ^= k1;

                // Process bytes for k2
                if tail.len() >= 5 {
                    k2 ^= tail[4] as u32;
                }
                if tail.len() >= 6 {
                    k2 ^= (tail[5] as u32) << 8;
                }
                if tail.len() >= 7 {
                    k2 ^= (tail[6] as u32) << 16;
                }
                if tail.len() >= 8 {
                    k2 ^= (tail[7] as u32) << 24;
                }

                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                *h2 ^= k2;
            }
            9..=12 => {
                let mut k1: u32 = 0;
                let mut k2: u32 = 0;
                let mut k3: u32 = 0;

                // Process bytes for k1
                if !tail.is_empty() {
                    k1 ^= tail[0] as u32;
                }
                if tail.len() >= 2 {
                    k1 ^= (tail[1] as u32) << 8;
                }
                if tail.len() >= 3 {
                    k1 ^= (tail[2] as u32) << 16;
                }
                if tail.len() >= 4 {
                    k1 ^= (tail[3] as u32) << 24;
                }

                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                *h1 ^= k1;

                // Process bytes for k2
                if tail.len() >= 5 {
                    k2 ^= tail[4] as u32;
                }
                if tail.len() >= 6 {
                    k2 ^= (tail[5] as u32) << 8;
                }
                if tail.len() >= 7 {
                    k2 ^= (tail[6] as u32) << 16;
                }
                if tail.len() >= 8 {
                    k2 ^= (tail[7] as u32) << 24;
                }

                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                *h2 ^= k2;

                // Process bytes for k3
                if tail.len() >= 9 {
                    k3 ^= tail[8] as u32;
                }
                if tail.len() >= 10 {
                    k3 ^= (tail[9] as u32) << 8;
                }
                if tail.len() >= 11 {
                    k3 ^= (tail[10] as u32) << 16;
                }
                if tail.len() >= 12 {
                    k3 ^= (tail[11] as u32) << 24;
                }

                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                *h3 ^= k3;
            }
            13..=15 => {
                let mut k1: u32 = 0;
                let mut k2: u32 = 0;
                let mut k3: u32 = 0;
                let mut k4: u32 = 0;

                // Process bytes for k1
                if !tail.is_empty() {
                    k1 ^= tail[0] as u32;
                }
                if tail.len() >= 2 {
                    k1 ^= (tail[1] as u32) << 8;
                }
                if tail.len() >= 3 {
                    k1 ^= (tail[2] as u32) << 16;
                }
                if tail.len() >= 4 {
                    k1 ^= (tail[3] as u32) << 24;
                }

                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                *h1 ^= k1;

                // Process bytes for k2
                if tail.len() >= 5 {
                    k2 ^= tail[4] as u32;
                }
                if tail.len() >= 6 {
                    k2 ^= (tail[5] as u32) << 8;
                }
                if tail.len() >= 7 {
                    k2 ^= (tail[6] as u32) << 16;
                }
                if tail.len() >= 8 {
                    k2 ^= (tail[7] as u32) << 24;
                }

                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                *h2 ^= k2;

                // Process bytes for k3
                if tail.len() >= 9 {
                    k3 ^= tail[8] as u32;
                }
                if tail.len() >= 10 {
                    k3 ^= (tail[9] as u32) << 8;
                }
                if tail.len() >= 11 {
                    k3 ^= (tail[10] as u32) << 16;
                }
                if tail.len() >= 12 {
                    k3 ^= (tail[11] as u32) << 24;
                }

                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                *h3 ^= k3;

                // Process bytes for k4
                if tail.len() >= 13 {
                    k4 ^= tail[12] as u32;
                }
                if tail.len() >= 14 {
                    k4 ^= (tail[13] as u32) << 8;
                }
                if tail.len() >= 15 {
                    k4 ^= (tail[14] as u32) << 16;
                }

                k4 = k4.wrapping_mul(C4_128);
                k4 = k4.rotate_left(18);
                k4 = k4.wrapping_mul(C1_128);
                *h4 ^= k4;
            }
            _ => unreachable!(), // tail.len() should be 0-15
        }
    }
}
