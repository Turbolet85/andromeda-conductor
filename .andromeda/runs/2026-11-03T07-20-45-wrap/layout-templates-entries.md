## 2026-11-03-linux-only-base-ci — one harness script, the PowerShell twin retired
**Section:** §Surface: cli → Primary screens (commands) (the `conductor cleanup` bullet, the harness bullet)
**Change:**
- The `conductor cleanup` bullet names one invoker: was "the mechanism `scripts/agent-run.{sh,ps1} cleanup` now invokes behind its existing run-id guard — the shells issue no SQL of their own"; now `scripts/agent-run.sh cleanup` invokes it behind its existing run-id guard (`valid_run_id`), and the script issues no SQL of its own.
- The harness bullet heads `scripts/agent-run.sh`, "the one harness script — test-plan §3": was `scripts/agent-run.{sh,ps1}` with both shells shipping at identical semantics (test-plan §3), a parity the bullet called a property of the scripts together with the invoking environment (a caller preempting native-command exits diverged the `.ps1` leg with zero script delta, measured 2026-09-07) — that parity clause retired.
- The two "in both shells" qualifiers retired: the unknown `--live` selector prints `usage: <script> run --live [real-model]` and exits 2 before any probe; the retired `--e2e` token falls to the unknown-token `usage:` exit 2.
- Unchanged: the five verbs `boot`/`run`/`status`/`cleanup`/`logs`, the `--unit`/`--integration`/`--live` stage flags and the bundled default, the `--live` and `--live real-model` precondition refusals at exit 1, the TTY gating.
**Why:** the report retires the PowerShell harness twin `scripts/agent-run.ps1` (with `Test-RunId`) and makes `scripts/agent-run.sh` the one harness, byte-unchanged, `valid_run_id` ahead of `conductor cleanup` and the closed `--live [real-model]` refusal kept; harness scripts 2 → 1.
**Kept:** the dated 2026-09-04 measured `boot` runs of both scripts and the earlier chunks' evidence citations beside them stand as history — the record of where a measurement stood, not a standing parity claim.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
