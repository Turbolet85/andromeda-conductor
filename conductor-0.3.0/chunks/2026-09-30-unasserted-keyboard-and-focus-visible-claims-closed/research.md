# Codebase Research — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

## Scope
- **Depth:** deep (webview surface + routine-arm harness) · **Reads:** 13 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full by structural extraction (lines
  1-46 · 47-57 · 58-68, offset-bounded, 29 Session Additions indexed and read). Additions applied: 2026-09-02
  (census before and after, pattern covering every spawnable family incl. `msedgewebview2`; the `--e2e` exit code
  cannot tell pass from skip; the verdict is mechanized), 2026-09-07 (only `--e2e` rebuilds the bundle),
  2026-09-10 (`expect` atoms from printed output), 2026-09-12 (a PowerShell capture is UTF-16 and greps empty), plus
  the auto-loaded `a11y.md` 2026-09-17 extension (read both driver and runtime versions before firing `--e2e`).
  `testing.md` is not a harness rule for this leg (its `paths:` cover no `scripts/`), and its a11y facts ride
  `a11y.md`.
- **Platform issues consulted:** none. No runner-only bullet was folded (the one CI row at take-up was in progress,
  not red), and no CI-reading entry is planned outside the operator leg.

## Files inspected
- `crates/conductor-tauri/ui/src/App.tsx` (full) — one `CoverageMatrix` in every run state (`:344-346`). Lamps are
  projected from the report (`:172`). `start()` invokes `start_run` (`:202-229`), and `holdPrompt` is set only by the
  hold `Channel` (`:211-220`). The dialog is rendered with `restoreFocusTo` (`:374-387`). No key handler.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — a plain `<table>` in a `tabIndex={0}`
  scroll `<div>` (`:60`). Every row is rendered (`:79`) with no virtualization, no row focus, no selection state.
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` (full) — the same shape (`:47-86`). The seeded fixture
  renders 3 rows.
- `crates/conductor-tauri/ui/src/components/RunControls.tsx` (full) — native Start (`aria-disabled`, `:21-30`) and
  Stop (native `disabled` when idle, `:31-37`) buttons.
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx` (full) — Radix `AlertDialog` with
  Cancel = Abort and Action = Proceed (`:59-71`), plus the explicit `onCloseAutoFocus` restore (`:43-48`).
- `crates/conductor-tauri/ui/src/components/*.css` (focus-visible + transition greps) — the seven `:focus-visible`
  outline rules and the six `transition` declarations, none on `outline`.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (full) — the routine arm. `tabCycle`/`focusSnapshot`
  (`:196-266`), the Operable pair (`:465-503`) and the two expected-skip markers (`:510-525`).
- `crates/conductor-tauri/ui/test/a11y/claim-ownership.ts` (full) — 10 claims, three `unasserted`, no SC 2.4.7 row.
- `crates/conductor-tauri/ui/test/a11y/check-claim-ownership.ts` (full) — the grading rules (`:51-95`) and the
  printed last line (`:120`).
- `crates/conductor-tauri/ui/wdio.conf.ts` (grep) — `specs` (`:320`), seed (`:122-132`, `:365`), the fixture runs
  dir as `CONDUCTOR_RUNS_DIR` (`:373`) and classic WebDriver (`:340`).
- `scripts/agent-run.sh` (`:55-57`, `:136-146`, `:249-290`, `:295`) — `ensure_frontend`, `run --unit`, the
  `--e2e` verdict assertion and its printed line (`:290`), and `A11Y_EXPECTED_SKIPS=2` (`:253`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (grep) — Tab walks into the coverage and report regions
  (`:430`, `:591-595`).
- `scripts/code-graph-cookbook.md` (`:7-56`) — schema and the impact query form.

## Graph impact (ts plane; `tree-query-2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed.json`)
- **CoverageMatrix** — referenced at `src/App.tsx` (import + render) and `src/Gallery.tsx` (import + render; DEV-only
  gallery). A new prop or a changed prop shape reaches both.
- **RunReport · OperatorPauseDialog** — the same pair of consumers (`App.tsx`, `Gallery.tsx`).
- **RunControls** — `App.tsx` only.
- **tabCycle** — two callers (`accessibility.e2e.ts` Operable pair). **focusSnapshot** — `tabCycle` only.
  **FOCUSABLE_SELECTOR** — `focusSnapshot` only.
- **CLAIMS · NA_CLAIMS** — `check-claim-ownership.ts` only (7 + 2 refs).
- Graph lines are 0-indexed (the `App.tsx` render row reads 344 against grep's 345).

## Patterns detected
- **Roving selection on the picker** (`ScenarioPicker.tsx:51,71`; `ScenarioPicker.css:58-60`): cmdk supplies roving
  focus. The committed choice carries `aria-current='true'` + a `--border-emphasis` left edge + a text label, never
  tint alone. This is the house model for any row-selection state.
- **Focusable scroll region without a group role** (`CoverageMatrix.tsx:57-60`, `RunReport.tsx:45-47`): the region is
  a tab stop so it scrolls by keyboard, and the table carries the name, because a named focusable group made NVDA read
  83 rows as one utterance.
- **Element identity by index in the live focusable set** (`accessibility.e2e.ts:205-234`): never by name, because
  the two scroll regions both project to `DIV`.
- **Lap detection by the first repeated identity** (`accessibility.e2e.ts:236-266`): never a `BODY` sentinel.
- **The expected half from the DOM, the received half from the walk** (`accessibility.e2e.ts:467-478`): the
  anti-vacuity form frontend.md 2026-09-17 requires.
- **Ownership graded by title + `expect(` presence** (`check-claim-ownership.ts:43-67`): an `owned` row's
  `assertedBy` must be byte-equal to a single-quoted `it('…'` title in its owner spec.

## Conventions to follow
- **Selectors**: role / text / `aria-*` / `[class~="…"]` token matches, never xpath or hashed classes
  (`accessibility.e2e.ts:12`, `:61-64`).
- **Token reads**: resolve `--color-focus` from the running webview (`token()`, `accessibility.e2e.ts:149-154`) and
  compare through colorjs (`Color`), never a literal hex.
- **Findings rendered, never counted**: fold context into the asserted value; `expect` takes no message argument
  (frontend.md 2026-09-02).
- **Signal-based waits**: `browser.waitUntil` over a probe that catches throws (`accessibility.e2e.ts:117-136`),
  never `sleep`.
- **`aria-disabled` keeps Start focusable** (`RunControls.tsx:16-27`): a shortcut must not bypass the
  `canStart && !running` guard it enforces.

## Mechanism the plan will originate (stated as the equality to verify at /implement)
- **SC 2.4.7:** for each element the Tab walk reaches in the live focusable set, after that `browser.keys('Tab')`,
  `getComputedStyle(document.activeElement).outlineStyle` is not `none` and its `outlineColor` equals the resolved
  `--color-focus` (colorjs-normalized). The deciding control is the seven `:focus-visible` rules above. Whether
  WebView2 sets `:focus-visible` for a WebDriver-synthesized Tab is the half that decides the case: a hypothesis
  until the leg runs, so the plan carries a negative witness. One example: a control that is NOT focused reads
  outline `none`, so a ring on every element regardless of focus cannot pass.

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/evidence/` — the leg
  census, the PREREQ gate readings and the `--e2e` verdict capture.

## Files to modify
<!-- Closed to the BUILD branch at P4 (overseer, founder-delegated, 2026-09-30). Gallery.tsx left the list: the
     current-row state is component-local, so no consumer's props change. -->
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — three routine-arm `it()` blocks (the SC 2.4.7 ring,
  idle-with-report row navigation, and the key-map substitute gate).
- `crates/conductor-tauri/ui/test/a11y/claim-ownership.ts` — the SC 2.4.7 row added, the three rows moved from
  `unasserted`, and the `inPlace` field.
- `crates/conductor-tauri/ui/test/a11y/check-claim-ownership.ts` — enforces a carve-out's in-place statement.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — roving row focus + `aria-current`.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.css` — the row ring and the current-row
  `--border-emphasis` edge.
- `crates/conductor-tauri/ui/src/App.tsx` — the start/stop shortcut listener.
- `crates/conductor-tauri/ui/src/components/RunControls.tsx` — `aria-keyshortcuts` on Start/Stop + the hint line.
- `crates/conductor-tauri/ui/src/components/RunControls.css` — the hint line's token styling.
- `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx` — Ctrl+Enter Proceed + `aria-keyshortcuts`.
- `crates/conductor-tauri/ui/wdio.conf.ts` — the driven suite's trimmed two-scenario catalog.
- `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` — the one-live-run driven spec.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — S0-09 / E0-05 / E0-09 re-aimed at the focused coverage
  row (P5 condition).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` — the three rows' node, expectation and tokens.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — the same three rows restated.

## Open questions
<!-- All three resolved at P4 (overseer, founder-delegated): BUILD with one tab stop per region; Ctrl+Enter / Ctrl+. /
     Escape; the driven arm owns live row navigation + the shortcuts in ONE live leg, fired only in the overseer's slot.
     Kept below as asked. -->
- Build-then-assert vs re-state for the three unbuilt claims (matrix row navigation ×2, first-class shortcuts). →
  blocks: plan-decision. Re-state moves no code but amends a11y-plan §5/§4, layout-templates §Primary navigation /
  block 1 and design-system §Navigation Pattern / §Component Patterns #3/#5 at wrap. It also leaves `v3-03`
  claimable only if the re-stated claims stop being claims, which risks weakening. Build adds product behaviour: roving
  row focus, a selection state, a shortcut key set whose keys must avoid NVDA and OS shortcuts and the picker's
  type-ahead input.
- If shortcuts are built, how is Start asserted on the routine arm without a real run mutating the seeded subject
  (`App.tsx:202-229` → a `Blocked` record in `runs/e2e-fixture`)? And is Proceed/Abort owned by the driven arm (not
  CI-run, a carve-out the §11 Strategy line must state)? → blocks: plan-decision.
- One routine assertion over the shared `CoverageMatrix` in the seeded idle-with-report state: does it own both the
  run-console-live and idle-with-report row-navigation claims, or does run-console-live need a live subject (driven
  arm)? → blocks: plan-decision.
