/// SimpleHash CLI
///
/// This is a command-line interface for the SimpleHash library, allowing quick calculation
/// of various non-cryptographic hash functions from the terminal.
use simplehash::fnv::Fnv1aHasher64;
use simplehash::rendezvous::RendezvousHasher;
use simplehash::{
    city_hash_64, fnv1_32, fnv1_64, fnv1a_32, fnv1a_64, murmurhash3_32, murmurhash3_128,
};
use std::env;
use std::hash::BuildHasherDefault;
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
    let city_hash_result = city_hash_64(bytes);

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
    println!(
        "CityHash-64:    0x{:016x} ({})",
        city_hash_result, city_hash_result
    );
    println!();
    println!("Computed all hashes in {:?}", elapsed);

    // Demonstrate rendezvous hashing
    println!("\n=== Rendezvous Hashing Example ===");

    // Create a hasher that uses FNV-1a 64-bit
    let hasher =
        RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
            Fnv1aHasher64,
        >::default());

    // Define some nodes (servers, cache instances, etc.)
    let nodes = vec![
        "server-us-east",
        "server-us-west",
        "server-eu-1",
        "server-ap-1",
        "server-sa-1",
    ];

    // Select the preferred node for the input key
    if let Some(selected_node) = hasher.select(&input, &nodes) {
        println!("Key '{}' is assigned to node: {}", input, selected_node);

        // Show full ranking
        println!("\nAll nodes ranked for key '{}':", input);
        let ranked = hasher.rank(&input, &nodes);
        for (i, node) in ranked.iter().enumerate() {
            println!("  {}. {}", i + 1, node);
        }

        // Show what happens when a node is removed
        let reduced_nodes = vec![
            "server-us-east",
            "server-us-west",
            "server-eu-1",
            "server-sa-1",
        ];
        let new_selected = hasher.select(&input, &reduced_nodes).unwrap();
        println!(
            "\nAfter removing 'server-ap-1', key is now assigned to: {}",
            new_selected
        );
    }
}
