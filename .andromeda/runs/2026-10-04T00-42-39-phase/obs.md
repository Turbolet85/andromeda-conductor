# obs extract

## Relevance
partial — a `#[cfg(test)]`-only origin fix that adds no span, metric or log; obs's stake is that the instrumentation the IPC tests dispatch through stays intact, and that the nextest/panic gates stay clean on both hosts.

## Constraints
- Every `#[tauri::command]` handler is required to open a MANUAL `tracing::info_span!("tauri.command.<name>").entered()` guard (the attribute form does not stack with the command macro). The chunk is test-only, so it must leave those guards byte-unchanged in the production handlers it dispatches into. Whether the six failing tests dispatch into guarded handlers, and which ones, is research's question (per obs-plan §4 Span / Trace Coverage, desktop-webview row; §1 Obs Scope Summary, desktop-webview / ipc-internal rows).
- Span names must stay inside the bounded set, which includes `tauri.command.*`. A host-derived origin is test-fixture input and must never become a span name or a span attribute (per obs-plan §11 Obs Anti-Patterns → Spans / Traces).
- The desktop-webview surface is specified as Windows/macOS/Linux, and the exporter is specified as platform-agnostic. A host-correct test origin matches that stated portability. It does not license a platform-specific obs path (per obs-plan §1 Obs Scope Summary, multi-platform-exporter-compat row).
- The zero-unlogged-panics invariant holds on every CI run: `cargo-nextest` plus log conformance plus panic-hook verification, with no retry-once policy. A test made green on Linux must pass by dispatching correctly. Masking a failure does not count (per obs-plan §10 SLO Invariants & Telemetry Budgets; §11 Obs Anti-Patterns → SLO, → CI).
- cargo-nextest structured output is a required CI telemetry artifact on every run. The Windows CI leg's nextest JSON must stay green and uploaded after the change (per obs-plan §9 CI Integration, Telemetry artifact handling / Pipeline integration).

## Patterns to follow
- Correlation across the IPC boundary is the `run_id` envelope field, never a `traceparent`. A test helper that builds an `InvokeRequest` changes only the dispatch origin and must not add trace context (per obs-plan §3 Correlation (no distributed tracing); §11 Obs Anti-Patterns → Spans / Traces).
- Keep one owning source for a host-dependent value. This mirrors the single-location ownership discipline obs-plan prescribes for its own cross-cutting rules: the scope's "one host-correct source" for both literal sites follows the same shape (per obs-plan §11 Obs Anti-Patterns → PII Scrubbing, single-location ownership clause).

## Anti-patterns to avoid
- NEVER add retry-once or skip behaviour to make a host-dependent test pass. That masks real failures, and the obs gates rely on nextest running every test (per obs-plan §11 Obs Anti-Patterns → SLO).
- NEVER introduce an OTel SDK, `tracing-opentelemetry` or W3C trace context while touching the IPC test path (per obs-plan §11 Obs Anti-Patterns → Telemetry Strategy, → Spans / Traces).

## Contract bindings
- obs ↔ tests: the CI stages read `cargo-nextest` JSON output as their unit/integration telemetry (per obs-plan §9 CI Integration, Pipeline integration). The scope's acceptance (gate 27 green on Linux; CI `windows-latest` green) is the tests-side consumer of that artifact.
- obs ↔ tests (panic gate): the zero-unlogged-panics gate greps `logs/agent-latest.jsonl` plus stderr for `^thread.*panicked` (per obs-plan §9 CI Integration, Zero-unlogged-panics gate). A Linux mock-runtime dispatch failure that surfaces as a test panic belongs to the tests domain. Whether such output reaches the gated streams is research's question.

## Acceptance criteria contributions
- (obs) The production `#[tauri::command]` handlers' `tauri.command.<name>` span guards are unchanged by the diff, and the chunk adds no span, metric or log call site (per obs-plan §4 Span / Trace Coverage, desktop-webview row).
- (obs) CI `windows-latest` uploads cargo-nextest structured output for the run over this change, with zero failed tests and no test skipped or retried (per obs-plan §9 CI Integration; §11 Obs Anti-Patterns → SLO).
- (obs) The diff introduces no OTel SDK, `tracing-opentelemetry` or `traceparent` usage (per obs-plan §11 Obs Anti-Patterns → Spans / Traces).
