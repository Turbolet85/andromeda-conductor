# P5 review — 2026-10-12-panel-shaped-types-retired · "Panel-shaped types retired"

Plan: `conductor-0.4.0/chunks/2026-10-12-panel-shaped-types-retired/plan.md` (527 lines, 348 of prose) ·
research: `…/research.md` · scope: `…/scope.md` · inputs: I1 the take-up word, I2 the directive, I3 your P4 answers.

## Goal
The panel-shaped types R1 lists leave or are re-framed for the command line:
- **Leaves:** `RunEvent` / `RunStage`, `SUITE_SELECTION` / `validate_selection`, the dialog-row reason for the
  checklist bound, the Tauri sink text, and the two webview fixtures with their seed-dir reader.
- **Re-framed and kept** (each serves a CARRY function): `drive_run`, `read_envelope`, `ScenarioSummary` /
  `list_scenarios`, and `MAX_CHECKLIST_TEXT = 200`.

The four functions without an entry point get one:
- `conductor scenarios` lists the catalogue (your word, B5).
- `conductor suite` stops on the first Ctrl-C. It keeps every finished record and exits 130; a second Ctrl-C ends at
  once.
- `conductor report` prints a past run's stored `[ENVIRONMENT-SUSPECT]` caption.
- The CLI hold prints its `[[checklist]]` items as indented `induced:` / `observe:` lines.

Nothing Conductor reads of Pulse moves.

## New command surface (directive §3)
- **New verb:** `conductor scenarios` (no argument) — named by you (`inputs#I3`).
- **New flags:** none.
  - The stop is the suite's Ctrl-C handling, not a flag. A `stop` verb or a stop file was rejected because it needs a
    socket, pipe or a new place on disk.
  - The past-run standing prints by default on `report`, like the caption on `run` / `suite`.
- **Changed behaviour of an existing verb:**
  - `suite`'s first Ctrl-C no longer kills at once: it finishes the running scenario, persists, and exits 130.
  - `report` gains the caption line for a suspect run, with no column change.
  - The interactive hold prints its checklist lines.
- No listener, socket, handle, place on disk or credential. `tokio` gains its `signal` feature on `conductor-cli`
  only, with no new package (lock census: 268 → 268 predicted, no package added against `0619048`).

## Top acceptance criteria
- **v4-01, claimed:**
  - the panel census is empty;
  - every function is exercised from the command line in the same gate;
  - the lock census reads 268 with nothing added;
  - the masters' no-interface line is present in 3 of 3;
  - CI is green on this chunk's push.
- The checklist bound: at 200 loads, at 201 or empty rejected as `CoreError::Validation`, and beside `expected`
  rejected as `CoreError::Config`. `dive` is kept.
- `conductor scenarios`: piped output carries no ANSI, an absent manifest is a harness fault (`error:` / `hint:`),
  and no host path leaks.
- A stopped suite's spinner stops in place, and the suite exits 130. A completed suite exits on the hard-Fail rule.
- Parity: `cross_surface_parity.rs` compares the in-process suite composition with `conductor run`, with
  verdict/state/seed equal.
- `cargo nextest` (in `agent-run.sh run`), `cargo test --workspace`, `cargo llvm-cov … 60`, `cargo audit` and
  `cargo deny` are green.

## Touchpoints
- **`conductor-cli`:** `cli.rs`, `main.rs`, `commands/{mod,suite,report}.rs`, new `commands/catalog.rs`, `pause.rs`,
  `render.rs`, `paths.rs`, `Cargo.toml`, `tests/{cli_smoke,cross_surface_parity}.rs`.
- **`conductor-run`:** `src/{drive,envelope,lib}.rs`, `tests/{composition_root,journal_conformance}.rs`; deleted:
  `tests/{envelope_fixture,lamps_fixture}.rs`, `tests/fixtures/{over-envelope.toml,lamps-journal.jsonl}`.
- **`conductor-core`:** `src/{scenario_catalog,lib,scenario,obs,pause}.rs`.
- `Cargo.lock`, only if Cargo records the feature.

## Test commands
24 `[[gate]]` entries:
- **Builds:** workspace `--locked`; standalone sweep `--lib` ×7 / `--bins`.
- **Lock census:** no-add against `0619048`, count 268.
- **Census guards:**
  - symbols and fixtures: exit 1, no output;
  - the sink outside the harvest: exit 1, no output;
  - window prose in the edited files: exit 1, no output.
- **Declarations:** `Scenarios` declared once; tokio `signal` declared.
- **Masters:** the no-interface line in 3 masters.
- **Suite:** `cargo fmt --check`, `bash scripts/agent-run.sh run`, `cargo test --workspace --lib --bins --tests`,
  `cargo run -- scenarios` (exit 0, `error-baseline-spike`, `P-009`, `scenarios`) and `cargo run -- --help`
  (`scenarios`).
- **Coverage:** llvm-cov collect + `--fail-under-lines 60`.
- **Supply chain:** advisory-db porcelain, then `cargo audit`, porcelain again, then `cargo deny`.
- **Operator legs:** `gate.py hygiene`, the clean-tree push, `ci.py conclusion --sha HEAD --wait 1200`.

No live leg and no run binding `:4317` / `:4318`. No smoke entry: no boot-path change.

### Baselines (16 new entries, on the untouched tree)
- **Green:**
  - lock no-add 0 and lock count 268;
  - sweep and `cargo test` (64 binaries, 1212 passed);
  - porcelain ×2, audit (1296 advisories, 268 crates) and deny (all four ok);
  - masters 3.
- **Red, on this chunk's own subject:**
  - symbol census (4883 B, all in the write set);
  - sink census (`obs.rs`);
  - prose census (6217 B);
  - `Scenarios` 0 and `signal` 0;
  - `scenarios` (exit 2, unrecognized subcommand);
  - `--help` lacks `scenarios`.

Controls, fired as the entry over a scratch clone (run dir `controls.md`): the lock guards read red on a package added
(1) or removed (267). The five census and declaration guards read green once their subject is built, and the masters
guard reads 2 when one line is reworded.

Two entries were re-spelt during validation:
- The sink census first read green VACUOUSLY: the harvest file sorted last. It now excludes the harvest and must be
  empty.
- The symbol census matched the prose word "over-envelope" in `render.rs:45`, `load_envelope.rs:7` and
  `run_contract.rs:286`, an existing property. It now matches the fixture names with their extensions.

## planlint check 4 (directive §8)
- **(5):** 0 `leg = 'live'` entries; the harness slot is filled. 2 leg entries carry a content atom (the operator
  hygiene and CI reads), and 0 lack an `atom from`.
- **(13):** 1 operator CI-outcome read, 0 not through `ci.py conclusion`.
- **(14):** 0 base-less git change reads. **(15):** 0 grep→`wc -l` zero-counts. **(16):** 0 exclusion guards.
- **(17):** 1 of 21 asserting entries reads a path the wrap writes: the masters entry (design-system,
  layout-templates, a11y-plan, written at the wrap's P2 before the light gate). Read: none of this chunk's expected
  amendments removes the opening `**Conductor has no interface.**` line, so it stays as written.
- Check 9: 1 CI read, operator-exempt. Check 2 ledger notes 0; check 5 placeholders 0. **Hits: 0.**

## Concretization preview (directive §8)

### v4-01 — CLAIMED by this chunk
**Current acceptance:** "The whole program builds and passes its full gate with the window toolkit absent from its
dependency graph, and no check anywhere starts a window or audits one. What must still hold without the window:
every function Conductor had — describing, starting, following, reading and cleaning up a run, and the capability
coverage view — is exercised from the command line in that same gate, and the masters say in plain words that
Conductor has no interface."

**Proposed concrete form:**

*The program builds and passes its full gate.*
- `cargo build --workspace --locked` and the standalone sweep are green.
- `bash scripts/agent-run.sh run` is green: nextest, doctest, clippy `-D warnings` and the two feature-gated lines.
- CI reads `verdict: green` on this chunk's push.

*The window toolkit is absent from its dependency graph.*
- The lock holds 268 packages, none added against `0619048`.

*No check starts a window or audits one.*
- `ci.yml` is untouched by this chunk: one job, `rust`.
- No harness arm drives a window (measured at the 2026-10-11 wrap).

*No type, limit, fixture or sink exists for a panel.*
- The census over `crates scripts .github` for `RunEvent|RunStage|validate_selection|SUITE_SELECTION|
  CONDUCTOR_E2E_SEED_DIR|envelope_fixture|lamps_fixture|lamps-journal.jsonl|over-envelope.toml` is empty.
- `conductor-tauri.jsonl` is named in `crates/` only by `operator_pause_harvest.rs`, which belongs to `Window grading
  retired`.
- The checklist bound's reason names no dialog.

*Every function is exercised from the command line in the same gate.*
- **describing:** `conductor scenarios` (binary entry + `scenarios_lists_the_catalogue`);
- **starting:** `conductor run` / `conductor suite` (`run_by_name_blocks_without_pulse_and_exits_zero`,
  `suite_runs_the_catalog_and_exits_zero`);
- **following:** the suite's progress count, its Ctrl-C stop that keeps what finished and exits 130, and the hold's
  checklist lines (`drive_run`'s three unit tests, `suite_exit`'s test, the unix SIGINT test, the checklist-lines
  test);
- **reading:** `conductor report` with the stored caption (`report_renders_the_latest_run`,
  `report_prints_the_stored_envelope_caption_for_a_suspect_run`);
- **cleaning up:** `conductor cleanup` (`cleanup_removes_the_run_rows_and_reports_the_count`);
- **coverage view:** `conductor coverage` (`coverage_lists_every_classified_pid`).

*The masters say in plain words that Conductor has no interface.*
- `**Conductor has no interface.**` opens design-system, layout-templates and a11y-plan (entry reads 3).

**Never-weaken read:**
- The outcome is unchanged; each clause names its witness.
- One clause is witnessed BELOW the binary. "Following" is exercised in `conductor-cli`'s inline tests and
  `conductor-run`'s unit tests, not through the spawned binary, because:
  - the spinner draws only on a TTY;
  - a hold needs a live Pulse;
  - a Blocked-spine suite ends before a Ctrl-C could land.
- The second Ctrl-C's exit-130 arm has no automated witness; it is the pre-chunk behaviour carried by one call.
- The unix SIGINT test runs on the Linux host only until `Linux-only base CI` (CI is `windows-latest` today).
- Judge whether that suffices for "exercised from the command line".

**Method:** stays `unit`.

### v4-09 — NOT claimed (stays pooled)
**Current acceptance:** "A run is started, followed while it runs and read after it ends from the command line alone,
and every status it shows is carried by a text label."

**Why not this chunk:** `working-route.md` lists v4-09 on `:47` (`Run read from the command line`), `:49` (`Harness
verbs on the one form`), `:107` (`Own log over a run of hours`) and `:113` (`Run written as it goes`, "progress
followed from the command line"). The requirement "a run is started, followed and read from the command line" has
legs this chunk does not build: the live progress following of `:113`, and the closed text-label set of `:47`.

**Ledger write after your word:** one dated note on v4-09 naming what this chunk adds toward it — the stop, the past
run's standing, the hold's checklist lines and the catalogue listing — and that the claim stays with those entries.

## Acts that may meet the auto-mode classifier (directive §5)
- **Deletion of 4 files (no directory):** `crates/conductor-run/tests/envelope_fixture.rs`, `lamps_fixture.rs`,
  `tests/fixtures/over-envelope.toml`, `tests/fixtures/lamps-journal.jsonl`. The emptied `tests/fixtures/` directory
  needs no `rm -r`; git drops it.
- **CI job or gate edits:** none (`ci.yml` untouched; `ci.yml:219` still runs `--test journal_conformance` whole).
- **Push:** the operator pass's own (`git push origin build/conductor-0.4.0`, clean-tree form).

## Size (directive §6)
26 modified files plus 1 new, mostly docs and small edits. It fits one chunk, so no cut is proposed.

## Expected amendments at the wrap
- **architecture §Occupied Resources:** the Tauri sink row and the `CONDUCTOR_E2E_SEED_DIR` row; `drive_run` as the
  suite's driver.
- **architecture config conventions:** the checklist bound's reason.
- **security-plan:** the seed-dir row and its 4 restatements; the checklist reason; the CLI-arguments row gains
  `scenarios` and the Ctrl-C stop.
- **test-plan:** §1 Vector 1; §3 log-format key; §5 the parity arm; §7 the fixture set at both sites.
- **obs-plan:** §3 / §6 the Tauri sink; §4 the parity arm.
- **layout-templates §Primary screens:** `scenarios`, the report caption, the stop line and exit 130, the checklist
  lines.
- **design-system §cli pattern 3:** the catalogue table.

## Beyond the plan
- **Deferral to an existing entry:** the unix SIGINT test is witnessed on the Linux dev host only until `Linux-only
  base CI — one Linux job keeping test, coverage-floor, zero-retry, supply-chain, secret-scan, static, own-log
  conformance and unlogged-panic gates; Windows runners, PowerShell harness twin leave (v4-07)` makes CI Linux. No
  edit to that entry is proposed: its Linux job runs the workspace tests, which include this test.
- **Accepted without a witness:** the second Ctrl-C's immediate exit 130 (`std::process::exit`), not assertable
  in-process. Recorded in the plan's notes as owed. No owner is proposed; your word accepts it as is or names one.
