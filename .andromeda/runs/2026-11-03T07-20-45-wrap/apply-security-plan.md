# Apply — security-plan

## Verdicts
- fanout-security-plan.md 1 · applied · E1
- fanout-security-plan.md 2 · applied · E2
- fanout-security-plan.md 3 · applied · E3
- fanout-security-plan.md 4 · folded into fanout-security-plan.md 1
- fanout-security-plan.md 5 · applied · E4
- fanout-security-plan.md 6 · applied · E5
- fanout-security-plan.md 7 · applied · E6
- fanout-security-plan.md 8 · applied · E7

## Edits
@fact id=fct-sbjbvw42 expect_rev=1
old: held by a `#[cfg(windows)]` unit test (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`), so
new: with its `#[cfg(windows)]` unit test (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`), so
old: so the flag VALUE is unit-held and its APPLICATION was evidenced by the operator-local SR leg until that leg retired with the window (2026-10-11; test-plan §1 untestable zones).
new: so the flag VALUE is unit-testable only on a Windows build and its APPLICATION was evidenced by the operator-local SR leg until that leg retired with the window (2026-10-11; test-plan §1 untestable zones). No measured leg — the Linux dev host and the `ubuntu-latest` CI job — compiles the `#[cfg(windows)]` arm or its test, so none witnesses the flag value or its application; the arm's disposition is owned by the route entry «Spawned sidecar and shared-machine gate retired».
old: on the ROUTINE `scripts/agent-run.{sh,ps1}` path
new: on the ROUTINE `scripts/agent-run.sh` path
old: where the shells invoke the repo-built
new: where the harness invokes the repo-built
old: admitted only after each shell's own run-id rejection (`valid_run_id` / `Test-RunId`)
new: admitted only after its own run-id rejection (`valid_run_id`)

@fact id=fct-orv7vnhf expect_rev=1
old: additionally has readers OUTSIDE `conductor-core`:**
new: additionally has a reader OUTSIDE `conductor-core`:**
old: `scripts/agent-run.{sh,ps1}` parse `warmup_ms` + `min_canary_poll_seconds` to derive the `boot` wall-clock budget, and carry the same
new: `scripts/agent-run.sh` parses `warmup_ms` + `min_canary_poll_seconds` to derive the `boot` wall-clock budget, and carries the same

@fact id=fct-fy2a4ady expect_rev=1
old: selector is a closed `case` allowlist — the `.ps1` binds the token by position count — and an unknown token
new: selector is a closed `case` allowlist in `scripts/agent-run.sh`, and an unknown token
old: the harness shells additionally reject it upstream (`valid_run_id` / `Test-RunId`) before the spawn
new: the harness additionally rejects it upstream (`valid_run_id`) before the spawn

@fact id=fct-gympmcsi expect_rev=1
old: `scripts/agent-run.{sh,ps1}` run `conductor preconditions` as a leading arm
new: `scripts/agent-run.sh` runs `conductor preconditions` as a leading arm

@fact id=fct-uxxrxviq expect_rev=1
old: under `conductor-0.3.0/chunks/*/evidence/`, workspace-root anchored
new: under `crates/conductor-run/tests/evidence/*/` (one folder per source chunk), workspace-root anchored

@fact id=fct-ppdqreyb expect_rev=1
old: may enter a chunk's committed `evidence/` tree, and only through
new: may enter a chunk's committed `evidence/` tree and its committed copy under `crates/conductor-run/tests/evidence/{marker}/`, and only through
old: outside the `evidence/` tree this exception names
new: outside both `evidence/` trees this exception names
old: an elided copy under `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/`,
new: an elided copy at `crates/conductor-run/tests/evidence/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/rm-capture-2026-09-22-elided.txt`,

@fact id=fct-okeb3fjj expect_rev=1
old: committed evidence records (the per-chunk `evidence/` tree)
new: committed evidence records (a chunk's `evidence/` tree and the committed capture copies under `crates/conductor-run/tests/evidence/`)

## Patterns
[[pattern]]
id = "sec-01"
fixed = "so the flag VALUE is unit-held"

[[pattern]]
id = "sec-02"
regex = "each shell's own run-id rejection|where the shells invoke"
ignore_case = true

[[pattern]]
id = "sec-03"
fixed = 'agent-run.{sh,ps1}` parse `warmup_ms`'

[[pattern]]
id = "sec-04"
regex = 'binds the token by position count|harness shells additionally reject'
ignore_case = true

[[pattern]]
id = "sec-05"
fixed = 'agent-run.{sh,ps1}` run `conductor preconditions`'

[[pattern]]
id = "sec-06"
fixed = 'rm-capture*.txt` under `conductor-0.3.0/chunks/*/evidence/`'

[[pattern]]
id = "sec-07"
fixed = 'an elided copy under `conductor-0.3.0/chunks/'

[[pattern]]
id = "sec-08"
fixed = 'committed evidence records (the per-chunk `evidence/` tree)'
