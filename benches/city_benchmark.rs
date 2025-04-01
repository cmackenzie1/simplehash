use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use simplehash::{city_hash_64, fnv1a_64, murmurhash3_64};

fn bench_city_hash(c: &mut Criterion) {
    let mut group = c.benchmark_group("city_hash_comparison");

    // Generate test data of different sizes
    let sizes = [4, 8, 16, 32, 64, 128, 256, 512, 1024, 4096];

    let mut rng = StdRng::seed_from_u64(42);

    for size in &sizes {
        // Create random data
        let data: Vec<u8> = (0..*size).map(|_| rng.r#gen::<u8>()).collect();

        // Create string-like data (mostly ASCII)
        let string_data: Vec<u8> = (0..*size)
            .map(|_| {
                let chr = rng.gen_range(32..127) as u8; // Printable ASCII
                chr
            })
            .collect();

        // Benchmark CityHash
        group.bench_with_input(
            BenchmarkId::new("city_hash_64_random", size),
            &data,
            |b, data| b.iter(|| city_hash_64(black_box(data))),
        );

        // Benchmark CityHash with string-like data
        group.bench_with_input(
            BenchmarkId::new("city_hash_64_string", size),
            &string_data,
            |b, data| b.iter(|| city_hash_64(black_box(data))),
        );

        // Benchmark FNV-1a for comparison
        group.bench_with_input(
            BenchmarkId::new("fnv1a_64_random", size),
            &data,
            |b, data| b.iter(|| fnv1a_64(black_box(data))),
        );

        // Benchmark FNV-1a with string-like data
        group.bench_with_input(
            BenchmarkId::new("fnv1a_64_string", size),
            &string_data,
            |b, data| b.iter(|| fnv1a_64(black_box(data))),
        );

        // Benchmark MurmurHash3 for comparison
        group.bench_with_input(
            BenchmarkId::new("murmurhash3_64_random", size),
            &data,
            |b, data| b.iter(|| murmurhash3_64(black_box(data), 0)),
        );

        // Benchmark MurmurHash3 with string-like data
        group.bench_with_input(
            BenchmarkId::new("murmurhash3_64_string", size),
            &string_data,
            |b, data| b.iter(|| murmurhash3_64(black_box(data), 0)),
        );
    }

    group.finish();
}

fn bench_city_hasher(c: &mut Criterion) {
    use simplehash::city::CityHasher64;
    use std::hash::Hasher;

    let mut group = c.benchmark_group("cityhash_std_hasher_trait");

    // Generate test data of different sizes
    let sizes = [4, 8, 16, 32, 64, 128, 256, 512, 1024];

    let mut rng = StdRng::seed_from_u64(42);

    for size in &sizes {
        // Create string-like data (mostly ASCII)
        let string_data: Vec<u8> = (0..*size)
            .map(|_| {
                let chr = rng.gen_range(32..127) as u8; // Printable ASCII
                chr
            })
            .collect();

        // Benchmark using the standard Hasher trait interface
        group.bench_with_input(
            BenchmarkId::new("city_hasher_string", size),
            &string_data,
            |b, data| {
                b.iter(|| {
                    let mut hasher = CityHasher64::new();
                    hasher.write(black_box(data));
                    hasher.finish()
                })
            },
        );

        // Benchmark using the direct function for comparison
        group.bench_with_input(
            BenchmarkId::new("city_hash_64_string", size),
            &string_data,
            |b, data| b.iter(|| city_hash_64(black_box(data))),
        );
    }

    group.finish();
}

// Benchmarks specifically for common string key patterns seen in hash tables
fn bench_string_key_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_key_patterns");

    // Define common string key patterns
    let keys = [
        "key",
        "user_id",
        "product_1234",
        "https://example.com/path/to/resource",
        "email@example.com",
        "f47ac10b-58cc-4372-a567-0e02b2c3d479", // UUID
        "{\"id\":1234,\"name\":\"example\",\"active\":true}", // JSON
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor.", // Long text
    ];

    for key in &keys {
        let data = key.as_bytes();

        // CityHash
        group.bench_with_input(BenchmarkId::new("city_hash_64", key), &data, |b, data| {
            b.iter(|| city_hash_64(black_box(data)))
        });

        // FNV-1a
        group.bench_with_input(BenchmarkId::new("fnv1a_64", key), &data, |b, data| {
            b.iter(|| fnv1a_64(black_box(data)))
        });

        // MurmurHash3
        group.bench_with_input(BenchmarkId::new("murmurhash3_64", key), &data, |b, data| {
            b.iter(|| murmurhash3_64(black_box(data), 0))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_city_hash,
    bench_city_hasher,
    bench_string_key_patterns
);
criterion_main!(benches);
