# Report — 2026-10-12-panel-shaped-types-retired

**Chunk:** Panel-shaped types retired — the panel types, limit, sink and fixtures leave; four functions get a command-line entry
**Date:** 2026-10-12
**Commits:** `acad00d` chore(2026-10-12-panel-shaped-types-retired): operator pre-CI commit (since `last_wrap` 2026-10-11T09:14:30Z; `0619048` is the prior wrap's own commit). Basis for every "changed" claim below: `06190485` (the parent of the oldest pre-CI commit) → the work tree.

## Changes (structured — detectors read this)
- **Files:** 26 source paths (`gate.py scope`: changed 26 · listed 26 · recorded 0; `git diff --name-only 0619048` lists 87 with the run dirs, chunk dir and bookkeeping):
  - new: `crates/conductor-cli/src/commands/catalog.rs`
  - edited: `crates/conductor-cli/{Cargo.toml, src/cli.rs, src/main.rs, src/paths.rs, src/pause.rs, src/render.rs, src/commands/mod.rs, src/commands/report.rs, src/commands/suite.rs, tests/cli_smoke.rs, tests/cross_surface_parity.rs}` · `crates/conductor-run/{src/drive.rs, src/envelope.rs, src/lib.rs, tests/composition_root.rs, tests/journal_conformance.rs}` · `crates/conductor-core/src/{lib.rs, obs.rs, pause.rs, scenario.rs, scenario_catalog.rs}`
  - deleted: `crates/conductor-run/tests/envelope_fixture.rs` (4 tests) · `crates/conductor-run/tests/lamps_fixture.rs` (2 tests) · `crates/conductor-run/tests/fixtures/over-envelope.toml` · `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` — the directory `crates/conductor-run/tests/fixtures/` no longer exists (basis: `git show 0619048:… | grep -c '#\[test\]\|#\[tokio::test'` → 4 and 2; `ls` after the removal)
  - `Cargo.lock`: unchanged (byte-identical to `0619048`; 268 packages, 0 added or moved — plan gate entries `comm -13 …` read 0 and `awk '/^name = /' Cargo.lock | wc -l` read 268)
- **Symbols / APIs:**
  - REMOVED `conductor_run::RunEvent` · `conductor_run::RunStage` (the Tauri `Channel` payload; no consumer left — the graph's refs were `drive.rs`, `lib.rs`, `tests/composition_root.rs` only, research.md §Graph impact)
  - REMOVED `conductor_core::SUITE_SELECTION` · `conductor_core::validate_selection` and their two tests (no caller outside their module; they guarded the Tauri `start_run` alone)
  - CHANGED `conductor_run::drive_run<R, P, A>(pf, scenarios, run_id, runs_dir, envelope, resolver, progress: P, should_abort: A) -> anyhow::Result<DriveOutcome>`, `P: FnMut(usize)` (scenarios completed so far, called after each), `A: Fn() -> bool` (polled before each scenario and once after the last; unchanged). Persists ONCE (unchanged); a stop keeps every finished record. Callers now: `conductor suite` (production — its FIRST production caller) and the tests in `drive.rs`, `envelope.rs`, `crates/conductor-cli/tests/cross_surface_parity.rs`
  - NEW `conductor_run::DriveOutcome { records: Vec<RunRecord>, stopped: bool }` (`crates/conductor-run/src/drive.rs` added 14-21), re-exported at `lib.rs` added 33
  - `conductor_run::read_envelope` — signature, `info!` boundary line and behaviour unchanged; its doc now names `conductor report` as its reader; FIRST production caller: `crates/conductor-cli/src/commands/report.rs` (added 21-24)
  - `conductor_core::ScenarioSummary` · `list_scenarios` — unchanged; docs re-framed for `conductor scenarios`; FIRST production caller: `Paths::list_scenarios` (`crates/conductor-cli/src/paths.rs` 46-54 @48). `drift.rs`'s test call and its `"ScenarioSummary"` leak token stay
  - `conductor_core::MAX_CHECKLIST_TEXT` — value `200` unchanged, both `length(min = 1, max = MAX_CHECKLIST_TEXT)` attributes and the `dive` unchanged; docs restate the reason as a bound on untrusted scenario text the CLI hold prints as one terminal line each (`scenario.rs` added 80-81 · 92-94)
  - CLI verb `conductor scenarios` NEW (`Commands::Scenarios`, `cli.rs` added 22-23; dispatched `main.rs` added 81; handler `commands/catalog.rs` 8-17): prints `render::scenarios_table` (comfy-table, `ContentArrangement::Dynamic`, columns Scenario · P-IDs · SLO, P-IDs ID-cyan behind `stdout_color()`, no verdict/state column) then the plain roll-up `render::scenarios_rollup` (`render.rs` 152-159 @153 — `1 scenario` for one, `{N} scenarios` otherwise; the operator pass's fix); an empty catalogue prints `No scenarios found.`; exit 0; loads through `list_scenarios` + the capability manifest, so an absent manifest is a harness fault (`error:` + `hint:`, exit 1)
  - `conductor suite` — now drives through `drive_run`; installs a Ctrl-C handler (`install_stop_handler`, `commands/suite.rs` 79-99 @84) on the current-thread runtime AFTER preflight, BEFORE driving: the first Ctrl-C sets the stop flag and prints `render::STOP_REQUESTED` on stderr (`render.rs` 113-115 @114); the second calls `std::process::exit(130)`. On a stop: the spinner `abandon()`s in place, the envelope caption and results table print over the kept records, `render::suite_stopped_line` (`render.rs` 117-120 @118, `suite stopped: {kept} of {total} scenarios kept`) prints on stderr, exit `ExitCode::from(130)` via `suite_exit` (`commands/suite.rs` 70-77 @71). A completed suite still exits on `exit_code(&records)` (hard-Fail rule)
  - `conductor report` — after the `Run report <id>` line and a blank line, prints `render::envelope_caption` of `read_envelope`'s stored standing once when suspect; the six-column table unchanged; no flag
  - CLI hold — `PromptResolver::resolve` prints `render::hold_checklist_lines` (`render.rs` 98-111 @101: `  induced: {induced}` / `  observe: {observation}` per item, plain, no bracket label) on stderr after `hold_line`, before `confirm`, inside the spinner-suspended closure (`pause.rs` added 38-40). `CliResolver::resolve` and the headless path unchanged
  - `main.rs` hint for `no scenario matches` now reads "list them with `conductor scenarios`, or pass a P-ID like P-009" (added 66)
  - Env vars: `CONDUCTOR_E2E_SEED_DIR` lost its ONLY reader (the deleted `envelope_fixture.rs`); no code reads it now (plan gate `git grep … CONDUCTOR_E2E_SEED_DIR … -- crates scripts .github` exit 1, no output). No env var added
  - Ports / sockets / listeners: none added. The Ctrl-C handler is an OS signal registration in-process (`tokio::signal::ctrl_c`), no bind, no file
  - The window log sink `logs/conductor-tauri.jsonl`: named in `crates/` only by `crates/conductor-run/tests/operator_pause_harvest.rs` (plan gate `git grep -l 'conductor-tauri\.jsonl' -- crates ':!…operator_pause_harvest.rs'` exit 1); `ObsSink`'s doc names only the cli's `--agent-mode` file (`obs.rs` added 59-60); `ObsSink` and its variants unchanged
- **Crates / modules:** module `conductor-cli::commands::catalog` added; no crate added or removed; `conductor-run::drive` re-shaped (module doc: the suite driver)
- **Dependencies:** none added or bumped. `conductor-cli`'s `tokio` gains the `signal` FEATURE (`crates/conductor-cli/Cargo.toml` added 18) over packages already locked (`signal-hook-registry 1.4.8` was already in the graph — research.md §Mechanisms re-derived); lock unchanged (above)
- **Schema / config:** none — no scenario key, no manifest key, no `runs.db` table, no journal shape changed. The `[[checklist]]` bound is unchanged in value and rule; it gained its first tests (`scenario.rs` 971-1026: `a_checklist_half_at_the_bound_loads`, `an_out_of_bound_checklist_half_is_a_validation_fault` ×4 rstest cases (201 chars / empty, induced / observation → `CoreError::Validation`), `a_checklist_beside_expected_checks_is_a_config_fault` → `CoreError::Config`)
- **Spec-master edits:** none (no master or registry file was edited before this wrap)
- **Counts / qualifiers moved:**
  - the CLI verb set: 7 → 8 (`scenarios` added; `conductor --help` lists it — plan gate `cargo run … -- --help` `contains scenarios` green). No master states a verb COUNT (sweep `scenarios-verb` control unfired over the masters and registries — `conductor scenarios` is named nowhere in them); layout-templates §Primary screens (commands) lists the verbs one bullet each (`layout-templates.md:32-38`, read by hand) and holds no `scenarios` bullet
  - test totals: 1218 tests pass under nextest (`bash scripts/agent-run.sh run`, Summary line) and under `cargo test --workspace --lib --bins --tests` (sum of `test result` lines), from the plan's baseline 1212: −9 retired (2 picker-guard tests, 4 + 2 webview-fixture tests, 1 committed-fixture conformance test) + 15 new; test binaries 64 → 62 (the two deleted fixture targets). No master states a test total (not swept — a figure this report states, not a master qualifier)
  - `crates/conductor-run/tests/fixtures/` left the committed-fixture family: the family is now `crates/conductor-core/tests/fixtures/` + `scripts/fixtures/mutation-gate/` (sweep `run-fixture-tree`: standing 1 — `test-plan.md:316`)
- **Dev-tool versions:** none — no host tool installed or changed (cargo-audit / cargo-deny / cargo-llvm-cov / cargo-nextest re-read as before; the advisory-db local copy clean, HEAD `7eebec69` = the remote's `main`, read live with `git ls-remote`)
- **Harness / gate surface:** no CI step, harness verb or status shape changed (`.github/workflows/ci.yml` byte-unchanged — `git diff --quiet -- .github` exit 0; jobs `['rust']`). Test targets removed: `conductor-run --test envelope_fixture`, `--test lamps_fixture`; `--test journal_conformance` keeps its production-writer subject (CI's `--test journal_conformance` step keeps a subject). New CLI exit 130 for a stopped `conductor suite` (a suite outcome, not a harness verb); the harness `run` verb's scenario leg drives `conductor run`, not `suite`, so no harness exit read changes
- **Cross-project / external claims:**
  - CI: `CI#38217858451`, push, on `acad00d067e4` (the pre-CI commit — this wrap's commit adds to that tree), completed/success, 1/1 check `Rust gate (build · test · lint · supply-chain · coverage)`, wall 382 s (`ci.py conclusion --sha HEAD`, recorded in `chunks/2026-10-12-panel-shaped-types-retired/evidence/operator-pass.md`)
  - RustSec advisory database: `cargo audit` loaded 1296 advisories, 268 crate dependencies scanned, exit 0; `cargo deny check advisories bans licenses sources` all ok, exit 0, no `advisory-not-detected` / `license-not-encountered`
  - inputs (`inputs.py verify`, re-fired after this report): `I1 · message: the operator (pc overseer), /andromeda-phase invocation, 2026-10-12 · copy message · n/a` · `I2 · ../additional/pc-overseer/relays/conductor-phase-panel-shaped-types-retired-2026-10-12.md · copy no-repo · unchanged` · `I3 · message: the operator, /andromeda-phase P4 fork round, 2026-10-12 · copy message · n/a` (the verb word `scenarios` (B5) and exit 130) · `I4 · message: the operator, P5 review of p5-card-1.md · copy message · n/a` (superseded by I5's card for the card's rows; its own words stand — the four plan edits and the three acceptances) · `inputs#I5` · message: the operator, P5 review of `.andromeda/runs/2026-10-12T06-44-11-phase/p5-card-2.md` · copy message · n/a — the governing card; its `## Beyond the plan` rows, verbatim, by the operator's own word (`inputs#I5`, approving `inputs#I4` (3)):
    - **Deferral to an existing entry:** until `Linux-only base CI — one Linux job keeping test, coverage-floor, zero-retry, supply-chain, secret-scan, static, own-log conformance and unlogged-panic gates; Windows runners, PowerShell harness twin leave (v4-07)` makes CI Linux, the unix SIGINT test is witnessed only on the Linux dev host. Accepted by your word (`inputs#I4` (3)); no edit to that entry.
    - **Accepted without a witness:** the second Ctrl-C's immediate exit 130. It stays unwitnessed by your word (`inputs#I4` (3)), with no owner.
  - `inputs#I6` · `../additional/pc-overseer/relays/conductor-wrap-panel-shaped-types-retired-2026-10-12.md` · copy no-repo · unchanged — this wrap's directive (snapped at P1): a split wrap, P1 only, `cascade.py leaves --all` in the second window (founder ruling F252 D2); v4-01 as claimed with its P5 note; v4-09 keeps its dated note and stays pooled; the masters' Tauri log-sink rows retire in this wrap; the two P4 answers are the operator's boundary words alone and halt in the ordinary form if the playbook reads a widening; the implement deviations as listed below; the prior handoff's owed items; the stops
- **Reverted / negative API facts:** the roll-up line was first written as the plan's literal `{N} scenarios` (reading "1 scenarios" for one) and replaced in the operator pass by `render::scenarios_rollup` on the operator's word — the literal form never shipped past the pre-CI commit
- **Insufficient fixes (written, kept, not the remedy):** none
- **Spec claims disproved by measurement:**
  - intent R1 / `verification-matrix.json#v4-01` `observed_gap` — "two webview fixtures with three tests": measured 4 + 2 tests in the two files at `0619048` (count above) and a third consumer of their fixture files, `journal_conformance.rs`'s `the_committed_fixture_journal_conforms` (1 test, deleted). Recorded at phase as a `premise-corrected` mark in `scope.md` item 6 only; the ledger carries no PREMISE-CORRECTION note. Owner proposed: a dated `ledger-note` on v4-01 at P7.3 (intent.md is immutable)
- **Expected amendments (from plan):** sites by the P1 site sweep (`cascade.py sweep --pre-pass`, patterns `{run_dir}/report-patterns.toml`, baseline HEAD `acad00d0`; `standing` = master + registry rows); three patterns' controls never fired over the masters and registries — `parity-arm` (`Tauri arm|start_run`), `scenarios-verb` (`conductor scenarios`), `ctrl-c-stop` (`Ctrl-C|Ctrl\+C|SIGINT`, case-insensitive) — the refusal line is their zero basis:
  - architecture §Occupied Resources — `logs/conductor-tauri.jsonl` row: carried (Symbols / APIs, the sink bullet) · `tauri-sink` standing 9 — `architecture.md:164` ×2
  - architecture §Occupied Resources — `CONDUCTOR_E2E_SEED_DIR` row: carried (Symbols / APIs, Env vars) · `seed-dir` standing 7 — `architecture.md:184`
  - architecture — the `conductor-run` crate row restating `drive_run` as the suite's driver with a stop flag: carried (Symbols / APIs, `drive_run`) · `live-counter` (`live[- ]counter|abort flag|stop button`, case-insensitive) standing 6 — the crate row is `architecture.md:155` ("the live-counter run driver") and its key file `registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:23` ("live-counter drive_run"), not §Occupied Resources; the other four rows (`design-system.md:123`, `layout-templates.md:19`, `:59`, `:108`) name the spinner's live counter, a true claim sharing the token — no change
  - architecture §Conventions (config) — the `MAX_CHECKLIST_TEXT` reason: carried (Symbols / APIs, `MAX_CHECKLIST_TEXT`) · `checklist-bound` standing 2 — `architecture.md:81` ("a dialog row's display bound")
  - security-plan §Input Validation — the `CONDUCTOR_E2E_SEED_DIR` row and its mentions: carried · `seed-dir` rows in security-plan: `:51` (Threat Model entry point), `:112` (the row), `:209` ×3 (Bootstrap scope), `:242` (Secret Management — Development), `:314` ×3 (Anti-Patterns → Input); `webview-fixtures` rows `:112` ×2, `:314`
  - security-plan §Input Validation — the checklist row's reason: carried · `checklist-bound` — `security-plan.md:107` ×2 ("ceiling `200` — a dialog row's display bound")
  - security-plan §Input Validation — the CLI-arguments row gains `conductor scenarios` (no argument) and `suite`'s Ctrl-C stop: carried (Symbols / APIs) · the row is `security-plan.md:116` (read by hand); `scenarios-verb` and `ctrl-c-stop` 0 over every master (refusal)
  - test-plan §1 Coverage triggers Vector 1 — the seed-dir reader class retires: carried · `seed-dir` — `test-plan.md:83`; `webview-fixtures` — `test-plan.md:83`
  - test-plan §3 Log format key — the Tauri sink row retires (lockstep with obs-plan §3): carried · `tauri-sink` — `registries/contracts/test-plan/log-format.md:6`
  - test-plan §5 Cross-surface parity — the arm is the in-process suite composition: carried (Symbols / APIs, `drive_run`; the test renamed its arm, below) · `drive-run` rows `test-plan.md:77` ×2, `:89`, `:204`, `:288` — each states the in-process arm as `conductor_run::{preflight, drive_run}`, which still holds; the amendment is the naming (now the composition `conductor suite` drives); `parity-arm` 0 in the masters (no "Tauri arm" there)
  - test-plan §7 — the committed-fixture tree set and its round-trip enumeration lose the two webview fixtures (both sites): carried (Files, Counts) · `run-fixture-tree` — `test-plan.md:316` (the set names `crates/conductor-run/tests/fixtures/`); `webview-fixtures` — `test-plan.md:317` ×2 (the enumeration names `lamps_fixture.rs` and `envelope_fixture.rs`)
  - obs-plan §3 Logging stack / Log file location and §6 Sink configuration — the Tauri backend sink retires: carried · `tauri-sink` — `obs-plan.md:75` ×2, `:99` ×2, `:396` ×2; key files `registries/contracts/obs-plan/log-file-location.md:4` ×2, `registries/contracts/obs-plan/logging-stack.md:6` ×2, `registries/obs-plan-contracts.toml:19`, `:29` (the labels `"Sink (Tauri backend)"`, `"Tauri backend"`)
  - obs-plan §4 In-process ↔ headless parity — the arm's naming: carried · `drive-run` — `obs-plan.md:126`, `:270`
  - layout-templates §Primary screens (commands) — a `conductor scenarios` entry (third table shape: Scenario · P-IDs · SLO, roll-up line `1 scenario` / `N scenarios`, `No scenarios found.`); `report` prints the stored caption; `suite`'s stop line and exit 130; the hold's indented checklist lines: carried (Symbols / APIs) · site `layout-templates.md:30-39` (read by hand); `scenarios-verb` and `ctrl-c-stop` 0 (refusal)
  - design-system §Surface: cli → Component Patterns 3 — the catalogue listing as a static table beside `coverage`: carried (Symbols / APIs, `conductor scenarios`) · the coverage-table sentence is `design-system.md:158` (read by hand); `scenarios-verb` 0 (refusal)
  - beyond the list — the directive's item 4 and the prior handoff (at `0619048`, Notes, "Kept on purpose") name the same Tauri log-sink rows as owed to this wrap: the `tauri-sink` rows above, all nine
  - other standing rows, no change: `picker-guard` standing 1 — `architecture.md:50` (`list_scenarios(dir, &manifest)` in the loader set — still true); `read-envelope` standing 1 — `obs-plan.md:406` (the DB READ boundary — unchanged); `drive-run` `architecture.md:110` ("`preflight()` … remains what `conductor suite` and `drive_run` call" — `suite` still calls `preflight` then `drive_run`)
- **Coverage of new surfaces:**
  - `conductor scenarios` → validation {manifest-checked load (`list_scenarios` + `CapabilityManifest`) ✓; no argument} · instrumentation {no new span (obs-plan §11 bans a listing span) — n/a} · PII {n/a} · tests {e2e `scenarios_lists_the_catalogue`, `scenarios_with_an_absent_capability_manifest_is_a_harness_fault`, the plan's `cargo run … -- scenarios` entry; unit `scenarios_table_plain_lists_identity_without_escapes`, `scenarios_rollup_is_singular_for_one_and_plural_otherwise`} · a11y {status never color-alone — no status column; piped output 0 ANSI bytes asserted ✓} · tokens {ID-cyan 117 reused, no new code ✓}
  - `conductor suite` Ctrl-C stop → validation {n/a — no input} · instrumentation {`drive_run`'s existing `info!` "run aborted by the operator" line with `run_id` + `count` ✓} · PII {n/a} · tests {unit `drive_run_*` ×3, `a_stopped_suite_exits_130_whatever_its_records_say`, `a_completed_suite_exits_on_the_hard_fail_rule_alone`, `#[cfg(unix)] the_first_interrupt_sets_the_stop_flag_and_the_process_lives` — Linux dev host only; the second Ctrl-C unwitnessed by the operator's word} · a11y {stderr lines plain text ✓} · tokens {none added ✓}
  - `conductor report` stored caption → validation {n/a — reads `runs.db` through bound params} · instrumentation {`read_envelope`'s existing boundary `info!` line; no `db.*` span ✓} · PII {n/a} · tests {e2e `report_prints_the_stored_envelope_caption_for_a_suspect_run`, `report_renders_the_latest_run` (asserts no caption in-envelope)} · a11y {`[ENVIRONMENT-SUSPECT]` bracket label always present ✓} · tokens {existing 246 mute ✓}
  - CLI hold checklist lines → validation {the `[[checklist]]` bound at load ✓} · instrumentation {n/a} · PII {n/a — scenario-authored text} · tests {unit `hold_checklist_lines_render_two_indented_plain_lines_per_item_in_order`; the interactive print itself needs a TTY and a live Pulse — `unrunnable-here`} · a11y {plain lines, no color ✓} · tokens {none ✓}

## Deviations from intent
- **Checklist lines on the interactive hold only** — plan step 9 and `inputs#I4` (4) keep the headless path unchanged; research.md §Conventions to follow says "checklist lines print above the confirm and in the headless path alike". The plan was followed (it binds; research is context). Reason: the plan and the operator's P5 word. Owner of the research line's stale wording: none needed — research.md is immutable; recorded here.
- **The listing table is sized to the terminal** (`ContentArrangement::Dynamic`, per plan step 7's "sized from the terminal width") while the sibling results and coverage tables use `ContentArrangement::Disabled` (`render.rs:276`, `:335`). Reason: the plan's words. See *Found and not owned* below for the spec wording this touches.
- **The stop handler is installed after preflight**, right before the drive (plan step 6: "before driving"). A Ctrl-C during preflight still ends the process at once — the pre-chunk behaviour. Reason: the plan's placement; preflight writes nothing a stop could keep.
- **`conductor-core`'s crate doc** (`crates/conductor-core/src/lib.rs:12`, "so the CLI and Tauri shells initialize logging identically") is left: outside the plan's prose census and plan step 1 edits only the re-export. Owner: `No-survivor sweep` (`working-route.md:136`).
- **The smoke fired** although the plan stated no boot-path change and listed no smoke entry: `main.rs` (dispatch) and `cli.rs` (the verb table) changed, which is the skill's boot-path condition. Driven under the `pulse;injection` lever: `SCENARIO=error-baseline-spike bash scripts/agent-run.sh run` minted `2026-10-12T07-23-34-092` (after the 07:23:20Z start), `status` read it back (`Blocked`, seed 424242), `conductor report` rendered it with no caption, `cleanup` removed 2 rows and the run files. Nothing reached Pulse (the read-back connect is refused before any canary).
- **62 test binaries against the plan baseline's 64** — the two deleted fixture targets; the passing total is 1218 (above).
- **The roll-up line** — implement wrote the plan's literal `{N} scenarios`; the operator pass replaced it with `1 scenario` / `N scenarios` on the operator's word (Reverted bullet); the plan text stays as written.
- **v4-01 set `implemented` with its CI clause resting on the operator pass's `ci.py conclusion` entry**, named in the ref; that entry read `verdict: green` on `acad00d` (Cross-project bullet).
- scope record: none — `gate.py scope` clean (changed 26 · listed 26 · recorded 0), at implement P4 and again at this wrap's P1.

## Decisions & corrections
- **Operator correction (product):** the catalogue roll-up reads `1 scenario` for one and `N scenarios` otherwise, pinned by a unit test for 1 and 2 — a plan's literal output form is not a UX ruling; the operator corrected it before the pre-CI commit.
- **Operator decisions carried:** the verb word `scenarios` (B5) and a stopped suite exiting 130 (`inputs#I3`); "following" witnessed below the binary, the SIGINT test on the Linux host until `Linux-only base CI`, the second Ctrl-C unwitnessed (`inputs#I4` (3)). The directive (`inputs#I6`) holds the two P4 answers as the operator's boundary words alone, PROVISIONAL at any widening halt.
- **The operator pass** was performed by the agent on the operator's explicit word: the pre-CI commit staged by explicit paths (all 87 `git status` paths — the push guard `git diff --quiet && git diff --cached --quiet` needs every tracked change committed, and the new `catalog.rs` had to ride for CI to build), the guarded push run by hand as the plan writes it (never through `gate.py --operator`, which the skill forbids for a history move), no merge-base probe (Conductor has no `main`, the operator's word).
- **Sweep hazards found this chunk:**
  - `cascade.py sweep --pattern` takes its ids space-separated after ONE flag (`--pattern a b c`); repeating the flag (`--pattern a --pattern b`) keeps only the LAST id — measured: seven repeated flags ran 1 pattern and printed `total (1 pattern)`.
  - a `cascade.py` pattern id is 1–16 chars; a 17-char id exits 2 naming the pattern.
  - a `cd DIR 2>/dev/null || true; …` compound moved the session cwd (the guard refuses a bare `cd` but this form ran) — measured once at this wrap's P1; restored with `cd` to the project root.
- **Testing mechanism (in-process SIGINT on a current-thread runtime):** `tokio::signal::ctrl_c()` registers its OS handler on its first POLL, not on the call — so the handler install spawns the listener task and then `yield_now().await`s once, letting the task take that poll before the caller proceeds; without it a SIGINT sent right after the install would hit the default disposition and kill the test process. The test sends the signal through a fixed-argv `kill -INT <own pid>` spawn (no `libc`, no `unsafe`) and awaits the flag under a bounded `tokio::time::timeout`.

## Outcome
- **v4-01** — re-asserted against the diff:
  - channel payload, picker guard, webview fixtures and seed-dir reader gone: `git grep -n -E 'RunEvent|RunStage|validate_selection|SUITE_SELECTION|CONDUCTOR_E2E_SEED_DIR|envelope_fixture|lamps_fixture|lamps-journal\.jsonl|over-envelope\.toml' -- crates scripts .github` → exit 1, no output — MET
  - `conductor-tauri.jsonl` named in `crates/` only by `operator_pause_harvest.rs` → MET; the checklist bound's reason names no dialog (prose census over the ten edited files → exit 1) → MET
  - every function exercised from the command line in the same gate: describing (`scenarios_lists_the_catalogue` + the `cargo run … -- scenarios` entry, 37 scenarios listed), starting (`run_by_name_blocks_without_pulse_and_exits_zero`, `suite_runs_the_catalog_and_exits_zero`), following (`drive_run`'s three unit tests, `suite_exit`'s two tests, the unix SIGINT test, the checklist-lines test), reading (`report_renders_the_latest_run`, `report_prints_the_stored_envelope_caption_for_a_suspect_run`), cleaning up (`cleanup_removes_the_run_rows_and_reports_the_count`), coverage (`coverage_lists_every_classified_pid`) — each confirmed by name in both runners' logs — MET
  - the window toolkit absent from the graph: 268 packages, 0 added against `06190485ec74ac07b1c797fbde6dc9441e60db2f`; `ci.yml` keeps its one `rust` job — MET
  - the masters say Conductor has no interface: `grep -l -F '**Conductor has no interface.**'` over design-system, layout-templates, a11y-plan → 3 — MET
  - CI `verdict: green` on the chunk's push: `CI#38217858451` on `acad00d` — MET
  - the three operator-accepted limits (`inputs#I4` (3)) stand as v4-01's P5 note states them
  - v4-09 — not claimed, its 2026-10-12 note stands (the directive's item 3)
- **Other criteria:** per-seam sweep exit 0 — MET · `conductor scenarios` absent manifest → exit 1, `error:` + `hint:`, no absolute path — MET · checklist bound tests (201 / empty → `CoreError::Validation`, 200 loads, beside `expected` → `CoreError::Config`) — MET · `cargo audit` / `cargo deny` exit 0 with the advisory-db porcelain empty before and after — MET · piped listing 0 ANSI bytes, the stored caption once with no new lamp, label or column — MET · a stopped suite's spinner `abandon()`s, exit 130, a completed suite on the hard-Fail rule (`suite_exit`'s tests) — MET · hold checklist lines plain and indented, headless path unchanged — MET · nextest and `cargo test` both exit 0, parity still asserts equal verdict / state / seed between the in-process arm and `conductor run` — MET · `cargo llvm-cov report --fail-under-lines 60` passes (95.81% lines) — MET · `read_envelope` unchanged, no `db.*` span, `agent_mode_routes_self_obs_to_the_log_file_not_stderr` green — MET
- **Gates** (the plan's `[[gate]]` entries by `run`, the final firing at implement's run dir after the roll-up fix — `entries 24 · green 21 · red 0 · recorded 0 · timeout 0 · not-run 3`):
  - `cargo build --workspace --locked` — green, exit 0
  - `comm -13 <(git show 06190485…:Cargo.lock | …) <(… Cargo.lock …) | wc -l` — green, `last line 0`
  - `awk '/^name = /' Cargo.lock | wc -l` — green, `last line 268`
  - the per-seam `--lib` ×7 + `conductor-cli --bins` sweep — green, exit 0
  - the `git grep … RunEvent|RunStage|…` census — green, `exit 1` + `no output`
  - the `git grep -l 'conductor-tauri\.jsonl' …` census — green, `exit 1` + `no output`
  - the edited-files prose census (`tauri|webview|picker|\bgui\b|dialog row|desktop`) — green, `exit 1` + `no output`
  - `grep -c … Scenarios … cli.rs` — green, `last line 1`
  - `grep -c '^tokio = .*"signal"' crates/conductor-cli/Cargo.toml` — green, `last line 1`
  - `grep -l -F '**Conductor has no interface.**' … | wc -l` — green, `last line 3`
  - `cargo fmt --all --check` — green, exit 0
  - `bash scripts/agent-run.sh run` — green, exit 0 (nextest 1218 passed, doctest, clippy `-D warnings` incl. the two feature-gated lines)
  - `cargo test --workspace --lib --bins --tests` — green, exit 0 (1218 passed, 0 failed, 0 ignored)
  - `cargo run -q -p conductor-cli --locked -- scenarios` — green, exit 0, `contains error-baseline-spike` · `contains P-009` · `contains scenarios`
  - `cargo run -q -p conductor-cli --locked -- --help` — green, `contains scenarios`
  - `cargo llvm-cov nextest --workspace --profile ci --no-report` — green, exit 0
  - `cargo llvm-cov report --fail-under-lines 60 --ignore-filename-regex …` — green, exit 0 (TOTAL lines 95.81%)
  - `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` (before) — green, `no output`
  - `cargo audit` — green, exit 0
  - the advisory-db porcelain (after) — green, `no output`
  - `cargo deny check advisories bans licenses sources` — green, `lacks advisory-not-detected` · `lacks license-not-encountered`
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired `--operator 22`: green, `contains hygiene: clean` (`evidence/operator-pass.md`)
  - `git diff --quiet && git diff --cached --quiet && git push origin build/conductor-0.4.0` — `leg = 'operator'`, run by hand on the operator's word: exit 0, `0619048..acad00d` fast-forward (`evidence/operator-pass.md`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1200` — `leg = 'operator'`, fired `--operator 24`: green, `contains verdict: green` — `CI#38217858451` on `acad00d`, completed/success (`evidence/operator-pass.md`)
  - smoke: fired (boot-path — Deviations), passed
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran (Setup 4's pre-CI commit `acad00d`; commit list `acad00d` only); the gate verdicts above are the firing after the operator's roll-up fix (implement's first firing read the same 21 green at 1217 tests); the final HEAD's CI run `CI#38217858451` is recorded in `evidence/operator-pass.md`; implement's P4 report (this conversation) for the deviations and the census.
- **Process hygiene:** implement's census — gate entries (cargo, nextest, llvm-cov, audit, deny) and the smoke's harness run, `report` and `cleanup`: started by this run, `terminated`. Re-measured at this wrap's P1 (`ps -eo pid,etimes,comm`): no `conductor`, `cargo`, `rustc`, `andromeda-pulse-mcp`, `pulse-app` or `stub_pulse_mcp` process; `cargo-mutants` (pid 73541) with short-lived `cargo-nextest` children — another project's session (`~/dev/projects/viola`), not this run's: `left running — its own session stops it`.

## Found and not owned (for the card, with a lean)
- layout-templates §Component — Primary content block 1 (`layout-templates.md:116`) says the 6-column results table's width is "detected dynamically from the terminal (never hardcoded, never wrapped at arbitrary points)", while `results_table_styled` sets `ContentArrangement::Disabled` (`render.rs:276`; at `0619048` too, `:238`) — comfy-table's `Disabled` sizes columns to content and never wraps; it reads no terminal width. The coverage table likewise (`render.rs:335`); `layout-templates.md:80` uses the same phrase for the run-header mockup line, a different subject. Predates this chunk. Lean: a wording amendment at this wrap ("sized to its content, never wrapped") over a code change — the never-wrapped half is what the table honours, and no criterion asks for terminal-width detection; else a pin to `No-survivor sweep` is wrong (not window text) and an owner entry would be needed.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 06190485 (the parent of the oldest pre-CI commit acad00d0) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/conductor-cli/Cargo.toml — added 1 line(s) in 1 range(s)
added: 18
### crates/conductor-cli/src/cli.rs — added 4 line(s) in 2 range(s)
added: 1-2 · 22-23
### crates/conductor-cli/src/commands/catalog.rs — new file · 17 line(s)
- 8-17 «pub fn scenarios(paths: &Paths) -> anyhow::Result<ExitCode> {»
  - 10-13 «if summaries.is_empty() {»
### crates/conductor-cli/src/commands/mod.rs — added 2 line(s) in 2 range(s)
added: 3 · 12
### crates/conductor-cli/src/commands/report.rs — added 6 line(s) in 2 range(s)
added: 1-2 · 21-24
  - 22-24 «if let Some(caption) = standing.as_ref().and_then(render::envelope_caption) {»
### crates/conductor-cli/src/commands/suite.rs — added 125 line(s) in 7 range(s)
added: 2-4 · 7-8 · 10 · 17-20 · 36 · 39-54 · 60-157
  - 39-48 «let outcome = pipeline::drive_run(»
  - 61-66 «if outcome.stopped {»
- 70-77 @71 «fn suite_exit(records: &[RunRecord], stopped: bool) -> ExitCode {»
  - 72-76 «if stopped {»
- 79-99 @84 «async fn install_stop_handler() -> Arc<AtomicBool> {»
  - 87-96 «tokio::spawn(async move {»
  - 106-120 «fn failed() -> RunRecord {»
  - 122-126 @123 «fn a_stopped_suite_exits_130_whatever_its_records_say() {»
  - 128-132 @129 «fn a_completed_suite_exits_on_the_hard_fail_rule_alone() {»
  - 134-157 @138 «async fn the_first_interrupt_sets_the_stop_flag_and_the_process_lives() {»
### crates/conductor-cli/src/main.rs — added 2 line(s) in 2 range(s)
added: 66 · 81
### crates/conductor-cli/src/paths.rs — added 13 line(s) in 2 range(s)
added: 10-12 · 46-55
- 10-12 «use conductor_core::{»
  - 46-54 @48 «pub fn list_scenarios(&self) -> anyhow::Result<Vec<ScenarioSummary>> {»
### crates/conductor-cli/src/pause.rs — added 5 line(s) in 2 range(s)
added: 22-23 · 38-40
### crates/conductor-cli/src/render.rs — added 132 line(s) in 6 range(s)
added: 19-20 · 93 · 98-121 · 146-160 · 305-330 · 729-792
- 98-111 @101 «pub fn hold_checklist_lines(hold: &HoldPoint) -> Vec<String> {»
  - 102-110 «hold.checklist»
- 113-115 @114 «pub const STOP_REQUESTED: &str =»
- 117-120 @118 «pub fn suite_stopped_line(kept: usize, total: usize) -> String {»
- 146-150 @148 «pub fn scenarios_table(summaries: &[ScenarioSummary]) -> String {»
- 152-159 @153 «pub fn scenarios_rollup(count: usize) -> String {»
  - 154-158 «if count == 1 {»
- 305-329 «fn scenarios_table_styled(summaries: &[ScenarioSummary], color: bool) -> String {»
  - 307-310 «table»
  - 311-314 @312 «if color {»
  - 315-327 «for s in summaries {»
  - 729-759 @730 «fn hold_checklist_lines_render_two_indented_plain_lines_per_item_in_order() {»
  - 761-765 @762 «fn scenarios_rollup_is_singular_for_one_and_plural_otherwise() {»
  - 767-791 @768 «fn scenarios_table_plain_lists_identity_without_escapes() {»
### crates/conductor-cli/tests/cli_smoke.rs — added 122 line(s) in 3 range(s)
added: 92-93 · 97-153 · 466-528
- 97-135 @100 «fn scenarios_lists_the_catalogue() {»
  - 109-116 «for token in [»
  - 117-121 «assert_eq!(»
  - 122-125 «assert!(»
  - 127-130 «assert!(»
  - 131-134 «assert!(»
- 137-152 @138 «fn scenarios_with_an_absent_capability_manifest_is_a_harness_fault() {»
  - 148-151 «assert!(»
  - 486-492 «std::fs::write(»
  - 494-498 «run.env("CONDUCTOR_RUNS_DIR", RUNS)»
  - 500-504 «let assert = conductor(&dir)»
  - 506-510 «assert_eq!(»
  - 511-523 «for header in [»
### crates/conductor-cli/tests/cross_surface_parity.rs — added 37 line(s) in 17 range(s)
added: 3-9 · 50-52 · 55 · 90-91 · 99 · 102 · 111-119 · 123 · 127-128 · 130-131 · 138 · 143 · 145-146 · 150 · 155 · 160
       181
  - 111-115 «assert_eq!(»
  - 116-119 «assert!(»
  - 127-128 «let inproc = read_run_journal(&runs_dir, "run-path7-inproc")»
### crates/conductor-core/src/lib.rs — added 1 line(s) in 1 range(s)
added: 74
### crates/conductor-core/src/obs.rs — added 2 line(s) in 1 range(s)
added: 59-60
### crates/conductor-core/src/pause.rs — added 5 line(s) in 4 range(s)
added: 10 · 12-13 · 99 · 243
### crates/conductor-core/src/scenario.rs — added 61 line(s) in 3 range(s)
added: 80-81 · 92-94 · 971-1026
  - 971-980 «fn checklist_toml(induced: &str, observation: &str, with_expected: bool) -> String {»
  - 982-993 @983 «fn a_checklist_half_at_the_bound_loads() {»
  - 995-1016 @1000 «fn an_out_of_bound_checklist_half_is_a_validation_fault(»
  - 1018-1025 @1019 «fn a_checklist_beside_expected_checks_is_a_config_fault() {»
### crates/conductor-core/src/scenario_catalog.rs — added 9 line(s) in 4 range(s)
added: 1-2 · 4-6 · 14-15 · 36-37
### crates/conductor-run/src/drive.rs — added 65 line(s) in 29 range(s)
added: 1 · 3-4 · 8 · 14-21 · 24-29 · 31-32 · 39 · 41 · 44 · 49 · 52 · 58 · 60-63 · 66 · 68-69 · 76 · 79 · 81-82 · 89
       95-100 · 110-111 · 118 · 123-131 · 137 · 140 · 147-148 · 158 · 162-164 · 169
### crates/conductor-run/src/envelope.rs — added 9 line(s) in 4 range(s)
added: 14-15 · 41-42 · 44-46 · 176-177
### crates/conductor-run/src/lib.rs — added 15 line(s) in 4 range(s)
added: 1 · 3-11 · 13-16 · 33
### crates/conductor-run/tests/composition_root.rs — added 3 line(s) in 3 range(s)
added: 4 · 43 · 63
### crates/conductor-run/tests/envelope_fixture.rs — deleted
### crates/conductor-run/tests/fixtures/lamps-journal.jsonl — deleted
### crates/conductor-run/tests/fixtures/over-envelope.toml — deleted
### crates/conductor-run/tests/journal_conformance.rs — added 2 line(s) in 1 range(s)
added: 16-17
### crates/conductor-run/tests/lamps_fixture.rs — deleted
