use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use simplehash::{fnv1_32, fnv1_64, fnv1a_32, fnv1a_64};

fn bench_fnv_functions(c: &mut Criterion) {
    // Create a group for FNV benchmarks
    let mut group = c.benchmark_group("FNV Hash Functions");

    // Input sizes to test
    let sizes = [8, 16, 32, 64, 128, 256, 512, 1024, 4096];

    for size in sizes {
        // Create test data of specified size
        let data = vec![0xAA; size];

        // Benchmark FNV1-32
        group.bench_with_input(BenchmarkId::new("FNV1-32", size), &data, |b, data| {
            b.iter(|| fnv1_32(black_box(data)));
        });

        // Benchmark FNV1a-32
        group.bench_with_input(BenchmarkId::new("FNV1a-32", size), &data, |b, data| {
            b.iter(|| fnv1a_32(black_box(data)));
        });

        // Benchmark FNV1-64
        group.bench_with_input(BenchmarkId::new("FNV1-64", size), &data, |b, data| {
            b.iter(|| fnv1_64(black_box(data)));
        });

        // Benchmark FNV1a-64
        group.bench_with_input(BenchmarkId::new("FNV1a-64", size), &data, |b, data| {
            b.iter(|| fnv1a_64(black_box(data)));
        });
    }

    group.finish();
}

// Benchmark with realistic input data
fn bench_realistic_inputs(c: &mut Criterion) {
    let mut group = c.benchmark_group("FNV Realistic Inputs");

    // Sample inputs that might be common in real use cases
    let inputs = [
        "hello world",                                 // Short string
        "https://example.com/path/to/resource",        // URL
        "The quick brown fox jumps over the lazy dog", // Medium string
        "user123@example.com",                         // Email
        // UUID
        "550e8400-e29b-41d4-a716-446655440000",
    ];

    for input in inputs {
        let data = input.as_bytes();
        let input_name = if input.len() > 20 {
            format!("{}...({}B)", &input[0..20], input.len())
        } else {
            input.to_string()
        };

        // Benchmark all FNV functions
        group.bench_with_input(
            BenchmarkId::new("FNV1-32", &input_name),
            &data,
            |b, data| {
                b.iter(|| fnv1_32(black_box(data)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("FNV1a-32", &input_name),
            &data,
            |b, data| {
                b.iter(|| fnv1a_32(black_box(data)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("FNV1-64", &input_name),
            &data,
            |b, data| {
                b.iter(|| fnv1_64(black_box(data)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("FNV1a-64", &input_name),
            &data,
            |b, data| {
                b.iter(|| fnv1a_64(black_box(data)));
            },
        );
    }

    group.finish();
}

// Benchmark for comparison between FNV variants
fn bench_fnv_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("FNV Comparison");

    // Input sizes to test
    let sizes = [16, 64, 256, 1024, 4096];

    for size in sizes {
        // Create test data of specified size
        let data = vec![0xAA; size];

        // Create a throughput metric (bytes processed per iteration)
        group.throughput(criterion::Throughput::Bytes(size as u64));

        // Benchmark all FNV variants
        group.bench_function(BenchmarkId::new("FNV1-32", size), |b| {
            b.iter(|| fnv1_32(black_box(&data)));
        });

        group.bench_function(BenchmarkId::new("FNV1a-32", size), |b| {
            b.iter(|| fnv1a_32(black_box(&data)));
        });

        group.bench_function(BenchmarkId::new("FNV1-64", size), |b| {
            b.iter(|| fnv1_64(black_box(&data)));
        });

        group.bench_function(BenchmarkId::new("FNV1a-64", size), |b| {
            b.iter(|| fnv1a_64(black_box(&data)));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_fnv_functions,
    bench_realistic_inputs,
    bench_fnv_comparison
);
criterion_main!(benches);
