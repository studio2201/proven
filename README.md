# Proven

[![secured by studio2201](https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield)](https://studio2201.com)
[![proven: ML-DSA-65 verified](https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e)](https://studio2201.com/proven)
[![proven][b-proven]][ci-proven]
[![Release](https://img.shields.io/badge/version-v0.2.9-blue.svg)](https://github.com/studio2201/proven/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

[b-proven]: https://img.shields.io/github/actions/workflow/status/studio2201/proven/proven.yml?label=proven&logo=shield
[ci-proven]: https://github.com/studio2201/proven/actions/workflows/proven.yml

<details>
<summary><b>Executive Assurance Scorecard</b> (EO 14028 · NIST SP 800-218 · OMB M-26-15)</summary>

| Tool | Mandate / Standard | Assurance Metric | Status |
| :--- | :--- | :--- | :---: |
| [**Snip**][u-snip] | EO 14028 §4 (Credential Defense) | `0 secrets` | [![snip][m-snip]][u-snip] |
| [**Vigil**][u-vigil] | NIST SP 800-218 (Supply Surface) | `0 dependencies` | [![vigil][m-vigil]][u-vigil] |
| [**Aegis**][u-aegis] | OMB M-26-15 (Post-Quantum Crypto) | `PQC compliant` | [![aegis][m-aegis]][u-aegis] |
| [**Proven**][u-proven] | NIST FIPS 204 (SLSA Level 3+) | `ML-DSA-65 verified` | [![proven][m-proven]][u-proven] |
| [**Boneyard**][u-boneyard] | Asset Ownership (Zombie Defense) | `maintained` | [![boneyard][m-boneyard]][u-boneyard] |

[u-snip]: https://studio2201.com/snip
[u-vigil]: https://studio2201.com/vigil
[u-aegis]: https://studio2201.com/aegis
[u-proven]: https://studio2201.com/proven
[u-boneyard]: https://studio2201.com/boneyard
[m-snip]: https://img.shields.io/badge/snip-0%20secrets-2f6f5e
[m-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e
[m-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e
[m-proven]: https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e
[m-boneyard]: https://img.shields.io/badge/boneyard-maintained-2f6f5e

</details>

**PQC-signed supply-chain attestor.** Computes bit-identity SHA-256 and Merkle roots, signs in-toto
provenance with NIST FIPS 204 ML-DSA-65, and asserts SLSA Level 3+ build reproducibility.

## Why This Action Is Needed

### The Tampered Build Vector & Provenance Mandates
A clean Git commit does not guarantee a clean binary. If a CI builder, package repository, or
distribution mirror is compromised, backdoors are injected post-compilation (e.g. SolarWinds,
XZ Utils CVE-2024-3094). Release integrity requires immutable, cryptographically verifiable
provenance binding the binary directly to its source.
- **[White House Executive Order 14028](https://www.whitehouse.gov/briefing-room/presidential-actions/2021/05/12/executive-order-on-improving-the-nations-cybersecurity/)**:
  Mandates non-falsifiable software provenance and supply-chain attestations for enterprise software procurement.
- **[OpenSSF SLSA Specification v1.0](https://slsa.dev/spec/v1.0/)**:
  SLSA Build Level 3+ requires hermetic, reproducible builds with authenticated in-toto attestations.
- **[NIST FIPS 204 (ML-DSA)](https://csrc.nist.gov/pubs/fips/204/final)**:
  Standardizes post-quantum lattice-based digital signatures, ensuring attestations remain unforgeable past 2030.
- **Automated CI Gates vs Manual Verification**: Manual verification of checksums and signatures is easily skipped
  during urgent releases. Proven automates binary hashing, Merkle tree construction, and ML-DSA-65 attestation.

## Autonomous Agent Integration

Deploy Proven into your release and CI pipelines using your AI coding assistant or copy the workflow below.

### Prompt for your AI Agent

Copy and paste this prompt to Cursor, Claude Code, Copilot Workspace, or Devin:

```text
Add a GitHub Actions workflow to this repository at .github/workflows/proven.yml using studio2201/studio2201@v1:
- Name: proven
- Triggers: pull_request, push to default branch (master/main), and workflow_dispatch.
- Concurrency: group ${{ github.workflow }}-${{ github.ref }} with cancel-in-progress on pull_request.
- Permissions: contents: read, checkout with fetch-depth: 0.
- Action: run studio2201/studio2201@v1 with tools: 'proven' and fail-on: 'block'.
- Artifacts: upload proven_report.md using actions/upload-artifact@v4 with if: always().
- README Badge: add dynamic Shields.io workflow status badge to top of README.md (replace <owner>/<repo>):
  [![proven][b-proven]][ci-proven]
  [b-proven]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/proven.yml?label=proven&logo=shield
  [ci-proven]: https://github.com/<owner>/<repo>/actions/workflows/proven.yml
```

### GitHub Actions Workflow

Commit this complete, production-ready workflow at `.github/workflows/proven.yml`:

```yaml
name: proven
on:
  push: { branches: [ master, main ] }
  pull_request: { branches: [ master, main ] }
  workflow_dispatch:
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
permissions: { contents: read }
jobs:
  proven:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      - uses: studio2201/studio2201@v1
        with: { tools: 'proven', fail-on: 'block' }
      - uses: actions/upload-artifact@v4
        if: always()
        with: { name: proven-report, path: proven_report.md, if-no-files-found: ignore }
```

## How It Works Under the Hood

1. **Pure `std::` SHA-256 Engine (`src/artifact.rs`)**: Implements FIPS 180-4 directly in standard Rust
   without external crates or OpenSSL dependencies.
2. **Streaming Merkle Tree (`src/merkle.rs`)**: Chunks binaries into 64 KiB blocks, hashes leaves, and
   recursively computes a 32-byte Merkle root. Any single-bit change alters the Merkle root.
3. **ML-DSA-65 In-Toto Attestation Envelope (`src/sign.rs`, `src/report.rs`)**: Formulates the canonical
   payload `sha256:merkleRoot:keyId` into an in-toto Statement v1 JSON document with ML-DSA-65 envelope.
4. **Air-Gapped Verification (`src/verify.rs`)**: Re-derives SHA-256 and Merkle root offline, verifying
   integrity without relying on external network transparency logs.
5. **Bit-Reproducibility Guarantee (`tools/dev/repro.sh`)**: Asserts that `cargo build --locked --release --offline`
   matches checked-in cryptographic SHA-256 baselines bit-for-bit.

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
