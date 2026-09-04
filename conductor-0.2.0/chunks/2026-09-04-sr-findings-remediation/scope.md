# Scope — SR findings remediation

**Marker:** `2026-09-04-sr-findings-remediation`
**Version:** conductor-0.2.0 · **Epoch:** 6a — Verification follow-ups
**Working entry (verbatim head):** _SR findings remediation — the eight remaining screen-reader findings from
the NVDA pass fixed at their defects, then the sr\* leg re-run against the spec_

---

## What this chunk builds

The eight screen-reader findings the operator's 2026-09-02 review left standing are fixed **at their
defects** — in the webview's markup/behaviour and, for one of them, in the Rust run driver — and the
`sr*` suites are then re-run against the shipped spec so each fixed row is graded from a fresh pass
rather than argued from the old one.

The governing constraint is the entry's own: **fix at the defect, never by relaxing a spec row.** Where a
spec row's `expected` text encodes the *defective* behaviour as expected (measured below — S0-06 does),
the row is TIGHTENED to the corrected behaviour; it is never loosened to keep a red row green.

## The eight findings (authoritative: `leg-verdict.md` §Findings for wrap)

Row ids re-verified present in `evidence/nvda-pass.json` (51 rows, all `arm: agent`).

| # | Finding | Rows | Row outcome as recorded |
|---|---|---|---|
| 1 | Initial focus sits in the picker input with a populated catalog, so a keyboard user's first Tab skips the titlebar controls | S0-16 (review cause) | `announced-differently` / `finding` |
| 2 | Focusing a scroll region reads its whole content in one utterance (`Coverage rows` 83 rows, `Run report rows`) — the `tabindex=0` group carries the table as its accessible content | S0-09, E0-05, E0-06 | all `announced-as-expected` |
| 3 | Assertive phase-line flips cancel the focus-restore announcement after a hold resolves | S2-07, S3-04 | both `announced-as-expected` |
| 4 | A load-time `role="alert"` is silent until NVDA binds the window — heard only on reload | R0-01 | `announced-as-expected` |
| 5 | `aria-current` is spoken as "current"; the ` · selected` text only on re-navigation | S0-06, S1-03 | both `announced-as-expected` |
| 6 | The picker's filter-miss prose (`role="presentation"`) is never announced | S0-16 | `announced-differently` / `finding` |
| 7 | The titlebar count is unlabeled and unannounced | S0-12, S1-02 | both `not-run-here` (browse class) |
| 8 | `Stop` during the LAST scenario announces `aborted` client-side then settles to `idle` while the backend records `Done` | S3-05 / T-01 | `announced-as-expected` / `not-run-here` |

**A property of this table that shapes the work** `[inferred]` — nine of the thirteen cited rows are graded
`announced-as-expected`. These findings are therefore **not failing rows**: the row passed its token
assertion and the operator judged the *product behaviour* defective anyway. Consequences: (a) a green
`sr*` re-run is not by itself evidence that a finding is fixed — the row's `expected`/`tokens` must first
be tightened to state the corrected behaviour; (b) findings 2, 3, 4, 5 and 8 each need a spec-row change
*and* a product change, and the spec change is the part that makes the re-run decisive.

**Finding 7 sits on the browse class** `[inferred]` — S0-12 and S1-02 are two of the fourteen rows that
record `not-run-here` because WebDriver-injected keys never reach NVDA's browse mode. So finding 7's fix is
verifiable on the agent arm **only if** the browse class becomes reachable (see the CARRY-2b boundary
question below); otherwise its proof is a finding, not a pass.

## Surfaces and contracts touched

- **Webview source** — `crates/conductor-tauri/ui/src/`: `App.tsx` (initial focus, load-time alert),
  `components/ScenarioPicker.tsx` (filter-miss prose, `aria-current`), `components/Titlebar.tsx` (count
  label, phase-line live-region politeness), `components/CoverageMatrix.tsx` +
  `components/RunReport.tsx` (scroll-region accessible content), `components/OperatorPauseDialog.tsx`
  (focus restore after a hold).
- **Spec + row table** — `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` and
  `rows.ts` (the 51-row source of `expected` / `tokens` / `cls`).
- **Leg harness** — `test/a11y/screen-reader.e2e.ts`, `screen-reader/parse-nvda-log.ts` (see the
  `tabsToStart` gap below).
- **Rust run driver** — `crates/conductor-run/src/lib.rs`, `drive_run`'s abort poll (finding 8).
- **a11y-plan §4 ARIA Patterns / §5 Keyboard Navigation** — the required focus order at `a11y-plan.md:124`
  and the dialog focus-restore contract at `:131` are the specs these fixes must satisfy; SC 4.1.3 Status
  Messages, SC 4.1.2 Name/Role/Value, SC 2.4.3 Focus Order and SC 1.3.1 are the criteria in reach.

## Boundaries — what this chunk does NOT do

- **The sidecar console pane** (`S1-01`, the host path NVDA spoke) is the SEPARATE route entry
  _Sidecar spawn without a console window_, next in Epoch 6a. Not this chunk's, even though it is the
  finding that silenced whole row windows during the leg.
- **No second automation stack.** The leg stays WebdriverIO + tauri-driver + NVDA's own `-l 12` speech log.
- **No spec-row relaxation** — the entry forbids it explicitly.
- ~~**Not a live-Pulse chunk.**~~ `[premise-corrected: the sr suite's `live` subject presses Start (S1-01)
  and waits for the in-run preflight canary hold (S2-01, `screen-reader.e2e.ts:304+`); `execute_scenario`
  returns a Blocked record at `!pf.ready` BEFORE its hold site, so with no ready preflight the hold never
  fires and every S2-*/S3-* row is unreachable. `conductor preconditions` measured all three subjects unmet
  at 2026-09-04T02:22Z (exit 1). **The `sr` suite IS a live-Pulse leg**; `sr-empty` and `sr-error` start no
  run and are not.]` — see research.md §Scope premise closure + §Open questions.

## Folded annotations

### CONTEXT (operator WRAP directive 2026-09-02, item 1)
Folded in full above. Its closing sentence — _"the 14 browse-class rows are the A11y CI gate's CARRY
(OS-level key injection), not this entry's"_ — is **stale by construction** and raises the one open
boundary question below.

### CARRY 2a — Guidepup as the agent-driven SR option: weighable here, NOT adopted
Verbatim claim preserved: `@guidepup/guidepup` 0.34.0 (+ `@guidepup/setup` 0.25.2) *"exists on npm and is
maintained"*; the chunk's zero-dependency form drove **33 of 51 rows** `arm: agent`. `[inferred]` — the
33/51 figure is **re-derived and confirmed** from `nvda-pass.json` (`announced-as-expected` = 33; all 51
rows carry `arm: agent`). The npm version/maintenance claim is NOT verified here (no registry read at
fold time) and stays a hypothesis to weigh at P4. A driver package earns its place **only** if it reaches
what the speech log cannot — chiefly the browse-mode rows — and it would be a second automation stack plus
a dependency delta under the standing advisory-DB deferral (test-plan §6, security-plan §Dependency
Security). Default stays agent-driven wherever a driver exists; the manual arm is the interim, never the
design.

### CARRY 2b — OS-level key injection for the 14 browse-mode rows: makes the manual arm zero
Verbatim mechanism claim preserved: the rows recorded `not-run-here` are *"static text NVDA reaches only
through its browse-mode commands, which its OS keyboard hook sees and WebDriver-injected keys never
deliver (**measured on every session**); `SendInput` / `SendKeys` from the leg's own process (or its
activation script) would deliver them, turning the browse class agent-driven with no new dependency.
Findings, never passes, never a manual arm, until this lands."* `[inferred]` — the fourteen ids
(S0-08, S0-10, S0-11, S0-12, S0-13, S0-14, S0-15, S1-02, S2-06, S3-06, S3-07, E0-01, E0-07, E0-08) are
**verified present and all `arm: agent` / `outcome: not-run-here`**. The *measured-on-every-session*
mechanism keeps its marker and is P3's to close at that depth; the `SendInput` remedy is an untested
hypothesis.

### PREREQ (standing, external decay) — re-check `cargo audit`
Folded verbatim. Standing deferral since 2026-08-09, ratified at the 2026-08-10 wrap; basis: the RustSec
advisory DB itself will not parse — a DATABASE fault, not a tool fault, so no floor raise exists to make.
Overlap `cargo deny check advisories bans licenses sources` runs green every chunk.
**SIGNATURE:** `cargo audit` exit **1**, first diagnostic
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
`cargo deny` exit **0** — reproduced byte-identically at the `2026-09-03-live-pulse-preconditions-probed`
wrap (the 48th, package count 564 unchanged); **this is the 49th**. Read the exit DIRECTLY, never through
a pipe. ANY deviation restores the FULL form and is reported. Both probes MUST appear in the plan's
`## Test Commands` with their dispositions, or the wrap cannot re-pin the deferral
(security-plan §Dependency Security, 2026-09-03 extension).

## Coordinate re-verification (fold-time; coordinates only)

| Coordinate as written | Verdict |
|---|---|
| `evidence/nvda-pass.json` | **OK** — present, 51 rows, 16 `findings`, `operator_review` |
| `leg-verdict.md` §Findings | **OK** — §"Findings for wrap (product and plan)" at `:81`; the eight map 1:1 |
| All 13 CONTEXT row ids | **OK** — every id present |
| All 14 CARRY-2b row ids | **OK** — present, all `not-run-here` |
| `conductor-run/src/lib.rs:936` | **CORRECTED** → the poll is `lib.rs:975`; `drive_run` is `:956`; the doc comment stating the behaviour is `:947`. `:936` lands inside the `RunStage` enum. |
| `` `@foreground.initialFocus` record `` | **CORRECTED — does not evidence finding 1.** See below. |

**The `@foreground.initialFocus` correction** `[inferred]` — the record exists (written at
`screen-reader.e2e.ts:110`, `FOREGROUND_STAMP` in `parse-nvda-log.ts:183`, surfaced as
`subjects.*.foreground.initial_focus`), but it reads **`"BODY"` on all three subjects**, because
`activeName()` samples `activeElement` while the defect lives in Chromium's *sequential-focus-navigation
starting point*, which is a separate thing (the code says so at `screen-reader.e2e.ts:80-83`). The field
that actually discriminates finding 1 is **`tabsToStart`** — captured into the actions ndjson at `:110`
but **dropped by the parser**: `parse-nvda-log.ts:439-444` emits only `activated` / `nvda_named_window` /
`initial_focus`, and `tabs_to_start` appears nowhere in the committed JSON. **Consequence for this
chunk:** re-running the leg after fixing finding 1 would produce no surfaced field proving the fix, so
extending the parser to surface `tabsToStart` is in scope as the evidence path for finding 1.

## Open boundary question — resolve at P4

**Does this chunk own CARRY 2b (OS-level key injection)?** Two dated instruments disagree:

- The **CONTEXT prose (2026-09-02)** says the browse rows are "the A11y CI gate's CARRY … not this
  entry's" — true when written, when both CARRYs sat on *A11y CI gate*.
- The **operator's 2026-09-04 wrap directive, item 2** re-pinned CARRY 2a and 2b onto *this* entry,
  byte-identical; the session handoff states this chunk "carries the eight NVDA findings, the two SR
  CARRYs re-pinned this wrap, and the standing cargo-audit PREREQ".

The CONTEXT sentence was preserved byte-identically across a move, so its internal cross-reference now
points at an entry that no longer holds the CARRY — the **same staleness class** the 2026-09-04 wrap
caught once and reworded (the third CARRY's "the entry that next touches the ui test tree"); this second
instance was inside prose and was not caught.

**Lean `[inferred]`:** the later dated directive governs — CARRY 2b is pinned at annotation position on
this entry and folds in. It also materially changes the chunk: finding 7's rows are browse-class, so
without 2b finding 7 cannot be proven on the agent arm at all. **But it is a large addition** (a new
`SendInput`/`SendKeys` path from the leg's process), so it is a contestable fork for P4, not a decision
taken here.
