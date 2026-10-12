# security extract

## Relevance
partial — the chunk touches a garde trust-boundary bound (`MAX_CHECKLIST_TEXT`), retires the one recorded-residual test-binary path handle (`CONDUCTOR_E2E_SEED_DIR`), and adds operator-facing CLI entries whose args, output and stop mechanism fall under input-validation, error-sanitization and trust-boundary rules; the rest (type removal, doc comments) carries no security duty.

## Constraints
- The `[[checklist]]` halves must stay bounded `length(min = 1, max = MAX_CHECKLIST_TEXT)` with the field `dive`d, never `skip`ped, and `check_checklist` must still run from `from_toml_str`. Only the panel ("dialog row") justification leaves; the plan's row states that justification as the ceiling's reason, so a re-justified or re-sized ceiling is a P4 decision, and its plan restatement goes through the wrap's amendments (per security-plan §Input Validation, scenario-config row).
- A sibling-reading rule on the checklist must remain a `Scenario::check_*()` raising `CoreError::Config`, never a `#[garde(custom)]` (per security-plan §Security Anti-Patterns → Input).
- Any new CLI argument (the listing verb, a stop flag, a `report` envelope-standing flag, a hold checklist print) is validated at the `conductor-cli` arg parser. A scenario-name argument resolves only through the validated scenario load. A `run_id`-class positional's sink stays a bound parameter or a lookup, never a path join. If it ever becomes a path join, the canonicalize duty attaches (per security-plan §Input Validation, CLI-arguments row).
- A re-homed past-run envelope reader in a SHIPPED binary that reads under `CONDUCTOR_RUNS_DIR` must resolve that handle through the canonicalize + bounds-check at the `conductor-cli` edge before any read. Whether `read_envelope` already resolves through `resolve_under` is research's question (per security-plan §Input Validation, env-var path-handles row).
- Output from the new entries (catalogue listing, envelope standing, checklist items, the stop's terminal lines) goes to the operator on cli stderr or stdout. That output must carry no absolute host path (canonicalized `CONDUCTOR_*` dirs, `ANDROMEDA_PULSE_DATA_DIR`) and no internal seam-crate struct names. A scenario is named by its file stem, never its path (per security-plan §Error Handling).
- The graceful mid-run stop must not open an inbound listener, socket or IPC endpoint of Conductor's own. The `:4317` occupier stays the only deliberate bind (per security-plan §Security Anti-Patterns → Universal).
- If the stop needs a new crate or feature (for example a signal-handling dependency), the delta lands only with `cargo audit` and `cargo deny check advisories bans licenses sources` green over the new `Cargo.lock`. The basis is stated as a package count or a new-package diff, and any new license or advisory is a justified `deny.toml` entry (per security-plan §Dependency Security).

## Patterns to follow
- The `run`-target refusal's form: an `anyhow` harness fault rendered `error:` + `hint:` that names scenario file stems, never a path. Model the listing verb's and new flags' failure texts on it (security-plan §Input Validation, CLI-arguments row).
- Read faults carry `e.kind()` only, never the path. Apply this to any file the re-homed envelope reader opens (security-plan §Input Validation, committed-manifests row).
- `thiserror` per-seam enums collapse to `anyhow` only at the `conductor-cli` edge. Verification outcomes stay `Ok(Verdict/ReportState)`, so a stopped run's kept results remain typed values, not errors (security-plan §Error Handling).
- The harness shells' closed allowlists and argv-word discipline (`valid_run_id` / `Test-RunId`, the closed `case` for `--live`). If `scripts/agent-run.{sh,ps1}` gain a pass-through for a new verb, it follows rule (b): fixed program, operator value as a separate argv word, never a shell string (security-plan §Security Anti-Patterns → Code Patterns).

## Anti-patterns to avoid
- Retiring the panel justification by dropping the checklist text bound or switching the field to `#[garde(skip)]`. That leaves the boundary silently unvalidated (security-plan §Security Anti-Patterns → Input).
- Implementing the mid-run stop as a control socket, named pipe or loopback listener that another process signals (security-plan §Security Anti-Patterns → Universal and → API, the occupier-is-not-a-listener ban).
- Echoing a resolved scenarios/runs directory or a `ANDROMEDA_PULSE_DATA_DIR` value in the listing or envelope-standing output (security-plan §Security Anti-Patterns → Logging).

## Contract bindings
- Removing `envelope_fixture.rs` removes `CONDUCTOR_E2E_SEED_DIR`'s only Rust reader. Several security-plan rows record that reader and its explicit residual: §Threat Model Summary attack surface, §Input Validation `CONDUCTOR_E2E_SEED_DIR` row, §Bootstrap phases, §Secret Management, §Security Anti-Patterns → Input. Those rows go stale and retire through the wrap's amendments, never a phase edit. The same handle's test-plan/rules mentions bind tests.
- The window log-sink rows (obs-plan §3, the log-format key) retire with this chunk. The `ObsSink::File` `--agent-mode` sink stays under security-plan §Security Anti-Patterns → Logging's no-path-leak ban (obs ↔ security redaction boundary).
- The supply-chain acceptance below is run by the `rust` CI job's `cargo audit` + `cargo deny` steps (security ↔ tests §CI Integration). Any CI step edited for the fixture removal keeps those steps intact.

## Acceptance criteria contributions
- Scenario load still rejects a `[[checklist]]` item with an empty `induced` or `observation`, or one over the kept ceiling, as a load-time harness fault. A unit test pins each arm, and grep shows `ChecklistItem`'s halves under `length(...)` with the field annotated `dive` (per security-plan §Input Validation).
- After the fixtures leave, `git grep -n CONDUCTOR_E2E_SEED_DIR -- crates scripts .github` returns 0. That proves no Rust reader is left behind with the recorded unguarded-join residual (per security-plan §Security Anti-Patterns → Input).
- Each new command-line entry is run with `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_RUNS_DIR` set to the host's absolute paths, and its captured stdout+stderr holds 0 occurrences of the workspace's absolute root, `/home/` or `/tmp/` (per security-plan §Error Handling).
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` prints nothing before and after `cargo audit`. `cargo audit` and `cargo deny check advisories bans licenses sources` both exit 0 over the chunk's committed, un-drifted `Cargo.lock` (per security-plan §Dependency Security).
