use rand::{Rng, SeedableRng};
use simplehash::farm::{
    farm_fingerprint64, farm_hash64, farm_hash64_with_seed, farm_hash64_with_seeds,
};

fn check(input: &[u8]) {
    assert_eq!(
        farm_hash64(input),
        farmhash_sys::farmhash::hash64(input),
        "hash64 mismatch for len {}",
        input.len()
    );
    assert_eq!(
        farm_fingerprint64(input),
        farmhash_sys::farmhash::fingerprint64(input),
        "fingerprint64 mismatch for len {}",
        input.len()
    );
    assert_eq!(
        farm_hash64_with_seed(input, 0xdead_beef),
        farmhash_sys::farmhash::hash64_with_seed(input, 0xdead_beef),
        "hash64_with_seed mismatch for len {}",
        input.len()
    );
    assert_eq!(
        farm_hash64_with_seeds(input, 1, 2),
        farmhash_sys::farmhash::hash64_with_seeds(input, 1, 2),
        "hash64_with_seeds mismatch for len {}",
        input.len()
    );
}

#[test]
fn test_empty_string() {
    assert_eq!(farm_hash64(b""), 0x9ae16a3b2f90404f);
    check(b"");
}

#[test]
fn test_real_strings() {
    for s in ["a", "abc", "hello world", "Hello, World", "Hello, 世界!"] {
        check(s.as_bytes());
    }
}

#[test]
fn test_all_small_lengths() {
    let data: Vec<u8> = (0..1024u32).map(|i| (i * 7 + 3) as u8).collect();
    for len in 0..=data.len() {
        check(&data[..len]);
    }
}

#[test]
fn test_special_patterns() {
    for len in [1, 16, 32, 33, 64, 65, 96, 97, 256, 257, 1000, 4096] {
        check(&vec![0u8; len]);
        check(&vec![0xffu8; len]);
    }
}

#[test]
fn test_unaligned() {
    let data: Vec<u8> = (0..600u32).map(|i| i as u8).collect();
    for off in 0..8 {
        check(&data[off..off + 300]);
    }
}

#[test]
fn test_random_data() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    for _ in 0..500 {
        let len = rng.gen_range(0..5000);
        let data: Vec<u8> = (0..len).map(|_| rng.r#gen()).collect();
        check(&data);
    }
}
