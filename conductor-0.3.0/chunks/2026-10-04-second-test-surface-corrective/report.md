# Report — 2026-10-04-second-test-surface-corrective

**Chunk:** Second test-surface corrective — the delegated-timing harvest split under the size line, and the Epoch 5 audit's new emit/run mutation survivors killed
**Date:** 2026-10-04T14:46Z
**Commits:** `4c1e21a` chore(2026-10-04-second-test-surface-corrective): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since `last_wrap` 2026-10-04T12:45:18Z; `git log --format='%h %s' dab66dc..HEAD`)

## Changes (structured — detectors read this)
- **Files:** semantic (8; `gate.py scope` against base `dab66dc`: changed 8 · listed 8 · recorded 0):
  - `crates/conductor-run/tests/delegated_timing_harvest.rs` — modified: module doc unchanged; `use std::path::Path;` removed; `mod delegated_timing_grading;` + `use delegated_timing_grading::*;` added; the grading helpers and three uncited synthetic families moved out; `mod tests` keeps the window-derivation pair, the P-075 round, the P-075 re-round, the 2026-08-21 legs, the 2026-09-07 re-driven leg and the 2026-09-29 graded leg, byte-verbatim and in base order.
  - `crates/conductor-run/tests/delegated_timing_grading/mod.rs` — new: the base's `:54-341` helpers moved verbatim with `pub(crate)` added on every item and on both structs' fields; `#![allow(dead_code)]`; declares the three children.
  - `crates/conductor-run/tests/delegated_timing_grading/synthetic_bounds.rs` — new: the base's `:568-730` (the `synthetic` helper + 8 tests).
  - `crates/conductor-run/tests/delegated_timing_grading/window_selection.rs` — new: the base's `:843-951` (`synthetic_hue`, `synthetic_tick` + 4 tests).
  - `crates/conductor-run/tests/delegated_timing_grading/p025_hard_grade.rs` — new: the base's `:953-1080` (three synthesizers + 5 tests).
  - `crates/conductor-emit/src/identity.rs` — one inline test added (`rekey_under_one_salt_twice_restores_every_id`); no production line changed.
  - `crates/conductor-run/src/canary.rs` — inline test helper `drive_storms` widened (returns also the call-to-first-arrival virtual duration and the `(trace_id, span_id)` set); its two callers destructure the wider tuple with assertions byte-unchanged; two inline tests added; no production line changed.
  - `scripts/agent-run.ps1` — five `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` lines added in the `''` arm, one after each cargo line; no line removed (`git diff dab66dc -- scripts/agent-run.ps1 | grep -v '^--- ' | grep -c '^-'` → 0).
  - Chunk folder: `plan.md` · `research.md` · `scope.md` · `report.md` · `evidence/{liveness-before.md, split-test-names-before.txt, split-test-names-after.txt, inverse-controls.md, carry-measurement.md, operator-pass.md}`.
  - Hygiene rewrite (not chunk source): `.andromeda/runs/2026-10-04T12-48-50-phase/.p5-dryrun.txt` — 4 host-path substrings elided to placeholders at the operator pass (gate 23).
- **Symbols / APIs:** none public. Test-only: `identity::tests::rekey_under_one_salt_twice_restores_every_id`; `canary::tests::{the_first_storm_leaves_at_the_call_with_no_gap_before_it, every_real_model_canary_occurrence_carries_its_own_span_identity}`; `canary::tests::drive_storms` return widened from `(usize, Vec<Duration>)` to `(usize, Vec<Duration>, Duration, BTreeSet<(Vec<u8>, Vec<u8>)>)` — its only callers are the two existing tests in the same module plus the two new ones. Production `rekey_trace_identity` / `xor_in_place` / `emit_canary_storms` unchanged (bodies byte-identical; the restore guard reads `last line 3`).
- **Crates / modules:** `conductor-run` integration target `delegated_timing_harvest` gains a `tests/delegated_timing_grading/` child-module tree (never a target of its own; `cargo nextest list` still lists ONE binary `conductor-run::delegated_timing_harvest`).
- **Dependencies:** none — `git diff --quiet dab66dc -- Cargo.lock Cargo.toml crates/conductor-run/Cargo.toml crates/conductor-emit/Cargo.toml` exit 0; `grep -c '^\[\[package\]\]' Cargo.lock` → 562.
- **Schema / config:** none.
- **Spec-master edits:** none (before this wrap's P2).
- **Counts / qualifiers moved:**
  - `delegated_timing_harvest.rs` tokei 14.0.0 code lines: 1130 (`dab66dc`) → root 310 · `mod.rs` 252 · `synthetic_bounds.rs` 135 · `window_selection.rs` 76 · `p025_hard_grade.rs` 99 (`tokei -o json` over the target, the plan's gate; `over_800 0`). The forecast was root ≈ 490. tokei reads the BASE file as 1130 code / 0 comments / 12 blanks over 1222 raw lines; the five split files read 872 code / 50 comments / 95 blanks (`tokei -o json` over `git show dab66dc:…` and over the split). A normalized line-multiset comparison base vs split differs only by the wiring (2 lines), three child `//!` lines, the module doc of `mod.rs` (3 lines) + its `#![allow]` and three `mod` lines, three `use super::*;` and one rustfmt re-wrap — so the 1130 → 872 drop is the instrument's base reading, not lost content.
  - Harvest target test count: 36 → 36 (nextest list; 19 under `tests::`, 17 under `delegated_timing_grading::{synthetic_bounds 8, window_selection 4, p025_hard_grade 5}`).
  - `conductor-emit --lib` 137 → 138; `conductor-run --lib` 50 → 52 (nextest + `cargo test`, both runners).
  - Workspace nextest (inside `agent-run.sh run`) 1204 passed locally; CI's dogfood read 1207 (Windows, `4c1e21a`).
- **Dev-tool versions:** none — cargo-mutants 27.1.0 and tokei 14.0.0 re-read, unchanged.
- **Harness / gate surface:** `scripts/agent-run.ps1` `run` verb, bundled default (`''` arm): it now stops at the first non-zero cargo line whoever calls it — with that line's own exit for a caller that does not set `$PSNativeCommandUseErrorActionPreference`, while a caller that sets it (CI's dogfood step) still sees the line throw first, step exit 1 [corrected at this wrap's P2: the first write said "with that line's exit, whoever calls it", the wording `evidence/carry-measurement.md` §3 keeps; the layout-templates doc-agent's reading caught it] — five `$LASTEXITCODE` checks, the file's own idiom (`agent-run.ps1:212`). Before, it blocked only when the CALLER set `$PSNativeCommandUseErrorActionPreference` (CI's dogfood step does; a bare local call did not). The script still never sets that preference (the `--e2e` arm's `$e2eRc = $LASTEXITCODE` capture is unchanged). `scripts/agent-run.sh` and `.github/workflows/ci.yml` byte-unchanged. No CI step added or removed. MEASURED this chunk: `agent-run.sh run` exits 101 at the workspace clippy line on a planted clippy-only lint (`evidence/carry-measurement.md` §1, quoted below).
- **Cross-project / external claims:** CI#37209452847 measured `4c1e21a` → `verdict: green · checks 3/3` (rust: dogfood step success, 1207 passed; a11y: `19 passing` · `2 skipped (expected 2)` · driven session present; frontend success). CI#34689135760 (sha `e3ff4e5f`, 2026-09-12, failure) re-read via `gh run view --log-failed`: `NativeCommandExitException` at `agent-run.ps1:280`, `cargo.exe` exit 100 (nextest), step exit 1.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - obs-plan.md:490 (§10 Build / deploy failure conditions) "`cargo clippy` warnings treated as CI annotations (non-blocking at Minimal, but visible to agent)" — MEASURED false for `agent-run.sh run` (exit 101 at the workspace clippy line on a planted `-D warnings` lint) and false by mechanism for CI's dogfood step (it runs the ps1 bundled default, which since this chunk exits non-zero on any red cargo line; before, under the step's own preference, CI#34689135760). Evidence: `evidence/carry-measurement.md`.
  - The plan's forecast "root ≈ 490 tokei code lines" — measured 310; basis above (an instrument reading, no acceptance rests on it; the acceptance `over_800 0` holds).
- **Expected amendments (from plan):**
  - obs-plan §10 `cargo clippy` failure-condition line (+ §9 Lint / typecheck row consistency) — **carried**: Harness / gate surface + Spec claims disproved bullets. Search: `clippy[^\n]{0,160}non-blocking|non-blocking[^\n]{0,160}clippy` over the seven masters + `contracts/**/*.md` (python `re`, `/tmp` scratchpad `sites.py`): 1 hit, `obs-plan.md:490`; `grep -n -i clippy .andromeda/obs-plan.md`: 3 hits — `:42` (§1 build-time row, names clippy as a build-time step: no change), `:443` (§9 Lint / typecheck row — "warnings to stderr … CI annotations": states the channel, not blocking-ness; consistency to re-read), `:490` (the line to restate). 0 hits in the other six masters.
  - test-plan §3 → `5-command-implementation` — **carried**: Harness / gate surface bullet. The keyed contract lives at `.andromeda/registries/contracts/test-plan/5-command-implementation.md` (registry `.andromeda/registries/test-plan-contracts.toml` row `5-command implementation`); search `PSNativeCommand|LASTEXITCODE|bundled default|set -euo` over it (python `re`): `PSNative` 1 hit @ char 10069 (the `--e2e` capture-then-print caveat — still true, no change), `clippy` hits @ 3876-4081 (the `run` command body listing the five cargo lines — where the both-shells stop rule belongs), @ 9096 / 12857 (stage-flag mapping, aggregate — no change); `LASTEXITCODE` / `set -euo` / `bundled default`: 0 hits in the contract. test-plan.md body: `bundled default` 2 hits (`:382`, `:391` — §9/§12 rows naming the three clippy lines inside the bundled default; no blocking claim, no change).
- **Coverage of new surfaces:**
  - `agent-run.ps1 run` bundled-default exit semantics → validation n/a · instrumentation n/a (harness script) · PII n/a · tests: sh half measured (CARRY-sh), ps1 green path CI#37209452847, ps1 red path `unrunnable-here` (no `pwsh` on the Linux dev host) · a11y n/a · tokens n/a
  - new inline tests (identity / canary) → validation n/a · instrumentation n/a (no span, no log field added) · PII n/a · tests unit ✓ · a11y n/a · tokens n/a

### The CARRY measurement, verbatim (`evidence/carry-measurement.md`)

> # CARRY measurement — does a lint warning fail `agent-run run` and CI's dogfood step?
>
> The basis for the obs-plan §10 `cargo clippy` failure-condition amendment. Measured at /implement on 2026-10-04 (Linux
> dev host, tree = base `dab66dc` + this chunk's edits), plus one recorded CI run. Host paths are elided throughout.
>
> ## 1. `scripts/agent-run.sh` — MEASURED: a lint warning fails the bundled default
>
> The one-shot CARRY-sh form of `plan.md` §Test Commands, run once from the project root:
>
> ```
> mkdir -p target/carry && f=crates/conductor-emit/src/identity.rs && cp "$f" target/carry/identity.rs.bak && trap 'cp target/carry/identity.rs.bak crates/conductor-emit/src/identity.rs' EXIT && printf '\n#[allow(dead_code)]\nfn planted_lint(v: &[u8]) -> bool {\n    v.len() == 0\n}\n' >> "$f" && { bash scripts/agent-run.sh run; test $? -eq 101; }
> ```
>
> - **Exit of `bash scripts/agent-run.sh run`: 101**, read from the bare command by the form's own `test $? -eq 101`
>   (the form exited 0, 2026-10-04T14:24:45Z → 14:25:01Z).
> - The run reached the workspace clippy line and stopped there. The workspace nextest before it was green
>   (`1204 tests run: 1204 passed, 0 skipped`) and so were the doctests; neither feature-gated clippy line ran, because
>   the log ends at the workspace line's failure (`set -euo pipefail`, `agent-run.sh:20`).
> - Clippy's printed error lines, verbatim:
>
> ```
> error: length comparison to zero
>    --> crates/conductor-emit/src/identity.rs:179:5
>     = note: `-D clippy::len-zero` implied by `-D warnings`
> error: could not compile `conductor-emit` (lib) due to 1 previous error
> error: items after a test module
>    --> crates/conductor-emit/src/identity.rs:54:1
>     = note: `-D clippy::items-after-test-module` implied by `-D warnings`
> error: could not compile `conductor-emit` (lib test) due to 2 previous errors
> ```
>
>   The planted `len_zero` failed the lib target. The lib-test target ALSO raised `items_after_test_module`, because
>   the plant was appended after the file's `mod tests`; the plan predicted `len_zero` alone. Both are clippy lints that
>   rustc does not raise, so the red is still clippy's alone. The nextest and doctest lines compiled the same file
>   green before clippy ran.
> - Restoration: `identity.rs`'s sha256 was identical before and after (`ae7d2940…52f5`); the planted-lint guard
>   (`grep -c planted_lint crates/conductor-emit/src/identity.rs`) reads 0.
>
> ## 2. CI's dogfood step — RECORDED witness: a native non-zero fails the step under CI's preference
>
> CI#34689135760, sha `e3ff4e5f` (2026-09-12, conclusion `failure`), re-read at /implement with
> `gh run view 34689135760 --log-failed`. The `rust` job's "Test + lint (dogfood agent-run)" step (`shell: pwsh`)
> sets `$PSNativeCommandUseErrorActionPreference = $true` before `.\scripts\agent-run.ps1 run` (`.github/workflows/ci.yml`).
> The step died with:
>
> ```
> NativeCommandExitException: <runner path>\scripts\agent-run.ps1:280
> Program "cargo.exe" ended with non-zero exit code: 100
> ##[error]Process completed with exit code 1.
> ```
>
> (the runner's checkout path is elided). This was a **nextest** red (exit 100), not a clippy red. No CI run has ever
> carried a clippy-only red (research.md §Files inspected: every other failure in the 40-run listing is the a11y job, or
> the supply-chain step at `30156520449`), so CI's failure on a CLIPPY line is inferred from this mechanism, not
> observed.
>
> ## 3. `scripts/agent-run.ps1` — FIXED in the script; the red path is UNMEASURED on this host
>
> Before this chunk, the `''` arm checked no `$LASTEXITCODE` after its cargo lines, and the script never sets
> `$PSNativeCommandUseErrorActionPreference`, so a red cargo line ended the bundled default non-zero ONLY when the CALLER
> had set that preference (as CI does, §2). After this chunk the arm carries `if ($LASTEXITCODE -ne 0) { exit
> $LASTEXITCODE }` immediately after EACH of its five cargo lines (workspace nextest · doctest · workspace clippy · the
> two feature-gated clippy lines), the file's own idiom (`agent-run.ps1:212`). So the bundled default stops at the first
> non-zero cargo line, with that line's exit, whoever calls it.
>
> **That red path is UNMEASURED.** This Linux dev host has no `pwsh` (`command -v pwsh powershell` → nothing), and CI's
> dogfood run proves only the GREEN path through the new checks. The claim rests on reading the script, not on a run.
>
> ## 4. The `--e2e` arm — no clippy line, so a lint warning cannot fail the `a11y` job
>
> The `--e2e` arm runs no clippy line in either shell (`agent-run.sh:303-316` and `agent-run.ps1:351-370`: 0 `clippy`
> occurrences each). It builds the release binary and runs the wdio leg, so a lint warning cannot fail the `a11y` job,
> whose entry point it is.
>
> ## Summary for the obs §10 line
>
> A `-D warnings` lint fails `agent-run run`'s bundled default:
> - in `sh`, measured: exit 101 at the workspace clippy line, by `set -euo pipefail`;
> - in `ps1`, by the per-line `$LASTEXITCODE` check this chunk adds. The red path is unmeasured on the Linux host.
>
> It also fails CI's dogfood step, under the step's own `$PSNativeCommandUseErrorActionPreference`. The recorded witness
> CI#34689135760 is a nextest red, not a clippy red. The `--e2e` arm runs no clippy line.

## Deviations from intent
1. **Move spans start at the doc comments, not at the plan's cited item lines** — the plan cited `:57-342` / `:571-731`; the doc comments above the first items begin at `:54` / `:568`. Step 2's "and every doc comment above each" governs; the moved spans are `:54-341`, `:568-730`, `:843-951`, `:953-1080`.
2. **Moved test bodies are whitespace-changed, not byte-verbatim** — dedented 4 spaces (they left `mod tests`), and rustfmt re-wrapped one statement in `p025_hard_grade.rs` (`let err = grade_in_window(…).expect_err(…);` onto one line). The fmt gate (`cargo fmt --all --check`) requires both. The normalized line-multiset comparison shows no other difference.
3. **The split was written by a python line-range script** reading `git show dab66dc:…`, not by Edit/Write — byte-exact by construction; the script's writes bypass the PostToolUse rustfmt hook, so `rustfmt --edition 2024` ran by hand on the one re-wrapped file.
4. **`pub(crate)` also on struct fields** (`DelegatedBound`'s four, `HueSample`'s two) — the root's tests read `.budget_ms` / `.duration_ms`; the plan named "HueSample (with its fields)".
5. **tokei root 310 vs the ≈490 forecast** — the forecast scaled from tokei's base reading, which counts the base as code nearly whole (0 comments / 12 blanks over 1222 lines).
6. **CARRY-sh raised two lints, not one** — beside the planted `len_zero` (lib target), the lib-test target also raised `items_after_test_module` because the plant was appended after `mod tests`. Both are clippy-only; the red is still clippy's alone; the plan predicted `len_zero` only.
7. **The plan cited `agent-run.sh:297-316` for the `--e2e` arm** — the arm is `:303-316` (`:297-302` are `--unit` / `--integration`, which also run no clippy).
8. **Operator pass**: gate 23's first hygiene run refused `.andromeda/runs/2026-10-04T12-48-50-phase/.p5-dryrun.txt` (the phase run's raw P5 dry-run capture: one temp-dir log path, two home-dir skills paths); nothing cites it, so it was rewritten in place (4 substitutions) and the re-run read `clean`. The first `git commit` failed on a transient `.git/index.lock`; no lock and no git process remained on inspection, and the re-run landed `4c1e21a`.

scope record: none — `gate.py scope` clean, 0 recorded (changed 8 · listed 8 · excluded 52).

## Decisions & corrections
- Overseer (founder-delegated) at P4: kills proven by inverse controls only — no targeted cargo-mutants re-run (test-plan §9's epoch-boundary-only ruling stays literal); ps1 fixed in the script with the file's idiom, red path stated unmeasured.
- Overseer directives at /implement: anchor diff-shaped probes to the chunk base `dab66dc`, not HEAD (W182), excluding the chunk's own new files; stop this repo's rust-analyzer flycheck before heavy cargo steps (none was running at any check); no pulse-app; restore every inverse-control mutation byte-identically (sha256-verified after each control); stop before the operator pass.
- Overseer word for the operator pass (2026-10-04): "Run the operator pass now: gate 23 hygiene and the pre-CI commit, gate 24 push, gate 25 CI read" — the commit and push are the operator's acts made on that word.
- Sweep hazard: tokei 14.0.0 reads `delegated_timing_harvest.rs@dab66dc` as 1130 code / 0 comments / 12 blanks over 1222 raw lines — its base reading is not a line count of the file, so a size forecast scaled from it over-shoots (here 490 forecast vs 310 measured). Count the split by the same instrument AND compare content by a line multiset before reading a drop as lost content.
- Sweep hazard: `gh run view --log` output for a run with a ~49 MB a11y job stopped mid-leg (no wdio summary present); the full job log came from `gh api --allow-escape-sequences repos/{repo}/actions/jobs/{id}/logs` (without `--allow-escape-sequences` the API call refuses with "the response contains terminal escape sequences").
- Sweep hazard: a cross-use grep for the helper `synthetic` matched the string literal `"synthetic"` passed to `check_digest` in a root-staying test — a name that doubles as a literal needs its hits read.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) split behaviour-preserving under both runners — MET: `36 tests run: 36 passed, 0 skipped` and `test result: ok. 36 passed; 0 failed`.
- (tests) leaf-name diff against the base's `#[test]` names empty — MET (gate: no output, exit 0).
- (obs) the 14 cited tests listed under `delegated_timing_harvest tests::…` — MET (gate: last line 14).
- (arch) every file of the target ≤ 800 tokei code lines — MET (`over_800 0`); root 310 recorded against the ≈490 forecast (deviation 5).
- (security) no committed evidence file and no `evidence_pin` helper moves; no `CONDUCTOR_*`/`ANDROMEDA_*` read and no new SUT-output literal in the split target — MET (gate exit 0; the moved Pulse lines are byte-identical by the line multiset).
- (tests) the five survivors killed by one-shot inverse controls, each at nextest exit 100, restore guard `last line 3`, both library suites green — MET (`evidence/inverse-controls.md`).
- (tests) emit lib 138, run lib 52 under both runners — MET.
- (arch) no production function body changed; deterministic canary and `dispatch_wire__*` goldens untouched — MET (restore guard; workspace nextest green inside `agent-run.sh run`).
- (obs) the new tests add no span name, attribute or log field — MET (the diff adds none).
- (tests) CARRY sh half measured: exit 101 printing `length comparison to zero`, planted-lint guard 0 — MET (plus `items_after_test_module`, deviation 6).
- (tests) ps1 `''` arm carries exactly five checks, no ps1 line removed, sh / ci.yml byte-unchanged, ps1 red path stated unmeasured — MET.
- (tests) `bash scripts/agent-run.sh run` green — MET (1204 passed, doctests, three clippy lines).
- (security) zero dependency delta, 562 packages, audit + deny exit 0 — MET (`cargo audit` exit 0, 7 allowed warnings; `cargo deny` advisories/bans/licenses/sources ok; advisory-db residue probe 0 porcelain lines).
- (security) no absolute host path in the report or evidence — MET (python scan of the evidence for `/home/` · `/Users/` · `/root/` · `/tmp/` · `/var/tmp/` · drive-letter: 0 matches per file; `gate.py hygiene` clean).
- (a11y) the pushed HEAD's CI reads `verdict: green` — MET: CI#37209452847 on `4c1e21a`.

Gates (by `run`, /implement's full run — 22 green, 0 red, 3 operator legs):
- advisory-db residue self-heal — green · `cargo fmt --all --check` — green · `cargo nextest run -p conductor-run --test delegated_timing_harvest --profile ci` — green (`contains 36 tests run: 36 passed, 0 skipped`) · leaf-name `diff <(git show dab66dc…)` — green (no output) · cited-tests `grep -cE … tests::(p075_round_assertion_|…)` — green (last line 14) · `tokei -o json … over_800` — green (last line `over_800 0`) · `cargo test -p conductor-run --test delegated_timing_harvest` — green · evidence/`evidence_pin` `git diff --quiet dab66dc` — green · `cargo nextest run -p conductor-emit --lib --profile ci` — green (138) · `cargo nextest run -p conductor-run --lib --profile ci` — green (52) · restore guard python count — green (last line 3) · `cargo test -p conductor-emit --lib` — green (138) · `cargo test -p conductor-run --lib` — green (52) · ps1 `''`-arm check count — green (last line 5) · ps1 removed-line count — green (exit 1, last line 0) · `git diff --quiet dab66dc -- scripts/agent-run.sh .github/workflows/ci.yml` — green · `grep -c planted_lint …` — green (exit 1, last line 0) · `bash scripts/agent-run.sh run` — green · manifests/lock `git diff --quiet dab66dc` — green · package count — green (562) · `cargo audit` — green · `cargo deny check advisories bans licenses sources` — green.
- `leg = 'operator'`: `gate.py hygiene` — `hygiene: clean` after one rewrite (deviation 8) · guarded `git push origin HEAD` — `PUSHED_SHA=4c1e21a35e6b63f4f4dccaddc4ed931ea91e238a` · `ci.py conclusion --sha HEAD --wait 1500` — `verdict: green`, CI#37209452847. Recorded in `evidence/operator-pass.md`.
- Smoke: skipped — no boot-path change (`canary.rs` edited only in its inline test module; the ps1 `run` arm is unrunnable on this host; the bundled `run` verb ran green as a gate).
- No `defer` entry; no deferral.

Watches: none.

Outcome basis: /implement's P4 report (this conversation) + the operator pass's final state — commit `4c1e21a` (`git log dab66dc..HEAD`: the one pre-CI commit) and CI#37209452847 on that HEAD (`evidence/operator-pass.md`).

Process hygiene: /implement's census — every cargo / nextest / clippy process the gates, the ICs and CARRY-sh started: terminated (measured from the host process list). rust-analyzer (pid 532775) predates the run, left running — the editor's; it had no flycheck cargo child at any check. The `andromeda-pulse` session's nextest was not touched. The operator pass started only `git` and `ci.py` (both exited).
