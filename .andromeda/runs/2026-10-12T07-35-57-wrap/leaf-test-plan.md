group: test-plan
read: .claude/docs/commands.md · 48 · «- `cargo modules generate tree` / `cargo tree` — module/depe»
read: .claude/docs/tests-summary.md · 50 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 40 · «- 2026-06-26: Instrument a `#[tauri::command]` with a MANUAL»
read: .claude/rules/testing.md · 96 · «- 2026-10-08: **`cargo nextest run --profile ci` is fail-fas»
read: .claude/rules/verification-harness.md · 78 · «- 2026-09-16: **A surviving grandchild holding an inherited »
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/rules/observability.md:17   class: retired
  leaf-text: «; Tauri backend → `conductor-tauri.jsonl` under `runs_dir.parent()/logs/`, with no writer since 2026-10-11 (the sink goes at `Panel-shaped types retired`).»
  source: .andromeda/registries/contracts/test-plan/log-format.md:6 «the `tracing` self-observation log stream (the cli's: stderr, or `logs/agent-latest.jsonl` under `--agent-mode`) carries per-line base fields»
  replace-with: « — the cli's is the one self-obs stream.»

- leaf: .claude/docs/tests-summary.md:27   class: omission
  leaf-text: «the in-process core arm (`conductor_run::{preflight, drive_run}`) vs headless `conductor run`»
  source: .andromeda/test-plan.md:205 «an IN-PROCESS arm calling `conductor_run::{preflight, drive_run}` — the composition `conductor suite` drives in production (`drive_run`'s first production caller since 2026-10-12)»
  replace-with: «the in-process core arm (`conductor_run::{preflight, drive_run}` — the composition `conductor suite` drives in production, `drive_run`'s first production caller since 2026-10-12) vs headless `conductor run`»

- leaf: .claude/docs/commands.md:19   class: omission
  leaf-text: «`conductor suite --seed <s>` — drive the full catalog, over the SUT capability manifest's accepted set (preflight first).»
  source: .andromeda/test-plan.md:47 «the second Ctrl-C's immediate `std::process::exit(130)` cannot be asserted in-process»
  replace-with: «`conductor suite --seed <s>` — drive the full catalog, over the SUT capability manifest's accepted set (preflight first). Ctrl-C stops it: the first interrupt sets the stop flag and the process lives, a stopped suite exits 130, and a second Ctrl-C exits 130 at once; a completed suite exits on the hard-Fail rule.»

- leaf: .claude/rules/testing.md:42   class: stale
  leaf-text: «only a hard `Fail` exits non-zero.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:12 «non-zero = at least one hard `Fail` (cargo-nextest `NextestExitCode` 100), or — in the bundled default — a red build, doctest or `-D warnings` lint line, which ends the run at that line»
  replace-with: «a hard `Fail` exits non-zero (nextest 100), as does a red build, doctest or `-D warnings` lint line in `run`'s bundled default; a Ctrl-C-stopped `conductor suite` exits 130 (test-plan §1), a completed one on the hard-Fail rule.»

- leaf: .claude/rules/testing.md:32   class: omission
  leaf-text: «Self-bootstrapping via the seeded `conductor-timeline` generator — no developer-seeded DB;»
  source: .andromeda/test-plan.md:317 «Tests MUST seed their own data from the sanctioned provenance SET — the seeded `conductor-timeline` generator, rstest fixtures, and the **committed fixture family**»
  replace-with: «Self-bootstrapping from the sanctioned provenance SET — the seeded `conductor-timeline` generator, rstest fixtures and the committed fixture family (today `crates/conductor-core/tests/fixtures/` and `scripts/fixtures/mutation-gate/`, each carrying its MEANING under test through a round-trip in the production sweep or the gate's `selftest`) — no developer-seeded DB;»
