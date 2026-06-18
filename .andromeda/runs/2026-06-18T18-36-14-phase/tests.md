# tests extract

## Relevance
Partial — this chunk adds an emission primitive (multi-service trace topology) within `conductor-emit`; unit and integration surfaces touched, but no CLI/E2E scenario expansion yet.

## Constraints
- per §1 Scope Summary: Test tier is Minimal (0) with determinism hard-bar + property-test coverage triggers on cross-field invariants.
- per §2 Test Strategy: Unit tests dominate; integration boundaries (OTLP egress loopback stub) are included; agent-runnable invariants (cargo-nextest JUnit, determinism via `#[tokio::test(flavor="current_thread", start_paused=true)]`).
- per §3 Test Harness Contract: emission artifacts flow into the Run-report envelope (verdict/state/latency_ms); JSONL journal shape locked by insta golden tests with `run_id`/timestamp redaction; loopback TraceService stub on ephemeral `127.0.0.1:0` for transport assertions.
- per §4 Unit Test Strategy: `conductor-emit` unit tests in `src/` cover span/trace construction in isolation; `tests/` integration tests exercise the loopback egress stub.
- per §5 Integration Test Strategy (entity `conductor-emit`): partially-testable — OTLP struct construction + byte-level encoding unit-testable; egress liveness to `127.0.0.1:4317` local-gate-only (live Pulse requirement, arch CI/CD note).
- per §7 Test Data & Fixtures: seeded synthetic generation; rstest table-driven valid/invalid matrix where applicable (scenario wiring deferred).

## Patterns to follow
- **Seeded determinism:** use `ChaCha8Rng` for service ID/trace ID generation; same scenario+seed ⇒ same stream shape. Assert via property/golden test on the seed-governed projection (wall-clock `start`/`end` excluded from determinism lock per latency/error-spans precedent).
- **Loopback TraceService stub:** spawn `TcpListener` on `127.0.0.1:0` (ephemeral port); mock server reads `ExportTraceServiceRequest`, asserts distinct `service.name` ResourceSpans + shared `trace_id` + cross-service `parent_span_id` linkage. Reuse the existing emit-test stub pattern (`tests/egress.rs` / `latency_shaping.rs`).
- **Golden snapshot discipline:** insta snapshots where a stable projection exists, with `run_id`/timestamp redaction; exact-string `assert_eq!` for serialization-golden lines.
- **Type-level safety:** thiserror `EmitError` for transport refusal → maps to verdict/error wall; no string-formatted error details.

## Anti-patterns to avoid
- Do NOT construct trace IDs / span IDs non-deterministically (avoid `rand::random()`, `SystemTime::now()` in the emission path — wall-clock only in the journal envelope).
- Do NOT skip W3C trace context propagation on cross-service spans (parent_span_id must reference a valid upstream span_id under the same trace_id).
- Do NOT live-bind to the actual Pulse `:4317` port in unit/integration tests — use loopback stub on ephemeral port (live leg is local-gate-only).

## Contract bindings
- **tests ↔ obs:** JSONL log-format binding (obs §3). Emission journal lines carry envelope fields (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`); the harness greps the journal for assertions.
- **tests ↔ emit (`span_tree.rs`):** the cross-service parent-linkage builder extends the existing root-vs-child placement from the error-spans chunk; tests reuse the raw OTLP scaffold and assert cross-service edges under one `trace_id`.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes for the new multi-service span builder unit tests.
- (tests) Coverage: new code (service.name partitioning, cross-service linkage) ≥ threshold per §10 Quality Gates.
- (tests) Loopback TraceService stub integration test asserts: distinct `service.name` ResourceSpans present; one shared `trace_id` across services; valid cross-service `parent_span_id` references.
- (tests) Determinism property test: same scenario+seed ⇒ same multi-service span stream shape (seed-governed projection); `ChaCha8Rng` cross-platform stable; wall-clock fields excluded from the lock.

## Relevant amendment history
(none)
