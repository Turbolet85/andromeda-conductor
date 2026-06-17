# obs extract

## Relevance
Partial — determinism-replay harness is a *test* chunk (proptest + insta snapshots), not production instrumentation; obs covers only the test infrastructure's logging + error boundary, not new scenario execution paths.

## Constraints
- Per §1 Obs Scope Summary (Minimal tier, 7 critical paths): determinism test harness must not introduce OTel SDK or new runtime telemetry signals (§11 Universal ban "NEVER introduce an OTel SDK + exporter for self-observation — breaks `current_thread` determinism").
- Per §3 Observability Harness Contract: no W3C trace context for test spans (§11 Spans ban "NEVER add W3C trace context… for self-observation — correlation is the `run_id` field").
- Per §4 Span / Trace Coverage (must-trace Scenario 1 headless path): the determinism test **does not** emit to the critical-path journal (`journal_emitted_at` + `read_back_observed_at`); it is a *test harness*, not a run-report producer (§11 Anti-Patterns "NEVER log in hot path at `info` level — use `trace` / `debug` gated by `RUST_LOG=conductor_timeline=debug`").
- Per §11 Logs anti-pattern: "NEVER leak absolute host paths or internal struct names in logs / run-report / runs.db — the redaction layer (`conductor-core::redact`) masks absolute host-file paths → `<redacted>`" — applies to test panics if any.
- Per §6 Log Coverage: if panic occurs in test, must be captured via `std::panic::set_hook()` + structured JSON (not unstructured backtrace); §9 CI Integration zero-unlogged-panics gate applies.

## Patterns to follow
- Insta snapshot tests should run under `#[tokio::test(flavor = "current_thread", start_paused = true)]` to preserve determinism; virtual-clock discipline means no `journal_emitted_at` ISO-8601 wall-clock correlation (per scope §Boundaries "keeping the wall-clock-vs-virtual-clock wall intact").
- Proptest regression files (`proptest-regressions/`) committed under `crates/conductor-timeline/` (per scope: "with **committed regressions**").
- Test logs (if emitted to stderr) via `tracing::info!` / `tracing::debug!` gated by `RUST_LOG` env var (test-side control, not runtime; per §6 per-module levels: `conductor-timeline = debug` opt-in).

## Anti-patterns to avoid
- Do NOT snapshot the JSONL run-report envelope (verdict, `read_back_observed_at`, `latency_ms`) — those are non-deterministic wall-clock fields (scope explicitly defers "NOT a golden over the **JSONL emission journal** or its `journal_emitted_at` stamps").
- Do NOT emit OTel traces or `traceparent` headers in test code (§11 Spans ban applies even to test harness).
- Do NOT add a bespoke panic hook inside test code; rely on proptest's panic capture and insta's built-in error handling.

## Contract bindings
- Test harness → tests-plan §3 (test results format + CI artifact upload): insta snapshots + proptest regressions are committed test artifacts. No direct binding to obs harness status endpoint (determinism test is synchronous, not a long-running service).
- Test harness (determinism property) → architecture §Design Philosophy ("same scenario + seed ⇒ identical stream shape"): the golden + proptest properties *enforce* the invariant, bridging intent to test.

## Acceptance criteria contributions
- (obs) Determinism test runs on `start_paused` (no real-time waits; zero flakiness per §1 determinism invariant).
- (obs) Insta golden captures only the seeded/virtual `Vec<PhaseTransition>` stream shape, not `journal_emitted_at` wall-clock fields — golden drift-tripwire for `PhaseTimeline`/scheduler output shape regression.
- (obs) Proptest property: same seed ⇒ identical shape (replay determinism); plus structural invariants (per-gap jitter bound, monotonic `elapsed_ms`) — all seeded and committed in `proptest-regressions/`.
- (obs) No OTel SDK / exporter / `traceparent` introduced; self-obs stays `tracing`-only and the test emits none to `:4317`.

## Relevant amendment history
- 2026-06-16-emission-journal-writer: Run-report envelope (`read_back_observed_at` + 11-field schema) — *not* relevant to determinism test (test does not produce envelope, only `Vec<PhaseTransition>` snapshot).
- 2026-06-15-structured-logging-stack: Self-obs base line (every `tracing` line carries `timestamp_ms`, `level`, `target`, service-identity, `run_id`) — relevant only if test code emits `tracing::debug!`.
- 2026-06-15-log-error-boundary-redaction: Redaction model (host-file-path anchor, allowlist + Display, `target` preserved) — relevant only if a determinism test panics (redaction applies to backtrace via `conductor-core::redact`).
