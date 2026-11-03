## 2026-11-03-linux-only-base-ci — the tokio runtime version in the scope summary
**Section:** §1 Obs Scope Summary (the conductor-timeline row; the tokio `current_thread` runtime row)
**Change:**
- The conductor-timeline row's Stack cell reads `tokio 1.54.1 `current_thread` flavor` (was `tokio 1.52.3`).
- The runtime row reads `tokio 1.54.1 `current_thread` runtime` (was `tokio 1.52.3`); its determinism invariant (no background batch tasks in exporters) unchanged.
**Why:** the report's Dependencies: tokio bumped `1.52.3` → `1.54.1`, lock-only under the workspace pin `tokio = "1.48"`, no other package moved, closing RUSTSEC-2026-0386 (`unsound`, `AsyncFd::new`), an external decay on an unchanged tree that `cargo deny` denied on the Linux CI leg. The bump was taken inside the chunk on the overseer's word (founder-delegated, PROVISIONAL), inputs#I9.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the port-occupier driver's capability address
**Section:** Fault-injection spans (chaos-instrumentation trigger, obs-scope §5) — where each span opens
**Change:**
- The `fault.port_occupier` driver is addressed as `cap:conductor-0.2.0/v2-15` (was `verification-matrix.json#v2-15`); the span's placement, parent `timeline.execute` and phase-boundary close are unchanged.
**Why:** the report retires the spine pair `conductor-0.{2,3,4}.0/{requirements.md,verification-matrix.json}` from the tree (the store is their one form); its spine-pair sweep names this site as a capability address in a retired file, the capability standing at `cap:conductor-0.2.0/v2-15`.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the clippy-blocking red path on one harness
**Section:** §10 SLO Invariants & Telemetry Budgets (Build / deploy failure conditions — the `cargo clippy … -- -D warnings` bullet)
**Change:**
- `agent-run run`'s bundled default stops at its first non-zero cargo line in `scripts/agent-run.sh`, the one harness, by `set -euo pipefail` (was "in both shells", `agent-run.sh` and `agent-run.ps1`); the planted clippy-only lint measurement (exit 101 on the workspace clippy line) stands.
- The `agent-run.ps1` `$LASTEXITCODE` clause, its unmeasured red path and the `pwsh` 7.6.6 dev-host aside leave.
- The `rust` job's dogfood step runs on `ubuntu-latest` as `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e {0}`); the script's `set -euo pipefail` under the step's `bash -e` is the step's red path (was the step's `$PSNativeCommandUseErrorActionPreference` turning the non-zero line into a terminating error, with its recorded nextest-red witness). No clippy-only red has reached CI, unchanged.
**Why:** the report's Retired: the Windows CI runner and the pwsh dogfood step with its `$PSNativeCommandUseErrorActionPreference` line, and the PowerShell twin `scripts/agent-run.ps1` with its `$LASTEXITCODE` per-line checks; its Harness / gate surface moves the one job `rust` to `ubuntu-latest`, the dogfood step running `bash scripts/agent-run.sh run` under `shell: bash`, with `agent-run.sh` byte-unchanged.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the console-suppression test no leg compiles
**Section:** §11 Obs Anti-Patterns → Project-specific bans (the MCP-sidecar spawn ban)
**Change:**
- The suppression reads SHIPPED (2026-09-04) (was "SHIPPED and test-held").
- The `#[cfg(windows)]` unit test pinning the `CREATE_NO_WINDOW` flag value (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`) is one no standing leg compiles: the one CI job runs on `ubuntu-latest`, where the `#[cfg(windows)]` arms are compiled out.
- No standing leg evidences the flag or its effect now (was "evidences it"); the applied flag stays not assertable at the unit tier, and the operator-local SR leg retired with the window on 2026-10-11.
**Why:** the report's Retired names the Windows CI runner, and its Harness / gate surface reads that no leg witnesses the `#[cfg(windows)]` `spawn.rs` arms (compiled out); the test held only while a Windows leg compiled it.
**Kept:** the ban itself, `build_command`'s `CREATE_NO_WINDOW` through `console_suppressing_flags()` and the unit test in source stand; the `#[cfg(windows)]` spawn arms go with the existing entry «Spawned sidecar and shared-machine gate retired», per the P5 card's Beyond-the-plan row (the overseer's word, founder-delegated, PROVISIONAL), inputs#I5.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
