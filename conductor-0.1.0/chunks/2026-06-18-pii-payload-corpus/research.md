# Codebase Research — 2026-06-18-pii-payload-corpus

## Scope
- **Depth:** moderate · **Reads:** 8 (message/exception/logs/span_tree/topology/lib/Cargo + 1 integration test) · **Globs/Greps:** 1 ls + 1 code-graph query
- Mature codebase; this is a new module in `conductor-emit` mirroring six existing sibling builders. No cold-start.

## Files inspected
- `crates/conductor-emit/src/message.rs` (full) — owns the shared `pub(crate)` primitives the corpus composes: `string_kv(key,value) -> KeyValue` (**the only attribute value-type Conductor emits — string-valued**, msg.rs:43), `service_resource(name)` (:35), `span(name,trace_id,span_id,parent,status)` (:55), `span_with_events(…,events)` (:68), `ok_status`/`error_status` (:116/:123), `unix_nanos()` wall-clock (:134). Spans are built field-by-field; `Span.attributes` defaults empty — **no current helper sets span attributes**.
- `crates/conductor-emit/src/exception.rs` (full) — the exceptions embedding site. `exception_event(spec)` (:153) puts `exception.type`/`exception.message`/`exception.stacktrace` on a span `Event` via `string_kv`; `render_stacktrace` (:166) formats frames. `exception_trace_request(service,seed,spec)` builds one ERROR root span carrying the event. Stacktrace test asserts no absolute host path (:315). This is the template for PII-in-exceptions (message + stacktrace are free-text string fields).
- `crates/conductor-emit/src/logs.rs` (full) — the logs embedding site. `severity_logs_request(service,&[Severity])` (:50) → `log_record(severity)` (:67) sets `LogRecord.body` from a fixed `LOG_BODY` const (:21) via `AnyValue::StringValue`; **`LogRecord.attributes` is unused (defaults empty)**. PII-in-logs goes in `body` and/or `attributes`. `Severity::new` is the self-validating-constructor pattern (`Option`, 1..=24).
- `crates/conductor-emit/src/span_tree.rs` (full) — `gen_id::<N>(&mut ChaCha8Rng)` (:80) is the seeded id generator (`rng.fill_bytes`); `ErrorPlacement` (:15) reused by topology. Shows the canonical seeded-builder body: `ChaCha8Rng::seed_from_u64(seed)` → `gen_id::<16>`/`gen_id::<8>`.
- `crates/conductor-emit/src/topology.rs` (full) — **the newest sibling and the closest pattern to mirror**: self-validating struct `ServiceTopology` (`new` → `Option`, :30), a builder composing `service_resource`/`span`/`gen_id` and constructing `ResourceSpans` inline (:99), and an in-crate test module using a **shape-projection** (`SpanShape`, ids+linkage+status, excludes wall-clock, :151) with both-directions seed assertions (`same_seed_reproduces_identical_shape` :258 / `different_seeds_diverge` :266).
- `crates/conductor-emit/src/lib.rs` (full) — `mod` decls (:11–18) + `pub use` re-exports (:20–27), one line per module. The corpus adds `mod pii;` + a `pub use pii::{…}` line here.
- `crates/conductor-emit/Cargo.toml` (full) — **`rand_chacha` + `rand_core` are already deps (:14–15); `tokio` + `tokio-stream` are already dev-deps (:18–19). NO new dependency needed** (closes the security extract's cargo-audit-on-new-PRNG concern).
- `crates/conductor-emit/tests/multi_service_topology.rs` (full) — the loopback integration-test pattern: `CapturingService` impl of `TraceServiceServer` on `TcpListener::bind("127.0.0.1:0")` via `TcpListenerStream`, `#[tokio::test(flavor="current_thread")]`, `TraceEmitter::connect(http://{addr})` → `.export(...)`, then assert on the captured request. Plus the `refused_transport_surfaces_emit_error` negative (:120) asserting `EmitError::Transport`.

## Graph impact (from the code-graph query → `tree-query-2026-06-18-pii-payload-corpus.json`)
- **`message::service_resource` / `string_kv` / `span_with_events` / `gen_id` / `unix_nanos`** — already called from `exception.rs`, `latency.rs`, `logs.rs`, `span_tree.rs`, `topology.rs`, `message.rs` (every emit sibling). Adding `pii.rs` as one more caller is **purely additive** — no existing caller changes, zero blast radius on the primitives.

## Patterns detected
- **Seeded builder** (`span_tree.rs:43`, `topology.rs:69`, `exception.rs:133`): `let mut rng = ChaCha8Rng::seed_from_u64(seed); let trace_id = gen_id::<16>(&mut rng);` — identity from the seed, never the clock.
- **Self-validating constructor returning `Option`** (`topology.rs:30`, `logs.rs:32`): reject illegal input at construction; `conductor-core` garde is the authoritative validator once it reaches the scenario model (deferred).
- **String-only attributes** (`message.rs:42`): every emitted attribute is `string_kv` — PII payloads are string values on span attributes / log body+attributes / exception event attributes.
- **Shape-projection determinism test** (`topology.rs:151`, `exception.rs:231`, testing.md 2026-06-17 learning): assert determinism on a projection that EXCLUDES wall-clock `*_time_unix_nano`; pair same-seed-identical with ≥2-seeds-divergent.
- **Loopback gRPC capture stub** (`tests/multi_service_topology.rs:39`, testing.md 2026-06-17 learning): ephemeral `127.0.0.1:0`, NEVER `:4317`.

## Conventions to follow
- Module doc-comment (`//!`) citing the P-IDs + the architecture decision, like every sibling (`topology.rs:1`, `logs.rs:1`).
- Re-export the public API from `lib.rs` (one `pub use` line); keep helpers `pub(crate)` (`lib.rs:20-27`).
- Transport faults are `EmitError` (`Result::Err`), never panic; verdict/error wall (`tests/multi_service_topology.rs:120`).
- No self-obs logging of payload VALUES; emit-side modules log nothing per-payload (obs `emit.batch`/`emit.logs_batch` cover egress with counts) — keep corpus values off `tracing` + out of run-report artifacts (security/obs extracts).
- Doctests: keep `cargo test --doc` at 0 unless a `///` example is added (workspace gate).

## New files to create
- `crates/conductor-emit/src/pii.rs` — `PiiCategory` enum over exactly the seven P-047 categories (email · JWT · bearer · API key · credit card · SSN · secret `key=value`), a seeded corpus generator (`seed_from_u64`/`ChaCha8Rng`) producing one structurally-valid synthetic value per category, and request-builder(s) embedding the corpus across the three signal types (span attributes · log body/attributes · exception event message/stacktrace).
- `crates/conductor-emit/tests/pii_payload_corpus.rs` — loopback `TraceServiceServer` + `LogsServiceServer` capture stubs (mirror `multi_service_topology.rs` and the logs integration test) asserting each of the seven categories reaches the wire on its signal type(s) + a refused-transport `EmitError` negative.

## Files to modify
- `crates/conductor-emit/src/lib.rs` — add `mod pii;` and a `pub use pii::{…}` re-export line.
- `crates/conductor-emit/src/message.rs` — **conditional (P4 decision):** add a `pub(crate) span_with_attributes(…, attributes: Vec<KeyValue>)` helper for the spans embedding site (symmetry with `span_with_events`), OR set `Span.attributes` inline in `pii.rs` after calling `span()`. No other change.

## Open questions
1. **Spans embedding site** — `span`/`span_with_events` don't expose span attributes; PII-in-spans needs `Span.attributes` set. Add a `pub(crate) span_with_attributes` helper in `message.rs` (mirrors `span_with_events`) or set `.attributes` inline in `pii.rs`? (Recommend the helper for symmetry; resolve at P4 — low stakes either way.)
2. **Builder API shape** — spans+exceptions are `ExportTraceServiceRequest`; logs are `ExportLogsServiceRequest` (two different request types). One trace-side builder (spans + exception event) + one logs-side builder, each taking `seed` + the categories to embed? Or a single struct (`PiiCorpus`) exposing per-signal builders? (Recommend a `PiiCorpus`/`PiiCategory` value type + `{trace,logs}` builders; resolve at P4.)
3. **Credit-card structural validity** — a card-number detector keys on Luhn; the seeded digits must carry a computed Luhn check digit (not a random 16th digit), else the payload isn't "structurally valid for its category." (Resolve in implementation; flagged so the plan's acceptance criterion is explicit.)
