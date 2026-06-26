# arch extract

## Relevance
partial — the chunk extends the Tauri GUI control surface (Occupied Resources, Standard Contracts, Conventions) with picker + start/stop commands; the core engine/model/contract are unchanged.

## Constraints
1. **Crate placement:** Code lives in `conductor-tauri` (the bin) per §Inherited Defaults and the established workspace boundary; picker UI lives in `crates/conductor-tauri/ui/` (npm asset subtree, not a Cargo member) per §Occupied Resources §Frontend asset subtree.
2. **Tauri command envelope:** New `#[tauri::command]` methods (`list_scenarios`, `list_suites`, `start_run`, `stop_run`) follow the IPC contract pattern established in §Standard Contracts and documented in layout-templates.md; request/response shapes must bind to the shared `conductor-tauri` IPC surface.
3. **Async runtime discipline:** `start_run` drives the existing core pipeline on the **core-owned `current_thread` runtime** (not Tauri's multi_thread shell) per §Established Decisions [Async Runtime Flavor]; the command must spawn a managed task that yields the core to the owned runtime, preserving determinism and timeline ownership.
4. **Denied capability permissions:** `capabilities/*.json` extends deny-by-default with ONLY the minimal list/start/stop allowlist per §Established Decisions; no blanket or forward-permission for later-chunk commands (run-report-view / operator-pause / live-Channel are deferred), per security-plan §Tauri GUI.
5. **No live streaming:** `Channel` for live emission counters + target status is **not added here** (ch4 concern); the chunk reports run-state coarsely via command return per §Real-time Strategy constraints, per scope.md boundaries.
6. **Frontend dependencies:** shadcn/ui + Radix introduction (Command/Select, cmdk, Radix primitives) adds npm deps; `package-lock.json` committed and `npm audit` gate applied per §Inherited Defaults §Frontend; Tailwind v4.1 (Oxide) integration tested per §Stack.
7. **No model change:** Verdict, ReportState, the run-report envelope, `runs.db` schema, and seam contracts are unchanged per §Established Decisions [Outcomes are values]; headless `agent-run` / CLI path untouched per Design Philosophy §Headless-drivable core.

## Patterns to follow
1. **Config-file enumeration:** Scenario/suite listing reads `CONDUCTOR_SCENARIOS_DIR` (environment-scoped path discovery per §Occupied Resources); the Tauri command mirrors the CLI's config discovery without hard-coding paths.
2. **Scenario-validation-at-load:** Picker's scenario selection string validated against the live catalog before passing to `start_run` (never shelled out); selection injection-class stays closed per security-plan §Code Patterns.
3. **IPC request/response shape:** `list_scenarios` / `list_suites` return a JSON array of scenario/suite objects (title + P-IDs); `start_run(selection)` returns run-state identifier (idle/running/done/aborted); `stop_run()` returns acknowledgment — all canonical-name serde shapes per §Conventions §Interface surfaces.
4. **Scenario-keying by P-ID:** Scenarios consumed by `start_run` are identified by their P-ID per §Conventions §Naming patterns ("no scenario without a P-ID"); picker rows display P-ID + title (not-color-alone) per a11y-plan §Visual.

## Anti-patterns to avoid
1. **Do NOT ship a live `Channel` stream in this chunk** — coarse state only; live counters are ch4 per scope.md boundaries §Boundaries.
2. **Do NOT introduce engine/seam MODEL change** — no Verdict/ReportState enum extension, no envelope schema alteration, no `runs.db` column addition; this chunk is a thin GUI shell over the unchanged core.
3. **Do NOT pre-add later-chunk permissions** — operator-pause / run-report-view / Coverage-matrix / live-Channel command permissions are deferred to their chunks; `capabilities/*.json` stays minimal per security-plan §Tauri GUI deny-by-default.

## Contract bindings
**IPC ↔ core command pipeline:** The `start_run` command binds to the existing `conductor-cli` `run` verb path; both call the same `conductor-core` library APIs (timeline engine, emit seam, verify seam, report seam) so determinism + journal-relative SLO math stay invariant. The command's use of the core-owned `current_thread` runtime (per §Established Decisions) ensures no work-stealing from the timeline scheduler — determinism-under-seed binds across the IPC boundary. **Security (dependency) ↔ arch:** New npm deps (shadcn/Radix/cmdk) must clear `npm audit` + be added to `package-lock.json` per §Inherited Defaults §Frontend; any transitive advisory justified in `deny.toml` per security-plan §Dependency Security. **Obs ↔ commands:** Commands self-observe via the existing `logs/conductor-tauri.jsonl` sink per obs-plan §3 + the amendment 2026-06-24-frameless-window-shell; no new obs surface or OTel SDK added.

## Acceptance criteria contributions
1. **(arch) Picker UI lives in `crates/conductor-tauri/ui/` (npm subtree) and commands in `conductor-tauri` bin per workspace boundary rules** (arch §Inherited Defaults, §Occupied Resources §Frontend asset subtree).
2. **(arch) `list_scenarios`, `list_suites`, `start_run`, `stop_run` are canonical-name serde IPC shapes conforming to §Standard Contracts §Internal core↔UI** (request/response JSON matched to command signature).
3. **(arch) `start_run` drives the core on the core-owned `current_thread` runtime preserving determinism** (arch §Established Decisions [Async Runtime Flavor] — no Tauri multi_thread leakage into the timeline scheduler).
4. **(arch) `capabilities/*.json` extends deny-by-default with ONLY the minimal list/start/stop allowlist; no blanket or forward-permission for ch4+** (security-plan §Tauri GUI, per scope.md boundaries).

## Relevant amendment history
- **2026-06-24-frameless-window-shell** (§Occupied Resources + §Stack + §Infrastructure Patterns): Registered `logs/conductor-tauri.jsonl` (Tauri backend self-obs stream) and the `tauri-build` / `generate_context!` frontend-before-cargo build-order coupling. **Relevance:** This chunk uses the same Tauri bin + self-obs sink; the build-order constraint applies (ensure npm `build` before cargo compile of `conductor-tauri`).
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Occupied Resources): Registered `CONDUCTOR_AGENT_MODE` env var and `logs/agent-latest.jsonl` artifact. **Relevance:** The chunk does not modify agent mode (read-only trigger), but the self-obs pattern (separate artifact sibling of runs dir) is inherited by the Tauri sink established above.
- **2026-06-15-design-token-typography-bundle** (§Stack + §Occupied Resources + §Inherited Defaults): Registered React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) + Fontsource + npm frontend stack; `package-lock.json` committed + `npm audit` gate. **Relevance:** This chunk first introduces shadcn/ui + Radix as new npm deps; the frontend stack, Tailwind v4 integration, and `npm audit` gate from this amendment apply directly.
- **2026-06-23-line-oriented-output-rendering** (§Stack): Registered owo-colors/indicatif/comfy-table + inquire as terminal-rendering crates. **Relevance:** The CLI's terminal-output pattern is out-of-scope (GUI uses web components), but the inquire prompting pattern (isatty-gated, non-blocking headless) may inform operator-pause design (future ch8) and the deny.toml advisory-management precedent applies if new npm deps introduce similar low-risk advisories.
