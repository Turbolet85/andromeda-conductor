# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · E1
- fanout-layout-templates.md 2 · applied · E2

## Edits
@fact id=fct-4artks5b expect_rev=1
old: **`scripts/agent-run.{sh,ps1}`** (both shells ship at identical semantics — test-plan §3; a property of the SCRIPTS together with the invoking environment, since a caller preempting native-command exits diverges the `.ps1` leg with zero script delta — measured 2026-09-07) — the headless
new: **`scripts/agent-run.sh`** (the one harness script — test-plan §3) — the headless
old: exits 2 before any probe, in both shells — a selector
new: exits 2 before any probe — a selector
old: unknown-token `usage:` exit 2 in both shells.
new: unknown-token `usage:` exit 2.

@fact id=fct-lxai4faj expect_rev=1
old: `scripts/agent-run.{sh,ps1} cleanup` now invokes behind its existing run-id guard — the shells issue no SQL of their own.
new: `scripts/agent-run.sh cleanup` now invokes behind its existing run-id guard (`valid_run_id`) — the script issues no SQL of its own.

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'both shells ship at identical semantics'
ignore_case = true

[[pattern]]
id = "lt-02"
fixed = 'in both shells'
ignore_case = true

[[pattern]]
id = "lt-03"
fixed = 'the shells issue no SQL of their own'
ignore_case = true
