# Report — 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration

**Chunk:** The a11y routine arm's terminal on the measured configuration — `:384`'s counting basis corrected so SC 2.1.1 measures the reachability property rather than one environment's focus-cycle wrap, and the arm's CI verdict read green
**Date:** 2026-09-17
**Commits:** `fc4a9c2` chore(2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration): operator pre-CI commit, for the run this chunk's verdict reads

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` · `.github/workflows/ci.yml` ·
  `conductor-0.3.0/verification-matrix.json` (ledger) ·
  `conductor-0.3.0/chunks/{marker}/evidence-ci-run-35208593666.md` (new evidence record).
  Basis: `git diff --stat` at `fc4a9c2` — 2 source files, 187 insertions / 180 deletions.

- **Symbols / APIs:**
  - ADDED (test-scope, module-private): `FOCUSABLE_SELECTOR` (const) · `FocusVisit` (interface) ·
    `focusSnapshot()` · `tabCycle()`. REMOVED: `activeName()` · `tabCycleNames()` — absorbed, not orphaned
    (`noUnusedLocals` is on in `test/tsconfig.json`, so an orphan would fail the typecheck).
  - **Remaining-caller fact:** `tabCycleNames` had exactly TWO callers, both inside its own file
    (`accessibility.e2e.ts` editor `:374` and `:394`, from the ts-plane code-graph impact query at phase P3);
    both were re-pointed at `tabCycle()`. No caller outside the file — this is a measured claim, not a default.
    The sibling `activeName()` definitions in `operator-hold.e2e.ts` and `screen-reader.e2e.ts` are independent
    copies and are UNTOUCHED.
  - **Ports / sockets / egress — the registry-bearing change.** The project's non-loopback egress count is
    **one before and one after**. REMOVED: the Evergreen bootstrapper fwlink at the former `ci.yml:313`
    (`go.microsoft.com/fwlink`, LinkId 2124703). ADDED: `msedgedriver.microsoft.com`, a version-pinned driver
    fetch. Basis: `grep -noE '(https?)://…' .github/workflows/ci.yml` — before: 2 hits (`:313` fwlink, `:647`
    `127.0.0.1:9515`); after: 2 hits (the driver host, `:647` unchanged). `:647` is loopback and is not an
    egress. Gate-asserted both directions by identity, not by absence.
  - **Env vars:** no new handle. `EDGEWEBDRIVER` changes PRODUCER only — previously read as the runner image
    set it; now WRITTEN by the pin gate through `GITHUB_ENV` to name the pinned directory, then read by the
    asserting step's existing precondition unchanged. `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_A11Y_STRICT`
    unchanged in both name and semantics.

- **Crates / modules:** none added, removed or changed. The edited `.ts` file lives under
  `crates/conductor-tauri/ui/`, the npm SPA subtree, which is not a Cargo workspace member.

- **Dependencies:** no crate delta, no npm delta, `Cargo.lock` and `package-lock.json` byte-unchanged.
  **Third-dependency-class membership is SWAPPED, not grown:** the class (a CI-time-fetched binary present in
  no lockfile, structurally invisible to `cargo audit` / `cargo deny` / `npm audit`) loses the Evergreen
  bootstrapper and gains the pinned msedgedriver. Both are admitted by the same sole control — a
  pre-execution `Get-AuthenticodeSignature` requiring `Status -eq 'Valid'` AND an `O=Microsoft Corporation`
  signer, neither step carrying `continue-on-error`. Basis: `grep -c 'Get-AuthenticodeSignature'
  .github/workflows/ci.yml` → 1 before, 1 after, while `grep -c 'WEBVIEW2_BOOTSTRAPPER_SIGNATURE'` → 3 before,
  0 after; the pair together shows the gate MOVED rather than merely survived.

- **Schema / config:** none. The a11y violation record's key set, the run-report envelope and every
  scrub/redaction shape are untouched.

- **Spec-master edits:** none by this chunk's own steps. Specs are read-only to /implement; every master edit
  rides P2's amendment flow below.

- **Counts / qualifiers moved:**
  - The routine arm's spec tally. Dev host: **10 passing / 2 failing / 2 skipped → 12 / 0 / 2**
    (baseline measured 2026-09-17T08:52Z on the untouched tree; post-fix run same session). CI: the last
    probe-configuration reading was 11 / 1 / 2 (run 35192876641); the shipping configuration now reads
    **12 passing / 0 failing / 2 skipped** (run 35208593666).
  - The expected-skip SET is UNCHANGED at two (the operator-pause dialog and the operator-checklist rows) and
    remains a SET, not a literal tally, in `Assert-A11yVerdict`'s `$A11yExpectedSkips`.
  - Spec-file block count unchanged: `grep -cE "^  it\(" accessibility.e2e.ts` → 14 before and after.
  - The a11y job's step count: 17 → 16 (the Evergreen install replaced by the pin gate; the redundant version
    diagnostic removed). Basis: YAML parse of `jobs.a11y.steps`.

- **Dev-tool versions:** **msedgedriver, the WebDriver the a11y leg drives, read on the CI image
  (`windows-2022`) — CHANGED from FLOATING to PINNED.** Unpinned the image supplied `151.0.4129.107`
  (run 35186499514) and `152.0.4191.77` (run 35185153012) over a native WebView2 runtime of `131.0.2903.86` —
  incoherent, and incoherent by a different amount each run. It is now derived from that runtime and read
  `131.0.2903.86` at run 35208593666, coherent. No lockfile resolves this tool; it is not a crate or npm
  package, and nothing sharing the stem `msedgedriver` appears in either lockfile. The dev host's own reading
  is unchanged and is a different machine's fact: runtime `153.0.4234.32` under driver `152.0.4191.53`.

- **Harness / gate surface:** `.github/workflows/ci.yml`, the `a11y` job only.
  - `runs-on: windows-2025 → windows-2022`.
  - The `Install WebView2 Evergreen runtime 152+ (gate)` step is REMOVED entirely — with it the `≥ 152` floor
    assertion and the fwlink fetch. REPLACED by `Pin msedgedriver to the image's WebView2 runtime (gate)`:
    reads the runtime from the two `EdgeUpdate` client keys (reported in the colon-free `reg.exe` form),
    fetches the driver for THAT version, Authenticode-gates it, asserts driver major == runtime major, and
    publishes `EDGEWEBDRIVER`. No `continue-on-error`, no `if:`.
  - The `WebView2 driver + runtime versions (diagnostic)` step is REMOVED — the pin gate prints both readings
    and asserts their relation, so a second reading would be a copy free to drift from the gate's own.
  - The asserting step `A11y routine arm` no longer launches through `scripts/a11y-limited-token-launch.ps1`.
    It invokes the leg's entry point `scripts/a11y-token-witness.ps1` directly, so the leg runs at the hosted
    step's native HIGH integrity. Its preconditions, its `$LASTEXITCODE` handling, the leg-console print and
    the captured-wdio print are unchanged.
  - `scripts/a11y-limited-token-launch.ps1` and `scripts/a11y-token-witness.ps1` are UNCHANGED and remain in
    the tree; the launcher keeps four call sites among the driver-alone diagnostics
    (`grep -c 'a11y-limited-token-launch' .github/workflows/ci.yml` → 5 before, 4 after: the asserting step's
    call is the one that left).
  - `scripts/agent-run.{sh,ps1}` UNCHANGED — the printed-verdict contract grades the driven-session banner, the
    `Spec Files:` failed count and skips `> $A11yExpectedSkips`, and NONE of the three is a pass tally, so the
    10/2/2 → 12/0/2 move touches no predicate.

- **Cross-project / external claims:** the WebView2 runtime and msedgedriver versions on GitHub's
  `windows-2022` and `windows-2025` images, and the availability of an archived driver build at
  `msedgedriver.microsoft.com`. Ground truth is Microsoft's and GitHub's, not this repo's. Basis read: CI job
  logs of runs 35186499514, 35185153012, 35192876641 and 35208593666, via `gh run view --log` with the
  step-source echo filtered. The pinned URL resolves an ARCHIVED build; it was available for `131.0.2903.86`
  at run 35208593666. If that major is pruned upstream the coherence gate fails loudly naming both versions —
  a loud failure, not a silent one.

- **Reverted / negative API facts:** a one-element perturbation of the SC 2.1.1 assertion was written and run
  DELIBERATELY as a known-positive control, confirmed red, then reverted; `grep -c '\.slice(1)'` → 0. It never
  shipped and was never intended to.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **The `≥ 152` Evergreen floor as a posture.** Not necessary (a coherent 131/131 pair runs the arm), not
     sufficient (a coherent 152/152 pair fails, run 34654076633), and destructive on the working
     configuration (run 35185153012 installed 153 over a native 131). Stated in `security-plan.md`
     §Dependency Security (third dependency class, version disposition) and mirrored in `architecture.md`
     §Infrastructure Patterns — Build system. **Now also false in code: the floor no longer exists.**
  2. **The Evergreen float's stated EXIT CONDITION names a mechanism that does not exist.**
     `security-plan.md` §Dependency Security records the float as retired when "the versioned Standalone
     Installer replaces the bootstrapper"; the Standalone Installer installs Evergreen and takes no version
     argument, and the only versioned route is Fixed Version (>250 MB, latest and second-latest majors only,
     so 131 is unobtainable). The float is retired here by a route that does exist — pinning the DRIVER to the
     runtime.
  3. **`windows-2025` platform verdicts.** Any sentence stating that the hosted image opens no
     remote-debugging endpoint, or that the endpoint question is image-independent, is falsified: hosted
     `windows-2022` at a coherent 131/131 pair at High integrity created a session and ran the arm green
     (run 35208593666). Retire by naming the measured SET, never by substituting a fresh single-platform
     literal.
  4. **The SEVENTH governed harness-spawn form ships nowhere in CI's asserting step.**
     `security-plan.md:363`'s seventh paragraph and its distillation at `.claude/rules/security.md:27`
     describe the `runas`/limited-token launcher as the asserting step's launch path. It is not: the asserting
     step invokes the leg directly, and the launcher survives only on the dev-only driver-alone diagnostics.
     **This is an ESCALATION, not routine drift** — security-plan rule (b) says this class escalates.
  5. **`:397` (SC 2.4.3) was never "wrap-insensitive".** The chunk's own scope asserted it; the dev-host
     baseline falsified it, failing as `Start then DIV`. Both Operable reds shared one root cause — a walk
     waiting on a `BODY` sentinel this webview's cycle does not contain — and `:397`'s pass on runtime 131 was
     luck about where the walk stopped.

- **Expected amendments (from plan):**
  - `architecture.md` §Established Decisions [CI/CD] — runner label → `windows-2022`. CARRIED; fact in
    Harness/gate surface. `grep -c 'windows-2025' .andromeda/architecture.md` → **3**.
  - `architecture.md` §Infrastructure Patterns (Build system) — the `≥152` floor retired, subject is
    coherence. CARRIED; fact in Spec claims disproved 1. `grep -c '152' .andromeda/architecture.md` → **6**.
  - `architecture.md` §Occupied Resources (Ports) — fwlink egress removed, pinned driver host registered,
    count stays one. CARRIED; fact in Symbols/APIs. `grep -c 'Evergreen' .andromeda/architecture.md` → **5**.
  - `architecture.md` §Cross-cutting Patterns (Trust boundary) — the third outbound surface re-described, not
    retired. CARRIED; same fact.
  - `security-plan.md` §Dependency Security — third dependency class keeps a member under the same
    Authenticode control; the float's exit condition rewritten to the mechanism used. CARRIED; facts in
    Dependencies + Spec claims disproved 1 and 2. `grep -c 'Evergreen' .andromeda/security-plan.md` → **4**;
    `grep -c '152'` → **4**.
  - `security-plan.md` §Security Anti-Patterns → Code Patterns rule (b) — the seventh form's disposition.
    CARRIED as an **ESCALATION** (Spec claims disproved 4), not as a routine edit.
    `grep -c 'limited-token' .andromeda/security-plan.md` → **1**; `grep -c 'runas'` → **1**.
  - `a11y-plan.md` §9 (Platform) and §11 (Strategy carve-out) — the arm's platform row and the routine pair's
    CI-proof-OWED clause, now discharged. CARRIED; facts in Harness/gate surface + Outcome.
    `grep -c 'windows-2025' .andromeda/a11y-plan.md` → **7**; `grep -c '152'` → **5**.
  - `test-plan.md` §9 (E2E-webview stage row) — the job's step set and the asserting step's launch path.
    CARRIED; fact in Harness/gate surface. `grep -c 'windows-2025' .andromeda/test-plan.md` → **6**;
    `grep -c 'limited-token'` → **2**.
  - `matrix#v3-02` notes — the terminal's outcome with its run id, and the measured root-cause correction.
    **ledger-note — owner P7.3.**
  - Derived tier the cascade owns (not separately owed): `.claude/rules/a11y.md` (windows-2025 ×1, 152 ×1) ·
    `.claude/rules/security.md` (Evergreen ×2, 152 ×1, runas ×1, limited-token ×1) ·
    `.claude/rules/testing.md` (windows-2025 ×1) · `.claude/docs/a11y-summary.md` (×3, ×2) ·
    `.claude/docs/security-summary.md` (×2, ×2) · `.claude/docs/tests-summary.md` (×1, ×1) ·
    `.claude/docs/stack.md` (×1) · `CLAUDE.md` (Evergreen ×1, 152 ×1).
  - **NOT targets, recorded here so the sweep does not reach for them:** `master-route.md` records are
    immutable by contract; the seven `*-amendments.md` sidecars are append-only history that correctly
    records what was true when written; `residuals.md` moves through P5, not P2; `playbook.md` grows only by
    propose→approve. And `.claude/rules/host-win32.md`'s single `runas` hit (`:134`) is KEPT: it states what
    the HOST IS — "Role and integrity are SEPARATE token attributes — `runas /trustlevel` strips the group and
    leaves the label" — measured, and true whether or not this project uses the mechanism.

- **Coverage of new surfaces:**
  - `msedgedriver.microsoft.com` (non-loopback egress, CI-job-scoped) → validation n/a (no operator input
    reaches it; the version is derived from the image's own registry reading) · instrumentation
    `span/metric/log n/a` (a CI step, not a Conductor runtime path) · PII `redacted✓` (the step prints handle
    NAME and state only — `MSEDGEDRIVER_SIGNATURE`, `MSEDGEDRIVER_COHERENCE` — never the resolved path) ·
    tests `e2e` (the a11y job itself, gate-asserted by two identity probes) · a11y n/a · tokens n/a.
  - No new interactive UI element, no new user-facing surface, no new Conductor runtime operation: the source
    delta is one wdio spec file and one CI job.

## Deviations from intent

1. **Kept the token-witness readback that plan step 7 directed removing.** Its premise was false: step 7 said
   the readback's "subject only exists inside a launched process", but `scripts/a11y-token-witness.ps1`'s own
   synopsis declares it "the a11y routine arm's entry point … witnesses the token it actually got, then runs
   the leg". Removing the launcher does not remove the witness. Following the clause literally would have
   deleted the evidence that the step ran undropped; and reading "the leg entry point" as `agent-run.ps1`
   would additionally have dropped the witness's `Start-Process -PassThru` without `-Wait` (a surviving
   grandchild holding an inherited stdout handle otherwise pins the parent), its repeated-pass orphan reaping,
   and its refusal of a zero exit whose wdio log never appeared. The step's INTENT — run at native High, no
   de-elevation — is implemented exactly.
2. **Voided the plan's `cargo nextest` deferral and ran it.** The plan deferred it as zero-Rust-delta, and
   `gate.py delta --defer-check rust` cleared it — but bucketed `conductor-0.3.0/verification-matrix.json` as
   *unmapped* while `crates/conductor-report/tests/matrix_ledger_gate.rs:42` genuinely opens it, and phase P5
   modified that file. That is precisely the measured failure the source-delta rule exists to prevent. Ran:
   **986 tests, 986 passed, 0 skipped**.
3. **Kept the `cargo clippy` deferral.** Zero `.rs` delta and no mechanism by which a JSON data file moves a
   lint verdict; a cold clippy over a ~78 GB `target/` buys no coverage on a TypeScript/YAML change.
4. **Ran an unplanned known-positive control.** The plan's own Implementation notes require hand-checking the
   green direction (`verification-harness.md:66a` — a red baseline validates only the failing direction). A
   deliberate one-element perturbation turned the assertion red; reverted and verified.
5. **Scope widened to `:397` and the shared walk helper**, beyond the entry's `:384` framing. Measured, not
   chosen: both reds share one root cause, and shipping a fix for one would have left a latent flake in the
   configuration this chunk claims green — barred by test-plan §10's zero-flakiness budget and a11y-plan §11's
   no-platform-dependent-assertion rule. Amended into `scope.md` at phase P4 and flagged at the P5 review.

## Decisions & corrections

- **Operator ruling (phase P4):** move the `a11y` job to the measured configuration — `windows-2022`, the
  `≥152` floor retired in favour of driver/runtime coherence, the launcher out of the asserting step.
- **Operator RE-RULING (phase P5), on a measurement that post-dated the first:** escalation 4 — the second
  non-loopback egress — RATIFIED, on a measured NET NARROWING. The first "no driver pin" ruling was taken
  before the image's incoherence was in front of the operator. The basis: the egress count goes one to one
  (the fwlink leaves), and the arriving crossing is strictly NARROWER because its version derives from the
  image rather than floating always-latest. Owed at wrap and not optional: registration in arch §Occupied
  Resources and §Cross-cutting Patterns, the Authenticode gate kept verbatim, and the float's exit condition
  rewritten.
- **Operator correction (phase P5), a false absence in my own plan.** I wrote that the image's driver
  coherence "was never exercised". It was, twice, and both readings are negative — `ci.yml:246` named the run
  ids two lines from prose I had already quoted, and I did not open them. The forecast was retracted in place
  with the measurements tabled.
- **Operator addition (phase P5):** a SECOND vacuous-by-construction assertion at `:388`, two lines from the
  one under repair, with the identical shape. I had read, quoted and flagged that shape at `:384` in the same
  pass and still called the neighbour correct.
- **Sweep hazard found this chunk — `session not created` is an OCCURRENCE/LINE trap.** In a CI log the string
  sits on 4 lines but occurs 8 times: each msedgedriver JSON body repeats it in its `error` and its `message`
  field. `grep -c` counts lines and under-reads by half; `grep -oE … | wc -l` is the occurrence count.
- **Sweep hazard found this chunk — a basename defer-check matches prose.** `gate.py delta --defer-check`
  reported `.andromeda/runs/…/security.md read by crates/conductor-verify/tests/jsonrpc_line_bound.rs`; the
  hit is a doc comment citing `.claude/rules/security.md`, a different file entirely. The same call missed a
  real reader. Both directions were settled by reading the hits, not by refining the pattern.
- **Claims carry their configuration.** The true sentence is not "the a11y arm is green in CI". It is: green
  on `windows-2022` with a coherent `131.0.2903.86` driver/runtime pair at High integrity, the driver pinned
  from the image's own runtime, measured at run 35208593666 (headSha `fc4a9c2`). Every one of those terms can
  move at the next image bump, and the record should make that visible rather than hide it behind a verdict.
  The over-general claim in the other direction is what cost six days.

## Outcome

Acceptance criteria, each re-asserted against the DIFF rather than the plan's text:

1. **SC 2.1.1 compares reachability as set equality over element identity, no literal count, no name-keyed
   set** — MET. `accessibility.e2e.ts` `:384` region: received side from the walk, expected side from the DOM
   roster; the `reached N/M [names]` pair can only match when the reached index set equals `{0…n-1}`.
2. **Both Operable specs are environment-independent** — MET, and measured on two configurations that differ
   in runtime major, driver major AND coherence: dev host 153-runtime/152-driver (incoherent) and CI
   131/131 (coherent), green on both.
3. **The `--e2e` leg prints its green verdict** — MET. 12 passing / 0 failing / 2 skipped against a measured
   10/2/2 baseline.
4. **No skip, retry, `continue-on-error` or relaxation obtained any green; expected-skip SET unchanged at
   two** — MET. YAML parse confirms the pin gate, the asserting step and the conformance gate each carry
   neither `continue-on-error` nor `if:`.
5. **No assertion in the Operable block compares a value against itself** — MET, verified by READING both
   assertions (a vacuous assertion passes, so green proves nothing here). `:384`'s two halves derive from
   disjoint sources; `:388`'s decorative `order:` half is gone and its `missing:` half compares against a
   literal.
6. **Non-loopback egress count one before, one after, asserted by two POSITIVE identity probes** — MET.
7. **Authenticode gate carried verbatim onto the pinned driver; no `continue-on-error` on fetch or verify** —
   MET, and confirmed in CI: `MSEDGEDRIVER_SIGNATURE: valid, Microsoft Corporation`.
8. **No eighth governed spawn crossing; the seventh keeps its `-File`-only guarded shape** — MET in code; its
   REGISTRATION is now stale and is escalated (Spec claims disproved 4).
9. **The `a11y` asserting step carries no `continue-on-error`/`if:`; green by assertion not by skip** — MET.
10. **Change lands in the npm subtree and the workflow only; no workspace member, no lockfile delta** — MET.
11. **Violation record key set unchanged; no absolute host path in artifacts or new text** — MET; the a11y
    violation-JSON conformance gate passed in CI (`8 tests run: 8 passed`), and the evidence record carries 0
    `letter:slash` runs (enumerated and read, not assumed).
12. **No token, hex, ms literal or palette row added; idle-console control composition untouched** — MET; no
    file under `ui/src/` is in the diff.
13. **CI-witnessed, spanning two runs by construction** — MET. Run **35208593666**, headSha
    `fc4a9c2da9149a9cce5aece64dca6416bf2ae32e`, whole run `success`; a11y job `success`.

**Gates** (by `run` text, in order):

- `(cd crates/conductor-tauri/ui && npm run typecheck:e2e)` — **green** · exit 0.
- `test -n "$CONDUCTOR_MSEDGEDRIVER" && test -f "$CONDUCTOR_MSEDGEDRIVER" && echo DRIVER_RESOLVED` — **green** ·
  exit 0 · `last line DRIVER_RESOLVED` held.
- `bash scripts/agent-run.sh run --e2e` — `leg = 'live'`, tool never fires it; **driven by hand at P2**,
  exit 0, `contains [a11y] verdict asserted` held, `lacks [a11y] leg skipped` held. 12 passing / 0 failing /
  2 skipped.
- `grep -c "go\.microsoft\.com" .github/workflows/ci.yml` — **green** · exit 1 · `last line 0` held
  (baseline red: exit 0 / 1).
- `grep -c "runs-on: windows-2022" .github/workflows/ci.yml` — **green** · exit 0 · `last line 1` held
  (baseline red: exit 1 / 0).
- `grep -c "msedgedriver\.microsoft\.com" .github/workflows/ci.yml` — **green** · exit 0 · `last line 1` held
  (baseline red: exit 1 / 0).
- `grep -c "WEBVIEW2_BOOTSTRAPPER_SIGNATURE" .github/workflows/ci.yml` — **green** · exit 1 · `last line 0`
  held (baseline red: exit 0 / 3).
- `grep -c "Get-AuthenticodeSignature" .github/workflows/ci.yml` — **green** · exit 0 · `last line 1` held
  (baseline green, with a known-positive control run at P5 proving it can fail).
- `cargo nextest run --workspace --profile ci` — **defer voided — conductor-0.3.0/verification-matrix.json**
  (read by `crates/conductor-report/tests/matrix_ledger_gate.rs:42`); RAN: 986 tests, 986 passed, 0 skipped,
  exit read from the bare command.
- `cargo clippy --workspace --all-targets -- -D warnings` — **defer** (zero `.rs` delta; re-runs at the next
  source-touching chunk).
- `cargo audit` — **green** · exit 0 · 1247 advisories · 562 packages · 7 allowed warnings.
- `cargo deny check advisories bans licenses sources` — **green** · exit 0.
- `git push origin HEAD && echo "PUSHED_SHA=$(git rev-parse HEAD)"` — `leg = 'operator'`, reserved by its own
  note; the OPERATOR performed it: `PUSHED_SHA=fc4a9c2da9149a9cce5aece64dca6416bf2ae32e`, `38d21d1..fc4a9c2`.
- `gh run list --branch build/conductor-0.3.0 --workflow ci.yml --limit 1 --json databaseId,headSha,conclusion
  --jq …` — `leg = 'operator'`, run once with `SHA` exported: exit 0, `contains databaseId` held,
  `{"conclusion":"success","databaseId":35208593666,"headSha":"fc4a9c2…"}`.

**Smoke:** the plan lists a `role = 'self-verify'` entry and it is `leg = 'live'`, so it was driven by hand at
P2 and P3 recorded it rather than re-driving (`run` then `status` is the boot+run pairing the plan template
bans).

**Outcome basis:** /implement's P4 report as given, held in this session's conversation — the friction-log
fallback was not read and is not this report's basis. Plus the operator's post-implement directive supplying
the push sha and the CI run id, and post-implement artifacts named above: CI run 35208593666's job log (read
via `gh run view --log`, step-source echo filtered per `verification-harness.md:66a`), the local leg captures,
and `chunks/{marker}/evidence-ci-run-35208593666.md`.

**Process hygiene:** re-measured here against the host process list, not recalled. `msedgedriver` 0 ·
`conductor-tauri` 0 · `node` 0. `msedgewebview2` **6**, every one at `StartTime 2026-09-16 18:33:31` — ~17
hours older than this session, the pre-existing CARRY 3 defect; this chunk's three leg runs (baseline,
post-fix, control) left ZERO survivors of their own, so wdio's `onComplete` teardown held on every run.
