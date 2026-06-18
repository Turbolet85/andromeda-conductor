# Codebase Research — 2026-06-18-severity-logs

## Scope
- **Depth:** moderate · **Reads:** 8 (`client.rs`, `message.rs`, `error.rs`, `span_tree.rs`, `lib.rs`, `tests/egress.rs`, `conductor-emit/Cargo.toml`, workspace `Cargo.toml`) · **Code-graph queries:** 2

## Files inspected
- `crates/conductor-emit/src/client.rs` (full) — `TraceEmitter` over `TraceServiceClient<Channel>`: `connect(endpoint) -> Result<Self, EmitError>` (via `Endpoint::from_shared(...).connect()`), `from_channel(Channel) -> Self`, `export(req) -> Result<(), EmitError>` with `#[tracing::instrument(name = "emit.batch", skip_all, fields(emission_count = …))]`. `DEFAULT_OTLP_ENDPOINT = "http://127.0.0.1:4317"`. **The LogsEmitter template.**
- `crates/conductor-emit/src/message.rs` (full) — shared raw-struct primitives, all `pub(crate)`: `service_resource(name) -> Resource`, `string_kv(k,v) -> KeyValue` (string is "the only attribute value-type Conductor emits"), `unix_nanos() -> u64` (`SystemTime::now()` — wall-clock, **not** virtual clock). `trace_request(svc, span)` shows the `ExportTraceServiceRequest { resource_spans: vec![ResourceSpans { resource: Some(service_resource(..)), scope_spans: vec![ScopeSpans { … }] }] }` nesting. **logs.rs reuses these helpers verbatim (same crate → `pub(crate)` reachable; no change to message.rs needed).**
- `crates/conductor-emit/src/error.rs` (full) — `EmitError` (`#[non_exhaustive]`): `Transport(#[from] tonic::transport::Error)` + `Status(#[from] tonic::Status)`. **Reusable for logs AS-IS — no new variant** (both transport-refused and collector-status faults are already covered; the `#[non_exhaustive]` comment explicitly anticipates Epoch-3 reuse).
- `crates/conductor-emit/src/span_tree.rs` (full) — the seeded-determinism + shape-projection reference: `ChaCha8Rng::seed_from_u64(seed)` + `gen_id::<N>(&mut rng)` for identity bytes; test helper `shape()` projects `(ids, status)` EXCLUDING wall-clock; tests `same_seed_reproduces_identical_shape` + `different_seeds_diverge`. **NB: `error_trace_request` takes a seed (rich span identity); `trace_request` does NOT (fixed ids). Severity-log records have thin identity (no parent chain) — the seed's role here is minimal (see Open questions).**
- `crates/conductor-emit/tests/egress.rs` (full) — the loopback-stub template: `CapturingService` impl `TraceService` (captures `last` request into an `Arc<Mutex<Option<…>>>`), `start_stub()` binds `127.0.0.1:0` + `TcpListenerStream` + `Server::builder().add_service(TraceServiceServer::new(svc))`, driven under `#[tokio::test(flavor="current_thread")]`. Two tests: well-formed export + `refused_transport_surfaces_emit_error` (connect `http://127.0.0.1:1` ⇒ `Err(EmitError::Transport(_))`). **The `tests/severity_logs.rs` template — swap Trace→Logs types.**
- `crates/conductor-emit/src/lib.rs` (full) — `mod {client,error,exception,message,span_tree}`; public re-exports. **Adds `mod logs;` + re-exports `LogsEmitter`, the logs builder, and the severity helper type.**
- `crates/conductor-emit/Cargo.toml` — `opentelemetry-proto = { workspace = true, features = ["gen-tonic", "trace"] }`. **MUST become `["gen-tonic", "trace", "logs"]`.** The `logs` feature gates already-vendored proto-gen types (no new external crate) ⇒ `Cargo.lock` stays un-drifted.
- workspace `Cargo.toml` — `opentelemetry-proto = "0.32.0"` (line 37), **no `default-features = false`** → the dormant transitive `opentelemetry_sdk` rides in via defaults. This is the deferred-trim site (Open question 1).

## Graph impact (code-graph query → `tree-query-2026-06-18-severity-logs.json`)
- **External callers INTO `conductor-emit`: `[]` (ZERO).** No other crate references any `conductor-emit` symbol — the timeline/cli do not yet wire emission. ⇒ (1) the new `LogsEmitter` + logs-builder public API is **purely additive, zero blast radius** (nothing that compiles today can break); (2) **resolves scope boundary #2** — severity-logs matches its siblings' altitude as a **standalone emit primitive; journal/timeline wiring is deferred** (there is no consumer to journal into yet).
- conductor-emit: 103 indexed symbols (well-indexed; not cold-start). Internal helpers (`service_resource`/`string_kv`/`unix_nanos`/`gen_id`) are intra-crate `pub(crate)` — reachable from a new `logs.rs` with no visibility change.

## Patterns detected
- **Thin gRPC emitter** (`client.rs:20-45`): a struct wrapping `XServiceClient<Channel>` with `connect`/`from_channel`/`export`, `#[tracing::instrument(name="emit.batch")]` on egress. Mirror as `LogsEmitter` over `LogsServiceClient<Channel>`.
- **Field-by-field raw message nesting** (`message.rs:22-33`): `ExportXServiceRequest { resource_x: vec![ResourceX { resource: Some(service_resource(..)), scope_x: vec![ScopeX { … }] }] }`. Logs mirror: `ExportLogsServiceRequest → ResourceLogs → ScopeLogs → LogRecord`.
- **Shape-projection determinism test** (`span_tree.rs:98-112`): assert on an identity/content projection that EXCLUDES wall-clock `*_time_unix_nano`. For logs the projection is `(severity_number, severity_text, body)`.
- **Loopback capturing stub** (`egress.rs:18-47`): ephemeral `127.0.0.1:0`, capture into `Arc<Mutex<Option<Req>>>`, assert on the captured request.

## Conventions to follow
- **Reuse `pub(crate)` helpers** (`message.rs:35,43,109`): `service_resource`, `string_kv`, `unix_nanos` — do not duplicate.
- **`EmitError` reuse** (`error.rs:11`): no new variant; transport/status faults already typed. Verdict/error wall: `Result::Err` only for harness faults, `tonic::Status` is a typed input never a panic.
- **Wall-clock from `std::time`** (`message.rs:109`): `LogRecord.time_unix_nano` / `observed_time_unix_nano` via `unix_nanos()` — NEVER tokio's virtual clock.
- **Never bind real `:4317` in tests** (`egress.rs:1-3`): ephemeral `127.0.0.1:0` only (`:4317` reserved for the Epoch-4 port-occupier).
- **Proto types** (logs feature): `opentelemetry_proto::tonic::collector::logs::v1::{logs_service_client::LogsServiceClient, ExportLogsServiceRequest, ExportLogsServiceResponse, logs_service_server::{LogsService, LogsServiceServer}}`; `opentelemetry_proto::tonic::logs::v1::{ResourceLogs, ScopeLogs, LogRecord, SeverityNumber}`; body via `common::v1::AnyValue` (string body using the existing `any_value::Value::StringValue` shape).

## New files to create
- `crates/conductor-emit/src/logs.rs` — the raw `ExportLogsServiceRequest` builder with controlled `SeverityNumber` + canonical matching `SeverityText`, a benign string `body`, and `unix_nanos()` timestamps; a small severity helper (a `Severity` newtype/enum mapping number⇄canonical text, so a record can't carry a mismatched/out-of-range pair). The 17-boundary helpers: construct records at `16/WARN` and `17/ERROR`.
- `crates/conductor-emit/tests/severity_logs.rs` — loopback `LogsService` stub (mirror `egress.rs`): well-formed export captured; `16/WARN` vs `17/ERROR` boundary records distinguishable in the captured request; refused transport ⇒ `EmitError::Transport`.

## Files to modify
- `crates/conductor-emit/Cargo.toml` — add `"logs"` to `opentelemetry-proto` features.
- `crates/conductor-emit/src/client.rs` — add `LogsEmitter` (mirror `TraceEmitter`; generalize the module doc to "trace + logs egress"). *(Placement recommendation — alongside `TraceEmitter` in the shared egress module; reuses `Endpoint`/`Channel` plumbing. Final placement is an implement-time call.)*
- `crates/conductor-emit/src/lib.rs` — `mod logs;` + re-export `LogsEmitter`, the logs builder fn, and the severity helper type.

## Open questions
1. **`opentelemetry-proto` `default-features = false` trim** (the deferred follow-up): this chunk already edits the dep's feature list, so it is the natural site to drop the dormant transitive `opentelemetry_sdk`. Risk: dropping defaults may remove a feature the build needs and WILL legitimately change `Cargo.lock` (dep removed). **→ P4 AskUserQuestion: fold in vs keep deferred.** Default lean: keep minimal (add `"logs"` only), defer the trim, unless the user wants it folded.
2. **Seed participation:** a severity `LogRecord` has thin identity (no span chain). The primitive is fully determined by its spec (severity/text/body) ⇒ deterministic by construction without an RNG (like `trace_request`, not `error_trace_request`). Recommendation: seedless builder; determinism criterion = "same inputs ⇒ identical shape-projection". A seed-divergence (both-directions) test applies ONLY if a seed is later added for optional trace-correlation identity. (HOW-leaning; resolve at implement.)
