//! main.rs — Proven CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, -f/--format, -o/--output, -q/--quiet, -v/--verbose.

mod cli;
mod doctor;
mod hash;
mod update;
mod xdg;

use cli::{parse_args, print_help, CliConfig, CliError, OutputFormat, Subcommand, VERSION};
use proven::{
    emit_slsa_json, emit_text_attestation, emit_verification_text, load_artifact, sign,
    verify_attestation, Attestation, SigningKey,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target {
        fs::write(path, content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o644));
        }
        Ok(())
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn extract_str_value(line: &str, key: &str) -> Option<String> {
    if let Some(pos) = line.find(key) {
        let rest = &line[pos + key.len()..];
        let start = rest.find('"')? + 1;
        let end = rest[start..].find('"')? + start;
        Some(rest[start..end].to_string())
    } else {
        None
    }
}

fn parse_attestation_json(content: &str) -> Result<Attestation, String> {
    let mut name = "artifact".to_string();
    let mut sha256 = String::new();
    let mut merkle = String::new();
    let mut algo = "ML-DSA-65 (FIPS 204)".to_string();
    let mut key_id = "key".to_string();
    let mut sig = String::new();

    for line in content.lines() {
        if let Some(v) = extract_str_value(line, "\"name\":") { name = v; }
        if let Some(v) = extract_str_value(line, "\"sha256\":") { sha256 = v; }
        if let Some(v) = extract_str_value(line, "\"merkleRoot\":") { merkle = v; }
        if let Some(v) = extract_str_value(line, "\"merkle_root\":") { merkle = v; }
        if let Some(v) = extract_str_value(line, "\"algorithm\":") { algo = v; }
        if let Some(v) = extract_str_value(line, "\"keyid\":") { key_id = v; }
        if let Some(v) = extract_str_value(line, "\"key_id\":") { key_id = v; }
        if let Some(v) = extract_str_value(line, "\"sig\":") { sig = v; }
    }

    if sha256.is_empty() {
        return Err("Malformed attestation: missing sha256".into());
    }
    Ok(Attestation {
        artifact_name: name,
        artifact_sha256: sha256,
        merkle_root: merkle,
        algorithm: algo,
        key_id,
        signature_hex: sig,
        slsa_level: "SLSA L3+".into(),
    })
}

fn run_app(config: &CliConfig) -> Result<i32, CliError> {
    let target_path = config
        .target_file
        .as_ref()
        .ok_or_else(|| CliError::Runtime("No target artifact file specified. See --help.".into()))?;

    if config.subcommand == Subcommand::Hash {
        let out = hash::run_hash(target_path, config.format)?;
        write_output(&out, config.output_file.as_ref())?;
        return Ok(0);
    }

    let artifact = load_artifact(target_path)
        .map_err(|e| format!("Failed to read {}: {}", target_path.display(), e))?;

    if config.verbose {
        eprintln!(
            "proven: loaded {} ({} bytes, sha256={})",
            artifact.name, artifact.size, artifact.sha256
        );
    }

    if config.subcommand == Subcommand::Verify {
        let att_path = config
            .attestation_file
            .as_ref()
            .ok_or_else(|| CliError::Runtime("Verification requires --attestation <FILE>".into()))?;
        let raw = fs::read_to_string(att_path)
            .map_err(|e| format!("Failed to read {}: {}", att_path.display(), e))?;
        let attestation = parse_attestation_json(&raw)?;
        let res = verify_attestation(&artifact, &attestation);

        if !config.quiet {
            println!("{}", emit_verification_text(&res));
        }
        return Ok(if res.verified { 0 } else { 1 });
    }

    let key = SigningKey::generate(&config.key_id);
    let attestation = sign(&artifact, &key).map_err(|e| format!("Attestation failed: {}", e))?;

    let output = match config.format {
        OutputFormat::Json | OutputFormat::Slsa => emit_slsa_json(&attestation),
        OutputFormat::Text => emit_text_attestation(&attestation),
    };

    write_output(&output, config.output_file.as_ref())?;
    Ok(0)
}

fn run() -> Result<i32, CliError> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    match config.subcommand {
        Subcommand::Help => {
            print_help();
            Ok(0)
        }
        Subcommand::Version => {
            println!("proven {}", VERSION);
            Ok(0)
        }
        Subcommand::Doctor => {
            let (code, output) = doctor::run_doctor("proven", VERSION, config.format);
            if !config.quiet || config.output_file.is_some() {
                write_output(&output, config.output_file.as_ref())?;
            }
            Ok(code)
        }
        Subcommand::Update => update::run_update("proven", VERSION).map_err(CliError::Runtime),
        _ => run_app(&config),
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(CliError::Parse(err)) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
        Err(CliError::Runtime(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
