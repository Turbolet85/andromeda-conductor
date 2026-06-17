# Codebase Research — 2026-06-17-raw-otlp-message-scaffold

## Scope
- **Depth:** moderate · **Reads:** 10 (workspace `Cargo.toml`, emit `Cargo.toml`+`lib.rs`, core `error.rs`/`lib.rs`/`phase_spec.rs`/`obs.rs`, timeline `lib.rs`/`scheduler.rs`, report `journal.rs`) · **Globs/Greps:** 3 (emit tree · core/timeline/report src glob · `build.rs` probe). `scripts/code-graph.py query <run_dir> <marker> "<sql>"` is available but direct reads were sufficient.

## Files inspected
- `Cargo.toml` (workspace, full) — `[workspace.dependencies]` **already pins** `opentelemetry-proto = "0.32.0"`, `tonic = "0.14.6"`, `tonic-prost = "0.14.6"`, `prost = "0.14"` (lines 36–40); `resolver=3`, edition 2024, `rust-version=1.94.1`; 8 members. `conductor-emit` only needs to reference these via `.workspace = true`.
- `crates/conductor-emit/Cargo.toml` — placeholder: `[dependencies]` = `conductor-core.workspace = true` only.
- `crates/conductor-emit/src/lib.rs` — a single `//!` doc line; empty seam.
- `crates/conductor-core/src/error.rs:13-27` — the seam-error pattern: `#[derive(Debug, Error)] #[non_exhaustive] pub enum CoreError`, `#[error("…: {0}")]` variants, a `pub type Result<T>` alias; the verdict/error-wall doc (`Err` = harness fault only).
- `crates/conductor-core/src/lib.rs:14-32` — module + `pub use` re-export convention; exports `EmissionSpec`/`PhaseSpec`/`Signal`, `RunRecord`, `now_rfc3339`/`mint_run_id`, `Verdict`/`ReportState`, etc.
- `crates/conductor-core/src/phase_spec.rs:43-70` — `EmissionSpec { signal: Signal }` (`#[non_exhaustive]`) + `Signal { Traces(#[default]), Metrics, Logs }` (closed, low-cardinality). Doc states "the concrete OTLP taxonomy … lands in the Epoch-3 emission seam, which extends `EmissionSpec`" — i.e. THIS seam, reading `Signal::Traces`.
- `crates/conductor-timeline/src/lib.rs` + `scheduler.rs:29-60` — `run_timeline(&PhaseTimeline, seed) -> Result<Vec<PhaseTransition>, TimelineError>`; `#[tracing::instrument(name = "timeline.execute", skip(timeline), fields(phase_count=…))]`; **explicitly "emits nothing — boundaries handed back as values"; the caller owns the runtime**; `TimelineError` is `#[non_exhaustive]` thiserror.
- `crates/conductor-report/src/journal.rs:18-60` — `JournalWriter` (create+append JSONL, flush-per-line); `JournalError` (thiserror, `#[from] std::io::Error`/`serde_json::Error`); records `conductor_core::RunRecord`; tests use `assert_fs::TempDir`.
- `crates/conductor-core/src/obs.rs:109,122` — `mint_run_id()`/`now_rfc3339()` from `std::time::SystemTime` (never tokio's virtual clock); `JsonObsLayer` field-allowlist redaction; the bounded span-name + `#[tracing::instrument]` convention.

## Patterns detected
- **Seam-error enum** (`error.rs:13`, `scheduler.rs:21`, `journal.rs:18`): each seam owns a `#[derive(Debug, thiserror::Error)] #[non_exhaustive]` enum; `Result::Err` is harness-fault-only (the verdict/error wall). → `EmitError` follows exactly: a transport-connect variant (`#[from] tonic::transport::Error`) + a returned-status variant (wrapping `tonic::Status`), both `Err`, never a verdict, never a panic.
- **Manual instrument span** (`scheduler.rs:34`): `#[tracing::instrument(name = "timeline.execute", …, fields(…))]`. → the emit export carries `name = "emit.batch"` (obs-plan §4 + `observability.md` bounded span set), `fields(emission_count, p_id_count)`, logged at `info`.
- **Runtime-agnostic async seam** (`scheduler.rs:8`, timeline `lib.rs:7`): the seam fn is `async` and the caller owns the runtime — no runtime built inside the seam. → emit's `export` is `async`.
- **Forward-compatible declarative emission descriptor** (`phase_spec.rs:44,62`): `EmissionSpec`/`Signal` are `#[non_exhaustive]`; the emit seam reads `Signal::Traces` now and extends the taxonomy later without reshaping the scenario model.
- **`std::time` for wall-clock** (`obs.rs:109,122`): `now_rfc3339`/`mint_run_id` from `SystemTime`; never tokio's virtual clock (journal-relative SLO math depends on it).
- **Test harness** (`journal.rs:62+`, `obs.rs:251+`): `assert_fs::TempDir` for fs; in-process `MakeWriter` capture for streams. → the emit loopback test is a tonic `TraceService` **server stub** over an in-process duplex (or ephemeral `127.0.0.1:0`) asserting the received `ExportTraceServiceRequest` — **NOT** the rmcp stub (that's MCP read-back, Epoch 5) and **NOT** the real `:4317` (reserved for the Epoch-4 port-occupier).

## Conventions to follow
- Module-per-concern with a `pub use` surface in `lib.rs` (`core/lib.rs:14-32`, `timeline/lib.rs:10-15`).
- `//!` module-header doc citing arch sections; minimal inline comments (matches every inspected file).
- Workspace dep refs via `.workspace = true`; OTLP features declared in the crate's own `Cargo.toml`.
- `#[non_exhaustive]` on the seam error enum (and any taxonomy enum).
- Verdict/error wall: `tonic::Status` + transport errors become typed `EmitError` (`Err`), never panic, never a verdict (`error.rs:1-6` doc).

## New files to create
- `crates/conductor-emit/src/error.rs` — `EmitError` (thiserror, `#[non_exhaustive]`): a transport/connect variant + a returned-`Status` variant.
- `crates/conductor-emit/src/message.rs` — raw-type builder(s): assemble `ExportTraceServiceRequest` ← `ResourceSpans` (a `Resource` carrying `service.name`) ← `ScopeSpans` ← `Span` (trace_id/span_id/name/start+end nanos/`Status`) from opentelemetry-proto raw structs.
- `crates/conductor-emit/src/client.rs` — async gRPC egress: a `TraceServiceClient` tonic channel to `127.0.0.1:4317` and `export(req) -> Result<(), EmitError>`; connect refusal → `EmitError` (the "OTLP egress liveness equivalent" at this layer).
- `crates/conductor-emit/tests/egress.rs` — loopback tonic `TraceService` server stub asserts the received request; a refused/torn-down transport surfaces `EmitError`, never a panic.

## Files to modify
- `crates/conductor-emit/Cargo.toml` — add `opentelemetry-proto = { workspace = true, features = ["gen-tonic", "trace"] }`, `tonic.workspace = true`, `prost.workspace = true`, `tracing.workspace = true` (add `tonic-prost.workspace = true` only if the generated client doesn't pull the codec transitively); `[dev-dependencies]` `tokio` (rt+macros) for the loopback stub test.
- `crates/conductor-emit/src/lib.rs` — declare the modules + `pub use` the emit surface (`EmitError`, the message builder(s), the client).
- `Cargo.lock` — regenerated by the build (new gRPC/protobuf/h2/hyper subtree); committed + un-drifted + `cargo audit`/`cargo deny` green before merge.
- **No change to `run_timeline`** — the timeline seam stays pure (it emits nothing by design); per-transition wiring is a later chunk / the CLI runner, per scope's deferral latitude.

## Open questions
- **opentelemetry-proto feature set** — propose enabling `["gen-tonic", "trace"]` only this chunk; `logs` (P-007) / `metrics` land with their chunks. Confirm at P4.
- **Test transport mechanism** — in-process duplex (`tokio::io::duplex` + tonic `Server`/`Endpoint`) vs ephemeral `127.0.0.1:0`; both avoid the reserved `:4317`. Pick the cleaner under tonic 0.14 at P4/implement.
