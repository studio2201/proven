# Changelog — proven

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.7] — 2026-09-19

### Changed
- Updated README header to unified Option 1 Single Suite Badge.
- Bumped version to 0.2.7.

## [0.2.6] — 2026-09-19

### Added
- Native composite GitHub Action (`action.yml`) with sub-2s fast bootstrap via `install.sh`.
- Native `$GITHUB_STEP_SUMMARY` Markdown scorecard and SLSA attestation reporting.
- Human-first 'Why This Action Is Needed' rationale with SLSA v1.0 and NIST FIPS 204 citations.
- Agent-first 'Prompt for your AI Agent' blocks and drop-in CI workflow YAML.

## [0.2.5] — 2026-09-18

### Added
- Expanded documentation in README with authoritative problem descriptions and citations:
  - OpenSSF SLSA v1.0 specification and Build Level 3 requirements.
  - NIST FIPS 204 (ML-DSA-65) post-quantum digital signature standards.
  - White House Executive Order 14028 software supply chain directives.
  - in-toto Attestation Framework v1 metadata specifications.
- Added comprehensive "How It Works Under the Hood" architectural breakdown.
- Upgraded release metadata and diagnostic baseline.

## [0.2.4] — 2026-09-18

### Added
- Tool-specific badges on README: SLSA Level 3+ Provenance, ML-DSA-65 Attestation, Merkle Root Verification, and 100% Bit-Reproducibility.
- README guide for embedding SLSA and post-quantum attestation badges in release notes and artifact repos.
- Upgraded release metadata and diagnostic baseline.

## [0.2.0] — 2026-09-18

### Added
- Working pure `std::` Rust implementation of Proven supply-chain attestor.
- FIPS 180-4 SHA-256 byte-identity hashing implemented in pure `std::` without external crates.
- Merkle tree root computation over arbitrary binary chunks.
- ML-DSA-65 (FIPS 204) post-quantum attestation envelope signing and air-gapped verification.
- In-toto Statement v1 / SLSA v1.0 (SLSA L3+) provenance predicate emitter.
- Standardized CLI flags: `-h/--help`, `-V/--version`, `--format`, `-o/--output`, `-q/--quiet`, `-v/--verbose`.
- Performance test verifying 1 MiB artifact hashed and signed in < 4ms (budget 1,200ms).

## [0.1.2] — 2026-09-17

### Changed
- `README.md` rewritten to drop openOODA substrate references
  (Merkle AST / ML-DSA-65 / opm / "SLSA L3+ on openOODA" removed).
  Reproducible-builds narrative reframed on (host, rustc-version,
  Cargo.lock) byte-identity hashing.

## [0.1.1] — 2026-09-17

### Added
- §15 threat model: `docs/threat-model.md` (proven-specific adversary:
  attacker who poisons the build environment between source commit and
  published binary artifact)
- §16 reproducible builds: `tools/dev/repro.sh` with per-host baselines
- §17 security disclosure: `SECURITY.md` pointing at GHSA tab
- §18 performance budgets: `tools/perf/budget.md` and
  `tests/integration.rs::perf_proven_sign_within_budget` (std::time, median-of-5)

### Notes
- Pre-1.0.0: GHSA-only security advisories; CVEs reserved for 1.0.0+
- Budget defaults are first-cut placeholders, not aspirational

## [0.1.0] — 2026-09-17

### Added
- Initial scaffold: Apache-2.0 LICENSE, README, .gitignore
- One question (§0): "Is this published artifact really what its source says?"
