# Proven

**PQC-signed supply-chain attestor.** Merkle AST + ML-DSA-65 + opm. SLSA L3+ on openOODA.

**Status:** pre-release scaffold (2026-09-17). No source code yet.

## What it does

Sign every published artifact with an ML-DSA-65 attestation that includes:
- Source commit hash
- Build environment fingerprint
- Dependency Merkle root
- Runtime cap-token matrix

Compatible with existing SLSA pipelines. Air-gappable. Reproducible.

## Why

- Classical SLSA signatures are RSA/ECDSA. They are quantum-vulnerable by 2030.
- EO 14412 makes PQC signatures for federal HVAs required by Dec 31, 2031.
- Nobody else ships PQC-native SLSA L3 yet. Proven does.

## Commercial plane

Hosted attestation ledger + multi-sig ML-DSA-65 + Ed25519 transition signing for the 2030 window.

## License

Apache-2.0.
