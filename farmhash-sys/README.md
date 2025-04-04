# farmhash-sys

Rust FFI bindings for a minimal implementation of Google's FarmHash hashing algorithms.

## Overview

This crate provides low-level Rust bindings to a minimal implementation of Google's FarmHash algorithm. FarmHash is a family of hash functions designed for fast hashing of strings and other data.

The implementation is simplified and optimized for simplicity and ease of use, while still providing the core FarmHash functions:

- `hash32` - 32-bit hash
- `hash32_with_seed` - 32-bit hash with seed
- `hash64` - 64-bit hash
- `hash64_with_seed` - 64-bit hash with seed
- `hash64_with_seeds` - 64-bit hash with two seeds
- `fingerprint128` - 128-bit fingerprint
- `fingerprint128_with_seed` - 128-bit fingerprint with seed

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
farmhash-sys = "0.1.0"
```

Example:

```rust
use farmhash_sys::farmhash;

fn main() {
    let data = b"hello world";
    
    // 32-bit hash
    let hash32 = farmhash::hash32(data);
    println!("32-bit hash: {}", hash32);
    
    // 64-bit hash
    let hash64 = farmhash::hash64(data);
    println!("64-bit hash: {}", hash64);
    
    // 128-bit fingerprint
    let (low, high) = farmhash::fingerprint128(data);
    println!("128-bit fingerprint: ({}, {})", low, high);
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

This is a simplified implementation based on Google's [FarmHash](https://github.com/google/farmhash) algorithm.