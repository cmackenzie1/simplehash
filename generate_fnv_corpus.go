package main

import (
	"encoding/json"
	"fmt"
	"hash/fnv"
	"math/rand"
	"os"
	"strings"
)

// FNVTestVector represents a test vector for FNV hash function verification
type FNVTestVector struct {
	Input      string   `json:"input"`
	InputBytes []int    `json:"input_bytes"`
	FNV1_32    uint32   `json:"fnv1_32"`
	FNV1a_32   uint32   `json:"fnv1a_32"`
	FNV1_64    uint64   `json:"fnv1_64"`
	FNV1a_64   uint64   `json:"fnv1a_64"`
}

// generateTestStrings creates a variety of test strings for validation
func generateTestStrings(count int) []string {
	testStrings := []string{}

	// Empty string
	testStrings = append(testStrings, "")

	// Single characters
	for _, c := range "abcdefghijklmnopqrstuvwxyz0123456789" {
		testStrings = append(testStrings, string(c))
	}

	// Common test strings
	commonStrings := []string{
		"hello",
		"hello world",
		"Hello World",
		"aaaa",
		"0123456789",
		"abcdefghijklmnopqrstuvwxyz",
		"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		"!@#$%^&*()_+-=[]{}|;:,.<>?/",
	}
	testStrings = append(testStrings, commonStrings...)

	// String with all ASCII values
	var allASCII []byte
	for i := 0; i < 256; i++ {
		allASCII = append(allASCII, byte(i))
	}
	testStrings = append(testStrings, string(allASCII))

	// Random strings of various lengths
	chars := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?/ "
	for len(testStrings) < count {
		length := rand.Intn(100) + 1
		var sb strings.Builder
		for i := 0; i < length; i++ {
			sb.WriteByte(chars[rand.Intn(len(chars))])
		}
		testStrings = append(testStrings, sb.String())
	}

	return testStrings
}

// calculateFNVHashes computes FNV hash values for each input string
func calculateFNVHashes(testStrings []string) []FNVTestVector {
	vectors := []FNVTestVector{}

	for _, s := range testStrings {
		byteSlice := []byte(s)
		inputBytes := make([]int, len(byteSlice))
		for i, b := range byteSlice {
			inputBytes[i] = int(b)
		}

		// Calculate FNV1-32
		fnv1_32 := fnv.New32()
		fnv1_32.Write(byteSlice)
		fnv1_32_hash := fnv1_32.Sum32()

		// Calculate FNV1a-32
		fnv1a_32 := fnv.New32a()
		fnv1a_32.Write(byteSlice)
		fnv1a_32_hash := fnv1a_32.Sum32()

		// Calculate FNV1-64
		fnv1_64 := fnv.New64()
		fnv1_64.Write(byteSlice)
		fnv1_64_hash := fnv1_64.Sum64()

		// Calculate FNV1a-64
		fnv1a_64 := fnv.New64a()
		fnv1a_64.Write(byteSlice)
		fnv1a_64_hash := fnv1a_64.Sum64()

		vector := FNVTestVector{
			Input:      s,
			InputBytes: inputBytes,
			FNV1_32:    fnv1_32_hash,
			FNV1a_32:   fnv1a_32_hash,
			FNV1_64:    fnv1_64_hash,
			FNV1a_64:   fnv1a_64_hash,
		}

		vectors = append(vectors, vector)
	}

	return vectors
}

func main() {
	// Set deterministic random seed for reproducibility
	rand.Seed(42)

	// Generate test strings
	testStrings := generateTestStrings(200)

	// Calculate hashes
	vectors := calculateFNVHashes(testStrings)

	// Ensure data directory exists
	if _, err := os.Stat("data"); os.IsNotExist(err) {
		os.Mkdir("data", 0755)
	}

	// Write results to file
	file, err := os.Create("data/fnv_test_corpus.json")
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(vectors); err != nil {
		fmt.Println("Error encoding JSON:", err)
		return
	}

	fmt.Printf("Generated FNV test corpus with %d entries.\n", len(vectors))
}