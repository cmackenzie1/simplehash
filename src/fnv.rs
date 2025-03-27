use std::hash::Hasher as StdHasher;

const FNV_32_OFFSET: u32 = 0x811c9dc5;
const FNV_32_PRIME: u32 = 0x01000193;
const FNV_64_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_64_PRIME: u64 = 0x00000100000001b3;

macro_rules! define_fnv_hasher {
    ($name:ident, $output:ty, $offset:expr, $prime:expr, $algorithm:ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            state: $output,
        }

        impl $name {
            #[inline]
            pub fn new() -> Self {
                Self {
                    state: $offset,
                }
            }

            #[inline]
            pub fn finish_raw(&self) -> $output {
                self.state
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl StdHasher for $name {
            #[inline]
            fn finish(&self) -> u64 {
                self.state as u64
            }

            #[inline]
            fn write(&mut self, bytes: &[u8]) {
                for &byte in bytes {
                    define_fnv_hasher!(@$algorithm self, byte, $prime, $output);
                }
            }
        }
    };
    (@fnv $self:ident, $byte:ident, $prime:expr, $type:ty) => {
        $self.state = $self.state.wrapping_mul($prime);
        $self.state ^= $byte as $type;
    };
    (@fnv1a $self:ident, $byte:ident, $prime:expr, $type:ty) => {
        $self.state ^= $byte as $type;
        $self.state = $self.state.wrapping_mul($prime);
    };
}

define_fnv_hasher!(FnvHasher32, u32, FNV_32_OFFSET, FNV_32_PRIME, fnv);
define_fnv_hasher!(FnvHasher64, u64, FNV_64_OFFSET, FNV_64_PRIME, fnv);
define_fnv_hasher!(Fnv1aHasher32, u32, FNV_32_OFFSET, FNV_32_PRIME, fnv1a);
define_fnv_hasher!(Fnv1aHasher64, u64, FNV_64_OFFSET, FNV_64_PRIME, fnv1a);
