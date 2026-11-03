# Apply — test-plan · part 2 of 2

## Verdicts
- fanout-test-plan.md 4 · applied · E1
- fanout-test-plan.md 5 · applied · E2
- fanout-test-plan.md 6 · applied · E3
- fanout-test-plan.md 7 · applied · E4
- fanout-test-plan.md 8 · applied · E5
- fanout-test-plan.md 9 · applied · E6
- fanout-test-plan.md 10 · applied · E7
- fanout-test-plan.md 11 · applied · E8

## Edits
@fact id=fct-4txlbtvf expect_rev=1
old: and never run them; identical in both shells, and CI reaches them through the `rust` job's dogfood step. The bundled default stops at its FIRST non-zero cargo line in BOTH shells: `agent-run.sh` by `set -euo pipefail` (as measured
new: and never run them; CI reaches them through the `rust` job's dogfood step. The bundled default stops at its FIRST non-zero cargo line: `agent-run.sh` by `set -euo pipefail` (as measured
old: after green nextest and doctest lines), `agent-run.ps1` by an explicit `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` after each of its five cargo lines (since 2026-10-04), so it no longer depends on its caller setting `$PSNativeCommandUseErrorActionPreference` — and the script itself never sets it. A caller that DOES set it (CI's dogfood step) still sees the red line throw before the check runs (a `NativeCommandExitException`, the step exit 1 — CI#34689135760); a caller that does not gets that line's own cargo exit. The ps1 green path is proven by CI#37209452847; its red path is read from the script, not measured — no leg has driven it (the Linux dev host has carried `pwsh` 7.6.6 since 2026-10-06, as measured at the `2026-10-08-version-close-on-measured-evidence` chunk's `evidence/version-close.md`, where the script was parsed and never executed; a missing shell is no longer the reason). Scenario invocation:
new: after green nextest and doctest lines); CI's dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e`), so a red line ends the step. Scenario invocation:
old: (`SCENARIO=<name|P-ID> [SEED=<n>] scripts/agent-run.{sh,ps1} run`)
new: (`SCENARIO=<name|P-ID> [SEED=<n>] scripts/agent-run.sh run`)
old: (arch files-over-env-defaults precedence; identical `.sh`/`.ps1` semantics)
new: (arch files-over-env-defaults precedence)

@fact id=fct-vwyevac2 expect_rev=1
old: Identical semantics in `.sh` and `.ps1` — the short-circuit
new: In `agent-run.sh`, the one harness, the short-circuit
old: (measured 2026-09-04, both shells exit 0 and emit `ReadyState` JSON
new: (measured 2026-09-04, `agent-run.sh` exits 0 and emits `ReadyState` JSON
old: the 2026-09-03 both-shells-exit-1 witness
new: the 2026-09-03 exit-1 witness

@fact id=fct-bslufraa expect_rev=1
old: measured in the full shipped shell SET at
new: measured in `scripts/agent-run.sh` at
old: report.md`: `scripts/agent-run.sh` 2026-09-04T17:07:17→17:08:04Z and `scripts/agent-run.ps1` 17:11:16→17:12:03Z, both exit 0 with
new: report.md`: 2026-09-04T17:07:17→17:08:04Z, exit 0 with

@fact id=fct-fuxx7zeo expect_rev=1
old: The *wall-clock wrapper* `scripts/agent-run.{sh,ps1}` `boot` applies around `conductor preflight`: both shells derive it per invocation from the same contract
new: The *wall-clock wrapper* `scripts/agent-run.sh` `boot` applies around `conductor preflight`, derived per invocation from the same contract

@fact id=fct-oinj3vis expect_rev=1
old: (a sibling `case` branch on `run`, so the 5-command discipline holds and both shells ship it at identical semantics)
new: (a sibling `case` branch of `agent-run.sh`'s `run`, so the 5-command discipline holds)
old: the ordered leg set the shells' own `live_leg_order` owns
new: the ordered leg set the harness's own `live_leg_order` owns
old: (read the order from `live_leg_order` in `scripts/agent-run.{sh,ps1}`, never
new: (read the order from `live_leg_order` in `scripts/agent-run.sh`, never
old: and exits 2 before any probe, in both shells;
new: and exits 2 before any probe;
old: unknown-token `usage:` exit 2 in both shells.
new: unknown-token `usage:` exit 2.

@fact id=fct-oo4n5l6h expect_rev=1
old: Both shells select that line by the envelope-only `seed` key
new: `agent-run.sh` selects that line by the envelope-only `seed` key
old: `has("seed")` · `Where-Object { $_.PSObject.Properties.Name -contains 'seed' }`).
new: `has("seed")`).

@fact id=fct-lpebvclu expect_rev=1
old: `rm -f runs/<run_id>.jsonl runs/<run_id>.md` in the shells, then
new: `rm -f runs/<run_id>.jsonl runs/<run_id>.md` in `agent-run.sh`, then
old: **the shells issue no SQL**
new: **the harness script issues no SQL**

@fact id=fct-uxjgyjwg expect_rev=1
old: wire `scripts/agent-run.{sh,ps1}` with the 5 commands
new: wire `scripts/agent-run.sh` with the 5 commands
old: per 5-command implementation above. Binding contract — both shell variants must expose identical semantics, a property asserted of the SCRIPTS **together with the invoking environment**: a caller that preempts native-command exits (`$PSNativeCommandUseErrorActionPreference = $true`) diverges the `.ps1` leg with zero script delta, measured in CI 2026-09-07 (Windows/Linux dev-OS targets).
new: per 5-command implementation above.

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'identical in both shells'
ignore_case = true

[[pattern]]
id = "tp2-02"
fixed = 'FIRST non-zero cargo line in BOTH shells'
ignore_case = true

[[pattern]]
id = "tp2-03"
fixed = 'no longer depends on its caller setting'
ignore_case = true

[[pattern]]
id = "tp2-04"
fixed = 'ps1 green path is proven by'
ignore_case = true

[[pattern]]
id = "tp2-05"
fixed = 'identical `.sh`/`.ps1` semantics'
ignore_case = true

[[pattern]]
id = "tp2-06"
fixed = 'Identical semantics in `.sh` and `.ps1`'
ignore_case = true

[[pattern]]
id = "tp2-07"
fixed = 'measured in the full shipped shell SET'
ignore_case = true

[[pattern]]
id = "tp2-08"
fixed = 'both shells derive it per invocation'
ignore_case = true

[[pattern]]
id = "tp2-09"
fixed = 'both shells ship it at identical semantics'
ignore_case = true

[[pattern]]
id = "tp2-10"
regex = 'exits? 2[^.;]{0,40}in both shells'
ignore_case = true

[[pattern]]
id = "tp2-11"
fixed = 'Both shells select that line'
ignore_case = true

[[pattern]]
id = "tp2-12"
fixed = 'the shells issue no SQL'
ignore_case = true

[[pattern]]
id = "tp2-13"
fixed = 'both shell variants must expose identical semantics'
ignore_case = true

[[pattern]]
id = "tp2-14"
fixed = 'diverges the `.ps1` leg'
ignore_case = true
