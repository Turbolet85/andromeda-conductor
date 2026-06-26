# Scope — Scenario/suite picker + start/stop

**Marker:** `2026-06-25-scenario-suite-picker-start-stop`
**Version:** conductor-0.1.0 · **Epoch 9 (Desktop control panel) ch3/10**
**Crate:** `conductor-tauri` (bin: `#[tauri::command]`s + `capabilities/` + `ui/`) — the first GUI **control surface**; **zero engine/seam MODEL change expected** (the start/stop commands DRIVE the existing core run path, they do not alter it). A read-only scenario-enumeration helper may add one small additive `conductor-core`/seam API (P4).

## What it builds

The first interactive control surface of the desktop panel: **choose a scenario or suite, then start/stop a run.** ch1 stood up the frameless shell; ch2 gave the titlebar count its run-state behaviour (DEV-driven). This chunk adds the **picker + run controls** and the `#[tauri::command]` contract + capability permissions behind them — the commands the frameless-window-shell scope explicitly deferred ("start/stop, scenario/suite picker … are later Epoch-9 chunks, each brings its own capability permission").

Pieces:

1. **Scenario/suite picker (shadcn Command/Select)** — a searchable picker listing the available scenarios (the `scenarios/*.toml` catalog, one per P-ID) and the named suites. shadcn/ui Command (searchable palette) and/or Select, built on Radix primitives — **this is the chunk that first introduces shadcn/Radix into `ui/`** (ch1/ch2 deferred it). The list is populated by a real **read-only** `list_scenarios` (+ `list_suites`) `#[tauri::command]` enumerating `CONDUCTOR_SCENARIOS_DIR`. Each row carries its P-ID(s) + title (not-color-alone, text-first). Exactly which shadcn components + Radix/cmdk/utility deps land, and their Tailwind v4 integration, is a **P4 decision**.

2. **Start/stop run controls** — semantic `<button>` controls (Start enabled once a scenario/suite is picked; Stop enabled while a run is in flight) wired to `start_run(selection)` / `stop_run()` `#[tauri::command]`s. Start kicks off a run over the **same core pipeline** the headless `conductor run`/`suite` verbs drive (timeline→emit→verify→report on the **core-owned `current_thread` runtime**, in a managed Tauri async task); Stop requests cancellation. **The DEPTH of `start_run` execution in this chunk — run end-to-end now vs stand up the command + run-lifecycle wired to the existing core path — is the central P4 decision**, bounded below.

3. **Backend run-lifecycle state** — a backend notion of "is a run in flight" (idle → running → done/aborted) that gates Start/Stop enablement and feeds the run-state the titlebar already renders (ch2's `RunState` prop: idle/live/hold/aborted). Start → `live`/running; Stop/done → `aborted`/`idle`. **No streaming `Channel`** — coarse state via command return / simple shared state only; the live emission-counter + target-status STREAM is **ch4 (Live-counter Channel stream)**, which also retires ch2's DEV-only cycler and drives `runState` from the Channel (its CARRY).

4. **Deny-by-default capability permissions** — extend `capabilities/*.json` with **only** the permissions these commands need (the `list_scenarios`/`list_suites`/`start_run`/`stop_run` allowlist entries). No blanket/wildcard; the run-report-view / operator-pause / live-`Channel` permissions stay deferred to their later chunks.

## Boundaries (what it does NOT touch)

- **No live `Channel` stream / live counters** — backend→frontend streaming of emission counters + target status is **ch4**, which also retires ch2's DEV-only run-state cycler. ch3 reports run-state coarsely (command return / simple state); it adds NO `Channel`.
- **No engine/seam MODEL change** — `Verdict`/`ReportState`, the scenario model, journal/envelope, `runs.db`, and the timeline/emit/verify/report seams are unchanged. `start_run` DRIVES the existing core pipeline (as the CLI does); it does not alter verdict/journal logic. Any scenario-enumeration helper is a read-only **additive** API.
- **Not the status-lamp / dialog / operator-checklist primitives** — the six status-lamp variants + dialog scaffold + operator-checklist primitive are **ch5 (Component primitives library)**. ch3 uses shadcn Command/Select for the picker only.
- **Not the coverage-matrix view (ch6), run-report / operator-checklist views (ch7), or the operator-pause go/no-go dialog (ch8)** — Start may transition the titlebar into a `hold` state visually, but the go/no-go `AlertDialog` that gates each committed timeline step is **ch8**; ch3 does not build it. Verified live-Pulse runs (non-Blocked) are an **Epoch-10** concern (live runs stay Blocked until then).
- **No a11y TEST harness** — axe/tauri-driver are the closing Epoch-9 chunks (ch9/ch10). ch3 honours the STATIC a11y rules (keyboard-operable picker, focus order + visible ring, semantic buttons, not-color-alone) without standing up the test stack.
- **GUI is convenience, never the release gate** — nothing here may block or alter the headless `agent-run` contract; the CLI run/suite/report path stands as-is (frontend.md). The picker/controls are a thin shell over the same core.

## Surfaces / contracts touched

- **architecture.md §Standard Contracts (Tauri commands) / §Async Runtime Flavor / §Real-time Strategy** — the internal `#[tauri::command]` request/response surface (`list_scenarios`/`list_suites`/`start_run`/`stop_run`); core-owned `current_thread` runtime drives the run under Tauri's shell runtime; the `Channel` is the (next-chunk) live surface, not added here.
- **layout-templates.md §(picker / control surface, desktop)** — the scenario/suite picker + run-control layout within the window body (below the ch1 `banner` titlebar).
- **frontend.md / design-system.md (expression 0.3)** — shadcn/ui + Radix introduced under React 19 / Vite 8 / Tailwind v4.1 (Oxide); design tokens by NAME; borders-only depth; CSS-only motion; no router. First shadcn landing ⇒ Tailwind v4 + Radix integration care.
- **a11y-plan.md §Keyboard / §Forms-controls / §Focus / §Visual** — the picker is fully keyboard-operable (Radix/cmdk give roving focus + type-ahead); Start/Stop are semantic `<button>`s with visible focus rings + disabled semantics; selection / run-state not-color-alone (text label, not tint alone).
- **security-plan.md §Tauri GUI / §Code Patterns / §Dependency Security** — deny-by-default `capabilities/*.json` extended with the minimal start/stop/list allowlist; `tauri ≥2.10.3`; no `shell-open` with scenario-derived strings; any new `ui/` dep (Radix/cmdk/cva/clsx/tailwind-merge/lucide) ⇒ `npm audit` clean + `package-lock.json` committed; the `start_run` selection is validated against the known catalog, never shelled out (the scenario string → `.env`/argv injection class stays closed).
- **obs-plan.md §3** — the start/stop/list commands self-observe via the existing `conductor-tauri` `tracing` sink (`logs/conductor-tauri.jsonl`); a run carries its `run_id`; processor-stage redaction reused (no new policy); NO OTel SDK / browser OTLP.
- **`conductor-tauri` `src/main.rs` (+ a `commands` module) · `tauri.conf.json` · `capabilities/*.json` · `Cargo.toml`** — the Rust command surface + ACL; **`ui/src/`** — the picker + run-control components + their wiring to the commands; **`ui/package.json` + `package-lock.json`** — the shadcn/Radix dep introduction.
- **Possible read-only `conductor-core` (or a seam) scenario-catalog enumeration API** — if listing scenarios warrants a shared helper rather than a Tauri-local dir walk (P4; mirrors how the CLI discovers scenarios).

## Acceptance intent (full criteria synthesized in plan.md)

- The window body renders a scenario/suite picker (shadcn Command/Select over Radix) populated by a read-only `list_scenarios`/`list_suites` command from `CONDUCTOR_SCENARIOS_DIR`; each entry shows its title + P-ID(s) (not-color-alone); the picker is fully keyboard-operable with a visible focus ring.
- Start/Stop are semantic `<button>` controls wired to `start_run`/`stop_run` `#[tauri::command]`s; Start is enabled only with a selection, Stop only while a run is in flight; the backend run-lifecycle state gates enablement and the titlebar run-state.
- A run started from the GUI drives the SAME core pipeline as the headless `run`/`suite` verb (to the depth fixed at P4), on the core-owned `current_thread` runtime, in a managed task; Stop requests cancellation; **no streaming `Channel` is added** (ch4).
- `capabilities/*.json` stays deny-by-default — extended with ONLY the list/start/stop allowlist entries; no blanket permission; no later-chunk command permissions pre-added.
- No engine/seam MODEL change; the headless `agent-run` contract is untouched; the CLI path still passes.
- Gates green: `ui/` `tsc` (strict — no `any`/`as`) + `vite build` → `ui/dist` · `npm audit` clean + `package-lock.json` committed (shadcn/Radix deps added) · workspace `cargo nextest` · doctest · clippy `-D warnings` · `cargo audit` + `cargo deny check` (any new advisory/license justified in `deny.toml`, never silently skipped) · `Cargo.lock` re-committed un-drifted · `agent-run.sh run` + `status` exit 0.
