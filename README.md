# SimpleHash

A simple, fast Rust library implementing common non-cryptographic hash functions.

## Features

* FNV-1 and FNV-1a (32-bit and 64-bit variants)
* MurmurHash3 (32-bit and 128-bit variants)
* Pure Rust implementation with no external dependencies
* Fast and efficient processing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simplehash = "0.1.0"
```

## Usage

### Library Usage

```rust
use simplehash::{fnv1_32, fnv1a_32, fnv1_64, fnv1a_64, murmurhash3_32, murmurhash3_128};

// Calculate FNV hashes
let input = "hello world".as_bytes();
let fnv1_32_result = fnv1_32(input);
let fnv1a_32_result = fnv1a_32(input);
let fnv1_64_result = fnv1_64(input);
let fnv1a_64_result = fnv1a_64(input);

// Calculate MurmurHash3 with a seed value
let seed = 0;
let murmur3_32_result = murmurhash3_32(input, seed);
let murmur3_128_result = murmurhash3_128(input, seed);
```

### Command Line Usage

This package also includes a command-line utility:

```bash
$ simplehash "hello world"
Input string: "hello world"
Input bytes:  [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]

FNV1-32:       0x8a718a8d (2322709133)
FNV1a-32:      0x22f4f271 (587238001)
FNV1-64:       0x8e59dd02f97b79c3 (10246997719728264643)
FNV1a-64:      0x779a65e7023cd2e7 (8681568671143534311)
MurmurHash3-32: 0x2e4ff723 (775550755)
MurmurHash3-128: 0x6e08d7bd92574023aeb46101661a9c3d

Computed all hashes in 37.742µs
```

## Why Use SimpleHash?

- **Simplicity**: Easy-to-use API with clear function names.
- **No dependencies**: Pure Rust implementation with no external dependencies.
- **Fast and efficient**: Optimized for performance.
- **Multiple hash variants**: Choose the hash function best suited for your use case.

## Non-Cryptographic Hashing

Please note that these are non-cryptographic hash functions, appropriate for:

- Hash tables and dictionaries
- Checksums
- Bloom filters
- Data partitioning
- Caches

They are NOT suitable for:
- Password hashing
- Digital signatures
- Any security/cryptographic purpose

## Testing and Validation

The library includes tests to validate hash outputs against reference implementations. The MurmurHash3 implementation is validated against the Python `mmh3` library.

## License

This project is licensed under the MIT License - see the LICENSE file for details.