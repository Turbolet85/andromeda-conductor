proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Primary screens (commands)"
    change: "Add a `conductor scenarios` bullet: lists the scenario catalogue (loaded through `list_scenarios` + the capability manifest) as a third `comfy-table` shape — Scenario · P-IDs · SLO, P-IDs in the ID-cyan mapping (ANSI 117) behind the stdout TTY gate, no verdict/state column and so no bracket prefix, sized to the terminal width — then a plain roll-up line (`1 scenario` for one, `N scenarios` otherwise, the count read from the catalogue, never a literal); an empty catalogue prints `No scenarios found.`; exit 0; an absent capability manifest is a harness fault (`error:` + `hint:`, exit 1)."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Primary screens gains the `conductor scenarios` catalogue-listing verb (Scenario · P-IDs · SLO table, singular/plural roll-up, empty-catalogue line, manifest-fault exit 1)."
    rationale: "Report Symbols / APIs: CLI verb `conductor scenarios` NEW (`Commands::Scenarios`, handler `commands/catalog.rs` 8-17, `render::scenarios_table` with `ContentArrangement::Dynamic`, `render::scenarios_rollup` 152-159). Counts / qualifiers moved: the verb set 7 → 8 and `layout-templates.md:32-38` holds no `scenarios` bullet. Expected amendments names this site `layout-templates.md:30-39`. A new user-facing surface region with no §Primary screens entry is drift."
    basis: "crates/conductor-cli/src/commands/catalog.rs:8-17 · crates/conductor-cli/src/render.rs:152-159 @153 · crates/conductor-cli/src/cli.rs:22-23"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Primary screens (commands)"
    change: "Extend the `conductor suite` bullet with the operator stop: after preflight, before driving, a first Ctrl-C prints the stop-requested line on stderr and the suite stops before its next scenario — the spinner is abandoned in place, the envelope caption and results table print over the kept records, `suite stopped: {kept} of {total} scenarios kept` prints on stderr, and the exit is 130; a second Ctrl-C exits 130 at once; a completed suite still exits on the hard-Fail rule."
    sidecar: "2026-10-12 (panel-shaped-types-retired): the `conductor suite` bullet records the Ctrl-C stop — stop-requested line, kept-records table, `suite stopped: {kept} of {total}` line, exit 130."
    rationale: "Report Symbols / APIs: `conductor suite` now drives through `drive_run` with `install_stop_handler` (`commands/suite.rs` 79-99 @84), `render::STOP_REQUESTED` (`render.rs` 113-115), `render::suite_stopped_line` (`render.rs` 117-120), exit `ExitCode::from(130)` via `suite_exit` (70-77). Expected amendments lists `suite`'s stop line and exit 130 at `layout-templates.md:30-39`; the bullet at `layout-templates.md:32-38` states none of it."
    basis: "crates/conductor-cli/src/commands/suite.rs:79-99 @84 · crates/conductor-cli/src/render.rs:113-115 · crates/conductor-cli/src/render.rs:117-120 @118"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Primary screens (commands)"
    change: "Extend the `conductor report <run_id>` bullet: after the `Run report <id>` line and a blank line it prints the stored run's `[ENVIRONMENT-SUSPECT]` load-envelope caption once when that run's stored standing is suspect (no flag; no new lamp, label or column), then the unchanged six-column table."
    sidecar: "2026-10-12 (panel-shaped-types-retired): the `conductor report` bullet records the stored `[ENVIRONMENT-SUSPECT]` caption printed once for a suspect run."
    rationale: "Report Symbols / APIs: `conductor report` prints `render::envelope_caption` of `read_envelope`'s stored standing once when suspect (`commands/report.rs` added 21-24); Expected amendments names `report prints the stored caption` at `layout-templates.md:30-39`."
    basis: "crates/conductor-cli/src/commands/report.rs:21-24"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Component — Hero / signature output (Paused-count hold-point — CLI mirror)"
    change: "Add to the interactive hold: after the HOLD phase line and before the `inquire` confirm, each declared `[[checklist]]` item prints two indented plain lines on stderr — `  induced: {induced}` and `  observe: {observation}` — no colour, no bracket label, while the spinner is suspended; the headless (non-TTY) path is unchanged."
    sidecar: "2026-10-12 (panel-shaped-types-retired): the CLI hold prints each checklist item's indented `induced:` / `observe:` lines between the HOLD line and the confirm (interactive path only)."
    rationale: "Report Symbols / APIs: CLI hold — `PromptResolver::resolve` prints `render::hold_checklist_lines` (`render.rs` 98-111 @101) on stderr after `hold_line`, before `confirm`, inside the spinner-suspended closure (`pause.rs` added 38-40); `CliResolver::resolve` and the headless path unchanged. Expected amendments lists the hold's indented checklist lines for layout-templates."
    basis: "crates/conductor-cli/src/render.rs:98-111 @101 · crates/conductor-cli/src/pause.rs:38-40"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Component — Primary content block 1 (results / SLO table)"
    change: "The table-shape paragraph that sets the 4-column coverage matrix apart from the 6-column results/SLO table names the third shape too: the `conductor scenarios` catalogue listing (Scenario · P-IDs · SLO, no state column and no bracket prefix, ending in its `1 scenario` / `N scenarios` roll-up line) — none of the three is to be conflated with another."
    sidecar: "2026-10-12 (panel-shaped-types-retired): §Primary content block 1's table-shape distinction names the catalogue listing as a third shape beside results/SLO and coverage."
    rationale: "Sweep of the primary's claim: the paragraph enumerates the table shapes as two ('The two must not be conflated'); the report's Expected amendments calls the catalogue listing the 'third table shape: Scenario · P-IDs · SLO', so the two-shape enumeration under-runs the surface."
    basis: "crates/conductor-cli/src/render.rs:146-150 @148"
    dependent-of: D-layout-surface
note: "D-layout-derived-count and D-platform-claim have no hit (the doc names the verb SET, never a count; no platform verdict retired). The report's Found-and-not-owned item — `layout-templates.md:116` says the 6-column results table's width is 'detected dynamically from the terminal' while `results_table_styled` uses `ContentArrangement::Disabled` (`render.rs:276`; coverage `:335`), lean: re-word to 'sized to its content, never wrapped' — predates this chunk and no layout-templates detector covers it; it is the orchestrator's to card. If applied, the suite mockup's 'width detected dynamically' (`:80`) and 'widths from terminal' header annotation share the phrasing; the new catalogue listing, by contrast, IS terminal-sized (Dynamic)."
