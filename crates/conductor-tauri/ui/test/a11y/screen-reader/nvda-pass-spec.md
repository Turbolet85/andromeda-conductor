# Screen-reader pass spec — NVDA over the Conductor webview

The per-state screen-reader pass the a11y plan requires (a11y-plan §3 _Screen reader test pattern_ ·
§3 Bootstrap `screen-reader-test-spec-setup`), written against the DOM the release bundle actually ships.

## Purpose and standing

- **Supplemental, never sole.** The automated baseline — axe-core, the colorjs.io token pairs, the WebdriverIO
  keyboard and focus assertions of `accessibility.e2e.ts` and `operator-hold.e2e.ts` (`v2-22`) — is the
  conformance evidence. This pass verifies runtime announcement QUALITY only and claims no WCAG conformance
  (a11y-plan §2 Agent-runnable invariants · §11 Universal).
- **Two columns, two kinds of truth.** `Expected NVDA output` is a transcribed expectation — what must be
  conveyed (name · role · state · text), phrased in NVDA's default verbosity where that phrasing is stable —
  never a measurement. `Heard` (in the evidence) is the only measured half, and it is NVDA's own speech log.
- **Agent-driven, operator-reviewed.** `screen-reader.e2e.ts` drives the rows below over the one WebdriverIO +
  tauri-driver stack while a portable NVDA logs every utterance (`-l 12`); `parse-nvda-log.ts` assigns the
  utterances to rows by the leg's action timeline and grades them on the row's required tokens. The record
  it writes (`nvda-pass.json`) is reviewed by the operator, and the review is transcribed into it. Grading is
  about CONTENT conveyed, never NVDA's connective wording, which is version-bound.
- **Row classes.** `focus` — a focus change; `live` — a live-region, alert or dialog-open event; `browse` —
  static text NVDA reaches only through its browse-mode commands (headings, landmarks, prose, table cells),
  which NVDA's own keyboard hook intercepts. The leg attempts every class and records the outcome.
- **Two input paths.** Tab, Shift+Tab and the browse keys (`h` · `d` · ArrowDown) ride the OS input path —
  `send-keys.ps1`, `SendInput`, sent only while Conductor holds the foreground — which NVDA's keyboard hook
  sees as physical keys. Picker text, Enter, Space, Escape and the picker's arrows stay WebDriver-injected.
  The split is measured, not assumed: under the leg's own driver launch, injected focus moves after the
  window's first burst were heard 0 of 9 while OS-path ones were heard 5 of 5 (2026-09-30, WebView2
  154.0.4258.37, NVDA 2026.2, Windows 26200.9457). Each row records its `input` (`os` · `webdriver` ·
  `mixed` · `none`) from the keys sent inside its window. A silent `browse` row driven on the OS path is
  `not-announced` on the agent arm; one with no OS key in its window falls to the operator's manual arm, and
  the evidence records the ARM per row.
- **Outcomes** (closed set): `announced-as-expected` · `announced-differently` · `not-announced` ·
  `not-run-here` · `subject-absent`. A not-announced or not-run-here row is a recorded FINDING — never a pass,
  never a hard failure. Rows whose node does not ship are `subject-absent` and never omitted: the spec names
  what SHIPS, and every plan-vs-shipped gap is a row.
- **VoiceOver (macOS) and Orca (Linux)** have no surface on this Windows host. They are declared
  not-runnable-here — no rows, no columns, no claimed pass. macOS stays manual-pass-only by the plan's own
  reading (a11y-plan §1).

## Preconditions

- A **portable NVDA** named by `CONDUCTOR_NVDA` (its version is recorded in the evidence; its path never is).
  The leg starts it as `nvda -m --no-sr-flag -l 12 -f <log>` before the app window exists and quits it with
  `nvda -q` afterwards; unset or not-a-file ⇒ the `sr*` suites skip at exit 0 with a recipe.
- The **release bundle** built `npm run build` then `cargo build --release -p conductor-tauri --features
  tauri/custom-protocol` (a bare `--release` opens `devUrl`, never the bundle), attached through the native
  WebDriver named by `CONDUCTOR_MSEDGEDRIVER` (matching the WebView2 Runtime major).
- The **WebView2 Runtime** version, read from the EdgeUpdate registry key by the parser and recorded.
- For the live subject, the **driven arm's live-Pulse form** (test-plan §6 / §9 · `verification-harness.md`):
  `pulse-app` up on a FRESH data dir with `ANDROMEDA_PULSE_MCP_ENABLED=true` and
  `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`; `ANDROMEDA_PULSE_DATA_DIR` equal to that dir; the sidecar's
  directory on `PATH` and `andromeda-pulse-mcp` resolving by NAME before launch (unresolved ⇒ `[BLOCKED]` in
  ~2 ms, indistinguishable at row level from a SUT gate failure); a quiet window ≥120 s idle + 30 s resolver
  tick after ANY prior canary, and never `agent-run boot` first (its canary primes the incident the app's own
  preflight then dedupes against).
- **Attach.** Whether NVDA attaches to the WebView2 window while WebDriver holds the session is measured by
  the first `focus` rows: a speech log silent through them means no attach (`attach_observed: false`), and
  the whole subject falls to the manual arm.

## The four run states — as shipped

`Titlebar.tsx` enumerates `RunState = idle · live · hold · aborted`, each with its own phase-line label in the
phase-line span, which is `aria-live="assertive"` ONLY while entering `hold` and `polite` otherwise (an always-assertive region cancelled the post-hold focus-restore announcement): `Conductor · idle` · `Conductor · live` · `Conductor · HOLD — operator pause` ·
`Conductor · aborted`. The terminal stages `done` / `blocked` settle the label to `idle` with the run-report
section populated, so "idle with a report" is a SUB-STATE of `idle`. The a11y-plan names the four as
`idle / live / HOLD / report-terminal` — its `report-terminal` is that sub-state, and it omits `aborted`, which
ships. Both are findings for the plan, not choices of this spec. The titlebar count is a SCENARIO COUNTER
(`RunEvent.count`), not a clock: `00:00:00` is the idle placeholder and `0` / `1` / `2` the live values.

## Stimulus recipes (firing form and stop form)

Three suites, one spec, one subject each. Every value in `<…>` is the operator's — never committed, never
echoed into an artifact. Set the whole block in ONE paste and echo it back before launching.

| Suite | Subject | App env chosen at the spawn site | What it produces |
|---|---|---|---|
| `npm run a11y:sr-empty` | `empty` | `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/empty` · `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` (the committed lamps journal, re-seeded clean) | the empty-catalog prose, the fixture's report rows and matrix lamps; no run, no canary |
| `npm run a11y:sr-error` | `error` | `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/bad` · `CONDUCTOR_RUNS_DIR=runs/sr-leg/runs` | the `role="alert"` load-error prose; no run, no canary |
| `npm run a11y:sr` | `live` | `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` (from the shell: `halo-breathing-encoding` + `halo-hue-encoding` + one short third scenario such as `high-severity-log-capture`, which never executes) · `CONDUCTOR_RUNS_DIR=runs/sr-leg/runs` (emptied per session) | idle → `Start` → live → hold #1 (P-026: Tab · Space · Enter on Proceed) → live → `Stop` during the second scenario → aborted → hold #2 (P-025: Escape) → the backend `Aborted` stage before the third scenario → the report reload → the quiet window → a SECOND, un-stopped run (Proceed at both holds, the third scenario executes) → `Done` / idle (T-01); TWO canaries, the second only after the first run's incidents idle out. The third scenario is load-bearing: `drive_run` checks the abort flag only BEFORE a scenario starts, so a `Stop` during the last scenario ends in `Done` / idle (measured 2026-09-02), never `Aborted` |

Firing form (PowerShell, from the ui package; the sidecar dir on `PATH` first):

```
$env:ANDROMEDA_PULSE_DATA_DIR = "<the live Pulse's data dir>"
$env:ANDROMEDA_PULSE_MCP_ENABLED = "true"
$env:ANDROMEDA_PULSE_L4_DETERMINISTIC = "true"
$env:PATH = "<the Pulse repo's target\release dir>;$env:PATH"
Get-Command andromeda-pulse-mcp
$env:CONDUCTOR_MSEDGEDRIVER = "<msedgedriver.exe>"
$env:CONDUCTOR_NVDA = "<the portable nvda.exe>"
$env:CONDUCTOR_SCENARIOS_DIR = "runs/sr-leg/scenarios"
npm run a11y:sr-empty; npm run a11y:sr-error; npm run a11y:sr
```

Expected timeline for `sr`: hold #1 ~50-60 s after `Start` (the in-run canary poll; 54 s measured on the driven
arm); `Stop` flips the label client-side at once and the backend emits `Aborted` at the next scenario boundary,
after hold #2. A re-run of `sr` needs the quiet window again. Stop forms: NVDA `-q` (the leg's, in
`onComplete`); the driver tree via `tauriDriver.kill()` (the leg's); on an interrupted run
`taskkill /F /IM conductor-tauri.exe /IM msedgedriver.exe`; `pulse-app` is the operator's — the census ends
with `pulse-app: left running — operator stops it`.

Every row's `Heard`, `Result` and `Arm` live in the evidence (`nvda-pass.json`), keyed by the row id below.

## Rows — live subject

### S0 · idle (the trimmed catalog, an empty runs dir)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S0-11 | browse | titlebar phase line at rest — the single h1 | after S0-02 (NVDA binds the document at the first OS-path Tab), ArrowDown until "idle" (browse mode, from a test-only blank focus target held first in `<body>` for S0-11/S0-12 — the browse caret follows DOM focus and this text precedes every focusable; focus then returns to Close window in-page; cap 4) | banner › h1[aria-live="polite"] "Conductor · idle" | "Conductor · idle" heading level 1, read by navigation — the region announces changes, not initial content | SC 4.1.3 · SC 1.3.1 |
| S0-12 | browse | titlebar count placeholder | ArrowDown until "Scenario count" (browse mode; cap 3) | banner › span[aria-live="polite"] › visually-hidden text "Scenario count: no run yet" + aria-hidden "00:00:00" (browse mode does not read an aria-label on a role-less span) | the count is NAMED — "Scenario count: no run yet" — never bare digits | SC 4.1.2 |
| S0-01 | focus | Minimize window control | Tab | button[aria-label="Minimize window"] | "Minimize window" + button | SC 4.1.2 |
| S0-02 | focus | Close window control | Tab | button[aria-label="Close window"] | "Close window" + button | SC 4.1.2 |
| S0-14 | browse | landmark list — banner · main · regions · contentinfo | `d` until "content info" (browse-mode next landmark, from Close window; cap 6) | header (banner) · main · section[aria-label] · footer (contentinfo) | "main landmark" … "content info landmark" | SC 1.3.1 |
| S0-03 | focus | scenario / suite picker | Tab | input[role="combobox"] "Scenario or suite picker" (cmdk), first option active | "Scenario or suite picker" + combo box, editable, the active option | SC 4.1.2 |
| S0-16 | browse | picker filter-miss prose | type `zzz`, then Backspace ×3 | div[aria-live="polite"] › p "No scenarios match." (cmdk Empty replaced — its role="presentation" is set after the prop spread) | ANNOUNCED when the filter stops matching — the region is mounted before the text arrives | SC 4.1.3 |
| S0-04 | focus | picker option | ArrowDown | [role="option"] "halo-breathing-encoding · P-026 · <20s" (active descendant) | the option text | SC 4.1.2 |
| S0-05 | focus | picker option | ArrowDown | [role="option"] "halo-hue-encoding · P-025 · <5s" | the option text | SC 4.1.2 |
| S0-06 | focus | select the suite | ArrowUp ×2, Enter on "Suite — all scenarios" | [role="option"][aria-current="true"] whose aria-label carries " · selected" | the committed choice announced WITH its selected state as it becomes current — no longer waiting for a re-read | SC 4.1.2 |
| S0-07 | focus | Start control after a selection | Tab | button "Start" (aria-disabled=false) | "Start" + button, NOT unavailable | SC 4.1.2 |
| S0-08 | browse | Stop control while idle | ArrowDown until "Stop" (browse mode, from Start — Stop is unfocusable; cap 3) | button "Stop" (native disabled) | "Stop" + button + unavailable, by navigation only | SC 4.1.2 |
| S0-13 | browse | heading list — the phase-line h1 · three h2 | `h` until "Coverage matrix" (browse-mode next heading, from Start; cap 3) | h1 phase line · h2 "Scenario / suite" · "Coverage matrix" · "Run report" | "Coverage matrix heading level 2" | SC 1.3.1 |
| S0-10 | browse | run report empty prose | `h` until "Run report" (cap 2), then ArrowDown until "No run yet" (cap 4), browse mode | p "No run yet" — plan prose "No run yet — pick a scenario/suite to begin" (divergence) | "No run yet" | SC 4.1.3 |
| S0-09 | focus | coverage matrix current row | Tab (from Start — the browse caret moved, DOM focus did not) | tr[tabindex=0][aria-current=true] in table[aria-label="Coverage rows"], named by its own four cells (aria-labelledby) | the focused row alone, its P-ID read ("P-001"), never the table's rows as one utterance (a second row's "P-002" heard fails it) | SC 1.3.1 |
| S0-15 | focus | coverage matrix roving move to a not-yet-run row | ArrowDown ×1 (the matrix's own roving move; NVDA is in focus mode on the row) | the next tr (P-002), named by its cells; td.cov__status "Not yet run" (text, never a tint) | "P-002" … "Not yet run" as the row becomes current | SC 1.4.1 |

### S1 · live

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S1-01 | live | phase line flip on Start | Shift+Tab to Start, Enter | span[aria-live="polite"] "Conductor · live" | "Conductor · live" announced, interrupting | SC 4.1.3 |
| S1-02 | browse | count after Start | ArrowDown until "Scenarios completed" (browse mode, from the test-only focus target held first in `<body>`, removed in S1-05; cap 4) | span.titlebar__count[aria-live="polite"] › visually-hidden text "Scenarios completed: 0" + aria-hidden "0" | ANNOUNCED as it advances — an aria-live region named "Scenarios completed"; graded on its name | SC 4.1.3 |
| S1-05 | browse | in-progress prose | none (focus is returned to Start in-page here, for S1-03) | (none) — the plan's "Run in progress" prose does not ship | subject-absent | SC 4.1.3 |
| S1-03 | focus | Start control while running | Shift+Tab, Tab | button "Start" (aria-disabled=true) | "Start" + button + unavailable | SC 4.1.2 |
| S1-04 | focus | Stop control while running | Tab | button "Stop" (enabled) | "Stop" + button, NOT unavailable | SC 4.1.2 |

### S2 · hold (halo-breathing-encoding — the trimmed catalog holds in sorted-filename order, so P-026 first)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S2-01 | live | phase line flip to HOLD | Shift+Tab back to Start (so the restore target is Start), then wait for the hold (the in-run canary poll) | span[aria-live="assertive"] "Conductor · HOLD — operator pause" | "HOLD — operator pause" announced (assertive; may interleave with the dialog) | SC 4.1.3 |
| S2-02 | focus | operator-pause dialog opens, focus on Abort | (same instant) | [role="alertdialog"] labelled "P-026 — operator-checklist", described "Observe the operator-checklist claim for this scenario"; Abort focused | dialog + title + description, then "Abort" + button | SC 4.1.2 · SC 2.4.3 |
| S2-03 | focus | Proceed action | Tab | button "Proceed" | "Proceed" + button | SC 4.1.2 |
| S2-04 | focus | checklist row | Tab | label › input[type="checkbox"] + induced + observation text | the row text ("… halo breathing rate tracks throughput?") + check box + not checked | SC 4.1.2 |
| S2-06 | browse | unticked roll-up | ArrowDown until "1 of 1 unconfirmed" (browse mode, from the focused checkbox — the roll-up follows the checklist; cap 4) | p[role="status"][aria-live="polite"] "1 of 1 unconfirmed" | the initial text by navigation (the polite region announces changes) | SC 4.1.3 |
| S2-05 | live | Space toggles the row; roll-up updates | Space | checkbox checked + p[role="status"] "All observations confirmed" | "checked", then "All observations confirmed" (polite) | SC 4.1.3 |
| S2-07 | focus | Proceed closes the dialog; focus restored | Tab ×2 to Proceed (checkbox → Abort → Proceed; after S2-06's browse walk NVDA handles Shift+Tab from its caret on the roll-up, whose previous focusable is the checkbox), Enter | button "Start" regains focus (onCloseAutoFocus → restoreFocusTo) | "Start" + button (+ unavailable while running) — the restore announcement completes; the phase line leaving hold is polite and no longer preempts it | SC 2.4.3 |
| S2-08 | live | phase line back to live | (same instant) | span[aria-live="polite"] "Conductor · live" | "Conductor · live" announced | SC 4.1.3 |

### S3 · aborted (Stop during halo-hue-encoding, the second scenario)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S3-01 | live | Stop during the second scenario | Tab to Stop, Enter | span[aria-live="polite"] "Conductor · aborted" | "Conductor · aborted" announced | SC 4.1.3 |
| S3-02 | focus | Start control after Stop | Shift+Tab | button "Start" (aria-disabled=false again) | "Start" + button, NOT unavailable | SC 4.1.2 |
| S3-03 | live | second hold still opens (abort is polled between scenarios) | wait for the hold | HOLD flip + [role="alertdialog"] "P-025 — operator-checklist" | "HOLD", then the P-025 dialog | SC 4.1.3 |
| S3-04 | focus | Escape resolves NoGo; focus restored | Escape | button "Start" regains focus | "Start" + button — spoken every session; the assertive flip that cancelled it is scoped to entering hold | SC 2.1.2 · SC 2.4.3 |
| S3-05 | live | terminal Aborted stage settles the phase line | (same instant as S3-04 — the Aborted stage fires as the Escape resolution returns; shared window) then the report reload | span[aria-live="polite"] "Conductor · aborted"; report header "2 scenarios · run {run_id}" | "Conductor · aborted" announced and STAYS aborted — a stop during the last scenario now reports the Aborted stage instead of settling to idle | SC 4.1.3 |
| S3-06 | browse | run report rows after the stopped run | `h` until "Run report" (cap 3), then ArrowDown until "Manual" (cap 24), browse mode from Start | table rows with the "Manual" lamp label | "Manual" as the status cell text | SC 1.4.1 |
| S3-07 | browse | coverage matrix lamp for P-025 | Tab to the current coverage row, then ArrowDown (the matrix's roving move) until P-025 is current (cap 30) | the P-025 row, named by its cells › lamp "Manual" | "P-025" … "Manual" as the row becomes current | SC 1.4.1 |

### Terminal · done (a second, un-stopped run in the same session)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| T-01 | live | un-stopped run settles to idle | Shift+Tab to Start; after the quiet window (≥170 s from the first run's Aborted stage — Pulse dedupes a second canary against an open incident), Enter; at each hold Tab to Proceed, Enter; stamped once the second hold resolves | h1[aria-live="polite"] "Conductor · idle"; the report reloads with three scenarios | "Conductor · idle" announced on the Done stage | SC 4.1.3 |

## Rows — empty subject (empty catalog, the seeded fixture report)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| E0-01 | browse | empty-catalog prose | ArrowDown until "No scenarios found" (browse mode, from Close window after E0-03 — from BODY the caret sat at the report table, past the prose; cap 4) | p "No scenarios found." — plan prose "No scenarios loaded" (divergence) | "No scenarios found." | SC 4.1.3 |
| E0-02 | focus | Minimize window control | Tab | button[aria-label="Minimize window"] | "Minimize window" + button | SC 4.1.2 |
| E0-03 | focus | Close window control | Tab | button[aria-label="Close window"] | "Close window" + button | SC 4.1.2 |
| E0-04 | focus | Start control with no selection possible | Tab | button "Start" (aria-disabled=true) | "Start" + button + unavailable | SC 4.1.2 |
| E0-10 | browse | run-level load-envelope banner | `h` until "Run report" (cap 3), then ArrowDown until "ENVIRONMENT-SUSPECT" (cap 4), browse mode from Start — the banner sits between the heading and the header text | p.report__envelope — the seeded fixture's `run_envelope` row (`lamps-fixture`, over-envelope), label text "ENVIRONMENT-SUSPECT" | "ENVIRONMENT-SUSPECT" read as text, never a colour | SC 1.4.1 |
| E0-07 | browse | run report header | ArrowDown until "lamps-fixture" (cap 4), browse mode from the envelope banner (E0-10) — the header sits before the report scroll region | header "3 scenarios · run lamps-fixture" | the header text | SC 1.3.1 |
| E0-08 | browse | run report status cells | ArrowDown until the "Blocked" status label (browse mode; case-sensitive stop, since the Scenario cell "lamps-fixture-blocked" precedes it; cap 20) | td.report__status › lamp labels "Pass" · "Blocked" · "Fail" (glyph aria-hidden) | "Blocked" read as text, never a colour | SC 1.4.1 |
| E0-05 | focus | coverage matrix current row | Tab (from Start — the browse caret moved, DOM focus did not) | tr[tabindex=0][aria-current=true] in table[aria-label="Coverage rows"], named by its own four cells (aria-labelledby) | the focused row alone, its P-ID read ("P-001"), never the table's rows as one utterance (a second row's "P-002" heard fails it) | SC 1.3.1 |
| E0-06 | focus | run report rows scroll region (fixture present) | Tab | div[tabindex=0] › table[aria-label="Run report rows"] | the region named "Run report rows" ONLY — not the table read as one utterance | SC 1.3.1 |
| E0-09 | browse | coverage lamp for the collided P-ID | Shift+Tab, then ArrowDown (the matrix's roving move) until P-019 is current (cap 24) | Shift+Tab lands on the current coverage row (tr[tabindex=0][aria-current=true]); row P-019, named by its cells › lamp "Blocked" (worst-lamp-wins over Pass) | "P-019" … "Blocked" as the row becomes current | SC 1.4.1 |

## Rows — error subject (a malformed catalog)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| R0-01 | live | scenario load error, re-announced on the first focus event | none — NO reload and no focus warm-up; the row is stamped inside the foreground step, between activation and the first Tab, because that Tab IS the first focus event the announcement fires on (a session-start read would work too, but would drag in the activation window, where NVDA speaks the WebView2 host window's own path-bearing title) | div[role="alert"] › p (sr-only) "Could not load scenarios: …" — the region mounts EMPTY and the VISIBLE copy sits outside it, so the re-assertion is the region's only content: one node, inserted once on the first `focusin`, announced once | "Could not load scenarios" announced ONCE; the text carries NO host path (a heard path is a security finding against the sanitize_error edge). NVDA binds a window on its first focus event, which necessarily follows a load-time paint — an empty-mounted region is necessary but NOT sufficient, which is what the re-assertion supplies. Silence here is a FINDING, never a pass | SC 4.1.3 |
| R0-02 | focus | Minimize window control | Tab | button[aria-label="Minimize window"] | "Minimize window" + button | SC 4.1.2 |
| R0-03 | focus | Close window control | Tab | button[aria-label="Close window"] | "Close window" + button | SC 4.1.2 |
| R0-04 | focus | Start control with nothing loaded | Tab | button "Start" (aria-disabled=true) | "Start" + button + unavailable | SC 4.1.2 |

## Process census and stop forms

The leg records `tasklist` before and after each session (`nvda.exe` · `conductor-tauri.exe` ·
`msedgedriver.exe` · `node.exe` · `andromeda-pulse-mcp.exe` · `pulse-app.exe`) into the evidence. Expected
final states: `nvda.exe` terminated (`-q`); `conductor-tauri.exe`, `msedgedriver.exe` and the driver's
`node.exe` terminated (`onComplete`); `andromeda-pulse-mcp.exe` terminated with the app that spawned it;
`pulse-app.exe` left running — operator stops it.

## Findings ledger (for wrap's amendment flow)

| Finding | Where the plan says otherwise |
|---|---|
| The shipped `RunState` set is `idle · live · hold · aborted`; `report-terminal` is idle with a report | a11y-plan §1 · §3 · §5 |
| The operator checklist ships at ONE render site (the HOLD dialog); the run report's `ManualCheck` rows do not expand | design-system §Component Patterns 6 / 7 · layout-templates §Operator-checklist · a11y-plan §4 |
| No footer status strip, so no `contentinfo`; no `h1` (three `h2`) — SHIPPED by `2026-09-30-the-screen-reader-content-findings-fixed` (the footer strip, the phase-line `h1`); the strip's unticked-ManualCheck count stays unbuilt with the report-site checklist | a11y-plan §4 · layout-templates §Component — Footer |
| The dialog title is `{p_id} — operator-checklist`, not a frozen count + step index | a11y-plan §4 |
| Shipped prose: `No scenarios found.` · `No run yet` · no in-progress prose | a11y-plan §3 · design-system §6 · layout-templates §blocks 1-2 |
| The titlebar count is an unlabeled number (a scenario counter) with no live region | design-system §Component Patterns 1 · a11y-plan §3 |
| The verdict lamp carries no live region | a11y-plan §4 · design-system §Component Patterns 4 |
| An NVDA automation library exists (`@guidepup/guidepup`); this leg uses NVDA's built-in log instead | a11y-plan §3 "No automated SR tool exists for the stack" |

### Content findings — the 2026-09-30 OS-path regrade, each confirmed against the rendered DOM

The 18 findings of `chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/nvda-pass.json`.
A **product fix** changes the DOM; a **re-token** changes the row's action, position or token because the DOM
already carries the content; a **re-action** drives a row that had no OS-path key in its window. Line cites are
the chunk base `d7da5d0` unless marked otherwise.

| Finding | Disposition | Reason (the rendered DOM) |
|---|---|---|
| E0-01 | re-action | the prose is in the DOM (`App.tsx:322-324`); no OS-path browse key fell in the row window (`parse-nvda-log.ts:304-305`), so it is now walked by ArrowDown from the focused Close window, directly above it (from BODY the caret followed the last DOM focus to the report table, measured at the regrade) |
| E0-05 | product fix | `tr.cov__row` carried no author name (`CoverageMatrix.tsx:113-123`) and NVDA heard "row current"; the row is now named by its own four cells through `aria-labelledby`, P-ID first |
| E0-07 | re-token | the header text is in the DOM (`RunReport.tsx:41-44`) BEFORE the report scroll region (`:47`), where the caret started; now `h` to "Run report" then ArrowDown from Start, tokens `3 scenarios` + `lamps-fixture` |
| E0-08 | re-token | "Blocked" is lamp-label text (`StatusLamp.tsx:17`) and one ArrowDown reached only "column 2 Scenario"; now ArrowDown until the case-sensitive "Blocked" label, because the Scenario cell `lamps-fixture-blocked` precedes it |
| E0-09 | product fix + re-token | the row name fix, plus a move that reaches P-019 (row index 18): the one ArrowDown was the matrix's roving move to index 1 (`CoverageMatrix.tsx:64-82`); tokens `P-019` + `Blocked` |
| S0-08 | re-action | Stop is in the DOM, natively disabled (`RunControls.tsx:36`); no OS-path browse key fell in the row window, so it is now walked by ArrowDown from Start |
| S0-09 | product fix | as E0-05 (`CoverageMatrix.tsx:113-123`): the focused row had no name; it now announces its P-ID from its own cells |
| S0-10 | re-action | the prose is in the DOM (`App.tsx:386-388`); no OS-path browse key fell in the row window, so it is now walked by `h` to "Run report" then ArrowDown from Start |
| S0-11 | re-action | the phase line is in the DOM (`Titlebar.tsx:32-38`) and is now the `h1`; no OS-path browse key fell in the row window, so it is walked by ArrowDown from a test-only focus target held first in `<body>` (the caret follows DOM focus and the line precedes every focusable), after S0-02 because NVDA binds the document only at the first OS-path Tab (walked before S0-01 it was silent, measured at the regrade), tokens `idle` + `level 1` |
| S0-12 | product fix + re-action | the count's name was an `aria-label` on a role-less span (`Titlebar.tsx:39-49`), which browse mode does not read: the regrade heard the bare digits; the name is now visually-hidden text inside the count with the digits `aria-hidden`, and the row is walked by ArrowDown beside S0-11 |
| S0-13 | product fix + re-token | there was no `h1` (three `h2`, `App.tsx:292,347,371`; a11y-plan `:323`) and the phase line is now the `h1`; `h` had gone from the focus-mode row to the app (`CoverageMatrix.tsx:76-77`), now from Start, tokens `Coverage matrix` + `level 2` |
| S0-14 | product fix + re-token | there was no `contentinfo` (whole-file `App.tsx`; a11y-plan `:548`) and the footer strip now ships; `d` had gone from the focus-mode row to the app, now from Close window, tokens `main` + `content info` |
| S0-15 | product fix + re-token | "Not yet run" is in the DOM (`CoverageMatrix.tsx:142`) but the ArrowDown was the roving move onto an unnamed row (`:64-82`); the row name now carries its status, and the row is re-classed focus, tokens `P-002` + `Not yet run` |
| S1-02 | product fix + re-action | as S0-12 (`Titlebar.tsx:39-49`): the regrade heard "heading level 1 Conductor · live" then the bare "0", so the name is now visually-hidden text; the row is walked by ArrowDown from the test-only focus target held first in `<body>`, and graded on its name token (the review flag dropped) |
| S2-06 | re-action | the roll-up is in the DOM after the checklist (`OperatorChecklistView.tsx:18-20`); no OS-path browse key fell in the row window, so it is walked by ArrowDown from the focused checkbox |
| S3-06 | re-token | "Manual" is lamp-label text (`StatusLamp.tsx:17`) and the ArrowDown from Start read the Stop line; now `h` to "Run report" then ArrowDown to the status cell |
| S3-07 | re-token | the P-025 row and its lamp are in the DOM, but the ArrowDown from Start read the hint line (`RunControls.tsx:43-45`); now Tab to the current row and the roving move to P-025, tokens `P-025` + `Manual` |
| T-01 | re-action | the phase line reaches "Conductor · idle" on Done (`App.tsx:83-88`), but the live subject stops at aborted; now a second, un-stopped run in the same session after the quiet window |

Outside the 18, the regrade's operator review recorded one new finding, routed forward: **E0-10** is graded
subject-absent by its row ("the fixture records no run_envelope row"), yet the ENVIRONMENT-SUSPECT banner was
heard in E0-07's window — `runs/e2e-fixture` now carries the envelope row the `--e2e` arm seeds — so its absent
reason no longer holds and the row is owed a grade against its expected content. **Disposition: re-token +
re-action** — the banner is in the DOM as label text in the seeded subject, so sr-empty is not re-seeded; the row
is now walked forward from Start (`h` to "Run report", then ArrowDown to "ENVIRONMENT-SUSPECT"), ahead of E0-07,
which continues ArrowDown from the banner to the header text; token `ENVIRONMENT-SUSPECT`, graded, no `absent`.
