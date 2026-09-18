//! main.rs — Proven CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, --format, -o/--output, -q/--quiet, -v/--verbose.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use proven::{emit_slsa_json, emit_text_attestation, emit_verification_text, load_artifact, sign, verify_attestation, Attestation, SigningKey};

const VERSION: &str = "0.2.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat { Text, Json, Slsa }

#[derive(Debug)]
struct CliConfig {
    subcommand: String,
    target_file: Option<PathBuf>,
    attestation_file: Option<PathBuf>,
    key_id: String,
    format: OutputFormat,
    output_file: Option<PathBuf>,
    quiet: bool,
    verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: "attest".into(), target_file: None, attestation_file: None,
            key_id: "studio2201-pqc-identity".into(), format: OutputFormat::Text,
            output_file: None, quiet: false, verbose: false,
        }
    }
}

fn print_help() {
    println!(
        "proven {} — PQC-signed supply-chain attestor\n\
        USAGE:\n  proven [SUBCOMMAND] [OPTIONS] [FILE]\n\
        SUBCOMMANDS:\n\
          attest, sign      Attest binary artifact with ML-DSA-65 (default)\n\
          verify            Verify artifact against attestation envelope\n\
          emit-slsa         Emit SLSA v1.0 / SLSA L3+ provenance predicate\n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          --format <fmt>          Output format: text, json, slsa [default: text]\n\
          -o, --output <file>     Write output to file\n\
          --key <key-id>          Signing key identity [default: studio2201-pqc-identity]\n\
          --attestation <file>    Attestation file to verify against\n\
          -q, --quiet; -v, --verbose\n\
        EXAMPLES:\n\
          proven attest target/release/app\n\
          proven verify target/release/app --attestation app.slsa.json\n",
        VERSION
    );
}

fn parse_args(args: &[String]) -> Result<Option<CliConfig>, String> {
    let mut c = CliConfig::default();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => { print_help(); return Ok(None); }
            "-V" | "--version" => { println!("proven {}", VERSION); return Ok(None); }
            "-q" | "--quiet" => c.quiet = true,
            "-v" | "--verbose" => c.verbose = true,
            "--format" => {
                i += 1; if i >= args.len() { return Err("Missing format".into()); }
                c.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json, "slsa" => OutputFormat::Slsa, _ => OutputFormat::Text,
                };
            }
            "-o" | "--output" => {
                i += 1; if i >= args.len() { return Err("Missing output file".into()); }
                c.output_file = Some(PathBuf::from(&args[i]));
            }
            "--key" => {
                i += 1; if i >= args.len() { return Err("Missing key identity".into()); }
                c.key_id = args[i].clone();
            }
            "--attestation" => {
                i += 1; if i >= args.len() { return Err("Missing attestation file".into()); }
                c.attestation_file = Some(PathBuf::from(&args[i]));
            }
            "attest" | "sign" | "verify" | "emit-slsa" => c.subcommand = args[i].clone(),
            arg if !arg.starts_with('-') => c.target_file = Some(PathBuf::from(arg)),
            other => return Err(format!("Unknown option: {}", other)),
        }
        i += 1;
    }
    Ok(Some(c))
}

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target { fs::write(path, content) } else { print!("{}", content); Ok(()) }
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

    if sha256.is_empty() { return Err("Malformed attestation: missing sha256".into()); }
    Ok(Attestation {
        artifact_name: name, artifact_sha256: sha256, merkle_root: merkle,
        algorithm: algo, key_id, signature_hex: sig, slsa_level: "SLSA L3+".into(),
    })
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args)? { Some(c) => c, None => return Ok(0) };

    let target_path = config.target_file.as_ref()
        .ok_or_else(|| "No target artifact file specified. See --help.".to_string())?;
    let artifact = load_artifact(target_path).map_err(|e| format!("Failed to read {}: {}", target_path.display(), e))?;

    if config.verbose {
        eprintln!("proven: loaded {} ({} bytes, sha256={})", artifact.name, artifact.size, artifact.sha256);
    }

    if config.subcommand == "verify" {
        let att_path = config.attestation_file.as_ref()
            .ok_or_else(|| "Verification requires --attestation <FILE>".to_string())?;
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

    write_output(&output, config.output_file.as_ref()).map_err(|e| format!("Write failed: {}", e))?;
    Ok(0)
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(err) => { eprintln!("error: {}", err); process::exit(2); }
    }
}
