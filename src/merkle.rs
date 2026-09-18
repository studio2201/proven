//! merkle.rs — Computes Merkle tree root hashes over source AST and chunks.
//! Pure std:: Rust.

use crate::artifact::{hex_encode, sha256_digest};

pub fn compute_merkle_root(chunks: &[&[u8]]) -> [u8; 32] {
    if chunks.is_empty() {
        return [0u8; 32];
    }

    let mut current_level: Vec<[u8; 32]> = chunks.iter().map(|c| sha256_digest(c)).collect();

    while current_level.len() > 1 {
        let mut next_level = Vec::with_capacity((current_level.len() + 1) / 2);
        for pair in current_level.chunks(2) {
            if pair.len() == 2 {
                let mut combined = [0u8; 64];
                combined[..32].copy_from_slice(&pair[0]);
                combined[32..].copy_from_slice(&pair[1]);
                next_level.push(sha256_digest(&combined));
            } else {
                next_level.push(pair[0]);
            }
        }
        current_level = next_level;
    }

    current_level[0]
}

pub fn compute_artifact_merkle(bytes: &[u8], chunk_size: usize) -> String {
    let sz = if chunk_size == 0 { 65536 } else { chunk_size };
    let mut slices = Vec::new();
    for c in bytes.chunks(sz) {
        slices.push(c);
    }
    let root = compute_merkle_root(&slices);
    hex_encode(&root)
}
