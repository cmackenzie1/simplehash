# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "mmh3",
# ]
# ///

import json
import random
import string
import mmh3

# Generate a variety of test strings for validation
def generate_test_strings(count=100):
    test_strings = []
    
    # Empty string
    test_strings.append("")
    
    # Single characters
    for c in "abcdefghijklmnopqrstuvwxyz0123456789":
        test_strings.append(c)
    
    # Common test strings
    test_strings.extend([
        "hello",
        "hello world",
        "Hello World",
        "aaaa",
        "0123456789",
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "!@#$%^&*()_+-=[]{}|;:,.<>?/",
    ])
    
    # Random strings of various lengths
    for _ in range(count - len(test_strings)):
        length = random.randint(1, 100)
        chars = string.ascii_letters + string.digits + string.punctuation + " "
        random_str = ''.join(random.choice(chars) for _ in range(length))
        test_strings.append(random_str)
    
    return test_strings

# Calculate hashes using mmh3 Python library
def calculate_mmh3_hashes(test_strings):
    results = []
    
    for s in test_strings:
        string_bytes = s.encode('utf-8')
        
        # Calculate MurmurHash3 (32-bit) with different seeds
        seed_0_32 = mmh3.hash(s, 0)
        seed_42_32 = mmh3.hash(s, 42)
        
        # Calculate MurmurHash3 (128-bit) with different seeds
        # mmh3.hash128 returns a signed 128-bit value
        seed_0_128 = mmh3.hash128(s, 0, signed=False)
        seed_42_128 = mmh3.hash128(s, 42, signed=False)
        
        results.append({
            "input": s,
            "input_bytes": [b for b in string_bytes],
            "murmur3_32_seed0": seed_0_32 & 0xFFFFFFFF,  # Convert to unsigned 32-bit
            "murmur3_32_seed42": seed_42_32 & 0xFFFFFFFF,
            "murmur3_128_seed0": seed_0_128,
            "murmur3_128_seed42": seed_42_128
        })
    
    return results

def main():
    # Generate test strings
    test_strings = generate_test_strings(200)
    
    # Calculate hashes
    results = calculate_mmh3_hashes(test_strings)
    
    # Write results to file
    with open("data/test_corpus.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print(f"Generated test corpus with {len(results)} entries.")

if __name__ == "__main__":
    main()
