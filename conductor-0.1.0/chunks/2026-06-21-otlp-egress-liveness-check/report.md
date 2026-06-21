# Report — 2026-06-21-otlp-egress-liveness-check

**Chunk:** OTLP egress liveness check — loopback :4317 connectable, refused ⇒ harness Err (conductor-emit)
**Date:** 2026-06-21
**Commits:** this chunk is uncommitted (committed in wrap P7); since last_wrap only the prior chunk's `33fd8ed feat(2026-06-21-preflight-readiness-gate)`.

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/src/client.rs`, `crates/conductor-emit/src/lib.rs`, `crates/conductor-emit/tests/egress.rs`. (Plus phase/wrap artifacts: chunk folder, master/working route, run-dirs — not code.)
- **Symbols / APIs:**
  - NEW pub `probe_egress(endpoint: impl Into<String>) -> Result<(), EmitError>` (async free fn, conductor-emit) — OTLP-egress liveness probe (connect + drop; refused/unreachable/timed-out ⇒ `EmitError::Transport`).
  - NEW pub const `DEFAULT_CONNECT_TIMEOUT: std::time::Duration` (= 5s) — bounded fail-fast connect timeout.
  - NEW private `egress_endpoint(endpoint) -> Result<Endpoint, EmitError>` helper (applies the bounded `connect_timeout`).
  - CHANGED `TraceEmitter::connect` / `LogsEmitter::connect` — now route through `egress_endpoint` (connect is bounded); public signatures UNCHANGED.
  - No new IPC method / endpoint / event / socket / port / env var. No new inbound bind (`:4317` is touched only as an outbound client connect target).
- **Crates / modules:** none added/removed; `conductor-emit` (client module) changed. Star topology preserved — `conductor-emit` still depends on `conductor-core` only (code-graph `crate_edges` for conductor-emit empty; no seam→seam edge introduced).
- **Dependencies:** none added, none bumped (`std::time::Duration` + existing `tonic`; `tokio` stays dev-only). `Cargo.lock` un-drifted.
- **Schema / config:** none.
- **Coverage of new surfaces:**
  - `probe_egress` (outbound OTLP/gRPC client connect to `:4317` — a liveness gate, not an inbound surface) → validation n/a (no deserialized external input; endpoint is an internal string/const, not a garde scenario-config surface) · instrumentation span ✗ — **intentional**: matches the existing un-instrumented `connect()`; the obs bounded span-set (obs-plan §4/§11: `emit.batch`/`emit.logs_batch`/…) has no liveness/connect span, and the must-trace op is emission, not the connect probe — no new span name introduced · PII n/a (no payload emitted) · tests unit/integ ✓ (3 tests, loopback stub on ephemeral `127.0.0.1:0` + refused `127.0.0.1:1`, never `:4317`) · a11y n/a (no UI) · tokens n/a (no UI)

## Deviations from intent
- **Timeout-*elapse* path not integration-tested.** Plan flagged a real black-hole wait as flaky/infeasible on loopback (zero-retry bar). Covered instead by the deterministic `default_connect_timeout_is_bounded_fail_fast` const-bound guard (0 < `DEFAULT_CONNECT_TIMEOUT` ≤ 10s) + observed evidence: the refused probe returns `Err(Transport)` in ~2.1s on this host, well under the 5s bound (fail-fast, no hang). Justified — no flaky wall-clock test added.
- **Tests extended `tests/egress.rs`** (reusing `start_stub`) rather than a new `tests/liveness.rs` — plan permitted either; reuse kept the loopback-stub discipline DRY.
- **`EmitError::Transport` reused** (no new `Timeout` variant) — as planned; tonic returns `transport::Error` on a `connect_timeout` elapse; `EmitError` stays `#[non_exhaustive]`.
- **`probe_egress` un-instrumented (no span)** — as planned; consistent with `connect()` and keeps the obs bounded span-set unchanged (no obs-plan amendment forced). The must-trace operation per obs-plan §4 is the emission/scenario run, not the pre-emission connect probe.

## Decisions & corrections
- **P4 AskUserQuestion — deliverable shape = Option A** (standalone `probe_egress` + bounded timeout), user-selected over (B) fold-the-timeout-into-`connect()`-only and (C) return-the-live-`Channel`. Rationale: faithful to arch §Standard Contracts "Liveness equivalent" as a distinct pre-emission gate (parallels `conductor-verify::run_preflight`); closes the unbounded-connect-timeout gap research found; emitter ownership/orchestration stays deferred to Epoch 8.
- **`DEFAULT_CONNECT_TIMEOUT` = 5s** — a fail-fast connect bound, explicitly NOT an SLO tier.
- **Bounded timeout threaded through both emitters** (`TraceEmitter`/`LogsEmitter::connect`) via the shared `egress_endpoint` helper — behavior-preserving (unbounded→bounded only); grep confirmed no external caller depends on the prior unbounded behavior.
- Research finding that reshaped scope: the refused⇒`EmitError` liveness *behavior* already existed via `connect()`; this chunk formalized it as a standalone primitive + added the missing bound (not a from-scratch build).

## Outcome
- **Acceptance criteria: all met** — refused/unreachable/timed-out ⇒ `Err(EmitError::Transport(_))`, never a `Verdict`/`ReportState`, never a panic · bounded std::time connect timeout (no hang) · typed + sanitizable error (`Display` "OTLP transport error: {0}", no host-path/struct-name leak) · no seam→seam edge, no inbound bind · tests green incl. the 3 cases, none binding `:4317` · no new span / no OTel SDK.
- **Gates green:** `cargo nextest run -p conductor-emit` 68/68 · `cargo nextest run --workspace --profile ci` 202/202 (199→202, +3) · `cargo test -p conductor-emit --doc` 0 doctests · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo audit` exit 0 · `cargo deny check` advisories·bans·licenses·sources ok · `Cargo.lock` un-drifted.
- **Smoke:** skipped — no boot-path change (library primitive; conductor-cli wiring is Epoch 8). Harness `status` verb confirmed intact (correct usage-guard, exit 2 on missing run_id).
