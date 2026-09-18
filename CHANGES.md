# Changelog — proven

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

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
