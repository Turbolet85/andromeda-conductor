# Report — 2026-09-07-sr-findings-fixed

**Chunk:** SR findings fixed — the three measured screen-reader findings closed at their defects, and the a11y CI gate's first red owned
**Date:** 2026-09-07
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:** **six product files** (`git diff --stat HEAD -- .github crates` → 6 files, 317+/39−):
  `.github/workflows/ci.yml` (117+) · `crates/conductor-tauri/ui/src/App.tsx` (54+) ·
  `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (82+) ·
  `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (95+) ·
  `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` (6±) ·
  `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (2±).
  Plus ledger/route/bookkeeping: `conductor-0.2.0/verification-matrix.json`, `master-route.md`,
  `working-route.md`, `.andromeda/friction-log.ndjson`, `.claude/session-handoff.md`, the chunk folder
  and the phase/wrap run dirs.
- **Symbols / APIs:**
  - NEW `activeProbe()` (`screen-reader.e2e.ts`) — bounded structural projection of `document.activeElement`
    (tag · id · role · label · testid · focusable index), never `textContent`.
  - CHANGED `bringToForeground(preFocusRow?: string)` — added optional parameter; **keeps its existing
    caller shape**, the two other subjects call it unchanged (`bringToForeground()`), so this is additive.
  - NEW `activeName()` + `tabCycleNames()` (`accessibility.e2e.ts`) — local to the routine spec file, not
    exported; `activeName()` deliberately names an `<input>` by label/placeholder because an input's
    `textContent` is always empty.
  - CHANGED `App.tsx` — new module const `SR_ONLY` (CSSProperties) and new state `loadErrorEcho` +
    `reannounced` ref; no exported surface changed, no prop added.
  - No Rust symbol changed — **zero `.rs` delta** in this chunk.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none added, none bumped. `Cargo.lock` untouched; `package.json` / `package-lock.json`
  untouched.
- **Schema / config:** none. The a11y violation-record envelope is byte-unchanged (the same
  `writeA11yEnvelope`); the SR record's `nvda-pass.json` shape is unchanged (`rows.ts` edited one row's
  prose fields, not the `SpecRow` interface).
- **Spec-master edits:** none in this chunk's own diff — the four expected amendments below are wrap's
  (P2) to apply.
- **Counts / qualifiers moved:** the routine a11y arm's spec count **10 → 12** (two hold-free Operable
  specs added; basis: the leg's own printed tally, `11 passing` before the fix / `12 passing` after,
  `/tmp/e2e-run1.log` vs `/tmp/e2e-run3.log`). The expected-skip SET is **unchanged at 2** (the two
  live-hold subjects). **CORRECTED at P2 — one master DOES bake the old value.** The first probe
  (`grep -rn '10 spec\|ten spec\|12 spec' .andromeda/*.md`) returned 0 hits and was reported as "no doc
  bakes it"; that probe keyed on a PROXY for the claim. `test-plan.md:307` spells it `10 passing / 2
  skipped` — `grep -rn '10 passing' .andromeda/*.md` returns that one hit, and `D-tests-derived-count`
  found it by searching for what the claim says rather than what I named it.
- **Dev-tool versions:** none installed or upgraded on this host. Measured on the CI runner image
  (diagnostic, not a change): msedgedriver / WebView2 Evergreen Runtime / Edge all **151.0.4129.101**;
  this dev host runs **152.0.4191.66** with msedgedriver 152.0.4191.53.
- **Harness / gate surface:**
  - `ci.yml` a11y job: `CONDUCTOR_MSEDGEDRIVER` moved OUT of the step's `env:` block and resolved in the
    step SHELL from `$env:EDGEWEBDRIVER`, with a printed handle-named `Test-Path` precondition that exits
    non-zero (handle NAME + boolean only, never the path).
  - `ci.yml` a11y job: `$PSNativeCommandUseErrorActionPreference = $true` REMOVED from the gate step.
  - `ci.yml` a11y job: **two new diagnostic steps** — *WebView2 driver + runtime versions* and *WebView2
    session isolation* — both `continue-on-error: true`, the second additionally `if: always()`, plus an
    `if: always()` artifact upload (`a11y-session-diag`). The GATE step itself carries no
    `continue-on-error` (a hollow green is banned).
  - `scripts/agent-run.{sh,ps1}`: **byte-unchanged** — the parity assertion was verified, not edited.
- **Cross-project / external claims:** the CI runner image's driver/runtime/Edge versions and its WebView2
  session behaviour — ground truth is the GitHub `windows-2025` image, read via runs `34157101273`,
  `34158355397`, `34160378753`, `34162118841` and the uploaded verbose driver log
  (`evidence/ci-msedgedriver-verbose.log`). No claim about the Pulse repo.
- **Reverted / negative API facts:** the first re-announce design placed the echo node INSIDE the same
  `role="alert"` region as the visible paragraph; it shipped nowhere — NVDA read the region whole and spoke
  the message twice (2 occurrences on one speaking line). Replaced before commit by moving the visible copy
  outside the region.
- **Insufficient fixes (written, kept, not the remedy):** the `ci.yml` driver-path resolution and the
  error-preference removal are both CORRECT, both proven in CI, and neither makes the a11y job green — the
  job now fails further down at WebView2 session creation. Owner: the newly minted route entry
  *Hosted-runner WebView2 session*.
- **Spec claims disproved by measurement:**
  1. **Route CARRY (working-route `:123`) — "the sequential-focus start point is still wrong; the first Tab
     landed on Close window".** Measured absent at HEAD: `initialFocus BODY`, `firstTabTarget "Minimize
     window"`, `firstTabProbe focusableIndex=0/5`, `tabsToStart 5` over 5 focusables = a full cycle
     (`evidence/actions.error.run1.jsonl`); R0-02 grades `announced-as-expected` in both runs; the routine
     arm's new SC 2.4.3 spec independently measures `Minimize window then Close window`. Its stated holder
     ("the picker input at mount") is also unsupported — `ScenarioPicker.tsx` has no focus site and cmdk's
     only `.focus()` path is guarded. **Disposition: no master states this — it is route-CARRY freight, and
     the corrected statement lives in `scope.md` (`[premise-corrected]`) + this report. **CORRECTED at P2:
     "no master states this" was wrong — `a11y-plan.md:516` carries the premise inside the carve-out
     sentence ("the sequential-focus start-point defect is open and its holder unmeasured"). Caught by
     `D-a11y-surface`; dispositioned by that amendment.**
  2. **Route CARRY (working-route `:123`) — "the leg has real run-to-run announcement variance".** Two runs
     per subject on the identical tree: error, empty and live each byte-identical grade sets (live: 34
     announced-as-expected / 15 not-run-here / 2 subject-absent, zero differences).
     **CORRECTED AT THE WRAP LIGHT GATE — it DOES reproduce, and the two-run conclusion was wrong.** The
     gate's THIRD live run of the same unchanged tree graded **33 / 15 / 2 / 1**: `S3-04` ("Escape resolves
     NoGo; focus restored") moved `announced-as-expected` → `not-announced`, `heard: []`, on a session
     whose `nvda_named_window` was `false` (`evidence/nvda-pass.live.run3-lightgate.json`, live subject
     recorded 2026-09-07T22:09:30.914Z). The CARRY's own warning — "ONE observation cannot separate
     variance from regression" — applies to a TWO-observation conclusion just as much: two identical runs
     are two samples, not proof of stability. The variance is REAL and rarer than first described (1 row of
     51, on the third of three runs), sits against test-plan §10's zero-flake bar, and is NOT closed here.
     **Disposition: no master states it; CARRY'd to *Release build and bundle*, whose final SLO
     verification pass is its plausible owner (route-resolve: carry-forward to a specific later entry is
     factual → AUTO).**
  3. **`verification-matrix.json#v2-24` acceptance — "the same specs run on Linux+xvfb".** Already recorded
     superseded by the prior chunk's own note; this chunk's concretization rewrote it to the measured
     Windows WebView2 runner. **Disposition: ledger, written at phase P5; nothing owed to a master.**
  4. **My own implement-stage reading — "msedgedriver launched the Tauri binary as though it were Edge".**
     RETRACTED as a probe artifact: the isolation POST omitted `browserName:"webview2"`, and the identical
     POST fails on THIS dev host too against a binary that passes 12/12 through tauri-driver (re-measured
     here: with `browserName` added it returns 200 with `goog:processID` + `debuggerAddress`).
     **Disposition: enters NO master. Retracted in `v2-24`'s `notes` and by friction record
     `2026-09-07T21:10:45Z-a`, which `retracts` the record carrying it.**
- **Expected amendments (from plan):**
  1. `a11y-plan.md` §11 Strategy (`:516`) — the Operable carve-out. **Carried.** Its stated structural
     condition ("until a hold-free keyboard path exists on the routine arm") is now MET: the routine arm
     carries SC 2.1.1 reachability + SC 2.4.3 focus order and is green locally at 12 passing / 2 skipped.
     Its second clause (the CI proof) is NOT met and is owned by the new route entry. Site search:
     `grep -nc 'CARVE-OUT\|carve-out' .andromeda/a11y-plan.md` → **1 hit** (a11y-plan only).
  2. `a11y-plan.md` §5 — record the routine arm as carrying the hold-free Operable pair. **Carried.** Site
     search: `grep -nc 'run-console-idle' .andromeda/a11y-plan.md` → **1 hit**.
  3. `test-plan.md` §3 — the `--e2e` printed-verdict contract holds only where the invoking environment does
     not preempt the capture-then-print; the `.sh`/`.ps1` identical-semantics contract broke under CI's
     error preference while both scripts stayed byte-unchanged. **Carried.** Site search:
     `grep -nc 'printed verdict\|PRINTED verdict\|identical semantics' .andromeda/test-plan.md` → **2 hits**.
  4. `matrix#v2-24 notes` — the concretization record. **Superseded by events**: the cap was CONCRETIZED at
     phase P5 and then UN-CLAIMED at implement on the operator's ruling, so the note that landed is the
     un-claim + the RETRACTION, not the concretization line the plan anticipated. Written already;
     **ledger-note — owner P7.3** confirms status/ref/acceptance untouched.
- **Coverage of new surfaces:**
  - `App.tsx` load-error re-announce region → validation n/a · instrumentation n/a (browser frontend emits
    no spans by obs-plan §11) · PII n/a (renders `sanitize_error` output; the SR ingest scrub is the second
    line) · tests `e2e` (SR row R0-01, graded twice) · a11y `WCAG✓` (SC 4.1.3, pass-spec row updated) ·
    tokens `design-token✓` (`var(--status-fail)`; `SR_ONLY` is a11y clip-rect mechanics, not a design value)
  - routine-arm SC 2.1.1 keyboard-reachability spec → validation n/a · instrumentation n/a · PII n/a ·
    tests `e2e` (green in the routine arm) · a11y `WCAG✓` · tokens n/a
  - routine-arm SC 2.4.3 focus-order spec → validation n/a · instrumentation n/a · PII n/a · tests `e2e` ·
    a11y `WCAG✓` (asserts no `tabindex > 0`) · tokens n/a
  - `ci.yml` two diagnostic steps + artifact upload → validation n/a · instrumentation n/a · PII
    `redacted✓` (handle NAME + boolean only; the uploaded log is path-scrubbed to `<ci-workspace>` /
    `<driver-udf>`, verified 0 residual) · tests `unrunnable-here` (a CI-only surface; exercised by probe
    runs, not by a local gate) · a11y n/a · tokens n/a

## Deviations from intent

1. **Two diagnostic `ci.yml` steps were added — not in the plan's touchpoints for that purpose.**
   Justification: the plan's step 11 charter is "own the fix", and both named suspects had to be measured
   before either could be dispositioned. Both are `continue-on-error`, the second `if: always()`, and the
   gate step itself takes neither — so they can never decide the gate. Operator-directed for the second
   probe round.
2. **The `v2-24` un-claim was performed at implement, though the release valve is the wrap's.**
   Justification: performed on the operator's explicit ruling in-session ("a platform-level cause → un-claim
   v2-24 with that measured premise in its notes"). Consequence for this wrap: the coverage gate is a no-op
   rather than a HALT, because the chunk now claims nothing. Recorded here so the valve's use is visible at
   its proper phase.
3. **Plan step 6 (fix the sequential-focus start point) shipped as a NO-OP.** Justification: step 5's
   measurement — the instrument the plan required *before* any remedy — found no defect
   (`initialFocus BODY`, first Tab → `Minimize window`, `focusableIndex 0/5`, `tabsToStart 5` = a full
   cycle). What step 6 delivered instead is the correction of the leg's own now-unsupported comment, which
   the step also asked for.
4. **The re-announce mechanism was redesigned mid-implement.** Justification: the first shape (echo node
   inside the `role="alert"` region) made NVDA read the region whole and speak the message twice, measured
   in the speech log. Final shape: the visible copy sits OUTSIDE the region and the region carries the
   re-assertion alone — one node, one insert, one utterance. This site now differs deliberately from the
   three sibling error regions, which keep the empty-mount shape because they fire at arbitrary times.
5. **Plan command (5)'s firing form was incomplete and command (4) named the wrong nextest form.** (5)
   omitted the `PATH` prefix resolving `andromeda-pulse-mcp`, so two `sr` live runs blocked instantly
   (`state=Blocked`, rule `:54`'s signature) before the prefix was added; (4) used nextest's positional
   FILTER form where `--test journal_conformance` is the target form CI itself uses (exit 4, 0 tests run,
   which reads as green if unexamined). Both corrected in-session; the plan text is frozen, so they are
   recorded here and curated.

## Decisions & corrections

- **Operator ruling (mechanism):** my "launched as though it were Edge" diagnosis was falsified — the
  isolation probe omitted `browserName:"webview2"` and therefore measured itself. Rule taken: *validate any
  diagnostic form on the dev host first; a probe that fails where the real path passes measures itself.*
- **Operator ruling (scope):** the hosted-runner failure is not a route-level question — diagnose it here,
  one probe. Done; the corrected probe returned the bounded fact now carried on the new route entry.
- **Operator ruling (valve):** un-claim `v2-24` with the measured premise rather than leaving a hollow
  claim, and replace the evidence log with the corrected probe's.
- **Correction (mine, twice):** the decisive `DevToolsActivePort` line was present in both earlier probe
  logs and missed because I grepped `ERROR|Failed` while the driver logs its retried attempts at `WARN`.
- **Correction (mine):** `gh run watch --exit-status` returned 0 for a run `gh run view` reports as
  `failure`; the queried verdict outranks the wrapper's exit.
- **Correction (mine):** the first scrub of the evidence log ran as an inline `python -c` carrying Windows
  path literals; the escaping was mangled, `str.replace` no-matched, and the success line printed anyway —
  the artifact kept a residual CI workspace path. Caught by grepping the WRITTEN file; redone as a
  scratchpad script run by path.
- **Correction (operator, dispositioned here):** "5 files" in the implement report was wrong — the chunk
  touches **six** product files.
- **Measured, dispositioned — NOT an SR finding:** on the `error`-subject captures E0-06 grades
  `not-announced` (heard `[]`) while E0-07 carries the region utterance, inverted from the `empty` and
  `live` captures. **Cause: an ingest attribution artifact.** `nvda-pass.json` is CUMULATIVE per subject
  (`subjects.{empty,error,live}.recorded_at`), so an `error` run leaves the `empty` rows exactly as the last
  `empty` run left them. The error captures were taken before this session's `empty` runs, so their E0 rows
  carry `2026-09-04T20:08:05.767Z` — and that prior record
  (`chunks/2026-09-04-sidecar-spawn-without-a-console-window/evidence/nvda-pass.json`) holds E0-06
  `not-announced` / E0-07 `announced-as-expected`, byte-identical. Comparing E0 rows across those captures
  compares two DATES, not two subjects. This session's actual `empty` measurement (19:33 / 19:34) is E0-06
  `announced-as-expected` in both runs — an improvement on 2026-09-04, not a regression.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

| Criterion | Verdict |
|---|---|
| (arch) `a11y` job on pinned `windows-2025`, driver from the image, no install step | **MET** — label unchanged; the driver is resolved from `$env:EDGEWEBDRIVER`, no install step added |
| (a11y) a CI run of `a11y` completes GREEN, zero axe violations, produces `runs/a11y/<run_id>.jsonl` | **UNMET** — the job fails at WebView2 session creation. Routed to the new route entry *Hosted-runner WebView2 session*. Not matrix-linked (the cap was un-claimed), so it is reported here, not as a P7 post-claim disproof |
| (a11y) routine arm carries the hold-free Operable pair, both PASS not skip | **MET** — 12 passing / 2 skipped, expected-skip set unchanged |
| (obs) a RED routine arm's wdio output reaches the job log | **MET** — proven in CI (full stack traces + `Spec Files:` summary present in run `34157101273` onward) and locally with a control |
| (obs) violation record carries the eleven keys + resource tags | **MET** — `journal_conformance` 8/8 over `runs/a11y` |
| (security) wdio-edge guards byte-unchanged; both strict/lax arms hold | **MET** — `nativeDriver()`/`nvdaExe()`/`UNSAFE_PATH` untouched in the diff; strict → exit 1, lax → exit 0 |
| (security) no host path in the precondition line, restored output or evidence record | **MET** — handle NAME + boolean; uploaded log scrubbed, 0 residual (grep-verified on the written file) |
| (tests) both shells assert the printed verdict at identical semantics | **MET** — both byte-unchanged and verified equivalent term-for-term |
| (a11y) finding (1) graded `announced-as-expected` with operator review | **MET** — R0-01 announced once, both runs; 0 `security_finding`, 0 `<host-path>` |
| (a11y) finding (2)'s holder measured before any remedy | **MET** — measured, and the defect measured ABSENT |
| (tests) finding (3) dispositioned across TWO runs, no retries/pinning/sleep | **MET** — two runs per subject, byte-identical grades; no retry, no pin, no sleep added |
| (design) contrast tokens byte-unchanged | **MET** — `--text-tertiary` / `--text-muted` absent from the diff |

**Gates green** (commands run): `agent-run.sh run --e2e` (12 passing / 2 skipped / verdict asserted, bare
exit 0) · strict arm exit 1 / lax arm exit 0 · `cargo nextest run -p conductor-run --test
journal_conformance` 8/8 · `cargo audit` 0 (17 allowed) · `cargo deny check advisories bans licenses
sources` 0 · `npm audit --omit=dev` 0 vulnerabilities · advisory-db `faedffd5`, porcelain clean ·
`npx tsc --noEmit` + `npm run typecheck:e2e` clean. **Deferred**: the workspace Rust gates
(`nextest --workspace`, `--doc`, `clippy`) — this chunk has ZERO `.rs` delta and no manifest/lockfile
touch; the `--e2e` arm's own `cargo build --release -p conductor-tauri --features tauri/custom-protocol`
proves the backend still compiles. Re-run at the next source-touching chunk.

**Smoke:** the UI self-verify ran as a P2 gate (a UI-surface chunk with a headful self-verify listed in
Test Commands) — recorded, not re-run at P3.

**Outcome basis:** implement's P4 report, PLUS two operator directives that followed it (the "diagnose it
here, one probe" ruling and the mechanism falsification) and the post-implement artifacts they produced —
CI runs `34160378753` and `34162118841` and `evidence/ci-msedgedriver-verbose.log` (replaced with the
corrected webview2-mode log). A detector reading this report must not inherit the superseded Edge-mode
claim; it is listed above only as a retracted item.

**Process hygiene** (implement P4's census, re-measured at this wrap — the host list is readable here):

| Process | Started by | Final state |
|---|---|---|
| `nvda.exe` | the SR legs (this run) | terminated (`nvda -q`) |
| `conductor-tauri.exe` · `msedgedriver.exe` · `tauri-driver` · `node.exe` | the a11y + SR legs (this run) | terminated — wdio `onComplete` → `tauriDriver.kill()`; re-measured 0 of each |
| `msedgedriver.exe` (dev-host validation probe) | this run's `browserName` validation | terminated (session DELETE + `Stop-Process`) |
| `pulse-app.exe` PID 63180 | **the operator**, for this run | **left running — the operator stops it after the commit** |
| loopback `4444` / `4445` | the driver stack | no listeners (re-measured 0) |
| `ci-probe/2026-09-07-sr-findings-fixed` ref | this run's probes | deleted; runs `34157101273` · `34158355397` · `34160378753` · `34162118841` remain viewable by id |

Overseer residue on the host, all gitignored: `runs/a11y/2026-09-07T21-05-58-a11y.jsonl` ·
`runs/a11y-e2e.log` · `%TEMP%` msedgedriver scoped dirs.
