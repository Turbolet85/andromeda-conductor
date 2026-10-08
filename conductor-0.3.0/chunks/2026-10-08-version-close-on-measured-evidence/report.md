# Report — 2026-10-08-version-close-on-measured-evidence

**Chunk:** Version close on measured evidence — the 11 capabilities stated on their measured basis, the carried
corrections made, the next version's direction recorded
**Date:** 2026-10-08
**Commits:** `42daf3e chore(2026-10-08-version-close-on-measured-evidence): operator pre-CI commit, for the run this
chunk's verdict reads` (the one commit since the chunk base `fb48cee`; `git log --format='%h %s' fb48cee..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (`git diff --name-only fb48cee`, 20 paths outside run dirs)
  - `scripts/agent-run.ps1` — one comment line re-pointed (line 230)
  - `scripts/a11y-token-witness.ps1` — one comment line re-pointed (line 160)
  - `contracts/pulse-real-model-leg-posture.md` — one dated add-only block at the end of `## Regime` (added range
    51-58 in the listing below)
  - new, in this chunk's folder: `evidence/version-close.md`, `evidence/next-version-direction.md`,
    `evidence/operator-pass.md`; the phase's `scope.md`, `research.md`, `plan.md` and five `inputs/` entries
  - pipeline bookkeeping the pre-CI commit carried: `.andromeda/master-route.md`, `conductor-0.3.0/working-route.md`,
    `.andromeda/friction-log.ndjson`, `.claude/session-handoff.md`, run dirs
- **Symbols / APIs:** none. No function, IPC method, endpoint, export, port, socket or environment handle was added,
  changed or removed. No Rust file moved (`git diff --quiet fb48cee -- crates` exits 0, a gate entry).
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none. `Cargo.lock` holds 562 packages, as at the base (`grep -c '^\[\[package\]\]' Cargo.lock`).
- **Schema / config:** `contracts/pulse-real-model-leg-posture.md` gained one dated note in its own form
  (`[corrected 2026-10-08 (2026-10-08-version-close-on-measured-evidence, …`). It says four things: the sentence "No
  Rust code reads it" is false as written since the 2026-09-30 series; no shipped code reads the file, and a test
  helper in `conductor-run`'s real-model harvest holds each dated section by sha256 and parses nothing from it; that
  digest hold is not the runtime read the section's notice names, so the committed-manifest input-boundary duties do
  not attach; the notice stands as written for a real runtime read. No existing line of the contract was edited or
  removed (one hunk, `@@ -50,0 +51,8`, `git diff -U0 fb48cee`), and no digest-pinned section moved (the harvest target
  reads 139 of 139). No scenario, TOML manifest, violation schema or redaction shape changed.
- **Spec-master edits:** none before this wrap. /implement wrote no master.
- **Counts / qualifiers moved:** none — verified. The chunk added no test: the secret-scan target reads 7, the harvest
  target 139 and the workspace 1239, each as forecast at P4 (the gate logs of the implement run and of this wrap's
  light gate). The two architecture registries read 38114 B and 38082 B, unchanged by /implement
  (`scripts/arch-registry-check.py measure`).
- **Dev-tool versions:** `pwsh` (PowerShell, the shell that runs the PowerShell harness `scripts/agent-run.ps1`) on
  the Linux dev host: read CHANGED against what the masters state. The masters say the host has none; measured
  2026-10-08, `pwsh` resolves on the inherited `PATH` and prints version 7.6.6, and the host's package log dates the
  install 2026-10-06 21:50 local (`installed powershell-bin (7.6.6-1)`). This chunk did not install it. The CI images
  are not this line's subject. No other host tool was read changed.
- **Harness / gate surface:** no verb, flag, arm, step, exit path, guard or verdict shape changed. The two harness
  scripts changed in comment text only, one line each: the citation of the retired `host-win32.md` now points at
  `.claude/rules/verification-harness.md` (the 2026-09-16 entry) in `scripts/agent-run.ps1` and at
  `.claude/docs/session-learnings.md` (the 2026-09-11 host-leaf entry) in `scripts/a11y-token-witness.ps1`. Basis: two
  gate entries (0 non-comment diff lines; 2 added lines), and PowerShell's own parser on the dev host — each script
  parses with 0 errors, its non-comment token stream equals the base's (2375 and 1374 tokens), 1 comment token
  changed in each; a control copy with one executed line appended read not equal (2375 against 2378). Neither script
  was executed on the dev host. No CI workflow file moved (`.github` is inside the "nothing else moves" gate entry).
- **Cross-project / external claims:**
  - Inputs, from `inputs.py verify` (5 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 3 · uncited 0 · unparsed 0):
    - `I1 · ../additional/pc-overseer/relays/conductor-phase-versionclose-2026-10-08.md · copy no-repo · unchanged`
    - `I2 · message: the operator, the /andromeda-phase invocation arguments, 2026-10-08 · copy message · n/a — a
      message has no live source`
    - `I3 · message: the operator, answers to the two P4 forks, 2026-10-08 · copy message · n/a`
    - `I4 · ../additional/pc-overseer/relays/conductor-versionclose-ruling-2026-10-08.md · copy no-repo · unchanged`
    - `I5 · message: the operator, the word at the P5 review, 2026-10-08 · copy message · n/a`
  - The wrap relay, `conductor-wrap-versionclose-2026-10-08.md` in the pc overseer's `relays/` folder, was read live
    at this wrap's start and is not snapshotted: `inputs.py snap` takes no wrap step (its `--step` choices are
    `phase:P1`, `phase:P3`, `implement`), and the folder is not a git repository. Its sha256 at the read:
    `3aa872692d28c334c0bac66b996caedaa3a6caac697d663fefda6da7d01ade06`.
  - The founder's ruling on the contract's readers (dialog, 2026-10-08, relayed by the pc overseer; `inputs#I4` §1).
    His pick, verbatim: «Нет, исправить фразу (Recommended)». A test-tier digest read is not the runtime read the
    contract's notice names; the duties do not attach; the notice stands for a real runtime read.
  - The founder's direction for the next version (dialog, 2026-10-07, relayed by the pc overseer; `inputs#I1` §5):
    recorded whole in `evidence/next-version-direction.md`, 344 characters in the relay copy and in the document,
    equal after line wraps are folded.
  - CI: run `CI#37763841102` on the pushed pre-CI commit `42daf3e47e3052aa36a56984871600b5a005ad98`, conclusion
    success, checks 3/3, 708 s wall. All three jobs concluded success with no non-success step. The `a11y` job's own
    log reads `[a11y] verdict asserted - 0 failed | 2 skipped (expected 2) | driven session present` on
    `webview2 131.0.2903.86`, 19 passing and 2 skipped. The verdict was taken on `42daf3e`; this wrap's commit adds to
    that tree.
  - Supply chain, external state read at the gate: `cargo audit` loaded 1294 advisories over 562 packages and printed
    7 allowed warnings, exit 0; the local advisory-db copy's porcelain was empty before and after; `cargo deny` read
    `advisories ok, bans ok, licenses ok, sources ok`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **"The Linux dev host has no `pwsh`."** Stated at two sites as the reason the PowerShell harness's red path is
     unmeasured: the test-plan `run` contract's key file
     `.andromeda/registries/contracts/test-plan/5-command-implementation.md:11` ("its red path is read from the
     script, not measured (the Linux dev host has no `pwsh`)") and `.andromeda/obs-plan.md:490` ("its red path
     unmeasured — the Linux dev host has no `pwsh`"). Measured false since 2026-10-06 (the Dev-tool versions bullet).
     What stays true: the red path is unmeasured, and nobody drove it at this close (the operator's word: no unplanned
     leg at the close). Sweep basis: `git grep -n -i -c pwsh` over the seven masters, `.andromeda/registries`,
     `CLAUDE.md`, `.claude/rules` and `.claude/docs` — 6 files, 7 hits · 2 state the reason (the two above) · 5
     no-change: `security-plan.md:383` names the shell of a dated 2026-09-04 measurement, `test-plan.md:387` is a CI
     step's `shell: pwsh`, `.claude/rules/security.md:55` is a command form, `.claude/docs/session-learnings.md:635`
     and `:994` are dated history. The plan's own Test Commands prose and its CI acceptance carry the same premise; a
     plan is not amended.
  2. **"No Rust code reads the posture contract."** A test helper does, since the 2026-09-30 series:
     `pre_registered` in `crates/conductor-run/tests/real_model_grading/mod.rs` opens the contract and compares one
     section's sha256 with a pin (six calling tests, code-graph and a name grep agreeing; research.md §Graph impact).
     No shipped code reads it. Sweep basis: `git grep -n -i -c -E 'no rust (code )?read|reader-less'` over the seven
     masters, `.andromeda/registries`, the playbook, the leaves, `contracts/`, `crates/` and `scripts/` — 8 files, 15
     hits:
     - `.andromeda/architecture.md:184`, the posture row — "the **SECOND** `contracts/` member with **NO Rust
       reader**, taking the P-025 regime above unchanged": false of this member as written. To correct, the ordinal
       kept.
     - `.andromeda/architecture.md:183`, the P-025 row — "the FIRST `contracts/` member with **NO Rust reader**": true
       of that member (no Rust reader by name, research). No change; the ordinal stays.
     - `.andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:30` — "the
       two members no Rust code reads": false of the second member. To correct ("no shipped code reads" is true of
       both).
     - `CLAUDE.md:14` and `.claude/docs/conventions.md:9` — the two leaves, same words: re-derived from the key line.
     - `contracts/pulse-real-model-leg-posture.md` (3 hits) — the contract's own sentence, corrected by this chunk's
       dated block; no further change.
     - `.andromeda/security-plan.md:116` and `:328`, and `.claude/rules/security.md:18` — a different subject (the
       host-dev-tool handles `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA`, which no Rust code reads): no change.
     - `.andromeda/playbook.md` (4 hits) — the reader-less-member rule; curated by the wrap, its ordinal clause
       untouched.
  3. **Noticed at take-up, not changed by this chunk: the reproduced envelope schema types `verdict` with no null
     arm.** The code's type is `Option<Verdict>` (`crates/conductor-core/src/run_record.rs:55`), and the masters' own
     prose reads `verdict` null for declare-only rows since 2026-08-21 (`git grep -c -E 'verdict.{0,3}null'`:
     test-plan 8 lines, obs-plan 6). The schema sample reads `"verdict": "Pass | Fail | CalibrationRegion"` at:
     `.andromeda/obs-plan.md:72`, `:93`, `:347` and
     `.andromeda/registries/contracts/obs-plan/log-format-json-schema.md:12`; `.andromeda/a11y-plan.md:103` and
     `.andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:13`; and at the owner, test-plan
     §3, `.andromeda/registries/contracts/test-plan/status-endpoint-shape.md:6`, with the prose form
     `verdict ∈ {Pass, Fail, CalibrationRegion}` at `.andromeda/test-plan.md:65` and in the leaf
     `.claude/rules/verification-harness.md:27`. The operator's call (wrap relay §3): a detector that proposes the
     correction with these coordinates is applied at this wrap; if none does, one open residual line.
     [added 2026-10-08 after the fan-out: this list under-ran by two sites, found by the detectors' own sweeps and
     confirmed by a re-run over all seven masters and every key file — `.andromeda/test-plan.md:68`, a third spelling
     (`verdict: "Pass"|"Fail"|"CalibrationRegion"`), and `.andromeda/architecture.md:136`, the prose set form. The
     first list was taken over three masters only. The whole family is in this wrap's `fanout-results.md`.]
- **Expected amendments (from plan):**
  - architecture §Occupied Resources — On-disk artifacts, the freeing step then the posture row: **carried** — Spec
    claims disproved, item 2. Site search above: `architecture.md` 2 hits (`:183` no change, `:184` corrected). Free
    first, then correct; both ordinals kept; the kind and tier of the reader live in the contract's dated block and in
    security-plan, and the row points at the first. Research's arithmetic: the by-reference face-field sentence frees
    52 B, the corrected sentence costs 46 B, predicted 38114 → 38108 B of a 38115 B threshold. The instrument's
    reading after the apply is the record. If no true sentence fits under the ordinal rule and the threshold, the
    wrap stops and shows the operator the arithmetic and writes no part of the row.
  - architecture §Infrastructure Patterns → Directory structure, the `contracts/` line: **carried** — item 2. One hit
    in the key file (`:30`); the cascade re-derives `CLAUDE.md` §Key directories and `.claude/docs/conventions.md`.
  - `matrix#v3-09 notes — the version close stated the one known exception, by location and without quoting it`:
    **ledger-note — owner P7.3**.
  - security-plan §Input Validation, the committed SUT-facing manifests row or a dated note beside it: **carried** —
    the Cross-project bullet's ruling and item 2. Site search: `git grep -c -F pulse-real-model-leg-posture --
    .andromeda/security-plan.md` reads 0 hits, so the plan names the contract nowhere today; the row is the one that
    names `contracts/scenario-audit-ledger.toml` and its test-binary readers (`.andromeda/security-plan.md:113`). The
    entry records the reader — a `conductor-run` harvest helper holding each dated section by sha256, root from
    `CARGO_MANIFEST_DIR`, no handle, read faults by error kind, parsing nothing — says the row's duties do not attach,
    and quotes the founder's pick. It widens nothing: no new crossing, input class or write.
  - "No amendment is expected in test-plan, obs-plan, a11y-plan, design-system or layout-templates": **superseded for
    test-plan and obs-plan** by a fact measured after the plan was written — Spec claims disproved, item 1 (two
    sites). Design-system and layout-templates: not carried; this chunk changed no value either states. A11y-plan and
    the other obs-plan and test-plan sites of item 3 are the operator's conditional call, not a plan entry.
  - Not amendments, named in the plan beside the list: the wrap's route-resolve appends one `open` line to
    `.andromeda/residuals.md` carrying the founder's quote and pointing at `evidence/next-version-direction.md`; the
    playbook's reader-less rule may gain the dated fact that its second member has a test-tier digest hold and that
    the founder ruled it attaches no duty.
- **Coverage of new surfaces:** none. The chunk added no external surface, hot-path operation or UI element.

## Deviations from intent
- **Step order.** Step 6 (the contract's block) was applied before steps 4 and 5; they do not depend on it.
- **The "ref cut at its first dash" rule does not fit two table rows.** `v3-10`'s ledger ref holds no dash and is
  written whole. `v3-03`'s first dash sits near the end of a 1291-character ref, so the row is cut at the ref's first
  semicolon clause, with the configuration and the arms the ref names kept verbatim. `v3-02` keeps its configuration
  from past the dash, as step 5 orders.
- **The `pwsh` premise.** The plan says the two `.ps1` files cannot be executed on this host for want of `pwsh`. The
  close record was first written with that sentence, then corrected when a one-line lookup measured it false. A
  parse-only token comparison was run beside the gate tool (the Harness bullet). No script was executed. The first
  `pwsh` call, the version read, ran without `POWERSHELL_TELEMETRY_OPTOUT` and may have sent PowerShell's start-up
  telemetry; the three later calls had it set.
- The operator accepted all three on 2026-10-08 ("accepted, including your pwsh correction and the three
  deviations" — the operator, 2026-10-08).
- scope record: none — `gate.py scope` clean, 0 recorded (changed 3 · listed 3 · excluded 54, base `fb48cee4`).

## Decisions & corrections
- **The founder's ruling on the contract's readers** (above), and the operator's overrule that both historical
  ordinals stay as written (`inputs#I4` §3; `.andromeda/playbook.md:243`).
- **The operator, at the operator pass (2026-10-08):** do not drive `pwsh scripts/agent-run.ps1 run` — no unplanned
  leg at the close; the wrap amends the stale reason.
- **The operator's calls on the three open items** (wrap relay §3): the `verdict` null arm is applied only where a
  detector proposes it with coordinates the report holds, else one open residual line; the masters' "Windows dev
  host" as the webview legs' home gets one open residual line and no rewrite, because which legs the Linux host can
  run is unmeasured; the retired leaf's UTF-16 clause has a home (`verification-harness.md`, the 2026-09-16 entry)
  and takes no action. The residual lines are notes for a next version's intake, not scope moved out of 0.3.0.
- **The stops** (wrap relay §4): flip this chunk and mint no entry; no merge into `main`, no tag, no release, no
  publishing version bump, no branch deletion, no `conductor-0.4.0` directory, no route for a next version; no model
  text outside `evidence/`.
- **A host claim was copied from the plan into committed evidence before it was measured.** The plan stated "no
  `pwsh`" from an older reading; the host had changed two days earlier. The check that found it was `command -v`,
  run for another reason. A recurrence of the Tier-1 rule of 2026-08-09 (a claim about a system is a hypothesis until
  the artifact that holds the fact confirms it).
- **A citation of a master section can resolve to a key file.** The plan cites the limit at "test-plan §3 `run`"; the
  master's §3 body is a pointer line and the sentence lives in
  `.andromeda/registries/contracts/test-plan/5-command-implementation.md`. Six probes of the master found nothing
  before the key file was read.
- **Sweep hazards found:**
  - `no rust (code )?read|reader-less` also matches the host-dev-tool handles' clause in security-plan and its leaf
    (three hits on a different subject).
  - `pwsh` alone matches a CI step's `shell: pwsh`, a dated measurement's shell name and a command form; only a hit
    beside "has no" states the retired reason.
  - The envelope's `verdict` type stands in two spellings: the JSON sample `"Pass | Fail | CalibrationRegion"` and the
    prose set `verdict ∈ {Pass, Fail, CalibrationRegion}`; a pattern for one misses the other.
  - The claim "no Rust reader" also stands as the ordinals "FIRST" and "SECOND" inside two multi-KB lines.
- **A `cargo audit` gate can wait on a lock another process holds.** The audit log read "advisory-db is locked,
  waiting" once: another project's audit ran on the host at the same time. The porcelain after the scan was empty.

## Outcome
Acceptance criteria, each re-asserted against the diff:
- (close) one table row for each of the 11 ids and no other — **met**: both counting entries read 11.
- (close) the ledger reads as the close states it — **met**: `verified 11/11 · deferred 0 · planned 0 · implemented
  0 · unclaimed 0`, `done-test: YES`.
- (arch) the `v3-09` statement is no wider than three drives, attributes the pass to no cause, keeps 2 misses of 4
  as a rate — **met**: `evidence/version-close.md`, Three statements 1 and Limits.
- (obs) the pairing-window statement names its three limits; the `v3-09` row names a harvest-tier test and offers no
  envelope state — **met**.
- (security) no corpus-rendered model text in a file this chunk writes — **met**, by construction: the frozen
  `report.md` and the captures were not opened; the exception is stated by location only.
- (security) no absolute host path in this chunk's evidence — **met**: the entry reads 0, re-read after
  `operator-pass.md` was completed.
- (security) the two script diffs are comment lines only — **met**: 0 and 2; the parser reading agrees.
- (security) `.andromeda/security-plan.md` is not written by /implement — **met**; what the wrap writes there is
  this wrap's P2.
- (security) the contract carries the ruling as one dated add-only block — **met**: 0, 0, 1 and 1; harvest 139 of
  139.
- (close) the close record carries the founder's pick verbatim, each part marked for whose words it is — **met**.
- (a11y) no a11y-backed row claims conformance beyond its owning chunk's evidence, none claims a VoiceOver, Orca or
  Linux-host leg as run — **met**.
- (tests) the secret-scan and harvest targets pass with every test run — **met**: 7 of 7 and 139 of 139.
- (tests) `bash scripts/agent-run.sh run` exits 0 on the final tree — **met**: 1239 of 1239, doctests ok, three
  clippy lines finished, on the tree after the last evidence edit of /implement.
- (tests) the CI run on the pushed commit reads `verdict: green` — **met**: `CI#37763841102`. The criterion's
  sentence "that run is the only execution the two edited `.ps1` files get" holds as a fact (neither was executed on
  the dev host). The plan's stated reason for it, no `pwsh` on this host, is false (Spec claims disproved, item 1);
  the operator accepted the correction and directed the amendment.
- (direction) the founder's words whole, in the file's only quotation, equal to the relay's — **met**: 1 opening
  mark; 344 and 344 characters, equal.
- (stops) the close record names every act it did not do, and none was done — **met**: the push moved the build
  branch only; the "nothing else moves" entry exits 0.
- (arch, the wrap's own) both registries within target after the amendment, the posture row true, both ordinals as
  at the base, the key line and the two leaves corrected — **owed at this wrap's P2**; the instrument's reading
  after the apply decides it, and a failure to fit is reported unmet with the arithmetic, never reworded.

Gates, by `run`, from /implement's second pass on the final tree (27 green, 0 red, 3 not run — operator legs):
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` (before the audit) — green · exit 0 · no
  output
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/matrix.py coverage --dir conductor-0.3.0` — green
- `git diff fb48cee… -- scripts/agent-run.ps1 scripts/a11y-token-witness.ps1 | … grep -cvE '^[+-][[:space:]]*#'` —
  green · exit 1 · last line 0
- `git diff -U0 fb48cee… -- (the two scripts) | grep -cE '^\+[^+]'` — green · last line 2
- `test -f … && cat (the two scripts) | grep -c 'host-win32'` — green · exit 1 · last line 0
- `grep -c -F '(.claude/rules/verification-harness.md, 2026-09-16)' scripts/agent-run.ps1` — green · 1
- `grep -c -F '(.claude/docs/session-learnings.md, the 2026-09-11 host-leaf entry)' scripts/a11y-token-witness.ps1` —
  green · 1
- `git diff --quiet fb48cee… -- Cargo.lock Cargo.toml crates scenarios .github …` — green · exit 0
- `git diff fb48cee… -- contracts/pulse-real-model-leg-posture.md | grep -v '^--- ' | grep -c '^-'` — green · 0
- `git diff -U0 … | grep '^@@ ' | grep -cvE '^@@ -(49,0 \+50|50,0 \+51)[, ]'` — green · 0
- `git diff -U0 … | grep -c '^@@ '` — green · 1
- `grep -c -F '[corrected 2026-10-08 (2026-10-08-version-close-on-measured-evidence,' contracts/…` — green · 1
- `grep -c -F '«Нет, исправить фразу (Recommended)»' …/evidence/version-close.md` — green · exit 0
- `grep -o -E '^\| `v3-(0[1-9]|1[01])` \|' … | sort -u | wc -l` — green · 11
- `grep -c -E '^\| `v3-' …/evidence/version-close.md` — green · 11
- `grep -o '«' …/evidence/next-version-direction.md | wc -l` — green · 1
- `ls …/evidence/version-close.md >/dev/null && ls …/evidence/ | grep -c '^rm-capture'` — green · exit 1 · 0
- `ls … && cat …/evidence/* | grep -cE "\b[A-Za-z]:[\\/]|…|%APPDATA%"` — green · exit 1 · 0
- `cargo fmt --all --check` — green
- `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci` — green · 7 tests run: 7 passed, 0
  skipped
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green · 139 tests run: 139 passed, 0
  skipped
- `bash scripts/agent-run.sh run` — green · exit 0 (1239 tests run: 1239 passed, 0 skipped)
- `grep -c '^\[\[package\]\]' Cargo.lock` — green · 562
- `cargo audit` — green · exit 0 (1294 advisories, 562 packages, 7 allowed warnings)
- `git -C … advisory-db status --porcelain` (after the audit) — green · no output
- `cargo deny check advisories bans licenses sources` — green
- `python -X utf8 scripts/arch-registry-check.py measure --file .andromeda/architecture.md` — green · 38114 B and
  38082 B
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`: fired by hand on
  the operator's word, exit 0, `hygiene: clean` (read 46, then 47 with the record in the set); recorded in
  `evidence/operator-pass.md`
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=…"` — `leg =
  'operator'`: the operator's act, made on the operator's word; exit 0, `PUSHED_SHA=42daf3e47e30…`; recorded
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1500` — `leg =
  'operator'`: exit 0, `verdict: green`, `CI#37763841102`; recorded
- No `defer` entry; no deferral. Smoke: skipped — no boot-path change (the one touch on a harness shell is a comment
  line; `bash scripts/agent-run.sh run` was green as a gate entry).

Watches: none folded.

Outcome basis: the operator pass ran. The gate verdicts rest on its final state: the one pre-CI commit `42daf3e`
(Setup's list) and that HEAD's CI run `CI#37763841102`, recorded in `evidence/operator-pass.md`. /implement's P4
report, given in this session's conversation, is the basis for what only it holds (the deviations, the parser
reading, the census). Between /implement and this report the operator gave two directives: the operator pass's word
(no `pwsh` leg; stop after the CI read) and the wrap relay (the masters owed, the three calls, the stops).
Post-implement artifacts: `evidence/operator-pass.md`, whose commit, push and CI sections were appended after the
push and ride this wrap's commit.

Process hygiene (implement P4's census, measured from the host's process list; re-measured at this wrap):
- `gate.py run` twice with their cargo, nextest, clippy, audit and deny children — started by this run — terminated.
- `pwsh` four times (the version read, three parse comparisons) — started by this run — terminated.
- `ci.py conclusion`, `gh` reads and the push at the operator pass — started by this run on the operator's word —
  terminated.
- another project's `cargo-mutants` and `cargo-nextest`, and a waiter process naming a builder session — not started
  by this run — left running: not this run's to stop.
- No `pulse-app` and no sidecar was started or found.
## New text, by line
Generated by `cites.py added` (cites v1.2); pasted by `splice.py`. No line of this section is typed or edited.
The diff: fb48cee4 (the parent of the oldest pre-CI commit 42daf3e4) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### contracts/pulse-real-model-leg-posture.md — added 8 line(s) in 1 range(s)
added: 51-58
### scripts/a11y-token-witness.ps1 — added 1 line(s) in 1 range(s)
added: 160
### scripts/agent-run.ps1 — added 1 line(s) in 1 range(s)
added: 230
