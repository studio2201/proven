//! tools/perf/bench.rs — §18 bench harness for proven.
//!
//! Lives as a `#[test]` in tests/integration.rs via `include!` so it is
//! exercised by `cargo test --release perf_proven_sign_within_budget`.
//! Honors §18: std::time only, median-of-5, line-oriented output.
//!
//! Budget: proven sign on 1 MiB artifact ≤ 1.2 s median ±25%.

use std::time::Instant;

#[test]
fn perf_proven_sign_within_budget() {
    let artifact = synth_artifact_with_size(1_048_576); // 1 MiB
    let budget_ms: f64 = 1_200.0;
    let tolerance: f64 = 0.25;
    let ceiling_ms = budget_ms * (1.0 + tolerance);

    // Warm-up: prime allocator and caches, but do not measure.
    let _ = proven::sign(&artifact, &signing_key());

    // Five timed runs.
    let mut samples: Vec<f64> = Vec::with_capacity(5);
    for _ in 0..5 {
        let t = Instant::now();
        let _ = proven::sign(&artifact, &signing_key());
        let elapsed_ms = t.elapsed().as_secs_f64() * 1000.0;
        samples.push(elapsed_ms);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_ms = samples[2]; // median of 5

    let pass = median_ms <= ceiling_ms;
    println!(
        "verb=proven_sign median_ms={:.3} budget_ms={:.0} pass={}",
        median_ms, budget_ms, pass
    );
    assert!(
        pass,
        "proven_sign regression: median {:.1}ms > ceiling {:.1}ms",
        median_ms, ceiling_ms
    );
}

// Fixture builders — synthetic, committed (per §18-C5).
fn synth_artifact_with_size(_bytes: usize) -> proven::Artifact { unimplemented!() }
fn signing_key() -> proven::SigningKey { unimplemented!() }
