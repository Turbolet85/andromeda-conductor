# Codebase Research — 2026-06-25-scenario-suite-picker-start-stop

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs:** 4 · **Code-graph queries:** 3

## Files inspected
- `crates/conductor-tauri/src/main.rs` (full) — **pure stub**: `init_observability(...)` + `tauri::Builder::default().run(generate_context!())`. NO `.invoke_handler`, NO `.manage(state)`, NO core runtime ("the core `current_thread` runtime is not wired in here yet"). This is the single insertion point for commands + managed run-state + (option B) the runtime.
- `crates/conductor-tauri/Cargo.toml` (full) — deps are **only** `conductor-core` + `tauri` + `tauri-build`. Option B adds seam edges (timeline/emit/verify/report) or a new run-lib dep + `tokio`.
- `crates/conductor-tauri/tauri.conf.json` (full) — `withGlobalTauri:false` (frontend invokes via `@tauri-apps/api`, not `window.__TAURI__`); CSP set; `beforeBuildCommand` = `npm run build` wired. Commands register in Rust, NOT here — no conf change needed for commands.
- `crates/conductor-tauri/capabilities/default.json` (full) — deny-by-default ACL: window `allow-start-dragging`/`allow-minimize`/`allow-close` only. Its own description says start/stop/picker/run-report/operator-pause/Channel perms "are added with their commands" — i.e. this chunk.
- `crates/conductor-tauri/ui/package.json` (full) — deps: react19, react-dom, `@tauri-apps/api ^2`, fontsource; devDeps: tailwind v4.1, vite 8, ts 5.6. shadcn/Radix/cmdk/lucide/cva/clsx/tailwind-merge are the **new** additions.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — mounts `<Titlebar runState count/>` + a `<main>` token/type demo body; owns `useState<RunState>('live')` + the DEV-only backtick cycler (`import.meta.env.DEV`). The picker + run-controls mount inside `<main>` (replacing the demo); start/stop call `setRunState`.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full) — `export type RunState = 'idle' | 'live' | 'hold' | 'aborted'`; props `{ runState, count }`. The prop contract start/stop drives (start → `live`, stop → `aborted`/`idle`).
- `crates/conductor-cli/src/pipeline.rs` (full) — **the run pipeline, BIN-LOCAL in conductor-cli**: `preflight()` (returns `Preflight{client,ready}`), `execute_scenario(pf, scenario, run_id, resolver: &CliResolver)` (Blocked-when-not-ready spine; coarse emit→read-back→classify), `coarse_emit()`. Takes a CLI-local `&CliResolver`. **This is the code "drive the same core pipeline" refers to — and it is not in a library.**
- `crates/conductor-cli/src/commands/run.rs` (full) — run verb: `paths.load_scenario(target,seed)` → `pipeline::preflight` → `CliResolver::select(None,agent_mode)` → `execute_scenario` → `persist` → `print_record` → `exit_code`.
- `crates/conductor-cli/src/commands/suite.rs` (full) — suite verb: `paths.load_all_scenarios(filter,seed)` → one `preflight` → loop `execute_scenario` → `persist` → `results_table`. **"Suite" = the whole catalog (optionally name-substring-filtered) over one gate — there are NO named suite groups.**
- `crates/conductor-cli/src/commands/mod.rs` (full) — shared `persist()` (= `JournalWriter::create` + `RunsDb::open/insert` + `RunReport::write`), `print_record()`, `exit_code()` (non-zero only on a hard `Fail`).
- `crates/conductor-cli/src/paths.rs` (full) — `Paths{scenarios_dir,runs_dir,manifest_path}` resolved via `resolve_under` (traversal guard, security-plan §Input Validation); `load_scenario` (by name or P-ID), `load_all_scenarios(filter,seed)` (read_dir → `*.toml` → `Scenario::from_toml_str`, sorted) — **the catalog enumerator, bin-local; the natural thing to share for `list_scenarios`.**
- `crates/conductor-cli/src/cli.rs` (full) — clap surface: `Run{target,seed}` / `Suite{filter,seed}` / `Report` / `Preflight{json}` / `Coverage{write}`; `--agent-mode`/`--debug` global.
- `crates/conductor-core/src/scenario.rs` (1-90) — `Scenario{ name:String, p_ids:Vec<PId>, seed:u64, slo_tier:SloTier, phases, jitter_ms }`; `PId(String)` serde-transparent; `SloTier` serde-renamed `<5s`/`<20s`/`<90s`. The lightweight DTO `list_scenarios` returns is `{name, p_ids, slo_tier}`.
- `Cargo.toml` (workspace, full) — 8 members; **`tauri = "2.10.3"` already pinned** (line 72; the security extract's "currently 2.10.1" is STALE — ch1 pinned it). So NO tauri bump is needed; the security gate is a ≥2.10.3 *guard*, not a bump.
- `scenarios/*.toml` (glob) — **31 scenarios** in the catalog (receiver-lifecycle-state … degraded-mode-report).

## Graph impact (from `tree-query-{marker}.json`)
- **`crate_edges` (conductor-tauri):** exactly one row — `conductor-tauri → conductor-core`. It is a near-leaf; Option A keeps this single edge (the list helper lands in core). Option B adds 4 seam edges (or one run-lib edge) + `tokio`.
- **`execute_scenario` callers:** exactly 2 — `conductor-cli` `run.rs:20` and `suite.rs:28`. **Zero cross-crate consumers** → extracting it to a library (Option B) is a contained move touching only those 2 call sites + the new lib home.
- **`load_all_scenarios` callers:** exactly 1 — `suite.rs:18` (a `Paths` method). Lifting catalog enumeration to a shared core helper (Option A) updates this one site.

## Patterns detected
- **Pipeline shape** (`pipeline.rs`): preflight-once → per-scenario `execute_scenario` → `persist`; the **Blocked path is the CI-tested spine** (no live Pulse → `Blocked`, exit 0). Any Tauri `start_run` mirrors this shape.
- **Injected resolver** (`pipeline.rs:107` + core `resolve_hold`/`HoldPoint`): `execute_scenario` takes `&CliResolver`, but the hold seam (`resolve_hold(resolver, &hold)`) already lives in `conductor-core` over a resolver abstraction — so generalizing `execute_scenario` to `&dyn`/`impl` resolver for Tauri is feasible (Option B).
- **CONDUCTOR_\* + `resolve_under` traversal guard** (`paths.rs`): scenario/runs dirs are resolved under cwd, rejecting traversal before IO — the Tauri list/start commands must resolve the same way (security: no path escape).
- **Catalog enumeration** (`load_all_scenarios`): `read_dir → *.toml → from_toml_str`, sorted — the `list_scenarios` command reuses exactly this.
- **RunState prop + App state + DEV cycler** (`Titlebar.tsx`/`App.tsx`; frontend.md learning): start/stop become real `setRunState` drivers; the DEV cycler is retired in ch4 (its CARRY), so it may coexist this chunk.
- **Persist triple** (`mod.rs persist`): `JournalWriter` + `RunsDb` + `RunReport` — Option B's `start_run` calls the same persist for CLI/Tauri parity.

## Conventions to follow
- **Tauri command self-obs** (`observability.md`, obs-plan §3/§4): `#[tracing::instrument]` → bounded span name `tauri.command.*`; `run_id` correlation; NO OTel SDK; anyhow-sanitized returns (no host paths / struct names to the webview).
- **Deny-by-default ACL** (`security.md`, security-plan §Tauri GUI): extend `capabilities/default.json` minimally for the new commands; **NOTE** Tauri 2 may treat app-local `#[tauri::command]`s as inherently callable (the ACL gates `core:`/plugin perms) — verify at implement; either way the deny posture must not widen.
- **Frontend** (`frontend.md`): tokens by NAME (`var(--…)`, never hex/px); shadcn vendored over Radix + Lucide (per stack); tokens on `:root` not `@theme`; `vite-env.d.ts` for `import.meta.env`; **build-order** — `npm run build` before any cargo compile of `conductor-tauri` (already wired in agent-run `run` + CI).
- **Cross-surface parity** (test-plan Path 7): `start_run(P-ID,seed=S)` must produce the identical `runs.db` envelope as `conductor run <P-ID> --seed S` — the gate for *executed* runs (Option B now; Option A defers to ch4).
- **Verdict/error wall**: command results are typed values; `tonic::Status`/MCP errors → `Blocked`/`Fail`, never panic.

## New files to create
- **ui** — `src/components/ScenarioPicker.tsx` (+ `.css`) · `src/components/RunControls.tsx` (+ `.css`) · vendored shadcn under `src/components/ui/` (`command.tsx`, `select.tsx`, `button.tsx`) + `src/lib/utils.ts` (`cn()` over clsx + tailwind-merge).
- **backend** — `crates/conductor-tauri/src/commands.rs` (or a `commands/` module): `list_scenarios`, `start_run`, `stop_run`.
- **Option A** — a small read-only catalog-list helper in `conductor-core` (e.g. `scenario_catalog.rs`: `list_scenarios(dir) -> Vec<ScenarioSummary{name,p_ids,slo_tier}>`), shared with the CLI.
- **Option B** — a library home for the run pipeline: either move `pipeline.rs` into `conductor-core` (no new member) or a new `conductor-run` crate (9th member); generalize the resolver to a trait object.

## Files to modify
- `crates/conductor-tauri/src/main.rs` — `.invoke_handler(generate_handler![...])` + `.manage(run-state)`; (Option B) stand up a core-owned `current_thread` runtime under Tauri.
- `crates/conductor-tauri/Cargo.toml` — (Option A) likely no new Rust dep if the list helper is in core; (Option B) seam/run-lib edges + `tokio`.
- `crates/conductor-tauri/capabilities/default.json` — extend with the command perms (if Tauri 2 requires them).
- `crates/conductor-tauri/ui/src/App.tsx` — mount picker + controls; wire start/stop → `setRunState` + `invoke`.
- `crates/conductor-tauri/ui/package.json` (+ `package-lock.json`) — shadcn/Radix/cmdk/lucide/cva/clsx/tailwind-merge; `npm audit` clean.
- **Option A** — `crates/conductor-cli/src/paths.rs` — delegate `load_all_scenarios` to the shared core helper (DRY).
- **Option B** — `crates/conductor-cli/src/{pipeline.rs,commands/run.rs,commands/suite.rs}` — pipeline extraction call-site updates; possibly `Cargo.toml` (workspace members) for a new crate.

## Open questions
1. **`start_run` depth — the central scope fork (→ P4 AskUserQuestion).** **(A)** Control surface + command scaffold: real `list_scenarios` (shared core helper) + picker + start/stop commands + capabilities + backend run-lifecycle state driving the titlebar `RunState`, but `start_run` validates + transitions only — real execution + the pipeline-to-library extraction defer to **ch4** (where the Channel makes a run observable; ch4 already CARRYs the run-state-driver swap). vs **(B)** Full execution now: extract `pipeline.rs` to a library + generalize the resolver + stand up a current_thread runtime under Tauri, `start_run` drives a real `RunRecord` (Blocked w/o live Pulse) writing `runs.db`/journal — CLI/Tauri parity (Path 7) assertable now, at the cost of a cross-seam refactor landing without the Channel.
2. **Tauri 2 ACL semantics** — do app-local `#[tauri::command]`s need explicit `capabilities` entries, or are they inherently callable (ACL gates only `core:`/plugin perms)? Confirm at implement; deny posture holds either way.
3. **Command set** — research shows no named suites, so likely `list_scenarios` + `start_run(selection)` + `stop_run` (3 commands), with suite-vs-single encoded in the `start_run` arg + an optional filter — not a separate `list_suites`. (Plan default; not a user fork.)
