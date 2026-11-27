//! verify.rs — Air-gapped verification of SHA-256, Merkle root, and HMAC-SHA256 signature.
use crate::artifact::{hex_encode, Artifact};
use crate::merkle::compute_artifact_merkle;
use crate::sign::{canonical_payload, ct_eq, hmac_sha256, Attestation, SigningKey, ALGO_NAME};

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub verified: bool,
    pub checks_passed: Vec<String>,
    pub failures: Vec<String>,
}

pub fn verify_attestation(artifact: &Artifact, attestation: &Attestation) -> VerificationResult {
    let key = SigningKey::from_env(&attestation.key_id);
    verify_attestation_with_key(artifact, attestation, &key)
}

pub fn verify_attestation_with_key(
    artifact: &Artifact,
    attestation: &Attestation,
    key: &SigningKey,
) -> VerificationResult {
    let mut checks = Vec::new();
    let mut failures = Vec::new();

    if artifact.sha256 == attestation.artifact_sha256 {
        checks.push(format!("Artifact SHA-256 matched: {}", artifact.sha256));
    } else {
        failures.push(format!(
            "SHA-256 mismatch! Artifact={}, Attestation={}",
            artifact.sha256, attestation.artifact_sha256
        ));
    }

    let derived_merkle = compute_artifact_merkle(&artifact.bytes, 65536);
    if derived_merkle == attestation.merkle_root {
        checks.push(format!("Merkle tree root verified: {}", derived_merkle));
    } else {
        failures.push(format!(
            "Merkle root mismatch! Re-derived={}, Attestation={}",
            derived_merkle, attestation.merkle_root
        ));
    }

    if attestation.algorithm == ALGO_NAME {
        checks.push(format!("Algorithm valid: {}", ALGO_NAME));
    } else {
        failures.push(format!("Unsupported algorithm: {}", attestation.algorithm));
    }

    let payload = canonical_payload(
        &attestation.artifact_sha256,
        &attestation.merkle_root,
        &attestation.key_id,
    );
    let expected_sig = hex_encode(&hmac_sha256(&key.raw_seed, &payload));
    if attestation.signature_hex.len() == 64
        && ct_eq(
            expected_sig.as_bytes(),
            attestation.signature_hex.as_bytes(),
        )
    {
        checks.push("HMAC-SHA256 attestation signature verified".to_string());
    } else {
        failures.push("Signature verification failed (invalid or tampered sig)".to_string());
    }

    VerificationResult {
        verified: failures.is_empty(),
        checks_passed: checks,
        failures,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::sign_artifact;

    #[test]
    fn test_verify_roundtrip_and_tamper_rejection() {
        let art = Artifact::from_bytes("app.bin", b"release-binary-payload");
        let key = SigningKey::from_secret("ci-key", b"super-secret");
        let att = sign_artifact(&art, &key).unwrap();
        assert!(verify_attestation_with_key(&art, &att, &key).verified);

        let mut bad_sig = att.clone();
        bad_sig.signature_hex = "0".repeat(64);
        assert!(!verify_attestation_with_key(&art, &bad_sig, &key).verified);

        let wrong_key = SigningKey::from_secret("ci-key", b"wrong-secret");
        assert!(!verify_attestation_with_key(&art, &att, &wrong_key).verified);

        let tampered_art = Artifact::from_bytes("app.bin", b"evil-payload");
        assert!(!verify_attestation_with_key(&tampered_art, &att, &key).verified);
    }
}
