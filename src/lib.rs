//! lib.rs — Proven: PQC-signed supply-chain attestor.
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
pub use verify::{verify_attestation, VerificationResult};

/// Signs an artifact and produces an ML-DSA-65 post-quantum attestation envelope.
pub fn sign(artifact: &Artifact, key: &SigningKey) -> Result<Attestation, SignError> {
    sign_artifact(artifact, key)
}
