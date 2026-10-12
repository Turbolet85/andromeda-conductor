# Codebase Research — 2026-10-12-panel-shaped-types-retired

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 14 (+ 1 code-graph query, 1 `cargo tree` pair)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (auto-loaded with `testing.md`
  and `observability.md` when this pass read `crates/**/tests/**`), Session Additions included; 0 additions applied:
  the chunk plans no live leg (every new entry is proven hermetically; ports 4317/4318 are not bound — `inputs#I2` §7)
- **Platform issues consulted:** none — no runner-only bullet (the one CI run Setup read is green) and no CI-reading
  entry outside the operator leg
- **External inputs:** `inputs#I1` — the take-up word (the founder's «deployed, go» through overseer1; stop at P5) ·
  `inputs#I2` — the operator's phase directive (CARRY is the command-line half; a new verb is named by the operator,
  a new flag shown at P5; Pulse-facing path untouched; classifier-meeting acts listed at P5)

## Files inspected
- `crates/conductor-run/src/drive.rs` (full) — `drive_run` walks scenarios, polls `should_abort` before each and once
  after the last (`:76`, `:91`), persists ONCE (`:94`), emits `RunEvent { stage, count }` (`:68-71`, `:83-86`,
  `:106-109`). `RunEvent`/`RunStage` derive `Serialize` and `rename_all = "lowercase"` only for the Tauri `Channel`
  (`:16-41`). Three unit tests pin abort-before-first, abort-during-last and the Blocked spine (`:119-213`).
- `crates/conductor-cli/src/commands/suite.rs` (full) — `suite` runs its OWN loop over `execute_scenario`
  (`:31-36`), ticks the spinner per scenario, persists once (`:39`), prints the envelope caption then the results
  table (`:40-43`). No abort poll: Ctrl-C today kills the process before `persist`, so nothing finished is kept.
- `crates/conductor-cli/src/commands/run.rs` (full) — one scenario; nothing between scenarios to stop at.
- `crates/conductor-cli/src/commands/report.rs` (full) — reads the journal and prints `Run report <id>` + the
  results table (`:17-20`); never reads `run_envelope`.
- `crates/conductor-cli/src/commands/mod.rs` (full) — `exit_code` (non-zero only on a hard `Fail`, `:31-37`) and
  `print_record`; `persist` is `conductor_run::persist` re-imported (`:22`).
- `crates/conductor-cli/src/cli.rs` (full) — `Commands` declares `run` · `suite` · `report` · `preflight` ·
  `coverage` · `preconditions` · `cleanup`; no listing verb.
- `crates/conductor-cli/src/main.rs` (`:55-95`) — `hint_for` points "no scenario matches" at "list scenarios in
  scenarios/" (`:66`); `dispatch` maps each verb (`:78-94`).
- `crates/conductor-cli/src/pause.rs` (`:25-125`) — `PromptResolver::resolve` (`:34-37`, the interactive arm) prints
  `render::hold_line` then calls `confirm`; neither reads `hold.checklist`. `CliResolver::resolve` (`:114-119`) only
  dispatches to `PromptResolver` (interactive) or the core `HeadlessResolver` (headless), printing nothing itself.
- `crates/conductor-cli/src/render.rs` (`:85-163`, `:190-215`) — `hold_line` prints `[HOLD] — operator pause ·
  scenario · P-ID · step` (`:201-206`); its doc names "the desktop titlebar hold signature" (`:94`);
  `envelope_caption` is `None` in-envelope (`:101-108`); `spinner` draws to stderr only on a terminal (`:138-150`).
- `crates/conductor-cli/src/paths.rs` (index of `fn`s) — `load_all_scenarios` / `load_scenario` load through
  `scenario_files` + the capability manifest (`:77-94`, `:40-41`); the listing can reuse the same manifest load.
- `crates/conductor-cli/Cargo.toml` (`[dependencies]`) — edges to `conductor-core`, `conductor-report`,
  `conductor-run` already declared; `tokio` features `["macros", "rt"]`.
- `crates/conductor-core/src/scenario_catalog.rs` (full) — `ScenarioSummary`, `list_scenarios`, `load_catalog`,
  `scenario_files`, `SUITE_SELECTION`, `validate_selection`; every doc line frames them "for the picker" (`:1-19`,
  `:40`, `:77`).
- `crates/conductor-core/src/obs.rs` (`:40-80`) — `ObsSink { Stderr, File(PathBuf) }`; the Tauri sink survives only
  as doc text (`:59-60`).
- `crates/conductor-run/src/envelope.rs` (`:1-80`) and `src/lib.rs` (full) — `read_envelope` (`:41-56`, doc
  "the desktop run-report", "`conductor-tauri` has no `conductor-report` edge") logs one `info!` boundary line;
  `lib.rs:1-16` names "both shells" / `conductor-tauri`.
- `crates/conductor-run/tests/{envelope_fixture,lamps_fixture}.rs` (fn index + heads) — 4 + 2 tests; fixtures
  `tests/fixtures/over-envelope.toml` and `tests/fixtures/lamps-journal.jsonl`.
- `crates/conductor-cli/tests/cross_surface_parity.rs` (`:1-60`, fn index) — one test; its in-process arm calls
  `conductor_run::drive_run` (`:102`) under the "Tauri arm" naming (`:15-16`, `:58`, `:93-154`, `:175`).
- `crates/conductor-run/tests/composition_root.rs` (`:20-70`) — pins the re-export surface, `RunEvent`/`RunStage`
  by name (`:43-44`) and `read_envelope`'s signature (`:28-29`); its doc says read_envelope is "the path the a11y
  fixtures and both shells depend on" (`:64`).
- `crates/conductor-core/src/drift.rs` (`:145-155`, `:355-365`, `:455-470`) — `list_scenarios`'s only non-catalog
  call is the backing gate's TEST (`:361`); `"ScenarioSummary"` is a leak-token in a sanitization test (`:463`).

## Graph impact
Query: `refs WHERE callee_name IN (drive_run, RunEvent, RunStage, read_envelope, list_scenarios, validate_selection,
SUITE_SELECTION, ScenarioSummary, MAX_CHECKLIST_TEXT, hold_line, envelope_caption, scenario_files, load_catalog)`,
rust plane, `rows: 76`, `db_state: fresh` (trace `.andromeda/runs/2026-10-12T06-44-11-phase/tree-query-2026-10-12-panel-shaped-types-retired.json`).
- **drive_run** — refs at `crates/conductor-run/src/drive.rs` (3, its own tests), `src/envelope.rs` (3, tests),
  `src/lib.rs` (re-export), `crates/conductor-cli/tests/cross_surface_parity.rs` (1). **No production caller** —
  the graph agrees with the 2026-10-11 research (`drive_run` keeps only test callers).
- **RunEvent / RunStage** — `drive.rs` (4 / 11), `lib.rs` re-export, `tests/composition_root.rs:43-44`. No other
  consumer: deleting them touches drive.rs, lib.rs, composition_root.rs.
- **read_envelope** — `envelope.rs` (3, def + unit test), `lib.rs` re-export, `tests/composition_root.rs` (3),
  `tests/envelope_fixture.rs` (1). No production caller.
- **list_scenarios** — `scenario_catalog.rs` (def), `lib.rs` re-export, `drift.rs` (1, test at `:361`). No
  production caller.
- **validate_selection / SUITE_SELECTION** — `scenario_catalog.rs` (6 / 2, def + its own tests) + `lib.rs`
  re-export. **No caller outside their module**: they existed for the Tauri `start_run` alone.
- **ScenarioSummary** — `scenario_catalog.rs` (4) + `lib.rs`; plus a leak-token STRING at `drift.rs:463` (not a ref).
- **MAX_CHECKLIST_TEXT** — `scenario.rs` (2, the two garde `length` attributes) + `lib.rs` re-export.
- **hold_line** — `render.rs` (def) + `pause.rs` (1). **envelope_caption** — `render.rs` + `commands/run.rs` +
  `commands/suite.rs`.
- **scenario_files / load_catalog** — production readers in `paths.rs`, `load_envelope.rs`, `scenario.rs`,
  `tests/scenario_audit_gate.rs`; untouched by this chunk.
- Grep cross-check (async-fn index-gap rule): `git grep -n 'drive_run\|read_envelope\|validate_selection\|
  SUITE_SELECTION\|list_scenarios\|ScenarioSummary\|RunEvent\|RunStage' -- crates` returns the same file set plus
  doc-comment mentions only (`drift.rs:151`).

## Patterns detected
- **Verdict lines and captions go to stdout, human messages to stderr** (`commands/suite.rs:40-43`,
  `main.rs:55-75`): the caption prints once above the table; errors print `error:` + `hint:`.
- **A run-level qualifier is computed, never stored twice** (`commands/run.rs:33-35`, `render.rs:101-108`): the
  caption renders from an `EnvelopeStatus`; a past run's standing is the same value read back from `run_envelope`.
- **The abort flag is polled between scenarios, never inside one** (`drive.rs:75-93`): a stop keeps every finished
  record and persists them once; a scenario already running completes.
- **A listing verb is a static table over a manifest-carrying load** (`commands/coverage.rs`, `render.rs:116-125`).

## Conventions to follow
- **Thin shell over a library function** (arch §Design Philosophy; `commands/*.rs` each ≤45 lines): every new
  entry is a `Commands` arm calling a `conductor-core` / `conductor-run` function.
- **`exit_code` decides the exit** (`commands/mod.rs:31-37`): a stopped suite exits on its kept records, never on
  the stop itself (outcomes are values).
- **isatty-gated prompt, headless never blocks** (`pause.rs`, `CliResolver::select`): checklist lines print above
  the confirm and in the headless path alike; no new prompt.
- **Inline `#[cfg(test)] mod tests` for the bin crate** (`commands/mod.rs:39-98`): the cli has no lib target, so
  CLI-internal logic is unit-tested inline; the real binary is driven by `assert_cmd` tests under `tests/`.

## Mechanisms re-derived (the equalities the plan rests on)
- **A graceful suite stop keeps what finished** — `drive_run(.., should_abort)` with `should_abort` returning true
  after scenario k yields k records persisted and a stopped outcome; measured by `drive.rs:173-213`
  (`drive_run_reports_an_abort_raised_during_the_last_scenario`, 1 record kept, journal written). The frame that
  produces it is `drive.rs:91-94` (re-poll after the loop, then `persist`).
- **No new package for a Ctrl-C handler** — `cargo tree --locked --offline -e normal -i signal-hook-registry`:
  `signal-hook-registry v1.4.8` is already in the graph through `tokio` (its `process` feature, enabled by
  `conductor-verify`) and through `crossterm` (`comfy-table`, `inquire`); `cargo tree -p conductor-cli -e features
  -i tokio` lists `signal-hook-registry` active and `signal` NOT active. So adding tokio's `signal` feature to
  `conductor-cli` turns on a feature over packages already locked; the package count (268) is the basis to
  re-measure at implement, never lockfile byte-identity.
- **A past run's standing is readable from the CLI** — `conductor-cli` already declares `conductor-report` and
  `conductor-run` (`crates/conductor-cli/Cargo.toml`), so `read_envelope`'s stated reason (the window crate had no
  `conductor-report` edge, `envelope.rs:44`) no longer holds; the CLI `report` path can call it directly.
- **A hold's checklist is on the value the CLI already receives** — `HoldPoint.checklist: Vec<ChecklistItem>`
  (`crates/conductor-core/src/pause.rs:71-76`, filled from the scenario at `crates/conductor-run/src/execute.rs:151`)
  reaches `PromptResolver::resolve(&HoldPoint)` through `CliResolver::resolve`'s interactive arm
  (`crates/conductor-cli/src/pause.rs:114-119` → `:34`); no plumbing is missing, only the print.

## New files to create
- `crates/conductor-cli/src/commands/catalog.rs` — the listing handler (the file name is fixed whatever word the verb takes)

## Files to modify
- `crates/conductor-cli/src/cli.rs` — the listing verb's `Commands` arm
- `crates/conductor-cli/src/main.rs` — dispatch the listing verb; the no-scenario hint points at it
- `crates/conductor-cli/src/commands/mod.rs` — register the listing handler
- `Cargo.lock` — rewritten only if Cargo records the tokio feature; its package set must not move
- `crates/conductor-cli/src/commands/suite.rs` — drive the catalogue through `conductor_run::drive_run` with a stop flag
- `crates/conductor-cli/src/commands/report.rs` — print the stored load-envelope caption above the table
- `crates/conductor-cli/src/pause.rs` — the hold prints its checklist items under the HOLD line
- `crates/conductor-cli/src/render.rs` — catalogue table, checklist lines, stop line; hold doc loses its desktop clause
- `crates/conductor-cli/src/paths.rs` — a catalogue read through the existing capability-manifest load
- `crates/conductor-cli/Cargo.toml` — tokio gains the `signal` feature
- `crates/conductor-cli/tests/cli_smoke.rs` — assert_cmd tests for the listing verb and the report caption
- `crates/conductor-cli/tests/cross_surface_parity.rs` — the "Tauri arm" becomes the in-process suite arm
- `crates/conductor-run/src/drive.rs` — `RunEvent`/`RunStage` leave; `drive_run` reports progress and a stopped outcome
- `crates/conductor-run/src/envelope.rs` — `read_envelope` and `persist` docs name the CLI, not the desktop
- `crates/conductor-run/src/lib.rs` — re-exports and crate doc follow
- `crates/conductor-run/tests/composition_root.rs` — the pinned surface follows the re-shaped exports
- `crates/conductor-run/tests/envelope_fixture.rs` — deleted
- `crates/conductor-run/tests/lamps_fixture.rs` — deleted
- `crates/conductor-run/tests/fixtures/over-envelope.toml` — deleted
- `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` — deleted
- `crates/conductor-run/tests/journal_conformance.rs` — its committed-fixture test leaves with the fixture
- `crates/conductor-core/src/scenario_catalog.rs` — `SUITE_SELECTION` and `validate_selection` leave; docs name the listing
- `crates/conductor-core/src/lib.rs` — re-exports follow
- `crates/conductor-core/src/scenario.rs` — the checklist bound's doc loses its dialog-row reason
- `crates/conductor-core/src/obs.rs` — the sink doc loses the Tauri backend
- `crates/conductor-core/src/pause.rs` — module and resolver docs lose the Tauri dialog

## Sweeps
- `drive_run|read_envelope|validate_selection|SUITE_SELECTION|list_scenarios|ScenarioSummary|RunEvent|RunStage` over
  `crates/`: 9 files · 8 changed · 1 no-change (`drift.rs` — `:151` doc names `list_scenarios`, which stays; `:361`
  test calls it; `:463` leak-token string kept while the type stays)
- `lamps-journal|over-envelope` over `crates scripts .github`: 3 files (`envelope_fixture.rs`, `lamps_fixture.rs`,
  `journal_conformance.rs:157-164`) · 3 changed · 0 no-change
- `journal_conformance|envelope_fixture|lamps_fixture|cross_surface|composition_root|E2E_SEED` over `.github
  scripts`: 1 hit (`ci.yml:219` runs `--test journal_conformance` whole) · 0 changed · 1 no-change (the job selects
  the target, not the test)
- `tauri|gui|desktop|picker|webview|dialog|both shells` over the files this chunk edits: 16 sites across
  `drive.rs`, `envelope.rs`, `run/src/lib.rs`, `scenario_catalog.rs`, `scenario.rs`, `obs.rs`, `core/src/pause.rs`,
  `render.rs:94`, `cross_surface_parity.rs`, `composition_root.rs:4,64` · all changed (this chunk's own types); the
  rest of the tree's window prose is `No-survivor sweep`'s (`working-route.md:136`)

## Open questions
- The listing verb's word → blocks: plan-decision (the operator names it, `inputs#I2` §3; P4 asks)
