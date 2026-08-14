# tests extract

## Relevance
Partial — the chunk ships a diagnostic capture, not new scenario coverage, but the capture channel, its artifact hygiene, and any Conductor-side regression lock are all tests-domain mechanisms.

## Constraints
- The capture channel must be a loopback gRPC stub on an ephemeral `127.0.0.1:0`, never a bind of the real `:4317` (reserved for the Epoch-4 port-occupier) and never a listener of Conductor's own — this is the tests-side form of the scope's "capture must not become an inbound listener" (per test-plan §2 Test Strategy Integration row + §11 Universal project-specific).
- The live-Pulse half of the capture is an operator/local gate (`workflow_dispatch` / `scripts/agent-run.sh`), never a CI gate; CI exercises stub legs only, and `conductor-emit`'s real `:4317` egress is already classified local-gate-only (per test-plan §9 Live-Pulse scenarios + §1 `conductor-emit` entity).
- Whatever artifact settles the fork must be agent-parseable (`serde_json::from_str` / `jq -e`) with a machine-readable pass/fail signal — no human-in-loop step, no screenshot, no visual review (per test-plan §2 Agent-runnable invariants + §11 Universal).
- The capture must not leak absolute host paths or internal seam-crate struct names, and any field or span attribute it adds to the self-obs stream is gated by the `conductor-core::redact` allowlist (per test-plan §3 Status endpoint shape + §3 Log format).
- `logs/agent-latest.jsonl` (self-obs, event + span-lifecycle line variants) and `runs/<run_id>.jsonl` (emission journal / SLO ground truth) are two distinct schemas that must not be conflated; test-plan §3 is the source of truth for the JSONL format and obs derives from it (per test-plan §3 Log format, "Self-obs stream is a distinct artifact").
- If the answer lands Conductor-side, the fix carries a deterministic regression test at the emit/dispatch-wire tier; wire-projection goldens exclude every `*_time_unix_nano` rather than masking it, and are committed fail-don't-write (never `cargo insta review`) (per test-plan §7 Golden artifacts row).
- Zero-flakiness bar: no `sleep(N)` synchronization on the capture, no nextest `retries`; wait on an explicit signal (captured request present, journal line, exit code) (per test-plan §10 Zero-flakiness budget + §11 E2E).

## Patterns to follow
- `D:\dev\projects\conductor\crates\conductor-emit\tests\exception_events.rs` — already the exact capture pattern for this chunk's question: capturing `TraceService` on an ephemeral port asserting the OTel `exception` event (type/message/stacktrace) on an ERROR span. Extend this rather than invent a new channel.
- `D:\dev\projects\conductor\crates\conductor-emit\tests\egress.rs` — the reusable `start_stub()` shape (`TcpListener::bind("127.0.0.1:0")` → `TcpListenerStream` → `Arc<Mutex<…ExportTraceServiceRequest>>`) under `#[tokio::test(flavor = "current_thread")]`.
- `D:\dev\projects\conductor\crates\conductor-run\tests\dispatch_wire.rs` — declared-shape → real `run_timeline_with` + `Dispatcher` → wire capture under `start_paused`, with per-family × seed goldens; the tier at which "what Conductor actually sent" is provable without Pulse.
- 5-command harness for the live leg: `agent-run boot` (two derived budgets, contract-floor clamped) → `run` → `logs` reading the journal; `status` reads disk artifacts, there is no endpoint (per test-plan §3 5-command implementation).
- Isolation for any subprocess leg: rstest seeded fixtures + `assert_fs::TempDir` with `CONDUCTOR_RUNS_DIR`, in-memory `runs.db` per test (per test-plan §3 Test data bootstrap + §7).

## Anti-patterns to avoid
- Never bind `:4317` from a test and never let the capture become a general-purpose inbound listener (per test-plan §11 Universal, project-specific).
- Never fake Pulse's reaction as a CI verdict — a stub capture proves Conductor's wire output only; Pulse's appender behavior stays a live/local-gate observation. And never force-fit wiremock-rs onto the gRPC egress (per test-plan §11 Test Strategy + §11 Mocking).
- Never stamp or assert capture artifacts from tokio's virtual clock, and never treat a `Blocked`/`ManualCheck`/`KnownResidual` outcome of the live leg as a non-zero process exit (per test-plan §11 Test Data + §11 E2E).

## Contract bindings
- tests §3 Log format ↔ obs §3: test-plan §3 owns the JSONL self-obs schema (base fields + the event / span-lifecycle line variants); if the capture adds a field or span attribute to `logs/agent-latest.jsonl`, both plans amend in lockstep (D-tests-obs-harness). The redaction allowlist itself is obs-owned; test-plan asserts that boundary via a negative test (per test-plan §3 log-format-bind-with-obs).
- tests ↔ security: the sidecar env/argv discipline (`.env(...)` only, fixed program path) and the `cargo audit --deny warnings` + `cargo deny check` + un-drifted `Cargo.lock` gate are enforced as tests-side CI build-failure conditions (per test-plan §9 Build failure conditions + §11 Mocking stack-specific).
- tests ↔ arch: `:4317` occupancy / no-inbound-listener scope law is enforced by the §11 universal ban and the ephemeral-port stub convention.

## Acceptance criteria contributions
- `cargo nextest run -p conductor-emit` and `cargo nextest run --workspace --profile ci` + `cargo clippy --workspace --all-targets -- -D warnings` green with zero retries, plus the carried supply-chain gate `cargo audit --deny warnings` and `cargo deny check` green over an un-drifted `Cargo.lock` (per test-plan §3 `run` + §10 Quality Gates).
- The capture artifact parses agent-side (`serde_json` / `jq -e`) and a negative assertion shows it carries no absolute host path and no internal seam-crate struct name (per test-plan §3 Status endpoint shape + §3 Log format).
- Any capture-driven test binds only an ephemeral `127.0.0.1:0` loopback stub; a new wire golden (if added) is committed fail-don't-write, one file per family × seed, with every `*_time_unix_nano` excluded from the projection (per test-plan §2 Integration row + §7 Golden artifacts).
- The live capture runs through the harness (`agent-run boot` derived-budget preflight → run → `logs`) and is not wired into CI (per test-plan §3 5-command implementation + §9 Live-Pulse scenarios).

## Relevant amendment history
- `2026-06-17-raw-otlp-message-scaffold` (§2 Integration row) — registered the OTLP-egress loopback gRPC `TraceService` stub (tonic over `tokio-stream::TcpListenerStream`, ephemeral `127.0.0.1:0`, new dev-dep `tokio-stream`) as an integration mechanism, explicitly never binding the real `:4317`. This is the sanctioned capture channel this chunk should reuse.
- `2026-08-13-dispatcher-determinism-goldens` (§7 Seed strategies, Golden artifacts row) — added the seeded stream golden families incl. `dispatch_wire__*`, and recorded that the dispatch-tier projection excludes every `*_time_unix_nano` rather than masking it. Governs any new wire golden this chunk commits.
- `2026-08-10-scenario-run-root-span-tree` (§3 Log format) — recorded the self-obs stream's two line variants (event line; span-lifecycle line adding `span` / `span_event` / optional `parent` / allowlisted attrs) and that span attributes are gated by the same `conductor-core::redact` allowlist. Directly governs the `emit.batch` span surface this chunk captures from.
- `2026-06-15-structured-logging-stack` (§3 Log format) — established that the self-obs stream is a SEPARATE artifact from the per-run emission journal and the two schemas must not be conflated. Prevents this chunk from routing capture data into the envelope.
- `2026-08-13-first-live-green-preflight` (§3 `boot` Timeout) — the two derived budgets (in-process canary poll clamped to `[incident_formation].min_canary_poll_seconds`; wall-clock wrapper `warmup_ms/1000 + poll + margin` in both shells, missing contract term = hard exit 2). This is item 4 of the carried operator recipe and is what makes the live capture leg survive warm-up.
