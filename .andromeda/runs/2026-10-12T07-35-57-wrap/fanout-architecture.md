proposals:
  - detector: D-arch-resources
    severity: warning
    section: "Occupied Resources (On-disk artifacts / database)"
    change: "Retire the `logs/conductor-tauri.jsonl` row: the sink it said goes at `Panel-shaped types retired` is gone with this chunk — no writer since 2026-10-11, no shipped code names the path, and the one remaining name in `crates/` is the test witness `crates/conductor-run/tests/operator_pause_harvest.rs`; `ObsSink`'s doc names only the cli's `--agent-mode` file (`logs/agent-latest.jsonl`). Drop the row from the registry."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Occupied Resources — the `logs/conductor-tauri.jsonl` row retired; the window log sink held until this chunk is gone (named in crates/ only by operator_pause_harvest.rs's test witness; ObsSink's doc names only the --agent-mode file)."
    rationale: "Report Changes, Symbols/APIs sink bullet: `conductor-tauri.jsonl` named in `crates/` only by `operator_pause_harvest.rs` (census exit 1), `ObsSink`'s doc names only the cli's `--agent-mode` file; Expected amendments lists `tauri-sink` standing at `architecture.md:164` x2; the wrap directive `inputs#I6` says the masters' Tauri log-sink rows retire in this wrap. The row registers a resource with no writer and states its own retirement point as this chunk."
    basis: "conductor-0.4.0/chunks/2026-10-12-panel-shaped-types-retired/report.md:28; .andromeda/architecture.md:164"
  - detector: D-arch-resources
    severity: warning
    section: "Occupied Resources (Environment variables)"
    change: "Retire the `CONDUCTOR_E2E_SEED_DIR` row: its only reader, `crates/conductor-run/tests/envelope_fixture.rs`, was deleted this chunk and it has had no setter since 2026-10-11, so no code reads or sets it (`git grep` over crates, scripts, .github exit 1) — the handle no longer occupies the `CONDUCTOR_*` namespace. Drop the row."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Occupied Resources — the `CONDUCTOR_E2E_SEED_DIR` row retired; its one reader envelope_fixture.rs was deleted (no setter since 2026-10-11), no code reads it."
    rationale: "Report Changes, Env vars bullet: `CONDUCTOR_E2E_SEED_DIR` lost its ONLY reader (the deleted `envelope_fixture.rs`); no code reads it now; no env var added. Files bullet lists `envelope_fixture.rs` deleted. The row states a reader that no longer exists. Expected amendments: `seed-dir` standing at `architecture.md:184`."
    basis: "conductor-0.4.0/chunks/2026-10-12-panel-shaped-types-retired/report.md:26; .andromeda/architecture.md:184"
  - detector: D-arch-resources
    severity: warning
    section: "Occupied Resources (Crate names — workspace members)"
    change: "In the `conductor-run` entry, replace `the live-counter run driver` with `the suite driver (`drive_run`, which `conductor suite` drives in production — a per-scenario progress callback and a stop flag polled before each scenario, set by the suite's Ctrl-C handler; persists once, a stop keeps every finished record)`."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Occupied Resources crate row — conductor-run's driver re-framed from the panel's live-counter driver to the suite driver drive_run (conductor suite its first production caller; Ctrl-C stop flag; DriveOutcome)."
    rationale: "Report Changes, Symbols/APIs: `drive_run` CHANGED to `progress: P` (`FnMut(usize)`, scenarios completed) + `should_abort`, returning the NEW `DriveOutcome { records, stopped }`; callers now `conductor suite` (its FIRST production caller) and tests; `RunEvent`/`RunStage` (the Tauri `Channel` payload) REMOVED; Crates/modules: `conductor-run::drive` re-shaped (module doc: the suite driver). The `live-counter` wording names the retired panel's channel-fed counter. Expected amendments: the crate row at `architecture.md:155`."
    basis: "conductor-0.4.0/chunks/2026-10-12-panel-shaped-types-retired/report.md:16-17; .andromeda/architecture.md:155"
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace)"
    change: "In the tree's `conductor-run/` comment, replace `live-counter drive_run` with `the suite driver drive_run (conductor suite; Ctrl-C stop flag)`."
    sidecar: "2026-10-12 (panel-shaped-types-retired): key file directory-structure — conductor-run's comment re-framed from live-counter drive_run to the suite driver drive_run, in lockstep with the §Occupied Resources crate row."
    rationale: "Same claim as the crate row, restated in the keyed contract; the report's Expected amendments name `registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:23` (live-counter drive_run) beside `architecture.md:155`."
    basis: ".andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:23"
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: "Conventions (Config conventions)"
    change: "Restate `MAX_CHECKLIST_TEXT = 200`'s reason: replace `a dialog row's display bound` with `a bound on untrusted scenario-authored text the CLI's interactive hold prints as one terminal line each (`induced:` / `observe:`, the headless path unchanged)`; value, rule and `dive` unchanged."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Conventions — MAX_CHECKLIST_TEXT's reason restated from a dialog row's display bound to a bound on untrusted scenario text the CLI hold prints one terminal line each; value 200 and rule unchanged."
    rationale: "Report Changes, Symbols/APIs: `MAX_CHECKLIST_TEXT` — value `200` unchanged, both `length` attributes and the `dive` unchanged; docs restate the reason as a bound on untrusted scenario text the CLI hold prints as one terminal line each; CLI hold bullet: `PromptResolver::resolve` prints `render::hold_checklist_lines` on the interactive hold only. The dialog the doc names left with the window, so §Conventions states a rationale the shipped code no longer gives. Expected amendments: `checklist-bound` standing at `architecture.md:81`."
    basis: "conductor-0.4.0/chunks/2026-10-12-panel-shaped-types-retired/report.md:20; .andromeda/architecture.md:81"
note: "No D-arch-collision hit (no port, socket, env var, artifact path or process gains an owner — the Ctrl-C handler is an in-process signal registration, no bind) and no D-platform-claim hit (the disproved-claim bullet is a test count, no platform verdict retired). D-arch-decisions: tokio's `signal` feature adds no library and fits the current_thread decision. Not proposed, for the orchestrator's judgment: §Stack and Technologies' Terminal output rendering row enumerates comfy-table's uses as results + capability-coverage tables and does not name the new `conductor scenarios` catalogue table (an incomplete enumeration, not a contradiction). Standing rows confirmed true, no change: `architecture.md:50` (list_scenarios loader set), `:110` (`preflight()` is what `conductor suite` and `drive_run` call), `:54` (Real-time Strategy retired — RunEvent/RunStage removal completes it)."
