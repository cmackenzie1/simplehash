pub fn farm_hash64(key: &[u8]) -> u64 {
    farmhash_sys::farmhash::hash64(key)
}
