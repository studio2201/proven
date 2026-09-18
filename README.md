# Proven

**PQC-signed supply-chain attestor.** Reproducible builds with byte-identity hashing keyed on `(host, rustc-version, Cargo.lock)`. Pre-1.0.0 services use GHSA-only security advisories; CVEs at ≥ 1.0.0.

**Status:** v0.2.0 release (2026-09-18).

## What it does

Sign every published artifact with a reproducibility attestation that includes:
- Source commit hash
- Build environment fingerprint
- Dependency Merkle root
- Runtime capability matrix

Compatible with existing SLSA pipelines. Air-gappable. Reproducible.

## Why

- Classical SLSA signatures are RSA/ECDSA. They are quantum-vulnerable by 2030.
- EO 14412 makes PQC signatures for federal HVAs required by Dec 31, 2031.
- Nobody else ships PQC-native SLSA L3 yet. Proven does.

## Commercial plane

Hosted attestation ledger + multi-sig PQC + Ed25519 transition signing for the 2030 window.

## License

Apache-2.0.
