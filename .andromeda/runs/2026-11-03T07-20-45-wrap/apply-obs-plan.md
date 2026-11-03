# Apply — obs-plan

## Verdicts
- fanout-obs-plan.md 1 · applied · E1
- fanout-obs-plan.md 2 · applied · E2
- fanout-obs-plan.md 3 · applied · E3
- fanout-obs-plan.md 4 · applied · E4
- fanout-obs-plan.md 5 · applied · E5

## Edits
@fact id=fct-wj6wca64 expect_rev=1
old: which stops at its first non-zero cargo line in both shells — `agent-run.sh` by `set -euo pipefail`
new: which stops at its first non-zero cargo line — `scripts/agent-run.sh`, the one harness, by `set -euo pipefail`
old: its nextest and doctest lines green), `agent-run.ps1` by a `$LASTEXITCODE` check after each of its five cargo lines (its red path unmeasured — no leg has driven it; the Linux dev host has carried `pwsh` 7.6.6 since 2026-10-06, as measured at the `2026-10-08-version-close-on-measured-evidence` chunk's `evidence/version-close.md`, where the script was parsed and never executed). So it fails the `rust` job's dogfood step, where the step's own `$PSNativeCommandUseErrorActionPreference` turns the non-zero cargo line into a terminating error (recorded witness CI#34689135760, a nextest red; no clippy-only red has reached CI).
new: its nextest and doctest lines green). So it fails the `rust` job's dogfood step on `ubuntu-latest`, which runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e {0}`): the script's `set -euo pipefail` under the step's `bash -e` is the step's red path (no clippy-only red has reached CI).

@fact id=fct-r77hb5fq expect_rev=1
old: **The suppression is SHIPPED and test-held (2026-09-04):**
new: **The suppression is SHIPPED (2026-09-04):**
old: (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`);
new: (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`) that no standing leg compiles — the one CI job runs on `ubuntu-latest`, where the `#[cfg(windows)]` arms are compiled out;
old: so no standing leg evidences it now
new: so no standing leg evidences the flag or its effect now

@fact id=fct-tia6ntih expect_rev=1
old: tokio 1.52.3
new: tokio 1.54.1

@fact id=fct-lqqdem2q expect_rev=1
old: tokio 1.52.3
new: tokio 1.54.1

@fact id=fct-amqff6tu expect_rev=1
old: `verification-matrix.json#v2-15`
new: `cap:conductor-0.2.0/v2-15`

## Patterns
[[pattern]]
id = "obs-01"
regex = 'first non-zero cargo line in both shells|by a `\$LASTEXITCODE` check after each|PSNativeCommandUseErrorActionPreference` turns'

[[pattern]]
id = "obs-02"
fixed = 'SHIPPED and test-held'

[[pattern]]
id = "obs-03"
fixed = 'tokio 1.52.3 `current_thread` flavor'

[[pattern]]
id = "obs-04"
fixed = 'tokio 1.52.3 `current_thread` runtime'

[[pattern]]
id = "obs-05"
fixed = 'verification-matrix.json#v2-15'
