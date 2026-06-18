# Report — 2026-06-17-error-spans

**Chunk:** Error spans — `Status.Code=ERROR` construction + intra-trace root-vs-child span placement (parent_span_id linkage) on the raw OTLP scaffold (conductor-emit, P-005/P-008)
**Date:** 2026-06-18 (UTC)
**Commits:** uncommitted — the chunk commit is created in wrap P7; prior session closed at `e4763d5` (raw-otlp-message-scaffold)

## Changes (structured — detectors read this)
- **Files:**
  - `M crates/conductor-emit/src/message.rs` — refactored `ok_span` onto a shared `pub(crate) span()` constructor; added `pub(crate) ok_status()`/`error_status()`; raised `service_resource` to `pub(crate)`. `trace_request` behavior unchanged.
  - `A crates/conductor-emit/src/span_tree.rs` — `ErrorPlacement`, `error_trace_request`, seeded `gen_id`, 4 unit tests.
  - `A crates/conductor-emit/tests/error_spans.rs` — 2 loopback-stub integration tests (root + deep-child error).
  - `M crates/conductor-emit/src/lib.rs` — `mod span_tree;` + re-exports.
  - `M crates/conductor-emit/Cargo.toml` — `rand_chacha` + `rand_core` edges.
  - `M Cargo.lock` — 2-line edge addition (no new crates).
- **Symbols / APIs:**
  - NEW public (re-exported): `error_trace_request(service_name: &str, seed: u64, placement: ErrorPlacement, message: &str) -> ExportTraceServiceRequest`; `enum ErrorPlacement { Root, DeepChild { depth: usize } }`.
  - NEW `pub(crate)` (message.rs): `span()`, `ok_status()`, `error_status()`; `service_resource()` visibility raised.
  - Unchanged public surface: `trace_request`, `TraceEmitter`, `EmitError`, `DEFAULT_SERVICE_NAME`, `DEFAULT_OTLP_ENDPOINT`.
  - No new IPC methods / endpoints / ports / sockets / env vars. Egress target unchanged (`127.0.0.1:4317`).
- **Crates / modules:** conductor-emit gains module `span_tree`. No crate added/removed; no cross-seam edge (deps stay conductor-core + opentelemetry-proto + tonic + thiserror + tracing + rand_chacha + rand_core).
- **Dependencies:** ADDED edges to conductor-emit — `rand_chacha` (0.9, workspace) + `rand_core` (0.9, workspace). Both already in `Cargo.lock` via conductor-timeline → no new external crate, no version bump. audit/deny green.
- **Schema / config:** none (no migrations, no config keys, no violation schemas, no `runs.db` touch).
- **Coverage of new surfaces:**
  - `error_trace_request` (OTLP product-emission builder) → validation **n/a** (no external input; `seed`/`placement`/`message` are typed args) · instrumentation **✓** (rides the existing `emit.batch` span via `TraceEmitter::export`; unchanged) · PII **n/a** (synthetic test payload; no PII corpus this chunk) · tests **unit+integ ✓** (4 unit + 2 loopback) · a11y **n/a** (no UI) · tokens **n/a** (no UI).
  - `ErrorPlacement` (public enum) → validation **n/a** · tests **✓** (root + deep-child) · a11y/tokens **n/a**.

## Deviations from intent
1. **Builder signature dropped the plan's redundant `depth` param** — shipped `(service_name, seed, placement, message)`. `ErrorPlacement::DeepChild { depth }` already carries the depth; chain length derives as `depth + 1` with the error at the leaf — well-defined, panic-free, no placement-beyond-chain validation. The plan explicitly left the signature to /implement ("name/signature is HOW"). Fully satisfies root-vs-deep (P-005/P-008).
2. **Spans auto-named** (`root` / `child-{i}`) rather than a name parameter — minimal API; product span names are unconstrained (the obs high-cardinality ban governs *self-obs* tracing spans, not emitted product spans).
3. **Integration-test stub duplicated** in `error_spans.rs` (own `CapturingService`/`start_stub`) rather than sharing `egress.rs`'s — `egress.rs` is outside the plan's touchpoints, so staying in-scope beat a DRY refactor of an out-of-scope file. Test-code duplication is acceptable.
4. **Determinism tested via a `shape()` projection** (ids + linkage + status, *excluding* wall-clock timestamps) — realizes research OQ2: `unix_nanos()` is wall-clock, so a full-byte golden would flake; the seed governs identity, not the clock.

## Decisions & corrections
- **ID-gen (research OQ1):** builder takes `seed: u64`, builds `ChaCha8Rng::seed_from_u64(seed)` internally, threads `&mut rng` into `gen_id::<N>` — mirrors `conductor-timeline/src/scheduler.rs:43,64` ("randomness as input, no global entropy").
- **Timestamp/determinism (research OQ2):** wall-clock `unix_nanos()` kept; determinism asserted on the seeded *shape* (structural + id), NOT a full-byte `insta` golden (would flake on timestamps). Durable testing pattern for seeded emitters whose payload also carries wall-clock fields.
- **Module split (research OQ3):** new `span_tree.rs` owns tree/placement/ID-gen; `message.rs` refactored to expose shared `pub(crate)` raw-struct primitives; `trace_request` public API preserved.
- **`Status` is exactly 2 fields** (`message`, `code`) in opentelemetry-proto 0.32 — `error_status` sets both exhaustively (no `..Default::default()`, which would trip `clippy::needless_update`).
- **Non-error spans carry baseline `OK`** (not `UNSET`) — consistent with the scaffold.
- **`default-features = false` opentelemetry-proto trim NOT done** — out of this chunk's plan scope (only `rand_*` edges added); remains the tracked obs follow-up (dormant `opentelemetry_sdk 0.32.1` still compiles, never initialized).
- **Fix-loop:** 2 iterations — (1) `Vec<Span>` type annotation (inference ordering); (2) `clippy::type_complexity` → `SpanShape` alias.

## Outcome
- **Acceptance criteria: ALL met** — raw `Status{code:ERROR,message}`; seeded ChaCha8 IDs (builder takes seed); shared `trace_id` + `parent_span_id` chain; ERROR placeable at root/deep-child; determinism both directions; loopback stub asserts ERROR at the expected span, never binds `:4317`; `EmitError` untouched (verdict/error wall); `Cargo.lock` un-drifted; audit/deny green.
- **Gates green:** `cargo nextest run -p conductor-emit` 10/10 · `cargo nextest run --workspace --profile ci` 91/91 · `cargo clippy --workspace --all-targets -- -D warnings` exit 0 · `cargo test -p conductor-emit --doc` ok (0 doctests) · `cargo audit` + `cargo deny check` green.
- **Smoke:** skipped — no boot-path change (library emission primitive; touchpoints are conductor-emit lib only; plan Test Commands exclude `agent-run.sh`).
