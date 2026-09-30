# Report — 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed

**Chunk:** Unasserted keyboard and focus-visible claims closed — coverage-matrix row navigation, the first-class
shortcuts and the SC 2.4.7 active-element ring given real assertions (v3-03), after the Rust gate deferral closes
**Date:** 2026-09-30T11:16Z
**Commits:** `4bab3c6 chore(2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed): operator pre-CI commit, for
the run this chunk's verdict reads` (since `last_wrap` 2026-09-30T08:46:15Z; basis = `git log 7ee2fea..HEAD`; `7ee2fea` is
the pre-CI commit's parent and every "base" below).

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only 7ee2fea -- crates/`, 14 files, all under `crates/conductor-tauri/ui/`)
  - `src/components/CoverageMatrix.tsx` · `src/components/CoverageMatrix.css` · `src/App.tsx` ·
    `src/components/RunControls.tsx` · `src/components/RunControls.css` · `src/components/OperatorPauseDialog.tsx`
  - `wdio.conf.ts`
  - `test/a11y/accessibility.e2e.ts` · `test/a11y/operator-hold.e2e.ts` · `test/a11y/claim-ownership.ts` ·
    `test/a11y/check-claim-ownership.ts` · `test/a11y/screen-reader.e2e.ts` · `test/a11y/screen-reader/rows.ts` ·
    `test/a11y/screen-reader/nvda-pass-spec.md`
  - chunk evidence: `evidence/{prereq-baseline.md, driver-runtime.md, driven-leg.md, nvda-pass.json, operator-pass.md}`.
- **Symbols / APIs:** (webview only; no Rust symbol, no `#[tauri::command]`, no IPC method changed)
  - `CoverageMatrix` — ROVING ROW FOCUS. Component-local `current` index (0 on mount); each `<tr class="cov__row">` gets
    `tabIndex={i === current ? 0 : -1}` and `aria-current="true"` on the current row only; a `<tbody>` `keydown` maps
    ArrowDown / ArrowUp (±1, clamped, no wrap) and Home / End (first / last), `preventDefault()` on those four keys only,
    then focuses that row (native scroll-into-view reaches off-screen rows — every manifest row renders, NO
    virtualization). A row focused by any means (Tab, click) becomes current (`onFocus`). The `cov__scroll` `<div>` is NO
    LONGER a tab stop (its `tabIndex={0}` removed): the region keeps exactly ONE tab stop, the current row. The table
    keeps `aria-label="Coverage rows"`. Props unchanged; consumers `App.tsx` + `Gallery.tsx` untouched (graph: the two
    sole references).
  - `CoverageMatrix.css` — the `--color-focus` ring moved from `.cov__scroll:focus-visible` to `.cov__row:focus-visible`
    (`outline: 2px solid var(--color-focus); outline-offset: -2px`); the current row's first cell carries a
    `--border-emphasis` 2px left edge (`.cov__row[aria-current='true'] .cov__pid`), every `.cov__pid` reserving a
    transparent 2px left border so columns never shift. The ring is drawn as `outline` with NO transition (unchanged
    mechanism, as the other six `:focus-visible` rules).
  - `App` — ONE `window` `keydown` listener (installed/removed in a `useEffect`, deps `[selection, runState,
    holdPrompt, start, stop]`): returns early on `defaultPrevented` · `repeat` · Alt/Shift/Meta · no Ctrl · a raised hold
    (`holdPrompt !== null`, the dialog owns its keys). **Ctrl+Enter** with `selection !== null && runState !== 'live'` →
    `preventDefault()` + the SAME `start()` the Start button calls; **Ctrl+.** with `runState === 'live'` →
    `preventDefault()` + the SAME `stop()`. No other key bound; Ctrl+W / Alt+F4 and single letters untouched. Ctrl+Enter
    inside the picker input only selects (cmdk prevents Enter first). No command, capability or listener added beyond
    this in-webview DOM listener.
  - `RunControls` — Start `aria-keyshortcuts="Control+Enter"`, Stop `aria-keyshortcuts="Control+."`; button TEXT
    unchanged (`Start` / `Stop`). NEW visible hint line: a sibling `<p class="run-controls__hint type-data">` inside the
    `run-controls` group reading exactly `Ctrl+Enter start · proceed   Ctrl+. stop   Esc abort` — not focusable, no
    live region.
  - `RunControls.css` — `.run-controls__hint { margin: 0; align-self: center; color: var(--text-tertiary);
    white-space: pre }`.
  - `OperatorPauseDialog` — `AlertDialog.Content` `onKeyDown`: Ctrl+Enter (no Alt/Shift/Meta) → `preventDefault()` +
    `onProceed()`; Proceed `aria-keyshortcuts="Control+Enter"`, Abort `aria-keyshortcuts="Escape"`. Escape stays Radix's
    own dismiss → `onOpenChange(false)` → `resolveHold('NoGo')`; `onCloseAutoFocus` / `restoreFocusTo` unchanged.
  - `wdio.conf.ts` — inside the existing `driven` branch: re-creates `runs/driven/scenarios` clean (`rmSync` +
    `mkdirSync` over the harness constant, never an operator handle), copies EXACTLY `scenarios/halo-breathing-encoding.toml`
    + `scenarios/halo-hue-encoding.toml` (the catalog's only two `[[checklist]]` scenarios; `grep -lE '^\[\[checklist\]\]'
    scenarios/*.toml` → 2), and sets `appEnv.CONDUCTOR_SCENARIOS_DIR = 'runs/driven/scenarios'` (repo-relative). One
    driven run therefore reaches two holds (halo-breathing first, halo-hue second, sorted order) behind ONE canary. The
    `UNSAFE_PATH` / `nativeDriver()` / `exitUnresolvedHandle` guard lines are byte-unchanged (probe below).
  - Test-side: `claim-ownership.ts` `Claim`'s `owned` variant gains an optional `inPlace: string`;
    `check-claim-ownership.ts` FAILS an owned row whose owner is not in `CI_SUITES` and whose `inPlace` is empty/absent
    (`carve-out with no statement of what CI gates in its place`) and prints `; in its place: {inPlace}` on each
    carve-out line (summary format + last line unchanged). `screen-reader.e2e.ts` gains `expectActiveCoverageRow()`
    (bounded `waitUntil`: active element is a `cov__row` with `tabindex="0"` + `aria-current="true"` inside the table
    named `Coverage rows`), used at S0-09, E0-05 and E0-09; `expectActiveScrollRegion('Run report rows')` (E0-06) kept.
  - Env vars: **no new handle.** `CONDUCTOR_SCENARIOS_DIR` (existing) is now ALSO set by `wdio.conf.ts` for the `driven`
    suite, value `runs/driven/scenarios` (it already set it for `sr-empty` / `sr-error`).
  - Artifact paths: `runs/driven/scenarios/` (new, gitignored, harness-owned, re-created per driven run); the driven
    spec READS the existing backend sink `runs/driven/logs/conductor-tauri.jsonl` (lines added after its own baseline
    count) — no new writer.
  - Ports / processes: none new. The driven and SR legs use the existing 4444/4445 driver stack; `:4317` only via the app's
    own canary in the operator slot.
- **Crates / modules:** none added / removed. Rust delta 0 (basis: plan entry 14's guarded-paths probe, `git diff
  --numstat 7ee2fea -- crates/conductor-*/…` + `git ls-files --others …` → no output, green).
- **Dependencies:** none — `package.json` / `package-lock.json` / `Cargo.toml` / `Cargo.lock` unchanged (same probe).
- **Schema / config:** none. The claim enumeration's data shape grew one optional field (`inPlace`) — a test-side type,
  not a runtime schema. No violation-schema / obs-log change.
- **Spec-master edits:** none (implement authored nothing; P2 applies below).
- **Counts / qualifiers moved:**
  - Claim enumeration (`npm run a11y:ownership`): `10 claims · 5 owned (3 operator-local, carve-out) · 2
    n/a-by-construction · 3 recorded gaps` (base) → `11 claims · 9 owned (5 operator-local, carve-out) · 2
    n/a-by-construction · 0 recorded gaps` (measured, gate entry 5). The three `unasserted` rows moved to `owned` and a
    NEW `visible focus ring on the active element` · `SC 2.4.7` row was added.
  - Routine arm tally: `13 passing · 2 skipped` → `16 passing · 2 skipped (expected 2)` (measured, gate entry 7). The
    expected-skip SET is unchanged at two. Masters baking the count: `grep -E '13 passing|16 passing'` over the seven →
    0 hits (the §9 moving-count rule holds).
  - Coverage region tab stops: one before (the scroll `<div>`), one after (the current row) — the Tab-stop COUNT is
    unchanged; the stop's element moved from `DIV` to `TR`.
- **Dev-tool versions:** none — msedgedriver (the driver, `CONDUCTOR_MSEDGEDRIVER`) re-read at 154.0.4258.37 against
  the WebView2 runtime 154.0.4258.37 on the dev host (evidence/driver-runtime.md); NVDA re-read at 2026.2 (the SR record's
  `nvda_version`).
- **Harness / gate surface:**
  - Routine arm (`accessibility.e2e.ts`, CI's `a11y` job): +3 `it()` blocks, placed after the Operable pair and before
    the two subject-absent markers — `the focused control shows a visible --color-focus ring and the control it left
    shows none (SC 2.4.7)` · `coverage-matrix rows navigate by Arrow keys and Home/End through one tab stop, the current
    row marked aria-current with a --border-emphasis edge (idle-with-report)` · `the console declares its shortcut map:
    aria-keyshortcuts on Start and Stop and a visible hint line` (the last asserts the WHOLE declared
    `aria-keyshortcuts` set equals `Start=Control+Enter | Stop=Control+.`, so no single-letter or platform binding can
    be declared). No CI workflow edit; `A11Y_EXPECTED_SKIPS=2` unchanged.
  - Driven arm (`operator-hold.e2e.ts`): the single `it` is REWRITTEN as one live run over the trimmed catalog —
    Ctrl+Enter from a matrix row starts it; ArrowDown ×2 / ArrowUp move focus while live; hold 1: trap, 8 Tabs +
    Shift+Tab containment, Space toggles a checklist row, Escape → No-Go, focus restored to the invoking ROW (compared by
    P-ID); the count update to `1` leaves focus on that row; Ctrl+. stops it; hold 2: Ctrl+Enter proceeds. Decisions are
    graded from the backend log (exactly one `: No-Go (` and one `: Go (` among added lines, plus `run aborted by the
    operator`), never from the dialog closing.
  - Ownership checker: the new carve-out-statement failure mode (above).
  - SR harness: S0-09 / E0-05 / E0-09 re-aimed at the focused coverage row (`rows.ts`: S0-09 and E0-05 → item `coverage
    matrix current row`, node `tr[tabindex=0][aria-current=true] in table[aria-label="Coverage rows"]`, `tokens:
    ['P-001']`, `forbidden: ['P-002']`; E0-09's node names the row as its Shift+Tab landing, tokens/class unchanged;
    `nvda-pass-spec.md` restates the three rows).
- **Cross-project / external claims:**
  - CI: run **36705777679** on `4bab3c6` (the pre-CI commit), `verdict: green · checks 3/3` (Frontend, Rust, A11y
    `success`), read through `ci.py conclusion --sha HEAD --wait 1200` (evidence/operator-pass.md) and overseer-read via
    `gh run view` (operator relay). This wrap's own commit adds to that tree.
  - Pulse (the SUT of the live slot): `andromeda-pulse` `target/release/pulse-app.exe` sha256
    `9e51d1d92e80fdc0b998fe5e1c65fbbd9c5eef4dd9e6b7c4883c5ccad1bf9ab4` — built, per pulse-builder (relayed), from Pulse
    `71f3369` PLUS its uncommitted P-027 discovery fix; measured here: HEAD `71f3369` with uncommitted `crates/triage/`
    edits (`baseline/mod.rs`, `lifecycle/mod.rs`, `lifecycle/registry.rs`). NOT a committed Pulse state. Sidecar
    `andromeda-pulse-mcp.exe` sha256 `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634`. Posture read from
    Pulse's own log: `inference_mode: "deterministic"`, workspace basename `drivenkeys`, OTLP bound `127.0.0.1:4317`.
  - NVDA 2026.2 / WebView2 154.0.4258.37 behaviour (below, Insufficient fixes).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):**
  - **The SR re-aim (plan step 13) shipped and did NOT yield the NVDA regrade.** What it resolved: the SR harness's DOM
    half follows the new tab stop — `expectActiveCoverageRow()` held at S0-09 and E0-05 in every run. What it did not:
    S0-09 and E0-05 grade `not-announced` (E0-09 `not-run-here`, browse class), because on this host NVDA receives NO
    webview focus event after the window is activated — EVERY focus row of both subjects is `not-announced` (E0-02 …
    E0-06, S0-01 / S0-02 / S0-04…S0-07 / S1-03 / S1-04 / S2-03 / S2-04 / S3-02), rows this chunk never touched included,
    while the live-region rows are heard (S1-01, S2-01, S2-08, S3-01, S3-05 `announced-as-expected`); the record's
    `attach_observed.empty` is `false`. **Basis — two-sided:** the BASE `7ee2fea` bundle (its `ui/src` built into
    `ui/dist`, the release binary relinked; zero Rust delta) under the same `sr-empty` leg is silent from `@foreground`
    onward in exactly the same way. Around activation NVDA speaks "Landscape" ×3 (a display event) in the chunk runs.
    `hypothesis:` the WebView2 runtime moving to 154.0.4258.37 is the variable that changed — untested. Owner: the
    route entry minted at P5 (operator relay §2).
  - A separate, recorded finding (not the cause above): in the first two `sr-empty` runs (10:09Z, 10:12Z, before the
    slot) the OS foreground moved from Conductor to the **pulse-builder session's Windows Terminal**, and in the second
    first to a **WebView2 host console window** whose title (the runtime's exe path) and "DevTools listening on
    ws://127.0.0.1:…" NVDA read aloud (the latter also on the base bundle — pre-existing).
  - The SR record's `build_commit` stamps HEAD (`7ee2fea`) although the bundle under test carried this chunk's
    uncommitted delta (evidence/driven-leg.md) — a stamp-provenance caveat of `nvda-pass.json`.
- **Spec claims disproved by measurement:**
  1. **Plan entries 8 / 10 / 11 (`expect` atom `contains Spec Files: 1 passed, 1 total`).** As planned: a single space
     after `Spec Files:`. As measured: wdio prints `Spec Files:` + TAB + space (`od -c` of entry 8's log: `S p e c   F i
     l e s : \t   1   p a s s e d`; the overseer read `runs/a11y-e2e.log` with `cat -A`: `Spec Files:^I 1 passed`). All
     three legs passed underneath (exit 0, `1 passing` each), so the atom was wrong, not the behaviour. Also stated in
     `.claude/rules/verification-harness.md:58` (a Session Additions entry — preserve-verbatim) and quoted in
     `verification-matrix.json#v3-03`'s acceptance. Masters: `grep -cE 'Spec Files: 1 passed'` over the seven → 0 hits.
  2. **a11y-plan §5 "No suite asserts … today"** at `a11y-plan.md:354`, `:356`, `:370` (grep `No suite asserts` → 3
     hits) — now false: each claim is asserted (routine / driven, above).
  3. **"off-screen virtual-scrolled rows" / "Virtual-scroll for the full wall"** — no virtualization exists (every
     manifest row renders; `CoverageMatrix.tsx` `rows.map`); off-screen rows are reached by focus-scroll. Sites: grep
     `virtual-scroll|virtual scroll|Virtual-scroll` → a11y-plan 6 (`:86 :138 :262 :334 :354 :549`), design-system 1
     (`:257`), layout-templates 2 (`:42 :129`).
  4. **design-system §Depth Strategy / §Motion — the focus ring as a `0 0 0 2px` `box-shadow` with a `--motion-micro`
     fade-in**; the code draws `outline` with NO transition on all seven `:focus-visible` rules (scope.md, verified at
     P3; unchanged by this chunk, now the ASSERTED mechanism: the SC 2.4.7 spec reads computed `outlineStyle` /
     `outlineColor`). Sites: `box-shadow` design-system `:124`; `fade-in` design-system `:160`, a11y-plan `:160 :422 :424
     :505 :541`.
  5. **The design-system / layout-templates k9s single-letter key register** (`k9s` → design-system `:27 :34 :261`,
     layout-templates `:114`) — the shipped map is modifier-based: Ctrl+Enter / Ctrl+. / Escape (P4 fork, overseer);
     single letters stay free for the picker's type-ahead and NVDA browse mode.
  6. The scope CARRY "the committed enumeration already names all four" — falsified at take-up (no SC 2.4.7 row);
     already recorded in `scope.md`; the row is added (Counts above).
- **Expected amendments (from plan):** (site basis: `grep -nE {pattern}` over the seven masters, run at this P1)
  - a11y-plan §5 run-console-live / idle-with-report / Per-surface keyboard shortcuts (retire "No suite asserts", name
    owners driven / routine / driven + the shipped mechanism, restate virtual-scroll) — carried: Symbols / Harness
    bullets + disproved 2, 3. `No suite asserts` a11y-plan 3 hits; virtual-scroll a11y-plan 6 hits.
  - a11y-plan §11 Strategy (extend "what gates in its place" to the two new driven-owned claims: live row navigation ←
    the routine row-navigation spec on the same component; shortcuts ← the routine key-map spec) — carried: Harness
    bullet (`inPlace` + checker). `gates in its place` a11y-plan 4 hits (`:355 :362 :366 :517`).
  - a11y-plan §4 → Coverage matrix (the current row is `aria-current`) — carried: Symbols. `aria-current|aria-selected`
    a11y-plan hits `:137 :334 :356`.
  - design-system §Navigation Pattern + §Component Patterns #5 (k9s single-letter register → the modifier map) —
    carried: disproved 5. k9s design-system 3 hits; `start/stop/proceed/abort` design-system `:269`.
  - design-system §Component Patterns #3 ("Virtual-scroll for the full wall" → shipped truth) — carried: disproved 3.
    design-system `:257`.
  - design-system §Depth Strategy + §Motion (ring = `outline`, no fade) — carried: disproved 4. `box-shadow` `:124`,
    `fade-in` `:160`, `motion-micro` `:161 :169 :229 :255`.
  - layout-templates §Component — Primary navigation (the key map + the hint line) and §Component — Primary content
    block 1 (virtual-scroll → shipped; the keyboard-selected row) — carried: Symbols + disproved 3, 5. k9s /
    start-stop layout-templates `:114`; virtual-scroll `:42 :129`.
  - test-plan §6 desktop-webview row (the driven arm seeds its own two-scenario catalog under `runs/driven/scenarios`
    and asserts the shortcuts and live row navigation) — carried: Symbols (`wdio.conf.ts`) + Harness. `runs/driven` /
    `CONDUCTOR_SCENARIOS_DIR` test-plan `:307`.
  - architecture §Occupied Resources → Environment variables (`CONDUCTOR_SCENARIOS_DIR` now also set by `wdio.conf.ts`
    for the driven suite) — carried: Symbols → Env vars. `CONDUCTOR_SCENARIOS_DIR` architecture `:186 :257`;
    `runs/driven` architecture `:172`. Headroom to be measured with `scripts/arch-registry-check.py measure` at apply.
- **Coverage of new surfaces:**
  - `CoverageMatrix` roving row focus + `aria-current` row → validation n/a · instrumentation n/a (frontend `console`
    only, no new op) · PII n/a · tests e2e (routine row-nav spec; driven live run) · a11y SC 2.1.1 / 2.4.3 / 2.4.7 kbd +
    focus ✓ · tokens design-token ✓ (`--color-focus`, `--border-emphasis`; hex probe entry 16 → 0).
  - Console shortcut listener (Ctrl+Enter / Ctrl+.) → validation guards = the buttons' own (`canStart && !running`,
    `running`) · instrumentation n/a · PII n/a · tests e2e (driven, real keypresses; routine key-map declaration) · a11y
    kbd ✓ (`aria-keyshortcuts`) · tokens n/a.
  - Dialog Ctrl+Enter Proceed → validation n/a · instrumentation: the existing `operator-checklist hold resolved` backend
    line ✓ · PII n/a · tests e2e (driven hold 2) · a11y kbd ✓ · tokens n/a.
  - Hint line `run-controls__hint` → a11y text, not focusable, no live region ✓ · tokens `--text-tertiary` ✓ · tests e2e
    (routine key-map spec).
  - The SR screen-reader half of all of the above → a11y `unrunnable-here` today (NVDA hears no focus events on this
    host; base reproduces).

## Deviations from intent
- **Plan step 10.3 widened within intent:** the key-map spec asserts the WHOLE declared `aria-keyshortcuts` set, not
  only Start/Stop + the hint — so the acceptance's "Ctrl+W / Alt+F4 and single letters are never bound" clause is
  actually tested (a bound key must be declared to be discoverable; an undeclared one is outside what this spec can
  see). Justification: the plan's acceptance names that clause and step 10.3's body did not reach it.
- **Driven spec counts focusables via `browser.execute`**, not `(await $$(…)).length` (types as `Promise<number>`
  under this wdio; `typecheck:e2e` red once, fixed).
- **SR regrade not obtained** (plan step 14 / acceptance "the committed nvda-pass.json grades S0-09 and E0-05
  announced-as-expected") — see Insufficient fixes; basis two-sided; owner the P5 entry.
- **Extra measurement outside the plan:** the base-bundle SR control — a detached `git worktree` at `7ee2fea` with a
  `node_modules` junction to the real one, its `ui/src` built into the real `ui/dist`, the release binary relinked, one
  `sr-empty` run, then the junction removed (non-recursively), the worktree removed, and the chunk bundle rebuilt +
  relinked (the served JS carries `run-controls__hint` again). No tracked file touched.
- **Plan atoms 8 / 10 / 11 (operator directive, this wrap):** the literal `contains Spec Files: 1 passed, 1 total` as
  planned vs `Spec Files:` + TAB + space as measured — corrected in `plan.md` by this wrap before its light gate (P7), on
  the relay's direction, so the gate reads the real output.
- **`sr-empty` fired four times** (twice before the slot with focus theft, once on the quiet desktop, once as the base
  control) — each preserved in the session scratchpad; only the chunk-bundle run's record is committed.
- scope record: none — `gate.py scope` clean (`changed 14 · listed 14 · recorded 0`), 0 recorded.

## Decisions & corrections
- Operator (overseer): granted the `:4317` slot; directed the agent to launch `pulse-app` itself (the standing memory);
  kept pulse-builder quiet for the SR legs; accepted the readings; directed the operator pass (hygiene → pre-CI commit
  → push → CI read) by the agent; directed the atom fix and a new route entry for the SR host condition (relay §1-§2).
- Operator rule (this session): a row silenced by focus theft is a finding naming the window that took focus, never a
  pass.
- Sweep hazards found: (a) a `contains` atom written from a spec line with collapsed whitespace (the runner prints a
  TAB) — a literal copied from a rule file's prose cannot be trusted for whitespace; read the bytes (`od -c` / `cat
  -A`). (b) A BOM-less `.ps1` carrying an em dash fails to PARSE under Windows PowerShell (`Unexpected token`), so the
  launch did nothing — keep scratch `.ps1` ASCII. (c) `mklink /J` through MSYS mangles the switch (`Invalid switch`);
  use PowerShell `New-Item -ItemType Junction`, and remove a junction with a non-recursive `Directory.Delete(path,
  $false)` before removing the directory that holds it. (d) `pulse-app`'s WebView2 child tree outlives the forced stop
  of its parent for a few seconds — a census taken immediately after reads it as orphans.
- NVDA's attach verdict is the tell: with `attach_observed.empty: false` a silent focus row is not a product finding.

## Outcome
Acceptance criteria (re-asserted against the diff):
- (tests) PREREQ closes — MET: `bash scripts/agent-run.sh run --unit` (1136/1136) + clippy green on `7ee2fea` before any
  edit (evidence/prereq-baseline.md) and again over the delta (1136/1136, 48.3 s; clippy exit 0).
- (a11y) `v3-03` ownership — MET: `npm run a11y:ownership` exit 0, `11 claims · 9 owned (5 operator-local, carve-out)
  · 2 n/a-by-construction · 0 recorded gaps`, last line `ownership: every claim resolved`; the inverse control (one
  `inPlace` removed in a scratch copy) exits 1 naming the claim.
- (a11y) SC 2.4.7 on the routine arm — MET (the ring spec passes inside strict `--e2e`).
- (a11y) idle-with-report row navigation — MET (the row-nav spec: one stop, DOM-order moves, End reaches an outside
  row, `aria-current` + `--border-emphasis` edge distinct from the ring, axe clean).
- **Affordance criterion** (a11y) shortcuts as real keypresses in ONE live run — MET: `npm run a11y:driven` exit 0,
  `1 passing (4m 2.1s)`, `Spec Files:<TAB> 1 passed, 1 total`; backend lines `…tauri-dialog: No-Go (1 checklist
  item(s))`, `…: Go (1 checklist item(s))`, `run aborted by the operator` (evidence/driven-leg.md).
- (a11y) SR regrade in this chunk — **UNMET**: both legs printed `Spec Files:<TAB> 1 passed, 1 total` and both
  subjects' `recorded_at` postdate their leg start (empty 10:46:39Z vs 10:45:45Z; live 10:49:56Z vs 10:47:15Z), but
  S0-09 / E0-05 grade `not-announced` → red — not this chunk's (below) → owner the P5 entry.
- (a11y) routine arm strict, skip SET unchanged — MET: `[a11y] verdict asserted — 0 failed · 2 skipped (expected 2)`.
- (layouts / design) key map declared — MET (key-map spec).
- (security / arch) against `7ee2fea` — MET: guarded-paths probe no output; `wdio.conf.ts` guard diff 0 lines.
- (obs / design) no `@opentelemetry`, no literal hex — MET (probe entry 16 → 0).
- (security) evidence 0 host-path tokens + census 0 survivors — MET (entries 17 and 13).
- (tests) CI green on the pre-CI commit — MET: CI#36705777679 on `4bab3c6`, green 3/3.

Gates (implement run dir `.andromeda/runs/2026-09-30T09-58-22-implement`; entries named by `run`):
- `bash scripts/agent-run.sh run --unit` — green (exit 0), base AND delta.
- `cargo clippy --workspace --all-targets -- -D warnings` — green, base AND delta.
- `cd crates/conductor-tauri/ui && npm run build` — green.
- `cd crates/conductor-tauri/ui && npm run typecheck:e2e` — red once (TS2365 in the new driven spec), fixed → green.
- `cd crates/conductor-tauri/ui && npm run a11y:ownership` — green (all three atoms).
- the driver/runtime coherence probe — green, `driver 154.0.4258.37 runtime 154.0.4258.37`.
- `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` — green (exit 0 · `[a11y] verdict asserted — 0 failed`
  · `(expected 2)`).
- `cd crates/conductor-tauri/ui && npm run a11y:sr-empty` — `red · contains 'Spec Files: 1 passed, 1 total'` with exit
  0 and `1 passing` — the atom defect (disproved 1), corrected in the plan by this wrap.
- `… conductor preconditions` (non-priming probe) — green, `[PRECONDITION] every live-Pulse precondition is satisfied`
  (fired in the operator slot).
- `… npm run a11y:driven` — `leg = 'live'`, driven by hand in the slot: exit 0, `1 passing`, `Spec Files:<TAB> 1
  passed, 1 total`.
- `… npm run a11y:sr` (live) — `leg = 'live'`, driven by hand in the slot: exit 0, `1 passing (2m 31s)`, `Spec
  Files:<TAB> 1 passed, 1 total`.
- `jq … evidence/nvda-pass.json` (S0-09 / E0-05) — `red — not this chunk's: the base 7ee2fea bundle under the same
  sr-empty leg is silent from @foreground in the same way (two-sided control, evidence/driven-leg.md Findings 2) → the
  P5-minted route entry "the screen-reader pass grades again on this host"`. Printed `E0-05 not-announced` / `S0-09
  not-announced`.
- `tasklist | grep -ciE …` census — green (0).
- guarded-paths probe · `wdio.conf.ts` guard probe · OTel/hex probe · evidence host-path sweep — green.
- `gate.py hygiene` (`leg = 'operator'`) — fired by hand on the operator's word: `hygiene: clean — read 35`.
- push (`leg = 'operator'`) — `PUSHED_SHA=4bab3c6c2ce1a5d9544e550694ab8a532d232b2b`.
- `ci.py conclusion --sha HEAD --wait 1200` (`leg = 'operator'`) — `verdict: green · checks 3/3 · runs
  CI#36705777679 completed/success`.
- Smoke: no boot-path change; the self-verify ran as the P2 `--e2e` gate, green.

Watches: none folded.

Outcome basis: the operator pass ran (`4bab3c6`, the only commit since `7ee2fea`); gate verdicts above rest on
implement's runs plus the final HEAD's CI run recorded in `evidence/operator-pass.md`; the plan-atom correction and the
route entry are operator directives between implement and this report (relay `conductor-wrap-63-2026-09-30.md`).

Process hygiene (implement P4's census, re-measured at the slot's end): `pulse-app` pid 20656 — started by this
session on the operator's word, terminated (forced after `CloseMainWindow` did not exit in 15 s); its msedgewebview2
tree (root 25780 + 4) — terminated on its own ~5 s later; every leg's tauri-driver / node / msedgedriver /
conductor-tauri / msedgewebview2 — terminated by wdio `onComplete`; NVDA per SR run — terminated by `nvda -q`; six
`msedgewebview2` under `SearchHost` — pre-existing, not ours. Final census identical to the baseline; no LISTENING socket
on 4317/4318/4444/4445.
