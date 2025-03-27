use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use simplehash::fnv::Fnv1aHasher64;
use simplehash::murmur::MurmurHasher32;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use std::time::Instant; // Import the specific hashers

// Custom BuildHasher for MurmurHash3
struct MurmurHash3Hasher32(u32);

impl MurmurHash3Hasher32 {
    fn new() -> Self {
        Self(0)
    }
}

impl Default for MurmurHash3Hasher32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for MurmurHash3Hasher32 {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = simplehash::murmurhash3_32(bytes, 0);
    }
}

#[derive(Default, Clone)]
struct MurmurHash3BuildHasher;

impl BuildHasher for MurmurHash3BuildHasher {
    type Hasher = MurmurHash3Hasher32;

    fn build_hasher(&self) -> Self::Hasher {
        MurmurHash3Hasher32::new()
    }
}

// Benchmark HashMap operations with different hashers
fn bench_hashmap_with_different_hashers(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap Performance");

    // Test with different numbers of elements
    let sizes = [100, 1_000, 10_000];

    for size in sizes {
        // Prepare data - use strings as keys
        let keys: Vec<String> = (0..size).map(|i| format!("key_{}", i)).collect();

        // 1. Standard HashMap (default SipHash)
        group.bench_function(BenchmarkId::new("StdHashMap-Insert", size), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = std::time::Duration::new(0, 0);

                for _ in 0..iters {
                    let mut map: HashMap<String, u32> = HashMap::with_capacity(size as usize);
                    let start = Instant::now();

                    for (i, key) in keys.iter().enumerate() {
                        map.insert(key.clone(), i as u32);
                    }

                    total_duration += start.elapsed();

                    // Prevent the map from being optimized away
                    black_box(&map);
                }

                total_duration
            });
        });

        // 2. HashMap with FNV1a-64
        group.bench_function(BenchmarkId::new("FNV1a64-HashMap-Insert", size), |b| {
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

                    // Prevent the map from being optimized away
                    black_box(&map);
                }

                total_duration
            });
        });

        // 3. HashMap with MurmurHash3-32
        group.bench_function(
            BenchmarkId::new("MurmurHash3-32-HashMap-Insert", size),
            |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = std::time::Duration::new(0, 0);

                    for _ in 0..iters {
                        let mut map: HashMap<String, u32, MurmurHasher32> =
                            HashMap::with_hasher(MurmurHasher32);

                        let start = Instant::now();

                        for (i, key) in keys.iter().enumerate() {
                            map.insert(key.clone(), i as u32);
                        }

                        total_duration += start.elapsed();

                        // Prevent the map from being optimized away
                        black_box(&map);
                    }

                    total_duration
                });
            },
        );

        // Benchmark lookup performance
        let lookup_keys: Vec<&String> = keys.iter().step_by(10).collect();

        // 1. Standard HashMap (default SipHash) - Lookup
        group.bench_function(BenchmarkId::new("StdHashMap-Lookup", size), |b| {
            let mut map: HashMap<String, u32> = HashMap::with_capacity(size as usize);
            for (i, key) in keys.iter().enumerate() {
                map.insert(key.clone(), i as u32);
            }

            b.iter(|| {
                for key in black_box(&lookup_keys) {
                    black_box(map.get(*key));
                }
            });
        });

        // 2. HashMap with FNV1a-64 - Lookup
        group.bench_function(BenchmarkId::new("FNV1a64-HashMap-Lookup", size), |b| {
            let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
                HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());

            for (i, key) in keys.iter().enumerate() {
                map.insert(key.clone(), i as u32);
            }

            b.iter(|| {
                for key in black_box(&lookup_keys) {
                    black_box(map.get(*key));
                }
            });
        });

        // 3. HashMap with MurmurHash3-32 - Lookup
        group.bench_function(
            BenchmarkId::new("MurmurHash3-32-HashMap-Lookup", size),
            |b| {
                let mut map: HashMap<String, u32, MurmurHash3BuildHasher> =
                    HashMap::with_hasher(MurmurHash3BuildHasher);

                for (i, key) in keys.iter().enumerate() {
                    map.insert(key.clone(), i as u32);
                }

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

// Benchmark collision resistance with similar keys
fn bench_collision_resistance(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap Collision Resistance");

    // Test with keys that might cause collisions with poor hash functions
    let size = 10_000;

    // Generate keys with different patterns
    let similar_keys: Vec<String> = (0..size)
        .map(|i| format!("prefix_{:010}", i)) // Keys with the same prefix
        .collect();

    // 1. Standard HashMap (default SipHash)
    group.bench_function("StdHashMap-SimilarKeys", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = std::time::Duration::new(0, 0);

            for _ in 0..iters {
                let mut map: HashMap<String, u32> = HashMap::with_capacity(size as usize);
                let start = Instant::now();

                for (i, key) in similar_keys.iter().enumerate() {
                    map.insert(key.clone(), i as u32);
                }

                total_duration += start.elapsed();

                // Prevent the map from being optimized away
                black_box(&map);
            }

            total_duration
        });
    });

    // 2. HashMap with FNV1a-64
    group.bench_function("FNV1a64-HashMap-SimilarKeys", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = std::time::Duration::new(0, 0);

            for _ in 0..iters {
                let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
                    HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());

                let start = Instant::now();

                for (i, key) in similar_keys.iter().enumerate() {
                    map.insert(key.clone(), i as u32);
                }

                total_duration += start.elapsed();

                // Prevent the map from being optimized away
                black_box(&map);
            }

            total_duration
        });
    });

    // 3. HashMap with MurmurHash3-32
    group.bench_function("MurmurHash3-32-HashMap-SimilarKeys", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = std::time::Duration::new(0, 0);

            for _ in 0..iters {
                let mut map: HashMap<String, u32, MurmurHash3BuildHasher> =
                    HashMap::with_hasher(MurmurHash3BuildHasher);

                let start = Instant::now();

                for (i, key) in similar_keys.iter().enumerate() {
                    map.insert(key.clone(), i as u32);
                }

                total_duration += start.elapsed();

                // Prevent the map from being optimized away
                black_box(&map);
            }

            total_duration
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_hashmap_with_different_hashers,
    bench_collision_resistance
);
criterion_main!(benches);
