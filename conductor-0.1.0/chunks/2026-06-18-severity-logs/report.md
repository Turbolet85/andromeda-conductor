# Report — 2026-06-18-severity-logs

**Chunk:** Severity logs — OTLP log records (LogsEmitter + raw ExportLogsServiceRequest) with controlled SeverityNumber + matching SeverityText across the 17 WARN→ERROR boundary (conductor-emit, P-007)
**Date:** 2026-06-18
**Commits:** none new since last_wrap (chunk is uncommitted; this wrap creates the chunk commit). Prior HEAD `a36ebf8` = the exception-events chunk.

## Changes (structured — detectors read this)
- **Files:** NEW `crates/conductor-emit/src/logs.rs` · NEW `crates/conductor-emit/tests/severity_logs.rs` · MOD `crates/conductor-emit/Cargo.toml` · MOD `crates/conductor-emit/src/client.rs` · MOD `crates/conductor-emit/src/lib.rs`
- **Symbols / APIs:** NEW public `Severity` (i32-newtype: `new(i32) -> Option<Self>` range-checked `1..=24`, `number()`, `text()` over a canonical OTel short-name table) · NEW public `severity_logs_request(service_name: &str, severities: &[Severity]) -> ExportLogsServiceRequest` · NEW public `LogsEmitter` (`connect`/`from_channel`/`export` over `LogsServiceClient<Channel>`, mirrors `TraceEmitter`). All re-exported from `conductor-emit`. NEW self-obs span name `emit.logs_batch` (field `record_count`) on `LogsEmitter::export`. **No IPC method / endpoint / port / socket / env-var / Tauri-command change** — loopback `:4317` egress unchanged, no new inbound listener.
- **Crates / modules:** `conductor-emit` gains `mod logs`; no crate added/removed; reuses `message.rs` `pub(crate)` helpers (`service_resource`/`unix_nanos`) + `EmitError` (no new variant).
- **Dependencies:** NONE added. `opentelemetry-proto` feature list gained `"logs"` (a feature of the already-present crate — `Cargo.lock` UN-DRIFTED, verified `git status` clean on `Cargo.lock`). `opentelemetry_sdk v0.32.1` remains a DORMANT transitive of `opentelemetry-proto` (default features; never initialized — self-obs stays tracing-JSON-only; the `default-features = false` trim is DEFERRED by user decision this session).
- **Schema / config:** NONE — no migrations, no config keys, no `runs.db` / JSONL-journal / run-report-envelope change. Standalone emit primitive, NOT timeline/journal-wired (zero external callers of `conductor-emit` confirmed via code-graph; matches sibling emit chunks' altitude).
- **Coverage of new surfaces:**
  - `conductor-emit::{severity_logs_request, LogsEmitter}` (OTLP logs-egress emission primitive) → validation: `Severity::new` range-checks `SeverityNumber ∈ 1..=24` (garde **n/a** — Rust-API constructor over caller code, NOT an external-input deserialize / config / path / MCP-stdout boundary) · instrumentation: `emit.logs_batch` tracing span (`record_count`) ✓ · PII: body is a fixed benign const `"conductor severity probe"`, no host paths / struct names ✓ · tests: unit (5 in `logs.rs`) + integration (2 in `tests/severity_logs.rs`) ✓ · a11y: **n/a** (no UI) · tokens: **n/a** (no UI)

## Deviations from intent
- **Logs egress span name `emit.logs_batch`** — the plan left the exact name to implement ("mirror `TraceEmitter`'s `emit.batch` convention"); chose the obs-extract's recommended low-cardinality bounded-set sibling so logs egress is distinguishable from trace egress in self-obs. *(Note: this is a NEW span name not yet in obs-plan §4's enumerated bounded set — candidate routine doc-reconciliation.)*
- **Fixed benign body** (`"conductor severity probe"`), not caller-parametrized — the plan said "benign string body" (singular); P-007 is severity-boundary control, not body content, so no body parameter (avoids premature abstraction per code-writing-discipline).
- **Body `AnyValue` built inline in `logs.rs`** rather than extracting a shared helper from `message.rs` — kept `message.rs` untouched (NOT in Files-to-modify); a 3-line literal is in-scope ("three similar lines beat a premature trait").
- **Seedless builder** (as the plan recommended): determinism proven by reproducibility (`same_inputs_reproduce_identical_shape`) + boundary-distinctness; NO seed-divergence test (no seed participates — severity is spec-controlled).

## Decisions & corrections
- **User decision (this session, /andromeda-phase P4 AskUserQuestion):** keep the `opentelemetry-proto` `default-features = false` trim **DEFERRED** — this chunk adds only the `"logs"` feature; the trim stays a tracked follow-up for a later emission chunk. The dormant `opentelemetry_sdk` remains in the tree (playbook rule 2026-06-17 confirms a dormant transitive is not drift).
- **Resolved in planning (research code-graph query):** journal/timeline wiring DEFERRED — `conductor-emit` has zero external callers, so there is no consumer to journal into; severity-logs is a standalone primitive like its 3 sibling emit chunks.
- **Convention reaffirmed:** the loopback gRPC stub test (now `LogsService` too) binds ephemeral `127.0.0.1:0`, never the real `:4317` (reserved for the Epoch-4 port-occupier).

## Outcome
- **Acceptance criteria: all met** — `16/WARN4` and `17/ERROR` records constructible + distinguishable (canonical text derived from number); `LogsEmitter` loopback egress with typed `EmitError` wall (refused transport ⇒ `EmitError::Transport`, `tonic::Status` a typed input); `std::time` stamps (not virtual clock); determinism shape-projection stable; loopback stub never binds `:4317`; `Cargo.lock` un-drifted.
- **Gates green:** `cargo nextest run -p conductor-emit` (25/25) · `cargo clippy -p conductor-emit --all-targets -- -D warnings` (clean) · `cargo test -p conductor-emit --doc` (0 doctests, ok) · `cargo nextest run --workspace --profile ci` (106/106, +7) · `cargo audit` (pass) · `cargo deny check` (advisories/bans/licenses/sources ok).
- **Tests use** cargo-nextest (runner, on-spec) + plain `#[test]` / `#[tokio::test(flavor="current_thread")]`, mirroring the sibling `egress.rs` / `error_spans.rs` (no `rstest` — these tests are not parametrized).
- **Smoke:** skipped — no boot-path change (`conductor-emit` is a library; no binary / entry-point touched; Test Commands include no `agent-run.sh`).
