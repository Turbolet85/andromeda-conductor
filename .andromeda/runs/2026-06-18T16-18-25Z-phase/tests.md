# tests extract

## Relevance
Partial — the chunk is an emit primitive (raw logs builder + LogsEmitter) that touches testing infrastructure (loopback gRPC stub, fixture wiring) but is not a full scenario integration.

## Constraints
- Per §2 Test Strategy: integration layer uses loopback gRPC logs-service stub (ephemeral `127.0.0.1:0`); real `:4317` never bound
- Per §4 Unit Test Strategy: conductor-report (envelope serialization) is golden-locked via exact-string `assert_eq!` at unit level; insta reserved for E2E
- Per §3 Test Harness Contract / Log format: structured JSONL with required fields (`journal_emitted_at`, `run_id`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) — no internal struct names, no absolute host paths
- Per §2 Test Strategy / Agent-runnable invariants: no real network beyond loopback gRPC stub; proptest counterexamples persisted to `proptest-regressions/`
- Per §3 Test Harness Contract / Log format: self-obs stream (stderr / `logs/agent-latest.jsonl`) is SEPARATE from per-run emission journal (`runs/<run_id>.jsonl`); two schemas must not be conflated
- Per test-scope §Coverage triggers (property-test): determinism under seed — same scenario + seed ⇒ identical emission-journal stream shape across runs

## Patterns to follow
- Integration boundary: loopback gRPC stub for `LogsService` (mirroring existing `TraceService` stub per §2 Integration row); ephemeral port `127.0.0.1:0`
- Unit-level fixture: rstest `#[fixture]` for seeded `conductor-timeline` generator + `#[rstest]` `#[case]` rows for valid/invalid severity configs; `rusqlite::Connection::open_in_memory()` per test
- Determinism test: property-test (or golden-replay) same seed + same log spec ⇒ identical shape-projection (severity + body + linkage, excluding wall-clock stamps)
- Egress error wall: refused/malformed transport ⇒ typed `EmitError` `Result::Err`; never panic on tonic::Status

## Anti-patterns to avoid
- Do NOT bind real `:4317` in tests; ephemeral loopback only
- Do NOT use tokio's virtual clock for `journal_emitted_at` or `observed_time_unix_nano` — use `std::time::SystemTime` / `Instant`
- Do NOT include internal struct names, absolute host paths, or service-identity fields (`service.name` resource is OK; flattened `service.*` fields belong to obs stream, not emission envelope)

## Contract bindings
- **Logs egress ↔ gRPC stub:** loopback stub must accept `ExportLogsServiceRequest` and return a valid `ExportLogsServiceResponse` (parallel to trace stub per 2026-06-17 amendment)
- **Severity number ↔ Pulse hard-signal:** P-007 requires records on both sides of the 17-boundary (SeverityNumber 16/WARN and 17/ERROR); stub must preserve `SeverityNumber` + `SeverityText` pairing for read-back assertion
- **Emission journal ↔ obs stream:** journal format (§3) is source of truth; obs derives its product-side envelope FROM this plan, not reverse (amendment 2026-06-15)
- **Determinism ↔ timeline seeding:** same seed ⇒ same record identity/timing bytes (seed governs only timing/identity, not severity — severity is spec-controlled)

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes for LogRecord builder + LogsEmitter unit tests
- (tests) Coverage: new-code line coverage ≥ threshold (§10 Quality Gates) for logs-request builder + LogsEmitter egress path
- (tests) Loopback gRPC stub receives `ExportLogsServiceRequest` with controlled `SeverityNumber` (16/WARN and 17/ERROR records distinguishable); refused/malformed egress surfaces as typed `EmitError`
- (tests) Determinism property: same seed + same log spec ⇒ identical shape-projection (severity + body + linkage), excluding wall-clock stamps

## Relevant amendment history
- **2026-06-15-structured-logging-stack (§3):** Self-obs stream (`logs/agent-latest.jsonl`, service-identity + `run_id`) is SEPARATE from per-run emission journal (`runs/<run_id>.jsonl`, SLO ground truth); do not conflate schemas
- **2026-06-17-raw-otlp-message-scaffold (§2):** OTLP-egress loopback gRPC stub (tonic + `tokio-stream::wrappers::TcpListenerStream`, ephemeral `127.0.0.1:0`) registered in integration mechanisms; never binds real `:4317`