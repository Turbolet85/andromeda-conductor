# Report — 2026-09-16-a11y-ci-gate-at-an-honest-terminal

**Chunk:** A11y CI gate at an honest terminal — the routine arm's CI verdict made honest
**Date:** 2026-09-16
**Commits:** none yet — this wrap's commit is the chunk's first (HEAD `38d21d1` throughout, pushed)

## Changes (structured — detectors read this)

- **Files:** `.github/workflows/ci.yml` (+20/−4 tracked) · `scripts/a11y-limited-token-launch.ps1` (new, then
  rewritten mechanism) · `scripts/a11y-token-witness.ps1` (new) · six `evidence/` records under
  `conductor-0.3.0/chunks/2026-09-16-a11y-ci-gate-at-an-honest-terminal/`.
- **Symbols / APIs:** no Rust, TS or crate surface touched. Two CI-only PowerShell scripts, each invoked
  solely from `ci.yml`'s `a11y` job and wired into neither harness shell:
  `a11y-limited-token-launch.ps1` (parameters `-Program`, `-ArgumentList`, `-WorkingDirectory`,
  `-ForwardEnv`, `-TimeoutSeconds`) and `a11y-token-witness.ps1` (no parameters; runs as the launched
  leg's entry point). Remaining-caller fact: neither has any caller outside that one job step.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none added, none bumped. `Cargo.lock` and `package-lock.json` untouched.
- **Schema / config:** none. No violation-schema, redaction-shape or config-key change.
- **Spec-master edits:** none by this chunk — all four expected amendments disposition below without a
  body edit (the verdict they would move did not change).
- **Counts / qualifiers moved:** none — verified. No documented derived value changed.
- **Dev-tool versions:** **dev host only, no repo pin exists or moved.**
  · `msedgedriver` (driver) — read at **152.0.4191.53** on the dev host; a **153.0.4234.32** driver was
    fetched (Authenticode `Valid`, signer `O=Microsoft Corporation`) and used for ONE leg through a
    **process-scoped** `CONDUCTOR_MSEDGEDRIVER` override; the operator's User-scope handle was left
    untouched and no repo file names a driver version (`ci.yml` takes it from the runner image via
    `EDGEWEBDRIVER`; `package.json` pins no driver package).
  · `WebView2 runtime` — moved **152 → 153.0.4234.32** on the dev host under Evergreen, unprompted; the
    version directory's creation time is 2026-09-15 21:38:03 and it is the only one present (Evergreen
    removes predecessors, so the prior version is unrecoverable). CI measured **152.0.4191.66** and is
    unaffected. Not a repository change; recorded here because it falsified a citation this chunk made.
- **Harness / gate surface:** the `a11y` job's **asserting step** now launches the leg through
  `a11y-limited-token-launch.ps1` instead of calling `agent-run.ps1` directly, and prints the leg's
  captured console and token witness afterwards; a new **`Remote-debugging-pipe route (diagnostic)`** step
  (`if: always()` + `continue-on-error: true`) was added after the existing diagnostics. The asserting
  step keeps no `continue-on-error`, no `if:`, and `CONDUCTOR_A11Y_STRICT: '1'`. No change to
  `agent-run.{sh,ps1}`, to the 5-command surface, to `wdio.conf.ts`, or to any verdict/status shape.
- **Cross-project / external claims:** none about another repo. External ground truth read this chunk is
  the GitHub Actions runner image (account identity, UAC posture, WebView2/driver versions) and Microsoft's
  driver distribution — both named with the runs that read them.
- **Reverted / negative API facts:** the **scheduled-task** launch mechanism was written, shipped to four
  CI runs, and then **replaced** by a `runas /trustlevel` mechanism in the same chunk. It is not dead code
  left behind — the file was rewritten — but the approach is recorded as closed by measurement, not taste.
- **Insufficient fixes (written, kept, not the remedy):**
  · The **limited-token launcher** (both mechanisms) is correct, shipped, and does **not** resolve the
    defect it was written for: neither can lower the mandatory integrity label, which is the measured
    discriminator. Owner of the remainder: the next route entry (§Route below).
  · The **orphan reap** in the witness improved survivors 7 → 17 reaped and is explicitly **not** done —
    6 in-window survivors remained at the last measurement. Owner: the same hygiene CARRY.
- **Spec claims disproved by measurement:**
  1. **This chunk's own plan** — `## Constraints & rejected approaches` states a `runas` form is excluded
     because it prompts and would break the headless invariant. Measured false: `/trustlevel` does **not**
     prompt (only `/user:` does); it ran non-interactively at exit 0. *Stated in:* `plan.md` rejected
     approaches. *Evidence:* local probe, parent IL `S-1-16-8192`, child `elevated=False`.
  2. **This chunk's own plan** — the security acceptance criterion demands "every supplied value is a
     separate array-form argv element", which is stricter than the master it cites. `security-plan.md:361`
     rule (b) forbids "a shell / `eval`-equivalent with operator-supplied input"; a scheduled task with
     `-File` and fixed literals is neither. **Operator-ruled 2026-09-16:** the master is not violated, the
     plan's restatement is over-strict. *Evidence:* `New-ScheduledTaskAction -Argument` is typed
     `System.String` and rejects an array (`ParameterBindingArgumentTransformationException`).
  3. **This chunk's own scope** — the `v3-02` obligation-(2) discharge ("the routine specs pass against the
     console as it stands") was a **citation of a 2026-09-10 verdict**, not a measurement. The "unchanged
     since" argument covered repository inputs only (`git log` over `ui/src`/`ui/test`; a `contentinfo`
     grep) and the arm depends on host tooling no repo grep can reach. Re-measured 2026-09-16: **10
     passing / 2 failing / 2 skipped** against the recorded **12 passing / 2 skipped**.
  4. **No spec master asserts a claim this chunk falsified.** The hosted-runner verdict in
     `test-plan.md` and `a11y-plan.md` is "wired, RED at WebView2 session creation" — still true. The
     detector at `drift-base.md:145` has no stating sentence to retire: this chunk did not change where the
     leg CAN run, only why it cannot.
- **Expected amendments (from plan):**
  · `test-plan.md` §1 Surfaces · §6 drivers-per-surface · §9 Matrix builds — **not carried**: these three
    move together only when the hosted-runner VERDICT changes; it did not (still red at session creation).
    Located by `grep -cE 'hosted|runner' .andromeda/test-plan.md` → 23 hits, none stating a verdict this
    chunk falsified. The refined CAUSE is chunk evidence, not a spec fact, until a remedy lands.
  · `a11y-plan.md` §1 CI integration · §9 Platform/Pipeline · §11 CI — **not carried**: these are three
    verbatim citations of arch §Stack's CI-rationale cell, which this chunk did not change.
    `grep -c 'build + test gating' .andromeda/a11y-plan.md` → **3** (the three citation sites, intact).
  · `security-plan.md` §Dependency Security — **not carried as an edit, escalated as evidence**: the
    float's pin-or-retire exit condition is "the a11y job actually GATES", and the job still does not
    gate, so the condition is unmet and the clause stands unchanged. What this chunk adds is *evidence
    about* the float — it charged a cost before the gate lit (§Decisions). Site: `security-plan.md:185`.
  · `architecture.md` §Established Decisions [CI/CD] · §Occupied Resources — **carried as a question for
    P2**: the a11y job gained a step (`Remote-debugging-pipe route`) and two CI-only scripts. §Occupied
    Resources already registers `scripts/webview2-cause-probe.ps1` by name and the `WEBVIEW2_*` handles
    (`architecture.md:199,201`), and §Established Decisions states the CI gate set as a SET. Whether two
    new CI-only scripts need registration rows is the fan-out's call, not pre-empted here.
- **Coverage of new surfaces:**
  - `scripts/a11y-limited-token-launch.ps1` (CI-only launcher) → validation `n/a` (no external input; the
    argv guard rejects whitespace/metacharacters before use, measured exit 90 on both controls) ·
    instrumentation `n/a` (not a shipped binary; prints `[precondition]`/`[diag]` lines) · PII `redacted✓`
    (handle NAMES and states only, no resolved path — evidence host-path gate green) · tests
    `unrunnable-here` (its subject is a hosted-runner token; the dev host cannot reproduce High integrity
    without an elevated shell) · a11y `n/a` · tokens `n/a`
  - `scripts/a11y-token-witness.ps1` (CI-only leg entry point + token witness) → validation `n/a` ·
    instrumentation `n/a` · PII `redacted✓` (registry keys reported in the colon-free `reg.exe` form; no
    resolved paths) · tests `unrunnable-here` (same reason) · a11y `n/a` · tokens `n/a`
  - `a11y` job step `Remote-debugging-pipe route (diagnostic)` → validation `n/a` · instrumentation `n/a` ·
    PII `redacted✓` · tests `n/a` (a `continue-on-error` diagnostic that can never decide the gate) ·
    a11y `n/a` · tokens `n/a`

## Deviations from intent

1. **Plan step 5 was a no-op at the site it named.** It asked to extend the session-isolation census to
   `msedgewebview2.exe`; that site already carries it at HEAD (`git show HEAD:.github/workflows/ci.yml |
   grep msedgewebview2` → present). The live omission is in `CENSUS_NAMES`
   (`parse-nvda-log.ts:130-137`), whose three call sites (`wdio.conf.ts:240,396,450`) are all
   screen-reader-gated — SR territory, outside this chunk's modify-set. **Justification:** surfaced as a
   CARRY rather than widened into scope; scope widening is authorised by a measured defect and the
   operator's word, never by a file list.
2. **The launch mechanism was replaced mid-chunk.** The plan named a scheduled task (the CARRY's arm);
   measurement retired it and `runas /trustlevel` replaced it. **Justification:** the plan's own step 2
   named the mechanism class, not the API, and the replacement was chosen on a measured constraint (RID
   500 has no split token) rather than preference.
3. **Four CI iterations were driven inside the fix-loop**, against a plan whose `leg = 'operator'` entries
   reserved the push for the operator. **Justification:** operator directive lifting the ci-probe blocker
   for this chunk; `/implement` still committed nothing (`HEAD` `38d21d1` throughout, index reset each
   time, probe branch deleted after each read).
4. **A host tool was changed.** A 153 driver was fetched and used for one leg. **Justification:** operator
   ruling ("restoring the coherence that existed when the green was recorded"), executed host-only,
   process-scoped, with the repo-pin check run first and the Authenticode control applied.

## Decisions & corrections

- **`IsInRole(Administrator)` returns False for a DENY-ONLY membership.** I reported the dev-host account
  as having no admin rights and declared two proposed tests impossible; the account IS a listed local
  administrator, merely not elevated. Sweep hazard: the same "elevated now" vs "has the right" conflation
  I had correctly separated one layer up, an hour earlier.
- **A count or site list read off a `tail`/`head` view is not a count — three instances in one session**,
  across two readers: a per-path grep quoted from a clipped combined search; a site enumeration narrower
  than the graph query that had already answered it; a `:397`-only failure reading that nearly became
  "alignment made it worse". All three dissolved when re-derived over the whole artifact.
- **An inherited stdout handle held by a grandchild pins the parent.** `$output = & cmd *>&1` keeps the
  pipeline open until every process holding the handle exits — and `Start-Process -Wait` has the same
  defect, because it drains redirection. Waiting on the process object does not. This was the ~15-minute
  CI tail after a leg was already dead.
- **A probe validated outside the context it runs in proves nothing.** The witness's UAC block passed in a
  bare shell and threw `PropertyNotFoundException` in CI under the `Set-StrictMode -Version Latest` the
  script itself sets, losing exactly the two readings that mattered.
- **An instrument can under-count its own subject.** `OrphansLeft` measures at the instant the last reap
  pass runs, while the process tree is still settling: self-reported 1, independently measured 6.
- **The Evergreen float charged a cost before the gate it is bounded by ever lit** — an unprompted
  overnight runtime bump broke the arm's two newest assertions with no repository input changing.
  `security-plan.md:185` bounds the float with "pin once the a11y job actually GATES"; `v3-02` carries the
  pin-or-retire disposition. That decision now has demonstrated evidence, not a theoretical exposure.

## Outcome

**The chunk's terminal is UNDECIDED, and that is the honest result.** Re-asserted against the diff:

- (a11y) *terminal closed on one of `v3-02`'s two arms* — **UNMET.** Neither arm reached. Routed: `v3-02`
  is **un-claimed** at P7.3 (operator ruling; the requirement is right and the world is wrong — the runner
  is High and no shipped mechanism lowers the label), returning to the pool for the successor entry.
  An exclusion is **not** ratified: after leg C it would record a falsehood.
- (a11y) *verdict honest by mechanism* — **MET, unchanged.** `CONDUCTOR_A11Y_STRICT: '1'` and
  `Assert-A11yVerdict`'s three assertions remain the basis of any exit-zero run; the diff does not touch
  them.
- (arch) *asserting step has no `continue-on-error`, no `if:`, strict handle set* — **MET**, asserted green
  by the probe entry with two known-positive controls.
- (security) *no shell string / `Invoke-Expression` / composed command line; seventh form named* —
  **MET as ruled.** The composition is forced by the API and is outside rule (b)'s eval class (`-File`,
  never `-Command`); the seventh governed form and the faithful criterion restatement are drafted in
  `evidence/surfaced-argv-composition.md` for the operator to land.
- (security) *strict arm not disabled, handle guards byte-unchanged* — **MET**, asserted green.
- (security/obs) *evidence records carry 0 absolute host paths* — **MET**, asserted green.
- (security) *WebView2 float dispositioned* — **UNMET, correctly:** its exit condition is the job gating,
  which did not happen. Owner: `v3-02`, which carries the disposition forward.
- (design/layouts) *no new lamp, ReportState, bracket label, ANSI entry or token* — **MET**; none minted.
- (tests) *if (B), a flagged zone with cause/source/owner* — **n/a**; (B) not taken.
- (obs) *a11y record and `journal_conformance` unaffected* — **MET**; the record path and the conformance
  step are untouched by the diff.

**Gates** (by `run` text, in order):
- `python … yaml.safe_load … sorted(d['jobs'])` — **green** (`last line a11y,frontend,rust`)
- `python … 'A11y routine arm' … continue-on-error/if/strict` — **green** (`last line True`)
- `git diff --quiet HEAD -- wdio.conf.ts agent-run.ps1 agent-run.sh` — **green** (`last line unchanged`)
- `test -d … evidence && ! grep -rqE …` — **green** (`last line clean`)
- `cargo nextest run --workspace --profile ci` — **defer voided — `conductor-0.3.0/verification-matrix.json`**
  (uncommitted and read by four Rust files incl. `conductor-report/tests/matrix_ledger_gate.rs`); run by
  hand, **986 tests run, 986 passed, 0 skipped**, exit read from the bare command
- `git push && git rev-parse HEAD` — `leg = 'operator'`; **treatment:** not re-run; re-verified by this
  session's recorded artifacts (probe SHAs and run ids in the implement run dir)
- `gh run list … select(.headSha=="<id>") … jobs` — `leg = 'operator'`; **treatment:** not re-run;
  re-verified by the recorded run ids `35079315258`, `35095825604`, `35102123985`, `35103823579`,
  `35111618735`
- **Smoke:** skipped — no boot-path / UI-surface change (modify-set is a CI workflow plus two CI-only
  `.ps1`; `crates/` delta 0; plan lists neither `smoke` nor `self-verify`).

**Outcome basis.** Implement's P4 report as given, PLUS post-implement artifacts and operator-run legs that
are **not** implement's own output and must not be read as such:
- **Legs A and B** were run by the **operator** from one elevated PowerShell; **leg C** by the **overseer**
  from a non-elevated one. All three are recorded in `evidence/integrity-level-is-the-discriminator.md`.
- Five CI runs, four of them fix-loop probes driven under the operator's ci-probe directive.
- Two operator directives between implement and this report: the rule-(b) ruling (criterion over-strict,
  register the seventh form) and the no-downgrade ruling (the 153 finding is dated advance warning).

**Process hygiene.** Measured on this host at wrap time: **19 `msedgewebview2` orphans alive**, attributed
by start time to the session's legs; `conductor-tauri` / `msedgedriver` / `tauri-driver` / `node` all **0**.
The reap shipped in the witness improved this (7 → 17 reaped in the verified run) and is **not** complete —
6 in-window survivors at the last measurement. CI-side survivors: `unmeasured — only the runner can see
them`; the ~15-minute post-leg tail is the same defect's CI surface.
