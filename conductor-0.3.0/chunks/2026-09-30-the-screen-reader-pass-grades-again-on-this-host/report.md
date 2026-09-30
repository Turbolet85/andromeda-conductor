# Report — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

**Chunk:** The screen-reader pass grades again on this host
**Date:** 2026-09-30T12:30Z
**Commits:** none of this chunk's own — HEAD is the chunk base `2c9b37d` (the prior chunk's wrap commit); no operator
pre-CI commit for this marker (`git log -F --grep 'chore(2026-09-30-the-screen-reader-pass-grades-again-on-this-host): operator pre-CI commit' HEAD` → 0 lines).

**Branch B, as the plan forecast** (plan §Goal: "the key is restored byte-identical, and the chunk records the
measurement and stops"; §Implementation notes: "Under Branch B the three regrade `jq` entries and the `recorded_at`
entry read red BY CONSTRUCTION (no regrade legs fire)"). The single-variable control `[UIA] allowInChromium = 2`,
witnessed applied, did not bring focus speech back.

## Changes (structured — detectors read this)
- **Files:**
  - new — `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/cause-control.md`
    (the two arms, the configuration table, the silence shape, the witness, the IA2 basis, the verdict, the USER
    consequence, the product lever named, the census), `evidence/nvda-pass.defaults.json` (the working record
    snapshotted after the K-reproduce arm, sha256 prefix `8dfd77f3818c27a1`), `evidence/nvda-pass.json` (the working
    record as the `= 2` arm left it, sha256 prefix `41c37f2227726832`), `scope-record.md` (one in-intent line).
  - edited then restored — `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-config/nvda.ini`: a `[UIA]` section
    (`allowInChromium = 2` + a four-line reason comment) was added for the K-object-model leg only, then removed; net
    diff over `2c9b37d` is EMPTY (`git diff --quiet 2c9b37d -- <file>` exit 0; the plan's report-only diff entry
    recorded 0 bytes).
  - no file under `crates` / `scripts` / `contracts` / `scenarios` / `.github` / `Cargo.lock` / `Cargo.toml` changed
    (the delta-guard entry: no output, exit 1).
- **Symbols / APIs:** none.
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none shipped. The leg profile `nvda.ini` is net-unchanged; NVDA's `[UIA] allowInChromium` key
  (NVDA `configSpec.py` master: `integer(0, 3, default=0)`, `0:default, 1:Only when necessary, 2:yes, 3:no`) was
  exercised at `2` for one leg and is NOT the leg's posture.
- **Spec-master edits:** none by implement.
- **Counts / qualifiers moved:** none — verified. No master states an SR focus-row tally (`grep -c 'not-announced'` over
  the seven masters: 0 in each). The plan's forecast "21 non-`announced-as-expected` focus rows" is a plan figure, not a
  master value; the committed record reads 24 (below).
- **Dev-tool versions:** none — the driver `msedgedriver` re-read at 154.0.4258.37 and the WebView2 runtime re-read at
  154.0.4258.37 on the dev host (coherence entry, 2026-09-30T12:14Z: `driver 154.0.4258.37 runtime 154.0.4258.37`,
  unchanged from the prior chunk); the screen reader NVDA re-read at 2026.2 (product version + both legs' log banners);
  OS re-read at 26200.9457 (`CurrentBuild`.`UBR`). The machine-wide Evergreen runtime directories on disk:
  `153.0.4234.48` (created 2026-09-20) and `154.0.4258.37` (created 2026-09-29).
- **Harness / gate surface:** none changed. The three `sr*` suites and `--e2e` ran in their registered firing forms;
  `CONDUCTOR_NVDA` (registered handle) was set per command to the one portable NVDA copy on the host — it is unset in
  this session's environment and not persisted at User or Machine scope.
- **Cross-project / external claims:**
  - NVDA 2026.2 behaviour, read from its own log: under `allowInChromium = 2` it took the key (no configuration/validation
    line; its exit-saved profile still holds `= 2`) and switched to UIA naming; under the default it builds the
    in-process IA2 buffer inside `msedgewebview2.exe` (the `vbufBase` teardown at `runs/sr-leg/nvda-speech.error.log:471-472`,
    absent in the `= 2` log). Basis: `runs/sr-leg/nvda-speech.{error,empty}.log` of this chunk's two legs.
  - NVDA source (`configSpec.py`, `UIAHandler/__init__.py` `_isUIAWindowHelper`, master branch) and wry
    `0.55.1/src/webview2/mod.rs:294-327` — read by research, cited in `cause-control.md`.
  - CI: none read by this chunk's gates (no CI entry; phase read CI#36709475243 green on `2c9b37d`, scope.md:36-41).
- **Reverted / negative API facts:** the `[UIA] allowInChromium = 2` leg posture — written for the K-object-model leg,
  reverted because that leg stayed silent (Branch B). It almost became the SR leg's stated AT posture (Branch A).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. a11y-plan §3 *Screen reader test pattern* states the measured platform
  set as "NVDA/Windows agent-driven with all three subjects attached (2026-09-02, v2-23)" — a dated measurement that
  stays true for its date; this chunk adds a later, configuration-bound measurement beside it (next bullet), it does not
  falsify it. The CARRY's hypothesis ("the WebView2 runtime moving to 154.0.4258.37 … is the variable that changed —
  untested") stays UNTESTED: no arm varied the runtime.
- **Measured findings (configuration-bound; the fact the Branch-B expected amendment carries):**
  - On WebView2 runtime/driver 154.0.4258.37 / 154.0.4258.37, Windows 26200.9457, NVDA 2026.2, the agent arm hears
    Conductor's webview focus only in the window's first burst and then NO later focus change, under BOTH of NVDA's
    Chromium object models: default (IA2) — `sr-error` 12:17Z, R0-02/R0-03/R0-04 `not-announced` (heard on 2026-09-07
    at 152.0.4191.66); forced UIA (`allowInChromium = 2`) — `sr-empty` 12:19Z, E0-02..E0-06 `not-announced`. In both
    logs NVDA recorded no entry of any kind between the burst and the app's close. Live regions still speak (R0-01
    heard). So the cause is not NVDA's object-model choice; the still-standing candidates are the runtime/driver pair
    (152 → 154, 153 between, its runtime dir still on disk), the Windows CUs KB5124008/KB5129195, and the desktop.
    Recorded as a reading, not an established mechanism (`cause-control.md` §Verdict).
  - USER consequence: an NVDA user on this configuration hears no focus change in Conductor after the first burst
    (`cause-control.md` §The USER consequence).
  - Product-side lever, named and NOT built (awaits the founder's live word): Tauri per-window `additionalBrowserArgs`
    (replaces wry's default `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`, so it must re-include it)
    or the WebView2 loader's additional-browser-arguments variable; no Chromium argument is nameable from source.
  - UIA naming observation: under UIA the titlebar buttons are spoken by glyph ("–", "✕"), not by their `aria-label`, so a
    regrade under UIA would read them `announced-differently`.
  - A WebView2 host window named by the runtime executable's absolute path is spoken by NVDA before activation and at
    close (12:19:44.889Z, 12:20:27.611Z) — pre-existing (prior chunk `driven-leg.md` Findings 1 and 3), outside every
    row window; the committed records carry no host path and 0 `security_finding` rows.
- **Expected amendments (from plan):**
  - Branch A · test-plan §6 desktop-webview row (`allowInChromium = 2` as the SR leg posture) — **superseded:** Branch B;
    the posture was not adopted and `nvda.ini` is net-unchanged. (`grep -c allowInChromium` test-plan.md: 0.)
  - Branch A · a11y-plan §3 *Screen reader test pattern* (verdict bound to NVDA's object-model setting, user-consequence
    finding) — **superseded:** Branch B; its Branch-B sibling below carries the configuration-bound finding instead.
  - Branch A · test-plan §9 Matrix builds (154.0.4258.37 × 154.0.4258.37 joins the passing set under the stated NVDA
    setting) — **superseded:** Branch B; the pair is NOT a passing SR configuration under either setting.
    (`grep -c '154\.0\.4258\.37'` test-plan.md: 0.)
  - Branch B · a11y-plan §3 *Screen reader test pattern* — the agent arm's focus rows are silent on 154.0.4258.37 /
    26200.9457 under both NVDA object models; a dated finding, configuration-bound — **carried:** Changes → *Measured
    findings*. Site search: `grep -n 'Screen reader test pattern' a11y-plan.md` → 3 hits (`:113` the §-summary,
    `:266` the section heading, `:298` the bootstrap-phase pointer); the section body's first bullet (`:268`) states
    the measured platform set. 0 hits in the other six masters.
- **Coverage of new surfaces:** none — no new external surface, hot-path operation or UI element.

## Deviations from intent
- **The cause is not removed** (scope.md §What this chunk builds #1–#3): Branch B, ruled at P4 as the chunk's stop
  ("the chunk records it and stops, and a runtime control would need its own ratification"). S0-09 / E0-05 and the other
  silent focus rows are NOT regraded; the plan's three regrade entries read red by construction (Outcome).
- **The forecast read 24, not 21**: the 21 counted the 2026-09-30 record, whose `error` subject was the 2026-09-07
  heard run; slot 1 refreshed that subject at defaults, adding R0-02..R0-04 `not-announced`. Measured, not widened.
- **`CONDUCTOR_NVDA` supplied per command**: the handle is unset in this session and not persisted; the one portable
  NVDA 2026.2 copy was located by a bounded search of the user profile and tool roots. No spawn form changed.
- **A third witness beside the plan's two** (step 4): the behavioural change (UIA wording, no `vbufBase` teardown) is
  recorded in `cause-control.md` §Witness as (c).
- Scope record — in-intent · `evidence/nvda-pass.defaults.json` · serves step 3 · self. `gate.py scope` (this wrap,
  2026-09-30T12:29Z): `scope: clean — changed 0 · listed 0 · recorded 0 … excluded 42`; the record line reads
  `not changed` because chunk evidence is excluded from the scope read.

## Decisions & corrections
- **Operator (take-up and each slot):** every NVDA run is a quiet-desktop slot, asked for one at a time; a focus theft
  onto a non-Conductor window VOIDS a leg rather than grading it. Both legs were checked for that before being graded:
  the terminal-output notifications NVDA logs only while a terminal holds focus stop before the graded rows and resume
  only after the app closes.
- **Operator relay (this wrap):** keep "a reading, not an established mechanism"; keep the mixed-configuration sentence
  where a reader of the record meets it; mint ONE successor entry before `:65` (the SR cause isolated on this host), its
  controls cheapest-first — a no-boundary control (NVDA at defaults over a plain focus page in Edge 154, and/or another
  WebView2 app) before the 153-runtime control (a Boundary-widening crossing, awaiting the founder's word); a
  Windows-update rollback is not a plannable control.
- **Sweep hazards found:**
  - NVDA's log is two lines per entry (a header with the local time, then the body): `grep -n` on a body token and
    `awk -F'[()]'` on the matched line read the BODY, not the header's time — pair them by header parse, not by grep.
  - `startNvda` deletes the previous session's log (`wdio.conf.ts:207`), so a log coordinate cited by an earlier
    reading is destroyed by the next leg of the same subject — the research's `:309-311` / `:351-355` teardown lines no
    longer exist on disk; cite such coordinates with their run, and snapshot what a later step must quote.
  - `nvda-pass.json` is cumulative per subject, so a forecast counted over a prior record moves when any subject is
    refreshed (21 → 24 here) — count over the record the gate will read.
  - Non-discriminating signals, measured: "Landscape" (×2 heard 09-07, ×3 silent default 09-30, ×0 silent UIA 09-30)
    and `@foreground.nvdaNamedWindow: false` (present on heard 2026-09-07 runs too).
  - The Bash PreToolUse guard rejects any command carrying a doubled backslash, including a regex; log readers go
    through a scratchpad script run by path.

## Outcome
- **Acceptance criteria**, against the diff (net: chunk evidence + a scope record; `nvda.ini` restored):
  - (tests) single-variable measurement against a control — **MET**: both arms in `cause-control.md` with their
    runtime × driver × NVDA × OS × `allowInChromium` × bundle rows; the report-only `nvda.ini` diff reads empty (Branch B).
  - (a11y) Branch A regrade — **N/A (Branch B)**.
  - (a11y) Branch B — **MET**: no row graded a pass it did not earn; `cause-control.md` names the `= 2` arm's silence
    with its configuration and the candidates still standing; `nvda.ini` byte-identical to `2c9b37d`.
  - (a11y/obs) USER consequence + product lever named, unbuilt — **MET** (`cause-control.md`).
  - (security) evidence host-path-free (hygiene entry 0) and closed sets / scrub pairing (0) — **MET**.
  - (security/arch) nothing under the code/harness/contract/scenario/CI trees changes — **MET** (no output, exit 1).
  - (obs) Branch A S1-01 witness — **N/A (Branch B)**; the entry still reads 0 over the committed record.
  - (design/layouts) no `ui/src` change — **MET**.
  - (tests) strict `--e2e` green — **MET**.
  - (arch) post-leg census 0 harness images, everything started stopped — **MET**.
- **Gates** (implement's runs, 2026-09-30T12:14Z–12:25Z):
  - `bash scripts/agent-run.sh run --unit` — not run — defer (key): zero Rust delta (re-read: no uncommitted file name
    is read by a Rust test; the one basename hit, `security.md`, is a doc comment in `jsonrpc_line_bound.rs:3` naming
    the unmodified rules file).
  - `cargo clippy --workspace --all-targets -- -D warnings` — not run — defer (key), as above.
  - `cd crates/conductor-tauri/ui && npm run typecheck:e2e` — green · exit 0.
  - driver/runtime coherence probe — green · exit 0 · `contains driver ` (`driver 154.0.4258.37 runtime 154.0.4258.37`).
  - `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` — green ×2 · exit 0 · `[a11y] verdict asserted — 0
    failed · 2 skipped (expected 2) · driven session present`, 16 passing; the first run relinked the release binary at
    the chunk base.
  - `npm run a11y:sr-error` — leg operator, fired by hand once (slot 1, NVDA defaults): exit 0, `Spec Files:` TAB ` 1
    passed, 1 total` held; recorded in `evidence/cause-control.md` §Operator-entry results.
  - `npm run a11y:sr-empty` — leg operator, fired by hand once (slot 2, `allowInChromium = 2`): exit 0, atom held;
    recorded there.
  - `conductor preconditions` (live probe) — not run — env `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_MCP_ENABLED`,
    `ANDROMEDA_PULSE_L4_DETERMINISTIC` unset; Branch A only, no `:4317` slot requested.
  - `npm run a11y:sr` (live) — leg live, not driven; Branch A only.
  - focus-row count `jq` — **red · last line '0' (last: '24')** — BY CONSTRUCTION under Branch B, as the plan forecast
    ("Under Branch B the three regrade `jq` entries and the `recorded_at` entry read red BY CONSTRUCTION (no regrade
    legs fire)"). This chunk's honest outcome, not a gate defect.
  - S0-09 / E0-05 / E0-09 `jq` — **red · contains 'S0-09 announced-as-expected'** — by construction (reads `S0-09
    not-announced`, `E0-05 not-announced`, `E0-09 not-run-here`).
  - one-configuration `recorded_at` `jq` — **red · last line 'true' (last: 'false')** — by construction (the `live`
    subject is the prior chunk's 10:49:56Z run).
  - The three reds, in the P7.1 ASSERT's form: `red — not this chunk's: P5 baseline red (each entry's `baseline`), and
    the same three filters read 21 / "S0-09 not-announced, E0-05 not-announced" / false over the parent's untouched
    record (`conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/evidence/nvda-pass.json`,
    `git diff --quiet 2c9b37d` exit 0) — the host's focus silence predates this chunk, which measured it and did not
    remove it → CARRY on "The SR cause isolated on this host" (minted this wrap, `working-route.md:67`), which owns the
    regrade.`
  - closed-set / scrub-pairing `jq` — green · `last line 0`.
  - S1-01 / `security_finding` `jq` — green · `last line 0`.
  - `git diff 2c9b37d -- …/nvda.ini` — recorded (0 bytes: Branch B).
  - evidence host-path hygiene `grep -c` — green · exit 1 · `last line 0`.
  - delta guard — green · exit 1 · no output.
  - `tasklist` census — green · exit 1 · `last line 0`.
- **Smoke:** no boot-path change; the self-verify entry ran as a P2 gate (above).
- **Watches:** none folded.
- **Outcome basis:** implement's P4 report as given in this session's conversation (no operator pass, no directive
  between implement and this report); post-implement artifacts: `evidence/cause-control.md`, `evidence/nvda-pass*.json`.
- **Process hygiene:** implement's census (`Win32_Process` with parentage, before the work and after every leg; all four
  readings equal the baseline of six SearchHost-owned `msedgewebview2.exe`, created 2026-09-26, not this session's). Every
  driver, app, webview child and NVDA instance the `--e2e` and `sr*` legs started: terminated. `pulse-app` /
  `andromeda-pulse-mcp`: never started. Re-measured at this wrap (2026-09-30T12:30Z): `tasklist` over `msedgedriver`,
  `conductor-tauri`, `nvda`, `tauri-driver`, `pulse-app`, `andromeda-pulse-mcp` → 0.
