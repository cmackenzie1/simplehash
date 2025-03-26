.PHONY: all test clean corpus corpus-fnv corpus-mmh3

all: corpus test

# Generate test corpus for both hash functions
corpus: corpus-fnv corpus-mmh3

# Generate FNV hash test corpus
corpus-fnv:
	go run generate_fnv_corpus.go

# Generate MurmurHash3 test corpus
corpus-mmh3:
	uv run generate_mmh3_corpus.py

# Run Rust tests
test:
	cargo test

# Clean generated test corpus files
clean:
	rm -f data/fnv_test_corpus.json data/mmh3_test_corpus.json