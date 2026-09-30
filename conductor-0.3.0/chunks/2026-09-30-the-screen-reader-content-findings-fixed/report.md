# Report — 2026-09-30-the-screen-reader-content-findings-fixed

**Chunk:** The screen-reader content findings fixed — rust gate deferral closed first, then each of the 18 SR
findings confirmed against the rendered DOM and fixed in the product or re-tokened with its reason, and the
regrade grades all 51 rows
**Date:** 2026-09-30
**Commits:** `dfa4f87 chore(2026-09-30-the-screen-reader-content-findings-fixed): operator pre-CI commit, for the run
this chunk's verdict reads` (the only commit since `last_wrap` 2026-09-30T16:24:27Z beside the prior chunk's wrap
`d7da5d0`; basis `git log --format='%h %s' d7da5d0..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only d7da5d0` + `git ls-files --others --exclude-standard`, excluding `.andromeda/`)
  - product (webview): `crates/conductor-tauri/ui/src/App.tsx` · `src/components/CoverageMatrix.tsx` ·
    `src/components/Titlebar.tsx` · `src/components/Titlebar.css` · NEW `src/components/Footer.tsx` ·
    NEW `src/components/Footer.css` · NEW `src/srOnly.ts`
  - harness (dev-only, `ui/test/`): `test/a11y/accessibility.e2e.ts` · `test/a11y/operator-hold.e2e.ts` ·
    `test/a11y/screen-reader.e2e.ts` · `test/a11y/screen-reader/rows.ts` · `test/a11y/screen-reader/nvda-pass-spec.md` ·
    `test/a11y/screen-reader/parse-nvda-log.ts`
  - chunk folder: `plan.md` · `research.md` · `scope.md` · `scope-record.md` · NEW `evidence/nvda-pass.json` ·
    NEW `evidence/operator-legs.md` · this report
  - route/state: `conductor-0.3.0/working-route.md` and `.claude/session-handoff.md` carried in from the phase run
    (no edit by implement)
- **Symbols / APIs:**
  - `CoverageMatrix` — each `tr.cov__row` gains `aria-labelledby` naming its own four cells (ids `{useId prefix}-{p_id}-{pid|cap|mode|status}`); props unchanged; callers unchanged (`App.tsx:366`, `Gallery.tsx:206` — Gallery is dev-only and keeps working with a second per-matrix prefix).
  - `Titlebar` — the phase line element is now `h1.type-heading.titlebar__label` (was a `span`), `aria-live` switch and drag region kept; the count keeps `aria-live="polite"` but its name moved from `aria-label` to a visually-hidden child (`SR_ONLY` text `Scenario count: no run yet` / `Scenarios completed: {n}`) and the digits sit in a new `span.titlebar__count-value[aria-hidden="true"]`.
  - NEW `Footer({ runState, records })` — `footer.footer.type-label` (top-level ⇒ `contentinfo`) holding one `span.footer__line`: `conductor · seed {records[0].seed | —} · {idle|live|HOLD|aborted} · {n} {StatusLamp}…` per non-zero lamp in `LAMP_ORDER` via `lampForRecord`; HOLD adds an `aria-hidden` `--count-hold` glyph; not focusable, not a live region. Mounted in `App` after `main`.
  - NEW `srOnly.ts` exporting `SR_ONLY` (moved verbatim out of `App.tsx`, now imported by `App` and `Titlebar`).
  - `parse-nvda-log.ts` (SR harness): `readSpeechLog` now also returns `inputs` (NVDA `Input: kb(…):{gesture}` entries); `KeySent` carries `ts` + `key`; NEW exported `calibrateClock(keys, inputs)` + `ClockCalibration`; `SubjectRecord.clock` added; a void calibration grades every non-absent row `not-run-here` with note `session void — NVDA's clock: …`.
  - `screen-reader.e2e.ts` (SR harness): NEW helpers `browseUntil`, `heardSince`, `roveTo`, `currentRowPid`, `holdCaretAtDocumentStart` / `releaseCaretSentinel`, `resetToDocumentStart`, `waitForSecondRunHold`; `act()` passes the stamp's log size to its drive; NEW constants `SECOND_RUN_QUIET_MS = 170_000`, `DONE_TIMEOUT_MS`; the actions timeline gains `@browse` diagnostic records and `@foreground.reset`.
  - `accessibility.e2e.ts` (routine arm): NEW specs "a coverage row is named by its own cells — its P-ID and its status", "the phase line is the single level-1 heading", "the window exposes exactly one main and one contentinfo landmark"; helpers `coverageRowName` (WebDriver `getComputedLabel`, in-page `aria-labelledby` fallback only on unknown command) and `computedRoles` (`getComputedRole`).
  - `operator-hold.e2e.ts` + `screen-reader.e2e.ts` `count()` read `[class~="titlebar__count-value"]` (was `titlebar__count`).
  - No new port, socket, env handle, Tauri command, `Channel`, process or spawn site (`grep -c 'spawnSync(' screen-reader.e2e.ts` = 2, `-Command` = 0; `git diff --numstat d7da5d0 -- send-keys.ps1 activate-window.ps1 wdio.conf.ts` empty).
- **Crates / modules:** no Rust change (`git diff --name-only d7da5d0 -- '*.rs' Cargo.toml Cargo.lock` empty); ui modules added `Footer`, `srOnly`.
- **Dependencies:** none (package.json / package-lock.json / Cargo.lock untouched — gate 8's list).
- **Schema / config:** the SR evidence record `nvda-pass.json` gains `subjects.{subject}.clock` = `{pairs, baseline_ms, max_excess_ms, shifted_stretches[{from_key,to_key,excess_ms}], void, pair_table[{key, leg_ts, offset_ms}]}`; rows carry `review_grade` from the transcription (as before). `rows.ts`: S0-11 tokens `idle`+`level 1`; S0-13 `Coverage matrix`+`level 2`; S0-14 `main`+`content info`; S0-15 re-classed `focus`, tokens `P-002`+`Not yet run`; S1-02 `review` flag removed; S3-07 `P-025`+`Manual`; T-01 `notRun` removed; E0-07 `3 scenarios`+`lamps-fixture`; E0-09 `P-019`+`Blocked`; row set still 51.
- **Spec-master edits:** none (implement touches no master).
- **Counts / qualifiers moved:**
  - SR landmark set: `banner + main + two regions` → `banner + main + regions + contentinfo` (measured: S0-14 heard "content info landmark"; routine spec `main 1 · contentinfo 1`); sites stating the old set: a11y-plan `:320`, `:323`, layout-templates `:158`, design-system `:265` ("footer strip is unbuilt"), `rules/a11y.md` Landmarks bullet (grep `contentinfo` per master: a11y-plan 6 hits, layout-templates 4, others 0).
  - headings: `three h2, no h1` → the phase line is the single `h1` (a11y-plan `:323` names it; grep `\bh1\b` a11y-plan 1 hit, others 0).
  - SR regrade: `not-run-here 8 → 0`, `focus-nonexpected 2 → 0`, findings `18 → 0 of the 18` (+1 new, E0-10, routed forward); rows stay 51 (basis: gates 20/21/22 over `evidence/nvda-pass.json`).
  - routine `--e2e` arm: 19 passing / 2 skipped (three specs added; basis `runs/a11y-e2e.log` of the last gate run). The masters' passing counts are dated run samples (`12 passing …` at named runs — grep `[0-9]+ passing`: architecture 2, test-plan 5, a11y-plan 4 hits, all dated), no change.
  - rule (b) forms: EIGHT, unchanged; ownership summary `11 claims · 9 owned (5 operator-local, carve-out) · 2 n/a-by-construction · 0 recorded gaps` unchanged.
- **Dev-tool versions:** none — msedgedriver (the driver) re-read at 154.0.4258.37 = WebView2 runtime 154.0.4258.37 (gate 6, dev host); NVDA (the screen reader) re-read at 2026.2 (the operator-supplied portable copy, passed per invocation — `CONDUCTOR_NVDA` was unset in the session and is set only for the leg).
- **Harness / gate surface:**
  - SR leg walk: browse rows now drive OS-path browse keys in their windows (`browseUntil`, bounded caps, a miss recorded not thrown); E0-07/E0-08 walk from Start before the Tabs into the matrix; S0-11/S0-12 walk after S0-02 from a held test-only focus target first in `<body>` (then focus returns to Close window in-page); S1-02 from the same target; S2-06 moved between S2-04 and S2-05 (from the focused checkbox); S2-07 is Tab ×2 (was Shift+Tab); the roving rows S0-15/E0-09/S3-07 are keyed on the DOM's current row; T-01 drives a SECOND un-stopped run in the same live session after a 170 s quiet window, through both holds, to Done (a guard throws if the run settles before its hold — a deduped canary).
  - The SR parser's grading mechanism: step-aware clock calibration (below, and in Spec claims disproved).
  - The live `sr` leg's length grew from ~5 to ~6 min wall for the spec (measured `Spec Files … in 00:05:58`) plus the pulse-app launch; the quiet window runs concurrently with S3-06/S3-07.
  - CI job steps: none changed.
- **Cross-project / external claims:**
  - CI#36770454038 (push) on `dfa4f87e7e98` — `verdict: green · checks 3/3 · wall 566 s` (`ci.py conclusion`, entry 32).
  - Pulse `5fbf762` release `pulse-app.exe`, sha256 `e3a65f3cc0cade02c804372a1f6829312702109ff147d591dfe9caa8ffe109dc`, posture read from its own log each launch (`inference_mode: deterministic`, workspace key naming the fresh dir, OTLP bind 127.0.0.1:4317/4318).
  - Relied on Pulse's dedupe of a new canary against ANY open incident (verification-harness `:50`, `:53`) — the basis for T-01's quiet window; the second run formed its incident and raised both holds (measured).
- **Reverted / negative API facts:**
  - The footer's first form (flex items with `aria-hidden` `·` separators) was replaced: NVDA read it "conductorseed —idle" (live leg #1). Now one text line with plain-text separators.
  - The count's `aria-label` (the pre-chunk form) was removed for the visually-hidden child (above).
  - A focus-and-REMOVE sentinel as S0-11/S1-02's caret start (live leg #1: seven ArrowDowns, no speech) → replaced by a HELD sentinel.
- **Insufficient fixes (written, kept, not the remedy):** `resetToDocumentStart` (the focus-and-remove sentinel) — written for sr-error's harness defect #1 (an injected reset cycle ending off BODY) and kept as `bringToForeground`'s fallback; every later session's cycle reached BODY (`@foreground.reset = "cycle"` in all six), so it has never been exercised on its own defect — owner: none needed until a cycle ends off BODY again.
- **Spec claims disproved by measurement:**
  1. Plan step 7 / research: browse walks start "from BODY after the reset cycle" at the document start — FALSE: NVDA's browse caret follows the last DOM focus (E0-01 from BODY walked the report table, `row 1 column 1 P-ID …`), and NVDA binds the document only at the first OS-path focus change (S0-11/S0-12 walked before S0-01 were silent; S0-01 heard "Conductor document"). Evidence: the saved logs of slots 3/5 (`evidence/operator-legs.md` slot history).
  2. research/scope: T-01 is drivable as a second un-stopped run "with no further condition" — needs a ≥150 s quiet window after the first run, or the second canary dedupes against the first run's open incident (verification-harness `:50`, `:53`); shipped as 170 s.
  3. `rows.ts` E0-10 absent reason "the seeded fixture records no run_envelope row" — FALSE on this configuration: the ENVIRONMENT-SUSPECT banner was heard in E0-07's window (all three sr-empty sessions); `runs/e2e-fixture` carries the envelope row the `--e2e` arm seeds. The operator review recorded it a FINDING, routed forward (directive: a CARRY on "Full-gate regression").
  4. `rows.ts`/research: the count "is NAMED" by its `aria-label` in browse mode — FALSE: browse mode read the bare digits (S1-02 heard "heading level 1 Conductor · live / 0"); product fix shipped (visually-hidden name).
  5. The SR parser's premise that NVDA's log clock and the leg's clock agree (it windows speech by NVDA header time; verification-harness `:59` "an action timeline stamped per row lets a parser assign utterances by time window") — FALSE within one session: NVDA's log clock stepped ~2.5 s ahead for keys 8–29 of live leg #1 and every row took the previous row's speech (proof: `content info` headered 19:25:48.392Z was already in the file at 19:25:46.713Z). Remedy: the step-aware calibration from the stimulus pairs (@key ↔ NVDA `Input:`), shifting only a stretch whose excess over the baseline exceeds 300 ms; known limitation: a single slow key send can cross the line (measured 357 ms on one key of an aligned session — it shifted a few hundred ms of speech and changed no grade).
  6. layout-templates `:158` footer "height `space-sm`" — an 8px box cannot hold a 12px Label line; shipped as `--space-xs` block padding.
  7. The plan's HOLD accent "`count-hold` accent on HOLD" (layouts `:75`) as TEXT colour: `--count-hold` on the light `--color-base` computes ≈4.47:1, under SC 1.4.3's 4.5:1 for 12px text; shipped as an `aria-hidden` glyph beside the word.
- **Expected amendments (from plan):**
  - a11y-plan §4 (landmark table `:320`, the note `:323`) — `contentinfo` + the phase-line `h1` SHIPPED — carried: Counts bullet 1–2 (grep `contentinfo` a11y-plan 6 hits: `:320`, `:323`, `:548` + others; `\bh1\b` 1 hit `:323`).
  - a11y-plan §3 Screen reader test pattern — T-01 by a second un-stopped run; no browse row left without an OS key; the regrade's configuration — carried: Harness bullet 1, Counts bullet 3 (grep `browse-mode|browse mode` a11y-plan 2 lines, `:268` the site; `T-01` 0 hits in any master).
  - layout-templates §Component — Footer (status strip) + the three wireframe footer rows — SHIPPED except the unticked-ManualCheck count; §Component — Header gains the `h1` — carried: Symbols bullet 3, Spec claims 6–7 (grep `DESIGNED, NOT SHIPPED|contentinfo` layout-templates: `:48`, `:75`, `:99`, `:158`).
  - design-system §Surface: desktop-webview → Component Patterns 7 — the footer strip is built — carried: Symbols bullet 3 (grep `footer strip` design-system 1 hit `:265`).
  - test-plan §1 Untestable zones (the browse-mode zone, now empty of rows) + §6 desktop-webview row (the live `sr` leg runs a second un-stopped run, and its length) — carried: Harness bullets 1 and 3, Counts bullet 3 (grep `browse-mode|browse mode` test-plan 1 hit `:47`; `a11y:sr` test-plan `:247`, `:307`).
- **Coverage of new surfaces:**
  - `footer.footer` (contentinfo status strip) → validation n/a · instrumentation n/a (presentational over held state) · PII n/a · tests e2e (routine `main 1 · contentinfo 1` spec; SR S0-14) · a11y ✓ (landmark, text labels, not focusable, not live) · tokens design-token✓ (`--space-xs`/`--space-md`/`--border-subtle`/`--text-tertiary`/`--count-hold`; the `1px` seam as the plan and Titlebar state)
  - `h1.titlebar__label` → validation n/a · instrumentation n/a · PII n/a · tests e2e (routine single-h1 spec; SR S0-11) · a11y ✓ · tokens design-token✓ (`type-heading`, `margin: 0`)
  - `tr.cov__row[aria-labelledby]` → validation n/a · instrumentation n/a · PII n/a · tests e2e (routine row-name spec; SR S0-09/E0-05/S0-15/E0-09/S3-07) · a11y ✓ (roving contract unchanged) · tokens n/a
  - `titlebar__count` visually-hidden name → validation n/a · instrumentation n/a · PII n/a · tests e2e (SR S0-12/S1-02; the driven arm's `count()` re-pointed) · a11y ✓ · tokens n/a (`SR_ONLY` is a11y mechanics, its px values deliberate, as `App.tsx` documented)
  - `calibrateClock` (SR parser grading) → validation n/a · instrumentation n/a · PII n/a · tests ✗ at a unit tier (none exists for the SR harness members); proven by saved-log controls — two aligned records re-parsed byte-identical, the stepped live log moved onto its own rows, a doctored 500 ms pair VOIDS — and by the final legs (48/3/71 pairs, max excess 230/33/233 ms, nothing shifted) · a11y n/a · tokens n/a

## Deviations from intent
- **S1-02 review flag dropped** (plan step 7 kept it "unchanged (review row)"): the parser grades any heard review row `announced-differently` (`parse-nvda-log.ts` `grade()`), so gate 22 `not-expected none` could never pass; the row now grades on its `Scenarios completed` token. Jointly contradictory pair, resolved toward the acceptance.
- **T-01 quiet window** (170 s) and its dedupe guard — not in the plan; required by Spec claim 2. Live leg length as measured.
- **E0-08 case-sensitive stop** (`/Blocked/`): the Scenario cell `lamps-fixture-blocked` precedes the Status cell; the parser's grade stays case-insensitive (a token proxy the review checked: the capitalised label was heard).
- **Roving rows keyed on the DOM** (`roveTo`) instead of a speech stop token (plan table's `P-025`/`P-019` stops): a silent NVDA would overshoot and turn a grade into a throw.
- **Start positions re-sited** (Spec claim 1): E0-01 walks from Close window (cap 4; plan 6 from BODY), S0-11/S0-12 after S0-02 from a held test-only focus target, S1-02 from the same target, E0-07/E0-08 from Start before E0-05 (as planned).
- **S2-07 Tab ×2** (was Shift+Tab): NVDA handles Shift+Tab from its browse caret on the roll-up, whose previous focusable is the checkbox, so focus never moved (live harness defect #1).
- **Footer:** `--space-xs` block padding (Spec claim 6); the HOLD accent on a glyph (Spec claim 7); separators as plain text in one line (Reverted bullet).
- **E0-01 cap 8→4** from Close window (the plan's 6 had zero slack from BODY).
- **Harness defects** — two, one per leg (sr-error: the reset cycle ending off BODY; live: S2-07), each fixed and re-fired once on a new slot as `verification-harness.md:71` allows; neither leg hit a second. The sr-empty slot-2 session was VOID (NVDA started while a terminal held the foreground and never bound the Tauri window — its own log: "Foreground took too long to change"); the operator minimized all windows before the re-fire.
- **The operator declared slot 6 the last full re-run** of the chunk: its grades are the record.
- **scope record** (`gate.py scope`: `clean — changed 13 · listed 10 · recorded 3 (companion 1 · mechanical 0 · in-intent 1 · widening 1)`):
  - companion — `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` · serves `Titlebar.tsx` (its `count()` reads the new digits-only class) · self
  - in-intent — `crates/conductor-tauri/ui/src/srOnly.ts` · serves step 9 (the named `SR_ONLY` remedy for S0-12/S1-02) · self
  - widening — `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` · serves `screen-reader.e2e.ts` · word: "C accepted as a scope-record widening on my word (parser file outside the lists; no security boundary moves) … Revised C (step-aware) … accepted; my two conditions were set before this measurement and conflict on it, yours is the measured form. The correction is anchored only on the stimulus pairs (@key vs NVDA Input), never on graded content." — the operator (overseer)

## Decisions & corrections
- Operator (overseer) directives this chunk: every window or NVDA run is a quiet-desktop slot asked first with its length; live `sr` also needs the `:4317` grant; a hold is answered with the hold option; a second harness defect on one leg stops and reports; diff-shaped probes name `d7da5d0`.
- Operator: "A focus theft voids a row, never a pass"; "If NVDA again fails to bind to the Tauri window, call that run void and STOP and report; do not re-fire it."
- Operator: clock calibration — first stated as one per-session offset with a spread > 300 ms voiding the session; after the measurement (a step, not a constant skew) replaced by the step-aware form on the operator's word, with the conditions: anchored only on stimulus pairs, the per-key pair table in evidence, controls re-parsed before any slot (aligned byte-identical, stepped moved, a doctored stretch VOIDS).
- Operator: the probe-is-the-capture form for the live re-fire; a harness throw on it is the second defect.
- Operator: slot 6 is the last full re-run — any row still off is recorded as measured and routed forward, never fixed and re-fired again here.
- Operator review: 49 accepted as heard, S1-05 accepted with reason, E0-10 a FINDING; the test-only caret target heard as "blank" is harness noise.
- Wrap directives: the parser widening on the operator's word; the footer and row-name fixes are spec-owned (a11y-plan `:548`, `:323`); E0-10 routed as a CARRY on "Full-gate regression", not a new entry; the 357 ms single-key limit recorded as a known limitation.
- Sweep hazards found: a case-insensitive stop token `stop` is satisfied by the hint line "Ctrl+. stop" and by "Control+dot"; `Blocked` by the scenario name `lamps-fixture-blocked`; a browse-walk `reached()` read over the raw log is the only clock-free timing witness (it exposed the NVDA clock step when headers could not).
- Host facts measured: `CONDUCTOR_NVDA` is not set in any scope of the session (the operator supplies the portable path per invocation); NVDA started with a terminal in the foreground may never bind the target window even though `send-keys.ps1`'s own foreground guard passes.

## Outcome
- **Acceptance criteria** (re-asserted against the diff):
  - (tests) PREREQ closed — MET: `bash scripts/agent-run.sh run --unit` exit 0 `1136 tests run: 1136 passed, 0 skipped` and `cargo clippy --workspace --all-targets -- -D warnings` exit 0, both run in full on the base tree first, then over each changed bundle, last at the final tree.
  - (a11y) row name carries P-ID + status, roving contract unchanged — MET (routine spec green; `tabIndex` / `aria-current` / handlers untouched in the diff).
  - (a11y) one `h1`, one `main`, one `contentinfo` — MET (routine specs green).
  - (design / layouts) footer one line, tokens only, not focusable, not live; row anatomy kept — MET, with the two deviations above (padding; the HOLD accent on a glyph).
  - (a11y / tests) strict `--e2e` `[a11y] verdict asserted — 0 failed` `(expected 2)`, axe clean, ownership unchanged — MET.
  - (a11y) regrade grades all 51 rows, `rows 51 not-run-here 0`, input path per row, one configuration, review transcribed — MET.
  - (a11y) routed reds closed: `focus-nonexpected 0`, S0-09/E0-05 heard P-001 — MET.
  - (a11y) every SR row conveys its required content, `not-expected none` — MET by the grade; the review then recorded E0-10 a finding (its subject-absent reason falsified) → routed as a CARRY per the directive.
  - (a11y) the 18 findings dispositioned in the ledger — MET (gate 24 `18`).
  - (security) no boundary moves — MET: `send-keys.ps1` / `activate-window.ps1` / `wdio.conf.ts` byte-unchanged, 2 `spawnSync(`, 0 `-Command`, no 4444/4445 listener or leg process survives. The delta guard (gate 8) does NOT print nothing: it prints exactly the three scope-recorded files — see gates.
  - (security / obs) closed sets 0, host-path hygiene 0, panics 0 — MET.
  - (obs) no OTel/OTLP in the webview delta — MET (gate 12 `0`).
  - (ci) `verdict: green` — MET: CI#36770454038 on `dfa4f87`.
  - No matrix capability claimed — MET (`matrix.py show --chunk`: `claimed … 0`, pool `unclaimed 0`).
- **Gates** (by `run`, final verdicts):
  - `bash scripts/agent-run.sh run --unit` — green, exit 0, `1136 passed, 0 skipped`
  - `cargo clippy --workspace --all-targets -- -D warnings` — green, exit 0
  - `cd crates/conductor-tauri/ui && npm run build` — green
  - `… npm run typecheck:e2e` — green
  - `… npm run a11y:ownership` — green (summary + last-line atoms held)
  - driver/runtime coherence probe — green (`driver 154.0.4258.37 runtime 154.0.4258.37`)
  - `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` — green (19 passing, 2 skipped, `(expected 2)`)
  - delta guard `{ git diff --name-only d7da5d0 … } | grep -vE …` — **red · exit 0 (expected exit 1), output = exactly the three `scope-record.md` files** (`operator-hold.e2e.ts` companion, `srOnly.ts` in-intent, `parse-nvda-log.ts` widening on the operator's word). The guard's allowlist predates the recorded edits; this chunk's own edits, recorded under fix-loop Trigger 3 — reported, not a not-this-chunk's red.
  - `git diff --numstat d7da5d0 -- send-keys.ps1 activate-window.ps1 wdio.conf.ts` — green (no output)
  - `grep -c 'spawnSync(' …screen-reader.e2e.ts` — green (`2`)
  - `grep -c -- "'-Command'" …` — green (`0`, exit 1)
  - webview OTel/OTLP delta grep — green (`0`)
  - `rm -f runs/sr-leg/nvda-pass.json` — leg operator, driven by hand, exit 0
  - `npm run a11y:sr-empty` — leg operator, exit 0, `Spec Files:` TAB ` 1 passed, 1 total`
  - `npm run a11y:sr-error` — leg operator, exit 0, same atom
  - liveness probe — leg operator, exit 0, `sidecar on PATH`, last line `True`
  - `… npm run a11y:sr` (live) — leg operator, exit 0, same atom
  - evidence copy — leg operator, exit 0
  - closed-set probe — green (`0`); rows probe — green (`rows 51 not-run-here 0`); focus probe — green (`focus-nonexpected 0`); not-expected probe — green (`not-expected none`); one-configuration probe — green (`one-configuration true review true`)
  - findings-ledger grep — green (`18`); prior-chunk diff stat — green (no output); evidence host-path hygiene — green (`0`); panic sink grep — green (`0`); 4444/4445 listener count — green (`0`); leg-image census — green (`0`)
  - `gate.py hygiene` — leg operator, exit 0, `hygiene: clean — read 32 (runs 30 · evidence 2)`
  - guarded push — leg operator, exit 0, `PUSHED_SHA=dfa4f87e7e98cddae3e477966b0f8aadc5b906af`
  - `ci.py conclusion --sha HEAD --wait 1200` — leg operator, exit 0, `verdict: green · checks 3/3`
  - smoke: no boot-path change; the self-verify ran as a P2 gate.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran — pre-CI commit `dfa4f87` (the only commit since the base `d7da5d0`), pushed and read green at CI#36770454038 (recorded in `evidence/operator-legs.md`); implement's report (this session's conversation) for everything else; the operator review transcribed into `evidence/nvda-pass.json`. Post-pass artifact: `evidence/operator-legs.md` gained the pass's table (uncommitted, rides this wrap's commit).
- **Process hygiene** (implement P4's census, re-measured here: `tasklist` shows no `msedgedriver` / `conductor-tauri` / `nvda` / `tauri-driver` / `pulse-app` / `andromeda-pulse-mcp`; no 4317/4318/4444/4445 listener):

  | process | started by | final state |
  |---|---|---|
  | NVDA | each sr leg | terminated (the harness's `nvda -q`) |
  | conductor-tauri, msedgedriver, tauri-driver | each leg's driver stack | terminated (`onComplete`) |
  | pulse-app (pids 25220, 58556, 55128) | this session, on the operator's word | terminated by this session (pid + creation time; forced after a 15 s close wait) |
  | andromeda-pulse-mcp, msedgewebview2 children | the apps | none survives |
