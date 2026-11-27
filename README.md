# Proven

<div align="center">

[![Release](https://img.shields.io/badge/version-v0.2.12-blue.svg)](https://github.com/studio2201/proven/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

| Security Pillar | Verification Badge |
| :--- | :---: |
| **Platform Standard** | [![secured by studio2201][b-studio]][u-home] |
| **Credential Defense** | [![snip][b-snip]][u-snip] |
| **Supply Chain Surface** | [![vigil][b-vigil]][u-vigil] |
| **Post-Quantum Cryptography** | [![aegis][b-aegis]][u-aegis] |
| **Build Integrity** | [![proven][b-proven]][u-proven] |
| **Repository Governance** | [![boneyard][b-boneyard]][u-boneyard] |

[b-studio]: https://img.shields.io/badge/secured%20by-studio2201-1b5e4a?logo=shield
[u-home]: https://studio2201.com
[b-snip]: https://img.shields.io/badge/snip-0%20secrets-1b5e4a?logo=shield
[u-snip]: https://studio2201.com/snip
[b-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-1b5e4a?logo=shield
[u-vigil]: https://studio2201.com/vigil
[b-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-1b5e4a?logo=shield
[u-aegis]: https://studio2201.com/aegis
[b-proven]: https://img.shields.io/badge/proven-HMAC--SHA256%20verified-1b5e4a?logo=shield
[u-proven]: https://studio2201.com/proven
[b-boneyard]: https://img.shields.io/badge/boneyard-maintained-1b5e4a?logo=shield
[u-boneyard]: https://studio2201.com/boneyard

</div>

**Supply-chain artifact attestor.** Computes bit-identity SHA-256 and 64 KiB Merkle roots, signs in-toto provenance envelopes with pure `std::` RFC 2104 HMAC-SHA256 (FIPS 198-1), and verifies build artifacts offline.

## Why This Action Is Needed

### The Tampered Build Vector & Provenance Integrity
A clean Git commit does not guarantee a clean binary. If a CI builder, package repository, or
distribution mirror is compromised, backdoors are injected post-compilation (e.g. SolarWinds,
XZ Utils CVE-2024-3094). Release integrity requires verifiable provenance binding the artifact to its digest.
- **[White House Executive Order 14028](https://www.whitehouse.gov/briefing-room/presidential-actions/2021/05/12/executive-order-on-improving-the-nations-cybersecurity/)**:
  Mandates verifiable software provenance and artifact integrity checks across software delivery.
- **[OpenSSF SLSA Specification v1.0](https://slsa.dev/spec/v1.0/)**:
  Defines the in-toto Statement v1 format used by Proven to bind binary SHA-256 digests and Merkle roots.
- **Automated CI Gates vs Manual Verification**: Manual verification of checksums and attestations is easily skipped
  during urgent releases. Proven automates SHA-256 hashing, Merkle tree construction, and HMAC-SHA256 verification.

## Autonomous Agent Integration

Deploy Proven into your release and CI pipelines using your AI coding assistant or copy the workflow below.

### Prompt for your AI Agent

Copy and paste this prompt to Cursor, Claude Code, Copilot Workspace, or Devin:

```text
Add a GitHub Actions workflow to this repository at .github/workflows/proven.yml using studio2201/proven@v1:
- Name: proven
- Triggers: pull_request, push to default branch (master/main), and workflow_dispatch.
- Concurrency: group ${{ github.workflow }}-${{ github.ref }} with cancel-in-progress on pull_request.
- Permissions: contents: read, checkout with fetch-depth: 0.
- Action: run studio2201/proven@v1 with path: 'target/release/your-binary'.
- Artifacts: upload proven_summary.md using actions/upload-artifact@v6 with if: always().
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
      - uses: actions/checkout@v6
        with: { fetch-depth: 0 }
      - uses: studio2201/proven@v1
        with: { path: 'target/release/your-binary' }
      - uses: actions/upload-artifact@v6
        if: always()
        with: { name: proven-report, path: proven_summary.md, if-no-files-found: ignore }
```

## How It Works Under the Hood

1. **Pure `std::` SHA-256 Engine (`src/artifact.rs`)**: Implements FIPS 180-4 directly in standard Rust
   without external crates or OpenSSL dependencies.
2. **Streaming Merkle Tree (`src/merkle.rs`)**: Chunks binaries into 64 KiB blocks, hashes leaves, and
   recursively computes a 32-byte Merkle root. Any single-bit change alters the Merkle root.
3. **HMAC-SHA256 In-Toto Attestation Envelope (`src/sign.rs`, `src/report.rs`)**: Formulates the canonical
   payload `sha256:merkleRoot:keyId` into an in-toto Statement v1 JSON document signed with pure `std::` RFC 2104
   HMAC-SHA256 (`PROVEN_SIGNING_KEY` env secret or key-id derivation).
4. **Fail-Closed Verification (`src/verify.rs`)**: Re-derives SHA-256, Merkle root, and constant-time HMAC-SHA256 offline.
5. **Bit-Reproducibility Guarantee (`tools/dev/repro.sh`)**: Asserts that `cargo build --locked --release --offline`
   matches checked-in cryptographic SHA-256 baselines bit-for-bit.

## Quick Start

```bash
# Install from source (or download release tarballs from GitHub Releases)
cargo install --git https://github.com/studio2201/proven

# Compute SHA-256 and Merkle root hashes
proven hash target/release/binary

# Sign artifact with HMAC-SHA256 envelope (text or json)
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

Emit in-toto v1 provenance and HMAC-SHA256 verification badges:

```markdown
<!-- In-Toto Provenance Badge -->
[![Provenance](https://img.shields.io/badge/provenance-in--toto%20v1-blue.svg)](https://studio2201.com/proven)

<!-- Artifact Attestation Badge -->
[![Attestation](https://img.shields.io/badge/attestation-HMAC--SHA256-2f6f5e.svg)](https://studio2201.com/proven)
```

## License

Apache-2.0.
