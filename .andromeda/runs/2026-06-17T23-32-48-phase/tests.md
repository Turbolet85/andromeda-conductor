# tests extract

## Relevance
Partial — chunk adds new OTLP emission primitives (error spans) within an existing test strategy framework.

## Constraints
- Per §2 Test Strategy integration pyramid: OTLP-egress loopback gRPC stub (`tokio-stream::wrappers::TcpListenerStream` on ephemeral `127.0.0.1:0`) is the mechanism for testing error-span emission (never binding real `:4317`).
- Per §4 Unit Test Strategy conductor-emit bullet: OTLP struct construction + byte-level encoding tested in isolation; egress liveness is local-gate-only, not unit.
- Per §5 Integration Test Strategy: cross-seam `conductor-emit` ↔ `conductor-core` boundaries tested via loopback gRPC stub assertion (received protobuf carries `Status.Code=ERROR` at expected span).
- Per §10 Quality Gates: new code line coverage ≥ threshold (Minimal-tier); cargo-nextest exit code semantics (0 = all Pass; non-zero = hard Fail).
- Per §3 Test Harness Contract / Verdict/error wall: `EmitError` for harness/transport faults; typed input at `tonic::Status` boundary; error details sanitized (no internal struct names leaked).
- Per scope definition of done: `Cargo.lock` committed + `cargo-audit`/`cargo-deny` green + `cargo clippy -D warnings` + `cargo nextest` clean.

## Patterns to follow
- Error-status construction via `Status { code: STATUS_CODE_ERROR, message }` tested against deterministic seed input (no entropy/clock reads); same scenario+seed ⇒ same stream shape (§2 determinism discipline).
- Multi-span trace golden locked via exact-string `assert_eq!` on canonical serialization (e.g. `verdict.rs`/`report_state.rs` pattern per §4), with parent/child linkage + trace_id/span_id determinism asserted at unit level.
- Loopback gRPC stub assertion on received protobuf: `parent_span_id` linkage well-formed, `Status.Code` sits at expected span (root or deep child), envelope shape conforms to OTLP wire format.
- Test-selection syntax `cargo nextest run -p conductor-emit` for fast iteration; section fixtures via rstest `#[fixture]` seeded generators.

## Anti-patterns to avoid
- Do NOT emit span/trace IDs via raw RNG or `Instant::now()` — use seeded deterministic generation passed as builder input.
- Do NOT test OTLP egress liveness to live Pulse (`:4317`) at unit/CI — that is local-gate-only; unit+integration use the loopback ephemeral stub.
- Do NOT leak internal struct names or absolute host paths in error artifacts (security anti-pattern §3).

## Contract bindings
- obs ↔ tests harness: the 5-command `logs` discipline reads per-run JSONL journals (emission wall-clock timestamps from `std::time::SystemTime`, NOT tokio virtual clock); span-error status messages are emitted as journal content and must not expose internal details (security: sanitization invariant per §3).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes with all new error-span unit tests (span-status construction, determinism, parent/child linkage assertions).
- (tests) Integration test asserting multi-span error trace shipped to loopback gRPC stub + received protobuf has `Status.Code=ERROR` at expected span.
- (tests) Coverage: new-code line coverage ≥ Minimal-tier threshold (per §10).
- (tests) `cargo clippy -D warnings` clean; `Cargo.lock` committed + `cargo-audit`/`cargo-deny` green.

## Relevant amendment history
- **2026-06-17-raw-otlp-message-scaffold**: OTLP-egress loopback gRPC stub (`tokio-stream::wrappers::TcpListenerStream` on ephemeral `127.0.0.1:0`) registered as the integration test mechanism (never `:4317`). This chunk uses that stub pattern for error-span emission verification. Dev-dep `tokio-stream` already approved in test-plan §2 Integration row.
- **2026-06-16-emission-journal-writer**: Unit serialization goldens use exact-string `assert_eq!` (canonical line shape locked at unit level); insta reserved for E2E journal goldens with redaction. Applies to error-status message serialization.
