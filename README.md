# SimpleHash

[![CI](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml/badge.svg)](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/simplehash)](https://crates.io/crates/simplehash)
[![docs.rs](https://img.shields.io/docsrs/simplehash)](https://docs.rs/simplehash/latest/simplehash/)

A simple Rust implementation of common non-cryptographic hash functions and consistent hashing algorithms, compatible with Rust's standard collections.

## Supported Hash Algorithms

- **FNV Hash Family**
  - FNV-1 (32-bit and 64-bit)
  - FNV-1a (32-bit and 64-bit)
- **MurmurHash3**
  - 32-bit implementation
  - 64-bit implementation
  - 128-bit implementation
- **CityHash**
  - 64-bit implementation
- **Rendezvous Hashing**
  - Consistent distribution algorithm (HRW - Highest Random Weight)
  - Works with any hasher implementing `std::hash::Hasher`

These hash functions (except for MurmurHash3 128-bit) implement the `std::hash::Hasher` trait, making them usable with `HashMap` and `HashSet` as faster alternatives to the default SipHash.

## Installation

```bash
# Add to your Cargo.toml
cargo add simplehash
```

## Usage

### Basic Hash Functions

```rust
use simplehash::{fnv1_32, fnv1a_32, fnv1_64, fnv1a_64, murmurhash3_32, murmurhash3_128, city_hash_64};

fn main() {
    let input = "hello world";
    let bytes = input.as_bytes();

    let fnv1_32_hash = fnv1_32(bytes);
    let fnv1a_32_hash = fnv1a_32(bytes);
    let fnv1_64_hash = fnv1_64(bytes);
    let fnv1a_64_hash = fnv1a_64(bytes);
    let murmur3_32_hash = murmurhash3_32(bytes, 0);
    let murmur3_128_hash = murmurhash3_128(bytes, 0);
    let city_hash = city_hash_64(bytes);

    println!("FNV1-32: 0x{:x}", fnv1_32_hash);
    println!("FNV1a-32: 0x{:x}", fnv1a_32_hash);
    println!("FNV1-64: 0x{:x}", fnv1_64_hash);
    println!("FNV1a-64: 0x{:x}", fnv1a_64_hash);
    println!("MurmurHash3-32: 0x{:x}", murmur3_32_hash);
    println!("MurmurHash3-128: 0x{:x}", murmur3_128_hash);
    println!("CityHash-64: 0x{:x}", city_hash);
}
```

### Using with HashMap and HashSet

You can use these hashers with Rust's standard collections for better performance:

```rust
use simplehash::fnv::Fnv1aHasher64;
use simplehash::murmur::MurmurHasher32;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, BuildHasherDefault};

// Using FNV-1a with HashMap
let mut map: HashMap<String, u32, BuildHasherDefault<Fnv1aHasher64>> =
    HashMap::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());
map.insert("key".to_string(), 42);

// Using FNV-1a with HashSet
let mut set: HashSet<String, BuildHasherDefault<Fnv1aHasher64>> =
    HashSet::with_hasher(BuildHasherDefault::<Fnv1aHasher64>::default());
set.insert("value".to_string());

// For MurmurHash3, create a BuildHasher implementation
#[derive(Default, Clone)]
struct MurmurHash3BuildHasher;

impl BuildHasher for MurmurHash3BuildHasher {
    type Hasher = MurmurHasher32;

    fn build_hasher(&self) -> Self::Hasher {
        MurmurHasher32::new(0) // Using seed 0
    }
}

// Using MurmurHash3 with HashMap
let mut murmur_map: HashMap<String, u32, MurmurHash3BuildHasher> =
    HashMap::with_hasher(MurmurHash3BuildHasher);
murmur_map.insert("key".to_string(), 42);
```

### Rendezvous Hashing for Consistent Distribution

Rendezvous hashing (also known as Highest Random Weight or HRW hashing) provides a way to consistently distribute data across a changing set of servers or nodes, with minimal redistribution when nodes are added or removed.

```rust
use simplehash::rendezvous::RendezvousHasher;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasherDefault;
use simplehash::fnv::Fnv1aHasher64;

// Create a RendezvousHasher with the standard hasher
let std_hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());

// Create a RendezvousHasher with FNV-1a 64-bit hasher (for better performance)
let fnv_hasher = RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
    BuildHasherDefault::<Fnv1aHasher64>::default()
);

// Define a list of servers or nodes
let nodes = vec!["server1", "server2", "server3", "server4", "server5"];

// Find the preferred node for a key
let key = "user_profile_12345";
let selected_node = fnv_hasher.select(&key, &nodes).unwrap();
println!("Key '{}' is assigned to node: {}", key, selected_node);

// Get all nodes ranked by preference for this key
let ranked_nodes = fnv_hasher.rank(&key, &nodes);
println!("Nodes ranked by preference for key '{}':", key);
for (i, node) in ranked_nodes.iter().enumerate() {
    println!("  {}. {}", i+1, node);
}

// When a node is removed, only keys assigned to that node are redistributed
let reduced_nodes = vec!["server1", "server2", "server4", "server5"]; // server3 removed
let new_node = fnv_hasher.select(&key, &reduced_nodes).unwrap();
```

## Algorithm Selection Guide

Each hash function has specific strengths:

- **FNV Hash Family**: Fast for small inputs (short strings, integers). Excellent for hash tables with small keys.
- **MurmurHash3**: Better performance and distribution for medium to large inputs.
- **CityHash**: Designed specifically for string hashing by Google. Excellent performance for string keys in hash tables.
- **Rendezvous Hashing**: Ideal for distributing data across multiple nodes with minimal redistribution when the node set changes.

Performance comparisons:

- FNV-1a is generally 1.5-2x faster than SipHash (Rust default) for small keys
- MurmurHash3 shows better collision resistance and performance for larger keys
- CityHash performs exceptionally well for string keys, often outperforming other algorithms
- Rendezvous hashing with FNV-1a or MurmurHash3 provides excellent distribution properties

**Note**: These non-cryptographic hash functions should only be used for trusted data as they lack the DoS protection of SipHash.

## Command Line Usage

```bash
# Build the project
cargo build --release

# Run the CLI
./target/release/simplehash "hello world"
```

## Verification

This project includes verification scripts to ensure the hash implementations match reference implementations:

### FNV Verification

```bash
# Generate the FNV test corpus using Go
go run generate_fnv_corpus.go

# Verify FNV implementations against Go's reference implementation
cargo test test_against_go_fnv
```

### MurmurHash3 Verification

```bash
# Generate the MurmurHash3 test corpus
uv run generate_mmh3_corpus.py

# Run the verification tests
cargo test test_against_mmh3_python
```

## Benchmarks

SimpleHash includes benchmarks using the Criterion.rs library:

```bash
# Run all benchmarks
cargo bench

# Run only FNV benchmarks
cargo bench --bench fnv_benchmark

# Run comparative hash benchmarks
cargo bench --bench hash_benchmark

# Run HashMap/HashSet performance benchmarks
cargo bench --bench hashmap_benchmark

# Run Rendezvous hashing benchmarks
cargo bench --bench rendezvous_benchmark
```

The benchmarks compare performance across various input types, sizes, and hash algorithms.

## License

MIT
