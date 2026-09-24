# Report — 2026-09-24-secret-scanning-ci-gate

**Chunk:** secret-scanning CI gate, key/cert ignores, undeclared-env-key step, write-guard backslash fix
**Date:** 2026-09-24T14:05:00Z
**Commits:** `6008a68` chore(2026-09-24-secret-scanning-ci-gate): operator pre-CI commit, for the run this chunk's verdict reads

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/tests/secret_scan_gate.rs` (new) · `crates/conductor-core/tests/workflow_env_gate.rs`
  (new) · `.github/workflows/ci.yml` · `.gitignore` · `Cargo.toml` · `crates/conductor-core/Cargo.toml` · `Cargo.lock` ·
  `.claude/settings.json` · chunk folder (`scope.md`, `research.md`, `plan.md`, `evidence/mutation-controls.md`,
  `evidence/ci-run-record.md`) · `conductor-0.3.0/verification-matrix.json` (`v3-11`). Basis: `git diff --name-only
  4837210 HEAD` plus `git status --short`.
- **Symbols / APIs:** two new `conductor-core` TEST targets. No shipped or public symbol is added or changed, and no
  crate-root export is added: both targets use only `std` and `regex`, never `conductor_core`.
  - `secret_scan_gate` (5 tests). Subject: `git ls-files -z --cached --others --exclude-standard`, spawned from the
    test binary as the fixed program `git` with fixed argv and no operator value. It is the first test binary in the
    workspace to spawn a PATH-resolved external program; the other 3 test spawns are in-tree `CARGO_BIN_EXE_*` binaries
    (`grep -rnE 'Command::new\(' crates/*/tests/` @ 4837210 → 3 sites). Rules: 14 content rules plus a
    secret-file-name class. A test-local `ALLOWLIST` const, EMPTY, is graded as an exact set in both directions. A hit
    renders as `{path}:{line}: secret-shaped string ({rule}, {N} chars)` or `{path}: secret-shaped file name
    (secret-file-name)`: repo-relative, never the matched text.
  - `workflow_env_gate` (5 tests). A line model over `.github/workflows/*.yml|yaml`: `${{ env.X }}` and bare
    `if: env.X` reads are checked against the `env:` keys at every level plus the keys written to `GITHUB_ENV`.
    Admission is file-wide.
  - **New env var:** `GHA_ENV_CONTEXT_PROBE`. CI-only and non-secret, written and read only by the `rust` job's two
    `GITHUB_ENV context probe` steps. Not a Conductor handle, not in the `CONDUCTOR_*` namespace, and read by no
    shipped or test code.
- **Crates / modules:** none added or removed. `conductor-core` gains two `tests/` targets (3 → 5 integration-test
  targets: `operator_pause`, `scenario_audit_gate`, `toolchain_smoke` + the two new ones; `ls crates/conductor-core/tests/*.rs`).
- **Dependencies:** `regex = "1.12"` added to root `[workspace.dependencies]` and as a `conductor-core`
  `[dev-dependencies]` edge. `regex 1.12.4` was already locked (transitive via `urlpattern` ← tauri-utils), so the
  package count stays **562** (`grep -c '^name = ' Cargo.lock` before and after). The `Cargo.lock` diff is exactly one
  line, `"regex",` under conductor-core's dependency list (`git diff Cargo.lock`). Dev-only, never in a shipped binary.
- **Schema / config:** `.gitignore` secrets block: `.env` + `.env.*` → `.env*` plus `*.pem *.p12 *.pfx *.key *.cer
  *.crt *.jks *.keystore id_rsa* id_dsa* id_ecdsa* id_ed25519*`, mirroring the gate's file-name class. Nothing tracked
  became ignored: the class matched 0 of 3 674 tracked paths @ 4837210. `.claude/settings.json:14`: the generated-dir
  PreToolUse write guard gains `| tr "\\134" "/"` after `tr -d "\\r"`, byte-identical to
  `andromeda-setup-project/references/hooks-matrix.md:89` (equality probe → True). Host tooling, not a spec master.
- **Spec-master edits:** none. /implement touched no master.
- **Counts / qualifiers moved:**
  - `rust`-job steps: 23 → 27, counting `^      - (name|uses):` between `rust:` and `frontend:` @ 4837210 / HEAD.
    No master bakes the literal: `grep -F '23 steps'` / `'22 → 23'` / `'step count'` over the seven → 0 each.
  - Workspace nextest total: 1077 at HEAD (measured, implement entry 3). The prior total of 1067 is DERIVED, not
    measured: 1077 minus the 10 arms of the two new targets, since this chunk touches no other test file (the
    previous chunk's report states no workspace total). No master bakes either value: `grep -F '1067'` / `'1077'`
    over the seven → 0.
  - The static-gate FAMILY changes KIND, not just size. The masters name it "the static gates over committed data",
    each "asserting a committed artifact against the production source it is derived from" (architecture.md:37; the
    same phrase at :60, :245, :248, :278 — `grep -F 'static gates over committed data'` → architecture 5, test-plan 1).
    Neither new gate is of that kind. The secret scan asserts a workspace-wide absence, and the env-context gate
    asserts a workflow file against ITSELF (its declarations). Every one of the six sites names its set as "the static
    gates over committed data", so each now under-describes the CI gate set.
- **Dev-tool versions:** none. No host tool was installed or upgraded. `regex` is a lockfile-resolved crate, NOT a
  host tool. `jq` re-read on the dev host at `/c/Users/turbo/scoop/shims/jq`, version not read (P5 baseline of the
  `command -v jq` entry).
- **Harness / gate surface:**
  - `.github/workflows/ci.yml`, `rust` job: four named steps after `Flakiness budget (assert zero-retry)`, each with
    keys {name, shell, run}, no `continue-on-error`, no `if:`. `continue-on-error` count 8 → 8, `if:` 11 → 11,
    `uses:` 15 → 15 and `Invoke-WebRequest|curl|wget|iwr` 2 → 2 (plan probes 16-20, green):
    - `Secret-scan gate` — presence guard `test -n "$(git ls-files)"` or `::error::` + exit 1, then `cargo nextest run
      -p conductor-core --test secret_scan_gate --profile ci`;
    - `GITHUB_ENV context probe (write)` — writes `GHA_ENV_CONTEXT_PROBE=written-by-an-earlier-step` to `$GITHUB_ENV`;
    - `GITHUB_ENV context probe (assert)` — compares `"${{ env.GHA_ENV_CONTEXT_PROBE }}"` (expression form) and on
      mismatch emits `::error::` + exit 1, else prints `GITHUB_ENV context probe: the env expression context carries a
      GITHUB_ENV key`;
    - `Workflow env-context gate` — presence guard `ls .github/workflows/*.yml` or `::error::` + exit 1, then `cargo
      nextest run -p conductor-core --test workflow_env_gate --profile ci`.
  - `scripts/agent-run.{sh,ps1}`: UNCHANGED. The new targets run inside `run`'s existing `nextest run --workspace
    --profile ci` (`agent-run.sh:314`), so there is no new command and no new stage flag.
  - The CI non-loopback egress count stays ONE (the msedgedriver pin). No marketplace action was added.
  - The `.claude/settings.json` write guard now blocks backslash paths. Smoke per setup `validation.md` §Hook smoke
    test step 6: `src/x.rs` → 0; `target\x.rs` and `C:\p\target\x.rs` → 2 (both were 0 on the unfixed guard).
- **Cross-project / external claims:**
  - CI run **36006370951**, measuring sha **`6008a68dfd82d91fc2bfae2ff4ff681e6a91ff13`**, conclusion **success**. All
    three jobs (Rust · Frontend · A11y) and all four new steps are `success`. The provider API reproduces it: 3 check
    runs, `total_count` 3, all completed/success. This wrap's own commit adds to that tree.
  - **F12, measured:** on `windows-latest` at that run, a key an earlier step writes to `GITHUB_ENV` DOES resolve
    through `${{ env.* }}` in a later step of the same job. The assert step printed its success line at
    13:37:04.2368847Z (`evidence/ci-run-record.md`). GitHub's docs do not state this: three pages fetched 2026-09-24
    (contexts · workflow-commands · use-variables), none addressing the combination.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `architecture.md:200` (§Occupied Resources — Environment variables, the `EDGEWEBDRIVER` bullet): "…resolves
    `CONDUCTOR_MSEDGEDRIVER` from it in the step SHELL (`$env:EDGEWEBDRIVER`), because GitHub's `${{ env.* }}`
    expression context holds only workflow/job/step declarations and never a runner-process variable."
    - Since 2026-09-17 the same bullet records that the pin gate WRITES `EDGEWEBDRIVER` through `GITHUB_ENV`, and this
      chunk measured that a `GITHUB_ENV`-written key DOES resolve through `${{ env.* }}` in a later step (run
      36006370951). The "because" therefore no longer explains the shell read of THIS value.
    - The shell read stays correct and needs no change. The context's "never a runner-process variable" half stays
      true of an IMAGE-set variable (the 2026-09-07 red, run 34148079506, predates the `GITHUB_ENV` producer).
    - Disposition: amend the body sentence's rationale.
    - The same stale rationale sits in a NON-master: `ci.yml:351-356` (the comment block above `A11y routine arm`),
      which says `EDGEWEBDRIVER` "stays a runner-PROCESS variable resolved in the step's SHELL: the ${{ env.* }}
      expression context holds only what a workflow, job or step declared". Its owner is route-resolve: a CARRY
      onto the next entry that edits `ci.yml`.
- **Expected amendments (from plan):**
  - security-plan §Security Decisions Log / §Bootstrap phases `secret-scanning-ci-gate` / §Secret Management "Secret
    scanning in CI" / §Threat Model Summary CI/CD list → **carried**: Changes "Harness / gate surface" + "Symbols". The
    operator selected an in-repo Rust static gate (P4, 2026-09-24), closing Decisions Log open question (3). Sites:
    `grep -c -F` in security-plan — `Security Decisions Log` 3 · `Open questions` 1 · `secret-scanning-ci-gate` 2 ·
    `Secret scanning` 1 · `CI/CD` 2.
  - architecture §Established Decisions [CI/CD] + the other CI gate-set enumeration sites (§Stack CI/CD row :37,
    `.github/workflows/` tree comment :245, §Infrastructure CI/CD approach :248, §Inherited Defaults — CI/CD :278) +
    §Occupied Resources — Environment variables (`GHA_ENV_CONTEXT_PROBE`; the measured `GITHUB_ENV` fact) →
    **carried**: Changes "Counts / qualifiers moved" (the family changes kind), "Symbols" (the env var) and
    "Cross-project" (F12). Sites: `static gates over committed data` → architecture 5; `GITHUB_ENV` → architecture 1
    (:200); `build + test gating` → architecture 1. The plan's budget warning stands: the 2026-09-24 compaction left
    186 B / 208 B of margin, so `D-arch-registry-size` is expected to trip, remedied by moving this wrap's history
    into the sidecar.
  - test-plan §6 (two sibling static-gate legs), §9/§10 build-failure conditions, no §9 stage-table row → **carried**:
    Changes "Harness / gate surface". Sites: `Scenario-assertion audit leg` 1 · `static gate` 2 · `Build failure` 2 ·
    `static gates over committed data` 1.
  - obs-plan §1 Not-instrumentable build-time list, §9 Pipeline integration, §10 build-failure conditions →
    **carried**: Changes "Harness / gate surface" and the Coverage line below (instrumentation n/a — build-time). Sites:
    `Not-instrumentable` 1 · `fmt` 4 · `Build / deploy` 1.
  - a11y-plan — the verbatim quotes of arch §Stack's CI rationale cell, only if that cell changes → **carried
    conditionally**: `build + test gating` → a11y-plan 3 hits, architecture 1 (:37). The cell's rationale text
    ("Formatting, build + test gating, the static gates each asserting a committed artifact against the production
    source…") is exactly what the family change touches.
- **Coverage of new surfaces:**
  - `secret_scan_gate` (CI + local test target) → validation n/a (no external input; the subject is git's own listing)
    · instrumentation n/a (build-time, obs-plan §1 Not-instrumentable) · PII redacted✓ (the hit never echoes the match;
    repo-relative paths; the mutation control counted 0 echoes) · tests unit✓ (5 arms + a one-shot mutation control) ·
    a11y n/a · tokens n/a
  - `workflow_env_gate` (CI + local test target) → validation n/a · instrumentation n/a (build-time) · PII n/a · tests
    unit✓ (5 arms + a one-shot mutation control) · a11y n/a · tokens n/a
  - `GITHUB_ENV context probe (write|assert)` (CI steps) → validation n/a · instrumentation n/a · PII n/a (fixed
    non-secret literal) · tests: self-asserting in CI, measured green at run 36006370951 · a11y n/a · tokens n/a
  - `.claude/settings.json` write guard (host hook) → validation n/a · instrumentation n/a · PII n/a · tests: gate probes
    13-15 (setup §Hook smoke step 6) · a11y n/a · tokens n/a

## Deviations from intent
- **Added, not in the plan:** a ONE-SHOT mutation control at implement P2 against the REAL subject. An untracked,
  non-ignored file with a runtime-built token turned `secret_scan_gate` red with
  `planted-secret-control.txt:2: secret-shaped string (github-token, 44 chars)` (0 echoes). A throwaway workflow with an
  undeclared read turned `workflow_env_gate` red naming `.github/workflows/zz-planted-control.yml:7`. Both plants were
  removed and both gates returned green (`evidence/mutation-controls.md`). Justification: the in-suite negative arms
  prove the check logic, while this proves the real subject enumeration reaches a planted file end to end. Being
  mutating, it is correctly NOT a listed entry.
- `secret_scan_gate`'s `Allowed.reason` field carries `#[allow(dead_code)]`: it documents each entry and is never
  read. Nothing else deviates.

## Decisions & corrections
- **P4 operator decisions (AskUserQuestion, 2026-09-24):**
  - Scanner form: an in-repo Rust static gate, rejecting gitleaks (a second CI egress and third-class member) and a CI
    `scripts/` Python instrument.
  - `GITHUB_ENV` keys ADMITTED, with a CI self-probe pinning the premise on every run.
- **P5 operator review (overseer):** concretize `v3-11` to what the chunk PROVES — the gate test fails on a planted
  sample and the step is wired unconditionally — never to "CI goes red", which no step observes. Keep the CI conclusion
  report-only. Order the push and both CI reads BEFORE the wrap, so a red gate step is seen before the matrix flip.
  Both were folded into the plan and the full P5 check set was re-run.
- **Operator instruction:** this build session made the pre-CI commit `6008a68`, ran the push entry exactly as
  listed, waited for the run and recorded entries 22–23.
- **Operator correction — attribution:** `evidence/ci-run-record.md` first said "The overseer made these". It was this
  build session, on the operator's instruction; the phrase was copied from the `fc4a9c2` precedent, where the overseer
  did make it. The evidence is corrected. The pushed commit `6008a68`'s body line 17 carries the same misattribution
  ("Made by the overseer on the operator's explicit instruction."). It is not amended (a force-push is the operator's
  call); THIS wrap's commit records the correction.
- **Sweep hazards found this chunk:**
  - A backslash payload typed through the Bash tool is halved. A Python substring probe for `tr "\\134"` printed False
    on a fix that a backslash-free equality probe proved present. Probe with equality or a file-borne script.
  - `grep -c continue-on-error ci.yml` counts a COMMENT line (`:527`). The 8 is 7 steps + 1 comment, which is fine as
    an unchanged-by-design count and wrong as a step count.
  - `git ls-files --cached` alone misses untracked new files, so a gate using it would not scan its own new source
    until committed.
  - A greedy end-anchored regex for the "rendered fence" of a two-fence template `.md` spans both fences. It reported
    a false 17-line template drift at session start (new-session check 11).
  - A secret gate's own rule TEXT and samples must not self-match. The rule text is written so no rule matches it, and
    every sample is assembled by concatenation. A URL sample written as a `format!` string (an https scheme, then two
    `{}` placeholders around a colon, then an at-sign) DOES self-match, because `{}` are legal userinfo chars. The
    concat form was used instead. Quoting that literal in prose trips the gate too: this wrap's own report and
    curation log did, and the light gate caught both (hence this description instead of the literal).
- `codebase-research.md` vs evolve `research.md` / validation check 7 disagree on the graph label for new test-only
  `.rs` files. Recorded as friction; research.md was re-worded to "not consulted".

## Outcome
**Acceptance criteria** (re-asserted against the diff):
- `secret_scan_gate` passes on the real subject with an empty allowlist; each of 14 rules fires on its sample; a planted
  sample fails naming `src/planted.rs:2:` + rule without the sample; file names fail; allowlist rot fails — **met**
  (5/5; `v3-11` ref written).
- `.gitignore` covers every secret file class — **met** (`secret-class ignore rules: all present`).
- `workflow_env_gate` positive + negatives + controls — **met** (5/5).
- Four named steps once each, no `continue-on-error` / `if:` added, presence guards — **met** (probes 16-18).
- No egress or action added; no new package; audit + deny green — **met** (probes 19, 20, 10; audit exit 0 — 1268
  advisories · 562 crates · 7 allowed = 6 unmaintained + 1 unsound; deny `advisories ok, bans ok, licenses ok, sources ok`).
- Write-guard smoke 0 / 2 / 2 — **met**.
- Workspace nextest 1077/1077, fmt, clippy — **met**.
- Hit reports repo-relative, never the match, no telemetry artifact touched — **met**: the gate's output is test output
  only; no logs/ or runs/ artifact is written.
- The operator's push and both CI reads recorded BEFORE the wrap — **met** (`evidence/ci-run-record.md`: run
  36006370951, success, all four new steps success).

**Gates** (/implement P2 run, `.andromeda/runs/2026-09-24T12-36-55-implement`, first full run 20 green / 0 red / 3 not
run):
- `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci` green (5/5) ·
  `cargo nextest run -p conductor-core --test workflow_env_gate --profile ci` green (5/5) ·
  `cargo nextest run --workspace --profile ci` green (1077/1077) · `cargo fmt --all --check` green ·
  `cargo clippy --workspace --all-targets -- -D warnings` green
- `git -C "$CARGO_HOME/advisory-db" rev-parse HEAD` green · `… status --porcelain` green (no output) · `cargo audit`
  green · `cargo deny check advisories bans licenses sources` green · `grep -c "^name = " Cargo.lock` green (562)
- the `git check-ignore -q --no-index` loop green · `command -v jq` green · the three hook-smoke probes green (0 · 2 · 2)
- the named-step count green (4) · `continue-on-error` green (8) · `if:` green (11) · egress green (2) · `uses:` green (15)
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=…"` — `leg = 'operator'`,
  run once by this build session on the operator's instruction: exit 0, `PUSHED_SHA=6008a68…`
- `gh run list … select(.headSha==env.SHA) …` — `recorded`: printed `36006370951`
- `gh run view <id> …` — `recorded`: `completed` / `success`, all four new steps `success` — this chunk's run,
  disposition: green, nothing to route.
- Smoke: skipped — no boot-path / UI-surface change.

**Outcome basis:** implement's P4 report as given, plus the operator directive after it (commit + push + CI reads,
performed by this build session) and the post-implement artifacts `evidence/ci-run-record.md` (attribution corrected)
and `evidence/mutation-controls.md`.

**Process hygiene:** implement's census, re-measured at 12:44Z through `Win32_Process` parentage. Every
`cargo`/`rustc`/`nextest`/`git` process this chunk's gate runs started had exited. The 5 `cargo` + 1 `cargo-nextest`
+ 1 `rustc` then alive belonged to a foreign `viola-harness … run --mutants` tree started from another `bash` session:
left running, not this chunk's to stop. This session started no other process; `gh run watch` exited on completion.
