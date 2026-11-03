group: security-plan
read: .claude/docs/security-summary.md · 49 · «**Full plan:** `.andromeda/security-plan.md`. Path-scoped ru»
read: .claude/rules/security.md · 57 · «- 2026-10-11: A silent `cargo deny check licenses` is no evi»

- leaf: .claude/rules/security.md:16   class: stale
  leaf-text: «`pulse-run-contract.toml` additionally has readers OUTSIDE `conductor-core` — `scripts/agent-run.{sh,ps1}` parse `warmup_ms` + `min_canary_poll_seconds` for the `boot` budget and carry the same rule»
  source: fct-orv7vnhf «**`pulse-run-contract.toml` additionally has a reader OUTSIDE `conductor-core`:** `scripts/agent-run.sh` parses `warmup_ms` + `min_canary_poll_seconds` to derive the `boot` wall-clock budget»
  replace-with: «`pulse-run-contract.toml` additionally has a reader OUTSIDE `conductor-core` — `scripts/agent-run.sh` parses `warmup_ms` + `min_canary_poll_seconds` for the `boot` budget and carries the same rule»

- leaf: .claude/rules/security.md:27   class: stale
  leaf-text: «the routine `scripts/agent-run.{sh,ps1}` path's teardown `conductor cleanup <run_id>` behind `valid_run_id` / `Test-RunId`»
  source: fct-sbjbvw42 «**(b) any HARNESS spawn** on the ROUTINE `scripts/agent-run.sh` path — ONE governed form since 2026-10-11»
  replace-with: «the routine `scripts/agent-run.sh` path's teardown `conductor cleanup <run_id>` behind `valid_run_id`»

- leaf: .claude/rules/security.md:23   class: stale
  leaf-text: «held by a `#[cfg(windows)]` unit test»
  source: fct-sbjbvw42 «No measured leg — the Linux dev host and the `ubuntu-latest` CI job — compiles the `#[cfg(windows)]` arm or its test, so none witnesses the flag value or its application»
  replace-with: «with its `#[cfg(windows)]` unit test, which no measured leg (the Linux dev host, the `ubuntu-latest` CI job) compiles»

- leaf: .claude/rules/security.md:23   class: omission
  leaf-text: «no standing leg evidences it now»
  source: fct-sbjbvw42 «so none witnesses the flag value or its application; the arm's disposition is owned by the route entry «Spawned sidecar and shared-machine gate retired»»
  replace-with: «no measured leg evidences the flag value or its application now; the arm's disposition is owned by the route entry "Spawned sidecar and shared-machine gate retired"»

- leaf: .claude/rules/security.md:10   class: omission
  leaf-text: «may enter a chunk's committed `evidence/` tree»
  source: fct-ppdqreyb «may enter a chunk's committed `evidence/` tree and its committed copy under `crates/conductor-run/tests/evidence/{marker}/`, and only through `mask_workspace_key`»
  replace-with: «may enter a chunk's committed `evidence/` tree and its committed copy under `crates/conductor-run/tests/evidence/{marker}/`»

- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «committed evidence records (the per-chunk `evidence/` tree)»
  source: fct-okeb3fjj «committed evidence records (a chunk's `evidence/` tree and the committed capture copies under `crates/conductor-run/tests/evidence/`) leak absolute host paths»
  replace-with: «committed evidence records (a chunk's `evidence/` tree and the committed capture copies under `crates/conductor-run/tests/evidence/`)»
