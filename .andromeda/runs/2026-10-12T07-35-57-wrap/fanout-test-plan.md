proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → Log format'
    change: 'The "Self-obs stream is a distinct artifact" label names the cli sinks alone (stderr / `logs/agent-latest.jsonl`); drop the "Tauri backend: `logs/conductor-tauri.jsonl`, with no writer since 2026-10-11 — the sink goes at `Panel-shaped types retired`" clause, or state it as retired at `Panel-shaped types retired` (2026-10-12): `ObsSink`''s doc names only the cli `--agent-mode` file and no shipped code names the sink. Apply in lockstep with the obs-plan §3 Logging stack / Log file location rows.'
    sidecar: 'test-plan §3 Log format — the Tauri backend self-obs sink `logs/conductor-tauri.jsonl` retired; the self-obs stream names the cli sinks only (lockstep with obs-plan §3).'
    rationale: 'Report Symbols / APIs, the window log sink bullet: `conductor-tauri.jsonl` is named in `crates/` only by `crates/conductor-run/tests/operator_pause_harvest.rs` (plan gate exit 1) and `ObsSink`''s doc names only the cli `--agent-mode` file; Expected amendments lists "test-plan §3 Log format key — the Tauri sink row retires (lockstep with obs-plan §3)", and the directive (`inputs#I6`) retires the masters'' Tauri log-sink rows in this wrap. The key file still says the sink "goes at `Panel-shaped types retired`" — this chunk — so leaving it one-sided against obs-plan §3 is drift.'
    basis: '.andromeda/registries/contracts/test-plan/log-format.md:6 · crates/conductor-core/src/obs.rs:59-60'
  - detector: D-tests-derived-count
    severity: warning
    section: '§7 Test Data & Fixtures'
    change: 'The committed fixture family''s "today" enumeration names `crates/conductor-core/tests/fixtures/` as the one crate-local tree (beside `scripts/fixtures/mutation-gate/`); drop `crates/conductor-run/tests/fixtures/` and its parenthetical (seeded into a gitignored runs dir through the production reader, or through `conductor_run::persist` for a `runs.db` row) — that tree left on 2026-10-12 with its two webview fixtures. Keep naming the family as a SET so the next removal does not re-stale it.'
    sidecar: 'test-plan §7 — `crates/conductor-run/tests/fixtures/` left the committed fixture family (2026-10-12, Panel-shaped types retired); the family is `crates/conductor-core/tests/fixtures/` + `scripts/fixtures/mutation-gate/`.'
    rationale: 'Report Counts / qualifiers moved: "`crates/conductor-run/tests/fixtures/` left the committed-fixture family: the family is now `crates/conductor-core/tests/fixtures/` + `scripts/fixtures/mutation-gate/` (sweep `run-fixture-tree`: standing 1 — `test-plan.md:316`)"; Files bullet: `over-envelope.toml` and `lamps-journal.jsonl` deleted and the directory no longer exists.'
    basis: '.andromeda/test-plan.md:316'
  - detector: D-tests-derived-count
    severity: warning
    section: '§7 Test Data & Fixtures'
    change: 'The round-trip enumeration (one per committed fixture) drops `crates/conductor-run/tests/lamps_fixture.rs` (lamp collision) and `crates/conductor-run/tests/envelope_fixture.rs` (the over-envelope breach and its persisted standing), leaving `conductor-core`''s `scenario_audit_gate::the_sweep_sees_a_wrapped_gloss_the_plain_form_misses` plus the mutation-gate `selftest`; the closing clause "so a persist-side change that leaves the file parseable but semantically inert fails" no longer has a persist-side member and should name the production reader/sweep side instead.'
    sidecar: 'test-plan §7 — the committed-fixture round-trip set loses `lamps_fixture.rs` and `envelope_fixture.rs` (deleted 2026-10-12); the wrapped-gloss round-trip and the mutation-gate selftest remain.'
    rationale: 'Report Files bullet: `crates/conductor-run/tests/envelope_fixture.rs` (4 tests) and `lamps_fixture.rs` (2 tests) deleted; Harness / gate surface: test targets `conductor-run --test envelope_fixture` and `--test lamps_fixture` removed; Expected amendments: "`webview-fixtures` — `test-plan.md:317` ×2 (the enumeration names `lamps_fixture.rs` and `envelope_fixture.rs`)".'
    basis: '.andromeda/test-plan.md:317'
    dependent-of: D-tests-derived-count
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Coverage triggers (security plan Vector 1)'
    change: '`CONDUCTOR_E2E_SEED_DIR` lost its only reader (`crates/conductor-run/tests/envelope_fixture.rs`, deleted 2026-10-12) — no code reads it — so its class and its mandated coverage (the seeder test''s unset NO-OP path, which no longer exists) leave this trigger, as the `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` / `CONDUCTOR_A11Y_STRICT` classes did on 2026-10-11.'
    sidecar: 'test-plan §1 Vector 1 trigger — the `CONDUCTOR_E2E_SEED_DIR` reader class and its seeder NO-OP coverage retired (its only reader `envelope_fixture.rs` deleted, 2026-10-12).'
    rationale: 'Report Symbols / APIs, Env vars: "`CONDUCTOR_E2E_SEED_DIR` lost its ONLY reader (the deleted `envelope_fixture.rs`); no code reads it now (plan gate `git grep … CONDUCTOR_E2E_SEED_DIR …` exit 1, no output)"; Expected amendments: "test-plan §1 Coverage triggers Vector 1 — the seed-dir reader class retires … `test-plan.md:83`". The trigger still mandates coverage through a test that no longer runs.'
    basis: '.andromeda/test-plan.md:83'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Untestable zones'
    change: 'Add an entry for the operator-accepted limits of `conductor suite`''s Ctrl-C stop: the first interrupt is witnessed by `#[cfg(unix)]` `the_first_interrupt_sets_the_stop_flag_and_the_process_lives` (a self-sent `kill -INT` awaited under a bounded `tokio::time::timeout`) on the Linux dev host only, until `Linux-only base CI` makes CI Linux (CI''s `rust` job runs on the Windows image §9 names); the second Ctrl-C''s immediate `std::process::exit(130)` is unwitnessed, with no owner — both by the operator''s word (`inputs#I4` (3)). The stop''s exit and the kept-records outcome stay unit-covered (`suite_exit`''s two tests, `drive_run`''s three).'
    sidecar: 'test-plan §1 Untestable zones — records the `conductor suite` Ctrl-C stop''s two accepted limits (unix SIGINT test on the Linux dev host only; second Ctrl-C unwitnessed), operator-accepted 2026-10-12.'
    rationale: 'Report Coverage of new surfaces (`conductor suite` Ctrl-C stop): "`#[cfg(unix)]` the_first_interrupt_sets_the_stop_flag_and_the_process_lives — Linux dev host only; the second Ctrl-C unwitnessed by the operator''s word"; Cross-project bullet, the `inputs#I5` card rows "Deferral to an existing entry" and "Accepted without a witness". A new path whose witness never runs in the CI arrangement test-plan §9 states (windows image, `rust` job) and a path left unwitnessed are tier gaps the doc should record as named limits rather than leave implicit, as it does for the Windows creation flag.'
    basis: 'crates/conductor-cli/src/commands/suite.rs:134-157 · crates/conductor-cli/src/commands/suite.rs:79-99'
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → Cross-surface parity'
    change: 'Name the in-process arm as the composition `conductor suite` drives in production: `conductor_run::{preflight, drive_run}` — `drive_run` (now taking a `progress: FnMut(usize)` callback and returning `DriveOutcome { records, stopped }`) has `conductor suite` as its first production caller since 2026-10-12 — so the parity leg compares the `suite` composition against the headless `conductor run`.'
    sidecar: 'test-plan §5 Cross-surface parity — the in-process arm is named as the `conductor suite` composition (`drive_run`''s first production caller, 2026-10-12).'
    rationale: 'Report Symbols / APIs, CHANGED `conductor_run::drive_run`: "Callers now: `conductor suite` (production — its FIRST production caller) and the tests in … `cross_surface_parity.rs`"; Expected amendments: "test-plan §5 Cross-surface parity — the arm is the in-process suite composition … each states the in-process arm as `conductor_run::{preflight, drive_run}`, which still holds; the amendment is the naming". The leg''s statement stays true; the naming records that the new production path is the arm parity covers.'
    basis: '.andromeda/test-plan.md:204'
note: 'Parity sites test-plan.md:77, :89, :288 state the in-process arm as `conductor_run::{preflight, drive_run}`, still true — no dependents raised; D-tests-framework (nextest, cargo test, llvm-cov, rstest) and D-platform-claim (no platform/runner verdict retired; no Spec-claims entry touches one) have no hit; the verb count 7→8 and test totals 1212→1218 / binaries 64→62 appear nowhere in test-plan.'
