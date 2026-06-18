# tests extract

## Relevance
Partial — exception-event emission + fingerprint computation are testable in isolation (unit + goldens); MCP read-back verification is out-of-scope (§6/§7); integration with `runs.db` journaling is in-scope.

## Constraints
- Per §1 testable entity `conductor-emit` (OTLP raw-type emission primitives): exception-event struct construction and byte-level encoding unit-testable in isolation; egress liveness to `127.0.0.1:4317` is local-gate-only, not CI (§1).
- Per §1 testable entity `conductor-faults` (fingerprints module): fingerprint generators as pure seeded functions (same seed ⇒ same shape); boundary-seam decision required on whether fingerprint-compute lands in `conductor-emit` or `conductor-faults` (scope.md "Boundary to resolve") (§1).
- Per §2 Test Strategy Integration row: OTLP-egress loopback gRPC `TraceService` stub via tokio-stream/tonic on ephemeral `127.0.0.1:0` is the integration mechanism for exception-event validation (§2; 2026-06-17-raw-otlp-message-scaffold amendment).
- Per §4 Unit Test Strategy: conductor-emit golden tests use exact-string `assert_eq!` at unit level for canonical shape (matching `verdict.rs`/`report_state.rs` serialization-golden pattern), with insta reserved for E2E journal redaction (§4; 2026-06-16-emission-journal-writer amendment).
- Per §3 Test Harness Contract Log format: the `fingerprints` field binds to the per-run emission journal (`runs/<run_id>.jsonl`) and run-report envelope — required field for exception-fingerprint identity (§3).
- Per §1 Critical paths Path 2: fingerprint-storm scenario exercises `conductor-faults` fingerprints via read-back; this chunk produces + journals the expected fingerprint only (scope.md out-of-scope: storm-cue thresholds) (§1).

## Patterns to follow
- Deterministic seeded exception-event generation under `#[tokio::test(flavor="current_thread", start_paused=true)]` matching timeline discipline: same seed + same exception spec ⇒ identical emitted shape (scope.md acceptance intent; §4).
- rstest fixture pattern: `#[fixture]` seeded exception-event builder (no developer-seeded raw OTLP objects) + `#[case]` table-driven variants over identical/path/line fingerprint relationships (§3/§4).
- Golden serialization via exact-string `assert_eq!` for the per-exception canonical shape (exception type + message + stacktrace frames + expected fingerprint), not insta, matching the envelope canonical pattern (§4; 2026-06-16 amendment).
- Integration test against the loopback gRPC stub (`TraceService` on ephemeral `127.0.0.1:0`) for exception-event egress validation (§2; 2026-06-17 amendment).

## Anti-patterns to avoid
- Deserializing OTLP protobuf without bounded recursion guards — bounded prost recursion must not panic on malformed input (§1 security-vector-coverage Vector 4 trigger).
- Leaking absolute host paths or internal seam-crate struct names in the emission journal / run-report `fingerprints` array (§3 Log format; security anti-pattern).
- Wall-clock-stamped fingerprint computation — fingerprints must use deterministic seeded generation (`ChaCha8Rng::seed_from_u64`, shape-projection), NOT `std::time::SystemTime` or tokio virtual clock (scope.md; §3 logs security anti-pattern).

## Contract bindings
- **tests ↔ obs §Log format:** the `fingerprints` JSON array in the per-run emission journal (`runs/<run_id>.jsonl`) + run-report envelope binds the two — tests/harness own the field; obs derives its envelope from the §3 Log format binding.
- **tests ↔ obs §Status endpoint:** the run-report envelope (fingerprints field + `journal_emitted_at`/`read_back_observed_at` timestamps) is the polling contract; tests assert envelope shape via goldens (§3).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes for exception-event unit tests + fingerprint-compute pure-function tests.
- (tests) Deterministic replay: same scenario seed + same exception spec ⇒ identical emitted exception-event + identical expected fingerprint across consecutive runs (golden journal shape).
- (tests) Fingerprint relationships: identical/path-variant/line-variant exception specs produce correct same/different/same fingerprint outputs (identical == line-variant; path-variant differs) (acceptance intent).
- (tests) The per-run emission journal `fingerprints` array is populated + journaled correctly; golden shape asserts no internal struct names leaked.
- (tests) Integration: loopback gRPC stub validates exception-event byte-level encoding on egress to `127.0.0.1:4317` (§5 integration surface, NOT live Pulse).
- (tests) Coverage: new-code line coverage ≥ threshold (§10; cargo-llvm-cov). Clippy `-D warnings` + `cargo audit` + `cargo deny check` + `Cargo.lock` committed + toolchain ≥1.94.1 green.

## Relevant amendment history
- **2026-06-17-raw-otlp-message-scaffold:** OTLP-egress loopback gRPC stub (tokio-stream::TcpListenerStream) added to §2 Integration mechanisms for exception-event egress validation — no harness envelope change.
- **2026-06-16-emission-journal-writer:** conductor-report unit serialization goldens clarified to use exact-string `assert_eq!` (not insta) for canonical line shape; the fingerprints field is part of the canonical envelope golden-locked at unit level.
- **2026-06-15-structured-logging-stack:** per-run emission journal schema (`runs/<run_id>.jsonl`) confirmed distinct from self-obs stream — the fingerprints field is emission-journal-side, not obs self-obs-side.
