# tests extract

## Relevance
relevant — P-026 is a new `conductor-emit` seeded primitive adding pure OTLP-shaping logic; it requires unit testing under the Minimal-tier Rust test strategy.

## Constraints
- (unit tests via cargo-nextest; per test-plan §4) `cargo nextest run -p conductor-emit` must pass for the new rate-shaping module, with the seeded determinism contract (`ChaCha8Rng`, same seed ⇒ same rate curve).
- (determinism golden-locking; per test-plan §4 conductor-report bullet + §2 agent-runnable invariants) The canonical rate-profile output shape is locked via exact-string `assert_eq!` at unit level (not insta), matching the established serialization-golden pattern in `conductor-core` / `verdict.rs` / `report_state.rs`.
- (loopback-capture integration; per test-plan §2 pyramid + §5.1) Rate-profile generator reuses existing OTLP trace builders and integrates via the loopback gRPC stub (`TraceService` over ephemeral `127.0.0.1:0`; dev-dep `tokio-stream`), asserting shaped span-count sequences.
- (no scenario-config wiring; per chunk scope boundary) This primitive is standalone exercised in isolation — scenario-config garde validation and per-P-ID fixture wiring are deferred to the scenario epoch.
- (no new dependency; per chunk scope + test-plan §4 tool-version policy) Reuses `rand_chacha` + existing OTLP message builders; external-CLI tools (cargo-nextest, cargo-llvm-cov) apply at reference-floor versions.
- (coverage gate; per test-plan §10 + §3.3) `cargo llvm-cov nextest --lcov` must meet the Minimal-tier line-coverage threshold; `--fail-under-lines` enforces exit-code gate in CI.

## Patterns to follow
- (fixture pattern; test-plan §4) Use rstest `#[fixture]` for the seeded generator; table-driven `#[rstest]` + `#[case]` for valid/invalid rate-curve test matrices. (NOTE: conductor-emit currently has no rstest dev-dep — prior chunks used plain `#[test]` loops; match crate convention or add the dev-dep deliberately.)
- (unit-level determinism; test-plan §2) Apply `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` IF the primitive is async/timeline-driven; pure-math counts need no async.
- (loopback-gRPC test pattern; per amendment 2026-06-17 + test-plan §5.1) Spin up a stub `TraceService` tonic server on ephemeral `127.0.0.1:0` to capture the shaped span stream; assert per-window span counts match the target rate curve.

## Anti-patterns to avoid
- (serialization goldens; test-plan §4 amendment 2026-06-16) Do NOT use insta for unit serialization goldens in `conductor-emit` — use exact-string `assert_eq!` to lock the canonical line shape (insta is reserved for E2E journal goldens with redaction in §6/§7).
- (no standalone dev-deps; test-plan §4 tool-version policy) Do NOT pin external-CLI tool versions (cargo-nextest, cargo-llvm-cov) as exact Cargo.lock entries — they are reference floors.

## Contract bindings
- **obs ↔ tests harness (§3):** Emission-journal NDJSON format (runs/<run_id>.jsonl) carries the journal envelope fields (`journal_emitted_at` wall-clock stamp from `std::time::SystemTime`, per amendment 2026-06-15). Tests assert JSONL shape; obs derives log-envelope schema from test-plan §3. (THIS chunk adds no new journal fields — see obs extract concurrence.)
- **test-plan §5 coverage-trigger property-test ↔ rate-profile determinism:** The generator satisfies the "property-test (determinism discipline)" trigger (same scenario + seed ⇒ same stream shape); golden test verifies identical seed reproduces identical emission stream.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes, including the new rate-shaping module and loopback-integration tests.
- (tests) Coverage: new-code line coverage ≥ Minimal-tier threshold (§10); `cargo llvm-cov nextest --fail-under-lines` gates CI.
- (tests) Determinism golden: identical seed on rate-profile generator ⇒ identical span-count sequence per-window (property-test + golden assertion via exact-string `assert_eq!`).
- (tests) Loopback-gRPC integration: rate-profile batches integrate via `TraceService` stub to produce the shaped emission stream; per-window counts assert within seeded tolerance.

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Log format): the `tracing` self-obs stream is SEPARATE from the per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth). Rate-profile tests emit into the latter; the two schemas must not be conflated.
- **2026-06-16-emission-journal-writer** (§4 conductor-report bullet): unit serialization goldens use exact-string `assert_eq!` (matching `verdict.rs`/`report_state.rs`); insta reserved for E2E. Applies to rate-profile canonical line-shape locking.
- **2026-06-17-raw-otlp-message-scaffold** (§2 pyramid Integration row): registered the OTLP-egress loopback gRPC stub (`TraceService` over `tokio-stream::wrappers::TcpListenerStream` on ephemeral `127.0.0.1:0`) as the integration-test mechanism for shaped span capture; `tokio-stream` is a dev-dep.
