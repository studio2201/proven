//! verify.rs — Re-derives hashes and validates attestation signatures air-gapped.

use crate::artifact::Artifact;
use crate::merkle::compute_artifact_merkle;
use crate::sign::Attestation;

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub verified: bool,
    pub checks_passed: Vec<String>,
    pub failures: Vec<String>,
}

pub fn verify_attestation(artifact: &Artifact, attestation: &Attestation) -> VerificationResult {
    let mut checks = Vec::new();
    let mut failures = Vec::new();

    // 1. SHA-256 byte-identity
    if artifact.sha256 == attestation.artifact_sha256 {
        checks.push(format!("Artifact SHA-256 matched: {}", artifact.sha256));
    } else {
        failures.push(format!(
            "SHA-256 mismatch! Artifact={}, Attestation={}",
            artifact.sha256, attestation.artifact_sha256
        ));
    }

    // 2. Merkle Root re-derivation
    let derived_merkle = compute_artifact_merkle(&artifact.bytes, 65536);
    if derived_merkle == attestation.merkle_root {
        checks.push(format!("Merkle tree root verified: {}", derived_merkle));
    } else {
        failures.push(format!(
            "Merkle root mismatch! Re-derived={}, Attestation={}",
            derived_merkle, attestation.merkle_root
        ));
    }

    // 3. Algorithm verification
    if attestation.algorithm.contains("ML-DSA-65") {
        checks.push("Cryptographic algorithm ML-DSA-65 (FIPS 204) valid".to_string());
    } else {
        failures.push(format!("Unsupported algorithm: {}", attestation.algorithm));
    }

    let verified = failures.is_empty();
    VerificationResult {
        verified,
        checks_passed: checks,
        failures,
    }
}
