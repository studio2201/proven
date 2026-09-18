//! tests/integration.rs — integration tests for proven.
//! Includes the §18 bench harness from tools/perf/bench.rs so it runs as
//! a `#[test]` via `cargo test --release perf_proven_sign_within_budget`.
//! Per plan decision (6): no separate [[bin]]; bench lives in the test target.

include!("../tools/perf/bench.rs");
