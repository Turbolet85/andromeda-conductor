# Apply — test-plan · part 1 of 2

## Verdicts
- fanout-test-plan.md 1 · applied · E1
- fanout-test-plan.md 2 · applied · E2
- fanout-test-plan.md 3 · applied · E3
- fanout-test-plan.md 12 · applied · E4
- fanout-test-plan.md 13 · applied · E5
- fanout-test-plan.md 14 · applied · E6
- fanout-test-plan.md 15 · applied · E7
- fanout-test-plan.md 16 · applied · E8
- fanout-test-plan.md 17 · applied · E9
- fanout-test-plan.md 18 · applied · E10
- fanout-test-plan.md 19 · applied · E11
- fanout-test-plan.md 20 · applied · E12
- fanout-test-plan.md 21 · applied · E13
- fanout-test-plan.md 22 · applied · E14
- fanout-test-plan.md 23 · applied · E15
- fanout-test-plan.md 24 · applied · E16
- fanout-test-plan.md 25 · applied · E17

## Edits
@fact id=fct-esm7gxmw expect_rev=1
old: **Untestable (in CI, and in-process):**
new: **Untestable (in-process):**
old: which runs on the Linux dev host only until `Linux-only base CI` makes CI Linux;
new: which runs on the Linux dev host and on the Linux CI leg, CI's one `rust` job (as measured at CI#41595128664);

@fact id=fct-m6asvzjt expect_rev=1
old: today `rust` alone, on `windows-latest`:
new: today `rust` alone, on `ubuntu-latest` (its dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash`):

@fact id=fct-7j7mxml4 expect_rev=1
old: The flag VALUE is unit-covered (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`, `#[cfg(windows)]`) and its APPLICATION was evidenced by
new: The flag VALUE test (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`) is `#[cfg(windows)]` and compiles out on every standing leg (the Linux dev host and the Linux CI job), so no run witnesses the value; its APPLICATION was evidenced by
old: to observe a boolean.
new: to observe a boolean. The `#[cfg(windows)]` `spawn.rs` arms' owner: the route entry «Spawned sidecar and shared-machine gate retired».

@fact id=fct-jd3veh5r expect_rev=1
old: first such line in both shells (§3
new: first such line under `agent-run.sh`'s `set -euo pipefail` (§3

@fact id=fct-iqllmv72 expect_rev=1
old: scripts/agent-run.{sh,ps1}
new: scripts/agent-run.sh

@fact id=fct-pzb6nngw expect_rev=1
old: no sixth `agent-run.{sh,ps1}` command
new: no sixth `agent-run.sh` command

@fact id=fct-fikilkyv expect_rev=1
old: `scripts/agent-run.{sh,ps1} boot`
new: `scripts/agent-run.sh boot`

@fact id=fct-btwjiwiz expect_rev=1
old: `scripts/agent-run.{sh,ps1} boot`
new: `scripts/agent-run.sh boot`

@fact id=fct-php6ab3n expect_rev=1
old: `scripts/agent-run.{sh,ps1} boot`
new: `scripts/agent-run.sh boot`

@fact id=fct-cmep33bg expect_rev=1
old: `scripts/agent-run.{sh,ps1} boot`
new: `scripts/agent-run.sh boot`
old: scripts/agent-run.{sh,ps1} run`
new: scripts/agent-run.sh run`

@fact id=fct-imnnp6ty expect_rev=1
old: `scripts/agent-run.{sh,ps1} boot`
new: `scripts/agent-run.sh boot`

@fact id=fct-ep2ez4nz expect_rev=1
old: `agent-run.{sh,ps1} run --unit`
new: `agent-run.sh run --unit`

@fact id=fct-cfw4k5hz expect_rev=1
old: `agent-run.{sh,ps1} run --integration`
new: `agent-run.sh run --integration`

@fact id=fct-yroyo3j2 expect_rev=1
old: run in `agent-run run`'s bundled default in both shells,
new: run in `agent-run.sh run`'s bundled default,

@fact id=fct-odwlbdq3 expect_rev=1
old: tokio 1.52.3 `test-util`
new: tokio 1.54.1 `test-util`

@fact id=fct-efo3vjjh expect_rev=1
old: tokio 1.52.3 `current_thread` flavor
new: tokio 1.54.1 `current_thread` flavor

@fact id=fct-goccvdfd expect_rev=1
old: under a chunk's `evidence/` tree —
new: under `crates/conductor-run/tests/evidence/{marker}/`, one folder per source chunk holding byte-equal copies of its captures (the exception at security-plan §Security Anti-Patterns → Data Protection) —
old: a capture is evidence, never a fixture copied elsewhere.
new: a capture is evidence held byte-equal under its pin, never a fixture.

## Patterns
[[pattern]]
id = "tp1-01"
fixed = "dev host only until"

[[pattern]]
id = "tp1-02"
fixed = "`rust` alone, on `windows-latest`"

[[pattern]]
id = "tp1-03"
fixed = "The flag VALUE is unit-covered"

[[pattern]]
id = "tp1-04"
fixed = "first such line in both shells"

[[pattern]]
id = "tp1-05"
fixed = "materialize scripts/agent-run.{sh,ps1}"

[[pattern]]
id = "tp1-06"
fixed = "sixth `agent-run.{sh,ps1}` command"

[[pattern]]
id = "tp1-07"
fixed = "agent-run.{sh,ps1} boot"

[[pattern]]
id = "tp1-08"
fixed = "agent-run.{sh,ps1} boot"

[[pattern]]
id = "tp1-09"
fixed = "agent-run.{sh,ps1} boot"

[[pattern]]
id = "tp1-10"
fixed = "agent-run.{sh,ps1} run"

[[pattern]]
id = "tp1-11"
fixed = "agent-run.{sh,ps1} boot"

[[pattern]]
id = "tp1-12"
fixed = "agent-run.{sh,ps1} run --unit"

[[pattern]]
id = "tp1-13"
fixed = "agent-run.{sh,ps1} run --integration"

[[pattern]]
id = "tp1-14"
fixed = "bundled default in both shells"

[[pattern]]
id = "tp1-15"
fixed = "tokio 1.52.3"

[[pattern]]
id = "tp1-16"
fixed = "1.52.3 `current_thread` flavor"

[[pattern]]
id = "tp1-17"
regex = "under a chunk's .evidence/. tree|never a fixture copied elsewhere"
ignore_case = true
