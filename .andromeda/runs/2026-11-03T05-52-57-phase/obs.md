# obs extract

## Relevance
partial. The chunk adds no span, log line, metric or allowlist field. It moves the obs CI gates (the log conformance producer and gate, zero-unlogged-panics, artifact uploads) to a Linux runner, swaps the harness the clippy-blocking condition runs through, and decides the fate of the Windows-only spawn arm that the plan's §11 records.

## Constraints
- obs-plan §9 CI Integration (Log conformance check) requires a CI gate that reads `logs/agent-latest.jsonl` and fails the build on any line missing the §3 self-obs base-line schema (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`), or on an absolute host path in any field. Its path set (drive-letter `X:\`, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`) must not be narrowed by the Linux restatement. Whether the producer step behaves the same under `bash` as under `pwsh` is research's question.
- obs-plan §9 CI Integration (Zero-unlogged-panics gate) requires CI to grep `logs/agent-latest.jsonl` and stderr for `^thread.*panicked` and fail on any unstructured panic. The Linux job must still feed that gate the same two inputs.
- obs-plan §9 CI Integration (Telemetry artifact handling) requires `logs/agent-latest.jsonl` and the cargo-nextest JSON to be uploaded as CI artifacts from every CI job. The `\\/`-shaped ignore regex and the upload steps re-state for Linux without dropping either artifact.
- obs-plan §9 CI Integration (Pipeline integration, Repository-hygiene gates row) requires secret-scan / workflow env-context / `GITHUB_ENV` probe output, and the `cargo fmt` diff, to stay in the job log only and never be written into `logs/agent-latest.jsonl` or `runs/**`. A Linux runner's `/home/runner/…` paths make that discipline load-bearing.
- obs-plan §10 SLO Invariants & Telemetry Budgets (Build / deploy failure conditions) requires a `cargo clippy … -D warnings` red to be BLOCKING through `agent-run run`'s bundled default. Today the plan records the CI-side mechanism as the `pwsh` step's `$PSNativeCommandUseErrorActionPreference`. Once the step becomes `agent-run.sh run`, the stop has to come from the script's `set -euo pipefail` and the step's shell propagating the non-zero exit. Whether the Linux step does so is research's question.
- obs-plan §3 Logging stack (contract `doc:obs-plan/contracts/logging-stack`) requires agent mode to fire on `--agent-mode` OR the `CONDUCTOR_AGENT_MODE` env, which the harness exports and Conductor only reads. With `scripts/agent-run.ps1` removed, `scripts/agent-run.sh` must be the one harness that sets whatever the CI obs producer depends on. Whether it already exports what the `.ps1` exported is research's question.
- obs-plan §11 Obs Anti-Patterns (PII Scrubbing; Project-specific bans) records `CREATE_NO_WINDOW` under `#[cfg(windows)]` in `crates/conductor-verify/src/spawn.rs` as the closure of the console-pane host-path channel, and the `#[cfg(windows)]` unit test as its holder. Removing those arms (scope item 3, P4's call) changes a recorded obs closure, so the plan text moves through the wrap's amendments, never a phase edit.

## Patterns to follow
- Stop-at-first-red harness: `scripts/agent-run.sh` under `set -euo pipefail` ends the run at the first non-zero cargo line (per obs-plan §10 SLO Invariants & Telemetry Budgets, Build / deploy failure conditions).
- `RUST_LOG` pairing: a per-target directive replaces the default, so always pair it as `info,{crate}=debug`, and never set either form for a run that also executes the test suite. The Linux job's env must not introduce one (per obs-plan §6 Log Coverage, per-module log levels).
- The secret-scan gate's path-free skip line for a VCS-less copy, and repo-relative `path:line` hits that never echo the matched text, stay as they are under the new runner (per obs-plan §9 CI Integration, Pipeline integration).
- CI artifact wiring follows the bootstrap item `obs-ci-gate-wire`: cargo-nextest JSON plus a `logs/agent-latest.jsonl` upload (per obs-plan §3 Bootstrap phases, contract `doc:obs-plan/contracts/bootstrap-phases-for-downstream-skills`).

## Anti-patterns to avoid
- Losing telemetry artifacts, or skipping the zero-unlogged-panics gate, in the runner move (per obs-plan §11 Obs Anti-Patterns, CI).
- Leaking absolute host paths (`/home/runner/…`) into logs, run reports or `runs.db` through a Linux-restated step (per obs-plan §11 Obs Anti-Patterns, Logs).
- Adding retry-once or continue-on-error around a gate step to get the Linux job green (per obs-plan §11 Obs Anti-Patterns, SLO).

## Contract bindings
- obs ↔ tests harness: the CI log conformance gate validates the obs self-obs base-line schema, and the run-journal conformance gate validates the tests-owned eleven-field envelope (two record shapes). Both must survive the move intact (per obs-plan §9 CI Integration, Log conformance check; obs-plan §6 Log Coverage, Required fields).
- obs ↔ security: the `CREATE_NO_WINDOW` closure in obs-plan §11 Obs Anti-Patterns (PII Scrubbing) mirrors security.md's record of the shipped flag. Both records move together if scope item 3 removes the `#[cfg(windows)]` arms.
- obs ↔ wrap amendments: after this chunk, three plan passages go stale and are amended at the wrap, never here. They are obs-plan §10's clippy bullet (names `agent-run.ps1` and the `pwsh` preference), obs-plan §1 Obs Scope Summary's `multi-platform-exporter-compat` trigger row (Windows/macOS/Linux), and obs-plan §11's project-specific ban text on the `#[cfg(windows)]` test.

## Acceptance criteria contributions
- (obs) The Linux `rust` job in `.github/workflows/ci.yml` still runs the obs conformance producer and the conformance-and-zero-unlogged-panics gate, and uploads `logs/agent-latest.jsonl` and the nextest JSON as artifacts. No such step has been removed, set to `continue-on-error`, or given a retry (per obs-plan §9 CI Integration).
- (obs) The log conformance gate's absolute-path set still holds every pattern it held at base `8e450f3`; nothing is dropped as "Windows-only" (per obs-plan §9 CI Integration, Log conformance check).
- (obs) The step that replaces `agent-run.ps1 run` fails the job when `scripts/agent-run.sh run` exits non-zero: no `|| true`, no swallowed exit (per obs-plan §10 SLO Invariants & Telemetry Budgets, Build / deploy failure conditions).
- (obs) No hygiene-gate hit text and no fmt diff line reaches `logs/agent-latest.jsonl` or `runs/**` on the Linux job (per obs-plan §9 CI Integration, Pipeline integration).
