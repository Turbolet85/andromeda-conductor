## 2026-11-03-linux-only-base-ci — tokio 1.54.1 at the test plan's two sites
**Section:** §1 Test Scope Summary (the `conductor-timeline` coverage entity, its arch §Stack quote); §8 Mocking & Stubbing Discipline (the Time mocking row)
**Change:**
- The `conductor-timeline` entity quotes arch §Stack as "tokio 1.54.1 `current_thread` flavor" (was tokio 1.52.3).
- The Time mocking row reads tokio 1.54.1 `test-util` (was 1.52.3); `start_paused = true` + `tokio::time::advance(Duration)` unchanged.
**Why:** the report's Dependencies: the lock moved tokio 1.52.3 → 1.54.1, lock-only under the unchanged workspace pin, closing RUSTSEC-2026-0386 (`unsound`, `AsyncFd::new`), which `cargo deny` denied on the first Linux CI run — external decay on an unchanged tree. The bump was taken inside this chunk on the overseer's word (delegate overseer, founder-delegated), PROVISIONAL (inputs#I9).
**Kept:** the version stays a named literal at both sites; dropping it for a lock-resolved wording was weighed and not taken, the report stating the moved version.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the Windows creation-flag value test compiled out on every leg
**Section:** §1 Test Scope Summary (Untestable at the unit tier: the APPLIED Windows creation flag on the sidecar spawn)
**Change:**
- The flag VALUE test `spawn.rs::the_sidecar_spawn_suppresses_the_child_console` is `#[cfg(windows)]` and compiles out on every standing leg — the Linux dev host and the Linux CI job — so no run witnesses the value (was: the value is unit-covered).
- The `#[cfg(windows)]` `spawn.rs` arms' owner is named: the route entry «Spawned sidecar and shared-machine gate retired».
**Why:** the report's Retired bullet (the Windows CI runner) and its Harness bullet: no leg witnesses the `#[cfg(windows)]` `spawn.rs` arms. The owner is the P5 card's Beyond-the-plan row, answered `yes` by the overseer (delegate overseer, founder-delegated), PROVISIONAL on the forks the operator owns (inputs#I5).
**Kept:** the APPLICATION half as it stood — evidenced by the operator-local `sr` leg until it retired with the window (2026-10-11), no standing leg evidencing it now, never by a unit assertion or a manual smoke step, and no Windows window-enumeration driver invented.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the first-interrupt test runs on the Linux CI leg
**Section:** §1 Test Scope Summary (Untestable: `conductor suite`'s Ctrl-C stop at the signal itself)
**Change:**
- The zone reads "Untestable (in-process)" (was "Untestable (in CI, and in-process)").
- The `#[cfg(unix)]` `the_first_interrupt_sets_the_stop_flag_and_the_process_lives` runs on the Linux dev host and on the Linux CI leg, CI's one `rust` job, as measured at CI#41595128664 (was: on the Linux dev host only until `Linux-only base CI` makes CI Linux).
**Why:** the report's Harness bullet: the test ran green on the first Linux CI leg, discharging the CI half the zone held pending on this entry.
**Kept:** the second Ctrl-C's `std::process::exit(130)` stays unwitnessed in-process with no owner, both accepted by the operator's word (2026-10-12); the stop's outcome stays unit-covered (`suite_exit`'s two tests, `drive_run`'s three).
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the harness named as `agent-run.sh` alone
**Section:** §1 Test Scope Summary (harness requirements: `run`); §3 Test Harness Contract (the setup-project note); §4 Unit Test Strategy (Mutation instrument); §6 E2E Test Strategy (the steps of the error-baseline-spike, Fingerprint-storm, Restart-suppression, Severity-lifecycle and Known-residual P-032 scenarios); §9 CI Integration (the Unit and Integration stage rows; Live-Pulse scenarios)
**Change:**
- Every `scripts/agent-run.{sh,ps1}` / `agent-run.{sh,ps1}` reads `scripts/agent-run.sh` / `agent-run.sh`: setup-project materializes `scripts/agent-run.sh`; the mutation gate adds no sixth `agent-run.sh` command; each scenario's preflight is reached via `scripts/agent-run.sh boot`, and Severity-lifecycle's five family scenarios run as `SCENARIO=<name> scripts/agent-run.sh run`; the stage rows read `agent-run.sh run --unit` and `agent-run.sh run --integration`.
- §1 `run`: a red build, doctest or `-D warnings` clippy line stops the run at the first such line under `agent-run.sh`'s `set -euo pipefail` (was "in both shells").
- §9 Live-Pulse: since 2026-10-04 both owed feature-gated lint lines run in `agent-run.sh run`'s bundled default (was `agent-run run`'s bundled default "in both shells").
**Why:** the report's Retired bullet: the PowerShell harness twin `scripts/agent-run.ps1` retires, and `agent-run.sh`, byte-unchanged, is the one harness (harness scripts 2 → 1).
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the capture evidence at `crates/conductor-run/tests/evidence/{marker}/`
**Section:** §7 Test Data & Fixtures (the sanctioned provenance set: committed live-leg capture evidence)
**Change:**
- The real-model harvest reads committed live-leg capture EVIDENCE under `crates/conductor-run/tests/evidence/{marker}/`, one folder per source chunk holding byte-equal copies of its captures, the exception at security-plan §Security Anti-Patterns → Data Protection (was: under a chunk's `evidence/` tree).
- A capture is evidence held byte-equal under its pin, never a fixture (was: evidence, never a fixture copied elsewhere); still read-only, `CARGO_MANIFEST_DIR`-anchored and checked against its sha256 digest pin before any grade reads it.
**Why:** the report's Files: 46 captures copied byte-equal into 11 per-source-chunk folders, file names and sha256 pins unchanged, the old `conductor-0.3.0/chunks/` tree removed. The move is the founder's ratification, his own live word by dialog in pc-overseer's session, relayed verbatim (inputs#I3): the Data Protection exception extends to `crates/conductor-run/tests/evidence/` and test-plan §7 is amended to match; the overseer (founder-delegated) confirmed this amendment lies inside it, PROVISIONAL as a confirm.
**Kept:** the ban on a developer-seeded `runs.db`, and the committed fixture family as it stood.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — CI's one job on `ubuntu-latest`
**Section:** §9 CI Integration (Matrix builds: the OS bullet)
**Change:**
- The shipped CI arrangement is `rust` alone, on `ubuntu-latest`, its dogfood step running `bash scripts/agent-run.sh run` under `shell: bash` (was `rust` alone, on `windows-latest`).
**Why:** the report's Retired and Harness bullets: the Windows runner (`runs-on: windows-latest`) retires with the pwsh dogfood step; no step added or removed, every other step byte-unchanged.
**Kept:** the job count 1 and no OS matrix; the per-variant nextest `job_name` advice should a matrix arrive.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the coverage regex's `ci.yml` lines re-pointed
**Section:** §10 Quality Gates & Coverage Targets (the shipped exclusion regex)
**Change:**
- `--ignore-filename-regex '[\\/]tests[\\/]'` cites `.github/workflows/ci.yml:141`/`:142`/`:145` (was `:143`/`:144`/`:147`); the regex, its 2026-09-13 measurement and the `--fail-under-lines 60` gate unchanged.
**Why:** this chunk's edit of `ci.yml` moved the cited lines; the wrap's citation sweep found the regex still on each re-pointed line.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the 5-command contract stated for one shell
**Section:** §3 → 5-command implementation (`boot` leading arm and command body, Timeout (b), `run` command body, CI stage selectors, `status`, `cleanup`); §3 → Bootstrap phases (5-command-discipline-wire)
**Change:**
- `boot`: the short-circuit is stated of `agent-run.sh`, the one harness (was identical `.sh`/`.ps1` semantics); the 2026-09-04 reading keeps `agent-run.sh` alone (the `agent-run.ps1` run dropped); Timeout (b)'s wrapper is `scripts/agent-run.sh` `boot`'s (was both shells).
- `run`: the bundled default stops at its FIRST non-zero cargo line by `agent-run.sh`'s `set -euo pipefail` (the 2026-10-04 measurement kept); CI's dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e`), so a red line ends the step. Retired: "identical in both shells", the `agent-run.ps1` `$LASTEXITCODE` per-line exit, the `$PSNativeCommandUseErrorActionPreference` caller clause, the ps1 green and unmeasured red paths with the `pwsh` host note; the scenario leg reads `scripts/agent-run.sh run`.
- Stage selectors: `--live` is a sibling `case` branch of `agent-run.sh`'s `run`, its leg order read from `live_leg_order` in `scripts/agent-run.sh`; the unknown-selector and `--e2e` `usage:` exit 2 lose "in both shells".
- `status`: `agent-run.sh` selects the newest envelope by `has("seed")` (the `Where-Object` form retired). `cleanup`: `rm -f` in `agent-run.sh`; the harness script issues no SQL.
- 5-command-discipline-wire wires `scripts/agent-run.sh`; the two-shell identical-semantics binding and its 2026-09-07 `.ps1` divergence measurement retired.
**Why:** the report's Retired bullet: `scripts/agent-run.ps1` with its `$LASTEXITCODE` checks, and the pwsh dogfood step with its native-error preference; its Counts: the dogfood red path → `set -euo pipefail` under `bash -e`. One shell owes no cross-variant identity.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
