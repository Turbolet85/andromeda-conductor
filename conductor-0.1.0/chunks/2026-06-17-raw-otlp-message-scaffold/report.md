# Report — 2026-06-17-raw-otlp-message-scaffold

**Chunk:** Raw OTLP message scaffold — hand-built opentelemetry-proto trace structs over tonic/tonic-prost gRPC egress to 127.0.0.1:4317 (conductor-emit)
**Date:** 2026-06-17T23:06:06Z
**Commits:** _uncommitted — wrap P7 commits this chunk._ (Intervening `8d56a7a` since last_wrap is the out-of-chunk code-graph tooling migration, separately committed; `a2fa56f` is the prior chunk.)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/Cargo.toml` (mod) · `src/lib.rs` (mod) · `src/error.rs` (new) · `src/message.rs` (new) · `src/client.rs` (new) · `tests/egress.rs` (new) · `deny.toml` (mod) · `Cargo.lock` (regenerated, +583 lines)
- **Symbols / APIs:** new public surface `conductor_emit::{EmitError, TraceEmitter, DEFAULT_OTLP_ENDPOINT, trace_request, DEFAULT_SERVICE_NAME}`; `TraceEmitter::{connect, from_channel, export}`. OTLP/gRPC **egress** to `127.0.0.1:4317` (client only — **no inbound bind**). Tests bind an ephemeral `127.0.0.1:0` loopback `TraceService` stub. No new env vars (service.name default hardcoded `"conductor"`; `CONDUCTOR_SERVICE_NAME` not yet wired).
- **Crates / modules:** `conductor-emit` placeholder → implemented (new modules `error`, `message`, `client`). Not a new workspace member (already registered in arch §Inherited Defaults / §Modules).
- **Dependencies:** `conductor-emit` gains (all workspace-pinned) `opentelemetry-proto` 0.32.0 (features `gen-tonic`,`trace`; **default features left ON**), `tonic` 0.14.6, `thiserror`, `tracing`; dev: `tokio` (macros/rt/net), **`tokio-stream` 0.1 (NEW — not previously in the workspace dep set)**. Transitive additions to the lock (+583): hyper, h2, tower, axum, matchit, prost, tonic-prost, hyper-util, hyper-timeout, **+ `opentelemetry` 0.32.0 and `opentelemetry_sdk` 0.32.1** (pulled by `opentelemetry-proto`'s default feature — **NOT used by conductor**; self-obs is unchanged, still tracing-JSON-only, no SDK initialized). `cargo audit` clean; `cargo deny check` green.
- **Schema / config:** `deny.toml` `[licenses] allow` += `"BSD-3-Clause"` (for `matchit`, `MIT AND BSD-3-Clause`, transitive via axum←tonic). No other config/schema change.
- **Coverage of new surfaces:**
  - `conductor_emit::TraceEmitter` (OTLP/gRPC egress client → 127.0.0.1:4317) → validation **n/a** (outbound client; builds its own messages, deserializes no external input this chunk) · instrumentation **span✓** (`emit.batch` `#[tracing::instrument]`, info-level, `emission_count` field) · PII **n/a** (no payload corpus yet; only `service.name="conductor"`) · tests **integ✓** (loopback stub round-trip + transport-error) · a11y **n/a** (headless) · tokens **n/a** (headless)
  - `conductor_emit::trace_request` (raw OTLP message builder) → validation **n/a** · instrumentation **n/a** (pure builder) · PII **n/a** · tests **unit✓** (2) · a11y **n/a** · tokens **n/a**
  - `EmitError` (transport/status faults) → tests **integ✓** (transport-refusal → `EmitError::Transport`) · PII/redaction **n/a** (not logged this chunk; `Display` carries the tonic endpoint/status — a loopback addr / gRPC status, no host **file** path / struct name; obs.rs field-allowlist would scrub if it were logged) · validation n/a

## Deviations from intent
1. **`deny.toml` edited — NOT in the plan's predicted Files-to-modify.** Added `BSD-3-Clause` to the license allow-list for `matchit` (transitive via axum←tonic gRPC tree). Justified: the chunk's acceptance criterion explicitly requires `cargo deny check` green over the new tree; `deny.toml` self-documents this additive license maintenance ("extend ONLY with licenses actually present in the committed lock"); `BSD-3-Clause` is OSI/FSF-permissive (same class as the existing MIT/Apache/BSL allow-list); local-only tool. Judged **in-scope**.
2. **`opentelemetry-proto` transitively pulls `opentelemetry` + `opentelemetry_sdk`.** The plan's `features = ["gen-tonic", "trace"]` did not set `default-features = false`, so opentelemetry-proto's default feature brings the OTel SDK conversion crates into the tree. They are **NOT used by conductor** (no direct reference; self-obs invariant holds — tracing JSON only, no SDK init). audit/deny green. **Possible follow-up:** `default-features = false` to slim the tree. Surfaced for the obs detector / user.
3. **`prost` + `tonic-prost` NOT added as direct deps** (plan step 1 listed `prost`; `tonic-prost` "if needed"). Both arrive transitively via opentelemetry-proto's `gen-tonic`; `conductor-emit` uses no `prost`/`tonic-prost` API directly — direct deps would be unused-dep noise. All gates green without them.
4. **`thiserror` added to `conductor-emit` deps** — the plan named `EmitError (thiserror)` but its step-1 dep list omitted `thiserror`; added (caught by IDE diagnostics pre-build). Plan-list completion.
5. **`emit.batch` span records `emission_count` only, not `p_id_count`** (obs extract named both). `p_ids` are scenario-level context unavailable at the raw-emit-client boundary; `p_id_count` belongs to the higher scenario-runner span (later epoch). In-impl judgment.
6. **API note (not a deviation):** opentelemetry-proto 0.32.0's `KeyValue` carries a third field `key_strindex` (newer OTLP string-table index) → handled with `..Default::default()`.

## Decisions & corrections
- Phase val-1 amended `scope.md` once: `tonic-prost-build` codegen is **not** needed — opentelemetry-proto's `gen-tonic` ships the generated `TraceServiceClient` + types (confirmed in implement: no `build.rs` exists).
- Test stub = loopback tonic `TraceService` on ephemeral `127.0.0.1:0` — **not** rmcp (MCP read-back, Epoch 5), **not** the real `:4317` (reserved for the Epoch-4 port-occupier).
- `run_timeline` left **pure** (emits nothing by design) — emit is a standalone seam; per-transition wiring deferred to a later chunk / the CLI runner (scope granted this latitude).
- (Session context) the code-graph tooling migration was committed at session start (`8d56a7a`); new-session C11/C12 health warnings are now the expected post-migration state.

## Outcome
- **Acceptance criteria: met.** Raw `ExportTraceServiceRequest` from opentelemetry-proto raw types (no `opentelemetry-otlp` exporter) ✓ · async `TraceServiceClient` → `127.0.0.1:4317` ✓ · transport refusal → typed `EmitError`, no panic ✓ · `conductor-emit` → `conductor-core`-only crate edge ✓ · `emit.batch` span, zero OTel SDK **initialized** ✓ · `Cargo.lock` committed + audit/deny green ✓ · loopback gRPC stub test ✓.
- **Gates green:** `cargo nextest run -p conductor-emit` (4/4) · `cargo nextest run --workspace --profile ci` (85/85, no regressions) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `cargo audit` (clean) · `cargo deny check` (advisories/bans/licenses/sources ok).
- **Smoke:** skipped — no boot-path change (library seam; `agent-run.sh status` requires a run_id this library produces none; egress verified via the loopback integration test).
