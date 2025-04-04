# cityhash-sys

Rust FFI bindings for Google's CityHash hashing algorithms.

## Overview

This crate provides low-level Rust bindings to Google's CityHash algorithm. CityHash is a family of hash functions designed for fast hashing of strings and other data.

The implementation provides the core CityHash functions:

- `city_hash_32` - 32-bit hash
- `city_hash_64` - 64-bit hash
- `city_hash_64_with_seed` - 64-bit hash with seed
- `city_hash_64_with_seeds` - 64-bit hash with two seeds

A Hasher implementation (`CityHashHasher`) is also provided for easy integration with Rust's standard library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cityhash-sys = "0.1.0"
```

Example:

```rust
use cityhash_sys::{city_hash_32, city_hash_64};
// Or use the module interface
use cityhash_sys::cityhash;

fn main() {
    let data = b"hello world";
    
    // 32-bit hash
    let hash32 = city_hash_32(data);
    println!("32-bit hash: {}", hash32);
    
    // 64-bit hash
    let hash64 = city_hash_64(data);
    println!("64-bit hash: {}", hash64);
    
    // Using the module interface
    let hash32_alt = cityhash::hash32(data);
    let hash64_alt = cityhash::hash64(data);
    
    assert_eq!(hash32, hash32_alt);
    assert_eq!(hash64, hash64_alt);
}
```

Using the hasher:

```rust
use std::hash::{Hash, Hasher};
use cityhash_sys::CityHashHasher;

fn main() {
    let mut hasher = CityHashHasher::new();
    "hello world".hash(&mut hasher);
    let hash = hasher.finish();
    
    println!("Hash: {}", hash);
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

This is an implementation based on Google's [CityHash](https://github.com/google/cityhash) algorithm.