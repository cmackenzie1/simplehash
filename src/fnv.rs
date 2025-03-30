use std::hash::Hasher as StdHasher;

const FNV_32_OFFSET: u32 = 0x811c9dc5;
const FNV_32_PRIME: u32 = 0x01000193;
const FNV_64_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_64_PRIME: u64 = 0x00000100000001b3;

// FNV-1 32-bit hasher implementation
#[derive(Debug, Copy, Clone)]
pub struct FnvHasher32 {
    state: u32,
}

impl FnvHasher32 {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: FNV_32_OFFSET,
        }
    }

    #[inline(always)]
    pub fn finish_raw(&self) -> u32 {
        self.state
    }
}

impl Default for FnvHasher32 {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl StdHasher for FnvHasher32 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state as u64
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // FNV-1 implementation (multiply-xor)
        let mut state = self.state;
        for &b in bytes {
            state = state.wrapping_mul(FNV_32_PRIME);
            state ^= b as u32;
        }
        self.state = state;
    }
}

// FNV-1 64-bit hasher implementation
#[derive(Debug, Copy, Clone)]
pub struct FnvHasher64 {
    state: u64,
}

impl FnvHasher64 {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: FNV_64_OFFSET,
        }
    }

    #[inline(always)]
    pub fn finish_raw(&self) -> u64 {
        self.state
    }
}

impl Default for FnvHasher64 {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl StdHasher for FnvHasher64 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // FNV-1 implementation (multiply-xor)
        let mut state = self.state;
        for &b in bytes {
            state = state.wrapping_mul(FNV_64_PRIME);
            state ^= b as u64;
        }
        self.state = state;
    }
}

// FNV-1a 32-bit hasher implementation
#[derive(Debug, Copy, Clone)]
pub struct Fnv1aHasher32 {
    state: u32,
}

impl Fnv1aHasher32 {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: FNV_32_OFFSET,
        }
    }

    #[inline(always)]
    pub fn finish_raw(&self) -> u32 {
        self.state
    }
}

impl Default for Fnv1aHasher32 {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl StdHasher for Fnv1aHasher32 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state as u64
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // FNV-1a implementation (xor-multiply)
        let mut state = self.state;
        for &b in bytes {
            state ^= b as u32;
            state = state.wrapping_mul(FNV_32_PRIME);
        }
        self.state = state;
    }
}

// FNV-1a 64-bit hasher implementation
#[derive(Debug, Copy, Clone)]
pub struct Fnv1aHasher64 {
    state: u64,
}

impl Fnv1aHasher64 {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: FNV_64_OFFSET,
        }
    }

    #[inline(always)]
    pub fn finish_raw(&self) -> u64 {
        self.state
    }
}

impl Default for Fnv1aHasher64 {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl StdHasher for Fnv1aHasher64 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // FNV-1a implementation (xor-multiply)
        let mut state = self.state;
        for &b in bytes {
            state ^= b as u64;
            state = state.wrapping_mul(FNV_64_PRIME);
        }
        self.state = state;
    }
}
