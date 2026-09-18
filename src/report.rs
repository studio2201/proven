//! report.rs — Emits SLSA L3+ statements, JSON envelopes, and terminal verification summaries.

use crate::sign::Attestation;
use crate::verify::VerificationResult;

pub fn emit_slsa_json(attestation: &Attestation) -> String {
    format!(
        "{{\n\
          \"_type\": \"https://in-toto.io/Statement/v1\",\n\
          \"subject\": [\n\
            {{\"name\": \"{}\", \"digest\": {{\"sha256\": \"{}\"}}}}\n\
          ],\n\
          \"predicateType\": \"https://slsa.dev/provenance/v1\",\n\
          \"predicate\": {{\n\
            \"buildDefinition\": {{\n\
              \"buildType\": \"https://studio2201.com/proven/v1\",\n\
              \"externalParameters\": {{\"merkleRoot\": \"{}\"}}\n\
            }},\n\
            \"runDetails\": {{\n\
              \"builder\": {{\"id\": \"studio2201/proven\"}},\n\
              \"metadata\": {{\"slsaLevel\": \"{}\"}}\n\
            }}\n\
          }},\n\
          \"signature\": {{\n\
            \"algorithm\": \"{}\",\n\
            \"keyid\": \"{}\",\n\
            \"sig\": \"{}\"\n\
          }}\n\
        }}\n",
        attestation.artifact_name, attestation.artifact_sha256,
        attestation.merkle_root, attestation.slsa_level,
        attestation.algorithm, attestation.key_id, attestation.signature_hex
    )
}

pub fn emit_text_attestation(attestation: &Attestation) -> String {
    format!(
        "proven: attested artifact '{}'\n\
         SHA-256:     {}\n\
         Merkle Root: {}\n\
         Algorithm:   {}\n\
         Level:       {}\n\
         Key ID:      {}\n",
        attestation.artifact_name, attestation.artifact_sha256,
        attestation.merkle_root, attestation.algorithm,
        attestation.slsa_level, attestation.key_id
    )
}

pub fn emit_verification_text(res: &VerificationResult) -> String {
    let mut out = String::new();
    if res.verified {
        out.push_str("proven verification: PASSED (bit-reproducible & PQC signature valid)\n");
        for c in &res.checks_passed {
            out.push_str(&format!("  ✓ {}\n", c));
        }
    } else {
        out.push_str("proven verification: FAILED\n");
        for f in &res.failures {
            out.push_str(&format!("  ✗ {}\n", f));
        }
    }
    out
}
