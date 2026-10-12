group: obs-plan
read: .claude/docs/obs-summary.md · 46 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 40 · «- 2026-06-26: Instrument a `#[tauri::command]` with a MANUAL»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/obs-summary.md:7   class: stale
  leaf-text: «8 crates + 2 surfaces»
  source: .andromeda/obs-plan.md:25 «The workspace's crate-per-seam members (`Cargo.toml [workspace] members`) + one surface (CLI headless) place Conductor at the lower end of the module-count spectrum.»
  replace-with: «8 crates + 1 surface (headless CLI)»

- leaf: .claude/docs/obs-summary.md:12   class: retired
  leaf-text: «cli → `logs/agent-latest.jsonl` (`--agent-mode`) / stderr (dev); Tauri backend → `conductor-tauri.jsonl` with no writer since 2026-10-11 (the sink goes at `Panel-shaped types retired`).»
  source: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «**CLI:** `logs/agent-latest.jsonl` (project root, relative to `CONDUCTOR_RUNS_DIR`) in agent mode; stderr in dev / non-agent mode»
  replace-with: «cli → `logs/agent-latest.jsonl` (`--agent-mode`) / stderr (dev).»

- leaf: .claude/rules/observability.md:16   class: omission
  leaf-text: «In-process ↔ headless parity is asserted by comparing the `runs.db` envelope»
  source: .andromeda/obs-plan.md:109 «Cross-surface parity (the headless CLI vs the in-process `conductor_run` composition `conductor suite` drives) is asserted by comparing the `runs.db` envelope»
  replace-with: «In-process (the `conductor_run` composition `conductor suite` drives) ↔ headless parity is asserted by comparing the `runs.db` envelope»

- leaf: .claude/rules/observability.md:17   class: retired
  leaf-text: «**Sinks:** cli → stderr (dev) / `logs/agent-latest.jsonl` (`--agent-mode`); Tauri backend → `conductor-tauri.jsonl` under `runs_dir.parent()/logs/`, with no writer since 2026-10-11 (the sink goes at `Panel-shaped types retired`).»
  source: .andromeda/registries/contracts/obs-plan/logging-stack.md:5 «**Sink (CLI):** JSON to stderr in dev / non-agent mode, JSON to file `logs/agent-latest.jsonl` under `--agent-mode` — a WRITER choice, never a format choice (one `JsonObsLayer` over every writer)»
  replace-with: «**Sinks:** cli → stderr (dev) / `logs/agent-latest.jsonl` (`--agent-mode`) — a writer choice, never a format choice (one `JsonObsLayer` over every writer).»

- leaf: .claude/rules/observability.md:26   class: omission
  leaf-text: «instrument MCP read-back / rusqlite / tonic egress with a manual client span (these seams aren't auto-instrumented).»
  source: .andromeda/obs-plan.md:190 «READ queries are witnessed by a §6 boundary-call `info!` inside the caller's span where the caller holds one (a `conductor-cli` caller, e.g. `conductor report` over `read_envelope`, holds none»
  replace-with: «instrument MCP read-back / tonic egress and the rusqlite run-persist WRITE (`db.insert_run`) with a manual client span (these seams aren't auto-instrumented) — a rusqlite READ gets a boundary `info!` inside the caller's span where the caller holds one (a `conductor-cli` caller such as `conductor report` holds none; the line is matched by `run_id`), never a `db.*` read span, and the `RunsDb::delete_run` teardown gets neither span nor log.»

- leaf: .claude/rules/observability.md:33   class: stale
  leaf-text: «(a `default-features = false` trim is a tracked follow-up)»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:3 «**The `default-features = false` follow-up is CLOSED, and its premise was measured false** (2026-09-07-dependency-polish).»
  replace-with: «(the `default-features = false` follow-up is CLOSED: the trim shipped 2026-09-07 at the workspace `opentelemetry-proto` entry and cannot shed the two SDK crates, which the `trace`/`logs` features also gate)»
