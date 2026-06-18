# Codebase Research — 2026-06-18-multi-service-topology

## Scope
- **Depth:** moderate · **Reads:** 8 (`span_tree.rs`, `message.rs`, `error.rs`, `lib.rs`, `client.rs`, `tests/error_spans.rs`, `tests/egress.rs`, `latency.rs`+`logs.rs` heads) · **Code-graph queries:** 4 (trace `tree-query-2026-06-18-multi-service-topology.json`)

## Files inspected
- `crates/conductor-emit/src/span_tree.rs` (full) — `error_trace_request(service_name, seed, placement, message)` builds a root→child chain in **one** `ResourceSpans` (one `service.name`). `ErrorPlacement::{Root, DeepChild{depth}}` (pub, re-exported). `gen_id::<N>(rng) -> [u8;N]` is `pub(crate)` — the shared seeded id generator. Determinism unit tests assert on a `shape()` projection `(trace_id, span_id, parent_span_id, status_code)` excluding wall-clock, with both-directions seed checks (same seed ⇒ identical; ≥2 seeds ⇒ divergent). **This is the closest sibling but a DISTINCT primitive:** here chain-length is driven by the error depth; for topology the chain-length is the #services (independent of where the error sits).
- `crates/conductor-emit/src/message.rs` (full) — shared `pub(crate)` primitives: `service_resource(name)` (one `service.name` Resource), `span(name, trace_id, span_id, parent_span_id, status)`, `ok_status()`, `error_status(msg)`, `string_kv`, `unix_nanos()`. `DEFAULT_SERVICE_NAME = "conductor"`. Wall-clock `start=end=unix_nanos()` for plain `span`.
- `crates/conductor-emit/src/latency.rs` (head) — sibling pattern of record: `LatencyProfile::new(...) -> Option<Self>` (validating constructor, `None` on invalid; mirrors `Severity::new`). Builder `latency_trace_request(service_name, seed, &[LatencyOp]) -> ExportTraceServiceRequest`, `let mut rng = ChaCha8Rng::seed_from_u64(seed)`, reuses `gen_id::<16>` (trace_id) + `gen_id::<8>` (span_id) + `service_resource`.
- `crates/conductor-emit/src/logs.rs` (head) — `Severity::new(number) -> Option<Self>` (range-validated); same `service_resource`/`-> Option` discipline.
- `crates/conductor-emit/src/client.rs` (full) — `TraceEmitter::{connect, from_channel, export}`; `export` is `#[tracing::instrument(name="emit.batch", fields(emission_count=count_spans))]`. **`count_spans` already flattens `resource_spans → scope_spans → spans`** — a multi-`ResourceSpans` request needs **no emitter change** and the existing `emit.batch` self-obs span already covers it.
- `crates/conductor-emit/src/error.rs` (full) — `EmitError::{Transport(#[from] tonic::transport::Error), Status(#[from] tonic::Status)}`, `#[non_exhaustive]`. Multi-service emission introduces **no new fault variant**.
- `crates/conductor-emit/src/lib.rs` (full) — flat re-export hub; the only modify target outside new files: add `mod topology;` + `pub use topology::{...}`.
- `crates/conductor-emit/tests/{error_spans,egress}.rs` (full) — the canonical loopback integration pattern: `CapturingService { last: Arc<Mutex<Option<ExportTraceServiceRequest>>> }`, `start_stub()` binds `127.0.0.1:0` (ephemeral — **never `:4317`**), `#[tokio::test(flavor="current_thread")]`, assert on the captured request; plus the `refused_transport_surfaces_emit_error` (`http://127.0.0.1:1` ⇒ `Err(EmitError::Transport(_))`) case to copy.

## Graph impact (from `tree-query-...json`)
- **`span_tree/error_trace_request`** — referenced only within `conductor-emit` (lib re-export + its own defs/tests + `tests/error_spans.rs`); **zero production or cross-crate callers**. New sibling builder ⇒ no blast radius; existing builder left untouched.
- **`message/service_resource`** — used by message/span_tree/exception/latency/logs (9 refs, all in-crate) — the canonical per-service `Resource` builder to call **once per service** in the topology.
- **`span_tree/gen_id`** — used by span_tree/exception/latency (8 refs, in-crate) — reuse for `trace_id` (`::<16>`) + per-service `span_id` (`::<8>`).
- **`crate_edges` touching `conductor-emit`** — **empty**: no resolved cross-crate edges in/out. `conductor-emit` is a leaf emission-primitive crate; the multi-service builder has no downstream consumers to break (faults/verify wire in later epochs).

## Patterns detected
- **Validating constructor → `Option<Self>`** (`latency.rs:32`, `logs.rs:32`): typed inputs self-validate in-crate, returning `None` on violation — emit primitives self-validate; `conductor-core` garde is the later scenario-wiring validator.
- **Free builder over reused `pub(crate)` primitives** (`latency.rs:72`, `span_tree.rs:37`): `(…names, seed, &[spec]) -> ExportTraceServiceRequest`; one `ChaCha8Rng::seed_from_u64(seed)`; no `Result` (only transport is `EmitError`).
- **Shape-projection determinism** (`span_tree.rs:100-159`): determinism asserted on `(ids, linkage, status)` excluding wall-clock `start/end`; paired both-directions (same-seed-identical + ≥2-seeds-diverge).
- **Loopback gRPC server stub on ephemeral port** (`tests/egress.rs:34`, `tests/error_spans.rs:36`): `TcpListenerStream` + `TraceServiceServer`, `current_thread` test; never binds `:4317`.

## Conventions to follow
- **Crate**: `conductor-emit` only (arch §Inherited Defaults, crate-per-seam). New module `topology.rs`; re-export through `lib.rs`.
- **Seeded identity** (`ChaCha8Rng`/`gen_id`): `trace_id` once, `span_id` per service; cross-platform-stable (`seed_from_u64`).
- **Wall-clock only in `start/end`** via the existing `span()` (`message.rs:55`); determinism governs ids+linkage+status only.
- **No new self-obs span** — pure function; `emit.batch` (`client.rs:43`) already covers export; obs bounded span-name set unchanged (`.claude/rules/observability.md`).
- **Tests**: per-seam `cargo nextest run -p conductor-emit`; loopback stub on ephemeral port; both-directions seed assertion; `refused_transport ⇒ EmitError::Transport`.

## New files to create
- `crates/conductor-emit/src/topology.rs` — `ServiceTopology` (validated ≥2 distinct services) + `service_topology_request(...)` builder producing ≥2 `ResourceSpans` (one per service) under one seeded `trace_id` with cross-service `parent_span_id` linkage and optional cross-service `Status.Code=ERROR` placement (reusing `ErrorPlacement`). In-crate `#[cfg(test)] mod tests`.
- `crates/conductor-emit/tests/multi_service_topology.rs` — loopback `TraceService` stub asserting: ≥2 distinct `service.name` ResourceSpans; one shared `trace_id`; valid cross-service `parent_span_id` linkage; cross-service error placement; refused-transport ⇒ `EmitError`.

## Files to modify
- `crates/conductor-emit/src/lib.rs` — add `mod topology;` and `pub use topology::{ServiceTopology, service_topology_request};` (re-`ErrorPlacement` already exported via `span_tree`).

## Open questions
- **Out-of-range error placement** (minor, implement-level): `ErrorPlacement::DeepChild{depth}` with `depth ≥ topology.len()` — saturate to the deepest service (documented, like `timed_span`'s `saturating_add`) vs validate the (topology, placement) pair. Recommend saturate + document; not user-facing. Resolve at P4/implement.
