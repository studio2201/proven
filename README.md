# Proven

[![CI](https://github.com/studio2201/proven/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/proven/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.3-blue.svg)](https://github.com/studio2201/proven/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

**PQC-signed supply-chain attestor.** Bit-reproducible build verification and byte-identity hashing. Air-gappable, post-2030 valid.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s proven

# Compute SHA-256 and Merkle root hashes
proven hash target/release/binary

# Sign artifact with ML-DSA-65 envelope
proven sign target/release/binary

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

## What it does

- Computes single-byte and streaming Merkle root hashes in pure `std::`.
- Emits cryptographic attestations with ML-DSA-65 envelope schema.
- Emits SLSA v1.0 Level 3+ provenance predicates.

## CLI Commands

- `proven hash <path>` — Compute artifact digests and Merkle roots
- `proven sign <path>` — Emit signed attestation envelope
- `proven verify <path>` — Verify attestation against signature
- `proven emit-slsa <path>` — Generate SLSA v1.0 provenance
- `proven doctor` — Run 7-point system diagnostics
- `proven update` / `proven upgrade` — Self-update binary
- `proven -h` / `--help` — Show help
- `proven -V` / `--version` — Show version

## Why

- Post-2030 supply-chain security requires quantum-resistant build attestations.
- Pure Rust, `std::` only. Zero crates.io dependencies. Strictly <= 256 LOC per source file.

## License

Apache-2.0.
