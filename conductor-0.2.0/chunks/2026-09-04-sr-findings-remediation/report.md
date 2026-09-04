# Report — 2026-09-04-sr-findings-remediation

**Chunk:** SR findings remediation — the eight standing NVDA findings fixed at their defects, then the sr* leg re-run against a tightened spec
**Date:** 2026-09-04
**Commits:** none since `last_wrap` (2026-09-04T01:53Z) — this wrap writes the chunk's first commit

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-run/src/lib.rs` · `crates/conductor-tauri/ui/src/App.tsx` ·
  `ui/src/components/{Titlebar,ScenarioPicker,CoverageMatrix,RunReport}.tsx` ·
  `ui/test/a11y/screen-reader.e2e.ts` · `ui/test/a11y/screen-reader/{parse-nvda-log.ts,rows.ts,nvda-pass-spec.md}`
  (10 source files). New: `conductor-0.2.0/chunks/2026-09-04-sr-findings-remediation/{scope,research,plan,report}.md`
  + `evidence/{nvda-pass.json,leg-verdict.md}`.
- **Symbols / APIs:**
  - `conductor_run::drive_run` — behaviour only, **signature unchanged**. The abort arm is unified: a single
    `aborted` flag set by the loop-head poll OR a new post-loop poll, then one `persist` + one terminal `emit`.
    Its call sites are unchanged and KEEP working: 1 production caller
    (`conductor-tauri commands/run_thread()` @ `crates/conductor-tauri/src/commands.rs:315`) + 5 in-crate tests
    (code-graph rust plane, `rows: 6`, `db_state: fresh`).
  - New in-crate test `conductor_run::tests::drive_run_reports_an_abort_raised_during_the_last_scenario`.
  - `Titlebar.tsx` — new exported const `IDLE_COUNT` (`'00:00:00'`), consumed by `App.tsx`'s count state so the
    placeholder has one source.
  - `ScenarioPicker.tsx` — new module-local component `FilterMiss` (not exported); new import
    `useCommandState` from `cmdk` (an existing dependency, no new package).
  - `screen-reader.e2e.ts` — new helper `expectActiveScrollRegion(name)`; 4 call sites moved off
    `expectActive`. `expectActive` itself is retained and still used by other rows.
  - `parse-nvda-log.ts` — `Timeline['foreground']` and the emitted `foreground` record both gain
    `tabsToStart` / `tabs_to_start`.
  - **No new IPC method, endpoint, export, port, socket or env var.** No `#[tauri::command]` added or changed.
- **Crates / modules:** none added, removed or renamed. Touched: `conductor-run`, `conductor-tauri` (ui subtree only).
- **Dependencies:** **none added, none bumped.** `Cargo.lock` un-drifted at **564 packages**;
  `package.json` / `package-lock.json` untouched.
- **Schema / config:** the SR evidence record (`nvda-pass.json`) `subjects[].foreground` object gains one
  field, `tabs_to_start: number | null` — emitted inside `writeNvdaPass`'s existing scrub/bound path, so the
  ingest controls cover it. No migration, no config key, no scrub/redaction shape change.
- **Spec-master edits:** **none** — no `.andromeda/` master was touched by this chunk (they are P2's).
- **Counts / qualifiers moved:** the SR pass census moved — `announced-as-expected` **33 → 34**,
  `announced-differently` **1 → 0**, the record's `findings` array **16 → 15** (S0-16 left it). **No spec-master
  BODY states any of these literals — verified by grep**; the 2026-09-02 census appears only at
  `a11y-plan-amendments.md:48`, sidecar history that is never edited.
- **Dev-tool versions:** none.
- **Harness / gate surface:** the live `sr` subject's firing form gains a **fifth** env handle,
  `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` — `wdio.conf.ts:126` gives the `sr` suite no `scenarios`
  field (unlike `sr-empty`/`sr-error`), so the shell must supply it; without it the app loads all 35 scenarios
  and the walk dies at S0-04 on option order. The four handles previously recorded (`CONDUCTOR_NVDA`,
  `CONDUCTOR_MSEDGEDRIVER`, the three `ANDROMEDA_PULSE_*`) are unchanged. No agent-run verb, xtask verb, CI
  step or status/verdict shape changed.
- **Cross-project / external claims:** the leg ran against a live **andromeda-pulse** instance (deterministic
  L4 + MCP) launched by the operator at 06:06Z with data dir under `%TEMP%/pulse-legs/`; sidecar
  `andromeda-pulse-mcp` resolved from that repo's `target/release` (built 2026-09-01). Basis read: the probe's
  own output plus the leg's live run `2026-09-04T06-56-42-321`. No claim about Pulse's source is made here.
- **Reverted / negative API facts:** the client's `setRunState('aborted')` in `App.tsx`'s `stop()` was
  **removed and then restored**. Removing it made the backend sole owner of the terminal state; the live leg
  measured the cost (the phase line still read `live` at the S3-01 bound, because `drive_run` polls only
  between scenarios) and it was restored with the backend's post-loop poll as its correctness guarantee.
- **Insufficient fixes (written, kept, not the remedy):** the **load-time alert region** (four `role="alert"`
  wrappers in `App.tsx`, mounted empty at first paint). The change is correct and shipped, and it did resolve
  the region-mounts-with-its-text defect; it does **not** discharge finding 4, because the leg's R0-01 action
  still reloads the document, so a first-load announcement and a post-reload one remain indistinguishable.
  Remainder owned by the CARRY on *Sidecar spawn without a console window*.
- **Spec claims disproved by measurement:**
  1. **`conductor preconditions` cannot exit 0 under a fully correct environment.** `observe_preconditions`
     (`crates/conductor-run/src/lib.rs:380`) passes all three `ANDROMEDA_PULSE_*` names through `declares()`
     (`:359`), which accepts only a value of `"true"`/`"1"`. `ANDROMEDA_PULSE_DATA_DIR` is a PATH, so it can
     never declare. Measured 2026-09-04T06:41Z with the full operator env: `egress-reachable` and
     `sidecar-resolvable` both satisfied, `handles-declared` unmet naming DATA_DIR alone, exit 1; a child
     process was independently confirmed to receive the value intact. Consequence: `agent-run boot`'s leading
     arm has short-circuited before every preflight since `480bc66`. Stated as satisfiable in
     `architecture.md` §Standard Contracts (Liveness equivalent) and `.claude/rules/verification-harness.md`
     §5-command discipline. Existing unit tests cannot catch it — they build the `declared` set directly.
  2. **The plan's deferral premise for findings 3 and 8 is withdrawn** — their hold rows were driven live
     (S2-01/S2-02 fired the hold; S2-07, S3-04, S3-05 all graded). Recorded because the plan's Goal states the
     deferral as fact.
  3. **Finding 1 is confirmed, not fixed.** `tabs_to_start` measures **live 3 · empty 6 · error 5**: the
     populated-catalog walk does not begin at the document start. The mechanism is NOT in the code — cmdk
     1.1.1 has no `autofocus` of any spelling, both its `.focus()` calls are guarded on focus already being
     inside cmdk, and no `src/` file passes `autoFocus` or focuses at mount.
- **Expected amendments (from plan):**
  - `a11y-plan.md` §3 / §11 (`:540`) — **carried.** Motivating fact: the picker's filter-miss string is
    `No scenarios match.` (`ScenarioPicker.tsx:29`, row S0-16); `No scenarios found.` is the empty-CATALOG
    prose (`App.tsx:245`, row E0-01). Two distinct states, two distinct rows; the doc attributes the catalog
    string to the picker/empty-state pair.
  - `design-system.md` §Component Patterns 3 Empty — **carried**, same fact.
  - `layout-templates.md` §Component — Primary content block 1 States (`:138`) — **carried**, same fact.
  - `test-plan.md` §6 — **not carried.** The condition was "if step 7 changes which terminal stage a
    last-scenario Stop reports". It does: a last-scenario Stop now reports `Aborted` instead of `Done`. The
    fact is stated here; §6's desktop-webview row describes the `sr` subject's walk, which is unchanged in
    shape, so the detector decides whether the row states the retired stage.
  - `.claude/rules/a11y.md:34` — **not an amendment entry** (names no spec master); recorded in the plan as a
    derived-tier site of the same string error. Routed to P3 curation by operator directive.
- **Coverage of new surfaces:**
  - `Titlebar` count live region (`aria-live="polite"` + `aria-label`) → validation n/a · instrumentation n/a ·
    PII n/a · tests `e2e (routine --e2e, 10 passing)` + `sr* leg (corroborated in 4 row windows)` ·
    a11y `WCAG✓ SC 4.1.2 + 4.1.3` · tokens `design-token✓` (no new literal; the three
    `titlebar__count--{state}` tints unchanged)
  - `Titlebar` phase-line politeness (`assertive` only entering `hold`) → validation n/a · instrumentation n/a ·
    PII n/a · tests `sr* leg (S2-07, S3-04 measured)` · a11y `WCAG✓ SC 4.1.3` · tokens n/a
  - `ScenarioPicker` `FilterMiss` region → validation n/a · instrumentation n/a · PII n/a ·
    tests `sr* leg (S0-16 announced-as-expected)` + `routine --e2e axe` · a11y `WCAG✓ SC 4.1.3` ·
    tokens `design-token✓` (reuses `.picker__empty`)
  - `ScenarioPicker` option `aria-label` carrying selection → validation n/a · instrumentation n/a · PII n/a ·
    tests `sr* leg (S0-06, S1-03)` · a11y `WCAG✓ SC 4.1.2 + 1.4.1` · tokens n/a
  - `CoverageMatrix` / `RunReport` scroll regions (`role="group"` dropped, name on the `<table>`) →
    validation n/a · instrumentation n/a · PII n/a · tests `sr* leg (S0-09, E0-05, E0-06)` +
    `routine --e2e axe` · a11y `WCAG✓ SC 1.3.1` · tokens n/a
  - `App.tsx` alert regions (mounted empty at first paint) → validation n/a · instrumentation n/a ·
    PII `redacted✓` (prose unchanged; R0-01 asserts no host path) · tests `sr* leg (R0-01)` ·
    a11y `WCAG✓ SC 4.1.3` · tokens `design-token✓` (`--status-fail` unchanged)
  - `drive_run` post-loop abort poll → validation n/a · instrumentation `log✓` (`info` with `run_id` + `count`,
    allowlisted fields, obs-plan §6) · PII n/a · tests `unit (drive_run_reports_an_abort_raised_during_the_last_scenario)`
    + `sr* leg (S3-05, loop-head arm)` · a11y n/a · tokens n/a
  - `tabs_to_start` in the SR evidence record → validation `bounded✓` (`typeof === 'number'`, else null) ·
    instrumentation n/a · PII `redacted✓` (inside `writeNvdaPass`'s scrub path; record host-path clean on all
    six anchors) · tests `sr* leg (all three subjects emitted it)` · a11y n/a · tokens n/a

## Deviations from intent

1. **Step 5 implemented differently than the plan's wording implies, and widened scope.** The plan said
   "focusing the scroll container announces the region, not its contents". Shipped: `role="group"` dropped and
   the name moved to the `<table>`. That broke four `expectActive('Coverage rows' / 'Run report rows')`
   assertions in `screen-reader.e2e.ts`, a file outside the plan's modify-set. **Operator authorised the
   widening** (post-implement directive item 2); the assertions moved to a new `expectActiveScrollRegion`
   helper that asserts focus landed on the container AND the container's table carries the name — stricter
   than the name-only check, not a relaxation.
2. **`role="status"` → `aria-live="polite"` on the two new live regions.** a11y-plan §4 permits either. Taken
   because the a11y suite's operator-checklist spec uses a bare `[role="status"]` as its "a hold is raised"
   subject-absent guard; the mandated new regions satisfied it, the skip stopped firing, and the spec asserted
   checkboxes it then found none of. `aria-live` keeps `role="status"` reserved for the checklist roll-up.
   The guard remains too loose for the next `role="status"` anyone adds — routed to the *A11y CI gate* CARRY.
3. **Finding 1 not fixed** — surfaced as a spec↔reality gap rather than guessed at. Its evidence half
   (`tabs_to_start`) shipped and measured the defect. See Spec claims disproved #3.
4. **Finding 4 not discharged** — see Insufficient fixes.
5. **An over-claim of mine was corrected mid-run.** My step-10 spec edit wrote R0-01's stimulus as "needs no
   reload" while the leg still reloads. Both `nvda-pass-spec.md` and `rows.ts` were corrected to state what is
   actually driven.
6. **The plan's deferral of findings 3 and 8 was withdrawn** by operator directive once Pulse was live; both
   were graded.

## Decisions & corrections

- **Operator, P4 fork (pre-implement):** fix all eight, defer findings 3+8's grading (later withdrawn); CARRY
  2b (OS-level key injection) out of scope, pin retained.
- **Operator, plan review:** the non-priming probe must precede the legs in the Test Commands block (the block
  executes top-down) and must use the harness's own firing form
  (`cargo run -q -p conductor-cli --bin conductor -- preconditions`), not a `target/debug` exe the block does
  not build.
- **Operator, correction of my citation:** I justified the findings-3/8 deferral in the P4 fork with a
  precedent — "the prior SR chunk deferred its hold assertions when Pulse was down". Measured **false**: the
  2026-09-02 pass ran against a LIVE Pulse (S2-07/S3-04/S3-05 all `announced-as-expected`). The handoff clause
  I inherited it from refers to the `a11y:driven` arm, not this leg. The deferral rests only on the probe's
  exit 1 and `execute_scenario`'s `!pf.ready` early return; the precedent is barred from the record by a
  binding note in the plan.
- **Measured this chunk:** the defect was never "the client announces an abort" but "the backend contradicts
  it". Deleting the client announcement removes feedback for the rest of the running scenario; the correct
  shape is both halves.
- **Measured this chunk:** a persistently-mounted live region is what makes an announcement fire — a region
  that mounts together with its text announces nothing. Applied at both the alert sites and the filter-miss.
- **Measured this chunk:** cmdk's `Command.Empty` spreads props BEFORE setting `role="presentation"`, so the
  role cannot be overridden by a prop; and a region nested in `Command.List` (a `listbox`) is an
  `aria-required-children` violation.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

| Criterion | Verdict |
|---|---|
| (a11y) Tab reaches titlebar controls before the picker, from `activeElement` readback | **UNMET — finding 1 confirmed unfixed.** Measured via `tabs_to_start` (live 3 · empty 6 · error 5) rather than the readback the criterion names. Unlinked to any matrix cap → routed as a P2-visible disproved claim + an operator CARRY. |
| (a11y) Focusing either scroll region announces the region, not the table, at both sites | **MET** — S0-09/E0-05/E0-06; heard "…table with 83 rows and 4 columns" + header row, not 83 rows. |
| (a11y) Filter-miss prose announced; titlebar count exposes an accessible name | **MET** — S0-16 announced-as-expected and left `findings`; count named (`Scenario count: no run yet` / `Scenarios completed: N`). |
| (a11y) HOLD flip still assertive, non-hold flips do not cancel the restore | **MET** — S2-07 and S3-04 both speak the restore. |
| (a11y) `sr-empty` + `sr-error` re-run non-interactively, graded from this session's log | **MET** — 06:43:58Z / 06:44:13Z, NVDA attached, both passing. |
| (tests) `cargo nextest -p conductor-run` AND `cargo test -p conductor-run` green with the last-scenario abort at the unit tier | **MET** — 173/173 under both runners; the new test named in the run. |
| (tests) routine `--e2e` green after the markup changes | **MET** — 10 passing / 2 subject-absent skips. |
| (obs) envelope keeps 11 fields + closed enums; abort logs `info` with `run_id`; zero unlogged panics | **MET** — no envelope field or enum touched; `tracing::info!(run_id, count, …)` on the abort arm. |
| (security) regenerated `nvda-pass.json` host-path clean incl. `tabs_to_start`; `heard` bounded; closed sets | **MET** — 0 hits on all six anchors (word-anchored drive letter, `%APPDATA%`, `/Users/`, `/home/`, `.cargo`, `.rustup`). |
| (security) `cargo audit` reproduces the pinned signature as the 49th; `cargo deny` exit 0 | **MET** — audit exit 1 / `duplicate advisory ID: RUSTSEC-2026-0244`; deny exit 0. |
| (design) token-bound edits, no new hex/px/ms literal, no color-only state | **MET** — no literal added; `--status-fail` / count tints reused by name. |
| (arch) changes stay in `ui/` + `conductor-run`, no new cross-seam edge, no new env handle | **MET** — no manifest edit; no `CONDUCTOR_*` handle added (the fifth `sr` handle is a pre-existing shipped handle newly RECORDED in the firing form, not introduced). |
| (deferred, recorded not claimed) findings 3/8 hold rows + finding 7 browse rows | **SUPERSEDED for 3/8** (graded live). Finding 7's rows stay `not-run-here` findings per test-plan §1. |

**Gates green** (commands run): `npm run build` · `cargo build --release -p conductor-tauri --features tauri/custom-protocol` ·
`tsc --noEmit` · `tsc -p test/tsconfig.json` · `cargo nextest run -p conductor-run` · `cargo test -p conductor-run` ·
`cargo nextest run --workspace --profile ci` (**825/825**) · `cargo test --workspace --doc` ·
`cargo clippy --workspace --all-targets -- -D warnings` · `npm audit --omit=dev` (0 production vulns) ·
`agent-run.ps1 run --e2e` (10 passing / 2 skipped) · `a11y:sr-empty` · `a11y:sr-error` · `a11y:sr` (all passing,
live Pulse) · `cargo audit` (**expected RED**, pinned signature, 49th re-check) · `cargo deny check advisories bans licenses sources` (exit 0).

**Smoke:** the boot-path and a UI surface both changed; the headful self-verify ran as a P2 gate (tauri-driver
launched the release binary and drove the real WebView2 window, 152.0.4191.53). Additionally the three `sr*`
suites drove the same window with NVDA attached against a live Pulse.

**Process hygiene** (re-measured against the host process list at wrap):

| Process | Started by | Final state |
|---|---|---|
| `tauri-driver` · `msedgedriver` · `conductor-tauri` | this chunk's `--e2e` (×3) and `sr*` (×4) runs | **terminated** — absent from the process list |
| `nvda` | the three `sr*` suites | **terminated** — absent (wdio `onComplete` `-q`) |
| `pulse-app` | **the operator**, 06:06Z | **left running: the light gate re-runs the `a11y:sr*` commands against it — the overseer stops it after the commit** |
| `msedgewebview2` ×6 | **not this session** — started 18:06, ~11 h before these runs | left running; pre-existing, not ours to stop |
