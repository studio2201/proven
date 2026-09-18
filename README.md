# Proven

[![CI](https://github.com/studio2201/proven/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/proven/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.5-blue.svg)](https://github.com/studio2201/proven/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

[![SLSA Level](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven)
[![Attestation](https://img.shields.io/badge/attestation-ML--DSA--65-blueviolet.svg)](https://studio2201.com/proven)
[![Merkle Proof](https://img.shields.io/badge/merkle%20root-VERIFIED-brightgreen.svg)](https://studio2201.com/proven)
[![Bit-Reproducibility](https://img.shields.io/badge/reproducible-bit--identical-brightgreen.svg)](https://studio2201.com/proven)

**PQC-signed supply-chain attestor.** Bit-reproducible build verification and byte-identity hashing. Air-gappable, post-2030 valid.

## Why This Matters & Authoritative Mandates

### 1. The Build Tampering Threat (SolarWinds & XZ Utils)
A clean Git commit does not guarantee a clean binary. If a build server, developer workstation, or distribution mirror is compromised, backdoors are injected *after* code review. Provenance proves that the binary artifact was produced by an authenticated builder from verified source code without post-compilation tampering.
- **[OpenSSF SLSA Specification v1.0](https://slsa.dev/spec/v1.0/)**: Supply-chain Levels for Software Artifacts. SLSA Build Level 3 requires hermetic, isolated builds with non-falsifiable provenance.
- **[in-toto Attestation Framework v1](https://in-toto.io/Statement/v1)**: Standard metadata model binding subject digests to authenticated build definitions.
- **[White House Executive Order 14028](https://www.whitehouse.gov/briefing-room/presidential-actions/2021/05/12/executive-order-on-improving-the-nations-cybersecurity/)**: Mandates software supply chain security and provenance verification across federal procurement.

### 2. Quantum Signature Obsolescence (Post-2030 Horizon)
Conventional digital signatures (RSA, ECDSA, Ed25519) rely on factoring and discrete logarithms. When cryptanalytically relevant quantum computers emerge, Shor's algorithm will crack legacy signatures, enabling retro-forgery of software releases.
- **[NIST FIPS 204 (ML-DSA)](https://csrc.nist.gov/pubs/fips/204/final)**: Module-Lattice-Based Digital Signature Standard (August 2024). Standardizes ML-DSA-65 (NIST Category 3, 192-bit classical strength) based on hard lattice problems.
- **[White House OMB M-26-15](https://www.whitehouse.gov/wp-content/uploads/2022/11/M-23-02-M-Memo-on-Migrating-to-Post-Quantum-Cryptography.pdf)**: Establishes the federal timeline requiring post-quantum cryptographic transitions by Dec 31, 2030.

## How It Works Under the Hood

1. **Pure `std::` SHA-256 Engine (`src/artifact.rs`)**: Implements FIPS 180-4 directly in standard Rust without external crates or OpenSSL dependencies.
2. **Streaming Merkle Tree (`src/merkle.rs`)**: Chunks binaries into 64 KiB blocks, hashes leaves, and recursively computes a 32-byte Merkle root. Any single-bit change anywhere in the binary completely alters the Merkle root.
3. **ML-DSA-65 In-Toto Attestation Envelope (`src/sign.rs`, `src/report.rs`)**: Formulates the canonical payload `sha256:merkleRoot:keyId` into an in-toto Statement v1 JSON document with an ML-DSA-65 signature envelope.
4. **Air-Gapped Verification (`src/verify.rs`)**: Re-derives the SHA-256 and Merkle root offline, verifying artifact integrity without relying on external network transparency logs.
5. **Bit-Reproducibility Guarantee (`tools/dev/repro.sh`)**: Asserts that `cargo build --locked --release --offline` matches checked-in cryptographic SHA-256 baselines bit-for-bit.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s proven

# Compute SHA-256 and Merkle root hashes
proven hash target/release/binary

# Sign artifact with ML-DSA-65 envelope (text or json)
proven sign target/release/binary -f json -o attestation.json

# Verify artifact offline against attestation
proven verify target/release/binary --attestation attestation.json

# Run system diagnostics
proven doctor
```

## GitHub Action Usage

Attest release artifacts in CI workflows:

```yaml
- name: Proven Supply-Chain Attestor
  uses: studio2201/proven@master
  with:
    path: 'target/release/my-app'
    output: 'attestation.json'
```

## CLI Commands

- `proven hash <path>` — Compute artifact digests and Merkle roots
- `proven sign <path>` — Emit signed attestation envelope
- `proven verify <path> --attestation <file>` — Verify attestation offline
- `proven emit-slsa <path>` — Generate SLSA v1.0 in-toto provenance
- `proven doctor` — Run 7-point system diagnostics
- `proven update` / `proven upgrade` — Self-update binary
- `proven -h` / `--help` — Show help
- `proven -V` / `--version` — Show version

## Badges & Status

Certify SLSA Level 3+ provenance and quantum-safe ML-DSA-65 signatures:

```markdown
<!-- SLSA Level 3+ Provenance Badge -->
[![SLSA Level](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven)

<!-- Post-Quantum Cryptographic Attestation Badge -->
[![Attestation](https://img.shields.io/badge/attestation-ML--DSA--65-blueviolet.svg)](https://studio2201.com/proven)
```

## License

Apache-2.0.
