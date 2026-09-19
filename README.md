# Proven

[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)
[![Release](https://img.shields.io/badge/version-v0.2.9-blue.svg)](https://github.com/studio2201/proven/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

<details>
<summary><a href="https://studio2201.com/agents#badges"><img src="https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield" alt="studio2201 Suite"></a> <b>Detailed Governance Scorecard</b></summary>

| Tool | Focus | Verdict | Status Badge |
| :--- | :--- | :---: | :---: |
| [**Snip**](https://studio2201.com/snip) | Vibe-Code & Secrets Gate | `SHIP` | [![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip) |
| [**Vigil**](https://studio2201.com/vigil) | Supply-Chain Dormancy | `HEALTHY` | [![Dormancy](https://img.shields.io/badge/dormancy-healthy-2f6f5e.svg)](https://studio2201.com/vigil) |
| [**Aegis**](https://studio2201.com/aegis) | PQC & Post-Quantum Scans | `QUANTUM-SAFE` | [![PQC](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis) |
| [**Proven**](https://studio2201.com/proven) | ML-DSA-65 Attestation | `VERIFIED` | [![SLSA](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven) |
| [**Boneyard**](https://studio2201.com/boneyard) | Tech-Debt Radar | `0/100 DEBT` | [![Boneyard](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard) |

</details>

**PQC-signed supply-chain attestor.** Bit-reproducible build verification and byte-identity hashing. Air-gappable, post-2030 valid.

## Why This Action Is Needed

### Build Tampering & Post-Quantum Provenance
A clean Git commit does not guarantee a clean binary. If a CI builder, package repository, or distribution mirror is compromised, backdoors are injected post-compilation (e.g. SolarWinds, XZ Utils CVE-2024-3094). Release integrity requires immutable, cryptographically verifiable provenance binding the binary directly to its source.
- **[White House Executive Order 14028](https://www.whitehouse.gov/briefing-room/presidential-actions/2021/05/12/executive-order-on-improving-the-nations-cybersecurity/)**: Mandates non-falsifiable software provenance and supply-chain attestations for enterprise software procurement.
- **[OpenSSF SLSA Specification v1.0](https://slsa.dev/spec/v1.0/)**: SLSA Build Level 3+ requires hermetic, reproducible builds with authenticated in-toto attestations.
- **[NIST FIPS 204 (ML-DSA)](https://csrc.nist.gov/pubs/fips/204/final)**: Standardizes post-quantum lattice-based digital signatures, ensuring release attestations remain unforgeable past 2030.
- **Automated CI Gates vs Manual Verification**: Manual verification of checksums and signatures is easily skipped during urgent releases. Proven automates binary hashing, Merkle tree construction, and ML-DSA-65 attestation generation directly within CI pipelines.

## Autonomous Agent Integration

Deploy Proven into your release and CI pipelines using your AI coding assistant or copy the workflow below.

### Prompt for your AI Agent

Copy and paste this prompt to Cursor, Claude Code, Copilot Workspace, or Devin:

```text
Add a GitHub Actions workflow to this repository at .github/workflows/proven.yml using studio2201/proven@master. Trigger on release tags and pull requests, verify bit-reproducibility of compiled artifacts, generate SLSA Level 3+ build provenance, and sign binaries with post-quantum ML-DSA-65 attestations.
```

### GitHub Actions Workflow

Commit this complete, production-ready workflow at `.github/workflows/proven.yml`:

```yaml
name: Proven Supply-Chain Attestor
on:
  push:
    tags: [ 'v*' ]
  pull_request:
    branches: [ master, main ]
permissions:
  contents: read
jobs:
  proven-attest:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Attest Artifact Provenance
        uses: studio2201/proven@master
        with:
          path: '.'
```

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
