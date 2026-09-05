# Codebase Research — 2026-09-05-audit-corrective

## Scope
- **Depth:** deep · **Reads:** 19 files/regions (render.rs full · coverage.rs full · jsonrpc.rs full · obs.rs:195-250 +
  :470-500 · commands.rs fn index + :165-245 + :255-360 + :400-525 · lib.rs header + item map + inline-test index ·
  dispatch_wire/canary_wire/canary_obs_witness heads · jsonrpc_correlation.rs:15-60 · verify `tests/common` index ·
  cli_smoke.rs structure · the audit's five `c-mutation-*.json` + `s-mutants.sh` + `s-record.py` · test-plan §4/§10/§12 ·
  `.config/nextest.toml` · the two Epoch 6a disposition plans/ledgers) · **Globs/Greps:** 14 · **Graph queries:** 8
  (rust plane, trace `.andromeda/runs/2026-09-05T18-11-30Z-phase/tree-query-2026-09-05-audit-corrective.json`).
- **Harness rules consulted:** none — no live leg in this chunk. `.claude/rules/testing.md` `## Session Additions` was read
  for the mutation read-out discipline (2026-09-03 ×3: nested `mutants.out/`; a TIMEOUT is a hang fixed in the TEST's
  await; an acceptance covers only the code its cited rule's mechanism lives in).

## Files inspected
- `crates/conductor-cli/src/render.rs` (full, 669 lines) — every public render delegates to a `*_styled(…, color: bool)`
  core (`:74-133`); the 20 inline tests (`:350-668`) call ONLY the cores, so the public wrappers' bodies never execute under
  test — which is why `paint` / `hold_line` / `envelope_caption` / `coverage_summary` survive whole-body replacement. The tty
  gates `stdout_color` (`:153-157`) / `stderr_color` (`:161-165`) are three-condition (`IsTerminal` ∧ `NO_COLOR` unset ∧
  `TERM != dumb`), private, and read the process's own handles. `coverage_summary_styled` (`:285-314`) is the roll-up W4b
  shares with `coverage.rs::summary_line`; `wire` / `latency` / `fingerprints` (`:328-348`) are the results-table cell
  helpers whose survivors no test's content assertion reaches (the plain-table test asserts `[PASS]` / `[BLOCKED]` / `—`
  only).
- `crates/conductor-report/src/coverage.rs` (full, 260 lines) — `summary_line(rows, unbacked)` (`:82-101`) is byte-for-byte
  the same roll-up arithmetic as `render.rs:285-314`, differing only in the `**Capabilities**` prefix and the absence of a
  tint; its tests (`:116-156`) lock the exact FORMAT over a synthetic set and the degenerate sets — the golden a shared
  computation must keep passing.
- `crates/conductor-verify/src/jsonrpc.rs` (full, 89 lines) — `request` (`:39-60`) writes then loops on `read_line`,
  skipping any message whose `id` ≠ ours (`:49`); `write_message` (`:69-75`). Under the `==` mutant every matching reply is
  skipped and the loop blocks in `read_line`; under `Ok(())` nothing reaches the stub and `read_line` blocks. Both are hangs.
- `crates/conductor-core/src/obs.rs:195-250` + `:470-500` — `now_rfc3339()` (`:200-206`) renders `civil_from_unix(secs)`
  into the RFC-3339 `journal_emitted_at` stamp; `civil_from_unix` (`:218-234`) is Hinnant's `civil_from_days` with the
  negative-era arm at `:224` (`if z >= 0 { z } else { z - 146_096 }`) — dead for `secs: u64` (`z ≥ 719_468`). The only
  existing test is `civil_from_unix_known_values` (`:482-485`: epoch + `1_000_000_000`), neither straddling a 100- or
  400-year term boundary.
- `crates/conductor-tauri/src/commands.rs` fn index + `:165-260` + `:262-355` + `:400-525` — `run_report` (`:172-201`) and
  `run_envelope` (`:231-259`) share the 13-line prologue (`runs_dir()?` → `resolve_under` a supplied id / `latest_run_id`,
  every `Err` via `sanitize_error`, the no-run arm logging then returning empty); the test helpers `invoke` (`:408-422`) and
  `invoke_expecting_error` (`:507-524`) share the 12-line `InvokeRequest` literal. The accepted tauri roster sits at
  `:272:8` (`!validate_selection` in `start_run`) and `:307:5` (`run_thread` body) at HEAD.
- `crates/conductor-run/src/lib.rs` header (`:1-46`) + item map + inline-test index — 1944 lines; imports at `:18-44`; the
  seams by line range: gate + canary `:47-130` / `:268-338` / `:413-511`; lifecycle `:131-267`; run-contract / preconditions
  `:339-412`; execution core `:512-864`; envelope trio `:865-925`; live driver `:926-1005`; `#[cfg(test)] mod tests`
  `:1006-1944` (52 test fns).
- `crates/conductor-run/tests/{dispatch_wire.rs:1-70, canary_wire.rs:1-50, canary_obs_witness.rs:1-60}` — each declares its
  own `Traces` alias, `Capture` struct with the `TraceService` impl, and a `start_stub()` binding `127.0.0.1:0`; identical
  modulo `dispatch_wire`'s extra `LogsService` arm. No `tests/common/` exists in conductor-run at HEAD.
- `crates/conductor-verify/tests/common/mod.rs` — `serve_stub(io, StubConfig)` (`:134`), `StubConfig` (`:50`), `WireLog`
  (`:27`), `DECOY_TOOL`: the shared stub every duplex test drives; the natural home of a bounded-await helper.
- `crates/conductor-verify/tests/jsonrpc_correlation.rs:15-60` — `#[tokio::test(flavor = "current_thread")]` (no
  `start_paused`) driving `connect_transport` → `list_tools` / `query_incident_list` over `tokio::io::duplex(4096)`; the
  await shape every other duplex test repeats (`readback.rs` 10 async tests · `preflight.rs` 15 · `readback_shape_witness.rs`
  1 · `preflight_spawn.rs` 1 (real child) · `jsonrpc_line_bound.rs` 2). Only ONE verify test file uses `start_paused`.
- `crates/conductor-cli/tests/cli_smoke.rs` (structure) — subprocess tests via assert_cmd with a TempDir of copied fixtures;
  `run_with_unknown_target_is_a_sanitized_error_with_a_hint` (`:140`) asserts only the `hint:` LABEL;
  `piped_coverage_output_carries_no_ansi_escapes` (`:344`) is the escape-absence precedent; `report_renders_the_latest_run`
  (`:270`) and `run_by_name_blocks_without_pulse_and_exits_zero` (`:102`) are where content assertions for `paint` /
  `paths.rs:73` land.
- `.andromeda/runs/2026-09-05T08-35-52-code-audit/c-mutation-conductor-{cli,verify,core,tauri}.json` + `s-mutants.sh` +
  `s-record.py` — the survivor lists with coordinates (cli 41 · verify 14 missed + 2 timeouts · core 6 · tauri 3), the tier
  invocation (`cargo mutants -p <unit> --test-tool=nextest --jobs 2 --output <dir>`; core `--shard 1/4`), and the jscpd
  form (`jscpd crates --format rust --reporters json --output <dir> --silent`; jscpd 5.0.16 resolves on PATH).
- `.andromeda/test-plan.md:228` (§4 mutation instrument), `:466` / `:502` (§10), `:587-613` (§12 rosters), `:216 / :456 /
  :478 / :508 / :567` (`cargo audit --deny warnings`) — the disposition rule, the roster coordinates, and the gate wording.
- `conductor-0.2.0/chunks/2026-09-03-conductor-tauri-survivors-dispositioned/plan.md` §Test Commands +
  `…-conductor-run-composition-root-survivors-dispositioned/evidence/disposition-ledger.md` — the precedent shapes: the
  per-run `--output` dir, `missed.txt` = accepted set, the ledger table (site · mutation · killed-by / rule), the pre/post
  tally table.
- `.config/nextest.toml` — `retries = 0` in both profiles; NO `slow-timeout`, so a hung test is not terminated by the runner.

## Graph impact (from the code-graph query; rust plane, `db_state` built)
- **conductor-run's external API surface** (Q7: `refs` where the callee is defined under `conductor-run/src/` and the
  referencing file is outside it) — **39 symbols**: fns `attribute_by_liveness` · `canary_spec` · `canary_storm_seed` ·
  `canary_warmup_seed` · `classify_run` · `drive_run` · `emit_canary_storm` · `evaluate_lifecycle` · `observe_preconditions`
  · `persist` · `preflight` · `probe_resolve_lifecycle` · `read_envelope` · `readiness` · `select_resolve_target`; consts
  `AUTO_RESOLVE_IDLE_SECONDS` · `CANARY_SERVICE_NAME` · `CANARY_STORM_COUNT`; types `Dispatcher` · `LifecycleObservation`
  (+ fields `after` / `before` / `control` / `idle_seconds` / `resolved`) · `LifecycleVerdict` (+ variants `NoControl` /
  `Proven` / `ProvenByLiveness` / `StillActive` / `Unattributable`) · `RunEvent` (+ `count` / `stage`) · `RunStage` (+
  `Aborted`) · the `record` / `checks` fields of `ScenarioOutcome`. Every one keeps its `conductor_run::` path via `pub use`
  after the split — this list is the byte-stability oracle.
- **Bins' usage** (Q8, the 31 rows on `conductor-cli/` + `conductor-tauri/`): cli `commands/{run,suite}.rs` → `preflight` ·
  `persist` · `classify_run` · `checks` · `record`; `commands/preflight.rs` → `readiness`; `commands/preconditions.rs` →
  `observe_preconditions`; `commands/mod.rs` → `persist`; `tests/cross_surface_parity.rs` → `preflight` · `drive_run`;
  tauri `commands.rs` → `preflight` · `drive_run` · `classify_run` · `read_envelope` · `RunEvent` / `RunStage::Aborted` /
  `count` / `stage`. `Preflight` (the struct) is never named outside the crate (crate-private fields) but is `preflight`'s
  return type — stays `pub` at the root.
- **Crate edges** (Q3): `conductor-run` → core · timeline · emit · faults · verify · report; `conductor-tauri` → core · run;
  `conductor-cli` → core · report · run (manifest `Cargo.toml:13-15`). W4b's shared roll-up rides cli→report (exists);
  no new edge anywhere.
- **cli survivor callers** (Q4): `paint` is called from `commands/preflight.rs:22`/`:28` and `commands/report.rs:15` (so a
  subprocess `report` / `preflight` content assertion reaches it); `stdout_color` from the 8 public wrappers in render.rs;
  `stderr_color` only from `error_block:82`; `wire` from `results_table_styled:247`.
- **`civil_from_unix`** (Q5): callers `now_rfc3339` (obs.rs:205) and the test at `:482-483` — a pure function with one
  production caller; a boundary-date table needs no clock.
- **`request` / `write_message`** (Q5): `request` ← `initialize:88` · `list_tools:109` · `call_tool:127` (client.rs);
  `write_message` ← `request:42` · `notify:65` — every read-back call funnels through the two mutated lines.
- **Module-name collision check** (Q6): conductor-run defines only `dispatch` and two `tests` modules — `canary` ·
  `preconditions` · `lifecycle` · `execute` · `envelope` · `drive` are free.
- **Public-API stability probe for `observe_preconditions`**: 1 external caller (`commands/preconditions.rs`) + arch
  §Standard Contracts cites it by path — the path is load-bearing prose, not only code.

## Patterns detected
- **`*_styled(…, color: bool)` seam** (render.rs:167-314): every render has a pure core taking the color decision as a
  parameter; tests assert plain + colored through it. The tty GATE has no such seam — that is the one W1 adds.
- **Read-the-host boolean accepted-deliberate** (spawn.rs:116 pair, test-plan §12 :612; testing.md 2026-09-03): a boolean
  whose one arm needs a host state no hermetic test can set is accepted with the rejection of the host-dependent kill
  recorded. `stdout_color -> false` / `stderr_color -> false` are this class exactly.
- **Escape-absence via a piped subprocess** (cli_smoke.rs:344): assert_cmd captures stdout/stderr through pipes, so
  `is_terminal()` is false deterministically under both runners — the only host-independent way to assert NO escape bytes
  from a public wrapper (`stderr_color -> true` dies here).
- **Bounded hang → kill** (tauri chunk, testing.md 2026-09-03 :35): a TIMEOUT mutant is a hang; the fix is a bound on the
  TEST's await, converting the class from timeout to caught with no new assertion.
- **Own binary per process-global consumer** (canary_obs_witness.rs:1-9, test-plan §11): the witness must remain its own
  test binary; a `tests/common/mod.rs` module is compiled into each binary and changes nothing about process isolation.
- **Per-run unique `--output`** (tauri plan; testing.md 2026-09-03 :34): `cargo mutants --output <dir>` nests
  `mutants.out/` INSIDE the dir; a reused dir is stale by construction — the command line mints its dir at invocation.
- **Prologue-with-span-guard** (commands.rs:172-201, :231-259): each handler opens its own `tauri.command.<name>` span
  FIRST, then the shared prologue work; a hoisted helper must be called AFTER the guard, inside it.

## Conventions to follow
- **Disposition ledger shape**: the `conductor-run` ledger's header table (mutants found / missed / caught / unviable /
  timeout, pre vs post) + a `## Killed` table (site · mutation · killed by) + a `## Accepted-deliberate` table (site ·
  mutation · rule) — `evidence/disposition-ledger.md`; read-out from the nested `mutants.out/`, never the exit code.
- **Mutation invocation**: `cargo mutants -p <unit> --test-tool=nextest --jobs 2 --output target/mutants-<unit>-$(date -u
  +%Y%m%dT%H%M%SZ)` (per unit), `-f crates/conductor-core/src/obs.rs` for the file-scoped core re-run (`-f` resolves from the
  workspace root — test-plan §4).
- **jscpd invocation**: `jscpd crates --format rust --reporters json --output target/jscpd-$(date -u +%Y%m%dT%H%M%SZ)
  --silent` — the audit's exact form, same tool version, so the pair list is comparable.
- **Tracing spans**: manual `tracing::info_span!("tauri.command.<name>").entered()` per handler (obs-plan §4); no
  `#[tracing::instrument]` on a `#[tauri::command]`; `target` is the module path and is allowlisted.
- **Error edge**: `sanitize_error(&e)` on every `Err` crossing a `#[tauri::command]`; `Display`, never `Debug`.
- **rstest `#[case]` tables** for discriminating inputs (test-plan §4); `assert_eq!` exact strings for rendered shapes.
- **snake_case sibling modules** under the crate root, `pub use`d from `lib.rs` (the `dispatch.rs` precedent at `:38-39`).

## New files to create
- `crates/conductor-run/src/canary.rs` — the gate + canary half: `Preflight` · `preflight` · `readiness` ·
  `unreachable_state` · `canary_blocked_state` · `canary_gate` · `emit_canary` · `warm_up_canary_service` · `canary_poll`
  · `emit_canary_storm` · `canary_storm_seed` · `canary_warmup_seed` · `canary_spec` · `CANARY_STORM_COUNT` ·
  `CANARY_SERVICE_NAME` (+ the warm-up tests `:1889-1943` as its `#[cfg(test)]`).
- `crates/conductor-run/src/preconditions.rs` — `load_run_contract` · `observe_run_contract` · `declares` ·
  `observe_preconditions` (+ the `:1631-1743` tests).
- `crates/conductor-run/src/lifecycle.rs` — `LifecycleObservation` · `LifecycleVerdict` · `select_resolve_target` ·
  `AUTO_RESOLVE_IDLE_SECONDS` · `attribute_by_liveness` · `evaluate_lifecycle` · `probe_resolve_lifecycle` ·
  `active_incident_ids` (+ the `:1745-1836` tests).
- `crates/conductor-run/src/execute.rs` — `execute_scenario` · `ScenarioOutcome` · `route_read_back` · `state_for` ·
  `manual_record` · `severity_rank` · `now_ms` · `now_unix_nanos` · `classify_fault` · `ramp_factor` · `fault_span` ·
  `phase_guard` (+ the private-item tests: read-back routing `:1018-1200`, guards `:1202-1286`, classify/ramp/stamps
  `:1535-1630`, fault span `:1837-1888`).
- `crates/conductor-run/src/envelope.rs` — `persist` · `read_envelope` · `classify_run`.
- `crates/conductor-run/src/drive.rs` — `RunEvent` · `RunStage` · `drive_run`.
- `crates/conductor-run/tests/composition_root.rs` — the pub-item tests that leave `lib.rs`: `execute_scenario_blocks_when_
  gate_not_ready` · `blocked_envelope_is_seed_identified` · the three `drive_run_*` · `read_envelope_round_trips_…` ·
  `classify_run_flags_…` · `an_over_envelope_run_…` · `an_in_envelope_run_…` (names unchanged).
- `crates/conductor-run/tests/common/mod.rs` — `Traces` / `Logs` aliases, `Capture` (both service impls), `start_stub()`
  returning `(SocketAddr, Traces, Logs)`; the three wire tests `mod common;` it.
- `crates/conductor-run/tests/common/mod.rs` is the ONLY new test-support file in conductor-run; conductor-verify's
  bound helper goes into the EXISTING `crates/conductor-verify/tests/common/mod.rs`.
- `conductor-0.2.0/chunks/2026-09-05-audit-corrective/evidence/disposition-ledger.md` (+ the tally / pair-list /
  API-listing / probe-transcript files beside it).

## Files to modify
- `crates/conductor-run/src/lib.rs` — reduce to crate docs + `mod` declarations + `pub use` re-exports of the 39-symbol
  external surface and every other currently-`pub` item (`Preflight`, `ScenarioOutcome`, `readiness`, …); the inline
  test module removed (partitioned as above). Callers threading through the graph: NONE change (paths preserved) — the
  callers are the oracle, not the modify-set: `conductor-cli/src/commands/{run,suite,preflight,preconditions,mod}.rs`,
  `conductor-tauri/src/commands.rs`, `conductor-cli/tests/cross_surface_parity.rs`, the 16 conductor-run test binaries.
- `crates/conductor-run/tests/severity_harvest.rs:454` — the `"target":"conductor_run"` pin relaxed to a prefix match
  (the line is emitted by relocated `execute_scenario` code; obs-plan §3: `target` is the module path).
- `crates/conductor-run/tests/{dispatch_wire.rs, canary_wire.rs, canary_obs_witness.rs}` — replace the local `Capture` /
  `start_stub` scaffolding with `mod common;` uses (W4c).
- `crates/conductor-cli/src/render.rs` — (W1) a pure `color_enabled(is_terminal: bool, no_color_set: bool, term:
  Option<&str>) -> bool` the two gates call; new unit tests calling the PUBLIC wrappers (content presence) + the cell
  helpers + the tint-placement and derived-breakdown assertions; (W4b) `coverage_summary_styled` consumes the shared
  roll-up from `conductor_report`.
- `crates/conductor-cli/src/commands/mod.rs` — a `#[cfg(test)]` test for `exit_code` over a `Fail` record.
- `crates/conductor-cli/src/pause.rs` — `#[cfg(test)]` tests for the two `kind()` impls.
- `crates/conductor-cli/tests/cli_smoke.rs` — content assertions: the hint TEXT for the unknown-target edge; `run` by name
  loads exactly the named scenario (journal/record count 1, named); `CONDUCTOR_SEED` reaches `seed` and `--seed` beats it;
  `report` output carries the `[BLOCKED]` prefix (kills `paint`); piped stderr of an error edge carries no escape byte.
- `crates/conductor-report/src/coverage.rs` + `crates/conductor-report/src/lib.rs` — the shared roll-up (`pub struct
  CoverageRollup { total, in_scope, breakdown: String, out_of_scope, out_label }` + `pub fn coverage_rollup(rows,
  unbacked)`) beneath `summary_line`, re-exported from `lib.rs`; `summary_line` keeps its name and exact output (its
  format golden at `:116-156` is the proof).
- `crates/conductor-tauri/src/commands.rs` — `fn resolve_run_target(run_id: Option<String>) -> Result<Option<(PathBuf,
  String)>, String>` (or equivalent) hoisting the `runs_dir` + `resolve_under` + `latest_run_id` + `sanitize_error`
  prologue, called inside each handler's span guard; a test-side `fn request(cmd, body) -> InvokeRequest` builder shared by
  `invoke` / `invoke_expecting_error`. Error strings byte-identical (`unknown scenario or suite selection`, the sanitized
  edge strings).
- `crates/conductor-core/src/obs.rs` — `civil_from_unix`: `let era = z / 146_097;` (dead negative-era arm removed, comment
  naming the unsigned domain) + an rstest `#[case]` table over the boundary instants.
- `crates/conductor-verify/tests/common/mod.rs` — `pub async fn bounded<F: Future>(fut: F) -> F::Output` =
  `tokio::time::timeout(Duration::from_secs(20), fut).await.expect("read-back call hung past the bound")` (dev-dep
  tokio `time` is present, `Cargo.toml:31`); `crates/conductor-verify/tests/{jsonrpc_correlation.rs, readback.rs,
  preflight.rs, readback_shape_witness.rs, preflight_spawn.rs}` — every `connect_transport` / `connect_command` /
  `list_tools` / `call_tool` / `query_incident_list` / `run_preflight` await wrapped in it (the 32 `#[tokio::test]` fns
  across those files; `jsonrpc_line_bound.rs`'s two tests already terminate on a decode error and need no bound).
- `.andromeda/test-plan.md` · `.andromeda/architecture.md` · `.andromeda/security-plan.md` — NOT touchpoints (spec
  masters): Expected amendments 1–3 + 6 in the plan.

## Open questions
- **Does `cargo test -p conductor-verify` (the shared-process runner) keep the 20 s bound deterministic?** → blocks:
  implementation-scope. Under `cargo test` the duplex tests share one process; a 20 s real-clock bound is far above any
  measured test duration (the whole verify suite ran in seconds at the audit) and fires only under a genuine hang. If a
  duplex test is converted to `start_paused = true`, `timeout` auto-advances when idle and fires instantly — preferred where
  the test's own timing does not depend on the real clock. Either form is deterministic; /implement picks per test and
  records the choice in the ledger.
- **Which exact boundary instants kill all four live `civil_from_unix` mutants?** → blocks: implementation-scope. The
  derivation in scope W2 (2000-02-29 for the 400-year term; 2100-03-01 for the 100-year term; era neighbours) is
  hand-computed; the file-scoped mutation re-run is the arbiter — a still-missed mutant means the table needs another
  straddling date, not an acceptance.
- **Does any `render.rs` test need `NO_COLOR` set in-process?** → resolved: NO. Presence assertions through the public
  wrappers hold under either tty state; escape-absence goes through assert_cmd pipes. No `unsafe { set_var }` anywhere
  (the tauri chunk's rule).
