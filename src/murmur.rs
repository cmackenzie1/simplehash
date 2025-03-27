use std::hash::Hasher;

// Constants for MurmurHash3 32-bit
const C1_32: u32 = 0xcc9e2d51;
const C2_32: u32 = 0x1b873593;

// Constants for MurmurHash3 128-bit
const C1_128: u32 = 0x239b961b;
const C2_128: u32 = 0xab0e9789;
const C3_128: u32 = 0x38b34ae5;
const C4_128: u32 = 0xa1e38b93;

// MurmurHash3 32-bit hasher
#[derive(Debug, Copy, Clone)]
pub struct MurmurHasher32 {
    state: u32,
    length: usize,
}

impl MurmurHasher32 {
    #[inline]
    pub fn new(seed: u32) -> Self {
        Self {
            state: seed,
            length: 0,
        }
    }

    #[inline]
    pub fn finish_u32(&self) -> u32 {
        let mut h1 = self.state;

        // Finalization
        h1 ^= self.length as u32;
        h1 = h1 ^ (h1 >> 16);
        h1 = h1.wrapping_mul(0x85ebca6b);
        h1 = h1 ^ (h1 >> 13);
        h1 = h1.wrapping_mul(0xc2b2ae35);
        h1 = h1 ^ (h1 >> 16);

        h1
    }
}

impl Default for MurmurHasher32 {
    #[inline]
    fn default() -> Self {
        Self::new(0)
    }
}

impl Hasher for MurmurHasher32 {
    #[inline]
    fn write(&mut self, data: &[u8]) {
        let nblocks = data.len() / 4;
        self.length += data.len();

        // Process 4-byte blocks
        for i in 0..nblocks {
            let block_index = i * 4;
            let mut k1 = (data[block_index] as u32)
                | ((data[block_index + 1] as u32) << 8)
                | ((data[block_index + 2] as u32) << 16)
                | ((data[block_index + 3] as u32) << 24);

            k1 = k1.wrapping_mul(C1_32);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2_32);

            self.state ^= k1;
            self.state = self.state.rotate_left(13);
            self.state = self.state.wrapping_mul(5).wrapping_add(0xe6546b64);
        }

        // Process remaining bytes
        let tail = &data[nblocks * 4..];
        let mut k1: u32 = 0;

        match tail.len() {
            3 => {
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                self.state ^= k1;
            }
            2 => {
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                self.state ^= k1;
            }
            1 => {
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_32);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_32);
                self.state ^= k1;
            }
            _ => {}
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.finish_u32() as u64
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
    #[inline]
    pub fn new(seed: u32) -> Self {
        Self {
            h1: seed,
            h2: seed,
            h3: seed,
            h4: seed,
            length: 0,
        }
    }
}

impl MurmurHasher128 {
    #[inline]
    pub fn write(&mut self, data: &[u8]) {
        let nblocks = data.len() / 16;
        self.length += data.len();

        // Process 16-byte blocks
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

            k1 = k1.wrapping_mul(C1_128);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2_128);
            self.h1 ^= k1;

            self.h1 = self.h1.rotate_left(19);
            self.h1 = self.h1.wrapping_add(self.h2);
            self.h1 = self.h1.wrapping_mul(5).wrapping_add(0x561ccd1b);

            k2 = k2.wrapping_mul(C2_128);
            k2 = k2.rotate_left(16);
            k2 = k2.wrapping_mul(C3_128);
            self.h2 ^= k2;

            self.h2 = self.h2.rotate_left(17);
            self.h2 = self.h2.wrapping_add(self.h3);
            self.h2 = self.h2.wrapping_mul(5).wrapping_add(0x0bcaa747);

            k3 = k3.wrapping_mul(C3_128);
            k3 = k3.rotate_left(17);
            k3 = k3.wrapping_mul(C4_128);
            self.h3 ^= k3;

            self.h3 = self.h3.rotate_left(15);
            self.h3 = self.h3.wrapping_add(self.h4);
            self.h3 = self.h3.wrapping_mul(5).wrapping_add(0x96cd1c35);

            k4 = k4.wrapping_mul(C4_128);
            k4 = k4.rotate_left(18);
            k4 = k4.wrapping_mul(C1_128);
            self.h4 ^= k4;

            self.h4 = self.h4.rotate_left(13);
            self.h4 = self.h4.wrapping_add(self.h1);
            self.h4 = self.h4.wrapping_mul(5).wrapping_add(0x32ac3b17);
        }

        // Process remaining bytes
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
                k4 = k4.wrapping_mul(C4_128);
                k4 = k4.rotate_left(18);
                k4 = k4.wrapping_mul(C1_128);
                self.h4 ^= k4;

                k3 ^= (tail[11] as u32) << 24;
                k3 ^= (tail[10] as u32) << 16;
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            14 => {
                k4 ^= (tail[13] as u32) << 8;
                k4 ^= tail[12] as u32;
                k4 = k4.wrapping_mul(C4_128);
                k4 = k4.rotate_left(18);
                k4 = k4.wrapping_mul(C1_128);
                self.h4 ^= k4;

                k3 ^= (tail[11] as u32) << 24;
                k3 ^= (tail[10] as u32) << 16;
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            13 => {
                k4 ^= tail[12] as u32;
                k4 = k4.wrapping_mul(C4_128);
                k4 = k4.rotate_left(18);
                k4 = k4.wrapping_mul(C1_128);
                self.h4 ^= k4;

                k3 ^= (tail[11] as u32) << 24;
                k3 ^= (tail[10] as u32) << 16;
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            12 => {
                k3 ^= (tail[11] as u32) << 24;
                k3 ^= (tail[10] as u32) << 16;
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            11 => {
                k3 ^= (tail[10] as u32) << 16;
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            10 => {
                k3 ^= (tail[9] as u32) << 8;
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            9 => {
                k3 ^= tail[8] as u32;
                k3 = k3.wrapping_mul(C3_128);
                k3 = k3.rotate_left(17);
                k3 = k3.wrapping_mul(C4_128);
                self.h3 ^= k3;

                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            8 => {
                k2 ^= (tail[7] as u32) << 24;
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            7 => {
                k2 ^= (tail[6] as u32) << 16;
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            6 => {
                k2 ^= (tail[5] as u32) << 8;
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            5 => {
                k2 ^= tail[4] as u32;
                k2 = k2.wrapping_mul(C2_128);
                k2 = k2.rotate_left(16);
                k2 = k2.wrapping_mul(C3_128);
                self.h2 ^= k2;

                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            4 => {
                k1 ^= (tail[3] as u32) << 24;
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            3 => {
                k1 ^= (tail[2] as u32) << 16;
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            2 => {
                k1 ^= (tail[1] as u32) << 8;
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            1 => {
                k1 ^= tail[0] as u32;
                k1 = k1.wrapping_mul(C1_128);
                k1 = k1.rotate_left(15);
                k1 = k1.wrapping_mul(C2_128);
                self.h1 ^= k1;
            }
            _ => {}
        }
    }

    #[inline]
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
}
