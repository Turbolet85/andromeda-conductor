group: test-plan
read: .claude/docs/tests-summary.md · 50 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 97 · «- 2026-10-12: **`tokio::signal::ctrl_c()` installs its OS ha»
read: .claude/rules/verification-harness.md · 78 · «- 2026-09-16: **A surviving grandchild holding an inherited »

- leaf: .claude/docs/tests-summary.md:10   class: stale
  leaf-text: «setup-project Phase 4 materializes `scripts/agent-run.{sh,ps1}`.»
  source: fct-iqllmv72 «setup-project to materialize scripts/agent-run.sh]_»
  replace-with: «setup-project Phase 4 materializes `scripts/agent-run.sh`.»

- leaf: .claude/docs/tests-summary.md:13   class: retired
  leaf-text: «stops at its FIRST non-zero cargo line in both shells — `.sh` by `set -euo pipefail` (measured: exit 101 on a clippy-only red), `.ps1` by a per-line `$LASTEXITCODE` check (since 2026-10-04, red path unmeasured) — so a lint warning fails `run` whoever calls it.»
  source: fct-4txlbtvf «The bundled default stops at its FIRST non-zero cargo line: `agent-run.sh` by `set -euo pipefail`»
  replace-with: «stops at its FIRST non-zero cargo line — `agent-run.sh` by `set -euo pipefail` (measured: exit 101 on a clippy-only red), and CI's dogfood step runs it under `shell: bash` — so a lint warning fails `run` whoever calls it.»

- leaf: .claude/rules/verification-harness.md:17   class: retired
  leaf-text: «## 5-command discipline (`scripts/agent-run.{sh,ps1}` — exactly 5, identical semantics across .sh and .ps1: a property of the SCRIPTS together with the invoking environment, since a caller preempting native-command exits diverges the `.ps1` leg with zero script delta — measured 2026-09-07)»
  source: fct-uxjgyjwg «wire `scripts/agent-run.sh` with the 5 commands (boot=precondition probe + preflight gate / run / status / cleanup / logs) per 5-command implementation above.»
  replace-with: «## 5-command discipline (`scripts/agent-run.sh` — exactly 5)»

- leaf: .claude/rules/verification-harness.md:18   class: retired
  leaf-text: «(measured 2026-09-04 in both shells — `agent-run.sh` 17:07:17→17:08:04Z, `agent-run.ps1` 17:11:16→17:12:03Z, both exit 0 with `ready:true`, zero `skipped preflight` lines)»
  source: fct-bslufraa «measured in `scripts/agent-run.sh` at `conductor-0.2.0/chunks/2026-09-04-preconditions-probe-reads-path-handles-by-presence/report.md`: 2026-09-04T17:07:17→17:08:04Z, exit 0 with `ready: true`»
  replace-with: «(measured 2026-09-04 in `agent-run.sh` — 17:07:17→17:08:04Z, exit 0 with `ready:true`, zero `skipped preflight` lines)»

- leaf: .claude/rules/verification-harness.md:18   class: retired
  leaf-text: «Identical semantics in `.sh` and `.ps1`; the command count stays five.»
  source: fct-vwyevac2 «In `agent-run.sh`, the one harness, the short-circuit is environment-CONDITIONAL, firing only on an actually-unmet subject»
  replace-with: «`agent-run.sh` is the one harness; the command count stays five.»

- leaf: .claude/rules/verification-harness.md:18   class: stale
  leaf-text: «the wall-clock wrapper both shells apply around the CLI»
  source: fct-fuxx7zeo «(b) The *wall-clock wrapper* `scripts/agent-run.sh` `boot` applies around `conductor preflight`, derived per invocation from the same contract as `warmup_ms/1000 + poll + margin`»
  replace-with: «the wall-clock wrapper `agent-run.sh` `boot` applies around the CLI»

- leaf: .claude/rules/verification-harness.md:19   class: retired
  leaf-text: «which stops at its FIRST non-zero cargo line in both shells — `.sh` by `set -euo pipefail` (measured 2026-10-04: exit 101 on a clippy-only red), `.ps1` by a `$LASTEXITCODE` check after each cargo line (since 2026-10-04; red path unmeasured — no leg has driven it, though the Linux dev host has carried `pwsh` since 2026-10-06), independent of whether the caller sets `$PSNativeCommandUseErrorActionPreference` (a caller that does sees the line throw first, step exit 1).»
  source: fct-4txlbtvf «CI's dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e`), so a red line ends the step.»
  replace-with: «which stops at its FIRST non-zero cargo line — `agent-run.sh` by `set -euo pipefail` (measured 2026-10-04: exit 101 on a clippy-only red); CI's dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e`), so a red line ends the step.»

- leaf: .claude/rules/verification-harness.md:19   class: stale
  leaf-text: «the ordered leg set `live_leg_order` owns in both shells»
  source: fct-oinj3vis «the ordered leg set the harness's own `live_leg_order` owns»
  replace-with: «the ordered leg set `live_leg_order` owns in `agent-run.sh`»

- leaf: .claude/rules/verification-harness.md:19   class: stale
  leaf-text: «an unknown selector exits 2 in both shells.»
  source: fct-oinj3vis «An unknown selector prints `usage: <script> run --live [real-model]` and exits 2 before any probe; `boot` keeps the flagless probe.»
  replace-with: «an unknown selector exits 2 before any probe.»

- leaf: .claude/rules/verification-harness.md:20   class: stale
  leaf-text: «Both shells select it by the envelope-only `seed` key.»
  source: fct-oo4n5l6h «`agent-run.sh` selects that line by the envelope-only `seed` key (`jq -s` filtering `has("seed")`).»
  replace-with: «`agent-run.sh` selects it by the envelope-only `seed` key (`jq -s` filtering `has("seed")`).»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «`rm -f runs/<run_id>.{jsonl,md}` in the shells, then `conductor cleanup <run_id>` — **the shells issue no SQL** (2026-09-06).»
  source: fct-lpebvclu «`rm -f runs/<run_id>.jsonl runs/<run_id>.md` in `agent-run.sh`, then `conductor cleanup <run_id>` — **the harness script issues no SQL** (2026-09-06).»
  replace-with: «`rm -f runs/<run_id>.{jsonl,md}` in `agent-run.sh`, then `conductor cleanup <run_id>` — **the harness script issues no SQL** (2026-09-06).»
