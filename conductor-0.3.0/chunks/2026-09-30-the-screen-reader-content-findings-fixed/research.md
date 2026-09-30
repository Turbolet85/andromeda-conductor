# Codebase Research — 2026-09-30-the-screen-reader-content-findings-fixed

## Scope
- **Depth:** deep on the webview's DOM and the `sr*` leg; minimal on Rust (the PREREQ's gates only) · **Reads:** 16 · **Globs/Greps:** 12
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: a structural read (71 lines; index `grep -nE '^#|^- '`). Read in full: the SR/webview entries `:58`, `:59`, `:61`, `:62`, `:63`, `:68`, `:70`, `:71`. The rest are Rust-harness and read-back entries, not in play. Its `paths:` do not cover `ui/test/`, so it is read here, not auto-loaded (`:58`'s own retrieval caveat).
  - `.claude/rules/a11y.md` and `.claude/rules/frontend.md`: auto-loaded in full, Session Additions included.
- **Platform issues consulted:** none. No CI verdict was folded (`d7da5d0` was `in progress` at Setup), and there is no runner-only bullet.

## Files inspected
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — the coverage row DOM:
  - `tr.cov__row` carries `tabIndex` and `aria-current` only (`:113-123`). There is no `aria-label` or `aria-labelledby`.
  - Its cells, in order: `td.cov__pid` (`:124`), `td.cov__cap` (`:125-128`), `td.cov__mode` (`:129-137`), `td.cov__status` (`:138-144`). The status cell holds `StatusLamp`, or `span.cov__unrun` "Not yet run" (`:142`).
  - `tbody` handles ArrowUp/ArrowDown/Home/End only; any other key returns without action (`:60-83`).
- `crates/conductor-tauri/ui/src/components/RunReport.tsx` (full) — `header.report__summary` renders `{n} scenarios · run {run_id}` (`:41-44`) BEFORE `div.report__scroll[tabindex=0]` (`:47`). The table has six header cells (`:51-68`), and the status cell is a `StatusLamp` (`:77`).
- `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` (full) — the glyph is `aria-hidden` (`:14`), and `span.lamp__label` renders the label TEXT (`:17`). "Blocked" and "Manual" are therefore in the DOM as text.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` (full) — the phase line is a `span.type-heading.titlebar__label[aria-live]` (`:32-38`), not a heading element. The count is a `span[aria-live][aria-label]` with no role (`:39-49`).
- `crates/conductor-tauri/ui/src/components/Titlebar.css` (`:15-17`) — `.titlebar__label` sets colour only.
- `crates/conductor-tauri/ui/src/App.tsx` (full):
  - The DOM holds exactly three `h2` (`:292`, `:347`, `:371`) and no `h1` anywhere.
  - Landmarks: `Titlebar`'s `header` (banner) and one `main` (`:281`). There is no `footer` or `contentinfo` element (whole-file read).
  - `No scenarios found.` is at `:322-324` and `No run yet` at `:386-388`.
- `crates/conductor-tauri/ui/src/components/RunControls.tsx` (full) — Stop is natively `disabled` when idle (`:36`). The hint line `p.run-controls__hint` sits between the buttons and the matrix (`:43-45`).
- `crates/conductor-tauri/ui/src/components/OperatorChecklist.tsx` (full) — the checklist rows. The roll-up `[role="status"]` lives in the dialog, and the leg's own DOM assertion reads it (`screen-reader.e2e.ts:539`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` (full) — the 51 rows. Each row id's tokens, class and notRun are listed at `:36-332`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (full) — the leg:
  - OS keys go through `osKey` (`:380-392`). `browseKey` refuses only from the combobox (`:402-407`).
  - The live walk is `:418-618` and the empty walk `:631-665`.
  - T-01 is stamped not-run (`:617`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (grep + `:275-318`):
  - A browse row with no heard text and no OS key is graded `not-run-here`, arm operator (`:304-305`).
  - The tokens are matched over the whole row window (`:309`), so several keys inside one window grade as one row.
  - `inputPathOf` gives `os` when only OS keys fell in the window (`:275-279`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1` (`:12`) — `ValidateSet('Tab','ShiftTab','h','d','ArrowDown')`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (headers + the 18 rows' table lines) — mirrors `rows.ts`, and the parser refuses an id missing from it (`rows.ts:2-3`).
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (`:50-75`, `:315-345`) and `operator-hold.e2e.ts` (`:72-100`) — every coverage-row and phase-line read keys on a CLASS (`cov__row`, `cov__pid`, `cov__status`, `lamp__label`, `titlebar__label`), never on an accessible name or a heading tag.
- `crates/conductor-tauri/ui/test/a11y/claim-ownership.ts` (grep `claim:`) — 11 claims, all a11y-plan §5 keyboard/focus claims. No row-name, heading or landmark claim.
- `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` (parsed) — three records:
  - `lamps-fixture-pass`: [P-019, P-041], Pass/Pass;
  - `lamps-fixture-blocked`: [P-019], Blocked/None;
  - `lamps-fixture-fail`: [P-030], Fail/Fail.

  So P-019 renders Blocked (worst-lamp-wins), and the report renders Pass, Blocked, Fail in that order.
- `contracts/pulse-capabilities.toml` (grep) — P-001…P-029 are contiguous in manifest order, and the matrix renders manifest order (`rows.ts:107`). So P-019 is row index 18 and P-025 is row index 24.
- `conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/nvda-pass.json` (the 18 rows, `heard`, and `operator_review`) — read through a scratchpad extractor, not whole.
- `.andromeda/a11y-plan.md` (grep `h1|contentinfo`) — the master itself states:
  - `:320`: `contentinfo` is the footer strip, "designed, NOT in the shipped DOM … route-owned";
  - `:323`: a "single `h1`-equivalent on the phase-line Heading role per state";
  - `:548`: "the desktop-webview frameless window MUST expose `main` … + `contentinfo`".

  This confirms the a11y extract's constraint against the master, not against the extract.

## Graph impact
- **CoverageMatrix / RunReport / StatusLamp / Titlebar / ROWS / rowsFor** — one `refs` query on the `ts` plane (trace `.andromeda/runs/2026-09-30T16-31-26-phase/tree-query-2026-09-30-the-screen-reader-content-findings-fixed.json`), 23 rows. Lines below are editor lines (graph line + 1):
  - `CoverageMatrix`: `App.tsx:366` and `Gallery.tsx:206`.
  - `RunReport`: `App.tsx:390` and `Gallery.tsx:213`.
  - `Titlebar`: `App.tsx:279`.
  - `StatusLamp`: `CoverageMatrix.tsx:140`, `RunReport.tsx:77` and `Gallery.tsx:157`.
  - `rowsFor`: `screen-reader.e2e.ts:35` and `parse-nvda-log.ts:382`.

  No prop signature changes in this chunk's design, so no caller threads anything. `Gallery.tsx` is a dev-only consumer with unchanged props.
- **Rust plane:** not queried. No Rust symbol is in the modify-set. The PREREQ runs gates over the Rust tree and changes none of it.

## Per-row DOM confirmation (the take-up directive: product fix vs re-token, reason recorded)
The mode evidence comes from the record's `heard` fields:
- **The focused coverage row is in focus mode.** E0-09 heard "row current" twice. S0-15's ArrowDown heard "row current". That is the app's roving ArrowDown moving focus (`CoverageMatrix.tsx:64-82`), not a virtual-cursor line.
- **A focused button or the report scroll div is in browse mode.** S3-06's ArrowDown from Start read the Stop line; S3-07 then read the hint line. E0-07/E0-08's ArrowDown from `div.report__scroll` walked into the report table cell by cell.
- [hypothesis] NVDA enters focus mode when focus lands on the focusable `tr`, and browse mode on a button or the scroll div. This is consistent with every heard line in the record; it is not measured directly.

| row | DOM carries the required content? | heard / why silent | disposition |
|---|---|---|---|
| E0-05, S0-09 | P-ID in `td.cov__pid`, but the row has NO author name (`CoverageMatrix.tsx:113-123`) | "row current" — an empty name | **product fix**: the row's accessible name carries its P-ID |
| E0-09 | yes (P-019 row, Blocked label) | one ArrowDown moves the roving focus to row index 1 (P-002), never index 18 | **product fix + re-token**: the name fix, plus an action that reaches P-019 (ArrowDown until P-019 is current) |
| S3-07 | yes (P-025 row + its lamp label) | the ArrowDown from Start read the hint line (browse mode) | **re-token**: reach the P-025 row (after the name fix, through the focused row) |
| S0-15 | yes (`span.cov__unrun` "Not yet run", `:142`) | "row current" — focus moved to row index 1 with an empty name | **product fix + re-token**: the row name carries its status, and the row is re-classed as the focus move it is |
| E0-07 | yes (`header.report__summary` "3 scenarios · run lamps-fixture", `RunReport.tsx:41-44`) | the virtual cursor started at the scroll div AFTER the header and moved forward into the table | **re-token**: reach the header by navigation from before it |
| E0-08 | yes ("Blocked" as text, `StatusLamp.tsx:17`) | one ArrowDown reached only "column 2 Scenario"; the Blocked cell is further on | **re-token**: continue until the Blocked cell |
| S3-06 | yes ("Manual" label text) | the ArrowDown from Start read the Stop line | **re-token**: reach the report's status cells |
| S0-13 | the three `h2` are present; NO `h1` (App.tsx, Titlebar.tsx) | `h` sent from the focused coverage row, which is in focus mode, so it goes to the app (`:76-77` ignores it) | **product fix** (an h1 per a11y-plan `:323`) **+ re-token** (send `h` from a browse-mode position, and re-derive the token) |
| S0-14 | banner + main present; NO `contentinfo` | as S0-13 (`d`) | **product fix or recorded reason** (a11y-plan `:548` requires it — a P4 fork) **+ re-token** |
| E0-01, S0-08, S0-10, S0-11, S0-12, S1-02, S2-06 | yes — each row's own DOM assertion passed on the 2026-09-30 leg (`screen-reader.e2e.ts:422`, `:425`, `:486`, `:496`, `:513`, `:539`, `:634`) | no OS key in the row's window (`parse-nvda-log.ts:304-305`) | **re-action**: OS-path browse keys inside each row's window. No product change is planned. [hypothesis] S0-12's `aria-label` sits on a role-less `span` (`Titlebar.tsx:39-47`), which ARIA 1.2 does not name, so browse mode may read the digits. The regrade measures this; any product change waits on it. |
| T-01 | yes (the phase line "Conductor · idle") | never driven: the live subject stops at aborted | **re-action**: a second, un-stopped run in the SAME live session after S3 (Start again, Proceed through both holds, Done). No new process or spawn form (see Patterns). |

## Patterns detected
- **Row grading spans the whole window** (`parse-nvda-log.ts:299-318`): the tokens are matched anywhere in the window's heard text. So a browse row may send several OS keys in one window, and its record's `input` stays `os`.
- **The leg reads the speech log while it runs** (`screen-reader.e2e.ts:48-54`, `:118-125`). A bounded "send ArrowDown until the row's token appears in the log since the stamp, cap N" loop keys on a signal and uses no sleep (test-plan §11 E2E ban scoped to in-test sync). The cap turns a miss into a graded finding.
- **Browse keys never go through the combobox** (`:402-407`). `h`, `d` and ArrowDown are already in the closed set (`send-keys.ps1:12`), so no key is added.
- **A second run inside one live session:** after S3-05 the app is idle-able and Start is enabled again (S3-02). The app's own sidecar spawn is rule (a), unchanged. No rule (b) form is added: `osKey` and `injectKeys` are the existing forms.
- **Routine and driven arms key on classes:** `cov__row`, `cov__pid` and `titlebar__label` (`accessibility.e2e.ts:61-64`, `:324`; `operator-hold.e2e.ts:79-97`). A row name or a heading tag change moves no selector. The Tab order is unchanged, because a heading and a footer are not focusable.

## Conventions to follow
- **Name from the visible text, never a composed `aria-label`:** `aria-label` is reserved for icon-only controls (a11y-plan §11 ARIA; `rules/a11y.md` Semantic HTML first). `aria-labelledby` pointing at the row's own cells names the row from text that is already rendered.
- **The one lamp truth:** status text comes from `LAMP_META` through `StatusLamp` (`frontend.md` 2026-06-26). The row name references that cell, and never re-spells a label.
- **Tokens by name** on any new visible node (`design-system` §Tokens; `Titlebar.css` binds `var(--text-secondary)`).
- **The `sr*` suites never rebuild** (`verification-harness.md:62`): an app change needs a `--e2e` run before any `sr*` regrade reads it.
- **`nvda-pass.json` is cumulative per subject** (`:63`): the regrade record is complete only when all three subjects ran after the app change, each with its own `recorded_at`.
- **Live `sr` firing form** (`:59`, `:61`): `CONDUCTOR_NVDA`, `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`, the three `ANDROMEDA_PULSE_*`, and the Pulse release dir on `PATH`. It needs a quiet desktop (`:59` extended 2026-09-30), and it ends with a census naming who stops each survivor (`:58`).
- **A harness defect gets one re-fire on a new slot; a key-path fix is proven by a short probe first** (`:71`).

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-content-findings-fixed/evidence/nvda-pass.json` — the regrade record, copied from `runs/sr-leg/` after the three subjects ran on the changed bundle, with the operator review transcribed
- `crates/conductor-tauri/ui/src/components/Footer.tsx` — only if P4 takes the ship-a-footer branch: the `contentinfo` status strip
- `crates/conductor-tauri/ui/src/components/Footer.css` — only if P4 takes the ship-a-footer branch: its token-bound styles

## Files to modify
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — the row's accessible name through `aria-labelledby` over its own P-ID and Status cells (per-row cell ids)
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — the phase line becomes the single `h1` (a11y-plan `:323`)
- `crates/conductor-tauri/ui/src/components/Titlebar.css` — the `h1` user-agent margin and size reset, so the visible anatomy does not move
- `crates/conductor-tauri/ui/src/App.tsx` — mounts the footer (ship branch only; untouched on the recorded-reason branch)
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` — re-tokened rows: E0-07, E0-08, E0-09, S0-13, S0-14, S0-15, S3-06, S3-07, and T-01's `notRun` removed
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — the OS-path browse actions for the 7 rows, the reaching actions for the re-tokened rows, and T-01's second run
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — the mirror of every changed row, and the findings ledger
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — routine-arm DOM assertions for the row name, the single h1 and the landmark set (the CI-gated baseline beside the supplemental SR pass)

## Open questions
- `contentinfo`: ship a footer strip, or record a reason for none? The master requires it (a11y-plan `:548`). Layouts' footer anatomy names `seed <n>`, and no existing command carries the seed (arch forbids a new command). So the ship branch cannot build the full anatomy, and each branch owes a different master amendment. → blocks: plan-decision (P4)
- Does "the next regrade grades all 51 rows" require T-01 to be HEARD, or does a graded finding satisfy it? Research shows T-01 is drivable with no new crossing, which leans towards heard. → blocks: plan-decision (P4)
