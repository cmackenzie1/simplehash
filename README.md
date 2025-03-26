# SimpleHash

[![CI](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml/badge.svg)](https://github.com/cmackenzie1/simplehash/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/simplehash)](https://crates.io/crates/simplehash)
[![docs.rs](https://img.shields.io/docsrs/simplehash)](https://docs.rs/simplehash/latest/simplehash/)


A simple Rust implementation of common non-cryptographic hash functions.

## Currently Implemented

- FNV-1 (32-bit and 64-bit)
- FNV-1a (32-bit and 64-bit)
- MurmurHash3 (32-bit and 128-bit)

## Usage

```rust
use simplehash::{fnv1_32, fnv1a_32, fnv1_64, fnv1a_64, murmurhash3_32, murmurhash3_128};

fn main() {
    let input = "hello world";
    let bytes = input.as_bytes();
    
    let fnv1_32_hash = fnv1_32(bytes);
    let fnv1a_32_hash = fnv1a_32(bytes);
    let fnv1_64_hash = fnv1_64(bytes);
    let fnv1a_64_hash = fnv1a_64(bytes);
    let murmur3_32_hash = murmurhash3_32(bytes, 0);
    let murmur3_128_hash = murmurhash3_128(bytes, 0);
    
    println!("FNV1-32: 0x{:x}", fnv1_32_hash);
    println!("FNV1a-32: 0x{:x}", fnv1a_32_hash);
    println!("FNV1-64: 0x{:x}", fnv1_64_hash);
    println!("FNV1a-64: 0x{:x}", fnv1a_64_hash);
    println!("MurmurHash3-32: 0x{:x}", murmur3_32_hash);
    println!("MurmurHash3-128: 0x{:x}", murmur3_128_hash);
}
```

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

## License

MIT
