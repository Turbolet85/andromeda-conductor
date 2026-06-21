# tests extract

## Relevance
Partial — OTLP egress liveness check is an integration/system-level gate (transport-layer connectivity), not a broad test coverage deliverable.

## Constraints
- Per test-plan §2 (Test Strategy), integration tests covered via loopback gRPC stub (`tokio-stream::wrappers::TcpListenerStream` on ephemeral `127.0.0.1:0`), never binding the reserved `:4317` — the liveness probe must not conflict with this stub pattern.
- Per test-plan §5 (Integration Test Strategy), OTLP egress boundaries must assert transport-connectability via a typed `Result::Err` (harness fault), never a `Verdict` value — this load-bearing invariant gates whether emission is even attempted.
- Per test-plan §8 (Mocking & Stubbing), gRPC transport faults are testable via turmoil 0.7.2 seeded network simulation (refusal/partition) OR an in-process tonic `TraceServiceServer` on ephemeral loopback; the connectability probe itself is a client-side connect attempt (no server stub running during the probe itself).
- Per test-plan §3 (Test Harness Contract), the harness `run` command's exit-code semantics are: 0 = all checks Pass; non-zero = hard Fail; transport-refusal surfaces as `Result::Err` (harness fault), NOT a process-failure exit.
- Per scope's acceptance intent (P5), a refused/down `:4317` must return `Result::Err` (a typed `EmitError`), never a panic and never a `Verdict` value.

## Patterns to follow
- Per test-plan §8, use dependency injection for transport faults (ephemeral-port loopback tonic server OR turmoil seeded deterministic refusal) — avoid monkey-patching production code.
- Per test-plan §4/§5, unit-test the connectability primitive in isolation (pure function or state-free probe) with a rust `#[tokio::test(flavor="current_thread")]` harness; integration-test the refusal-to-blocked-state mapping with an rmcp stub + insta golden (run-report envelope).
- Per test-plan §7 (Test Data & Fixtures), use rstest `#[case]` table-driven variants for connectable / refused / timeout cases, seeding the probe with deterministic scenarios.

## Anti-patterns to avoid
- Do NOT bind `:4317` in unit/integration tests — that port is reserved for the security Vector 6 (port-occupier fault); use ephemeral `127.0.0.1:0` or `127.0.0.1:<unused>` for the test stubs.
- Do NOT return a `Verdict`/`ReportState` value for a refused transport — the scope explicitly requires `Result::Err` (harness fault only) at the module boundary.
- Do NOT introduce a seam→seam dependency edge to invoke the probe from outside `conductor-emit` — the star topology (every seam depends on `conductor-core` only) forbids this; the primitive lives in conductor-emit and surfaces its error as a typed `EmitError`.

## Contract bindings
- obs ↔ tests harness: the probe-error sanitization (no absolute host path, no `tonic` struct name, no stack trace) must pass through the conductor-cli / `anyhow` edge; verify via a negative-test assertion that the error message is operator-safe (test-plan §3 artifact-sanitization boundary).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-emit` passes, including a unit test asserting a connectable `:4317` returns `Ok(())` and a refused/unreachable target returns `Result::Err(EmitError)` — never a panic, never a `Verdict`.
- (tests) Integration test via insta golden asserts the refusal error maps to a harness-fault condition in the run-report envelope (not `Blocked`/`Fail`/`Pass`), and the error is sanitized at the CLI output boundary.
- (tests) The emit crate's star-topology is verified via `Cargo.toml` edge audit — no new seam dependencies introduced.

## Relevant amendment history
2026-06-17-raw-otlp-message-scaffold — the OTLP-egress loopback gRPC `TraceService` stub (`tokio-stream` + ephemeral loopback) was registered as a new integration mechanism; the liveness probe primitive must be tested within the same deterministic stub harness (turmoil or in-process tonic server on ephemeral port), never against the reserved `:4317`.
