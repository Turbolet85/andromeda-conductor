# Codebase Research — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

## Scope
- **Depth:** moderate · **Reads:** 13 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full by structural extraction. That
  meant a `grep -n` index of the headers and the 31 Session-Additions introducers, then offset reads `:1-47`,
  `:48-57` and `:58-70` covering every span. The SR-leg additions that apply here are `:58` (census), `:59` (OS
  foreground, first-focus binding, injected keys never reaching browse mode, the parallel-session theft rule, the
  NVDA-log theft decision), `:61` (the live `sr` subject's sidecar `PATH`), `:62` (`sr*` never rebuild), `:63`
  (`nvda-pass.json` is cumulative per subject), `:68` (read NVDA's log by headers), `:69` (admission gates
  execution) and `:70` (absolute `-c`/`-f`). The live-Pulse recipe is `:48`, `:50`, `:52`, `:53`, `:57` and `:60`.
  Also read: `.claude/rules/a11y.md` and `frontend.md`, auto-loaded on `crates/conductor-tauri/ui/**`, and
  `host-win32.md`, always loaded.
- **Platform issues consulted:** none — there is no runner-only bullet. CI over `ff4f571` was re-read green, and no
  CI entry sits outside the operator legs.

## Files inspected
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (full) — every key the leg sends goes through
  `browser.keys` (WebDriver-injected):
  - `tab()` `:355-357`, `shiftTab()` `:359-361`, `browseKey()` `:363-368`;
  - the picker walk's text, arrows, Enter, Space and Escape inline at `:400-560`.

  `bringToForeground()` `:68-129` runs `activate-window.ps1` through `spawnSync('powershell.exe', ['-NoProfile',
  '-ExecutionPolicy', 'Bypass', '-File', script, '-Title', WINDOW_TITLE])` (`:69-74`). It then cycles injected Tabs to
  BODY before the rows start (`:94-105`); that is "the harness's start-of-document cycle" the 153 arm heard. Where
  the regraded rows sit:
  - S0-09: live subject, `:449-452`. The live walk continues into S1 (Start, `:461`), so the subject needs a live
    Pulse.
  - E0-02..E0-06 and E0-09: empty subject, `:597-624`.
  - R0-02..R0-04: error subject, `:656-668`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/activate-window.ps1` (full) — the ONE committed OS-level input
  in the leg: `keybd_event` ALT down/up (`:33-34`), then `SetForegroundWindow`. It types nothing else (`:6`).
- `crates/conductor-tauri/ui/wdio.conf.ts` (full):
  - `SR_SUITES` `:164-168` (subject → runs/scenarios dirs);
  - `startNvda` `:206-242` (absolute `-c`/`-f`, readiness `NVDA initialized` + settle) and `stopNvda` `:244-250`;
  - the one tauri-driver spawn `:417-421` (`--native-driver`, cwd = repo root, env = `appEnv`);
  - `onComplete` `:445-466`, which writes `runs/sr-leg/nvda-pass.json` through `writeNvdaPass`.

  No key-path code lives here.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` (full):
  - `grade()` `:275-294`: a browse row with empty `heard` → `not-run-here`, arm `operator`, with the fixed note
    `BROWSE_NOT_DELIVERABLE` (`:271-273`, "driver-injected keys bypass NVDA's keyboard hook"). A browse row WITH
    heard text is token-graded on the agent arm like any other row. So grading needs no change to accept an
    OS-driven browse row; only the empty-row note is input-path-bound.
  - `readStamps()` `:212-256` ignores unknown stamp fields and every other `@…` record.
  - `PassFile` / `PassRow` `:26-101` carry no input-path field. The only configuration fields are `webview2_runtime`
    (the session's `browserVersion`), `webview2_runtime_registry`, `nvda_version` and `build_commit`.
  - The record is cumulative per subject (`:445-466`).
  - The file is also a CLI (`:473-500`): `tsx parse-nvda-log.ts <speech-log> <actions.jsonl> <out.json> --subject …`
    grades any timeline in the leg's stamp format.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` (grep) — S0-09 `:102` (focus), E0-05 `:272` (focus),
  E0-09 `:298` (browse), R0-02 `:319` (focus). The browse rows are S0-11..S0-15, S0-16, S1-02, S2-06, S3-06, S3-07 and
  E0-01, E0-07..E0-09.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (grep) — `:21-24` states that driver-injected
  keys reach focus/live events and that a browse row the agent arm cannot reach falls to the operator. The E0-09
  action is at `:174`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-config/nvda.ini` (full) — `[general]`, `[speech]` and
  `[virtualBuffers]` only. There is no `[keyboard]` section, so `handleInjectedKeys` keeps its default of true.
- `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/no-boundary-control.md` (full) —
  the measured basis (§Arms, §Candidates), both session scripts as run (§Scripts), the census (§Census).
- `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/research.md` `:141-215` — S's no-boundary
  argument covered an UNCOMMITTED script. Committing into the driver-stack locus is a different question.
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/plan.md` `:210-234` — the
  three reds' witnesses, verbatim:
  - the focus count (`jq … select(.class=="focus" and .outcome!="announced-as-expected") | length`, bar 0);
  - S0-09 / E0-05 / E0-09 (`jq … .id + " " + .outcome`; E0-09's then-expected outcome was `not-run-here`);
  - one configuration (`[.subjects.{live,empty,error}.recorded_at] | map(. > base) | all`).
- `…/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/nvda-pass.json` (jq) — 51 rows. Focus
  agent rows: 20 `not-announced` + 4 `announced-differently` = 24. Browse operator rows: 15 `not-run-here`.
  `attach_observed` reads `{live: true, empty: false, error: true}`. The subjects' `recorded_at` are `empty`
  12:20:29Z and `error` 12:17:40Z.
- `.andromeda/security-plan.md:367` (offset-window read, one 9 476-char line) — rule (b)'s governed forms, quoted in
  §Patterns detected.
- `.andromeda/playbook.md:124-126` — *Boundary widening*, `verdict: escalate`, "always a human's call".

## Graph impact
- ts plane (trace `.andromeda/runs/2026-09-30T14-14-44-phase/tree-query-2026-09-30-the-sr-pass-regrades-on-the-os-input-path.json`),
  one query over eight names, all with callers:
  - **tab** — 21 call sites (the trace's 21 `tab` rows), all in `screen-reader.e2e.ts`: `:95`, `:101`, `:307`, `:389`, `:393`, `:397`, `:442`,
    `:450`, `:479`, `:484`, `:503`, `:507`, `:542`, `:598`, `:602`, `:606`, `:611`, `:615`, `:657`, `:661`, `:665`
    (editor lines, graph +1). A key-path change inside `tab()` reaches every focus row and the start-of-document
    cycle at once.
  - **shiftTab** — 6 sites (`:462`, `:478`, `:493`, `:526`, `:548`, `:621`).
  - **browseKey** — 7 sites (`:453-455`, `:576-577`, `:618-619`, `:623`).
  - **bringToForeground** — 3 sites (`:380`, `:593`, `:640`), one per subject.
  - **writeNvdaPass** — 2 callers: `wdio.conf.ts:454` and the parser's own CLI `:486`.
  - **readStamps** — 1 caller (`writeNvdaPass`, `:355`). **grade** — 1 caller (`:398`).
  - **census** — `wdio.conf.ts:247`, `:408` and `:462`, plus the CLI `:494`.

  Every helper is file-local to the SR spec or the parser. No rust-plane symbol is touched, so there is no
  cross-crate blast radius.

## Patterns detected
- **Rule (b)'s window-activation form** (`security-plan.md:367`): "a FIXED OS program with a fixed argv
  (`spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', <repo script>, '-Title',
  <constant>])` — the window-activation step, no operator input anywhere in the vector)". That form is registered
  for ONE step, and its synthetic input is the ALT that lifts the foreground lock.
  - A committed OS-key script would add a new spawn site in the dev-only driver-stack locus, carrying a new
    capability: keystroke synthesis into whatever window is foreground.
  - The security history's first precedent applies: `2026-09-01-webview-self-verify-windows-host`, "a chunk that
    genuinely ADDS a spawn … escalates rather than being dismissed". So does `2026-09-16 — a11y-ci-gate-at-an-honest-terminal`,
    "an eighth crossing escalates again".
  - The playbook `:124` pattern holds on its clause "a subprocess/IPC boundary gains a new crossing".
  - Ratification is the founder's live word only (`security.md` Session Additions 2026-09-29).
- **Foreground-guarded OS keys** (`no-boundary-control.md` §Scripts, `key-walk.ps1`) — a `SendInput` helper that
  checks the foreground process before each key. `Foreground()` activates only when the target is not already
  foreground, because a synthetic ALT on a foreground app opens its System menu (`verification-harness.md:59`).
  `ForegroundPid()` is stamped per key, which makes the theft check possible.
- **The W3C driver session from a session script** (`no-boundary-control.md` §Scripts, `edge-webdriver-walk.ps1`)
  — driver readiness via `/status`, `POST /session`, `POST /session/{id}/actions` for an injected Tab (the escape
  built from `[char]92`, since the Write tool decodes a JSON backslash-u escape), `DELETE /session`, then a
  driver-tree stop.
- **Rows stamped before their action, graded by time window** (`screen-reader.e2e.ts:132-136`,
  `parse-nvda-log.ts:366-369`), plus `@end` before teardown. A shared window of 50 ms covers paired events.
- **Configuration from the session, not the registry** — `webview2_runtime` comes from the session's
  `browserVersion` (`screen-reader.e2e.ts:55-58`, `parse-nvda-log.ts:437`).

## Conventions to follow
- **Stop and ask before every NVDA launch**, stating its expected run length. This is operator directive
  (scope.md), and holds in implement as well.
- **Stop every process started**, with a before/after census by parentage (`verification-harness.md:58`,
  host-win32.md 2026-09-17). The census covers `msedgewebview2`, and must check for Edge's background relaunch.
- **Scratch `.ps1` stays ASCII**, with `[uint16]`/`[uint32]` only. Dry-run any typed function before its slot
  (host-win32.md 2026-09-30 ×2).
- **Driver/runtime re-read before every leg**, because the Evergreen runtime moves between sessions (a11y.md
  2026-09-17). Measured at P3, 2026-09-30T14:3xZ: `driver 154.0.4258.37 runtime 154.0.4258.37`. The
  `EdgeWebView/Application` folder lists `153.0.4234.48`, `154.0.4258.37` and `SetupMetrics`. There is no
  `4444`/`4445` listener.
- **The `sr*` suites never rebuild** (`verification-harness.md:62`). An app-side change needs `--e2e` first. A
  harness-side change (spec, parser) takes effect immediately.
- **The live `sr` subject's firing form** (`verification-harness.md:59`, `:61`, `:57`, `:52`, `:53`):
  - handles `CONDUCTOR_NVDA`, `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` and the
    three `ANDROMEDA_PULSE_*`;
  - the Pulse release dir prepended to `PATH` in POSIX form;
  - a running `pulse-app` on a letters-only leaf under `%TEMP%/pulse-legs/`;
  - a quiet window after any canary, and no `boot` before the leg.

  The Pulse release binaries are present (`andromeda-pulse/target/release/{pulse-app,andromeda-pulse-mcp}.exe`).
  The Pulse tree's HEAD is `d708ad7` (2026-09-30T15:41+02:00), which the parallel pulse-builder session moved;
  re-check the binary against it before a live leg.

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/confound-control.md`
- `conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/nvda-pass.json` — only on a regrade branch
- `crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1` — only if the founder ratifies the committed OS key path

## Files to modify
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — only if the founder ratifies the committed OS key path
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` — only on the ratified branch
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — only on the ratified branch

## Open questions
- Whether to commit an OS-level key path into the `sr*` leg: a new spawn in the driver-stack locus, playbook `:124`
  *Boundary widening* → blocks: plan-decision. P4 escalates it for the founder's live word, and the operator relays.
- Step 1's outcome (heard / silent / silence not reproduced under the script's launch) decides whether step 2
  exists at all → blocks: implementation-scope. The branch file lists above are provisional on it.
