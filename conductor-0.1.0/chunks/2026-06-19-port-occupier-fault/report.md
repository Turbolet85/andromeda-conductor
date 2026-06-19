# Report — 2026-06-19-port-occupier-fault

**Chunk:** Port-occupier fault — sacrificial loopback :4317 binder with RAII bind→hold→release lifecycle, the sole deliberate inbound bind (conductor-faults, P-003)
**Date:** 2026-06-19T18:45:39Z
**Commits:** none yet — chunk uncommitted; wrap P7 commits it (prior HEAD `27874fc` traffic-rate-ramps)

## Changes (structured — detectors read this)
- **Files:**
  - new: `crates/conductor-faults/src/error.rs` · `crates/conductor-faults/src/port_occupier.rs` · `crates/conductor-faults/tests/port_occupier.rs`
  - modified: `crates/conductor-faults/src/lib.rs` (mod decls + re-exports + crate doc) · `crates/conductor-faults/Cargo.toml` (+`thiserror`) · `Cargo.lock` (intra-workspace edge only)
- **Symbols / APIs:** new public — `PortOccupier` (struct) with `occupy(port: u16) -> Result<Self, FaultError>` · `occupy_default()` · `local_addr() -> SocketAddr` · `release(&mut self)` · `impl Drop`; `FaultError` (enum, `#[non_exhaustive]`, variant `Bind { addr: SocketAddr, source: io::Error }`); `const OTLP_INGEST_PORT: u16 = 4317`.
- **Sockets / ports:** `PortOccupier` **binds `127.0.0.1:<port>`** (loopback-only; default `4317`) — Conductor's **sole deliberate inbound bind**, already documented in arch §Occupied Resources ("the port-occupier fault scenario also binds `:4317`") + §Cross-cutting Patterns (Trust boundary). It never `accept`s and never speaks OTLP. No new endpoint / IPC method / event / env var.
- **Crates / modules:** `conductor-faults` (already a registered workspace member, arch §Occupied Resources crate-names) gains its first real modules (`error`, `port_occupier`). No crate added/removed. Crate-per-seam held: imports only `std` + `thiserror` + `crate::*` — **no** `conductor-emit`/`conductor-timeline`/`conductor-report`.
- **Dependencies:** added `thiserror.workspace = true` to `conductor-faults` — **already a workspace dependency** (arch §Stack names `thiserror 2.0.18`; used by `conductor-core`/`conductor-emit`), audit-green. **`Cargo.lock` gained only the intra-workspace dependency edge — zero new `[[package]]` entries** (no new external crate / no new advisory surface). No `tokio`, no `tracing`.
- **Schema / config:** none — no scenario config, no garde, no `runs.db`, no violation schema, no env var, no config key.
- **Coverage of new surfaces:**
  - `PortOccupier::occupy(port: u16)` (loopback TCP bind) → validation **n/a** (programmatic `u16` argument — NOT an external-input boundary: no scenario-config / `CONDUCTOR_*` path / MCP-child-stdout / deserialized struct) · instrumentation **✗ deferred** (obs §4 `fault.port_occupier` span — already in the bounded span set — applies when the fault is driven under a timeline/`run_id` context, Epoch 7; the standalone primitive logs nothing) · PII **n/a** · tests **unit+integ ✓** (7 tests on ephemeral `:0` loopback, synchronous — deterministic, no real network/clock; llvm-cov 88.57%) · a11y **n/a** (headless) · tokens **n/a** (no UI)
  - `FaultError` (typed bind-failure) → validation **n/a** · instrumentation **n/a** (no logging) · PII **n/a** (Display carries a loopback `SocketAddr`, not a host file path — no `conductor_*` struct-name leak) · tests **✓** (Display + chained-source unit test; error.rs 100%) · a11y **n/a** · tokens **n/a**

## Deviations from intent
- **`local_addr()` returns `SocketAddr` infallibly** (plan stated `Result<SocketAddr, FaultError>`). Justified: the resolved addr is captured once at bind time (required for the `:0` ephemeral case anyway), so there is no fallible read site — cleaner API, no dead error branch. The bind-time resolution call's (essentially-unreachable) failure is mapped to `FaultError::Bind` inside `occupy`, preserving the no-panic invariant.
- **Struct stores a resolved `addr` field** alongside `Option<TcpListener>` (plan implied a listener-only struct). Justified: backs the infallible `local_addr()` and keeps the addr readable after `release()` (the tests read addr → release → rebind).
- **`occupy_default()` is not exercised by an auto-test** (calling it would bind the real `:4317`, banned by the testing rule). Justified: a `default_target_is_the_otlp_ingest_port` test asserts the `OTLP_INGEST_PORT` constant instead; the 1-line `occupy_default` body is part of the 4 uncovered lines, coverage still 88.57% ≫ 60%.
- Otherwise the acceptance-criteria intent is fully met.

## Decisions & corrections
- **Design:** sync `std::net::TcpListener` (no `tokio`) — the occupier only binds+holds+drops (never `accept`s), so no async runtime is needed; this also makes the lifecycle test a plain sync `#[test]`. (Resolves the scope's std-vs-tokio open question.)
- **Design:** port-only constructor with hard-coded loopback IP (`Ipv4Addr::LOCALHOST`) — reconciles "loopback, never a user-supplied address" (security) with "ephemeral `:0` test isolation" (tests): only the `u16` port is a parameter; the IP can never be widened to `0.0.0.0`.
- **Design:** obs span / `tracing` instrumentation deferred to the scenario epoch (Epoch 7), matching how `conductor-emit`'s pure primitives (`rate.rs`/`latency.rs`) return values without logging.
- No user corrections this session; no new universal "from now on" conventions (the above are chunk-specific design resolutions).

## Outcome
- **Acceptance criteria:** met — loopback `:4317` exclusive bind · IP hard-coded (port-only param) · exclusivity proven on ephemeral `:0` · bind-failure typed `FaultError` (never a panic) · idempotent `release()` + RAII `Drop` · crate-per-seam (only `thiserror` added) · no `tokio`/`tracing`.
- **Gates (all green, 0 fix iterations):** `cargo nextest run -p conductor-faults` 7/7 · `cargo nextest run --workspace --profile ci` 153/153 · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo test -p conductor-faults --doc` 0 doctests (deliberately none — a runnable doctest would bind `:4317`) · `cargo llvm-cov nextest -p conductor-faults --fail-under-lines 60` → 88.57% (error.rs 100% · port_occupier.rs 84.6%).
- **Smoke:** skipped — no boot-path change (pure library primitive; the `cleanup`-command bind-release wiring is Epoch 8).
