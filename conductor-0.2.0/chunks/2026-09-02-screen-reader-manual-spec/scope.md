# Scope — 2026-09-02-screen-reader-manual-spec

**Working entry (verbatim, Epoch 5 — Verification surfaces):**
> Screen-reader manual spec — per-state NVDA/VoiceOver/Orca must-announce pass spec for the four run states

**Matrix entry:** `v2-23` _Screen-reader manual spec_ — requirement "An NVDA/VoiceOver manual pass spec
accompanies the automated sweep." · method `manual` · acceptance (outcome-level, route-authored): "A per-state
pass spec exists for the four run states listing each must-announce item with its expected screen-reader
output, and one operator pass is recorded against it." · `status: planned` · `chunk: null`.
[verified against `verification-matrix.json`]

**Intent:** F14 (`intent.md` §Theme 6) — "… plus an NVDA/VoiceOver manual spec. **Do NOT re-author the
harness**". [verified at source]

**Standing operator directive ("the NVDA directive stands", take-up 2026-09-02) — as read.** No in-repo
artifact records the directive's text (a repo-wide case-insensitive search for `nvda` finds only the route
entry, the matrix/requirements/intent wording, the a11y-plan and its derived leaves). The reading below is
reconstructed from the sweep's route-level ruling (master-route `2026-09-01-desktop-a11y-sweep`: "affordance
proof agent-driven, the judgment class operator-gated"), its plan (`plan.md:47-52`: the screen-reader pass
"is its own route entry (v2-23)"; "macOS VoiceOver has no automated SR test, but there is no macOS surface on
this host") and the 2026-08-22 Tier-1 host rule (Windows-only host; the operator's eyes reserved for the
judgment class). **CONFIRMED as written by the operator at the P5 review card (2026-09-02)**, with two
rulings added: (i) the spec is this chunk's authored artifact and ONE operator NVDA pass recorded in
`evidence/nvda-pass.json` is its proof, the NVDA install being the named operator precondition — and
`/andromeda-implement` ENDS with that pass PENDING for the operator's own time: the chunk HOLDS across that
seam (stays `pending`, wrapped only after the pass is recorded), never wrap-with-owed; (ii) the NVDA
automation driver research found stays rejected for this chunk but is surfaced as a ROUTE CANDIDATE at
wrap's route-resolve (natural owner: the Epoch-6 _A11y CI gate_ entry) — the standing default is agent-driven
wherever a driver exists, so the manual pass is the interim.
- The recorded pass is **NVDA on this Windows host, performed by the operator** — the judgment class. The
  agent authors the spec, the stimulus recipe and the evidence record; the operator supplies what NVDA said.
- **VoiceOver (macOS) and Orca (Linux) have no surface on this host.** The spec may carry them only as
  declared-not-runnable-here notes (the a11y-plan names all three), never as claimed passes.
- **No screen-reader automation is introduced by this chunk** — no SR driver, no NVDA add-on, no second
  automation stack. The pass stays manual; whether a driver EXISTS is a research question (below), never an
  action here.

## Re-shape — P5 `review` relay (operator, 2026-09-02): the PASS is agent-driven, the SPEC stays

The operator's consolidated relay answered the first P5 card with `review`: the "operator ears" premise
fell on this host — NVDA 2026.2 is installed as a portable copy and its built-in speech log (`-l 12`) records
every utterance — so the pass is made AGENT-DRIVEN with ZERO new dependencies and the operator's role becomes
REVIEW of the evidence (the judgment class, done remotely). This section supersedes the bullets it names.
- **The pass** (supersedes _What this chunk builds_ 3 and the "recorded operator pass" wording elsewhere):
  the driven arm the harness already owns (WebdriverIO + tauri-driver, live Pulse) drives the four run states
  + HOLD while NVDA — started before the app with `-m --no-sr-flag -l 12 -f <log>`, quit with `-q` after —
  logs what it would speak; `evidence/nvda-pass.json` is PRODUCED BY THE LEG from the speech log and the leg's
  own action timeline, not by hand. The operator REVIEWS the evidence and the review is recorded in it.
- **The fallback** (operator-stated): whether NVDA attaches to the WebView2 window while WebDriver holds the
  session is untested — measured at implement; if it cannot, the documented fallback is the manual operator
  pass in the evening. The evidence records WHICH ARM produced each row (`agent` / `operator`), and a row the
  agent arm cannot reach (browse-mode reading the driver's injected keys may not trigger) falls to the
  operator arm the same way.
- **The harness is EXTENDED, never relaxed** (narrows _Boundaries_ "Not a harness change"): a third suite
  family over the ONE stack (`sr` live subject · `sr-empty` · `sr-error`), one new spec, one node-built-ins
  parser, a `CONDUCTOR_NVDA` host-tool handle guarded exactly like `CONDUCTOR_MSEDGEDRIVER` (unset ⇒ skip at
  exit 0), NVDA start/quit bracketing the driver, and per-suite subject env at the one spawn site. The
  routine arm's spec set, both existing specs and the dependency trees are untouched. Added on the `yes`
  (operator, 2026-09-02): at that same spawn site the existing `driven` suite gets its OWN runs dir
  (`runs/driven/runs`) instead of the lamps fixture dir, and `seedFixtureRuns` re-creates the fixture dir
  clean before the copy — today's spawn forces `runs/e2e-fixture` for every suite and the seed never cleans,
  so a driven or `sr` session's persisted journal pollutes the routine arm's subject (a latent flake in the
  lamps assertions, closed in passing).
- **Automation boundary** (supersedes _Boundaries_ "Not screen-reader automation"): NVDA's own log is the
  driver; no SR driver package, no NVDA add-on, no speech-capture library. Guidepup stays REJECTED for this
  chunk and is surfaced at wrap's route-resolve as a ROUTE CANDIDATE (owner: Epoch-6 _A11y CI gate_).
- **The directive, re-read:** the `[inferred]` reading is otherwise confirmed — the spec + ONE recorded pass;
  the chunk HOLDS across any seam (now the operator's review), never wrap-with-owed.
- **Process hygiene:** NVDA is the leg's to stop (`-q` in the stop form); Pulse is the operator's — the leg's
  census ends with "Pulse left running: operator stops it", and the operator stops it on that signal.
- **Host state at the relay (re-verified 2026-09-02):** the portable NVDA exe + `userConfig` present, no NVDA
  running; `pulse-app` up with `127.0.0.1:4317` LISTENING, deterministic L4 + MCP enabled, fresh data dir
  under `%TEMP%\pulse-legs\`, no canary fired; the sidecar built in the Pulse repo's release dir. Every one of
  these is a HOST value — handles and placeholders in the artifacts, never the values.

## What this chunk builds

1. **The per-state pass spec** — one committed, host-path-free artifact that, for each of the four run states
   the shipped webview renders, lists every must-announce item with: the action or key sequence that
   produces it · the node carrying it (role + accessible name, and the live-region mechanism where one
   exists) · the EXPECTED NVDA speech, written as the phrase NVDA's default verbosity yields from the
   accessible name/role/state (variable spans marked, e.g. the count value) · the WCAG SC it evidences
   (SC 4.1.3 status messages, SC 4.1.2 name/role/value, SC 1.3.1 landmarks) · a result cell
   (announced-as-expected / announced-differently / not-announced) with a "heard" column.
2. **The must-announce list**, derived from the four masters and reconciled against the SHIPPED DOM:
   a11y-plan §3 _Screen reader test pattern_ (empty-state real prose · in-progress announced as status not
   Fail · HOLD phase-line flip via `aria-live="assertive"` · frozen count value · verdict lamp `aria-live`
   status announcement · operator-checklist unticked-count roll-up); a11y-plan §4 (landmarks `main` /
   `contentinfo` / `banner`; lamp `status`; `alertdialog` + `aria-modal`; checklist `checkbox` +
   `aria-checked`; picker `combobox`/`listbox`; icon-only controls `aria-label`ed); design-system
   §Component Patterns 1 / 4 / 6 / 7; layout-templates §Component — Header / block 2 / Operator-checklist /
   Footer. Where the plan names prose or a node the shipped UI does not render, the spec names what SHIPS and
   the gap is recorded as a finding (see Boundaries).
3. **The recorded pass** [re-shaped by §Re-shape: produced by the agent-driven leg, reviewed by the
   operator] — one NVDA pass recorded in the chunk's evidence: each row's outcome, heard text and producing
   ARM · NVDA version · WebView2 runtime version · the build identity (commit) · the date · the operator's
   review · host-path-free. This is the acceptance's second clause.
4. **The stimulus recipe per state** — how each state is produced on this host, cheapest form first:
   `idle` needs only the launched release bundle (built `--features tauri/custom-protocol`, else the window
   opens on `devUrl`); `live` / `hold` / `aborted` need a run — `hold` needs a hold-declaring scenario
   (`halo-hue-encoding`) against a live preflight-ready Pulse in the driven-arm firing form the sweep
   registered (PATH prefix resolving the sidecar · `ANDROMEDA_PULSE_L4_DETERMINISTIC` · the shared data dir).
   [verified in research: no cheaper stimulus exists for `live` / `hold` / `aborted` — a no-Pulse run settles
   `Blocked` within seconds (before the canary poll), `Stop` is natively disabled while idle, and the backend
   emits `Aborted` only BETWEEN scenarios, so one live leg over a trimmed two-scenario dir exercises all three
   plus the terminal report; the idle-with-report sub-state alone has a no-Pulse stimulus — the committed
   `lamps-journal.jsonl` fixture seeded under `CONDUCTOR_RUNS_DIR=runs/e2e-fixture`, exactly as the routine
   `--e2e` arm does]

## The four run states — as shipped, not as the plan names them

The shipped `RunState` type (`Titlebar.tsx`) enumerates `idle` · `live` · `hold` · `aborted`, each with its
own titlebar label (`Conductor · idle` / `Conductor · live` / `Conductor · HOLD — operator pause` /
`Conductor · aborted`) in the `aria-live="assertive"` span. The a11y-plan (§1 entity, §3 pass-spec format,
§5) names the four as `idle / live / HOLD / report-terminal`. The terminal stages `done` / `blocked` settle
the titlebar to `idle` with the run-report section populated (`App.tsx` `STATE_FOR_STAGE`), so the plan's
"report-terminal" is the idle state WITH a report rendered — a sub-state of `idle`, not a distinct
`RunState` — while `aborted`, a distinct shipped label, is not among the plan's four. [verified in research
against `Titlebar.tsx:4-11` and `App.tsx:67-72`; layout-templates §Component — Header DOES name an abort
behaviour, so the omission is the a11y-plan's alone]

The spec therefore covers the four SHIPPED states, with the report-populated render treated as the terminal
sub-state of `idle`, and records the plan-vs-shipped naming divergence as a finding for wrap's amendment flow
(a11y-plan §1 / §3 / §5 name a state the code does not have, and omit one it does). Phase never edits the
masters.

## Boundaries — what this chunk does NOT do

- **Not screen-reader automation by a NEW dependency.** [superseded in part by §Re-shape] No SR driver
  package (Guidepup-class or otherwise), no NVDA add-on, no speech-capture library — NVDA's own speech log is
  the driver, and the a11y-plan's "no automated SR tool exists for the stack" was checked and measured false
  (recorded for wrap; Guidepup a route candidate).
- **Not a harness RE-AUTHORING.** [narrowed by §Re-shape] The axe / colorjs.io / WebdriverIO specs and the
  routine arm's spec set are not touched (F14: "Do NOT re-author the harness"); the harness is EXTENDED with
  the `sr*` suites, one spec, one parser and the `CONDUCTOR_NVDA` guard — extended, never relaxed (the
  `v2-22` clause). The A11y CI gate + violation JSON is `v2-24`; the envelope parity is `v2-25`; both stay
  pooled.
- **Not a UI fix, by default.** When the pass finds a must-announce item the shipped DOM cannot announce (a
  lamp without a live region, an empty state whose prose differs from the plan's), the chunk RECORDS it — a
  not-announced row in the evidence plus a finding for wrap — rather than fixing it: the acceptance is a
  recorded pass, not a green one, and a remedy is a separate route entry. An attribute-level remedy on an
  element the spec names is a P5 option the operator may pull into scope; it is not assumed. [a scope
  decision, not a premise — research named the three candidate remedies: `role="status"` on the lamp, an
  accessible name on the titlebar count, a `contentinfo` footer]
- **Not the CLI surface** — not-assertable (a11y-plan §1); its `[PASS]`/`[HOLD]` prefixes are output-stream
  discipline, not a screen-reader path.
- **Not a WCAG conformance claim.** The SR pass verifies runtime announcement quality only, supplemental to
  the automated baseline (a11y-plan §2 / §11 Universal); conformance evidence stays the axe / colorjs.io JSON
  of `v2-22`.
- **Not VoiceOver or Orca passes** — no macOS or Linux surface on this host (directive as read above).

## Surfaces and contracts touched

- **New artifacts:** the spec file and the evidence record. Their home is decided in the plan; the candidate is
  the ui test tree beside the harness the a11y-plan places the spec with (`crates/conductor-tauri/ui/test/…`),
  whose `README.md` still carries the pre-2026-09-01 Linux-only framing and names this spec as "the next
  chunk" — repointing that README at the spec is in scope only insofar as it names this artifact. [verified
  viable in research: `ui/test/` is git-tracked (4 files), the wdio `specs` / `suites` entries are literal
  `.e2e.ts` file paths and `test/tsconfig.json` compiles `a11y/**/*.ts` only, so a Markdown spec there is
  picked up by neither]
- **Read-only (the announcement sources):** `Titlebar.tsx` (assertive label, count span, `aria-label`ed
  min/close) · `App.tsx` (state wiring, empty/loading prose, `role="alert"` errors, the `<main>` landmark) ·
  `OperatorPauseDialog.tsx` · `OperatorChecklist.tsx` (the dialog-site checklist) · `OperatorChecklistView.tsx`
  (`role="status" aria-live="polite"` roll-up) · `StatusLamp.tsx` (glyph `aria-hidden`, visible text label) ·
  `CoverageMatrix.tsx` ("Not yet run" cell; `role="group"` scroll region) · `RunReport.tsx` · `RunControls.tsx`
  (`aria-disabled` start/stop) · `ScenarioPicker.tsx` (`aria-current`).
- **Matrix:** `v2-23` is concretized at P5 and claimed by this chunk ONLY if the operator pass can be recorded
  within it (the pass needs NVDA installed and the operator's session); if it cannot, the cap stays pooled per
  the contract and the spec ships as a partial advance.
- **Masters (wrap-owned; surfaced, never edited by phase):** a11y-plan §1 / §3 / §5 four-state naming vs
  shipped; §3 must-announce prose vs shipped prose; the ui test README's stale framing.

## Preconditions measured at take-up (2026-09-02)

- **NVDA is not installed on this host** — no Program Files install and no uninstall-registry key. The
  operator pass therefore starts with an NVDA install (installer or portable build, operator's choice); the
  spec records the version used. This is an operator action, not the agent's.
- WebView2 runtime 151.0.4129.107 — measured 2026-09-01 and re-read from the EdgeUpdate registry key on
  2026-09-02, unchanged — the surface NVDA reads (Chromium's IAccessible2 / UIA) on this host; that NVDA needs
  no configuration for it is confirmed at the operator's first launch, not from the repo.
- An automated NVDA driver EXISTS on the npm registry (`@guidepup/guidepup` 0.34.0, `@guidepup/setup` 0.25.2,
  both updated 2026-08-31/09-01) — the a11y-plan's "no automated SR tool exists for the stack" is false as a
  general claim. NOT adopted here (one-automation-stack rule, dependency gate, directive) — and, by operator
  ruling at P5, surfaced at wrap's route-resolve as a ROUTE CANDIDATE owned by the Epoch-6 _A11y CI gate_
  entry, since agent-driven is the standing default wherever a driver exists.
- The `hold` state needs a live preflight-ready Pulse with a hold-declaring scenario; the sweep's driven arm
  measured the firing form on 2026-09-01 and registered it in test-plan §6/§9.

## Premise closure (P3, 2026-09-02) — the take-up `[inferred]` list, each bullet closed

1. **VERIFIED.** The four states = the shipped `RunState` set; the plan's `report-terminal` = `idle` with the
   report rendered; `aborted` is shipped but unnamed by the a11y-plan (`Titlebar.tsx:4-11`, `App.tsx:67-72`).
2. **VERIFIED, sharpened.** Shipped prose: `No scenarios found.` · `No run yet` · NO in-progress prose (the live
   state rides the titlebar label + count only); plus three loading strings (`Loading scenarios…` /
   `Loading coverage…` / `Loading run report…`), three `role="alert"` error strings and the picker's
   `No scenarios match.` (`role="presentation"`) that the plan never names. The spec names what ships.
3. **VERIFIED.** The lamp carries no live region (`StatusLamp.tsx`: glyph `aria-hidden` + label text, no
   `role="status"`); the matrix / report scroll regions are `role="group"`. A lamp change is reached by
   navigation only.
4. **VERIFIED + [premise-corrected: the plan's second render site does not ship].** The dialog-site roll-up IS
   announced — `OperatorPauseDialog.tsx` renders the same `OperatorChecklistView` (`role="status"
   aria-live="polite"`, `{n} of {m} unconfirmed` / `All observations confirmed`). But the run report's
   `ManualCheck` rows do NOT expand into a checklist in the release bundle: `RunReport.tsx` renders lamps
   only, and the component's only other caller is the `import.meta.env.DEV`-gated Gallery (code-graph, 88-row
   caller query). Design §Component Patterns 6/7, layout-templates §Operator-checklist and a11y-plan §4 all
   describe two sites; one ships. Spec rows for the report-site checklist are therefore recorded as
   subject-absent, and the divergence is a finding for wrap.
5. **VERIFIED.** `main` and the implicit `banner` (`<header>`) ship, plus three named regions (`Capability
   coverage matrix` · `Run report` · `Operator checklist`) and named groups; NO footer, so NO `contentinfo`;
   three `<h2>` and no `<h1>` (the plan's phase-line Heading role is a styled `<span>`).
6. **VERIFIED, corrected.** `idle` (empty): launch the bundle. Idle-with-report: seed the committed fixture and
   launch with `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` — no Pulse. `live` / `hold` / `aborted`: one live leg —
   a no-Pulse run settles `Blocked` in seconds (unmet `shell-declaration` term, or egress-unreachable, both
   before the canary poll), `Stop` is natively disabled while idle, and `drive_run` polls the abort flag
   only between scenarios (`lib.rs:932-936`), so an observed `aborted` needs ≥2 scenarios with `Stop` pressed
   during the first (the label flips client-side at once, `App.tsx` `stop()`).
7. **VERIFIED — the plan claim is false as stated.** `@guidepup/guidepup` 0.34.0 + `@guidepup/setup` 0.25.2
   exist on npm (NVDA + VoiceOver automation, maintained). Finding only — see Preconditions.
8. **UNVERIFIED in the repo, confirmed at the pass.** NVDA reads WebView2 (Chromium: IAccessible2 / UIA) without
   configuration; the spec carries it as a precondition the first launch confirms.

New findings the spec RECORDS (never fixes here): the dialog title ships as `{p_id} — operator-checklist` with
description `Observe the operator-checklist claim for this scenario` (`pause.rs:38`, `lib.rs:535-542`), not the
plan's frozen-count + step-index header; the titlebar count is a bare unlabeled number (scenarios completed);
`Start` announces `aria-disabled` while `Stop` is natively disabled.

**Closed at the P5 review card (2026-09-02):** the reading of the standing NVDA directive was confirmed as
written, with the seam rule and the Guidepup route-candidate ruling recorded (§Standing operator directive
above). No `[inferred]` bullet remains.
