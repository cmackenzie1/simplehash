# SimpleHash

[![CI](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml/badge.svg)](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/simplehash)](https://crates.io/crates/simplehash)
[![docs.rs](https://img.shields.io/docsrs/simplehash)](https://docs.rs/simplehash/latest/simplehash/)


A simple Rust implementation of common non-cryptographic hash functions that are compatible with Rust's standard collections.

## Installation

This repository contains submodules. To clone the repository with all its submodules:

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/cmackenzie1/simplehash.git

# Or, if you've already cloned the repository without submodules:
git submodule init
git submodule update
```

## Currently Implemented

- FNV-1 (32-bit and 64-bit)
- FNV-1a (32-bit and 64-bit)
- MurmurHash3 (32-bit, 64-bit, and 128-bit)
- CityHash (64-bit)
- Rendezvous hashing (HRW - Highest Random Weight) algorithm

These hash functions (except for MurmurHash3 128-bit) implement the `std::hash::Hasher` trait, making them usable with `HashMap` and `HashSet` as faster alternatives to the default SipHash. The Rendezvous hashing implementation works with any hasher implementing the `std::hash::Hasher` trait.

## Usage

### Basic Usage

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

### Using Rendezvous Hashing

Rendezvous hashing (also known as Highest Random Weight or HRW hashing) provides a way to consistently distribute data across a changing set of servers or nodes. This implementation is particularly useful for distributed systems that need minimal redistribution when nodes are added or removed.

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

// The beauty of rendezvous hashing is that when a node is removed,
// only keys that were assigned to that specific node are redistributed
let reduced_nodes = vec!["server1", "server2", "server4", "server5"]; // server3 removed
let new_node = fnv_hasher.select(&key, &reduced_nodes).unwrap();
```

### When to Use Alternative Hashers

- **For performance-critical code**: When dealing with a large number of hash operations or collections with many elements
- **For small keys**: FNV performs exceptionally well with small keys, such as integers or short strings
- **For medium to large inputs**: MurmurHash3 offers better performance for larger inputs
- **For string keys**: CityHash was specifically designed by Google for string hashing and performs very well for string keys in hash tables (based on the [original implementation](https://github.com/google/cityhash) by Geoff Pike and Jyrki Alakuijala)
- **For internal/trusted data only**: These hash functions lack the DoS protection of SipHash (Rust's default)
- **For distributed systems**: Rendezvous hashing is ideal for distributing data across multiple servers or nodes with minimal redistribution when the node set changes

Based on benchmarks, these hashers can provide significant performance improvements:
- FNV-1a is generally 1.5-2x faster than SipHash for small keys
- MurmurHash3 shows better performance for larger keys and provides better collision resistance
- CityHash performs exceptionally well for string keys, often outperforming other algorithms for common string lengths
- Rendezvous hashing with FNV-1a or MurmurHash3 provides excellent distribution properties while maintaining consistency when nodes are added or removed

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

SimpleHash includes benchmarks using the Criterion.rs library. To run the benchmarks:

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

The benchmarks compare:
- FNV hashing implementations (FNV-1 and FNV-1a, 32-bit and 64-bit variants)
- MurmurHash3 implementations (32-bit, 64-bit, and 128-bit)
- Performance across various input sizes
- Different input patterns (zeros, ones, alternating, incremental)
- Realistic data inputs (strings, URLs, JSON, UUIDs)
- HashMap and HashSet performance with different hashers (SipHash vs FNV vs MurmurHash3)
- Collision resistance evaluation with similar keys
- Rendezvous hashing performance with different underlying hash functions and node counts

## License

MIT
