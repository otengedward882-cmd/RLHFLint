//! Fast hash-based prompt deduplication engine.

use std::collections::HashSet;

pub struct Deduplicator {
    seen_hashes: HashSet<u64>,
}

impl Deduplicator {
    pub fn new() -> Self {
        Self {
            seen_hashes: HashSet::new(),
        }
    }

    /// Computes 64-bit FNV-1a hash over normalized prompt text
    pub fn is_duplicate(&mut self, text: &str) -> bool {
        let normalized = text
            .trim()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        let hash = Self::fnv1a_hash(normalized.as_bytes());
        if self.seen_hashes.contains(&hash) {
            true
        } else {
            self.seen_hashes.insert(hash);
            false
        }
    }

    fn fnv1a_hash(bytes: &[u8]) -> u64 {
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;

        let mut hash = FNV_OFFSET_BASIS;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    }
}
