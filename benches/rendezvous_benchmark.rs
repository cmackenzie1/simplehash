use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use simplehash::fnv::Fnv1aHasher64;
use simplehash::murmur::MurmurHasher64;
use simplehash::rendezvous::RendezvousHasher;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasherDefault;

fn benchmark_rendezvous_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendezvous_select");

    // Create hashers with different underlying hash functions
    let std_hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());
    let fnv_hasher =
        RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
            Fnv1aHasher64,
        >::default());

    // Create a murmur hasher build hasher
    #[derive(Default, Clone)]
    struct MurmurHasher64BuildHasher;

    impl std::hash::BuildHasher for MurmurHasher64BuildHasher {
        type Hasher = MurmurHasher64;

        fn build_hasher(&self) -> Self::Hasher {
            MurmurHasher64::new(0)
        }
    }

    let murmur_hasher =
        RendezvousHasher::<_, MurmurHasher64BuildHasher>::new(MurmurHasher64BuildHasher);

    // Test with various node counts
    let node_counts = [5, 10, 20, 50, 100];

    for &count in &node_counts {
        // Create nodes
        let nodes: Vec<String> = (0..count).map(|i| format!("node_{}", i)).collect();

        // Benchmark standard hasher
        group.bench_with_input(BenchmarkId::new("std_hasher", count), &count, |b, _| {
            b.iter(|| {
                for i in 0..100 {
                    let key = format!("key_{}", i);
                    black_box(std_hasher.select(&key, &nodes));
                }
            })
        });

        // Benchmark FNV hasher
        group.bench_with_input(BenchmarkId::new("fnv_hasher", count), &count, |b, _| {
            b.iter(|| {
                for i in 0..100 {
                    let key = format!("key_{}", i);
                    black_box(fnv_hasher.select(&key, &nodes));
                }
            })
        });

        // Benchmark MurmurHash3 hasher
        group.bench_with_input(BenchmarkId::new("murmur_hasher", count), &count, |b, _| {
            b.iter(|| {
                for i in 0..100 {
                    let key = format!("key_{}", i);
                    black_box(murmur_hasher.select(&key, &nodes));
                }
            })
        });
    }

    group.finish();
}

fn benchmark_rendezvous_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendezvous_rank");

    // Create hasher with FNV (typically fast for small inputs)
    let fnv_hasher =
        RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
            Fnv1aHasher64,
        >::default());

    // Test with various node counts
    let node_counts = [5, 10, 20, 50, 100];

    for &count in &node_counts {
        // Create nodes
        let nodes: Vec<String> = (0..count).map(|i| format!("node_{}", i)).collect();

        group.bench_with_input(BenchmarkId::new("rank", count), &count, |b, _| {
            b.iter(|| {
                for i in 0..10 {
                    let key = format!("key_{}", i);
                    black_box(fnv_hasher.rank(&key, &nodes));
                }
            })
        });
    }

    group.finish();
}

fn benchmark_node_distribution(c: &mut Criterion) {
    let mut group = c.benchmark_group("node_distribution");

    // Create hasher with FNV
    let fnv_hasher =
        RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
            Fnv1aHasher64,
        >::default());

    // Fixed number of nodes
    let node_count = 10;
    let nodes: Vec<String> = (0..node_count).map(|i| format!("node_{}", i)).collect();

    // Test with different key counts to assess distribution quality
    let key_counts = [100, 1000, 10000];

    for &count in &key_counts {
        group.bench_with_input(
            BenchmarkId::new("distribution", count),
            &count,
            |b, &key_count| {
                b.iter(|| {
                    let keys: Vec<String> = (0..key_count).map(|i| format!("key_{}", i)).collect();
                    let mut distribution = vec![0; node_count];

                    for key in &keys {
                        if let Some(idx) = fnv_hasher.select_index(key, &nodes) {
                            distribution[idx] += 1;
                        }
                    }

                    black_box(distribution)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_rendezvous_select,
    benchmark_rendezvous_rank,
    benchmark_node_distribution
);
criterion_main!(benches);
