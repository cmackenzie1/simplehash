//! Pure-Rust port of Google's FarmHash 64-bit functions (portable variants).
//!
//! - [`farm_fingerprint64`] is the `na` variant: stable across platforms and versions.
//! - [`farm_hash64`] is the `xo` variant, which is what the C++ library selects on
//!   platforms without SSE4.2 acceleration (and what `farmhash-sys` is built to use).
//!   Unlike fingerprints, FarmHash does not promise `Hash64` stays fixed forever.
//!
//! All functions read input as little-endian, so results are identical on every platform.

const K0: u64 = 0xc3a5c85c97cb3127;
const K1: u64 = 0xb492b66fbe98f273;
const K2: u64 = 0x9ae16a3b2f90404f;

#[inline]
fn fetch64(s: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(s[i..i + 8].try_into().unwrap())
}

#[inline]
fn fetch32(s: &[u8], i: usize) -> u64 {
    u32::from_le_bytes(s[i..i + 4].try_into().unwrap()) as u64
}

#[inline]
fn rotate(val: u64, shift: u32) -> u64 {
    val.rotate_right(shift)
}

#[inline]
fn shift_mix(val: u64) -> u64 {
    val ^ (val >> 47)
}

#[inline]
fn hash_len16(u: u64, v: u64) -> u64 {
    const MUL: u64 = 0x9ddfea08eb382d69;
    let mut a = (u ^ v).wrapping_mul(MUL);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(MUL);
    b ^= b >> 47;
    b.wrapping_mul(MUL)
}

#[inline]
fn hash_len16_mul(u: u64, v: u64, mul: u64) -> u64 {
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b.wrapping_mul(mul)
}

fn hash_len0to16(s: &[u8]) -> u64 {
    let len = s.len();
    if len >= 8 {
        let mul = K2.wrapping_add(len as u64 * 2);
        let a = fetch64(s, 0).wrapping_add(K2);
        let b = fetch64(s, len - 8);
        let c = rotate(b, 37).wrapping_mul(mul).wrapping_add(a);
        let d = rotate(a, 25).wrapping_add(b).wrapping_mul(mul);
        return hash_len16_mul(c, d, mul);
    }
    if len >= 4 {
        let mul = K2.wrapping_add(len as u64 * 2);
        let a = fetch32(s, 0);
        return hash_len16_mul((len as u64).wrapping_add(a << 3), fetch32(s, len - 4), mul);
    }
    if len > 0 {
        let a = s[0] as u32;
        let b = s[len >> 1] as u32;
        let c = s[len - 1] as u32;
        let y = a.wrapping_add(b << 8);
        let z = (len as u32).wrapping_add(c << 2);
        return shift_mix((y as u64).wrapping_mul(K2) ^ (z as u64).wrapping_mul(K0))
            .wrapping_mul(K2);
    }
    K2
}

fn hash_len17to32(s: &[u8]) -> u64 {
    let len = s.len();
    let mul = K2.wrapping_add(len as u64 * 2);
    let a = fetch64(s, 0).wrapping_mul(K1);
    let b = fetch64(s, 8);
    let c = fetch64(s, len - 8).wrapping_mul(mul);
    let d = fetch64(s, len - 16).wrapping_mul(K2);
    hash_len16_mul(
        rotate(a.wrapping_add(b), 43)
            .wrapping_add(rotate(c, 30))
            .wrapping_add(d),
        a.wrapping_add(rotate(b.wrapping_add(K2), 18))
            .wrapping_add(c),
        mul,
    )
}

fn hash_len33to64_na(s: &[u8]) -> u64 {
    let len = s.len();
    let mul = K2.wrapping_add(len as u64 * 2);
    let a = fetch64(s, 0).wrapping_mul(K2);
    let b = fetch64(s, 8);
    let c = fetch64(s, len - 8).wrapping_mul(mul);
    let d = fetch64(s, len - 16).wrapping_mul(K2);
    let y = rotate(a.wrapping_add(b), 43)
        .wrapping_add(rotate(c, 30))
        .wrapping_add(d);
    let z = hash_len16_mul(
        y,
        a.wrapping_add(rotate(b.wrapping_add(K2), 18))
            .wrapping_add(c),
        mul,
    );
    let e = fetch64(s, 16).wrapping_mul(mul);
    let f = fetch64(s, 24);
    let g = y.wrapping_add(fetch64(s, len - 32)).wrapping_mul(mul);
    let h = z.wrapping_add(fetch64(s, len - 24)).wrapping_mul(mul);
    hash_len16_mul(
        rotate(e.wrapping_add(f), 43)
            .wrapping_add(rotate(g, 30))
            .wrapping_add(h),
        e.wrapping_add(rotate(f.wrapping_add(a), 18))
            .wrapping_add(g),
        mul,
    )
}

fn weak_hash_len32_with_seeds_raw(w: u64, x: u64, y: u64, z: u64, a: u64, b: u64) -> (u64, u64) {
    let a = a.wrapping_add(w);
    let b = rotate(b.wrapping_add(a).wrapping_add(z), 21);
    let c = a;
    let a = a.wrapping_add(x).wrapping_add(y);
    let b = b.wrapping_add(rotate(a, 44));
    (a.wrapping_add(z), b.wrapping_add(c))
}

fn weak_hash_len32_with_seeds(s: &[u8], i: usize, a: u64, b: u64) -> (u64, u64) {
    weak_hash_len32_with_seeds_raw(
        fetch64(s, i),
        fetch64(s, i + 8),
        fetch64(s, i + 16),
        fetch64(s, i + 24),
        a,
        b,
    )
}

/// farmhashna::Hash64 — also the forever-stable fingerprint.
fn na_hash64(s: &[u8]) -> u64 {
    let len = s.len();
    if len <= 32 {
        return if len <= 16 {
            hash_len0to16(s)
        } else {
            hash_len17to32(s)
        };
    } else if len <= 64 {
        return hash_len33to64_na(s);
    }

    const SEED: u64 = 81;
    let mut x = SEED;
    let mut y = SEED.wrapping_mul(K1).wrapping_add(113);
    let mut z = shift_mix(y.wrapping_mul(K2).wrapping_add(113)).wrapping_mul(K2);
    let mut v = (0u64, 0u64);
    let mut w = (0u64, 0u64);
    x = x.wrapping_mul(K2).wrapping_add(fetch64(s, 0));

    let end = ((len - 1) / 64) * 64;
    let last64 = end + ((len - 1) & 63) - 63;
    let mut p = 0;
    loop {
        x = rotate(
            x.wrapping_add(y)
                .wrapping_add(v.0)
                .wrapping_add(fetch64(s, p + 8)),
            37,
        )
        .wrapping_mul(K1);
        y = rotate(y.wrapping_add(v.1).wrapping_add(fetch64(s, p + 48)), 42).wrapping_mul(K1);
        x ^= w.1;
        y = y.wrapping_add(v.0).wrapping_add(fetch64(s, p + 40));
        z = rotate(z.wrapping_add(w.0), 33).wrapping_mul(K1);
        v = weak_hash_len32_with_seeds(s, p, v.1.wrapping_mul(K1), x.wrapping_add(w.0));
        w = weak_hash_len32_with_seeds(
            s,
            p + 32,
            z.wrapping_add(w.1),
            y.wrapping_add(fetch64(s, p + 16)),
        );
        std::mem::swap(&mut z, &mut x);
        p += 64;
        if p == end {
            break;
        }
    }
    let mul = K1.wrapping_add((z & 0xff) << 1);
    let p = last64;
    w.0 = w.0.wrapping_add(((len - 1) & 63) as u64);
    v.0 = v.0.wrapping_add(w.0);
    w.0 = w.0.wrapping_add(v.0);
    x = rotate(
        x.wrapping_add(y)
            .wrapping_add(v.0)
            .wrapping_add(fetch64(s, p + 8)),
        37,
    )
    .wrapping_mul(mul);
    y = rotate(y.wrapping_add(v.1).wrapping_add(fetch64(s, p + 48)), 42).wrapping_mul(mul);
    x ^= w.1.wrapping_mul(9);
    y = y
        .wrapping_add(v.0.wrapping_mul(9))
        .wrapping_add(fetch64(s, p + 40));
    z = rotate(z.wrapping_add(w.0), 33).wrapping_mul(mul);
    v = weak_hash_len32_with_seeds(s, p, v.1.wrapping_mul(mul), x.wrapping_add(w.0));
    w = weak_hash_len32_with_seeds(
        s,
        p + 32,
        z.wrapping_add(w.1),
        y.wrapping_add(fetch64(s, p + 16)),
    );
    std::mem::swap(&mut z, &mut x);
    hash_len16_mul(
        hash_len16_mul(v.0, w.0, mul)
            .wrapping_add(shift_mix(y).wrapping_mul(K0))
            .wrapping_add(z),
        hash_len16_mul(v.1, w.1, mul).wrapping_add(x),
        mul,
    )
}

fn na_hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    hash_len16(na_hash64(s).wrapping_sub(seed0), seed1)
}

fn uo_h(x: u64, y: u64, mul: u64, r: u32) -> u64 {
    let mut a = (x ^ y).wrapping_mul(mul);
    a ^= a >> 47;
    let b = (y ^ a).wrapping_mul(mul);
    rotate(b, r).wrapping_mul(mul)
}

/// farmhashuo::Hash64WithSeeds
fn uo_hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    let len = s.len();
    if len <= 64 {
        return na_hash64_with_seeds(s, seed0, seed1);
    }

    let mut x = seed0;
    let mut y = seed1.wrapping_mul(K2).wrapping_add(113);
    let mut z = shift_mix(y.wrapping_mul(K2)).wrapping_mul(K2);
    let mut v = (seed0, seed1);
    let mut w = (0u64, 0u64);
    let mut u = x.wrapping_sub(z);
    x = x.wrapping_mul(K2);
    let mul = K2.wrapping_add(u & 0x82);

    let end = ((len - 1) / 64) * 64;
    let last64 = end + ((len - 1) & 63) - 63;
    let mut p = 0;
    loop {
        let a0 = fetch64(s, p);
        let a1 = fetch64(s, p + 8);
        let a2 = fetch64(s, p + 16);
        let a3 = fetch64(s, p + 24);
        let a4 = fetch64(s, p + 32);
        let a5 = fetch64(s, p + 40);
        let a6 = fetch64(s, p + 48);
        let a7 = fetch64(s, p + 56);
        x = x.wrapping_add(a0).wrapping_add(a1);
        y = y.wrapping_add(a2);
        z = z.wrapping_add(a3);
        v.0 = v.0.wrapping_add(a4);
        v.1 = v.1.wrapping_add(a5).wrapping_add(a1);
        w.0 = w.0.wrapping_add(a6);
        w.1 = w.1.wrapping_add(a7);

        x = rotate(x, 26).wrapping_mul(9);
        y = rotate(y, 29);
        z = z.wrapping_mul(mul);
        v.0 = rotate(v.0, 33);
        v.1 = rotate(v.1, 30);
        w.0 ^= x;
        w.0 = w.0.wrapping_mul(9);
        z = rotate(z, 32);
        z = z.wrapping_add(w.1);
        w.1 = w.1.wrapping_add(z);
        z = z.wrapping_mul(9);
        std::mem::swap(&mut u, &mut y);

        z = z.wrapping_add(a0).wrapping_add(a6);
        v.0 = v.0.wrapping_add(a2);
        v.1 = v.1.wrapping_add(a3);
        w.0 = w.0.wrapping_add(a4);
        w.1 = w.1.wrapping_add(a5).wrapping_add(a6);
        x = x.wrapping_add(a1);
        y = y.wrapping_add(a7);

        y = y.wrapping_add(v.0);
        v.0 = v.0.wrapping_add(x.wrapping_sub(y));
        v.1 = v.1.wrapping_add(w.0);
        w.0 = w.0.wrapping_add(v.1);
        w.1 = w.1.wrapping_add(x.wrapping_sub(y));
        x = x.wrapping_add(w.1);
        w.1 = rotate(w.1, 34);
        std::mem::swap(&mut u, &mut z);
        p += 64;
        if p == end {
            break;
        }
    }
    let p = last64;
    u = u.wrapping_mul(9);
    v.1 = rotate(v.1, 28);
    v.0 = rotate(v.0, 20);
    w.0 = w.0.wrapping_add(((len - 1) & 63) as u64);
    u = u.wrapping_add(y);
    y = y.wrapping_add(u);
    x = rotate(
        y.wrapping_sub(x)
            .wrapping_add(v.0)
            .wrapping_add(fetch64(s, p + 8)),
        37,
    )
    .wrapping_mul(mul);
    y = rotate(y ^ v.1 ^ fetch64(s, p + 48), 42).wrapping_mul(mul);
    x ^= w.1.wrapping_mul(9);
    y = y.wrapping_add(v.0).wrapping_add(fetch64(s, p + 40));
    z = rotate(z.wrapping_add(w.0), 33).wrapping_mul(mul);
    v = weak_hash_len32_with_seeds(s, p, v.1.wrapping_mul(mul), x.wrapping_add(w.0));
    w = weak_hash_len32_with_seeds(
        s,
        p + 32,
        z.wrapping_add(w.1),
        y.wrapping_add(fetch64(s, p + 16)),
    );
    uo_h(
        hash_len16_mul(v.0.wrapping_add(x), w.0 ^ y, mul)
            .wrapping_add(z)
            .wrapping_sub(u),
        uo_h(v.1.wrapping_add(y), w.1.wrapping_add(z), K2, 30) ^ x,
        K2,
        31,
    )
}

fn xo_h32(s: &[u8], mul: u64, seed0: u64, seed1: u64) -> u64 {
    // Always called on exactly 32 bytes.
    let a = fetch64(s, 0).wrapping_mul(K1);
    let b = fetch64(s, 8);
    let c = fetch64(s, 24).wrapping_mul(mul);
    let d = fetch64(s, 16).wrapping_mul(K2);
    let u = rotate(a.wrapping_add(b), 43)
        .wrapping_add(rotate(c, 30))
        .wrapping_add(d)
        .wrapping_add(seed0);
    let v = a
        .wrapping_add(rotate(b.wrapping_add(K2), 18))
        .wrapping_add(c)
        .wrapping_add(seed1);
    let a = shift_mix((u ^ v).wrapping_mul(mul));
    shift_mix((v ^ a).wrapping_mul(mul))
}

fn xo_hash_len33to64(s: &[u8]) -> u64 {
    let len = s.len();
    let mul0 = K2.wrapping_sub(30);
    let mul1 = K2.wrapping_sub(30).wrapping_add(2 * len as u64);
    let h0 = xo_h32(&s[..32], mul0, 0, 0);
    let h1 = xo_h32(&s[len - 32..], mul1, 0, 0);
    h1.wrapping_mul(mul1).wrapping_add(h0).wrapping_mul(mul1)
}

fn xo_hash_len65to96(s: &[u8]) -> u64 {
    let len = s.len();
    let mul0 = K2.wrapping_sub(114);
    let mul1 = K2.wrapping_sub(114).wrapping_add(2 * len as u64);
    let h0 = xo_h32(&s[..32], mul0, 0, 0);
    let h1 = xo_h32(&s[32..64], mul1, 0, 0);
    let h2 = xo_h32(&s[len - 32..], mul1, h0, h1);
    h2.wrapping_mul(9)
        .wrapping_add(h0 >> 17)
        .wrapping_add(h1 >> 21)
        .wrapping_mul(mul1)
}

fn xo_hash64(s: &[u8]) -> u64 {
    match s.len() {
        0..=16 => hash_len0to16(s),
        17..=32 => hash_len17to32(s),
        33..=64 => xo_hash_len33to64(s),
        65..=96 => xo_hash_len65to96(s),
        97..=256 => na_hash64(s),
        _ => uo_hash64_with_seeds(s, 81, 0),
    }
}

/// Platform-independent 64-bit hash (FarmHash `Hash64`, portable `xo` variant).
#[must_use]
pub fn farm_hash64(s: &[u8]) -> u64 {
    xo_hash64(s)
}

/// 64-bit hash with one seed (FarmHash `Hash64WithSeed`).
#[must_use]
pub fn farm_hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    farm_hash64_with_seeds(s, K2, seed)
}

/// 64-bit hash with two seeds (FarmHash `Hash64WithSeeds`).
#[must_use]
pub fn farm_hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    na_hash64_with_seeds(s, seed0, seed1)
}

/// Stable 64-bit fingerprint (FarmHash `Fingerprint64`); never changes between versions.
#[must_use]
pub fn farm_fingerprint64(s: &[u8]) -> u64 {
    na_hash64(s)
}

/// A hasher usable with `std::collections`. Buffers input and hashes on `finish`.
#[derive(Default)]
pub struct FarmHasher64 {
    buffer: Vec<u8>,
}

impl FarmHasher64 {
    /// Creates a new FarmHasher64
    #[must_use]
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl std::hash::Hasher for FarmHasher64 {
    fn write(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finish(&self) -> u64 {
        farm_hash64(&self.buffer)
    }
}

impl std::hash::BuildHasher for FarmHasher64 {
    type Hasher = FarmHasher64;

    fn build_hasher(&self) -> Self::Hasher {
        FarmHasher64::new()
    }
}

/// A type alias for using FarmHasher64 with std::collections
pub type FarmHashMap<K, V> = std::collections::HashMap<K, V, FarmHasher64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_k2() {
        assert_eq!(farm_hash64(b""), K2);
        assert_eq!(farm_fingerprint64(b""), K2);
    }

    #[test]
    fn matches_cpp_across_all_length_classes() {
        let data: Vec<u8> = (0..2048u32)
            .map(|i| (i.wrapping_mul(31) >> 2) as u8)
            .collect();
        for len in 0..=data.len() {
            let s = &data[..len];
            assert_eq!(
                farm_hash64(s),
                farmhash_sys::farmhash::hash64(s),
                "hash64 len {len}"
            );
            assert_eq!(
                farm_fingerprint64(s),
                farmhash_sys::farmhash::fingerprint64(s),
                "fingerprint64 len {len}"
            );
            assert_eq!(
                farm_hash64_with_seed(s, 42),
                farmhash_sys::farmhash::hash64_with_seed(s, 42),
                "seed len {len}"
            );
            assert_eq!(
                farm_hash64_with_seeds(s, 7, 99),
                farmhash_sys::farmhash::hash64_with_seeds(s, 7, 99),
                "seeds len {len}"
            );
        }
    }
}
