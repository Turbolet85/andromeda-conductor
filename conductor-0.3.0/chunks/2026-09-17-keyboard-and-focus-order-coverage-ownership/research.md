# Codebase Research — 2026-09-17-keyboard-and-focus-order-coverage-ownership

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8 · **Graph queries:** 2 (`ts` plane, `db_state` warm)
- **Harness rules consulted:** `.claude/rules/a11y.md` (read IN FULL, `## Session Additions` included — it
  auto-loads on `crates/conductor-tauri/ui/**`) **and `.claude/rules/verification-harness.md`**, read at P5
  when check 4 (5) fired: the plan carries a `leg = 'live'` entry (`bash scripts/agent-run.sh run --e2e`), so
  the accumulated firing form was owed. At 71 485 bytes over 66 lines it is past the read cap, so the in-full
  read was PERFORMED as a structural extraction — `grep -n` for the section headers **and** the per-entry
  `^- 20\d\d-` introducers (6 headers, 27 entries indexed), then offset-bounded reads of the three entries
  governing this leg: `:58` (firing form + stop form + the mandated before/after process census), `:62` (only
  `--e2e` rebuilds the bundle), `:64` (a live leg's `expect` atoms are the one gate class with no mechanical
  check — assert what the runner PRINTS, read from a recorded output or the tool's own print site).
  **This corrects P3's own slot**, which read "No live leg in this chunk" — written before P4 authored the
  `--e2e` entry, and exactly the mismatch check 4 (5) exists to catch.
- Measured: `.claude/rules/testing.md` and `.claude/rules/verification-harness.md` do **NOT** auto-load on this
  chunk's touched files — both glob `crates/**/tests/**` and the a11y suites live under
  `crates/conductor-tauri/ui/test/` (**`test`, singular**), which that pattern does not match. The harness rule
  was therefore read deliberately, not delivered — which is the failure mode its own `:51` entry warns about.

## Files inspected
- `.andromeda/a11y-plan.md` `:112`, `:131`, `:253`, `:353-372`, `:514-520` (offset-bounded, char-exact) — the
  claim population and the §11 *Strategy* carve-out that is the statement's home.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (`:200-235`, `:405-470`, plus an `it(`/`describe(`
  index) — the routine arm: which claims it really asserts, and which it only names.
- `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` (`:1-60`, `:104-160`, plus an assertion index) —
  the driven arm: the hold-dependent assertions in full.
- `crates/conductor-tauri/ui/wdio.conf.ts` `:318-346` — the suite registry.
- `.claude/rules/a11y.md` — in full.

## Graph impact (code-graph, `ts` plane — both queries in `tree-query-2026-09-17-keyboard-and-focus-order-coverage-ownership.json`)
- **`tabCycle` @ `accessibility.e2e.ts:215`** — 2 call sites, **both inside its own file**: `:409` (the SC 2.1.1
  spec) and `:425` (the SC 2.4.3 spec). No external caller.
- **`focusSnapshot` @ `accessibility.e2e.ts:184`** — 2 call sites, `:216` and `:222`, both inside `tabCycle`.
- **`activeName` / `focusInsideDialog`** — defined TWICE, independently, per suite: `operator-hold.e2e.ts:25`
  / `:37` and `screen-reader.e2e.ts:185` / `:247`. They are **not** a shared helper; each suite carries its own.
- **Consequence for the plan:** every keyboard helper is file-local. Any edit inside a spec file has zero blast
  radius beyond that file, and there is no shared helper whose change would touch two arms at once.
- *(Graph `line`/`def_line` are 0-indexed per `code-graph-cookbook.md`; every number above is the editor line,
  i.e. `line + 1`.)*

## Patterns detected
- **Independent-halves assertion shape** (`accessibility.e2e.ts:409-422`): the SC 2.1.1 expectation builds its
  ACTUAL side from the Tab walk and its EXPECTED side from a `document.querySelectorAll` roster, so the two can
  actually differ — the predecessor's correction of the `${names}`-on-both-sides defect. The same shape recurs
  at `operator-hold.e2e.ts:147-151` for restoration (`restored=… landed_on=…` vs the invoker name).
- **Identity-keyed cycle detection** (`accessibility.e2e.ts:215-232`): `tabCycle` delimits a lap by the first
  REPEATED element identity and deliberately does not wait for a `BODY` sentinel, with the reason recorded in
  its doc comment. Rotation-compared at `:425-433`, so where the walk enters the ring is not asserted.
- **Skip-guard-then-assert** (`accessibility.e2e.ts:456-467`): the checklist spec guards on the hold marker
  `[role="alertdialog"]` (never on `role="status"`, which is a growing population) and then asserts.
- **Per-arm ownership prose already exists in the DERIVED tier**: `.claude/rules/a11y.md` §Testing already
  names all three families and assigns "the HOLD-dependent half only — trap, Space toggle, Escape→NoGo, focus
  restoration" to the operator-only driven arm. The master `a11y-plan.md` §5 does not.

## Conventions to follow
- **Owners are named by job/suite name, never by workflow line coordinates** — `a11y-plan.md` §3 *Configuration*
  states this in its own words; the registry to name against is `wdio.conf.ts:320-327`: default `specs` =
  `accessibility.e2e.ts` (routine), `suites.driven` = `operator-hold.e2e.ts`, `suites.sr`/`sr-empty`/`sr-error`
  = `screen-reader.e2e.ts`.
- **A CI claim carries its configuration** (`a11y-plan.md:516`) — hosted `windows-2022`, coherent
  `131.0.2903.86` pair, High integrity, run 35208593666 — never a bare verdict.
- **Name the SET, not a fresh literal** (`.claude/rules/a11y.md` 2026-09-17; design/tests/arch extracts agree).

## New files to create
- *(decided at P4 — see Open questions 1; a doc-only outcome creates none.)*

## Files to modify
- *(provisional, gated on Open question 1)* `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — only
  if P4 elects to close the vacuous `:450` spec. Graph-confirmed zero external callers.
- **NOT a touchpoint under any option:** `.andromeda/a11y-plan.md`. `plan-template.md` §Discipline forbids any
  of the seven spec masters in `Files to modify`; its content changes ride
  `Expected amendments (wrap): a11y-plan.md §… — …`.

## Measured findings

**F1 — `accessibility.e2e.ts:450` is vacuous by construction.** Its title is
`'operator-pause dialog: alertdialog role, focus trap, Escape resolves NoGo, focus restores'` — four claims —
and its body (`:450-454`) is the `this.skip()` guard **and nothing else**: `sed -n '450,455p' … | grep -c 'expect('`
returns **0**. Even with a hold raised it would assert nothing. This is the same class the predecessor closed at
`:388`, and it sits inside this chunk's claim population. **Naming the routine arm as owner of those four claims
on the strength of this spec's title would attribute four claims to a spec that asserts none.** Its sibling
`:456` is not vacuous (one assertion after the guard).

**F2 — the driven arm genuinely asserts the hold-dependent half.** `operator-hold.e2e.ts:44` asserts trap
containment forward and backward (`focusInsideDialog()` true at `:79` and `:83`), Space toggling a row with the
`role=status` roll-up changing (`:106-117`), Escape dismissing the dialog (`:119-125`), and restoration to the
recorded invoker with `landed_on` empty (`:147-151`). So it is a real owner, not a nominal one.

**F3 — `:366` carries a retired attribution that `:362` contradicts, inside one section.** `:366` reads
"(Radix AlertDialog default, layout excerpt's Focus Management Anchors)", while `:362` states restoration comes
"via the dialog's explicit `onCloseAutoFocus` + `restoreFocusTo` contract, **NOT** a Radix default (… as measured
2026-09-01 Radix restored to `<body>`)". `:131`, `:253` and `.claude/rules/a11y.md` all agree with `:362`. `:366`
is the lone survivor of the retired reading and is one of the bullets this chunk must attribute.

**F4 — §5 omits the checklist rows that §1 and §3 include.** `:355` and `:362` describe the HOLD cycle as
"across Proceed/Abort" only; `:131` and `:253` both put the operator-checklist `checkbox` rows inside the cycle
("across the operator-checklist `checkbox` rows … and then Proceed/Abort"). The layouts extract independently
confirms the rows ARE in the trap population (§Component — Hero / Operator-checklist), and the driven arm
asserts them (`:87`, `:106`). So §5's two bullets are the abbreviated pair, not §1/§3.

**F5 — SC 2.4.7's "visible ring on the active element" is asserted nowhere.** Derivation:
`grep -rnE "2\.4\.7|color-focus|focus-visible|focusRing|outline|boxShadow|box-shadow" crates/conductor-tauri/ui/test/a11y/`
returns **exactly 1 hit** across the whole a11y tree — `accessibility.e2e.ts:33`, a token-pair CONTRAST row
(`{ fg: '--color-focus', bg: '--color-base', need: 3 }`). That proves the ring's 3:1 ratio, not that a ring is
rendered on the focused element. §5 `:372` binds the population to "SC 2.1.1 / SC 2.4.3 / SC 2.1.2 / SC 2.4.7"
and §3 `:253` asks for a "visible `--color-focus` ring on the active element (SC 2.4.7, paired with the contrast
harness for the 3:1 ring)" — the paired half exists, the active-element half does not.

**F6 — the derived tier already says what the master does not.** `.claude/rules/a11y.md` §Testing carries the
per-arm attribution in full. The ownership content is therefore already agreed in this project; what is missing
is the master's own per-claim statement and the carve-out's substitute-gate sentence.

**F7 — the carve-out at `:516` is closer to done than scope assumed.** It already narrows to "the HOLD-DEPENDENT
Operable half only" and already states *why* that half is operator-local ("needs a live Pulse and stays barred
from CI by test-plan §11 and architecture §Established Decisions [CI/CD]"). What it does **not** do is (a) name
the owning SUITE, and (b) say what gates in its place — which is exactly v3-03's second sentence.

## Scope premise closure
Every `[inferred]` bullet in `scope.md` was re-read against these findings; `scope.md` is amended in the same
pass (P4 consumes the corrected copy).

- **VERIFIED** — v3-03 is the targeted capability; the "for every … claim" quantifier is the ledger's, not the
  entry's; `operator-hold.e2e.ts` is the driven suite and CI never runs it (`wdio.conf.ts:321`, test-plan §11,
  architecture §Established Decisions [CI/CD], `.claude/rules/a11y.md`); only the named-owner-plus-carve-out
  terminal is reachable; the `a11y` job's spec set is untouched; no Rust surface and no dependency delta.
- **VERIFIED** — the open channel fork is real and unresolved: `plan-template.md` §Discipline bars spec masters
  from `Files to modify`, so the master's text can only move through `Expected amendments (wrap)`.
- **PREMISE-CORRECTED** — the scope's claim-population table said the `run-console-idle` row was "unnamed
  (asserted in the routine arm)". `:353` in fact **already carries an explicit attribution** ("**Asserted
  UNATTENDED on the routine arm since 2026-09-07**"), so that row is already owned and needs no new statement.
  The genuinely unattributed rows are `:354`, `:355`, `:362`, `:366`, `:370`.
- **PREMISE-CORRECTED** — the scope said "for those two the work is attribution, not relocation", treating the
  routine arm's hold-dependent specs as a non-issue. F1 falsifies the easy reading: `:450` names four claims and
  asserts none, so attribution to the routine arm is not merely unnecessary there, it would be **false**.

## Open questions
1. **Which channel authors the ownership statement, and therefore what /implement builds?** `a11y-plan.md`
   cannot be a touchpoint, so either (a) the chunk is doc-only and everything rides
   `Expected amendments (wrap)` — in which case v3-03's decisive artifact is wrap-authored and the capability is
   **not claimable by this chunk** (`verification-matrix-contract` claim-reachability) — or (b) /implement
   writes an enumeration it CAN author (a committed ownership map beside the suites, and/or closing F1), with
   the master's prose riding wrap. → blocks: **plan-decision** (P4 resolves before synthesis).
2. **Does closing F1 (`:450`) belong to this chunk?** `scope.md` bans "new keyboard or focus-trap assertions",
   and closing a vacuous spec is arguably a fix rather than a new assertion — but it is the only way the routine
   arm's own file stops claiming coverage it does not provide. → blocks: **implementation-scope**.
3. **Do F3/F4/F5 ride this chunk's amendment set, or are they adjacent findings for the route?** All three sit
   inside the claim population, none is an ownership question per se. → blocks: **plan-decision**.
