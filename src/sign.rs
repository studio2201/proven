//! sign.rs — RFC 2104 HMAC-SHA256 attestation signing in pure std:: Rust.
use crate::artifact::{hex_encode, sha256_digest, Artifact};
use crate::merkle::compute_artifact_merkle;
use std::fmt;

pub const ALGO_NAME: &str = "HMAC-SHA256 (FIPS 198-1)";
pub const PROVENANCE_STANDARD: &str = "in-toto v1 (SHA-256 + Merkle)";

#[derive(Debug, Clone)]
pub struct SigningKey {
    pub key_id: String,
    pub raw_seed: [u8; 32],
}

impl SigningKey {
    pub fn generate(key_id: &str) -> Self {
        SigningKey {
            key_id: key_id.to_string(),
            raw_seed: sha256_digest(key_id.as_bytes()),
        }
    }

    pub fn from_secret(key_id: &str, secret: &[u8]) -> Self {
        SigningKey {
            key_id: key_id.to_string(),
            raw_seed: sha256_digest(secret),
        }
    }

    pub fn from_env(key_id: &str) -> Self {
        match std::env::var("PROVEN_SIGNING_KEY") {
            Ok(s) if !s.is_empty() => Self::from_secret(key_id, s.as_bytes()),
            _ => Self::generate(key_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attestation {
    pub artifact_name: String,
    pub artifact_sha256: String,
    pub merkle_root: String,
    pub algorithm: String,
    pub key_id: String,
    pub signature_hex: String,
    pub slsa_level: String,
}

#[derive(Debug)]
pub struct SignError(pub String);

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signing error: {}", self.0)
    }
}

impl std::error::Error for SignError {}

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut k0 = [0u8; 64];
    if key.len() > 64 {
        k0[..32].copy_from_slice(&sha256_digest(key));
    } else {
        k0[..key.len()].copy_from_slice(key);
    }
    let mut inner = Vec::with_capacity(64 + msg.len());
    let mut outer = Vec::with_capacity(96);
    for b in k0 {
        inner.push(b ^ 0x36);
        outer.push(b ^ 0x5c);
    }
    inner.extend_from_slice(msg);
    let inner_hash = sha256_digest(&inner);
    outer.extend_from_slice(&inner_hash);
    sha256_digest(&outer)
}

pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub fn canonical_payload(sha256: &str, merkle_root: &str, key_id: &str) -> Vec<u8> {
    format!("{}:{}:{}", sha256, merkle_root, key_id).into_bytes()
}

pub fn sign_artifact(artifact: &Artifact, key: &SigningKey) -> Result<Attestation, SignError> {
    let merkle_root = compute_artifact_merkle(&artifact.bytes, 65536);
    let payload = canonical_payload(&artifact.sha256, &merkle_root, &key.key_id);
    let sig = hmac_sha256(&key.raw_seed, &payload);

    Ok(Attestation {
        artifact_name: artifact.name.clone(),
        artifact_sha256: artifact.sha256.clone(),
        merkle_root,
        algorithm: ALGO_NAME.to_string(),
        key_id: key.key_id.clone(),
        signature_hex: hex_encode(&sig),
        slsa_level: PROVENANCE_STANDARD.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sha256_vector() {
        let mac = hmac_sha256(b"key", b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            hex_encode(&mac),
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        );
    }
}
