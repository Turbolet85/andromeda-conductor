# Report — 2026-09-11-hosted-runner-endpoint-cause-closed

**Chunk:** Hosted-runner endpoint cause closed — the two measurements the probing chunk could not make, plus a
third arm the first one's reading made possible.
**Date:** 2026-09-12
**Commits:** `c93a379` chore(ci): take the module reading inside the isolation step's live window (diagnostic) ·
`eecc7f4` chore(ci): make the Evergreen step honour the 152+ gate its name states — both the operator's act,
mid-implement, because the readings require a CI run of committed code and CI fires only on push.

## Changes (structured — detectors read this)

- **Files:** `.github/workflows/ci.yml` (+24/−2 semantic, `git diff -w b3b51e6..HEAD`; +64/−? raw) ·
  `scripts/webview2-cause-probe.ps1` (**+34/−3 semantic**, `git diff -w`; 110/76 raw — ~73 lines are pure
  re-indentation from wrapping three sections in `if` blocks) · new
  `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md`.
- **Symbols / APIs:** `scripts/webview2-cause-probe.ps1` gains `[CmdletBinding()]` and a
  `[ValidateSet('all','policy','modules','session')] $Section` parameter beside the existing `-OutFile`
  (both optional; the no-arg invocation is unchanged). New emitted string
  `[probe] complete: {n}/1 sections ({Section})` for a selected run; the all-sections marker
  `[probe] complete: 3/3 sections` is byte-unchanged and neither string is a substring of the other. The
  install step's post-step reading renames `WebView2 Evergreen Runtime after install:` →
  `… in effect:` (basis: `grep -rn "after install"` over `*.md *.yml *.ts *.rs` returned only the two emit
  sites, no external readers). **No shipped binary, crate, export, port, socket or env var is added or
  changed** — the probe script is CI-only and wired into neither harness shell.
- **Crates / modules:** none — zero `.rs` delta (basis: `git diff --name-only b3b51e6..HEAD` plus the
  worktree diff, `grep -cE '\.rs$|Cargo\.(toml|lock)$'` = 0).
- **Dependencies:** none added or bumped. `Cargo.lock` and `package-lock.json` untouched. **The CI-time
  third-party fetch (the WebView2 Evergreen bootstrapper, the third dependency class) becomes CONDITIONAL:**
  it now runs only when the measured pre-step runtime major is below 152, so the outbound surface is strictly
  smaller, never wider. Its admitting controls are unchanged and were verified by parsing the step —
  `continue-on-error` ABSENT, `Get-AuthenticodeSignature` at indent 2 before the bootstrapper executes on the
  only path that fetches, and the `≥ 152` floor assertion at indent 0 **outside** the conditional.
- **Schema / config:** none. No migration, no config key, no violation-schema or redaction-shape change.
- **Spec-master edits:** none applied by /implement (it authored no spec change). Two amendments are EXPECTED
  and dispositioned below; P2 owns applying them.
- **Counts / qualifiers moved:**
  - The WebView2 runtime **in effect on the hosted `windows-2025` image**: `153.0.4234.32` → `152.0.4191.66`
    (basis: run `34645345201` vs `34654076633`, `[diag] WebView2 Evergreen Runtime …` lines). Any doc baking
    a post-install runtime literal for that image is stale.
  - **Unchanged and verified:** the governed harness-spawn form count stays **SIX** (no new spawned program;
    `grep -c 'Start-Process' .github/workflows/ci.yml` = 3, the same three forms as before). The `a11y` job's
    `continue-on-error` diagnostic-step SET is unchanged at `{WebView2 driver + runtime versions · WebView2
    session isolation · WebView2 cause probes · Upload session diagnostics}` (basis: parsed from `ci.yml`) —
    this chunk added an invocation INSIDE an existing step and minted no new step, so the SET the predecessor
    de-literalised does not move.
- **Dev-tool versions:** none installed or upgraded.
- **Harness / gate surface:** the `a11y` CI job changes in two places, both inside existing steps.
  (1) `WebView2 session isolation (diagnostic)` gains a `-Section modules` invocation of the same repo script
  inside probe (a)'s live window, above the `Stop-Process` that ends the bare app — the module reading's
  subject is a live process and the later `WebView2 cause probes (diagnostic)` step runs after that window
  closes. (2) `Install WebView2 Evergreen runtime 152+ (gate)` installs only below the floor. The job's
  verdict remains the routine arm's printed verdict; the gate step still carries no `continue-on-error`; no
  agent-run command, stage selector or `CONDUCTOR_*` handle is added (the 5-command surface is untouched).
- **Cross-project / external claims:** facts about the GitHub-hosted `windows-2025` image, read from this
  repo's own CI job logs (runs `34645345201`, `34654076633`, branch `build/conductor-0.3.0`, event push):
  the image ships `msedgedriver 152.0.4191.66`, `Edge browser 152.0.4191.66` and a WebView2 runtime at
  `152.0.4191.66` before the install step, and the runner process is elevated (`IsElevatedAdmin: True`, run
  `34586959536`). Ground truth is the image, not this repo; basis is the committed job-log capture.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** **arm C.** Making the install step honour its own
  `152+` gate is correct and shipped and stays — it removed a real confound and is the reason the skew
  candidate could be retired — but it did **not** open the endpoint. The defect it was written to test
  (`session not created: DevToolsActivePort file doesn't exist`) is unchanged; what it resolved is the
  driver/runtime major skew. The remainder is owned by elevation (established below) and, for the remedy, by
  the `v3-02` route entry.
- **Spec claims disproved by measurement:**
  1. `security-plan.md:185` — "**Version disposition — always-latest Evergreen, deliberately and temporarily:**
     the chunk that added it is a PROBE whose subject is the runtime MAJOR, so fetching the newest is the
     measurement rather than a build choice." **EXHAUSTED, not contradicted.** The probe's subject is no
     longer the runtime major — run `34280136892` falsified that as the discriminator — but the driver/runtime
     SKEW, and against that subject fetching the newest is the confound that creates it. Measuring evidence:
     runs `34645345201` (install → 153 over a 152 image) and `34654076633` (install skipped, all three on 152).
  2. `architecture.md:59` §Established Decisions [CI/CD] — the clause stating that the module-version
     candidate "returned NO reading at all because that probe's step runs after the isolation step stops the
     app (0.645 s later, measured)", and that elevation is "an observed difference and NOT a demonstrated
     cause, since no probe varied it". **Both retired by measurement:** the reading was taken (arm A), and
     elevation was varied (arm B).
  3. **Driver/runtime major skew is retired as a cause** — by direct variation with a control, not argument.
     No spec states it as a claim (it was an open candidate), so this is a candidate closure rather than a
     doc correction; recorded for `D-platform-claim`'s benefit because it retires a driver/runtime verdict.
  4. **`v3-02`'s terminal is no longer a candidate permanent exclusion.** Elevation reproduces the runner's
     exact signature on the known-good host with a control on both sides, so a remedy exists to test.
- **Expected amendments (from plan):** both entries are **carried**, and both have **TWO owners each** — the
  plan named one apiece, and the seven-master sweep run at this report's authoring found a co-owner for each.
  A single-site apply would leave the disproved statement alive in the sibling master.
  - **Amendment 1 — the always-latest Evergreen posture.** Fact: the *Dependencies* bullet (conditional
    fetch) plus disproved-claim 1. Sweep: `grep -c 'always-latest'` across all seven masters →
    `architecture 1 · security-plan 1 · design-system 0 · layout-templates 0 · test-plan 0 · obs-plan 0 ·
    a11y-plan 0`. Sites: **`security-plan.md:185`** (§Dependency Security, third dependency class — the
    version-disposition rationale) and **`architecture.md:205`** (§Infrastructure Patterns — Build system:
    "the WebView2 RUNTIME is deliberately FLOATED — since 2026-09-08 the `a11y` job installs always-latest
    Evergreen in-job"). The plan named only security-plan.
  - **Amendment 2 — the unread module probe and the unvaried elevation.** Fact: the *Harness / gate surface*
    bullet plus disproved-claim 2. Sweep: `grep -c '0.645'` across all seven →
    `architecture 1 · test-plan 1 · all others 0`; and `grep -c 'demonstrated cause'` →
    `architecture 1 · test-plan 1 · a11y-plan 0`. Sites: **`architecture.md:59`** (§Established Decisions
    [CI/CD]) and **`test-plan.md:469`** (which restates BOTH the 0.645 s no-reading fact and the
    "difference and not a demonstrated cause" clause). The plan named only architecture.
- **Coverage of new surfaces:**
  - `scripts/webview2-cause-probe.ps1 -Section` (CI-only diagnostic parameter) → validation
    **`[ValidateSet]`✓ + `[CmdletBinding()]`✓** (a closed enum for the value; strict binding for the name —
    without it a plain `param()` block collects an unknown parameter NAME into `$args` and the selector
    degrades silently to a full run, measured on the untouched tree) · instrumentation **n/a** (a CI step is
    Not-instrumentable, obs-plan §1) · PII **redacted✓** (`ModuleName` + `FileVersion` only, never
    `FileName`; registry keys reported in the colon-free `reg.exe` form) · tests **probe-gated✓** (its own
    `[[gate]]` entry asserting the selected marker, red-before-green) · a11y **n/a** · tokens **n/a**
  - `Install WebView2 …` conditional install (CI-only) → validation **n/a** · instrumentation **n/a** · PII
    **n/a** · tests **probe-gated✓** (the SKIPPED line and the `majors: 2 1` pair, both red-before-green) ·
    a11y **n/a** · tokens **n/a**
  - `evidence/reading.md` (committed record) → PII **redacted✓**, proven by the committed
    `grep -cE` hygiene gate reading `0`, not asserted · tests **gate-asserted✓** · others **n/a**

## Deviations from intent

1. **/implement edited `plan.md`**, which its own MUST NOT forbids. The operator directed it explicitly
   mid-implement and named the precedent channel ("plan edits between runs … you have already used twice").
   Recorded as an operator override with attribution, not a defect. What it added: arm C, its three gate
   entries, the reordering that puts arm B last and conditional, and the two Expected-amendment lines.
2. **The commit and push were the OPERATOR's act**, twice (`c93a379`, `eecc7f4`), mid-implement. The readings
   require a CI run of committed code; CI fires only on push; /implement reserves committing for wrap. The
   skill correctly refused to commit and surfaced the tension — the same deviation the predecessor recorded.
3. **Arm C was not in the original plan.** It was minted mid-implement on the operator's ruling after arm A's
   reading exposed the version skew. Justification: arm A's own result created a cheaper discriminator than
   arm B, and running the cheap arm first meant arm B's UAC acceptance was only spent once arm C had failed
   to explain the defect.
4. **Arm B ran before its conditional was satisfied in the strict sense.** The plan made arm B conditional on
   arm C leaving the endpoint closed; arm C did leave it closed, so the condition held — recorded because the
   ordering was decided mid-flight rather than at P4.
5. **A first arm-B attempt produced no reading** and is recorded rather than omitted: it died during the
   release build with its error invisible (`[Console]::Error.WriteLine` writes the raw console handle, which a
   PowerShell `*>` redirect does not capture; the script's `exit` killed the wrapper before any sentinel). A
   capture-correct retry (child process, real handle redirection, sentinel) produced the reading.

## Decisions & corrections

- **Fix the output, never the anchor** — applied twice this chunk. The committed-reading hygiene gate
  (`[A-Za-z]:[\/]`) fired on a quoted URL scheme (scheme + colon + slash satisfies it), and then again on the
  prose written to explain that false positive, because the explanation spelled the offending tokens. Both
  fixed in the output; the anchor was never loosened, since loosening it would blind the gate to real leaks.
  This is the same class as the `reg.exe` registry-provider rendering found the day before — **n=2 in two
  days** for one anchor.
- **A gate's GREEN case can be vacuous where its RED baseline could not show it.** The install-SKIPPED counter
  baselined red (count 0) on a log where the conditional did not exist, then measured **2** on its first green
  run: GitHub echoes the step source, so the literal appears in the echoed `Write-Output` template as well as
  in the real output. Fixed with a `grep -av Write-Output` stage. The identical trap had been found and fixed
  on a sibling entry earlier in the same authoring pass and was not carried across.
- **Assert the count beside the distinctness.** The skew probe was first written as `sort -u | wc -l`
  expecting one distinct major; baselined against the pre-arm-C log it returned `1` and read GREEN on a log
  that carries the skew, because the reading it half-depends on is introduced by this chunk. Re-authored to
  print `majors: {n} {d}` — a form that cannot pass on a missing reading.
- **An operator-dictated reading is a claim to verify.** Every relayed number was re-derived from the
  captured artifacts before it entered the record; three relays, zero corrections to the operator's figures,
  and one reading the relays did not name (`Edge browser: 152.0.4191.66`) that materially sharpened arm C's
  rationale — the image ships a coherent 152 triple and the job creates its own skew.
- **A UTF-16 artifact reads as empty to grep.** `runs/a11y-e2e.log` is UTF-16 with a BOM (PowerShell
  redirect); the first search for its driver/session lines returned zero hits on a file containing all of
  them. A zero there would have read as "the leg never got that far".
- **Credit the mechanism that actually fired.** The arm-B leg's non-zero came from wdio's own failure, not
  from `CONDUCTOR_A11Y_STRICT` — the leg never reached the printed-verdict assertion. The handle was
  insurance against a vacuous green and proved unnecessary; the record says so rather than crediting it.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

- (v3-01) **MET** — `evidence/reading.md` records the module reading taken while the subject processes were
  alive on the hosted runner, proven by `grep -c "\[diag\] (2) msedgewebview2 : [1-9]"` = `1`, beside its
  dev-host control; and arm B's elevated result beside the committed non-elevated control, with the decision
  stated (REMEDIABLE IN PRINCIPLE) and bounded to the host it was measured on.
- (arch) **MET** — the probe body stays in the one repo script, CI-only, invoked by no harness shell, no 6th
  agent-run command, no inline or duplicated copy.
- (arch) **MET** — outbound/bind set unchanged; no new bind, no new spawned program; the CI-time fetch
  narrowed. `Start-Process` count in `ci.yml` = 3, unchanged.
- (arch) **MET** — no env handle newly read or set, no `CONDUCTOR_*` name minted. The `WEBVIEW2_*` pair's
  registered lifetime ("that one `continue-on-error` diagnostic step") is preserved verbatim: the added
  invocation sits INSIDE that same step (verified against `architecture.md:197`).
- (security) **MET** — spawn-form count still SIX; the added invocation is a `run:`-step call of a
  repo-vendored script with literal argv, the class the `2026-09-09` precedent records as not moving the
  enumeration.
- (security) **MET** — Authenticode check, no-`continue-on-error`, and the `≥ 152` floor all survive; verified
  by parsing the step's indentation, not asserted (floor at indent 0, outside the conditional).
- (arm C) **MET** — install SKIPPED once (`1` after filtering GitHub's source echo); driver major and
  runtime-in-effect major equal (`majors: 2 1`); the discriminator recorded whichever way it fell
  (`never within 90s`).
- (obs/security/a11y) **MET** — zero absolute host paths in the committed reading, proven by the `grep -cE`
  entry reading `0`.
- (obs) **MET** — no span, metric, self-obs field or telemetry path minted; the a11y violation record and its
  in-job conformance assertion are untouched.
- (tests/a11y) **MET** — the job's verdict stays the routine arm's printed verdict; the added invocation
  inherits the isolation step's `if: always()` + `continue-on-error: true`.
- (tests) **MET** — the two markers are mutually non-substring-satisfiable and the default invocation still
  emits `[probe] complete: 3/3 sections` as its last line.
- (tests) **UNMET — and not this chunk's.** `cargo nextest run --workspace --profile ci` is RED. See the gate
  table and the attribution below.
- (a11y/layouts, layouts/design) **MET** — graded on the printed verdict, no second automation stack, no new
  bracket label / lamp / `ReportState` / ANSI entry / palette token.

**Gates** (the `[[gate]]` entries by `run`, in order):

| entry | verdict |
|---|---|
| `cargo nextest run --workspace --profile ci` | **RED — exit 100.** 906 run, **905 passed, 1 failed**. Not this chunk's (below) |
| workflow parses to 3 jobs | green — exit 0, last line `a11y,frontend,rust` |
| full probe, default path | green — exit 0, last line `[probe] complete: 3/3 sections` |
| `-Section modules` probe | green — exit 0, contains `[probe] complete: 1/1 sections (modules)`; baseline was red |
| `gh auth status` | green — exit 0 |
| `gh run list … <id>` | green — exit 0; freshness pinned on `headSha == eecc7f4`, not on the exit |
| elevated probe capture (`leg = 'operator'`) | green — contains `[diag] (3) IsElevatedAdmin: True`; operator-run, re-verified here from the recorded artifact |
| elevated routine arm (`leg = 'operator'`) | operator-run 2026-09-12, `E2E_EXIT=1`, sentinel `ARMB2_DONE`; reproduces `session not created: DevToolsActivePort file doesn't exist`. Re-verified here from this session's recorded artifacts, never re-run |
| `gh run view <id> --log` (`leg = 'operator'`) | green — run `34654076633` captured; never re-run here |
| arm A liveness grep | green — exit 0, last line `1` |
| install SKIPPED grep | green — exit 0, last line `1` (after the source-echo filter) |
| skew probe | green — exit 0, last line `majors: 2 1` |
| discriminator | report-only (`expect = []`) — `never within 90s` |
| host-path hygiene grep | green — exit 1, last line `0` |

No `defer` entry this chunk: the PREREQ gate RAN (it was not re-deferred) and is red on a pre-existing defect.
No smoke or self-verify entry, each absence stated with its reason in the plan's prose. No `leg = 'live'`
entry. Smoke: **skipped — no boot-path / UI-surface change** (the touchpoints are a CI workflow file and a
diagnostic PowerShell script; the plan's fence carries only `e2e`/`probe`/`unit` roles, verified by parsing,
because grepping the role tokens returns prose stating the deliberate absence).

**The red, attributed:** `crates/conductor-report/tests/matrix_ledger_gate.rs:49` filters requirement ids by a
baked `starts_with("v2-")` while the same file's header records that the version-directory resolution was
deliberately generalised — "a baked `conductor-0.2.0` would silently stop covering the ledger at 0.3.0, which
is the failure mode this gate exists to prevent." `conductor-0.3.0/requirements.md` declares 11 `v3-` ids and
0 `v2-`, so the required-set is empty and the gate's own anti-vacuity assertion fires. It landed in `b3b51e6`
— the predecessor's wrap commit, which also deferred the very gate that would have caught it — and this chunk
has **zero `.rs`/`Cargo.*` delta**, so it cannot have caused it. Red in CI too, on both `c93a379` and
`eecc7f4`. Out of this chunk's modify-set; /implement soft-exited on it (Trigger 3) rather than fixing it,
after measuring that a correct generalisation touches seven `v2-` literals (`:219 :221 :228 :250 :256-259`)
plus the id-space assertion at `:208`. Owner minted at P5 as the next Epoch-1 entry, BLOCKING.

**Outcome basis:** /implement's P4 report, plus three operator relays between implement and this report
(arm A's CI reading; arm C's ruling and its CI reading; arm B's elevated reading) and the post-implement
artifacts each named above — CI runs `34645345201` and `34654076633`, and the dev-host captures under
gitignored `runs/webview2-probe/`. Every relayed figure was re-derived from those artifacts before it entered
this report.

**Process hygiene** (from /implement's census, re-measured here where readable):

| process | started by | final state |
|---|---|---|
| `msedgedriver` · `tauri-driver` · `conductor-tauri` · `node` | the operator's elevated arm-B leg | terminated — the leg's own after-census records 0 of each |
| `powershell.exe` (probe gates) | this chunk's gate runs | terminated — the live set predates the runs or is the census command itself |
| `msedgewebview2` ×6 | pre-existing operator session | left running: the long-lived set the harness rule documents — not this chunk's to stop |
