# Report — 2026-10-04-real-model-test-surface-corrective

**Chunk:** Real-model test-surface corrective — the real-model harvest split by series with its grading lifted, the
secret-scan gate skipping where no .git exists, and the stub-server test targets compiled by a gate
**Date:** 2026-10-04T12:35Z
**Commits:** since `last_wrap` 2026-10-04T11:41:51Z — `91f0f04 chore(2026-10-04-real-model-test-surface-corrective):
operator pre-CI commit, for the run this chunk's verdict reads` (base `1208ca5`, the pre-CI commit's parent).

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-status 1208ca5` + `gate.py scope`: changed 15 · listed 15 · recorded 0)
  - modified: `crates/conductor-run/tests/real_model_harvest.rs` · `crates/conductor-run/tests/real_model_common/mod.rs`
    · `crates/conductor-core/tests/secret_scan_gate.rs` · `scripts/agent-run.sh` · `scripts/agent-run.ps1`
  - new: `crates/conductor-run/tests/real_model_grading/` — `mod.rs` · `witnesses.rs` · `scrub_and_sweep.rs` ·
    `canary_pairing.rs` · `capture_tokens.rs` · `capture_population.rs` · `workspace_mask.rs` ·
    `series_2026_09_29.rs` · `series_2026_09_30.rs` · `series_2026_10_01.rs`
  - no `src/` file changed in any crate; no manifest; no `ci.yml`.
- **Symbols / APIs:** test-tier only; no pub signature in any `src/` changes.
  - `real_model_harvest` target: stays ONE target (no rename). The rule section (between the `// ---- rule: begin/end ----`
    markers) is byte-identical to `1208ca5` (diff probe: no output). The pre-committed-rule items (`row()`, `Grade`,
    `grade`, …) keep their home in `real_model_harvest.rs`, private. The families a01–a21, the posture table, the
    `p031_*`/`p034_*`/`p044_*` arms, the canary-token arm and the 2026-09-23 pinned-capture section stay in the root.
    Moved verbatim into CHILD modules (`real_model_grading/*`, each `use crate::*;`): B1/launch/route witnesses,
    host-path mask + sweep window, canary pairing, fingerprint elision + producer/grader agreement, the 14-capture
    population arm, the workspace-key mask arms. The three dated series are rewritten as thin named `#[test]`s over one
    harness in `real_model_grading/mod.rs` (`Series { drives, evidence, measured }` with `capture` · digest arm ·
    rule-recorded arm · no-fingerprint arm · grades-as-ledger arm · real-model/launch witness arm · `graded()`;
    free fns `v3_09_met`, `contract_section`, `pre_registered`).
  - The harvest's private `sha256_hex` / `check_digest` / `committed` / `pinned` are DELETED; the target now declares
    `mod evidence_pin;` and uses its four (third caller of `tests/evidence_pin`, beside `lifecycle_harvest.rs` and
    `delegated_timing_harvest.rs`).
  - `tests/real_model_common/mod.rs`: new `pub fn envelope_record(block, label) -> Option<RunRecord>` (the `envelope:`
    line's key set held to `ENVELOPE_KEYS_SORTED` on a `serde_json::Value`, then the typed parse); `#![allow(dead_code)]`
    added (the `live-pulse` capture does not call the helper). Callers: the root's 2026-09-23 arm + the three series'
    eleven-keys arms. The module doc's two-user sentence is unchanged and still true.
  - Root `no_committed_capture_text_sits_in_test_source`: detection extracted to `capture_text_in(sources, captures) ->
    Result<usize, String>`; source set widened from 4 files to 4 + all 10 `real_model_grading/*.rs` (`GRADING_MODULES`,
    `include_str!` each). New arms: `the_capture_text_arm_scans_every_grading_module` (runtime `read_dir` of
    `tests/real_model_grading/`, `CARGO_MANIFEST_DIR`-anchored, set-equal to the list) and
    `a_report_line_planted_in_a_grading_module_is_caught` (in-memory inverse control; a capture report line read at run
    time, never quoted in source).
  - `secret_scan_gate.rs`: `workspace_files(root: &Path) -> Option<Vec<String>>` — `None` when `root.join(".git")` does
    not exist (`Path::exists`, decided before any spawn; a `.git` FILE counts as a repository); otherwise the same fixed
    `git -C <root> ls-files -z --cached --others --exclude-standard` spawn and its "the gate has no subject" assert.
    Sole caller `the_workspace_holds_no_secret_shaped_string`, which on `None` prints ONE path-free stderr line
    `secret-scan gate: skipped — no git repository at the workspace root` and returns; on `Some` its three vacuity guards
    and the grading are unchanged. New arms `a_tree_with_no_git_entry_is_skipped_without_a_spawn` and
    `a_listing_failure_inside_a_repository_still_fails` (`#[should_panic(expected = "the gate has no subject")]`), each
    over a fresh dir under `std::env::temp_dir()` removed by a `ScratchDir` drop guard. No new spawn, argv element or env var.
- **Crates / modules:** no crate added/removed. Test-tier module `crates/conductor-run/tests/real_model_grading/` added (a
  `tests/` subdirectory module, never a target). Targets unchanged in number and name.
- **Dependencies:** none (manifests and `Cargo.lock` byte-unchanged vs `1208ca5`; `[[package]]` count 562).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** test counts only, none baked in a master — `real_model_harvest` 102 → 104;
  `secret_scan_gate` 5 → 7; workspace nextest 1201 passed locally (Linux) / 1204 in CI's dogfood (Windows). Basis: the
  gate logs; master sweep for `1197|1201|1204|104 tests|102 tests|5 tests|7 tests|five arms|seven tests` over the seven:
  0 hits (`\b102\b`: 1 hit, `test-plan.md:38` = `anyhow 1.0.102`, no change). Line counts (`wc -l`): root
  `real_model_harvest.rs` 2602 → **1391** (forecast ~1300; the excess is step 6's `GRADING_MODULES` list and two arms);
  grading modules 60–235 each (`mod.rs` 170); `real_model_common/mod.rs` 353 → 375.
- **Dev-tool versions:** none.
- **Harness / gate surface:** `agent-run run` bundled default (both shells, additions only — `git diff 1208ca5 -- scripts/`
  removes 0 lines) gains, after `clippy --workspace --all-targets -- -D warnings`:
  `cargo clippy -p conductor-verify --features stub-server --all-targets -- -D warnings` and
  `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` — a feature-gated LINT gate kind
  (compile + lint of the `stub-server` and `live-pulse` targets: `preflight_spawn.rs`, the `stub_pulse_mcp` bin, and
  the five `live-pulse` test targets incl. `real_model_live.rs`). CI reaches it through the existing `rust`-job step
  `Test + lint (dogfood agent-run)` (`.\scripts\agent-run.ps1 run`); NO new `ci.yml` step (`ci.yml` byte-unchanged).
  The harness still exposes exactly boot/run/status/cleanup/logs. The secret-scan gate's CI step and its presence guard
  (`ci.yml:79`) are unchanged; the gate itself now skips (one path-free line, exit 0) only where the workspace root has
  no `.git` entry, never on a listing failure inside a repository.
- **Cross-project / external claims:** CI run **CI#37201730301** (push) on sha `91f0f042832700350aef7530fb6c8db581cd4149`
  — `verdict: green · checks 3/3 · wall 680 s` (`ci.py conclusion`); the dogfood step's log shows three
  `Finished \`dev\` profile` clippy lines after `1204 tests run: 1204 passed, 0 skipped` (read via `gh run view
  37201730301 --log`). The overseer re-verified the run's success via `gh`. This wrap's commit adds to that tree.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none. Stated limit (not a defect): the split keeps the root
  at 1391 lines, so the code audit's `sizes.over_800` metric (M2) is NOT cleared for `real_model_harvest.rs` — by the
  overseer's ruling at P5, the root stays that size so matrix v3-10's `ref` citations (by FILE) stay true; the next
  epoch-boundary audit measures it. The audit's four self-clone rows are predicted to fall away; unmeasured here (jscpd
  runs at the audit).
- **Spec claims disproved by measurement:** none.
- **Expected amendments (from plan):**
  - test-plan §3 keyed contract `5-command-implementation` (`.andromeda/registries/contracts/test-plan/5-command-implementation.md:11`)
    — carried: Harness / gate surface. Sweep `clippy --workspace --all-targets` over `registries/**`: 1 hit (that
    file); over the seven masters: 1 hit (`test-plan.md:382`).
  - test-plan §9 Lint row (`test-plan.md:382`) + Live-Pulse scenarios paragraph (`test-plan.md:391`) — carried: Harness /
    gate surface. Sweep `stub-server`: test-plan 1 hit (`:391`), 0 elsewhere; `clippy … live-pulse`: test-plan 1 (`:391`).
  - security-plan §Secret Management → Secret-scan gate shape (`security-plan.md:262`; also `:225`, `:260`), test-plan §6
    Repository-hygiene legs (`test-plan.md:291`), obs-plan §9 Repository-hygiene gates row (`obs-plan.md:447`) —
    carried: Symbols / APIs (secret_scan_gate) + Harness / gate surface. Sweep `secret_scan_gate|[Ss]ecret-scan gate`:
    architecture 1 (`:60`, the CI gate list — subject wording, no skip claim), security-plan 4 (`:225 :260 :262 :370`),
    test-plan 2 (`:173` no-change — names other gates; `:291`), obs-plan 2 (`:447`, `:492` failure-condition line).
  - test-plan §4 Mutation instrument — carried: Symbols / APIs (secret_scan_gate) — `conductor-core`'s mutation tier no
    longer depends on `--copy-vcs true`, measured by gate 11's `.git`-less copy run, never by a mutation run. Sweep
    `copy-vcs`: 0 hits in the seven; semantic sweep `vcs|VCS|\.git\b|copy mode|unmeasurable|no git repository|git
    ls-files`: 2 hits (architecture:60, test-plan:291 — both the subject's `git ls-files` wording, neither the
    dependency). No master states the dependency: an ADDITION for the test-plan detector to judge, not a correction.
  - Architecture — none expected (plan); no arch master site enumerates the bundled `run`'s clippy lines (sweep above:
    0 architecture hits for `clippy --workspace`). Any arch draft is measured with `scripts/arch-registry-check.py
    measure` BEFORE the write (overseer directive).
- **Coverage of new surfaces:**
  - `envelope_record` (test helper) → validation n/a · instrumentation n/a · PII n/a · tests unit (5 callers green) · a11y n/a · tokens n/a
  - `real_model_grading` harness (`Series`) → validation n/a · instrumentation n/a · PII n/a (no capture text in source; the widened no-capture-text arm + its inverse control) · tests unit · a11y n/a · tokens n/a
  - secret-scan skip line (stderr, test binary) → validation `Path::exists` before spawn✓ · instrumentation log line (path-free)✓ · PII n/a · tests unit + integration (`.git`-less copy) · a11y n/a · tokens n/a
  - feature-gated clippy lines (harness) → validation n/a · instrumentation n/a · PII n/a · tests integration (`agent-run.sh run` green; `.ps1` via CI#37201730301) · a11y n/a · tokens n/a

## Deviations from intent
- `contract_section` and the "series rule fixed before d1" body are lifted into the harness (`pre_registered`) — the
  plan listed `contract_section` among the series files' own items, but the 09-30 and 10-01 series used it
  identically. Test names unchanged; each series file keeps its own `SERIES_*_RULE_SHA256` pin.
- Three harness helpers the plan did not name: `Series::graded()`, `v3_09_met()`, the `Measured` alias — the v3-09 arms'
  shared computation; each series keeps its own expectation.
- Cross-sibling items: `KEY` stays in `workspace_mask.rs` (`pub(super)`) and `series_2026_09_30.rs` imports it;
  `un_elided_keyed_values` is `pub(super)` in `capture_population.rs` for `capture_tokens.rs`. The rule section gained
  no `pub`.
- Range boundaries: the plan's ranges overlapped at `:1535` (that arm stays in the root, as the plan's keep-list says);
  section-header comments moved with their families.
- `envelope_record` takes a `label` and returns `Option` — the 2026-09-23 arm and the 09-29 series both tolerate an
  absent envelope (unchanged semantics).
- Root at 1391 lines vs the ~1300 forecast (stated, by the overseer's wrap note).
- scope record: none — `gate.py scope` clean, 0 recorded (changed 15 · listed 15).

## Decisions & corrections
- Overseer P4 rulings (founder-delegated), carried: split = child modules in ONE target; gate site = `agent-run run`
  bundled default, both shells; feature set = `stub-server` + `live-pulse`.
- Overseer directive at /implement: diff-shaped probes anchor on the chunk base `1208ca5`, never HEAD (W182); stop the
  rust-analyzer flycheck before heavy cargo steps (none was running at either check); no pulse-app needed.
- Overseer (overseer1 adversarial read): the secret-scan skip fires ONLY with no VCS at all; a listing failure inside a
  checkout stays red; a test pins both arms; no `--copy-vcs` pin. Met by the two new arms + gate 11.
- Overseer wrap note: `real_model_harvest.rs` stays 1391 lines so matrix v3-10's citations stay true; mint a SECOND
  corrective entry at route-resolve (founder "A").
- Operator pass performed by the agent on the operator's explicit word (hygiene · pre-CI commit · push · CI read) —
  recorded as the operator's acts made on that word (`evidence/operator-pass.md`).
- Sweep hazard: `\b102\b` over the masters hits `anyhow 1.0.102` — a version, not a test count.
- Host: a `cd` at the head of one probe persisted as the session cwd (fourth consecutive session) — recovered with
  absolute paths before any relative write.

## Outcome
- Acceptance, re-asserted against the diff:
  - (tests) 104 passed under nextest and `cargo test`; 102 base names all present (`comm -23` → 0); exactly 2 new
    (`comm -13` → 2) — MET.
  - (tests/security) rule section byte-identical (diff → no output); `rule_predates_the_drive` and the three
    `each_*_recorded_the_current_rule_before_it_fired` arms pass; no `rm-capture*.txt` / `attempt-ledger.md` moved — MET.
  - (security) the no-capture-text arm scans every grading module; the listing arm and the planted-line inverse control
    pass; an implement-time control (a stray `.rs` planted in the dir) turned the listing arm red, then was removed — MET.
  - (arch/obs) no target, member, package, Verdict/ReportState variant, envelope key, span name or attribute added;
    `row()` stays in `real_model_harvest.rs` — MET (the diff adds a test module and a test helper only).
  - (security/tests/obs) secret_scan_gate 7 passed under both runners; `.git`-less copy: exit 0 + the skip line + 7
    passed — MET.
  - (tests/layouts) both shells carry both lines once (`… | last line 4`); 0 removed lines; 5 verbs — MET.
  - (tests) both feature clippy lines exit 0; `bash scripts/agent-run.sh run` exit 0 — MET.
  - (a11y) `ci.yml` byte-unchanged vs `1208ca5` — MET.
  - (security) zero dependency delta; 562 packages; advisory-db self-heal green; `cargo audit` + `cargo deny` exit 0 — MET.
  - (CI) CI#37201730301 on `91f0f04` `verdict: green`, run id in `evidence/operator-pass.md`; dogfood ran both new
    clippy lines on Windows — MET.
- Gates (implement run, by `run` text): advisory-db self-heal `green` · `cargo fmt --all --check` `green` · `cargo nextest
  run -p conductor-run --test real_model_harvest --profile ci` `green` (`104 tests run: 104 passed, 0 skipped`) ·
  `comm -23 …` `green` (last line 0) · `comm -13 …` `green` (last line 2) · rule-section `diff <(git show …)` `green` (no
  output) · `git diff --quiet … rm-capture*/attempt-ledger` `green` · `cargo test -p conductor-run --test
  real_model_harvest` `green` (104 passed) · `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci`
  `green` (7) · `cargo test -p conductor-core --test secret_scan_gate` `green` (7) · `.git`-less copy run `green` (skip
  line + 7 passed; rebuilt `conductor-core` inside the copy) · `cargo clippy -p conductor-verify --features stub-server …`
  `green` · `cargo clippy -p conductor-run --features live-pulse …` `green` · four-line count probe `green` (4) ·
  removed-lines probe `green` (exit 1, last line 0) · `git diff --quiet … ci.yml` `green` · `bash scripts/agent-run.sh
  run` `green` (1201 passed + 3 clippy) · manifests/lock `git diff --quiet` `green` · package count `green` (562) ·
  `cargo audit` `green` · `cargo deny check …` `green` · `gate.py hygiene` leg operator → `hygiene: clean` (evidence) ·
  guarded push leg operator → `PUSHED_SHA=91f0f04…` (evidence) · `ci.py conclusion` leg operator → `verdict: green`,
  CI#37201730301 (evidence). Smoke: skipped — no boot-path / UI-surface change (the harness's edited `run` arm ran as a gate).
- Watches: none folded.
- Outcome basis: the operator pass ran — commit list `91f0f04` (the pre-CI commit) and the final HEAD's CI run
  CI#37201730301 in `evidence/operator-pass.md`; implement's report (this conversation) for the gate verdicts.
- Process hygiene: implement's census — no process left by this chunk's runs (the gate run's cargo/nextest/clippy trees
  exited; the `.git`-less copy's temp dir removed by its trap; no rust-analyzer flycheck tree was running at either
  check); `target/vcsless-copy/` remains as gitignored build cache. Re-measured at this wrap: no cargo/nextest/clippy
  process of this repo running.
