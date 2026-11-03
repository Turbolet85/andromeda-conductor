# Apply — architecture

## Verdicts
- fanout-architecture.md 1 · applied · E1
- fanout-architecture.md 2 · applied · E2
- fanout-architecture.md 3 · applied · E3
- fanout-architecture.md 6 · applied · E4
- fanout-architecture.md 7 · applied · E5
- fanout-architecture.md 8 · applied · E6
- fanout-architecture.md 9 · applied · E7
- fanout-architecture.md 10 · applied · E8
- fanout-architecture.md 11 · applied · E9
- fanout-architecture.md 12 · applied · E10
- fanout-architecture.md 13 · applied · E11
- fanout-architecture.md 15 · applied · E12
- fanout-architecture.md 16 · refused · a citation's digits move by the cascade sweep alone — `rec put` refuses the in-line re-point of fct-4rt3oej6 (`Cargo.toml:154` → `:153`) as `digit moved`; the sweep carries it
- fanout-architecture.md 4 · applied · E13
- fanout-architecture.md 5 · applied · E14
- fanout-architecture.md 14 · applied · E15

## Edits
@fact id=fct-fjnaazvl expect_rev=1
old: | tokio 1.52.3 (`current_thread` flavor) |
new: | tokio 1.54.1 (`current_thread` flavor) |

@fact id=fct-h4z4u2a2 expect_rev=1
old: Rust 2024 + tokio 1.52.3:**
new: Rust 2024 + tokio 1.54.1:**

@fact id=fct-jiaouq3r expect_rev=1
old: + tokio 1.52.3 `current_thread`.
new: + tokio 1.54.1 `current_thread`.

@fact id=fct-f6qqnofr expect_rev=1
old: nextest / clippy on the dev OS target verify
new: nextest / clippy on the job's `ubuntu-latest` runner verify

@fact id=fct-h4tkqxtv expect_rev=1
old: exported by `scripts/agent-run.{sh,ps1}`;
new: exported by `scripts/agent-run.sh`;

@fact id=fct-gv4qlmgx expect_rev=1
old: read by `scripts/agent-run.{sh,ps1}` `boot` wrapping
new: read by `scripts/agent-run.sh` `boot` wrapping
old: (~47s wall-clock in both shipped shells, as measured at
new: (~47s wall-clock, as measured at

@fact id=fct-pjwesp6u expect_rev=1
old: the `run --live` banner in both `scripts/agent-run.{sh,ps1}`.
new: the `run --live` banner in `scripts/agent-run.sh`.

@fact id=fct-t5yqommt expect_rev=1
old: arm of both harness shells REMOVES it
new: arm of `scripts/agent-run.sh` REMOVES it
old: never a test binary, and the `.ps1` READS and restores the caller's value in a `finally`, because a dot-invoked script shares the process environment. Conductor
new: never a test binary. Conductor

@fact id=fct-ror5pd2n expect_rev=1
old: Harness-owned by `scripts/agent-run.{sh,ps1}` with NO
new: Harness-owned by `scripts/agent-run.sh` with NO

@fact id=fct-22zuwdf4 expect_rev=1
old: a sixth `agent-run.{sh,ps1}` command
new: a sixth `agent-run.sh` command

@fact id=fct-apqmcxio expect_rev=1
old: and the harness shells issue no SQL of their own.
new: and the harness script `scripts/agent-run.sh` issues no SQL of its own.

@fact id=fct-ou2ozyvp expect_rev=1
old: scrubbed real-model capture evidence in a chunk's `evidence/` tree, security-plan
new: scrubbed real-model capture evidence committed under `crates/conductor-run/tests/evidence/{marker}/`, security-plan

@fact id=fct-wjtvroma expect_rev=1
old: `core.autocrlf=true` is the norm on the Windows dev host and the one CI job, `rust`, runs on a `windows-latest` runner (
new: `core.autocrlf=true` is the norm on a Windows checkout, while the one CI job, `rust`, runs on an `ubuntu-latest` runner (

@fact id=fct-4ega3gmt expect_rev=1
old: GitHub Actions on the dev OS target runs one job
new: GitHub Actions on an `ubuntu-latest` runner runs one job

@fact id=fct-uhih6b6b expect_rev=1
old: invoked by no CI step, wired into neither harness shell, no 6th command
new: invoked by no CI step, not wired into `agent-run.sh`, no 6th command

## Patterns
[[pattern]]
id = "arch-01"
regex = 'tokio\s+v?1\.52\.3'

[[pattern]]
id = "arch-02"
fixed = 'on the dev OS target'
ignore_case = true

[[pattern]]
id = "arch-03"
fixed = 'agent-run.{sh,ps1}'

[[pattern]]
id = "arch-04"
fixed = 'in both shipped shells'
ignore_case = true

[[pattern]]
id = "arch-05"
fixed = 'arm of both harness shells REMOVES'
ignore_case = true

[[pattern]]
id = "arch-06"
fixed = 'READS and restores the caller'
ignore_case = true

[[pattern]]
id = "arch-07"
fixed = 'the harness shells issue no SQL'
ignore_case = true

[[pattern]]
id = "arch-08"
fixed = "capture evidence in a chunk's `evidence/` tree"
ignore_case = true

[[pattern]]
id = "arch-09"
fixed = 'runs on a `windows-latest` runner'
ignore_case = true

[[pattern]]
id = "arch-10"
fixed = 'wired into neither harness shell'
ignore_case = true
