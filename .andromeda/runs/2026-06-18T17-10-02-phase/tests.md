# tests extract

## Relevance
Relevant — this chunk adds a pure latency-shaping function to `conductor-emit` that must be unit-tested and integrated into the emission path.

## Constraints
- Unit tests must assert seeded determinism: same target profile + seed ⇒ identical duration sequence (per test-plan §2 Agent-runnable invariants + §7 Seed strategies for synthetic telemetry stream) — replicable via property testing or exact golden assertion over a fixed seed
- Configuration precondition: the p50 ≤ p95 ≤ p99 ordering is already validated upstream in `conductor-core`'s garde layer; this primitive consumes an ordered profile (may assert as a local precondition, per scope.md) — do not duplicate validation (per test-plan §4 "What unit tests cover" + scope boundaries §Not config validation)
- Must integrate with the existing raw-OTLP `Span` timing fields (`start_time_unix_nano` / `end_time_unix_nano`) and `TraceEmitter` path — integration tests verify the shaped durations appear correctly in the emitted stream (per test-plan §5 Module ↔ module cross-seam testing pattern + §8 "What NOT to mock: pure functions in seams under test")
- Per-operation independence: distinct operations carry distinct profiles within one batch — tests must assert no cross-operation duration bleed (per scope.md acceptance intent)
- Coverage floor for the Minimal tier is ≥60% line coverage, measured at the workspace level via cargo-llvm-cov `--fail-under-lines 60` (per test-plan §10 Quality Gates)

## Patterns to follow
- Unit test fixture pattern: rstest 0.26.1 `#[fixture]` producing a seeded RNG (matching the existing `conductor-timeline` seeded-fixture discipline) — parametrized over the P-011/P-012 profiles + seed combinations (per test-plan §4 Fixture pattern + §7 Seed strategies)
- Use `#[rstest]` + `#[case]` table-driven rows for multiple latency profiles (e.g., "normal p50/p95/p99" vs. "regressed 3× p99" from scope.md) and edge-case percentile orderings
- Integration tests: assert shaped durations appear in the raw-OTLP message stream via a loopback gRPC `TraceService` stub on `127.0.0.1:0` (per test-plan §5 Integration boundary types + §8 "OTLP/gRPC egress" mocking via tonic server) — verify the bytes encode the expected timing fields
- Golden test the canonical serialized-duration output shape (if stored) via insta 1.46.1 JSON snapshot (per test-plan §4 conductor-report bullet + §7 "Golden artifacts")

## Anti-patterns to avoid
- Do NOT rely on real wall-clock timing assertions or `tokio::time::sleep` for duration validation — use seeded RNG + exact sampling assertions under a fixed seed (per test-plan §11 Unit anti-pattern "NEVER assert on a real wall-clock duration")
- Do NOT fake the profile ordering validation when the primitive receives out-of-order p50/p95/p99 — assert (as a precondition) or let it panic, but do NOT silently ignore (scope.md says core is the authoritative validator; this primitive asserts, not validates)
- Do NOT over-mock the `TraceEmitter` or the OTLP `Span` struct — emit real bytes to the tonic stub and verify the timing fields are present and correctly ordered (per test-plan §11 Mocking anti-pattern "NEVER over-stub")

## Contract bindings
emit ↔ core: config validation (p50 ≤ p95 ≤ p99 ordering) owned by core's garde layer; emit consumes and may assert the precondition locally · emit ↔ verify: latency-shaping output feeds P-011 (baseline) and P-012 (regression detection) verification logic, live-gate-only per scope.md § "Not verification" · emit ↔ obs: wall-clock journal stamps from `std::time::SystemTime`, not tokio virtual clock (per test-plan §3 Log format + §8 Mocking "NOT for journal/report stamps")

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes for new unit tests covering the latency-shaping seeded-generator function.
- (tests) Coverage: new-code line coverage ≥60% (Minimal tier floor per test-plan §10).
- (tests) Determinism: fixed seed + profile pair reproduces identical duration sequence across re-runs (unit golden or property-test assertion per test-plan §7 determinism enforcement).
- (tests) Per-operation independence: distinct operation profiles in one batch yield distinct duration streams (integration assertion via the loopback gRPC `TraceService` stub, verifying Span timing fields per operation name).

## Relevant amendment history
2026-06-17-raw-otlp-message-scaffold — the OTLP-egress loopback gRPC stub (`tonic` `TraceService` server on `127.0.0.1:0` via `tokio-stream::wrappers::TcpListenerStream`) was registered in test-plan §2 Integration test-pyramid row and §5 cross-module patterns as the mechanism for verifying raw-OTLP egress behavior. This chunk's integration-level latency-shaping tests will drive the Span timing fields through this same stub server (no live `:4317` bind — only the loopback ephemeral port).