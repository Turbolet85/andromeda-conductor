
## 2026-10-04-second-test-surface-corrective — a clippy lint red is blocking
**Section:** §10 SLO Invariants & Telemetry Budgets → Build / deploy failure conditions; §9 Pipeline integration → Lint / typecheck row
**Change:** was "`cargo clippy` warnings treated as CI annotations (non-blocking at Minimal, but visible to agent)"; now a `cargo clippy … -- -D warnings` lint red is BLOCKING at Minimal:
- any lint warning fails `agent-run run`'s bundled default, which stops at its first non-zero cargo line in both shells — `.sh` by `set -euo pipefail` (as measured: a planted clippy-only lint ended the run at exit 101 on the workspace clippy line, nextest and doctest green), `.ps1` by a `$LASTEXITCODE` check after each of its five cargo lines (red path unmeasured — no `pwsh` on the Linux dev host);
- so it fails the `rust` job's dogfood step, where the step's own `$PSNativeCommandUseErrorActionPreference` turns the red line into a terminating error (recorded witness CI#34689135760, a nextest red; no clippy-only red has reached CI);
- the `--e2e` arm runs no clippy line, so a lint never fails the `a11y` job.
The §9 row's clippy consumer was "CI annotations"; it is now the bundled default's exit (and with it the dogfood step), the stderr agent-readable from the job log. The fmt clause is unchanged.
**Why:** the CARRY from the previous wrap, measured this chunk: the old line described annotations, while the bundle runs three `-D warnings` clippy lines whose red ends the run. The earlier wrap rejected this amendment because its basis was harness source its report did not carry; this chunk's report carries the measurement verbatim (`evidence/carry-measurement.md`).
**Ref:** .andromeda/runs/2026-10-04T14-44-02-wrap/
