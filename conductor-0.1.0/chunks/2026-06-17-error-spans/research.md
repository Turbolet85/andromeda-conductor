# Codebase Research — 2026-06-17-error-spans

## Scope
- **Depth:** moderate · **Reads:** 7 (full conductor-emit seam + Cargo.toml + egress test + code-graph-views head) · **Globs/Greps:** 3 (emit `*.rs` glob; timeline RNG grep; cross-crate `conductor_emit` grep) · **code-graph:** 1 query (not cold-start — populated tree.db)

## Files inspected
- `crates/conductor-emit/src/message.rs` (full) — the current builder. `trace_request(service_name, span_name)` → a single `OK` span. **IDs are hardcoded** `trace_id: vec![1; 16]` / `span_id: vec![1; 8]` (degenerate placeholder — fine for one span, unusable for a multi-span tree where children need distinct span_ids). Helpers: `service_resource()` (stamps `service.name` KeyValue), `ok_span(name)`, `unix_nanos()` (← `SystemTime::now()`). Imports `status::StatusCode`, `span::SpanKind`, `ResourceSpans/ScopeSpans/Span/Status` already.
- `crates/conductor-emit/src/client.rs` (full) — `TraceEmitter::{connect, from_channel, export}`; `DEFAULT_OTLP_ENDPOINT = "http://127.0.0.1:4317"`. `export()` carries the `#[tracing::instrument(name = "emit.batch", …, fields(emission_count = count_spans(&request)))]` span. **`count_spans()` already sums across all `scope_spans[].spans` — multi-span requests are already supported by the egress path; no client.rs change needed.**
- `crates/conductor-emit/src/error.rs` (full) — `EmitError` is `#[derive(thiserror::Error)] #[non_exhaustive]` with `Transport(#[from] tonic::transport::Error)` + `Status(#[from] tonic::Status)`. Doc-comment explicitly: "later Epoch-3 chunks extend the fault surface." (No new variant expected this chunk — an ERROR-status span is a normal emitted value, not a harness fault.)
- `crates/conductor-emit/src/lib.rs` (full) — re-exports `TraceEmitter, DEFAULT_OTLP_ENDPOINT, EmitError, trace_request, DEFAULT_SERVICE_NAME`. New public builder + placement type get added here.
- `crates/conductor-emit/Cargo.toml` (full) — deps: `conductor-core`, `opentelemetry-proto[gen-tonic,trace]`, `tonic`, `thiserror`, `tracing`. dev: `tokio[macros,rt,net]`, `tokio-stream[net]`. **No `rand_chacha`/`rand_core` yet** — must add the dep edges (both are already `[workspace.dependencies]`, so this is an edge, not a new external crate → no new audit/deny surface).
- `crates/conductor-emit/tests/egress.rs` (full) — the loopback pattern to reuse: `CapturingService` impl of `TraceService` storing the `last: Arc<Mutex<Option<ExportTraceServiceRequest>>>`; `start_stub()` binds `127.0.0.1:0`, serves over `TcpListenerStream`, returns `(addr, captured)`; tests under `#[tokio::test(flavor = "current_thread")]`. Asserts reach into `received.resource_spans[0].scope_spans[0].spans[0]`.
- `crates/conductor-timeline/src/scheduler.rs` (grep) — **the seeded-RNG pattern to mirror**: `use rand_chacha::ChaCha8Rng; use rand_core::{RngCore, SeedableRng};` → `let mut rng = ChaCha8Rng::seed_from_u64(seed);` (line 43) → helper signature `jittered_gap(base, bound_ms, rng: &mut ChaCha8Rng)` (line 64). `rand_chacha.workspace = true` in timeline's Cargo.toml.

## Graph impact (code-graph query → `tree-query-2026-06-17-error-spans.json`)
- **`EmitError`** — 6 refs · **`trace_request()` / `TraceEmitter` / `DEFAULT_SERVICE_NAME`** — 5 refs each · all other symbols ≤3. **Every ref resolves inside `conductor-emit` itself or its `tests/egress.rs`.** Cross-crate grep confirms: `conductor_emit` appears only in its own `Cargo.toml` + test. **Zero external callers → the change is purely additive within the seam, no blast radius** (no other crate depends on conductor-emit yet; run_timeline wiring is deferred, consistent with the scaffold chunk).

## Patterns detected
- **Field-by-field raw OTLP assembly** (`message.rs:21-62`): build `ExportTraceServiceRequest → ResourceSpans → ScopeSpans → Span` by hand, `..Default::default()` for unspecified fields, `StatusCode::Ok as i32` / `SpanKind::Internal as i32` for enum fields. Byte-level control is the design intent.
- **Seeded determinism via injected RNG** (`scheduler.rs:43,64`): `ChaCha8Rng::seed_from_u64(seed)` constructed at the entrypoint, then `&mut ChaCha8Rng` threaded into pure helpers. This is the canonical "randomness as input, not global entropy" shape every extract asked for.
- **Loopback `CapturingService` stub** (`egress.rs:34-47`): ephemeral `127.0.0.1:0`, never `:4317`; capture-and-assert on the received protobuf. (Tier-2 rule in `testing.md`.)
- **`#[non_exhaustive]` forward-extensible `EmitError`** (`error.rs:10`) — extend only if a genuine new harness fault appears (not for ERROR-status, which is a value).
- **Bounded `emit.batch` self-obs span** (`client.rs:40`) — already records `emission_count`; no per-span/high-cardinality naming.

## Conventions to follow
- **Randomness/IDs are builder INPUT** — take a seed (`u64`) or `&mut ChaCha8Rng`; never read entropy/clock for IDs (arch §Determinism RNG; obs/security/tests extracts; `scheduler.rs:43`).
- **Raw proto structs + `..Default::default()`** for new fields (`message.rs` idiom).
- **Status via `StatusCode::Error as i32`**, message string on `Status.message` (OTel semantic convention; P-005).
- **Test both determinism directions** — same seed ⇒ identical trace_id/span_ids AND ≥2 fixed seeds ⇒ divergent ids (`testing.md` 2026-06-16 learning); loopback stub only, never `:4317` (`testing.md` 2026-06-17).
- **No OTel SDK / no W3C `traceparent` for self-obs** — `run_id` is the only correlation key; the product `trace_id`/`parent_span_id` are *emitted payload*, not self-obs correlation (`observability.md`; obs extract).
- **Sanitization** — no host paths / internal struct names in any emitted/logged artifact (`security.md`; obs §11).

## New files to create
- `crates/conductor-emit/src/span_tree.rs` (name TBD in P4 — could be `trace.rs`/`error_span.rs`) — the span-tree assembly + ERROR-status construction + root-vs-child placement selector, built on a seeded `ChaCha8Rng`. *(Alternative: extend `message.rs` in place — P4 chooses module split vs in-file.)*

## Files to modify
- `crates/conductor-emit/src/message.rs` — generalize `ok_span()` into a status-parameterized private span constructor (status code + message + injected trace_id/span_id/parent_span_id); keep `trace_request` working.
- `crates/conductor-emit/src/lib.rs` — re-export the new builder fn + placement type.
- `crates/conductor-emit/Cargo.toml` — add `rand_chacha.workspace = true` + `rand_core.workspace = true` dep edges.
- `crates/conductor-emit/tests/` — a new `error_spans.rs` (or extend `egress.rs`) asserting a root-error trace and a deep-child-error trace land `Status.Code=ERROR` at the expected span with well-formed parent/child linkage + id determinism.

## Open questions
1. **ID-gen API shape** — `seed: u64` param (construct `ChaCha8Rng` inside the trace builder) vs `&mut ChaCha8Rng` param (caller owns the RNG). *Lean:* take `seed: u64` at the public builder entrypoint, thread `&mut rng` to per-span id helpers — mirrors `scheduler.rs:43`+`64` exactly and keeps the public surface a pure `fn(seed, …) -> request`. → resolve P4.
2. **Timestamp determinism** — `unix_nanos()` reads wall-clock, so a full-byte `insta` golden over the whole request would flake on `start/end_time_unix_nano`. *Lean:* assert *structurally* (span count, parent/child topology, `Status.Code=ERROR` at the expected span index) + *id-determinism* (same seed ⇒ identical ids; divergent seeds ⇒ different ids), NOT a byte-golden over timestamps — matches the determinism-replay precedent (golden the seeded shape, not wall-clock). → resolve P4.
3. **Unify vs alongside** — refactor `ok_span`/`trace_request` onto the new parameterized span constructor, or add the error-tree builder alongside the untouched `trace_request`? *Lean:* extract a shared private `span(...)` constructor, keep `trace_request` as the public OK-baseline entry, add `error_trace_request`(or similarly named) as the new public entry. → resolve P4.
