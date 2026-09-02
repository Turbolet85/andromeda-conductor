# Codebase Research — 2026-09-02-screen-reader-manual-spec

## Scope
- **Depth:** moderate · **Reads:** 27 (12 ui source files in full, 5 Rust ranges, 4 config/manifest files, the
  committed fixture + its pin test, the hold-declaring scenario, 2 e2e specs by anchor grep, 3 installed
  packages by grep) · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL including `## Session
  Additions` (its `paths:` are `scripts/agent-run.*` · `conductor-cli/**` · `conductor-verify/**` ·
  `crates/**/tests/**`, so a chunk landing under `crates/conductor-tauri/ui/test/` — singular — does NOT
  auto-load it; the read is deliberate). Eight additions applied to the stimulus recipe: fresh data dir per leg
  under `%TEMP%\pulse-legs\<ts>` (2026-08-16/-18) · NEVER `boot` before a leg that fires its own preflight —
  quiet window ≥120s idle + 30s resolver tick after ANY canary (2026-08-19, extended 2026-09-01) · the PATH
  prefix as a spawn-resolution input, POSIX form in bash, `which andromeda-pulse-mcp` before the leg
  (2026-08-20) · the attended GUI leg's recipe — PowerShell not bash, the whole env block in ONE paste
  (`PATH` · `ANDROMEDA_PULSE_DATA_DIR` · `ANDROMEDA_PULSE_MCP_ENABLED` · `ANDROMEDA_PULSE_L4_DETERMINISTIC` ·
  `CONDUCTOR_SCENARIOS_DIR`), echoed back before launch (2026-08-22) · the trimmed repo-relative scenarios
  dir under gitignored `runs/` that puts a hold at count 0 in seconds instead of ~76 minutes (2026-08-19
  extended 2026-08-22) · the bundle-serving build is `--features tauri/custom-protocol` (2026-08-22, corrected
  2026-09-01) · every leg booting an external process ends with a before/after PROCESS CENSUS (`tasklist`) and
  names its STOP form beside its firing form (2026-09-02). Also auto-loading on the artifact home: `a11y.md`
  and `frontend.md` (both cover `crates/conductor-tauri/ui/**`).

## Files inspected
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full) — `RunState = 'idle' | 'live' | 'hold' | 'aborted'`;
  labels `Conductor · idle` / `Conductor · live` / `Conductor · HOLD — operator pause` / `Conductor · aborted` in a
  `<span aria-live="assertive">`; the count is a bare `<span>` (default text `00:00:00`) with NO accessible name
  and no live region; min/close buttons carry `aria-label="Minimize window"` / `"Close window"`; the `<header>`
  is a direct child of the root div, so it exposes the implicit `banner` landmark.
- `crates/conductor-tauri/ui/src/App.tsx` (1-59, 60-130, 130-330) — `STATE_FOR_STAGE`: `progress→live`,
  `blocked→idle`, `done→idle`, `aborted→aborted`; `start()` sets `live` + `count '0'` BEFORE invoking, and the
  Channel's `count` is the number of scenarios completed (an integer, not a clock); `stop()` sets `aborted`
  client-side the moment `stop_run` returns; the hold Channel captures `document.activeElement` as the focus-
  restore target and sets `hold`. Render tree: `<main>` with three `<h2>` sections (`Scenario / suite`,
  `Coverage matrix`, `Run report`), NO `<h1>`, NO footer. Prose as shipped: `Loading scenarios…` /
  `Could not load scenarios: …` (`role="alert"`) / `No scenarios found.` · `Loading coverage…` / `Could not
  load coverage: …` (`role="alert"`) / `No coverage data.` · `Loading run report…` / `Could not load run
  report: …` (`role="alert"`) / `No run yet`; `actionError` renders `role="alert"`. There is NO "Run in
  progress" prose anywhere — the live state is carried by the titlebar label + count only.
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` + `src/lamp.ts` (full) — `span.lamp` = glyph
  `aria-hidden="true"` + visible `lamp__label` text from the closed set `Pass` / `Fail` / `HOLD` / `Manual` /
  `Residual` / `Blocked`; NO `role="status"`, NO `aria-live` on the lamp. `lampForRecord` is verdict-first.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — `<section aria-label="Capability
  coverage matrix">` (a named `region`), a tally header, `div.cov__scroll tabIndex=0 role="group"
  aria-label="Coverage rows"`, a real `<table>` with `<th scope="col">` P-ID / Capability / Mode / Status; the
  Status cell is a lamp or the text `Not yet run`. No live region anywhere.
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` (full) — `<section aria-label="Run report">`; the
  optional envelope `<p>` (label + cause); header `{N} scenario(s) · run {run_id}`; `div.report__scroll
  tabIndex=0 role="group" aria-label="Run report rows"`; `<table>` P-ID / Scenario / Status / Latency / SLO / FP.
  It renders ONLY lamps — no `ManualCheck` row expands into a checklist here.
- `crates/conductor-tauri/ui/src/components/OperatorChecklistView.tsx` + `OperatorChecklist.tsx` (full) —
  `<section aria-label="Operator checklist">` → `<ul>` of `<label>` + native `<input type="checkbox">` +
  induced/observation spans, then `<p role="status" aria-live="polite">` reading `{n} of {m} unconfirmed` /
  `All observations confirmed`.
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx` (full) — Radix `AlertDialog` (`role=
  "alertdialog"`, `aria-modal`, `aria-labelledby` → Title, `aria-describedby` → Description); Title = the
  backend `title`, Description = `body`; the checklist renders as a SIBLING of Description via
  `OperatorChecklistView`; actions `Abort` (`AlertDialog.Cancel`, present when `allowNoGo`) then `Proceed`
  (`AlertDialog.Action`); `onCloseAutoFocus` restores focus to `restoreFocusTo()`.
- `crates/conductor-tauri/ui/src/components/RunControls.tsx` (full) — `role="group" aria-label="Run controls"`;
  `Start` uses `aria-disabled` (kept focusable as the hold's restore target); `Stop` uses native `disabled`
  when not running.
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` (full) — cmdk `Command label="Scenario or
  suite picker"`, input placeholder `Filter scenarios by name or P-ID…`, `Command.Empty` `No scenarios match.`,
  items with `aria-current` on the selection and text `{name}` + `{p_ids · slo_tier}` (` · selected`).
- `crates/conductor-tauri/ui/src/main.tsx` (10-17) — the Gallery is `import.meta.env.DEV`-gated and tree-shaken
  from the release bundle.
- `crates/conductor-tauri/src/commands.rs` (160-335) — `run_report` / `run_envelope` read the latest run's
  journal from the runs dir (empty dir ⇒ empty list ⇒ "No run yet", never an error); `start_run` spawns
  `run_thread`: a preflight `Err` (harness fault) or a `drive_run` `Err` emits `RunStage::Aborted`.
- `crates/conductor-tauri/src/pause.rs` (24-41) — `HoldPrompt.title = "{p_id} — {step}"`, `body = prompt`.
- `crates/conductor-run/src/lib.rs` (300-312, 425-450, 471-560, 905-950) — OTLP egress unreachable at preflight
  is a BLOCKED `ReadyState` (`canary emission failed: OTLP egress to 127.0.0.1:4317 unreachable`), never an
  `Err`; `execute_scenario` returns a `Blocked` row when `!pf.ready` before any emission; the operator-checklist
  hold fires AFTER the scenario's phases and its read-back, with `step: "operator-checklist"` and
  `prompt: "Observe the operator-checklist claim for this scenario"`, carrying the TOML `[[checklist]]` items;
  `drive_run` polls `should_abort` only BETWEEN scenarios and emits `Aborted` there, else `Blocked` (all rows
  blocked) or `Done`.
- `scenarios/halo-hue-encoding.toml` (full) — P-025, two silent phases (`gap_ms = 3000` each, no emission),
  one `[[checklist]]` row: induced "error-pressure phase window; this scenario declares no emission of its own
  (the preflight canary's storm is the only error pressure)" · observation "hue shifted toward burgundy under
  error pressure?".
- `crates/conductor-tauri/ui/wdio.conf.ts` (20-92, 96-98, 133-180) — `specs: ['./test/a11y/accessibility.e2e.ts']`
  and `suites.driven: ['./test/a11y/operator-hold.e2e.ts']` are literal file paths, not globs; the routine arm
  seeds `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` into `runs/e2e-fixture/lamps-fixture.jsonl`
  and launches with `CONDUCTOR_RUNS_DIR=runs/e2e-fixture`, cwd = the workspace root; readiness waits on
  `#root` mounting.
- `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` + `tests/lamps_fixture.rs` (1-30) — three records:
  `lamps-fixture-pass` (P-019, P-041 · Pass · 2000 ms · <5s), `lamps-fixture-blocked` (P-019 · Blocked),
  `lamps-fixture-fail` (P-030 · Fail · 4000 ms · <90s); the collided P-019 renders Blocked (worst-lamp-wins).
- `crates/conductor-tauri/ui/test/tsconfig.json`, `test/README.md`, `package.json` scripts — the test tree
  compiles `a11y/**/*.ts` only; scripts `a11y` (routine) and `a11y:driven`; the README still carries the
  pre-2026-09-01 Linux-only framing and names the SR spec as "the next chunk".
- `crates/conductor-tauri/tauri.conf.json` + `ui/index.html` — window title `Conductor`, `decorations: false`,
  `<html lang="en">`, `<title>Conductor</title>`.
- `.andromeda/a11y-plan.md` §1 · §3 · §4 · §5 · §11 — the four-state naming, the must-announce list, the §4
  catalog, the §5 focus order, the Screen Reader bans (read for the plan-vs-shipped reconciliation).
- `crates/conductor-tauri/ui/test/a11y/*.e2e.ts` (anchor grep) — the shipped anchor vocabulary: `[role=
  "alertdialog"]`, `[role="status"]`, `[role="option"]*=…`, `button=Start`, `input[type="checkbox"]`,
  `[class~="report__envelope-label"]`, the six `LAMP_LABELS`, `aria-label` / `aria-disabled` attribute reads.
- Installed packages (grep, not docs): `cmdk` 1.1.1 renders `combobox` (input, with `aria-expanded` /
  `aria-controls` / `aria-activedescendant` / `aria-autocomplete`) + `listbox` + `option` (`aria-selected`)
  and `Command.Empty` as `role="presentation"`; `@radix-ui/react-alert-dialog` 1.1.17 `onOpenAutoFocus`
  focuses `cancelRef` (the Abort button) on open.

## Graph impact (from the code-graph query, ts plane, `db_state: fresh`)
- Query 1 (symbol probe, 15 rows) confirmed the announcement sources are indexed: `Titlebar()` +
  `STATE_LABEL` (`Titlebar.tsx:5/12`), `LAMP_META` (`lamp.ts:11`), `StatusLamp()`, `OperatorChecklistView()`,
  `OperatorChecklist()`.
- Query 2 (callers, 88 rows) — **`StatusLamp()`** is rendered by `CoverageMatrix.tsx:95`, `RunReport.tsx:74` and
  the DEV-only `Gallery.tsx:156`; **`OperatorChecklistView()`** by `OperatorPauseDialog.tsx` (the dialog site)
  and `Gallery.tsx:198` ONLY — no release-bundle caller renders the checklist in the run report; **`Titlebar()`**
  by `App.tsx:218`; **`OperatorPauseDialog()`** by `App.tsx:312` + Gallery; **`RunReport()`** / **`CoverageMatrix()`**
  by `App.tsx:307` / `:286` + Gallery. The chunk modifies none of these — the graph fixes the SHIPPED
  announcement inventory the spec enumerates, and shows the plan's "two checklist render sites" reduces to one
  in release. Trace: `.andromeda/runs/2026-09-02T06-56-57Z-phase/tree-query-2026-09-02-screen-reader-manual-spec.json`.

## Patterns detected
- **Announcement carriers as shipped** (`Titlebar.tsx:25`, `App.tsx:240/262/279/300`, `OperatorChecklistView.tsx:18`):
  exactly THREE mechanisms exist — the assertive titlebar label, `role="alert"` error prose, and the polite
  `role="status"` checklist roll-up. Everything else (lamps, count, tables, empty/loading prose, picker empty
  state) is static content a screen reader reaches only by navigation.
- **Landmarks as shipped** (`Titlebar.tsx:22`, `App.tsx:221`, `CoverageMatrix.tsx:55`, `RunReport.tsx:34`,
  `OperatorChecklistView.tsx:16`): `banner` (implicit `<header>`) · `main` · three named regions (`Capability
  coverage matrix` · `Run report` · `Operator checklist`) · two named groups (`Run controls` · `Coverage rows` /
  `Run report rows`). No `contentinfo`, no `<h1>`.
- **The hold sequence** (`lib.rs:471-560`, `pause.rs:38`, `OperatorPauseDialog.tsx`): phases → read-back →
  `HoldPoint` → dialog opens with title `P-025 — operator-checklist`, description `Observe the
  operator-checklist claim for this scenario`, one checkbox row, roll-up `1 of 1 unconfirmed`, focus on
  `Abort`; Space toggles → roll-up `All observations confirmed`; Proceed / Abort / Escape resolve and restore
  focus to `Start`.
- **Fixture-seeded terminal state without a live Pulse** (`wdio.conf.ts:58-79`, `lamps_fixture.rs`): copying
  the committed journal to `runs/e2e-fixture/lamps-fixture.jsonl` and launching with
  `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` from the repo root renders the report (`3 scenarios · run
  lamps-fixture`, rows Pass / Blocked / Fail) and the matrix lamps (P-019 Blocked, P-041 Pass, P-030 Fail, the
  rest `Not yet run`) — the idle-with-report sub-state, reproducible on any host.
- **Evidence convention** (`conductor-0.2.0/chunks/*/evidence/`): `leg-verdict.md` + machine-readable
  witnesses (`leg-witnesses.jsonl`, `envelope-status.json`) per chunk; dated live legs record the data dir's
  provenance without host paths.

## Conventions to follow
- **Host-path-free artifacts**: names of env handles, never values (`verification-harness.md` §Status envelope;
  security-plan §Error Handling); the NVDA install location is never recorded, only its version.
- **Anchor vocabulary is role / accessible name / text** (test-plan §6 Selector strategy; the shipped specs use
  `[role="alertdialog"]`, `[role="status"]`, `button=Start`): the spec's node column names nodes the same way.
- **Kebab-case Markdown filenames** for on-disk artifacts (architecture §Conventions).
- **Declared-not-runnable-here rows** follow the `CONDUCTOR_MSEDGEDRIVER` skip precedent — named, never green.
- **The attended-leg env block in ONE paste, echoed back** (`verification-harness.md` 2026-08-22).

## New files to create
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — the per-state pass spec (four shipped
  states, the idle-with-report sub-state, a row per must-announce item, the stimulus recipe per state, the
  VoiceOver/Orca not-runnable-here note, the supplemental/no-conformance-claim statement).
- `conductor-0.2.0/chunks/2026-09-02-screen-reader-manual-spec/evidence/nvda-pass.json` — the recorded operator
  pass, machine-parseable (one object per spec row: state · item · outcome · heard · SC), plus the identity
  block (NVDA version · WebView2 runtime · build commit · date · stimulus run_id where one exists).
- `conductor-0.2.0/chunks/2026-09-02-screen-reader-manual-spec/evidence/leg-verdict.md` — the human-readable
  verdict + process census, the house evidence shape.

## Files to modify
- `crates/conductor-tauri/ui/test/README.md` — point the "NVDA/VoiceOver manual spec are the next chunk" line at
  the spec file (the Linux-only framing in the same README is pre-existing drift the sweep already surfaced;
  rewording it beyond the pointer is wrap's amendment territory, not this chunk's).
- `conductor-0.2.0/verification-matrix.json` — `v2-23` `chunk` + concretized acceptance at P5 (ledger, not spec).
- No `.rs`, no `.tsx`, no `wdio.conf.ts`, no manifest, no lockfile — unless the operator pulls an
  attribute-level remedy into scope at P5 (then `StatusLamp.tsx` / `Titlebar.tsx` / `App.tsx` are the sites).

## Open questions
- **Which stimulus for `live` / `hold` / `aborted` the plan commits to** → blocks: plan-decision. Measured: a
  no-Pulse run settles to `Blocked` within seconds (unmet `shell-declaration` term or egress-unreachable, both
  BEFORE the canary poll), so `live` lasts too briefly to observe and `Stop` is natively disabled the rest of
  the time; `hold` fires only after a green preflight; the backend emits `Aborted` only at a scenario boundary,
  so an observed `aborted` needs a run of ≥2 scenarios with `Stop` pressed during the first (the label flips
  client-side at once, then follows the Channel). Therefore ONE live leg over a trimmed two-scenario dir
  (`halo-hue-encoding` + `halo-breathing-encoding`) exercises `live`, `hold`, `aborted` and the terminal report;
  `idle` (empty) and idle-with-report need no Pulse. Resolved in P4 as the leaned recipe.
- **Where the not-announced findings go** → blocks: implementation-scope. Default per scope: recorded, not
  fixed; the candidate attribute-level remedies are (a) `role="status"` on the lamp or its cell (plan §4),
  (b) an accessible name on the titlebar count, (c) a `contentinfo` footer (a design surface, not an
  attribute). Operator's call at P5.
- **Whether NVDA reads WebView2 content on this host without configuration** → blocks: nothing until the pass
  (WebView2 is Chromium and exposes IAccessible2/UIA like Edge; confirmed at the operator's first launch, not
  from the repo).

## Scope premise closure
Every `[inferred]` bullet in `scope.md` was re-checked; `scope.md` is amended accordingly.
- **VERIFIED** — (1) the four states are the shipped `RunState` set and the plan's `report-terminal` is `idle`
  with a report rendered; `aborted` is shipped and unnamed by the plan (`Titlebar.tsx:4-11`, `App.tsx:67-72`).
- **VERIFIED, sharpened** — (2) the plan's prose differs from what ships: `No scenarios found.` (plan: "No
  scenarios loaded"), `No run yet` (plan: "No run yet — pick a scenario/suite to begin"), and NO in-progress
  prose at all; three loading strings and three `role="alert"` error strings exist that the plan does not name;
  the picker's `No scenarios match.` is `role="presentation"`.
- **VERIFIED** — (3) the lamp carries no live region (`StatusLamp.tsx`); the matrix and report scroll regions are
  `role="group"`; so a lamp change is announced only by navigation.
- **VERIFIED + premise-corrected** — (4) the dialog-site roll-up IS announced — the dialog renders the same
  `OperatorChecklistView` (`role="status" aria-live="polite"`); but the plan's SECOND render site (the run
  report's `ManualCheck` rows expanding into a checklist) does NOT ship in the release bundle — the only other
  caller is the DEV-only Gallery (graph query 2; `RunReport.tsx` renders lamps only).
- **VERIFIED** — (5) `main` and the implicit `banner` ship; NO footer / `contentinfo` (`App.tsx:219-311`); three
  `h2`, no `h1`.
- **VERIFIED, corrected** — (6) cheapest stimuli: `idle` = launch; idle-with-report = the fixture seed + env
  handle, no Pulse; `live` / `hold` / `aborted` need one live leg (no-Pulse runs settle `Blocked` in seconds;
  `Stop` is disabled when idle; `Aborted` is emitted only between scenarios — ≥2 scenarios in the dir).
- **VERIFIED — the a11y-plan claim is FALSE as stated** — (7) an automated NVDA driver EXISTS: `@guidepup/guidepup`
  0.34.0 (npm registry, modified 2026-08-31) with `@guidepup/setup` 0.25.2 — "Screen reader automation library
  for testing", NVDA + VoiceOver. Adoption is barred here by the one-automation-stack rule (architecture §Scope
  law, a11y-plan §11, test-plan §6), the dependency gate, and the standing directive; recorded as a finding for
  wrap (a11y-plan §3 "No automated SR tool exists for the stack" needs the qualifier "adopted", not "exists").
- **UNVERIFIED IN REPO, deferred to the pass** — (8) NVDA reads WebView2 through IAccessible2 / UIA without
  configuration; stated as a precondition to confirm on the first launch.
- **Still `[inferred]`, awaiting the P5 card** — the reading of the standing NVDA directive.
- **New findings the spec records (not fixes):** the dialog title is `{p_id} — operator-checklist`, not the
  plan's frozen-count + step-index header (`pause.rs:38`, `lib.rs:538`); the titlebar count has no accessible
  name and its meaning (scenarios completed) is visual-only; the `Start` control announces `aria-disabled`
  ("unavailable") while `Stop` is natively disabled.

## Addendum — P5 `review` relay (2026-09-02): the pass re-shaped to agent-driven

The operator's consolidated relay fell the "operator ears" premise on this host and re-shaped the PASS (the
spec stays). Facts below were relayed as verified and RE-VERIFIED here before the plan was re-synthesized.
- **NVDA 2026.2 is present as a PORTABLE copy** (user-space, no admin) at an operator-owned path with its
  `userConfig` beside it — the exe and the config dir both exist (ls, 2026-09-02); no `nvda.exe` was running.
  Its path is a HOST value: it reaches the harness only through a new env handle (`CONDUCTOR_NVDA`, the
  `CONDUCTOR_MSEDGEDRIVER` shape — existence + `isFile` + metacharacter rejection, unset ⇒ skip at exit 0),
  never committed, never logged.
- **NVDA's own speech log is the driver — zero new dependencies.** Operator smoke-tested CLI form: start
  `nvda.exe -m --no-sr-flag -l 12 -f <log-file>` (`-m` no sounds/UI; `--no-sr-flag` leaves the system
  screen-reader flag alone; `-l 12` = input/output level, every utterance sent to the synthesizer is logged),
  stop `nvda.exe -q` (0 nvda processes afterwards). The log IS the `Heard` column.
- **Pulse is UP for the leg** (operator-launched): `pulse-app.exe` PID 50164 holds `127.0.0.1:4317`
  LISTENING (netstat, PID-matched); deterministic L4 + MCP enabled (relayed, log-verified by the operator);
  the data dir is a fresh leg dir under `%TEMP%\pulse-legs\` with `corpus/ logs/ run/` present; no canary
  fired yet, so the quiet window is satisfied. The sidecar `andromeda-pulse-mcp.exe` exists in the Pulse
  repo's `target/release` (built 2026-09-01) — its directory is the `PATH` prefix; the 2 ms `[BLOCKED]` rule
  applies if the name does not resolve.
- **Harness fact the plan now depends on** (`wdio.conf.ts:128-140`): `onPrepare` spawns tauri-driver with
  `env: { ...process.env, CONDUCTOR_RUNS_DIR: FIXTURE_RUNS_DIR }` for EVERY suite — the app under test
  inherits the shell env (so `CONDUCTOR_SCENARIOS_DIR` from the shell reaches it) but `CONDUCTOR_RUNS_DIR`
  is forced to the seeded fixture. A suite that must start idle on `No run yet`, or on an empty / malformed
  catalog, therefore needs the spawn site to select its subject env by the invoked suite name (wdio passes
  the launcher `config`, `suite` included, to `onPrepare`). One spawn site, subject chosen per suite — an
  extension of the harness, not a re-authoring.
- **What WebDriver-injected keys can and cannot make NVDA say (to be MEASURED at implement, not assumed):**
  focus changes, live-region updates (`aria-live`, `role="alert"`, `role="status"`) and dialog-open events are
  accessibility events NVDA announces regardless of how focus moved, so Tab / Space / Enter / Escape sent
  through the driver produce speech. NVDA's BROWSE-MODE commands (H / D / arrows over static text, headings
  and table cells) are intercepted by NVDA's own OS-level keyboard hook, which the driver's synthesized key
  events may bypass — rows whose only path is browse-mode reading (headings list, landmark list, prose,
  table cells) may be unreachable by the agent arm. The spec classes every row `focus/live` or `browse`;
  the leg measures whether browse rows speak, and any row the agent arm cannot produce falls to the
  operator's manual arm with the arm recorded per row.
- **The attach question the operator named:** whether NVDA attaches to the WebView2 window while WebDriver
  holds the session — measured by the first idle rows (a silent log through the idle probes = no attach);
  fallback = the manual operator pass, arm recorded.
- **Updated new-file / modify lists (supersede the lists above):** NEW `crates/conductor-tauri/ui/test/a11y/
  screen-reader.e2e.ts` (the SR spec: drives the states, writes the action timeline) · NEW `crates/
  conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (node built-ins only: speech log + action
  timeline → the pass JSON, first-pass grading by required-token match) · NEW `crates/conductor-tauri/ui/
  test/a11y/screen-reader/nvda-pass-spec.md` (the spec, unchanged home) · NEW `chunks/…/evidence/
  nvda-pass.json` + `leg-verdict.md` (the reviewed copy of the leg's output) · MODIFY `wdio.conf.ts` (three
  `sr*` suites, the `CONDUCTOR_NVDA` guard, NVDA start before / quit after the driver, per-suite subject env)
  · MODIFY `package.json` (`a11y:sr`, `a11y:sr-empty`, `a11y:sr-error` scripts — no dependency change) ·
  MODIFY `test/README.md` (pointer) · MODIFY `verification-matrix.json` (`v2-23`). `test/tsconfig.json`
  already includes `a11y/**/*.ts`, so both new TS files ride `typecheck:e2e` unchanged.
