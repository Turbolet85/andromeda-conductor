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
- **Row classes.** `focus` — a focus change; `live` — a live-region, alert or dialog-open event; both are
  accessibility events NVDA announces however focus moved, so driver-injected keys reach them. `browse` —
  static text NVDA reaches only through its browse-mode commands (headings, landmarks, prose, table cells),
  which NVDA's own keyboard hook intercepts and the driver's synthesized keys may bypass. The leg attempts
  every class and records the outcome; a `browse` row the agent arm cannot reach falls to the operator's
  manual arm, and the evidence records the ARM per row.
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
`aria-live="assertive"` span: `Conductor · idle` · `Conductor · live` · `Conductor · HOLD — operator pause` ·
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
| `npm run a11y:sr` | `live` | `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` (from the shell: `halo-breathing-encoding` + `halo-hue-encoding` + one short third scenario such as `high-severity-log-capture`, which never executes) · `CONDUCTOR_RUNS_DIR=runs/sr-leg/runs` (emptied per session) | idle → `Start` → live → hold #1 (P-026: Tab · Space · Enter on Proceed) → live → `Stop` during the second scenario → aborted → hold #2 (P-025: Escape) → the backend `Aborted` stage before the third scenario → the report reload; ONE canary. The third scenario is load-bearing: `drive_run` checks the abort flag only BEFORE a scenario starts, so a `Stop` during the last scenario ends in `Done` / idle (measured 2026-09-02), never `Aborted` |

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
| S0-11 | browse | titlebar phase line at rest | none (navigation) | banner › span[aria-live="assertive"] "Conductor · idle" | "Conductor · idle" read by navigation — the assertive region announces changes, not initial content | SC 4.1.3 |
| S0-12 | browse | titlebar count placeholder | none (navigation) | banner › span "00:00:00" — no accessible name (finding) | the digits only; nothing names the value | SC 4.1.2 |
| S0-01 | focus | Minimize window control | Tab | button[aria-label="Minimize window"] | "Minimize window" + button | SC 4.1.2 |
| S0-02 | focus | Close window control | Tab | button[aria-label="Close window"] | "Close window" + button | SC 4.1.2 |
| S0-03 | focus | scenario / suite picker | Tab | input[role="combobox"] "Scenario or suite picker" (cmdk), first option active | "Scenario or suite picker" + combo box, editable, the active option | SC 4.1.2 |
| S0-16 | browse | picker filter-miss prose | type `zzz`, then Backspace ×3 | cmdk Empty "No scenarios match." (role="presentation") | not announced on appearance; reachable by navigation only | SC 4.1.3 |
| S0-04 | focus | picker option | ArrowDown | [role="option"] "halo-breathing-encoding · P-026 · <20s" (active descendant) | the option text | SC 4.1.2 |
| S0-05 | focus | picker option | ArrowDown | [role="option"] "halo-hue-encoding · P-025 · <5s" | the option text | SC 4.1.2 |
| S0-06 | focus | select the suite | ArrowUp ×2, Enter on "Suite — all scenarios" | [role="option"][aria-current="true"] with " · selected" | the committed choice announced as "current" (aria-current); the " · selected" text is reached on re-reading | SC 4.1.2 |
| S0-07 | focus | Start control after a selection | Tab | button "Start" (aria-disabled=false) | "Start" + button, NOT unavailable | SC 4.1.2 |
| S0-08 | browse | Stop control while idle | none (unfocusable) | button "Stop" (native disabled) | "Stop" + button + unavailable, by navigation only | SC 4.1.2 |
| S0-09 | focus | coverage rows scroll region | Tab | div[tabindex=0][role="group"] "Coverage rows" | "Coverage rows" + grouping | SC 1.3.1 |
| S0-13 | browse | heading list — three h2, no h1 (finding) | `h` (browse-mode next heading) | h2 "Scenario / suite" · "Coverage matrix" · "Run report" | "Scenario / suite heading level 2" | SC 1.3.1 |
| S0-14 | browse | landmark list — banner · main · two regions; contentinfo absent (finding) | `d` (browse-mode next landmark) | header (banner) · main · section[aria-label] | "main landmark" | SC 1.3.1 |
| S0-15 | browse | coverage matrix not-yet-run cell | ArrowDown (browse-mode next line) | td.cov__status "Not yet run" (text, never a tint) | "Not yet run" as a table cell | SC 1.4.1 |
| S0-10 | browse | run report empty prose | none (navigation) | p "No run yet" — plan prose "No run yet — pick a scenario/suite to begin" (divergence) | "No run yet" | SC 4.1.3 |

### S1 · live

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S1-01 | live | phase line flip on Start | Shift+Tab to Start, Enter | span[aria-live="assertive"] "Conductor · live" | "Conductor · live" announced, interrupting | SC 4.1.3 |
| S1-02 | browse | count after Start | none | span.titlebar__count "0" — no live region (finding) | NOT announced; the value reads by navigation only; operator grades | SC 4.1.3 |
| S1-05 | browse | in-progress prose | none | (none) — the plan's "Run in progress" prose does not ship | subject-absent | SC 4.1.3 |
| S1-03 | focus | Start control while running | Shift+Tab, Tab | button "Start" (aria-disabled=true) | "Start" + button + unavailable | SC 4.1.2 |
| S1-04 | focus | Stop control while running | Tab | button "Stop" (enabled) | "Stop" + button, NOT unavailable | SC 4.1.2 |

### S2 · hold (halo-breathing-encoding — the trimmed catalog holds in sorted-filename order, so P-026 first)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S2-01 | live | phase line flip to HOLD | Shift+Tab back to Start (so the restore target is Start), then wait for the hold (the in-run canary poll) | span[aria-live="assertive"] "Conductor · HOLD — operator pause" | "HOLD — operator pause" announced (assertive; may interleave with the dialog) | SC 4.1.3 |
| S2-02 | focus | operator-pause dialog opens, focus on Abort | (same instant) | [role="alertdialog"] labelled "P-026 — operator-checklist", described "Observe the operator-checklist claim for this scenario"; Abort focused | dialog + title + description, then "Abort" + button | SC 4.1.2 · SC 2.4.3 |
| S2-06 | browse | unticked roll-up at open | none | p[role="status"][aria-live="polite"] "1 of 1 unconfirmed" | the initial text by navigation (the polite region announces changes) | SC 4.1.3 |
| S2-03 | focus | Proceed action | Tab | button "Proceed" | "Proceed" + button | SC 4.1.2 |
| S2-04 | focus | checklist row | Tab | label › input[type="checkbox"] + induced + observation text | the row text ("… halo breathing rate tracks throughput?") + check box + not checked | SC 4.1.2 |
| S2-05 | live | Space toggles the row; roll-up updates | Space | checkbox checked + p[role="status"] "All observations confirmed" | "checked", then "All observations confirmed" (polite) | SC 4.1.3 |
| S2-07 | focus | Proceed closes the dialog; focus restored | Shift+Tab to Proceed, Enter | button "Start" regains focus (onCloseAutoFocus → restoreFocusTo) | "Start" + button (+ unavailable while running) | SC 2.4.3 |
| S2-08 | live | phase line back to live | (same instant) | span[aria-live="assertive"] "Conductor · live" | "Conductor · live" announced | SC 4.1.3 |

### S3 · aborted (Stop during halo-hue-encoding, the second scenario)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| S3-01 | live | Stop during the second scenario | Tab to Stop, Enter | span[aria-live="assertive"] "Conductor · aborted" | "Conductor · aborted" announced | SC 4.1.3 |
| S3-02 | focus | Start control after Stop | Shift+Tab | button "Start" (aria-disabled=false again) | "Start" + button, NOT unavailable | SC 4.1.2 |
| S3-03 | live | second hold still opens (abort is polled between scenarios) | wait for the hold | HOLD flip + [role="alertdialog"] "P-025 — operator-checklist" | "HOLD", then the P-025 dialog | SC 4.1.3 |
| S3-04 | focus | Escape resolves NoGo; focus restored | Escape | button "Start" regains focus | "Start" + button | SC 2.1.2 · SC 2.4.3 |
| S3-05 | live | terminal Aborted stage settles the phase line | (same instant as S3-04 — the Aborted stage fires as the Escape resolution returns; shared window) then the report reload | span[aria-live="assertive"] "Conductor · aborted"; report header "2 scenarios · run {run_id}" | "Conductor · aborted" announced again | SC 4.1.3 |
| S3-06 | browse | run report rows after the stopped run | ArrowDown (browse-mode next line) | table rows with the "Manual" lamp label | "Manual" as the status cell text | SC 1.4.1 |
| S3-07 | browse | coverage matrix lamp for P-025 | ArrowDown (browse-mode next line) | td.cov__status › lamp "Manual" beside P-025 | "P-025" … "Manual" | SC 1.4.1 |

### Terminal · done

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| T-01 | live | un-stopped run settles to idle | not run — the live subject is stopped by design; a second un-stopped session is optional | span[aria-live="assertive"] "Conductor · idle" | "Conductor · idle" announced on the Done stage | SC 4.1.3 |

## Rows — empty subject (empty catalog, the seeded fixture report)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| E0-01 | browse | empty-catalog prose | none (navigation) | p "No scenarios found." — plan prose "No scenarios loaded" (divergence) | "No scenarios found." | SC 4.1.3 |
| E0-02 | focus | Minimize window control | Tab | button[aria-label="Minimize window"] | "Minimize window" + button | SC 4.1.2 |
| E0-03 | focus | Close window control | Tab | button[aria-label="Close window"] | "Close window" + button | SC 4.1.2 |
| E0-04 | focus | Start control with no selection possible | Tab | button "Start" (aria-disabled=true) | "Start" + button + unavailable | SC 4.1.2 |
| E0-05 | focus | coverage rows scroll region | Tab | div[role="group"] "Coverage rows" | "Coverage rows" + grouping | SC 1.3.1 |
| E0-06 | focus | run report rows scroll region (fixture present) | Tab | div[role="group"] "Run report rows" | "Run report rows" + grouping | SC 1.3.1 |
| E0-07 | browse | run report header | ArrowDown (browse-mode next line) | header "3 scenarios · run lamps-fixture" | the header text | SC 1.3.1 |
| E0-08 | browse | run report status cells | ArrowDown (browse-mode next line) | td.report__status › lamp labels "Pass" · "Blocked" · "Fail" (glyph aria-hidden) | "Blocked" read as text, never a colour | SC 1.4.1 |
| E0-09 | browse | coverage lamp for the collided P-ID | Shift+Tab, ArrowDown | row P-019 › lamp "Blocked" (worst-lamp-wins over Pass) | "P-019" … "Blocked" | SC 1.4.1 |
| E0-10 | browse | run-level load-envelope banner | none | p.report__envelope — not rendered (the fixture records no run_envelope row; the DOM proof is v2-25's) | subject-absent | SC 1.4.1 |

## Rows — error subject (a malformed catalog)

| # | Class | Item | Producing action / key path | Node (role · accessible name · mechanism) | Expected NVDA output | WCAG SC |
|---|---|---|---|---|---|---|
| R0-01 | live | scenario load error (alert on document load) | focus warm-up (Tab), then reload the document — NVDA tracks a window only from its first focus event, so an alert rendered on the original load is never spoken (finding); the reload fires it while the window is tracked | p[role="alert"] "Could not load scenarios: …" | "Could not load scenarios" announced; the text carries NO host path (a heard path is a security finding against the sanitize_error edge) | SC 4.1.3 |
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
| No footer status strip, so no `contentinfo`; no `h1` (three `h2`) | a11y-plan §4 · layout-templates §Component — Footer |
| The dialog title is `{p_id} — operator-checklist`, not a frozen count + step index | a11y-plan §4 |
| Shipped prose: `No scenarios found.` · `No run yet` · no in-progress prose | a11y-plan §3 · design-system §6 · layout-templates §blocks 1-2 |
| The titlebar count is an unlabeled number (a scenario counter) with no live region | design-system §Component Patterns 1 · a11y-plan §3 |
| The verdict lamp carries no live region | a11y-plan §4 · design-system §Component Patterns 4 |
| An NVDA automation library exists (`@guidepup/guidepup`); this leg uses NVDA's built-in log instead | a11y-plan §3 "No automated SR tool exists for the stack" |
