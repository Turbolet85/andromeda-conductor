# Codebase Research — 2026-08-20-verifier-self-hardening

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 9 · **Live measurements:** 3 (`cargo test` runs)

## Files inspected
- `crates/conductor-run/src/lib.rs` (`:214-224`, `:280-290`, `:515-520`, `:629-634`) — `declares` predicate; the `min_canary_poll_seconds` `.max(...)` at `:285`; both journal-stamp helpers.
- `crates/conductor-verify/src/jsonrpc.rs` (`:18`, `:74-84`) — `MAX_LINE_BYTES = 16 * 1024 * 1024`; the `read_line` guard and its `VerifyError::Decode` arm.
- `crates/conductor-verify/src/preflight.rs` (`:321-342`, `:372-390`) — `poll_canary` retry loop; `assert_canary` freshness comparison and its `CanaryFidelity` arms.
- `crates/conductor-run/tests/canary_wire.rs` (full) — 5 tests; the failing witness test at `:211-268`.
- `crates/conductor-core/src/obs.rs` (`:63-81`) — `ObsSink`, `init_observability` contract.
- `crates/conductor-cli/src/main.rs` (`:1-46`) — bootstrap; `init_observability` call at `:28`.
- `crates/conductor-cli/Cargo.toml` (`:19`) — `tracing.workspace = true`.
- `.andromeda/obs-plan.md` (`:46-50`) — the §4 per-surface instrumentation table.

## Graph impact (from the code-graph query — trace `rows: 9`, `db_state: fresh`)
Query: callers of `declares` / `now_ms` / `now_unix_nanos` / `assert_canary` / `poll_canary`.
- **`now_ms` / `now_unix_nanos`** — callers all intra-crate in `conductor-run`: `emit_canary()` @ `lib.rs:232,239,244`, crate-level @ `:349`, `:390`, `fault_span()` @ `:573`. No cross-crate consumer.
- **`assert_canary`** — one caller, `poll_canary()` @ `preflight.rs:329`.
- **`poll_canary`** — one caller, `run_preflight()` @ `preflight.rs:203`.
- **Reading:** every survivor symbol is reached only from inside its own crate. Zero cross-crate blast radius for the killing tests — but also **zero public entry point**: each must be asserted through the crate's public surface.

## Scope premise closure (the three `[inferred]` bullets)

### (a) Paused-clock killing shape — **VERIFIED, with a refinement**
`#[tokio::test(flavor = "current_thread", start_paused = true)]` is well established: **11 uses** in
`crates/conductor-run/tests/dispatch_wire.rs` (`:186`…`:372`) and one in
`crates/conductor-core/tests/operator_pause.rs:43`.
**Refinement that matters:** explicit `tokio::time::advance` is used **nowhere** in the workspace — its single
occurrence (`operator_pause.rs:46`) is a comment stating that *completing without any advance* is what proves
the hold consumes no virtual time. So the house idiom is `start_paused` + tokio's **auto-advance**, with the
assertion made on observed virtual time, not on a manual advance call. A loop-guard killing test should follow
that idiom rather than importing an unused one.

### (b) No existing stamp-magnitude assertion — **VERIFIED, plus a new constraint**
Zero test files reference `now_ms` / `now_unix_nanos`; all 8 hits are src-side call sites plus the two
definitions. **New constraint:** both are **crate-private** (`fn`, not `pub fn`) at `lib.rs:516` and `:630`, and
arch bans widening visibility to create a test seam. A killing test must therefore assert through a public
observable that CARRIES the stamp — the journal's `journal_emitted_at`, or `CanaryMarker.emitted_at_unix_nano`
— not by calling the helper.

### (c) The §B3 mechanism — **VERIFIED BY MEASUREMENT, and the carried fix is confirmed wrong**
Three runs settle it:

| Invocation | Result |
|---|---|
| `cargo test -p conductor-run --test canary_wire` | **FAILED** — 4 passed, `the_wire_shape_witness_reaches_the_self_obs_artifact` failed |
| same, filtered to that test alone | **ok** — 1 passed |
| same, `-- --test-threads=1` | **ok** — all 5 passed |

Alone it passes; serially all five pass; only *concurrently with its siblings* does it fail. The cause is
therefore **concurrent cross-test interference**, not install-ordering and not a broken test. Since
`init_observability` installs a **process-global** subscriber (`obs.rs:68-77`, first-install-wins) and the four
sibling tests all call `emit_canary_storm`, their `emit.batch` debug lines land in the witness test's file
while it is counting — the count assertion over-counts. The carried CARRY fix ("a per-test temp obs sink") is
**confirmed not the defect**: the test already owns an `assert_fs::TempDir` sink; a per-test FILE cannot
isolate a process-global SUBSCRIBER.

**Diagnostic aside worth keeping:** the failure prints **no panic message** under `cargo test` — the `failures:`
block is empty. `init_observability` installs the panic hook alongside the subscriber (`obs.rs:68`), so the
assertion text is routed into the obs sink rather than the harness's captured stdout. That opacity is part of
why this sat unnoticed; `--nocapture` on the single test does not reproduce it (it passes alone).

## Patterns detected
- **Paused-clock scheduling tests** (`dispatch_wire.rs:186-372`): `current_thread` + `start_paused`, asserting emitted-stream shape; no manual advance.
- **Loopback tonic stub + captured requests** (`canary_wire.rs:41-70`): `Arc<Mutex<Vec<ExportTraceServiceRequest>>>` behind a `TraceService` impl on an ephemeral port — the established way to observe what `conductor-emit` puts on the wire without `:4317`.
- **Typed decode rejection** (`jsonrpc.rs:79-81`): the size-bound breach already returns `VerifyError::Decode { reason }` — the verdict/error wall is satisfied in the code; what is missing is any test that exercises it.
- **Freshness via strict `>`** (`preflight.rs:385`): `opened > canary.emitted_at_unix_nano` over `.iter().any(...)`, with `NotFound::StaleCorpus` as the negative arm and `NotFound::EmptyCorpus` for an empty list — three distinct outcomes a boundary test can key on.

## Conventions to follow
- **Crate-local integration tests** in `crates/{crate}/tests/*.rs` (`conductor-verify/tests/` holds `preflight.rs`, `preflight_spawn.rs`, `readback.rs`, `verdict.rs`, `expected_slo.rs`, `common/`) — the natural homes for the verify-side killing tests.
- **Assert through the public seam**, since every survivor symbol is crate-private (graph result above).
- **`ObsSink::File` + `assert_fs::TempDir`** is the established self-obs test shape (`canary_wire.rs:222`, `port_occupier.rs:71`, `obs_span.rs:20`) — note all three install the global subscriber, so the same interference class exists wherever two of them could run concurrently in one process.

## New files to create
- None required. (P4 may add test modules to existing files; no new crate, no new source module.)

## Files to modify
- `crates/conductor-run/tests/canary_wire.rs` — the runner-portability fix (`:211-268` witness test; `:220` env write).
- `crates/conductor-verify/tests/preflight.rs` — canary freshness-boundary and poll-loop killing tests.
- `crates/conductor-verify/tests/readback.rs` — the `MAX_LINE_BYTES` boundary test (or a new sibling under `tests/`).
- `crates/conductor-run/` test surface (file TBD at P4) — `declares` predicate matrix + journal-stamp magnitude assertions, asserted through public observables.
- `crates/conductor-cli/Cargo.toml` (`:19`) — remove `tracing.workspace = true`.
- `Cargo.lock` — moves as a consequence of the dependency removal (supply-chain gate input).

## Extract correction carried into P4 (overrides the obs extract)
The obs extract states: *"obs-plan §4 (Auto-instrumentation per surface — `cli` row) requires manual
`#[tracing::instrument]` on `fn main()` and core scenario handlers."* **Verified against the source: it does
not.** obs-plan `:48` (the cli row) names *"`tracing` 0.1.x crate + `tracing-subscriber` JSON formatter"* as the
surface's hook; the `#[tracing::instrument]` mandates sit on the **desktop-webview** (`:49`) and
**ipc-internal** (`:50`) rows, both about `#[tauri::command]` handlers. `conductor-cli` satisfies its row
through `conductor_core::init_observability` at `main.rs:28` — the subscriber lives in `conductor-core`. So
removing the unused `tracing` declaration breaches no obs mandate, and the rider needs no operator fork.

## Open questions
1. The `MAX_LINE_BYTES` bound is **16 MiB** and `read_line` is private, so a genuine over-limit test must either allocate a >16 MiB line (slow, memory-heavy, but honest) or make the bound injectable — which arch's "no new config handle / no test seam" constraint leans against. → blocks: **plan-decision** (P4 must choose before synthesis).
2. Whether the runner-portability fix belongs in the test (scope the count assertion to this test's own `run_id`, which the witness lines already carry) or in the harness (serialize the affected binary) — the former preserves the "real artifact from the real production init path" premise obs requires; the latter is a nextest/cargo-test configuration change. → blocks: **plan-decision**.
