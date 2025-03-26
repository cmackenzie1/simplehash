/// SimpleHash CLI
///
/// This is a command-line interface for the SimpleHash library, allowing quick calculation
/// of various non-cryptographic hash functions from the terminal.
use simplehash::{fnv1_32, fnv1_64, fnv1a_32, fnv1a_64, murmurhash3_32, murmurhash3_128};
use std::env;
use std::time::Instant;

/// Main entry point for the CLI application.
///
/// Calculates and displays multiple hash values for the provided input string.
/// If no input string is provided, displays usage information.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <string_to_hash>", args[0]);
        return;
    }

    let input = &args[1];
    let bytes = input.as_bytes();
    let now = Instant::now();

    // Perform all hash calculations
    let fnv1_32_result = fnv1_32(bytes);
    let fnv1a_32_result = fnv1a_32(bytes);
    let fnv1_64_result = fnv1_64(bytes);
    let fnv1a_64_result = fnv1a_64(bytes);
    let murmur3_32_result = murmurhash3_32(bytes, 0);
    let murmur3_128_result = murmurhash3_128(bytes, 0);

    let elapsed = now.elapsed();

    // Display results
    println!("Input string: \"{}\"", input);
    println!("Input bytes:  {:?}", bytes);
    println!();
    println!(
        "FNV1-32:       0x{:08x} ({})",
        fnv1_32_result, fnv1_32_result
    );
    println!(
        "FNV1a-32:      0x{:08x} ({})",
        fnv1a_32_result, fnv1a_32_result
    );
    println!(
        "FNV1-64:       0x{:016x} ({})",
        fnv1_64_result, fnv1_64_result
    );
    println!(
        "FNV1a-64:      0x{:016x} ({})",
        fnv1a_64_result, fnv1a_64_result
    );
    println!(
        "MurmurHash3-32: 0x{:08x} ({})",
        murmur3_32_result, murmur3_32_result
    );
    println!("MurmurHash3-128: 0x{:032x}", murmur3_128_result);
    println!();
    println!("Computed all hashes in {:?}", elapsed);
}
