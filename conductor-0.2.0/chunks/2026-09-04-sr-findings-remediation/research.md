# Codebase Research — 2026-09-04-sr-findings-remediation

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11 · **Code-graph queries:** 1 (rust plane)
- **Harness rules consulted:** `.claude/rules/a11y.md` (full, 40 lines) · `.claude/rules/verification-harness.md`
  (structural in-full read: seeded body `:9-37` read directly; `## Session Additions` `:39-59` indexed by
  per-entry introducer, with the two SR/GUI-leg entries `:57` and `:59` read in full — the file is 50 KB
  over 59 lines, so an unbounded read overflows) · `.claude/rules/host-win32.md` (unconditional)

## Files inspected
- `crates/conductor-run/src/lib.rs` (`:930-1010`, `:510` region) — `drive_run`'s abort poll and
  `execute_scenario`'s readiness gate; the two sites findings 8 and the live-subject reachability turn on.
- `crates/conductor-tauri/src/commands.rs` (`:300-330`) — `run_thread`, the ONE production `drive_run` caller.
- `crates/conductor-tauri/ui/src/App.tsx` (`:42-75`, `:186-215`, `:240-305`) — `STATE_FOR_STAGE`, the Stop
  handler, the four `role="alert"` sites and the two empty-state strings.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full, 55) — the assertive live region and the
  unlabelled count.
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` (full, 61) — `Command.Empty`, `aria-current`,
  the ` · selected` meta text.
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx` (full, 77) — the focus-restore contract.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (`:50-70`, `:98`) ·
  `RunReport.tsx` (`:30-55`) — the two `tabIndex={0} role="group"` scroll regions.
- `crates/conductor-tauri/ui/wdio.conf.ts` (`:64-137`, `:229-300`) — the three `sr*` suites, their per-suite
  runs/scenarios dirs, the ONE spawn site.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (`:70-115`, `:304+`) — the `@foreground` stamp and
  the live subject's Start/hold walk.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (`:183-230`, `:436-450`) — the
  `@foreground` parse and the `writeNvdaPass` emit.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` (`:66-70`, `:120`, `:236`) — S0-16's definition and
  the two empty-prose rows.
- `crates/conductor-tauri/ui/node_modules/cmdk/dist/index.mjs` — the `Command.Empty` role (verified, not assumed).
- `conductor-0.2.0/chunks/2026-09-02-screen-reader-manual-spec/evidence/nvda-pass.json` — 51 rows, per-row
  `subject` / `class` / `arm` / `outcome` / `review_grade`.

## Graph impact (rust plane, `db_state: fresh`, trace `rows: 6`, no `probe_hits` warning)
- **`drive_run`** — 6 call sites: **one production** (`conductor-tauri commands/run_thread()` @
  `crates/conductor-tauri/src/commands.rs:315`) and five in-crate tests
  (`lib.rs:1384`, `:1436`, `:1466`, plus two more in the same module). A behaviour change to the abort arm has a
  single production caller to thread and an existing test neighbourhood
  (`drive_run_honors_abort_before_the_first_scenario` @ `lib.rs:1464`) to extend — that test pins the
  BEFORE-first-scenario arm, which is exactly the arm that already works.

## Patterns detected
- **The abort poll is top-of-loop** (`crates/conductor-run/src/lib.rs:975`): `for scenario in scenarios { if
  should_abort() { persist; emit(Aborted); return } … }`. A Stop during the LAST scenario finds no next
  iteration, so the loop falls through to the terminal `Blocked`/`Done` emit at `:986-993`. The doc comment at
  `:947` already states the contract ("polled between scenarios") — the behaviour is as documented, and it is
  the DOCUMENTED behaviour that produces finding 8.
- **The client half is optimistic** (`App.tsx:202`): `stop()` awaits `invoke('stop_run')` then
  `setRunState('aborted')` unconditionally; the backend's terminal `done` stage then maps through
  `STATE_FOR_STAGE` (`App.tsx:67-71`, `done: 'idle'`) and overwrites it. Finding 8 is therefore a genuine
  two-sided divergence, confirmed on both sides, not a rendering artefact.
- **One live region serves four states** (`Titlebar.tsx:22-27`): `aria-live="assertive"` sits on the phase-label
  span, so EVERY `idle`/`live`/`hold`/`aborted` flip is assertive — not just the HOLD flip that
  design-system §Component Patterns 1 mandates be assertive. This is the mechanism behind finding 3.
- **The focus-restore contract is already SHIPPED** (`OperatorPauseDialog.tsx:43-47`): `onCloseAutoFocus`
  calls `restoreFocusTo?.()`, `preventDefault()`s and focuses the target; `App.tsx:180` captures the invoker
  when the hold arrives. So finding 3 is NOT a missing restoration — the restore works and the DOM holds; what
  the operator heard was the assertive flip cancelling its ANNOUNCEMENT.
- **`Command.Empty`'s role is the library's, not ours** (`cmdk` 1.1.1, `dist/index.mjs`): the Empty forwardRef
  renders `createElement(D.div, {…, "cmdk-empty":"", role:"presentation"})`. Finding 6 cannot be fixed by
  editing our JSX text alone — the role must be overridden at the call site (or the node replaced).
- **Both scroll regions share one shape** (`CoverageMatrix.tsx:57`, `RunReport.tsx:45`):
  `<div tabIndex={0} role="group" aria-label="…">` wrapping a `<table>`. A focusable `group` computes the whole
  table as its accessible content, which is what NVDA reads on focus — finding 2, identical at both sites.
- **`tabsToStart` is captured and then dropped**: written into the actions ndjson at
  `screen-reader.e2e.ts:110`, parsed into `timeline.foreground` (`parse-nvda-log.ts:193`, `:228`), but
  `writeNvdaPass` emits only `activated` / `nvda_named_window` / `initial_focus` (`:439-444`). It appears
  nowhere in the committed `nvda-pass.json`.

## Conventions to follow
- **Single write point for the committed record**: `writeNvdaPass` (`parse-nvda-log.ts:~436`) — extend it for a
  new field rather than emitting beside it, so the leg's hygiene scrub and closed-set gates still cover it.
- **Per-suite isolation at ONE spawn site** (`wdio.conf.ts:272-299`): `sr` → `runs/sr-leg/runs`, `sr-empty` →
  the routine fixture dir + `runs/sr-leg/empty` scenarios, `sr-error` → `runs/sr-leg/runs` +
  `runs/sr-leg/bad`; the subject is written to `runs/sr-leg/subject.txt`.
- **Re-activate the window after anything that spawns a process** (`verification-harness.md:59` item 4) — a
  foreground steal detaches NVDA silently.
- **Never `tabindex` > 0; selection distinct from focus** (`--border-emphasis` vs `--color-focus`) —
  `.claude/rules/a11y.md` §Keyboard / §Visual.

## New files to create
- (none anticipated — every fix lands in an existing file)

## Files to modify
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — scope `aria-live` to the HOLD flip; give the count
  an accessible name (findings 3, 7).
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` — override `Command.Empty`'s
  `role="presentation"`; carry the selected state in the accessible name (findings 5, 6).
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` · `RunReport.tsx` — the scroll-region role so
  focus does not read the whole table (finding 2, both sites).
- `crates/conductor-tauri/ui/src/App.tsx` — the load-time `role="alert"` sites (`:240`, `:262`, `:279`, `:300`)
  and the Stop-handler / `STATE_FOR_STAGE` reconciliation (findings 4, 8-client).
- `crates/conductor-run/src/lib.rs` — `drive_run`'s abort arm (finding 8-backend) + its unit-test neighbourhood
  at `:1464`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` — surface `tabsToStart` through
  `writeNvdaPass` (finding 1's evidence path).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` +
  `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — tighten the `expected`/`tokens` of the
  rows whose behaviour changes (the nine currently graded `announced-as-expected`).
- **Caller threading:** `drive_run`'s signature is unchanged by the anticipated fix (the abort arm is internal),
  so the single production caller `commands.rs:315` is a read-only confirmation, not an edit — UNLESS the fix
  changes the emitted stage vocabulary, in which case `App.tsx:67-71` (`STATE_FOR_STAGE`) is the frontend half
  and is already listed.

## Scope premise closure
- `[inferred]` "nine of thirteen cited rows are graded `announced-as-expected`" — **VERIFIED** (recomputed from
  `nvda-pass.json`: S0-09, E0-05, E0-06, S2-07, S3-04, R0-01, S0-06, S1-03, S3-05).
- `[inferred]` "Finding 7 sits on the browse class" — **VERIFIED** (S0-12, S1-02 both `class: browse`,
  `outcome: not-run-here`).
- `[inferred]` "**Not a live-Pulse chunk**" — **FALSIFIED**
  `[premise-corrected: the sr suite's 'live' subject presses Start (S1-01) and waits for the in-run preflight
  canary hold (S2-01, screen-reader.e2e.ts:304+); execute_scenario returns a Blocked record at !pf.ready
  BEFORE its hold site (lib.rs:510 body), so with no ready preflight the hold never fires and every S2-*/S3-*
  row is unreachable. conductor preconditions measured all three subjects unmet at 2026-09-04T02:22Z (exit 1:
  nothing on :4317, andromeda-pulse-mcp not on PATH, all three ANDROMEDA_PULSE_* handles undeclared).]`
- CARRY 2a's "33 of 51 rows `arm: agent`" — **VERIFIED** (33 `announced-as-expected`; all 51 carry
  `arm: agent`). The npm version/maintenance half is still unverified (no registry read).
- CARRY 2b's fourteen row ids — **VERIFIED** (all present, all `arm: agent` / `not-run-here`).

## Corrections to the P2 extracts (carried into P4 as overriding)
- **The picker's filter-miss string is `No scenarios match.`** (`ScenarioPicker.tsx:29`), NOT
  `No scenarios found.`. The design and layouts extracts — and `.claude/rules/a11y.md` §Status messages — name
  `No scenarios found.` as "the picker/matrix empty"; that string is `App.tsx:245`, the empty-CATALOG prose
  (row E0-01, a different subject). Two distinct states, two distinct rows (S0-16 vs E0-01). Any fix or spec
  tightening for finding 6 must use the shipped `No scenarios match.`
- **Finding 3 is not a missing focus-restore.** The `restoreFocusTo` / `onCloseAutoFocus` contract the layouts
  and a11y extracts call for is already implemented (`OperatorPauseDialog.tsx:40-46`); the defect is the
  competing assertive announcement, so the fix belongs in `Titlebar.tsx`, not the dialog.

## Open questions
- **Which findings can be graded at all without a live Pulse?** Measured mapping: findings 3 (S2-07, S3-04) and
  8 (S3-05, T-01) sit on hold/post-hold rows that are unreachable while preflight is not ready; finding 4
  (R0-01) is the `error` subject and finding 2 has two `empty`-subject rows (E0-05, E0-06). Findings 1, 5, 6, 7
  sit on `live`-subject S0-*/S1-* rows that precede the hold. → blocks: **plan-decision** (P4 must decide
  whether the chunk's leg is deferred, split, or run against an operator-started Pulse).
- **Does CARRY 2b fall inside this chunk?** The P1 boundary question stands; research adds that finding 7's two
  rows are browse-class, so without 2b finding 7 is fixable but not gradeable on the agent arm. → blocks:
  **plan-decision**.
- **Does overriding `Command.Empty`'s role require replacing the node?** cmdk sets `role="presentation"`
  internally; whether a `role` prop on `<Command.Empty>` wins over the library's own attribute order is a
  one-line implementation detail. → blocks: **implementation-scope**.
