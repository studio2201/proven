//! hash.rs — Artifact SHA-256 and Merkle root calculation for proven.
use crate::cli::OutputFormat;
use proven::artifact::load_artifact;
use proven::merkle::compute_artifact_merkle;
use std::path::Path;

pub fn run_hash(path: &Path, fmt: OutputFormat) -> Result<String, String> {
    let artifact = load_artifact(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let merkle_root = compute_artifact_merkle(&artifact.bytes, 65536);

    match fmt {
        OutputFormat::Json => Ok(format!(
            "{{\"name\":\"{}\",\"sha256\":\"{}\",\"merkle_root\":\"{}\",\"size\":{}}}\n",
            artifact.name, artifact.sha256, merkle_root, artifact.size
        )),
        _ => Ok(format!("{}  {}\n", artifact.sha256, artifact.name)),
    }
}
