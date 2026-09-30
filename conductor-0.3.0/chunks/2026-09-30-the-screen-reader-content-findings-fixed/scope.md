# Scope — 2026-09-30-the-screen-reader-content-findings-fixed

**Working entry (`working-route.md:71`):** The screen-reader content findings fixed. Every SR row conveys its required
content, and the next regrade grades all 51 rows.

**Matrix target:** none expected. At take-up the pool read `verified 10/11 · deferred 1 · planned 0 · unclaimed 0`
(`matrix.py coverage`, new-session 2026-09-30T16:30Z). P4 re-reads the pool with `show --unclaimed`.

**Chunk base (W182):** `d7da5d0` (`d7da5d07b94e5a7552dc5340f30b68b5920bd822`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat d7da5d0 -- <f>`), because an operator pre-CI commit moves HEAD before
the wrap (founder directive W182, restated by the operator at this take-up).

## Operator directives and founder rulings at take-up (2026-09-30)
- **Order: the PREREQ comes first.** The Rust gate deferral is closed before any product or harness work (below).
- **Founder ruling: `/andromeda-evolve-diagnose` waits.** The new-session nudge for Epoch 4 stays unacted.
- **Founder ruling: arm K is retired.** It is not run, not planned and not simulated in this chunk. The record reads
  "not run — founder ruling, cause already isolated by C1" (a11y-plan §3).
- **Founder ruling: the U35 upgrade door is not this chunk.** `upgrade.py detect` reads U35 `behind`, awaiting a
  0-pending wrap door. This chunk neither opens nor closes it.
- **Confirm each row against the rendered DOM before planning a change.** For every row below, research reads the
  rendered DOM node the row names. It then records a disposition: **product fix** (the DOM does not carry the content
  the row requires) or **re-token** (the DOM carries it, and the row's token, node or action is what is wrong). The
  reason is recorded either way.
- **Every NVDA or window run is the operator's quiet-desktop slot.** Research and implement alike STOP and ask before
  any NVDA launch or any run that raises a window, and state the run's expected LENGTH. Nothing starts unasked.
- **A live `sr` run also needs the operator's `:4317` grant.** Ask for it alongside the slot, with its length.
- **Any new crossing is a P4 escalation for the founder's live word.** This covers a ninth harness-spawn form, a new
  listener, or any other boundary widening (security-plan; `rules/security.md` rule (b) holds EIGHT forms). A delegate
  cannot ratify it (`rules/security.md` Session Additions 2026-09-29).

## The PREREQ, folded
- `PREREQ: close rust gate deferral (deferred since 2026-09-30-the-sr-pass-regrades-on-the-os-input-path)`.
  At that chunk's wrap (its `report.md` §Gates), the **clippy** `defer` stood: zero Rust delta, and clippy reads Rust
  source only. The **unit** `defer` was voided at the same wrap: `bash scripts/agent-run.sh run --unit` ran in full
  (exit 0, `1136 tests run: 1136 passed, 0 skipped`).
- Closing it here: run `cargo clippy --workspace --all-targets -- -D warnings` and `bash scripts/agent-run.sh run
  --unit` in full against this chunk's tree, and record both verdicts. "Close" means both gates run in full, each as
  its own command (test-plan §9 keeps Lint out of `--unit`; tests extract). The Rust delta is zero: research's
  modify-set is `crates/conductor-tauri/ui/**` plus the chunk's evidence only. So a new deferral must not be
  minted, and the plan lists both gates unconditionally.

## The CARRY, folded (from `2026-09-30-the-sr-pass-regrades-on-the-os-input-path`; relay `conductor-wrap-osinput-2026-09-30` §3)
Record: `conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/nvda-pass.json`. The
operator review is transcribed: 31 accepted as heard, 18 findings, 2 accepted with reason (E0-10, S1-05,
subject-absent). Measured on WebView2 154.0.4258.37 × NVDA 2026.2 × Windows 26200.9457 × the OS input path. The 18
findings split into five groups:

1. **The P-ID is missing from what is heard** (E0-05, S0-09, E0-09, S3-07).
   - E0-05 and S0-09: focus rows, `Tab` on the OS path, node `tr[tabindex=0][aria-current=true]` in
     `table[aria-label="Coverage rows"]`. Heard "Capability coverage matrix region · Coverage rows table · row current";
     `P-001` is missing. These are the entry's two routed reds: `announced-differently`, missing P-001, and the
     focus-row count (2, bar 0).
   - E0-09: `Shift+Tab` then ArrowDown. Heard "row current" twice; `P-019` is missing.
   - S3-07: browse row, ArrowDown. Heard the keyboard-hint line; `P-025` is missing.
   - Claim, as the CARRY states it: "the coverage row's accessible name carries its P-ID". This is the product fix
     for E0-05 and S0-09: `tr.cov__row` carries no author name (`CoverageMatrix.tsx:113-123`), and NVDA heard an
     empty name. [premise-corrected: E0-09 and S3-07 are not the row-name defect alone. E0-09's one ArrowDown moves
     the roving focus to row index 1 (P-002), and P-019 is index 18. S3-07's ArrowDown from Start read the hint line
     in browse mode. Both need the name fix PLUS an action that reaches their row — research.md §Per-row DOM
     confirmation.]
2. **The report and matrix cells convey their status and fixture values** (E0-07, E0-08, S0-15, S3-06). Each is a
   browse row driven by ArrowDown on the OS path.
   - E0-07: `lamps-fixture` missing; heard "table with 4 rows and 6 columns row 1 column 1 P-ID".
   - E0-08: `Blocked` missing; heard "column 2 Scenario".
   - S0-15: `Not yet run` missing; heard "row current".
   - S3-06: `Manual` missing; heard "button unavailable Control+dot Stop".
   - [premise-corrected: the DOM carries all four values — `lamps-fixture` in `header.report__summary`
     (`RunReport.tsx:41-44`), "Blocked"/"Manual" as `span.lamp__label` text (`StatusLamp.tsx:17`), and "Not yet run"
     as `span.cov__unrun` (`CoverageMatrix.tsx:142`). E0-07, E0-08 and S3-06 heard neighbouring lines, because the
     virtual cursor started after (E0-07) or short of (E0-08, S3-06) the node. They are **re-tokens** of the action,
     not product defects. S0-15 is different: its "row current" is the roving focus moving to row index 1 with an
     empty name, so it rides the row-name product fix (the name carries the status) and is re-classed as a focus
     move.]
3. **An h1, and a `contentinfo` landmark or a recorded reason for none** (S0-13, S0-14). NVDA logged the `h` and `d`
   keys and spoke nothing.
   - S0-13: the node lists three `h2` and no `h1`.
   - S0-14: banner, main and two regions; `contentinfo` is absent.
   - [premise-corrected: both halves hold, for different reasons. The product facts are confirmed in the DOM: three
     `h2` and no `h1` (`App.tsx:292,347,371`; the phase line is a `span`, `Titlebar.tsx:32-38`), and no `footer` or
     `contentinfo` (whole-file `App.tsx`). The master requires both (a11y-plan `:323` single h1-equivalent on the
     phase line; `:548` MUST expose `contentinfo`). The silence is a leg-state fact. `h`/`d` were sent from the
     focused coverage row, and the record shows that row is in focus mode (hypothesis, consistent with every heard
     line), so the keys went to the app, which ignores them (`CoverageMatrix.tsx:76-77`). These rows need the product
     fix AND their keys sent from a browse-mode position, with the tokens re-derived.]
4. **The 7 browse `not-run-here` rows are driven with OS-path browse keys in their windows** (E0-01, S0-08, S0-10,
   S0-11, S0-12, S1-02, S2-06). Each was `arm: operator`, `input: none`: no browse key went through the OS input path
   in the row's window. This is a leg coverage gap, not a product defect: each row's own DOM assertion passed on the
   2026-09-30 leg (research.md). `send-keys.ps1`'s closed `-Key` set admits `h`, `d` and ArrowDown
   (`send-keys.ps1:12`), so driving these rows needs no new key and no new spawn form.
5. **T-01 is a live-class row, not run by design.** The live subject stops at `aborted` (the aborted state needs a
   Stop), so the un-stopped run's "Conductor · idle" on the Done stage is never reached. It needs a second, un-stopped
   live session, not a browse key. That session needs the operator's quiet-desktop slot and `:4317` grant.
   [premise-corrected: T-01 does not need a second SESSION. It needs a second, un-stopped RUN in the same live
   session: after S3 settles, Start is enabled again (S3-02), and a run driven through both holds reaches Done. That
   uses the existing `osKey`/`injectKeys` forms and the app's own rule (a) sidecar spawn, so there is no new process,
   port or spawn form, and no escalation. It lengthens the live leg by one more run, which the slot request states.]

- **Hypothesis, kept as stated:** "these are product a11y defects and the product fix is the remedy, not the tokens;
  the entry's research confirms each row against the rendered DOM before it plans a change, and a row whose token is
  wrong rather than the product is re-tokened with the reason recorded." [premise-corrected: partly true. The
  product defects are the coverage row's missing name (E0-05, S0-09, and through it S0-15 and E0-09), the missing
  `h1` (S0-13) and the missing `contentinfo` (S0-14). E0-07, E0-08, S3-06 and S3-07 carry their content in the DOM
  and are re-tokens of the action. The seven browse rows and T-01 are leg coverage gaps. Per-row table:
  research.md.]
- **Done means the next regrade grades all 51 rows.** Resolved at P4: `rows 51 not-run-here 0`, every row carrying its
  input path, every row except the two subject-absent ones `announced-as-expected`, and the two routed reds
  (focus-row count 2 → bar 0; S0-09/E0-05) closed. T-01 is HEARD, driven by a second un-stopped run in the same live
  session (no crossing — research.md §Patterns), so no accepted-with-reason grade stands in for it (plan.md
  §Acceptance Criteria).

## Boundaries
- In: the React webview's accessible structure (row names, status cells, heading and landmark structure), the `sr*`
  legs' row table and key actions (the spec at `crates/conductor-tauri/ui/test/a11y/screen-reader/`), the regrade
  record, and the PREREQ gates.
- Out: arm K (retired), the U35 door, evolve-diagnose, Pulse's own UI, and any change to `send-keys.ps1`'s closed key
  set that adds a spawn form without the founder's word.
- The routine `--e2e` arm, the axe/contrast `a11y` job and every existing test must stay green. [premise-corrected:
  the routine and driven arms key coverage rows and the phase line on CLASSES (`cov__row`, `cov__pid`,
  `titlebar__label` — `accessibility.e2e.ts:61-64,324`; `operator-hold.e2e.ts:79-97`), so a row name or a heading tag
  moves no selector. A heading and a footer are not focusable, so the Tab order is unchanged.]
- No new dependency is expected.

## CI verdict read at Setup (5a)
- `d7da5d0`: **verdict not yet available** — run CI#36744501243 (push), `in progress`; checks 3/3; the oldest running
  check was "A11y gate (routine arm · axe · contrast · violation JSON)" at 164 s. Read 2026-09-30 at phase Setup
  (`.andromeda/runs/2026-09-30T16-31-26-phase/`). No red to disposition at take-up. This run reads the previous
  chunk's wrap commit, and implement re-reads it before the pre-CI commit.
