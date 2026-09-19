//! cli.rs — Standard CLI arguments and subcommand parser for proven.
use std::fmt;
use std::path::PathBuf;

pub const VERSION: &str = "0.2.10";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Slsa,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Subcommand {
    Hash,
    Sign,
    Verify,
    EmitSlsa,
    Doctor,
    Update,
    Help,
    Version,
}

#[derive(Debug)]
pub struct CliConfig {
    pub subcommand: Subcommand,
    pub target_file: Option<PathBuf>,
    pub attestation_file: Option<PathBuf>,
    pub key_id: String,
    pub format: OutputFormat,
    pub output_file: Option<PathBuf>,
    pub quiet: bool,
    pub verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: Subcommand::Sign,
            target_file: None,
            attestation_file: None,
            key_id: "studio2201-pqc-identity".into(),
            format: OutputFormat::Text,
            output_file: None,
            quiet: false,
            verbose: false,
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    Parse(String),
    Runtime(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Parse(s) | CliError::Runtime(s) => write!(f, "{}", s),
        }
    }
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::Runtime(s)
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::Runtime(s.to_string())
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Runtime(e.to_string())
    }
}

pub fn print_help() {
    println!(
        "proven {} — PQC-signed supply-chain attestor\n\
        \n\
        USAGE:\n\
          proven [SUBCOMMAND] [OPTIONS] [FILE]\n\
        \n\
        SUBCOMMANDS:\n\
          hash             Compute SHA-256 and Merkle root of artifact\n\
          sign, attest     Attest binary artifact with ML-DSA-65 (default)\n\
          verify           Verify artifact against attestation envelope\n\
          emit-slsa        Emit SLSA v1.0 / SLSA L3+ provenance predicate\n\
          doctor           Inspect system health and environment\n\
          update, upgrade  Update binary to latest release\n\
          help             Print help information\n\
          version          Print version information\n\
        \n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          -f, --format <fmt>      Output format: text, json, slsa [default: text]\n\
          -o, --output <file>     Write output to file\n\
          --key <key-id>          Signing key identity [default: studio2201-pqc-identity]\n\
          --attestation <file>    Attestation file to verify against\n\
          -q, --quiet             Quiet mode; status code only\n\
          -v, --verbose           Verbose diagnostic logging\n\
        \n\
        EXAMPLES:\n\
          proven hash target/release/app\n\
          proven sign target/release/app\n\
          proven verify target/release/app --attestation app.slsa.json\n\
          proven doctor\n",
        VERSION
    );
}

pub fn parse_args(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" | "help" => {
                config.subcommand = Subcommand::Help;
                return Ok(config);
            }
            "-V" | "--version" | "version" => {
                config.subcommand = Subcommand::Version;
                return Ok(config);
            }
            "-q" | "--quiet" => config.quiet = true,
            "-v" | "--verbose" => config.verbose = true,
            "-f" | "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for format option".into()));
                }
                config.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json,
                    "slsa" => OutputFormat::Slsa,
                    "text" => OutputFormat::Text,
                    other => return Err(CliError::Parse(format!("Unknown format: {}", other))),
                };
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for output option".into()));
                }
                config.output_file = Some(PathBuf::from(&args[i]));
            }
            "--key" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for --key".into()));
                }
                config.key_id = args[i].clone();
            }
            "--attestation" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for --attestation".into()));
                }
                config.attestation_file = Some(PathBuf::from(&args[i]));
            }
            "hash" => config.subcommand = Subcommand::Hash,
            "sign" | "attest" => config.subcommand = Subcommand::Sign,
            "verify" => config.subcommand = Subcommand::Verify,
            "emit-slsa" => config.subcommand = Subcommand::EmitSlsa,
            "doctor" => config.subcommand = Subcommand::Doctor,
            "update" | "upgrade" => config.subcommand = Subcommand::Update,
            arg if !arg.starts_with('-') => config.target_file = Some(PathBuf::from(arg)),
            other => return Err(CliError::Parse(format!("Unknown option: {}", other))),
        }
        i += 1;
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flags() {
        let args = vec!["proven".into(), "-q".into(), "-f".into(), "json".into()];
        let cfg = parse_args(&args).unwrap();
        assert!(cfg.quiet);
        assert_eq!(cfg.format, OutputFormat::Json);
    }

    #[test]
    fn test_subcommands() {
        let args = vec!["proven".into(), "hash".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Hash);
        let args = vec!["proven".into(), "sign".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Sign);
        let args = vec!["proven".into(), "attest".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Sign);
        let args = vec!["proven".into(), "doctor".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Doctor);
    }

    #[test]
    fn test_parse_errors() {
        let args = vec!["proven".into(), "--invalid".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
        let args = vec!["proven".into(), "-f".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
    }
}
