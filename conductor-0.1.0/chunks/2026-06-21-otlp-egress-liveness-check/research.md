# Codebase Research — 2026-06-21-otlp-egress-liveness-check

## Scope
- **Depth:** moderate · **Reads:** 5 (client.rs, error.rs, lib.rs, tests/egress.rs, code-graph-views.sql) · **Globs/Greps:** 2 greps + 1 code-graph query

## Headline finding
The OTLP-egress liveness *behavior* already exists as a side effect of emitter construction. `client.rs:30`
`TraceEmitter::connect()` does `Endpoint::from_shared(endpoint.into())?.connect().await?` → `Result<Self, EmitError>`,
and its module doc (`client.rs:6`) literally names this "**the OTLP-egress liveness surface at this layer**". A refused
transport already returns `EmitError::Transport`, and `tests/egress.rs:73` `refused_transport_surfaces_emit_error`
already asserts it. **So this chunk is not "build connect" — it is: (1) promote the liveness check to a first-class,
named, standalone preflight primitive decoupled from emitter construction (mirroring how `conductor-verify::run_preflight`
is a distinct entry point), and (2) close the unbounded-connect-timeout gap.**

## Files inspected
- `crates/conductor-emit/src/client.rs` (full) — `TraceEmitter`/`LogsEmitter`, each with `connect(endpoint) -> Result<Self, EmitError>` (eager `Endpoint::from_shared(...)?.connect().await?`, lines 30-33 / 67-70) and `from_channel(Channel)` (the in-process test path). `DEFAULT_OTLP_ENDPOINT = "http://127.0.0.1:4317"` (line 20). `export` is `#[tracing::instrument(name="emit.batch"/"emit.logs_batch")]`; **`connect` is NOT span-instrumented.**
- `crates/conductor-emit/src/error.rs` (full) — `EmitError` is `#[derive(Debug, thiserror::Error)] #[non_exhaustive]` with `Transport(#[from] tonic::transport::Error)` (the refused/invalid-endpoint case — and the case tonic returns when a `connect_timeout` elapses) and `Status(#[from] tonic::Status)`. `#[non_exhaustive]` ⇒ adding a variant is non-breaking.
- `crates/conductor-emit/src/lib.rs` (full) — re-exports `client::{LogsEmitter, TraceEmitter, DEFAULT_OTLP_ENDPOINT}`, `error::EmitError`, and the free request-builders (`trace_request`, etc.). A new probe fn + timeout const get a `pub use client::...` line here.
- `crates/conductor-emit/tests/egress.rs` (full) — the loopback-stub pattern: `start_stub()` binds `TcpListener` on `127.0.0.1:0`, serves `TraceServiceServer` over `TcpListenerStream`, `#[tokio::test(flavor="current_thread")]`. Connectable case (`exports_well_formed_request_to_loopback_stub`) + refused case (`refused_transport_surfaces_emit_error`, dials `http://127.0.0.1:1`).
- `scripts/code-graph-views.sql` (full) — schema for the query (views `crate_edges`, `calls_m`, `symbol`).

## Graph impact (from the code-graph query → `tree-query-2026-06-21-otlp-egress-liveness-check.json`)
- **`crate_edges WHERE from/to = conductor-emit` → `[]`** — conductor-emit has no resolved seam→seam edges in either direction: no seam depends on it yet (timeline/cli wire-up is Epoch 7/8), and it calls no other seam. **Adding the probe introduces zero new edges; star topology is trivially preserved.**
- **Grep blast radius** — `TraceEmitter`/`LogsEmitter`/`.connect(`/`DEFAULT_OTLP_ENDPOINT` occur ONLY in `conductor-emit/src/{lib,client}.rs` + `conductor-emit/tests/*.rs`. No external production caller; the change is intra-crate + its own tests.
- **`connect_timeout|.timeout(|Duration` → no matches in conductor-emit** — confirms there is no bounded timeout anywhere in the seam today (the gap this chunk closes).

## Patterns detected
- **Runtime-agnostic async client** (`client.rs:1-3`): the caller owns the tokio runtime (matches `conductor_timeline`); functions are `async fn`, no internal `spawn`/runtime.
- **Refused/invalid endpoint ⇒ `EmitError::Transport` via `?`** (`client.rs:31`): both `from_shared(...)?` and `.connect().await?` map through `#[from] tonic::transport::Error`. A connect-timeout elapsing also yields `tonic::transport::Error` ⇒ already `EmitError::Transport` (no new variant strictly required).
- **`from_channel(Channel)` split** (`client.rs:36`): connection construction is separable from the typed client — the probe can reuse the same `Endpoint`-building step.
- **Span instrumentation only on `export`** (`client.rs:43,80` = `emit.batch`/`emit.logs_batch`); `connect` carries no span. The obs bounded span-name set (obs-plan §4/§11) does NOT contain a liveness name (it lists `emit.batch`, `emit.logs_batch`, …).
- **Loopback-stub test discipline** (`tests/egress.rs`, testing.md 2026-06-17): ephemeral `127.0.0.1:0` stub for connectable; `127.0.0.1:1` for refused; never bind `:4317` (reserved for the P-003 port-occupier fault).

## Conventions to follow
- **std::time for the timeout** (arch + security + obs extracts): the bounded `connect_timeout` is `std::time::Duration`, never tokio's virtual clock. `Endpoint::connect_timeout(Duration)` is the tonic builder method.
- **No new span name** (obs extract + observed pattern): `connect` is un-instrumented today, and `emit.liveness_check` is NOT in the obs bounded set — adding a span would require an obs-plan §4/§11 amendment. Prefer matching the existing un-instrumented `connect` (optionally a single structured `tracing` event on outcome carrying `run_id`), so no bounded-set amendment is forced. (Surface as a decision.)
- **`EmitError::Transport` is the refusal contract** — reuse it; only add a variant if a distinct `Timeout` aids diagnostics (it is `#[non_exhaustive]`, so additive-safe).
- **Free-function + const surface** (lib.rs re-export style): a free `pub async fn probe_egress(...)` + a `DEFAULT_CONNECT_TIMEOUT` const re-exported from lib.rs matches the crate's existing `trace_request` / `DEFAULT_OTLP_ENDPOINT` style.

## New files to create
- (likely none) — extend `client.rs`. Optionally a dedicated `crates/conductor-emit/tests/liveness.rs` if the probe tests outgrow `tests/egress.rs` (P4 decides; reuse `start_stub()` either way).

## Files to modify
- `crates/conductor-emit/src/client.rs` — add the standalone `probe_egress` liveness fn (connect + drop, bounded `connect_timeout`); factor a private `endpoint(addr, timeout) -> Result<Endpoint, EmitError>` helper and thread the bounded timeout through `TraceEmitter::connect` / `LogsEmitter::connect` too (so emission connect is bounded as well). Add `DEFAULT_CONNECT_TIMEOUT`.
- `crates/conductor-emit/src/lib.rs` — `pub use` the new `probe_egress` + `DEFAULT_CONNECT_TIMEOUT`.
- `crates/conductor-emit/tests/egress.rs` (or new `tests/liveness.rs`) — probe tests: connectable stub ⇒ `Ok(())`, refused (`127.0.0.1:1`) ⇒ `Err(EmitError::Transport(_))`, and a bounded-timeout assertion; reuse `start_stub()`.
- `crates/conductor-emit/Cargo.toml` — confirm no new dep needed (`std::time::Duration` + existing `tonic`); `tokio` stays dev-only.

## Open questions
1. **Probe shape — connect-and-drop vs. return the live `Channel`.** A pure `probe_egress() -> Result<(), EmitError>` (connect, drop) is the clean preflight but double-connects when Epoch-8 then builds the emitter; returning/retaining the `Channel` avoids the second connect but couples to emitter ownership (an Epoch-8 concern). Lean: connect-and-drop now (orchestration/reuse deferred). Resolve at P4.
2. **Bounded-timeout default + variant.** Pick `DEFAULT_CONNECT_TIMEOUT` (e.g. a few seconds, fail-fast; not an SLO tier) and decide fold-into-`Transport` vs. a distinct `EmitError::Timeout`. Lean: small const + reuse `Transport`. Resolve at P4.
3. **Obs touch.** Confirm "no new span; optional structured `tracing` event only" so no obs-plan bounded-set amendment is triggered. (If a span is wanted, it's an obs-plan §4 amendment — surface at P5.)
