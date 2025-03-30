use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use simplehash::fnv::Fnv1aHasher64;
use simplehash::murmur::MurmurHasher32;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::time::Instant;

// BuildHasher for MurmurHash3
#[derive(Default, Clone)]
struct MurmurHash3BuildHasher;

impl BuildHasher for MurmurHash3BuildHasher {
    type Hasher = MurmurHasher32;

    fn build_hasher(&self) -> Self::Hasher {
        MurmurHasher32::new(0) // Use seed 0
    }
}

/// Generate random strings of a specific length
fn generate_random_strings(count: usize, length: usize, seed: u64) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(seed);
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();

    (0..count)
        .map(|_| {
            (0..length)
                .map(|_| charset[rng.gen_range(0..charset.len())])
                .collect()
        })
        .collect()
}

// Benchmark HashMap operations with different hashers and key lengths
fn bench_hashmap_key_lengths(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap Performance by Key Length");
    group.sample_size(50); // Adjust sample size for more stable results

    // Test with different key lengths
    let key_lengths = [4, 16, 64, 256]; // Reduced set for faster benchmarking
    let num_keys = 5_000; // Reduced number of keys for faster benchmarking
    let seed = 42; // Fixed seed for reproducibility

    // Outer loop over key lengths
    for &length in &key_lengths {
        // Generate random keys of the specified length
        let keys = generate_random_strings(num_keys, length, seed);

        // 1. Standard HashMap (default SipHash) - Insert
        group.bench_function(BenchmarkId::new("StdHashMap-Insert", length), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = std::time::Duration::new(0, 0);

                for _ in 0..iters {
                    let mut map: HashMap<String, u32> = HashMap::with_capacity(num_keys);
                    let start = Instant::now();

                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as u32);
                    }

                    total_duration += start.elapsed();
                    black_box(&map);
                }

                total_duration
            });
        });

        // 2. HashMap with FNV1a-64 - Insert
        group.bench_function(BenchmarkId::new("FNV1a64-HashMap-Insert", length), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = std::time::Duration::new(0, 0);

                for _ in 0..iters {
                    let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
                        HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());

                    let start = Instant::now();

                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as u32);
                    }

                    total_duration += start.elapsed();
                    black_box(&map);
                }

                total_duration
            });
        });

        // 3. HashMap with MurmurHash3-32 - Insert
        group.bench_function(
            BenchmarkId::new("MurmurHash3-32-HashMap-Insert", length),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = std::time::Duration::new(0, 0);

                    for _ in 0..iters {
                        let mut map: HashMap<String, u32, MurmurHash3BuildHasher> =
                            HashMap::with_hasher(MurmurHash3BuildHasher);

                        let start = Instant::now();

                        for (i, key) in keys.iter().enumerate() {
                            map.insert(key.clone(), i as u32);
                        }

                        total_duration += start.elapsed();
                        black_box(&map);
                    }

                    total_duration
                });
            },
        );

        // Now benchmark lookup performance

        // 1. Standard HashMap (default SipHash) - Lookup
        group.bench_function(BenchmarkId::new("StdHashMap-Lookup", length), |b| {
            let mut map: HashMap<String, u32> = HashMap::with_capacity(num_keys);
            for (i, key) in keys.iter().enumerate() {
                map.insert(key.clone(), i as u32);
            }

            // Randomly select 1000 keys for lookup
            let mut rng = StdRng::seed_from_u64(seed + 1);
            let lookup_indices: Vec<usize> =
                (0..1000).map(|_| rng.gen_range(0..num_keys)).collect();
            let lookup_keys: Vec<&String> = lookup_indices.iter().map(|&i| &keys[i]).collect();

            b.iter(|| {
                for key in black_box(&lookup_keys) {
                    black_box(map.get(*key));
                }
            });
        });

        // 2. HashMap with FNV1a-64 - Lookup
        group.bench_function(BenchmarkId::new("FNV1a64-HashMap-Lookup", length), |b| {
            let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
                HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());

            for (i, key) in keys.iter().enumerate() {
                map.insert(key.clone(), i as u32);
            }

            // Randomly select 1000 keys for lookup
            let mut rng = StdRng::seed_from_u64(seed + 1);
            let lookup_indices: Vec<usize> =
                (0..1000).map(|_| rng.gen_range(0..num_keys)).collect();
            let lookup_keys: Vec<&String> = lookup_indices.iter().map(|&i| &keys[i]).collect();

            b.iter(|| {
                for key in black_box(&lookup_keys) {
                    black_box(map.get(*key));
                }
            });
        });

        // 3. HashMap with MurmurHash3-32 - Lookup
        group.bench_function(
            BenchmarkId::new("MurmurHash3-32-HashMap-Lookup", length),
            |b| {
                let mut map: HashMap<String, u32, MurmurHash3BuildHasher> =
                    HashMap::with_hasher(MurmurHash3BuildHasher);

                for (i, key) in keys.iter().enumerate() {
                    map.insert(key.clone(), i as u32);
                }

                // Randomly select 1000 keys for lookup
                let mut rng = StdRng::seed_from_u64(seed + 1);
                let lookup_indices: Vec<usize> =
                    (0..1000).map(|_| rng.gen_range(0..num_keys)).collect();
                let lookup_keys: Vec<&String> = lookup_indices.iter().map(|&i| &keys[i]).collect();

                b.iter(|| {
                    for key in black_box(&lookup_keys) {
                        black_box(map.get(*key));
                    }
                });
            },
        );
    }

    group.finish();
}

// Benchmark HashMap operations with different hashers and varying fill factors
fn bench_hashmap_fill_factor(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap Performance by Fill Factor");

    // Test different fill factors (percentage of capacity that's filled)
    let fill_factors = [25, 75, 99]; // Reduced set for faster benchmarking
    let capacity = 20_000; // Reduced capacity for faster benchmarking
    let key_length = 16; // Fixed key length
    let seed = 42; // Fixed seed for reproducibility

    for &fill_factor in &fill_factors {
        let num_keys = (capacity * fill_factor) / 100;
        let keys = generate_random_strings(num_keys, key_length, seed);

        // 1. Standard HashMap (default SipHash)
        group.bench_function(BenchmarkId::new("StdHashMap", fill_factor), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = std::time::Duration::new(0, 0);

                for _ in 0..iters {
                    let mut map: HashMap<String, u32> = HashMap::with_capacity(capacity);
                    let start = Instant::now();

                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as u32);
                    }

                    total_duration += start.elapsed();
                    black_box(&map);
                }

                total_duration
            });
        });

        // 2. HashMap with FNV1a-64
        group.bench_function(BenchmarkId::new("FNV1a64-HashMap", fill_factor), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = std::time::Duration::new(0, 0);

                for _ in 0..iters {
                    let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
                        HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());
                    map.reserve(capacity);

                    let start = Instant::now();

                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as u32);
                    }

                    total_duration += start.elapsed();
                    black_box(&map);
                }

                total_duration
            });
        });

        // 3. HashMap with MurmurHash3-32
        group.bench_function(
            BenchmarkId::new("MurmurHash3-32-HashMap", fill_factor),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = std::time::Duration::new(0, 0);

                    for _ in 0..iters {
                        let mut map: HashMap<String, u32, MurmurHash3BuildHasher> =
                            HashMap::with_hasher(MurmurHash3BuildHasher);
                        map.reserve(capacity);

                        let start = Instant::now();

                        for (i, key) in keys.iter().enumerate() {
                            map.insert(key.clone(), i as u32);
                        }

                        total_duration += start.elapsed();
                        black_box(&map);
                    }

                    total_duration
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_hashmap_key_lengths,
    bench_hashmap_fill_factor
);
criterion_main!(benches);
