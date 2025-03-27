use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use simplehash::{fnv1_32, fnv1a_32, fnv1_64, fnv1a_64, murmurhash3_32, murmurhash3_128};

fn bench_all_hash_functions(c: &mut Criterion) {
    // Create a benchmark group
    let mut group = c.benchmark_group("Hash Functions Comparison");
    
    // Test different input sizes
    let sizes = [16, 64, 256, 1024, 4096];
    
    for size in sizes {
        // Create test data of specified size
        let data = vec![0xAA; size];
        
        // Set throughput to measure performance in bytes per second
        group.throughput(criterion::Throughput::Bytes(size as u64));
        
        // Benchmark all hash functions
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
        
        group.bench_function(BenchmarkId::new("MurmurHash3-32", size), |b| {
            b.iter(|| murmurhash3_32(black_box(&data), 0));
        });
        
        group.bench_function(BenchmarkId::new("MurmurHash3-128", size), |b| {
            b.iter(|| murmurhash3_128(black_box(&data), 0));
        });
    }
    
    group.finish();
}

// Benchmark with different input patterns
fn bench_input_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hash Function Input Patterns");
    
    // Create different input patterns of the same size
    let size = 1024;
    
    // Pattern 1: All zeros
    let zeros = vec![0x00; size];
    
    // Pattern 2: All ones
    let ones = vec![0xFF; size];
    
    // Pattern 3: Alternating
    let mut alternating = Vec::with_capacity(size);
    for i in 0..size {
        alternating.push(if i % 2 == 0 { 0x55 } else { 0xAA });
    }
    
    // Pattern 4: Incremental
    let mut incremental = Vec::with_capacity(size);
    for i in 0..size {
        incremental.push((i % 256) as u8);
    }
    
    // Test different patterns for each hash function
    for (name, data) in [
        ("zeros", &zeros),
        ("ones", &ones),
        ("alternating", &alternating),
        ("incremental", &incremental),
    ] {
        group.bench_function(BenchmarkId::new("FNV1a-32", name), |b| {
            b.iter(|| fnv1a_32(black_box(data)));
        });
        
        group.bench_function(BenchmarkId::new("FNV1a-64", name), |b| {
            b.iter(|| fnv1a_64(black_box(data)));
        });
        
        group.bench_function(BenchmarkId::new("MurmurHash3-32", name), |b| {
            b.iter(|| murmurhash3_32(black_box(data), 0));
        });
        
        group.bench_function(BenchmarkId::new("MurmurHash3-128", name), |b| {
            b.iter(|| murmurhash3_128(black_box(data), 0));
        });
    }
    
    group.finish();
}

// Benchmark with realistic data types
fn bench_realistic_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("Realistic Data");
    
    // Sample real-world inputs
    let inputs = [
        // Short string
        "hello world",
        
        // URL
        "https://example.com/path/to/resource?param1=value1&param2=value2",
        
        // JSON
        r#"{"id":123,"name":"John Doe","email":"john@example.com","active":true}"#,
        
        // UUID
        "550e8400-e29b-41d4-a716-446655440000",
    ];
    
    for input in inputs {
        let data = input.as_bytes();
        let input_name = if input.len() > 15 {
            format!("{}...({}B)", &input[0..15], input.len())
        } else {
            input.to_string()
        };
        
        // Set throughput
        group.throughput(criterion::Throughput::Bytes(data.len() as u64));
        
        // Test fastest hash functions for realistic data
        group.bench_function(BenchmarkId::new("FNV1a-32", &input_name), |b| {
            b.iter(|| fnv1a_32(black_box(data)));
        });
        
        group.bench_function(BenchmarkId::new("FNV1a-64", &input_name), |b| {
            b.iter(|| fnv1a_64(black_box(data)));
        });
        
        group.bench_function(BenchmarkId::new("MurmurHash3-32", &input_name), |b| {
            b.iter(|| murmurhash3_32(black_box(data), 0));
        });
    }
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_all_hash_functions, 
    bench_input_patterns,
    bench_realistic_data
);
criterion_main!(benches);