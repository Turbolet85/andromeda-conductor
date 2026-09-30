# Scope — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

**Working entry (`working-route.md:63`):** Unasserted keyboard and focus-visible claims closed — coverage-matrix row
navigation, the first-class shortcuts and the SC 2.4.7 active-element ring given real assertions.

**Matrix target:** `v3-03` (*Keyboard and focus-order coverage has a stated owner*), unclaimed at take-up (the only id in
the pool). The entry's CARRY names it; P4 re-reads the pool through `matrix.py show --unclaimed`.

**Chunk base (W182):** `7ee2fea` (`7ee2feadac235678e0b2696d4a5202dcb2bb2886`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat 7ee2fea -- <f>`), because the operator pre-CI commit moves HEAD before
the wrap (founder directive, W182, as applied at `2026-09-30-mutation-gate-grades-every-tally-it-rests-on/scope.md:10-13`).

## Operator directives at take-up (2026-09-30)
- **The PREREQ runs first:** close the Rust gate deferral (workspace nextest + clippy) before the chunk's own work.
- **Driver/runtime coherence is checked before any `--e2e` leg.** MEASURED at take-up: `CONDUCTOR_MSEDGEDRIVER` reports
  `Microsoft Edge WebDriver 154.0.4258.37`, and the WebView2 runtime `pv` under the machine-wide (HKLM) EdgeUpdate client
  key is `154.0.4258.37`. They match. The leg re-reads both before it fires, because the Evergreen runtime moves between
  sessions under an unchanged handle (a11y.md, 2026-09-17 as extended 2026-09-30).
- **Diff-shaped probes name the chunk base (W182):** `7ee2fea`, above.
- **Overseer rulings carried to THIS chunk's wrap** (the wrap's directives, not phase or implement work):
  - the curation conflict "letters-only data-dir leaves vs `verification-harness.md` 2026-08-18
    `%TEMP%/pulse-legs/<ts>`" resolves as **REFINED**: keep the parent `pulse-legs`, make the leaf letters-only;
  - the proposed playbook rule "a relayed founder ruling that reverses a spec's cadence/scope claim → routine apply" is
    **NOT added**.

## PREREQ — close the Rust gate deferral
- `cargo nextest run --workspace --profile ci` and `cargo clippy --workspace --all-targets -- -D warnings` were deferred
  at `2026-09-30-mutation-gate-grades-every-tally-it-rests-on` on a zero-Rust-delta `defer` key (its `report.md:172-175`;
  `plan.md:313-321`). This chunk runs both, un-deferred, as gate entries. They are its first measured work, run against
  the chunk base before any change of this chunk's lands. After that, they run again over the chunk's own delta.
- The webview bundle must be built before any workspace compile of `conductor-tauri` (frontend.md, 2026-06-24:
  `generate_context!` reads `ui/dist` at compile time). VERIFIED at P3: `scripts/agent-run.sh:55-57`
  `ensure_frontend` runs `npm run build`, and `run --unit` (`:295`) is `ensure_frontend` then workspace nextest. So
  `bash scripts/agent-run.sh run --unit` is the harness form of the nextest half. Clippy is its own command after a
  frontend build.

## What this chunk builds (the four claims)
The four claims sit in a11y-plan §5, and their coordinates were re-verified at take-up: `:354` run-console-live
coverage-matrix row navigation; `:356` idle-with-report coverage-matrix row navigation plus `aria-selected`/`aria-current`
and `--border-emphasis` selection marking; `:370` the first-class start / stop / proceed / abort shortcuts; and SC 2.4.7's
visible `--color-focus` ring on the active element. SC 2.4.7 is named in §5's harness pattern (`:372`), not in a claim bullet.
1. **Each claim gets a real assertion in exactly one suite**, preferably the routine `--e2e` arm (the one CI runs), and
   its `claim-ownership.ts` row moves from `unasserted` to `owned`. `npm run a11y:ownership` then holds the owner to
   reality (the entry's CARRY: "the enumeration is the acceptance surface, not a second place to update").
2. **Closing all four unblocks `v3-03`**, which stayed pooled at `2026-09-17-keyboard-and-focus-order-coverage-ownership`
   because its acceptance quantifies over every §5 claim and these were unasserted (matrix `v3-03` notes, 2026-09-17).

## Premises found at take-up — closed at P3
- **"Unasserted" is "unbuilt" for three of the four — VERIFIED at P3.**
  - `crates/conductor-tauri/ui/src` at the chunk base carries **0** key handlers
    (`grep -rnE "onKeyDown|keydown|ArrowDown|ArrowUp|addEventListener\(['\"]key|accessKey|aria-keyshortcuts|useHotkey" src`
    → 0 hits, re-run at P3).
  - The coverage matrix (`CoverageMatrix.tsx:60-108`) and the run report (`RunReport.tsx:47-86`) are plain `<table>`s
    inside a `tabIndex={0}` scroll region. Keyboard SCROLLING exists; row focus and row selection do not. There is no
    virtualization: every manifest row renders (`rows.map`, `:79`), so §5's "off-screen virtual-scrolled rows" has no
    referent.
  - `aria-selected`/`aria-current` + `--border-emphasis` exist only on the picker (`ScenarioPicker.tsx:51,71`,
    `ScenarioPicker.css:59-60`).
  - Start/Stop are native `<button>`s (`RunControls.tsx:21-38`) and Proceed/Abort are Radix `AlertDialog.Action`/`Cancel`
    buttons (`OperatorPauseDialog.tsx:59-71`). Enter/Space activate them only once focused, and Escape resolves NoGo via
    `onOpenChange`. No shortcut exists.
  - **The chunk's real fork is therefore build-then-assert vs re-state the §5 claim to what ships.** It is P4's fork.
- **run-console-live and idle-with-report are the SAME component — VERIFIED at P3.** `App.tsx:344-346` renders one
  `CoverageMatrix` in every run state, with lamps projected from the run report (`App.tsx:172`). The routine arm's seeded
  subject IS the idle-with-report state: the fixture journal has 3 records
  (`grep -c . crates/conductor-run/tests/fixtures/lamps-journal.jsonl` → 3) and the matrix carries every manifest row.
  The `live` state is reachable only through `start_run`, which the routine arm has no Pulse for.
- **SC 2.4.7 is built and unasserted — VERIFIED at P3.**
  - Seven `:focus-visible { outline: 2px solid var(--color-focus) }` rules exist: `Titlebar.css:90`,
    `RunControls.css:24`, `ScenarioPicker.css:25`, `CoverageMatrix.css:23`, `RunReport.css:49`,
    `OperatorChecklist.css:34`, `OperatorPauseDialog.css:99`.
  - The routine arm's only ring-adjacent row is the token-pair CONTRAST row (`accessibility.e2e.ts:33`), re-derived at P3
    (`PAIRS` holds `--color-focus`/`--color-base` 3:1). It proves the ratio, not that a ring renders.
  - No transition animates `outline` anywhere (`grep -rn "transition" src/components/*.css` → 6 hits, none naming
    outline), so a computed-style read taken right after a key sees the final ring, not a mid-fade value.
  - [premise-corrected: design-system §Depth Strategy states the ring as a `0 0 0 2px` `box-shadow` and §Motion as a
    `--motion-micro` fade-in; the code draws an `outline` with no fade. A ring assertion keys on presence + colour, not on
    the drawing property. Reconciling spec to code is a wrap amendment candidate, not this chunk's code.]
- **The CARRY's "the committed enumeration already names all four" is FALSIFIED at take-up.**
  `claim-ownership.ts` carries THREE `unasserted` rows (`:52-56`, `:65-70`, `:101-106`) and no SC 2.4.7 row; a search of
  the file for `2.4.7` returns 0. So SC 2.4.7 enters the enumeration as a new row. That row is a move of the population,
  not just of the state, so the enumeration's `NA_CLAIMS` set logic and the ownership checker's own population must be
  re-read at P3.
- **Row label mismatch — VERIFIED as a fact, left to P4.** `claim-ownership.ts:53` carries `SC 2.1.1` and `:67`
  carries `SC 2.4.3`. The 2026-09-17 chunk authored both, and the checker (`check-claim-ownership.ts`) never reads `sc`.
  So the label is documentary, and a move re-labels only if the asserting spec proves a different criterion.
- **The ownership checker admits a new row without code change — VERIFIED at P3.** `check-claim-ownership.ts:51-76`
  grades each row by its state. The only SET assertion is `NA_CLAIMS` (`:85-95`), which an `owned` SC 2.4.7 row does not
  touch.
  - An `owned` row needs `assertedBy` to equal an `it('…'` title in its owner spec, whose body carries `expect(`
    (`:43-67`).
  - The title match is a literal `it('${title}'` search, so the title must be single-quoted and byte-equal.
- **Adding tab stops moves other suites.** The routine Operable pair walks the whole focusable set
  (`FOCUSABLE_SELECTOR`, `accessibility.e2e.ts:196-197`). Its roster is live-derived, so it adapts, but the "first two"
  literal (`:492-494`) stays. The operator-local SR leg walks Tab into the coverage region (`screen-reader.e2e.ts:430`,
  S0-09) and the report region (`:591-595`, E0-05/E0-06). Any row-focus design that ADDS tab stops shifts those rows. A
  roving-tabindex design (one stop per region) keeps the stop count.

## Decided at P4 (validation-1: intent-incomplete, amended here)
Overseer rulings, founder-delegated, 2026-09-30:
- **Build, then assert.** The three unbuilt claims are BUILT in the frontend (roving matrix row focus with one tab stop
  per region; the Ctrl+Enter / Ctrl+. / Escape key map) and then asserted. Building what a11y-plan §5 already designs is
  not new trajectory, and re-stating would be a deferral (founder: none).
- **Key map:** Ctrl+Enter starts and proceeds, Ctrl+. stops, Escape aborts. The design-system k9s single-letter register
  gets its amendment at wrap.
- **Ownership:** the routine arm (CI) owns SC 2.4.7 and idle-with-report row navigation. The DRIVEN arm owns
  run-console-live row navigation and the shortcuts, in ONE live run over a trimmed two-scenario catalog (two holds, one
  canary). This refines "preferably the routine arm" above for the two claims only a live run can show. Each
  carve-out's in-place CI gate is stated in the enumeration and enforced by its checker.
- **The live leg needs the overseer's slot.** The Pulse builder runs its own `:4317` legs in the same window, so
  /implement stops and asks before the probe and the leg.
- **The SR regrade is IN scope (P5 condition, overseer: founder allows no deferral).** Three SR rows (S0-09 on `sr`;
  E0-05 and E0-09 on `sr-empty`) assert the coverage scroll `<div>` the build removes as a tab stop. They are re-aimed
  at the focused row, and both subjects are fired in this chunk: `sr-empty` needs no Pulse, and `sr` live shares the
  driven leg's slot. The host's portable NVDA exists (the handle is not persisted), so an agent runs it. Only if NVDA
  genuinely cannot start does /implement say so in one line, and the wrap then pins it to an owner entry in this
  version, never residuals.

## Folded freight (route.py pins: 3 blocks on `:63` — CONTEXT 940 · CARRY 564 · PREREQ 105 chars)
- **CONTEXT:** the four claims were measured asserted by NO suite on 2026-09-17 and recorded in §5 with this entry as
  owner. Re-verified at take-up: §5 `:354`, `:356` and `:370` each name this entry as owner. Every Arrow-key assertion
  drives the picker listbox on the screen-reader leg. The entry's ring grep (`… crates/conductor-tauri/ui/test/a11y/` →
  1 hit, `accessibility.e2e.ts:33`) is a coordinate P3 re-runs, never inherits.
- **CARRY:** `v3-03` is unblocked by closing these (above). The enumeration is the acceptance surface; its
  "names all four" clause is falsified (above).
- **PREREQ:** close the Rust gate deferral (deferred since `2026-09-30-mutation-gate-grades-every-tally-it-rests-on`), above.

## Boundaries
- A driven-arm (`npm run a11y:driven`) claim is not moved; the hold-dependent half stays the driven arm's. The HOLD
  dialog's Proceed/Abort SHORTCUTS belong to `:370`. VERIFIED at P3 that a routine-arm assertion cannot reach them: the
  dialog renders only while `holdPrompt` is non-null (`App.tsx:374-375`), which only `start_run`'s hold `Channel` sets
  (`:211-220`), behind a preflight-ready live Pulse.
- **A routine-arm Start shortcut would start a REAL run** — FINDING at P3. `start()` (`App.tsx:202-229`) invokes
  `start_run` with the seeded `CONDUCTOR_RUNS_DIR` (`wdio.conf.ts:373`). With no sidecar on `PATH` (CI; and the dev
  host's Bash `PATH` resolves none, `which andromeda-pulse-mcp` → not found), preflight blocks in ~0s
  (verification-harness.md 2026-08-20). A `Blocked` record then lands in the seeded runs dir, mutating the subject that
  later specs in the same file read. A Start-shortcut assertion must either sit last and tolerate that, or assert
  something other than a started run.
- No CI workflow change is planned — VERIFIED at P3: the routine arm is `wdio.conf.ts:320`'s default `specs`, so a new
  `it()` rides the `a11y` job unchanged. The skip allowance is `A11Y_EXPECTED_SKIPS=2` in both harness shells
  (`agent-run.sh:253`). A new skipping spec would need that constant moved in both shells, a harness change the
  boundary does not plan.
- The screen-reader leg — [premise-corrected at the P5 review: it is IN scope for the three rows the new tab stop
  moves (S0-09, E0-05, E0-09); see "Decided at P4" above]. The rest of the SR pass is untouched.

## CI at take-up (Setup 5a)
- `7ee2fea` (the last wrap commit; the only sha since the last flip): **verdict not yet available**. CI#36691941969 was in
  progress, checks 3/3, the oldest running check (`Rust gate`) at 145 s when read. Not folded, not read as green.

## Watch items
- The working tree at take-up carried three post-wrap bookkeeping files (the handoff's session-end stamp, friction
  appends, the wrap's evolve JSON). They are expected-transient, not this chunk's delta.
