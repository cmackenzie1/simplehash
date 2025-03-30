use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use simplehash::{murmurhash3_32, murmurhash3_128};

// Benchmark MurmurHash3 with various input sizes
fn bench_murmur_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("MurmurHash3 Input Sizes");

    // Test different input sizes including small strings which are common in hash tables
    let sizes = [4, 8, 16, 32, 64, 128, 256, 512, 1024, 4096];

    for size in sizes {
        // Create test data of specified size
        let data = vec![0xAA; size];

        // Set throughput to measure performance in bytes per second
        group.throughput(criterion::Throughput::Bytes(size as u64));

        // Benchmark 32-bit version
        group.bench_function(BenchmarkId::new("MurmurHash3-32", size), |b| {
            b.iter(|| murmurhash3_32(black_box(&data), 0));
        });

        // Benchmark 128-bit version
        group.bench_function(BenchmarkId::new("MurmurHash3-128", size), |b| {
            b.iter(|| murmurhash3_128(black_box(&data), 0));
        });
    }

    group.finish();
}

// Benchmark MurmurHash3 with small inputs (critical for hash table performance)
fn bench_murmur_small_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("MurmurHash3 Small Keys");

    // Common hash table key sizes
    let keys = [
        // Empty string (edge case)
        "",         // Single character
        "a",        // Two characters (test tail processing)
        "ab",       // Four characters (one block)
        "abcd",     // Eight characters (two blocks for 32-bit)
        "abcdefgh", // Small string with varied content
        "key123",   // Common ID format
        "id:12345",
    ];

    for key in keys {
        let data = key.as_bytes();

        // Benchmark with both hash variants
        group.bench_function(BenchmarkId::new("MurmurHash3-32", key), |b| {
            b.iter(|| murmurhash3_32(black_box(data), 0));
        });

        group.bench_function(BenchmarkId::new("MurmurHash3-128", key), |b| {
            b.iter(|| murmurhash3_128(black_box(data), 0));
        });
    }

    group.finish();
}

// Benchmark MurmurHash3 with different seeds
fn bench_murmur_seeds(c: &mut Criterion) {
    let mut group = c.benchmark_group("MurmurHash3 Seeds");

    // Test data
    let data = "This is a test string for seed variation".as_bytes();

    // Different seed values
    let seeds = [0, 42, 0xdeadbeef, 0x12345678];

    for seed in seeds {
        // Benchmark with different seeds
        group.bench_function(BenchmarkId::new("MurmurHash3-32", seed), |b| {
            b.iter(|| murmurhash3_32(black_box(data), seed));
        });

        group.bench_function(BenchmarkId::new("MurmurHash3-128", seed), |b| {
            b.iter(|| murmurhash3_128(black_box(data), seed));
        });
    }

    group.finish();
}

// Benchmark MurmurHash3 with byte patterns that test tail processing
fn bench_murmur_tail_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("MurmurHash3 Tail Processing");

    // Create inputs of different lengths to test all tail processing branches
    for len in 1..17 {
        let data = vec![0xAA; len];

        // Benchmark with focus on tail processing
        group.bench_function(BenchmarkId::new("MurmurHash3-32", len), |b| {
            b.iter(|| murmurhash3_32(black_box(&data), 0));
        });

        if len <= 16 {
            // For 128-bit, tail processing only matters up to 16 bytes
            group.bench_function(BenchmarkId::new("MurmurHash3-128", len), |b| {
                b.iter(|| murmurhash3_128(black_box(&data), 0));
            });
        }
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_murmur_sizes,
    bench_murmur_small_keys,
    bench_murmur_seeds,
    bench_murmur_tail_processing
);
criterion_main!(benches);
