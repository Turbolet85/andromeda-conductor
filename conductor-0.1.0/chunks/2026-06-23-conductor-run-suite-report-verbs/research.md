# Codebase Research — 2026-06-23-conductor-run-suite-report-verbs

## Scope
- **Depth:** deep · **Reads:** 19 files · **Globs:** 2 · **Graph queries:** 1
- This is the **integration chunk**: the CLI is the first/only composition root that wires the five
  engine seams (core·timeline·emit·verify·report) into an end-to-end run. Research mapped every
  public entry point + the data types that flow between seams.

## Files inspected
- `crates/conductor-cli/{Cargo.toml,src/main.rs,tests/cli_smoke.rs}` — current skeleton: `main()` only
  calls `conductor_core::init_observability("conductor", None)`; deps = `conductor-core` only; smoke =
  `Command::cargo_bin("conductor").assert().success()` + a tempdir fixture. This chunk's starting point.
- `crates/conductor-core/src/lib.rs` — the public surface (everything the CLI needs is re-exported):
  `init_observability`/`mint_run_id`/`now_rfc3339`/`ServiceIdentity`, `Scenario`/`PId`/`SloTier`,
  `RunRecord`, `Verdict`/`ReportState`/`Lamp`, `ExpectedCheck`/`ClaimClass`/`ComparisonKind`,
  `Decision`/`HeadlessResolver`/`HoldPoint`/`resolve_hold`/`PauseResolver`, `resolve_under`, `sanitize_error`.
- `crates/conductor-core/src/scenario.rs` — `Scenario{name,p_ids,seed,slo_tier,phases,jitter_ms,expected}`;
  `Scenario::from_toml_str(&str) -> Result<Scenario>` (parse→`CoreError::Config`, garde→`CoreError::Validation`).
- `crates/conductor-core/src/phase_spec.rs` — `PhaseSpec{name,gap_ms,emission:EmissionSpec}`;
  `EmissionSpec{signal:Signal}` (`#[non_exhaustive]`); `Signal ∈ {Traces,Metrics,Logs}`. **Coarse — see Graph impact.**
- `crates/conductor-core/src/run_record.rs` — the 11-field envelope; `RunRecord::blocked(run_id,seed,scenario,p_ids,slo_tier)`
  and `RunRecord::measured(...11 args...)`. Golden-locked field order + wire forms.
- `crates/conductor-core/src/obs.rs` — `init_observability(default_service_name,run_id:Option<String>) -> ServiceIdentity`
  (global subscriber + panic hook, first-install-wins; resolves `CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV`).
  `mint_run_id()` (`YYYY-MM-DDTHH-MM-SS-mmm`), `now_rfc3339()` (`…Z`). Both `std::time`.
- `crates/conductor-core/src/config_path.rs` — `resolve_under(base:&Path,candidate:&Path) -> Result<PathBuf>`:
  rejects absolute / `..` / symlink-escape; base must exist, candidate need not. The `CONDUCTOR_*` edge guard.
- `crates/conductor-core/src/pause.rs` — `HeadlessResolver::proceed()` (never blocks, answers `Go`);
  `resolve_hold(&resolver,&hold) -> HoldResolution`; `HoldPoint{scenario,p_id,step,prompt,allow_no_go}`.
- `crates/conductor-timeline/src/lib.rs` + `scheduler.rs` + `phase.rs` + `convert.rs` —
  `impl From<&Scenario> for PhaseTimeline` (infallible, timing-only); `run_timeline(&PhaseTimeline,seed) ->
  Result<Vec<PhaseTransition>,TimelineError>` (async, collects ALL transitions, **emits nothing**);
  `PhaseTransition{index,name,elapsed_ms:u128}` (virtual-clock ms).
- `crates/conductor-emit/src/lib.rs` + `client.rs` — `probe_egress(endpoint) -> Result<(),EmitError>`;
  `TraceEmitter::connect(endpoint).await` + `.export(ExportTraceServiceRequest)`; `LogsEmitter` sibling;
  request builders (`trace_request`, `error_trace_request`, `severity_logs_request`, …) take **their own
  rich spec args**, not `EmissionSpec`. `DEFAULT_OTLP_ENDPOINT="http://127.0.0.1:4317"`, `DEFAULT_CONNECT_TIMEOUT=5s`.
- `crates/conductor-verify/src/lib.rs` + `preflight.rs` + `slo.rs` + `record.rs` —
  `run_preflight(client,manifest,canary,data_dir) -> Result<ReadyState,VerifyError>` and
  `preflight_boot(transport,…)` (connect/`initialize` failure → `Blocked` ReadyState, never `Err`);
  `ReadyState.ready` / `.report_state()`; `evaluate_check(check,observed,tier,emitted_ms,observed_ms) ->
  CheckOutcome`; `CheckOutcome::to_run_record(run_id,seed,scenario,p_ids,emitted,observed,fingerprints) -> RunRecord`.
- `crates/conductor-report/src/lib.rs` + `journal.rs` + `db.rs` + `report.rs` —
  `JournalWriter::create(runs_dir,run_id)` + `.append(&RunRecord)` (create+append, never truncates);
  `RunsDb::open(runs_dir)` + `.insert(&RunRecord)` + `.get(run_id,scenario)` (bound params; PK `(run_id,scenario)`
  → duplicate is `Err`); `RunReport::write(runs_dir,run_id,&[RunRecord]) -> PathBuf` (`create_new` → repeat
  run_id is a loud `Err`) + `RunReport::render(run_id,&records) -> String` (pure, deterministic).
- `scenarios/*.toml` — **31 scenarios** (the full Epoch-7 catalog) the `suite` verb iterates.

## Graph impact (code-graph query → tree-query-2026-06-23-conductor-run-suite-report-verbs.json)
- **`crate_edges` (resolved usage):** the ONLY edges are `conductor-{cli,report,tauri,timeline,verify} → conductor-core`.
  **The seams are mutually independent** — timeline/emit/verify/report do NOT depend on each other.
  → The CLI is the **sole composition root**; wiring it adds NEW outbound edges
  `conductor-cli → {timeline,emit,verify,report}`. `conductor-cli` has zero inbound consumers (leaf bin) →
  the change has **zero cross-crate blast radius** beyond the new Cargo edges.
- **The two unbuilt content bridges** (a direct consequence of the seam independence):
  1. **phase → emit:** `EmissionSpec` carries only a coarse `Signal` class; `convert.rs` explicitly drops
     the emission descriptor ("rides in the config model for the Epoch-3 emission seam") and the emit seam
     built **standalone** rich builders that were never wired to `EmissionSpec`. So nothing maps a scenario
     phase to a specific OTLP incident shape. **Unbuilt.**
  2. **check → read-back observed:** `evaluate_check` needs an `observed:&str` per `ExpectedCheck`, but
     nothing maps an `ExpectedCheck` to *which* MCP tool to call and *how* to extract that observed string
     from the read-back. **Unbuilt.** Both bridges are LIVE-Pulse-only (can't be CI-verified) → they are the
     heart of the Epoch-10 "Live-Pulse E2E proof" chunks.

## Patterns detected
- **Runtime-agnostic seams, caller owns the runtime** (`scheduler.rs:35`, `client.rs:41/54`): `run_timeline`
  / `probe_egress` / emitters / preflight are all `async` with the caller supplying the runtime → the CLI's
  `#[tokio::main(flavor="current_thread")]` IS that runtime (arch [Async Runtime Flavor]).
- **Verdict/error wall everywhere** (`scheduler.rs:23`, `journal.rs:20`, `db.rs:37`, `report.rs:27`,
  `preflight.rs:86`): every seam returns typed `Err` for harness faults; verdicts/states are `Ok` values.
  The CLI maps `Err` → anyhow at the edge; states → exit code.
- **Already-resolved `runs_dir` contract** (`journal.rs:46`, `db.rs:59`, `report.rs:43` doc-comments all say
  "the cli edge applies `CONDUCTOR_RUNS_DIR` + `resolve_under`"): the persist seam is handed a resolved
  `&Path` — **the resolution is explicitly ch1's job** at the binary edge.
- **The producer bridge** (`record.rs:22`): `CheckOutcome::to_run_record` assembles a measured `RunRecord`
  with `state = verdict.default_report_state()`; a context-specific state (degraded_mode `KnownResidual`)
  builds via `RunRecord::measured` directly.
- **Preflight degrades to Blocked, never Err** (`preflight.rs:180`): `preflight_boot` maps a failed
  spawn/`initialize` to a `Blocked` `ReadyState` with `UNREACHABLE_PRECONDITION`. → A run with **no live
  Pulse** (CI: no `andromeda-pulse-mcp` at the fixed path) deterministically yields `Blocked`, short-circuiting
  before emission. **This is the CI-testable path.**
- **Spans pre-named in the seams** (`scheduler.rs:34` `timeline.execute`, `client.rs:67` `emit.batch`,
  `preflight.rs:85` `verify.readback.preflight`): the CLI adds the root `scenario.run` span (obs-plan §4).

## Conventions to follow
- `#[tokio::main(flavor="current_thread")]` on `main`; `init_observability("conductor", Some(run_id))` FIRST,
  with the run_id pre-minted (`mint_run_id`) so the same id stems the journal/report and tags every log line.
- anyhow only at the `conductor-cli` edge; `Display` (sanitized via `sanitize_error`/`redact_value`), never `Debug`;
  no host paths / struct names to stderr or artifacts (security-plan §Error Handling; `report.rs`/`db.rs` tests
  already assert no leaks in artifacts).
- Resolve `CONDUCTOR_RUNS_DIR`/`CONDUCTOR_SCENARIOS_DIR`/`CONDUCTOR_CONTRACT_MANIFEST` via `resolve_under` at
  the edge before any read/write (`config_path.rs`).
- Exit codes: 0 on all `Pass`; non-zero ONLY on a hard `Fail`; `Blocked`/`ManualCheck`/`KnownResidual`/
  `CalibrationRegion` are reported states, not non-zero exits (test-plan §1 / verification-harness.md).
- `RunsDb` PK is `(run_id, scenario)` → within one run each scenario is a distinct row (suite is fine);
  `RunReport::write` is `create_new` → one report per run_id (the per-run run_id makes this safe).

## New files to create (shape finalized at P4)
- `crates/conductor-cli/src/cli.rs` — clap `Cli` + `Commands{Run,Suite,Report}` derive structs + global flags.
- `crates/conductor-cli/src/commands/{run,suite,report}.rs` — the three verb handlers.
- `crates/conductor-cli/src/pipeline.rs` (likely) — the run-orchestration glue (resolve→preflight→[emit→verify]→persist).
- `crates/conductor-cli/src/exit.rs` (maybe) — the verdict/state → process-exit-code mapping.

## Files to modify
- `crates/conductor-cli/Cargo.toml` — add `clap`(derive), `tokio`(current_thread/macros), `anyhow`, `tracing`,
  + `conductor-{timeline,emit,verify,report}` (workspace deps).
- `crates/conductor-cli/src/main.rs` — replace the init-only stub with bootstrap + clap parse + dispatch.
- `crates/conductor-cli/tests/cli_smoke.rs` — extend into help/exit-code/run-Blocked/suite/report E2E (`assert_cmd`).

## Open questions
1. **(Material — resolve at P4 via AskUserQuestion)** How far does ch1 execute the LIVE measured path? The
   two content bridges (faithful per-scenario emission; per-check read-back observed-extraction) are unbuilt
   and live-only. Options: (A) full orchestration with *coarse* live emit/verify, faithful bridges deferred to
   Epoch-10; (B) structural skeleton — wire resolve→preflight→persist, defer emit/verify entirely to Epoch-10;
   (C) build faithful bridges now (largest, not CI-verifiable, overlaps Epoch-10).
2. **(Default: ch2)** `preflight` is consumed internally by `run`/`suite` regardless; a *standalone*
   `conductor preflight --json` subcommand naturally belongs to ch2 (`boot=preflight`), not ch1's run/suite/report.
3. **(Epoch-10)** The read-back observed-extraction map (ExpectedCheck → MCP tool + observed string) is
   live-only and is the substance of the Epoch-10 MCP-verified proof — out of scope here.
