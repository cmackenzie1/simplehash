use farmhash_sys::{Uint128, farmhash};

// Test direct top-level API calls
#[test]
fn test_top_level_32bit_functions() {
    let input = b"hello world";

    // Test farm_hash_32
    let hash32 = farmhash_sys::farm_hash_32(input);
    // Hash values depend on platform, so we don't test exact values
    assert_ne!(hash32, 0);

    // Test farm_hash_32_with_seed
    let hash32_seeded = farmhash_sys::farm_hash_32_with_seed(input, 42);
    assert_ne!(hash32, hash32_seeded); // Should be different with seed
    assert_eq!(
        hash32_seeded,
        farmhash_sys::farm_hash_32_with_seed(input, 42)
    ); // Should be deterministic

    // Test farm_fingerprint_32
    let fingerprint32 = farmhash_sys::farm_fingerprint_32(input);
    assert_ne!(hash32, fingerprint32); // Hash and fingerprint should be different
    assert_eq!(fingerprint32, farmhash_sys::farm_fingerprint_32(input)); // Should be deterministic
}

#[test]
fn test_top_level_64bit_functions() {
    let input = b"hello world";

    // Test farm_hash_64
    let hash64 = farmhash_sys::farm_hash_64(input);
    assert_ne!(hash64, 0);

    // Test farm_hash_64_with_seed
    let hash64_seeded = farmhash_sys::farm_hash_64_with_seed(input, 42);
    assert_ne!(hash64, hash64_seeded); // Should be different with seed

    // Test farm_hash_64_with_seeds
    let hash64_two_seeds = farmhash_sys::farm_hash_64_with_seeds(input, 42, 43);
    assert_ne!(hash64_seeded, hash64_two_seeds); // Should be different with different seeds

    // Test farm_fingerprint_64
    let fingerprint64 = farmhash_sys::farm_fingerprint_64(input);
    assert_eq!(fingerprint64, farmhash_sys::farm_fingerprint_64(input)); // Should be deterministic

    // Test farm_fingerprint_64_with_seed
    let fingerprint64_seeded = farmhash_sys::farm_fingerprint_64_with_seed(input, 42);
    assert_ne!(fingerprint64, fingerprint64_seeded); // Should be different with seed
}

#[test]
fn test_top_level_128bit_functions() {
    let input = b"hello world";

    // Test farm_fingerprint_128
    let fingerprint128 = farmhash_sys::farm_fingerprint_128(input);
    assert!(fingerprint128.low != 0 || fingerprint128.high != 0);
    assert_eq!(fingerprint128, farmhash_sys::farm_fingerprint_128(input)); // Should be deterministic

    // Test farm_fingerprint_128_with_seed
    let seed = Uint128::new(42, 43);
    let fingerprint128_seeded = farmhash_sys::farm_fingerprint_128_with_seed(input, seed);
    assert_ne!(fingerprint128, fingerprint128_seeded); // Should be different with seed

    // Test conversions
    let tuple: (u64, u64) = fingerprint128.into();
    assert_eq!(tuple.0, fingerprint128.low);
    assert_eq!(tuple.1, fingerprint128.high);
}

// Test namespaced module API
#[test]
fn test_farmhash_32bit_functions() {
    let input = b"hello world";

    // Test hash32
    let hash32 = farmhash::hash32(input);
    // Hash values depend on platform, so we don't test exact values
    assert_ne!(hash32, 0);

    // Test hash32_with_seed
    let hash32_seeded = farmhash::hash32_with_seed(input, 42);
    assert_ne!(hash32, hash32_seeded); // Should be different with seed

    // Test fingerprint32
    let fingerprint32 = farmhash::fingerprint32(input);
    assert_ne!(hash32, fingerprint32); // Hash and fingerprint should be different
}

#[test]
fn test_farmhash_64bit_functions() {
    let input = b"Moscow";

    // Test hash64
    let hash64 = farmhash::hash64(input);
    // Hash values depend on platform, so we don't test exact values
    assert_ne!(hash64, 0);

    // Test hash64_with_seed
    let hash64_seeded = farmhash::hash64_with_seed(input, 42);
    assert_ne!(hash64, hash64_seeded); // Should be different with seed

    // Test hash64_with_seeds
    let hash64_two_seeds = farmhash::hash64_with_seeds(input, 42, 43);
    assert_ne!(hash64_seeded, hash64_two_seeds); // Should be different with different seeds

    // Test fingerprint64
    let fingerprint64 = farmhash::fingerprint64(input);
    assert_eq!(fingerprint64, farmhash::fingerprint64(input)); // Should be deterministic

    // Test fingerprint64_with_seed
    let fingerprint64_seeded = farmhash::fingerprint64_with_seed(input, 42);
    assert_ne!(fingerprint64, fingerprint64_seeded); // Should be different with seed
}

#[test]
fn test_farmhash_128bit_functions() {
    let input = b"hello world";

    // Test fingerprint128
    let fingerprint128 = farmhash::fingerprint128(input);
    assert!(fingerprint128.low != 0 || fingerprint128.high != 0);

    // Test fingerprint128_with_seed
    let seed = Uint128::new(42, 43);
    let fingerprint128_seeded = farmhash::fingerprint128_with_seed(input, seed);
    assert_ne!(fingerprint128, fingerprint128_seeded); // Should be different with seed
}

// Test compatibility between top-level and module API
#[test]
fn test_api_compatibility() {
    let input = b"hello world";

    // 32-bit functions
    assert_eq!(farmhash_sys::farm_hash_32(input), farmhash::hash32(input));
    assert_eq!(
        farmhash_sys::farm_hash_32_with_seed(input, 42),
        farmhash::hash32_with_seed(input, 42)
    );
    assert_eq!(
        farmhash_sys::farm_fingerprint_32(input),
        farmhash::fingerprint32(input)
    );

    // 64-bit functions
    assert_eq!(farmhash_sys::farm_hash_64(input), farmhash::hash64(input));
    assert_eq!(
        farmhash_sys::farm_hash_64_with_seed(input, 42),
        farmhash::hash64_with_seed(input, 42)
    );
    assert_eq!(
        farmhash_sys::farm_hash_64_with_seeds(input, 42, 43),
        farmhash::hash64_with_seeds(input, 42, 43)
    );
    assert_eq!(
        farmhash_sys::farm_fingerprint_64(input),
        farmhash::fingerprint64(input)
    );
    assert_eq!(
        farmhash_sys::farm_fingerprint_64_with_seed(input, 42),
        farmhash::fingerprint64_with_seed(input, 42)
    );

    // 128-bit functions
    let top_level = farmhash_sys::farm_fingerprint_128(input);
    let module = farmhash::fingerprint128(input);
    assert_eq!(top_level.low, module.low);
    assert_eq!(top_level.high, module.high);

    // With seed
    let seed = Uint128::new(42, 43);
    let top_level_seeded = farmhash_sys::farm_fingerprint_128_with_seed(input, seed);
    let module_seeded = farmhash::fingerprint128_with_seed(input, seed);
    assert_eq!(top_level_seeded.low, module_seeded.low);
    assert_eq!(top_level_seeded.high, module_seeded.high);
}
