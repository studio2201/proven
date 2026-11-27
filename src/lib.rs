//! lib.rs — Proven: SHA-256, Merkle, and HMAC-SHA256 supply-chain attestor.
//! Pure std:: Rust (Edition 2021). Zero external dependencies.

pub mod artifact;
pub mod merkle;
pub mod report;
pub mod sign;
pub mod verify;

pub use artifact::{hex_encode, load_artifact, sha256_digest, Artifact, ArtifactError};
pub use merkle::{compute_artifact_merkle, compute_merkle_root};
pub use report::{emit_slsa_json, emit_text_attestation, emit_verification_text};
pub use sign::{sign_artifact, Attestation, SignError, SigningKey};
pub use verify::{verify_attestation, verify_attestation_with_key, VerificationResult};

/// Signs an artifact and produces an HMAC-SHA256 in-toto attestation envelope.
pub fn sign(artifact: &Artifact, key: &SigningKey) -> Result<Attestation, SignError> {
    sign_artifact(artifact, key)
}
