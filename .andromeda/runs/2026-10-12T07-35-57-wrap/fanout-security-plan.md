proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (the `CONDUCTOR_E2E_SEED_DIR` row, security-plan.md:112)"
    change: "Retire the row to one retired line: `CONDUCTOR_E2E_SEED_DIR` has no reader and no setter since 2026-10-12 (its only reader `crates/conductor-run/tests/envelope_fixture.rs` was deleted at `2026-10-12-panel-shaped-types-retired`; its setter `wdio.conf.ts` left 2026-10-11); the recorded unguarded-join residual and the seeding-test enforcement cell leave with the reader."
    sidecar: "security-plan §Input Validation: the CONDUCTOR_E2E_SEED_DIR row retired — its one reader envelope_fixture.rs deleted, no code reads the handle (2026-10-12-panel-shaped-types-retired)."
    rationale: "Report Changes → Symbols / APIs, Env vars bullet: CONDUCTOR_E2E_SEED_DIR lost its ONLY reader (the deleted envelope_fixture.rs); no code reads it now (git grep over crates scripts .github exit 1); Files bullet: envelope_fixture.rs and the fixtures/ directory deleted. The row still names envelope_fixture.rs as the reader and as the enforcement point, and carries a residual whose subject no longer exists. Expected amendments lists this site (seed-dir and webview-fixtures rows at :112)."
    basis: "crates/conductor-run/tests/envelope_fixture.rs (deleted)"
  - detector: D-security-input
    severity: escalate
    section: "Threat Model Summary (the CLI/env entry-point vector, security-plan.md:51)"
    change: "Drop `CONDUCTOR_E2E_SEED_DIR` and its test-binary-handle qualifier from the `CONDUCTOR_*` entry-point list; the namespace's entry points are `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`."
    sidecar: "security-plan Threat Model: CONDUCTOR_E2E_SEED_DIR leaves the entry-point list — no reader since 2026-10-12."
    rationale: "Same retired claim as the :112 row (report Env vars bullet: no code reads the handle). An entry point with no reader is not an attack-surface entry. Report Expected amendments names security-plan :51 (Threat Model entry point)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "Bootstrap phases (derive for route / setup-project) → input-validation-library-install (security-plan.md:209)"
    change: "Remove the three CONDUCTOR_E2E_SEED_DIR clauses: the 'One handle is out of this phase's scope' sentence, the 'CONDUCTOR_E2E_SEED_DIR is not the only handle reaching Rust in a test binary' framing (restate as: CONDUCTOR_RUNS_DIR also reaches Rust in test binaries — journal_conformance.rs and the three live-pulse readers), and the closing 'CONDUCTOR_E2E_SEED_DIR's recorded residual is unchanged'."
    sidecar: "security-plan Bootstrap input-validation-library-install: the CONDUCTOR_E2E_SEED_DIR scope carve-out and residual clause retire with the handle's last reader."
    rationale: "Report Env vars bullet: the handle's only reader was deleted; the phase text still scopes work around that reader and asserts its residual stands. Report Expected amendments names security-plan :209 x3 (Bootstrap scope)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "Secret Management → Development (security-plan.md:242)"
    change: "Drop `CONDUCTOR_E2E_SEED_DIR` and its qualifier from the list of non-secret `CONDUCTOR_*` configuration handles."
    sidecar: "security-plan Secret Management: CONDUCTOR_E2E_SEED_DIR leaves the handle list — no reader since 2026-10-12."
    rationale: "Report Env vars bullet: no code reads the handle and no env var was added. Report Expected amendments names security-plan :242 (Secret Management — Development)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "Security Anti-Patterns → Input (security-plan.md:314)"
    change: "Remove the 'ONE reserved-namespace path handle sits outside this rule: CONDUCTOR_E2E_SEED_DIR ... read by envelope_fixture.rs' sentences and their explicit-residual sentence; restate the per-READER clause without it ('CONDUCTOR_RUNS_DIR also has Rust test-binary readers — journal_conformance.rs and the three live-pulse readers'), and drop the closing 'CONDUCTOR_E2E_SEED_DIR's explicit residual above is unchanged'."
    sidecar: "security-plan Anti-Patterns → Input: the CONDUCTOR_E2E_SEED_DIR carve-out and residual retire with envelope_fixture.rs."
    rationale: "Report Env vars bullet and Files bullet (envelope_fixture.rs deleted): the ban's carve-out names a reader that no longer exists and a residual with no subject. Report Expected amendments names security-plan :314 x3 (seed-dir) and :314 (webview-fixtures)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (the scenario-config row, `[[checklist]]` block, security-plan.md:107)"
    change: "Replace '(ceiling `200` — a dialog row's display bound)' with '(ceiling `200` — a bound on untrusted scenario text the CLI hold prints, one indented plain line per half)', pin it to its load-path tests (`a_checklist_half_at_the_bound_loads`, `an_out_of_bound_checklist_half_is_a_validation_fault` → `CoreError::Validation`, `a_checklist_beside_expected_checks_is_a_config_fault` → `CoreError::Config`), and change '`HoldPoint.checklist` carries the same dived vector to the shells' to '... to the CLI hold (`PromptResolver::resolve`, interactive path; the headless path prints none)'."
    sidecar: "security-plan §Input Validation: the [[checklist]] bound's reason re-stated for the CLI hold (no dialog row remains) and pinned to its first tests; HoldPoint.checklist's consumer is the CLI hold alone."
    rationale: "Report Symbols / APIs: MAX_CHECKLIST_TEXT value 200 and both length attributes unchanged, docs restate the reason as a bound on untrusted scenario text the CLI hold prints as one terminal line each (scenario.rs added 80-81, 92-94); CLI hold bullet: PromptResolver::resolve prints hold_checklist_lines, CliResolver and the headless path unchanged (pause.rs added 38-40); Schema / config bullet: the bound gained its first tests (scenario.rs 971-1026). Expected amendments names security-plan :107 x2 ('a dialog row's display bound')."
    basis: "crates/conductor-core/src/scenario.rs:80-81"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (the CLI arguments / stdin row, security-plan.md:116)"
    change: "Add: `conductor scenarios` (2026-10-12) takes no argument and loads the catalogue through `list_scenarios` + the capability manifest (absent manifest a harness fault, `error:` + `hint:`, exit 1, no absolute path); `conductor suite`'s stop is an in-process OS signal registration (`tokio::signal::ctrl_c`, installed after preflight) carrying no value — the first Ctrl-C sets the stop flag, the second exits 130 — so neither adds an input to validate."
    sidecar: "security-plan §Input Validation CLI-arguments row: records conductor scenarios (no argument, manifest-checked load) and suite's value-free Ctrl-C stop."
    rationale: "Report Symbols / APIs: CLI verb `conductor scenarios` NEW (cli.rs added 22-23; handler commands/catalog.rs 8-17), loads through list_scenarios + the capability manifest; `conductor suite` installs a Ctrl-C handler (commands/suite.rs 79-99 @84); Ports bullet: an OS signal registration, no bind, no file. The validation the row mandates is present (Coverage of new surfaces: manifest-checked load ✓, no argument; test scenarios_with_an_absent_capability_manifest_is_a_harness_fault) — the drift is the row's enumeration of CLI input surfaces omitting them. Expected amendments names security-plan :116."
    basis: "crates/conductor-cli/src/commands/catalog.rs:8-17"
note: "D-security-subprocess, D-security-deps, D-platform-claim: no hit (no sidecar/preflight/data-dir change; Dependencies none — tokio gains the `signal` feature over already-locked packages, Cargo.lock byte-identical; no platform verdict retired). Unproposed observations for the orchestrator: (1) suite.rs's `#[cfg(unix)]` test spawns a fixed-argv `kill -INT <own pid>` from a cargo TEST binary — covered by §Code Patterns' categorical 'a spawn from a cargo TEST binary sits outside rule (b)' clause, which records only secret_scan_gate by name; (2) the checklist bound is length-only per the doc, so 'one line per half' is a rendering claim, not one the validation enforces (no newline/control-character rule); (3) the report does not state the path-resolution route of `conductor scenarios`' capability-manifest read, which §Input Validation :109 requires of every SHIPPED reader (fixed default_path() through resolve_under)."
