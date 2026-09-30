# obs extract

## Relevance
partial — the chunk adds no new traced operation. Obs binds through the gates it re-runs: the self-obs log conformance, zero-unlogged-panics, the a11y violation-record conformance and the live-drive path's self-obs and envelope records. It also binds through two CARRY fixes that touch obs-governed surfaces: C3's refusal path in `conductor-cli`, and C4's comment beside the §9 workflow hygiene gates.

## Constraints
- The live-drive leg's self-obs stream (`logs/agent-latest.jsonl` under agent mode) must carry the §3 base set on every line: `timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment` and `run_id`. It must hold no absolute host path in any field (per obs-plan §3 Log format JSON schema "Two record shapes"; §9 Log conformance check). Whether the current tree still satisfies this is for the regression run to measure.
- The two record shapes stay distinct. The scenario-result envelope in `runs/<run_id>.jsonl` is graded on the eleven envelope keys by key PRESENCE; a null admitted on blocked rows. The per-check `CheckRecord` lines are a separate nine-key grain. The self-obs line is graded on the §3 base set, never on the envelope (per obs-plan §3 Log format JSON schema; §6 Required fields).
- The a11y routine arm's violation record at `runs/a11y/<run_id>.jsonl` is an admitted SUPERSET: the eleven envelope keys plus the §9 resource tags. It is 13 keys locally and 15 under CI, where `ci.run.id` and `git.commit.sha` join. Its conformance is asserted in-job by `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y` (per obs-plan §3 Log format JSON schema; §9 Telemetry artifact handling).
- `RUST_LOG` per-target directives REPLACE the default. Never set a bare `{crate}=debug`, and never set either form for a run that also executes the test suite: the environment reaches the runner's child processes and fails the CLI agent-mode self-obs test (per obs-plan §6 Per-module log levels).
- `conductor-cli` declares no `tracing` dependency, so C3's refusal of an ambiguous P-ID surfaces as the CLI's printed or `anyhow`-edge output, not as a `tracing` event. Adding a `tracing` dep to `conductor-cli` would change the crate's §6 row, and whether C3's fix needs one is research's question (per obs-plan §6 Per-module log levels; §6 Boundary-call wrappers, DB DELETE bullet).
- Output from the gate diagnostics this chunk re-runs is agent-readable in the job log only. That covers fmt `Diff in` lines, the secret-scan and workflow env-context hits and the `GITHUB_ENV context probe (assert)` verdict. None of it is ever written into a telemetry artifact such as `logs/agent-latest.jsonl` or `runs/**` (per obs-plan §9 Pipeline integration).
- Build/deploy fails on any unlogged panic, a log-conformance violation, a red `cargo-audit` or a red repository-hygiene gate. These are the obs-owned members of "every gate green" (per obs-plan §10 Build / deploy failure conditions).

## Patterns to follow
- The Tauri backend sink moves with `CONDUCTOR_RUNS_DIR`, one directory per suite. The routine `--e2e` and `sr-empty` legs use `runs/logs/conductor-tauri.jsonl`, the driven arm `runs/driven/logs/` and `sr`/`sr-error` `runs/sr-leg/logs/`. A regression diagnosis reads the suite's own sink (per obs-plan §3 Log file location).
- A `runs.db` READ (`conductor_run::read_envelope` over `RunsDb::get_envelope`, reached by the `run_envelope()` command that C5's `run_envelope` row question touches) logs one `info!` with `run_id` and the outcome on the allowlisted `message` field, inside the caller's `tauri.command.*` span. It mints no `db.*` read span (per obs-plan §6 Boundary-call wrappers, DB READ bullet).
- Tauri command handlers are instrumented by a manual `tracing::info_span!("tauri.command.<name>").entered()` guard, never `#[tracing::instrument]`. A probe grepping for the attribute as a proxy for instrumentation will always return 0 (per obs-plan §1 Telemetry surfaces, desktop-webview row).
- C1's rustdoc de-linking in `conductor-emit` is documentation-only. It must leave the `emit.batch` wire-shape witness unchanged: it stays at `debug`, inside the existing span, on the allowlisted `message` field (per obs-plan §6 Boundary-call wrappers, gRPC emit bullet).

## Anti-patterns to avoid
- NEVER add a retry-once or rerun-until-green policy to make a gate green under either runner. It masks real failures, which parallels the zero-flakiness bar (per obs-plan §11 SLO).
- NEVER write the unstructured `^thread.*panicked` panic form. Only formatted `tracing::error!(panic=…)` JSON events are admissible, and a multi-line backtrace must be serialized to one field (per obs-plan §9 Zero-unlogged-panics gate; §11 Logs).
- NEVER leak absolute host paths or internal struct names into logs, the run report or `runs.db`. C3's new refusal message is covered too: use `Display` not `Debug` at the `anyhow` edge, with no resolved scenario path in the text (per obs-plan §11 Logs; §11 Error Reporting).

## Contract bindings
- obs ↔ tests: the §9 log conformance and zero-unlogged-panics gates read the self-obs format test-plan §3 owns, and the envelope and `CheckRecord` shapes are also test-plan §3's (per obs-plan §3 Log format JSON schema; §9).
- obs ↔ a11y: the a11y routine arm emits its violation record in the envelope shape plus resource tags, and conformance is held by `journal_conformance` (per obs-plan §3 Log format JSON schema; §9 Telemetry artifact handling).
- obs ↔ security: this binding is the host-path scrub and field allowlist in `conductor-core::redact`. The warning that the three named application sites do not bound every host-path channel still stands, and the SR leg's `<host-path>` scrub is its second line of defence (per obs-plan §11 PII Scrubbing).
- obs ↔ CI (`ci.yml`, C4): the `Workflow env-context gate` and the `GITHUB_ENV context probe (assert)` are obs-registered hygiene gates. C4's reworded comment must agree with the probe's measured mechanism, which is that a `GITHUB_ENV`-written key does resolve (per obs-plan §9 Pipeline integration, Repository-hygiene gates row; §10).

## Acceptance criteria contributions
- (obs) For the live-drive leg: every line of `logs/agent-latest.jsonl` carries the seven §3 base-set keys, and no field matches the host-path set (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`). The allowlisted `target` module path is exempt (per obs-plan §9 Log conformance check).
- (obs) Zero lines matching `^thread.*panicked` across `logs/agent-latest.jsonl` and the captured stderr of every regression run (per obs-plan §9 Zero-unlogged-panics gate; §10 Always-required SLO invariant).
- (obs) The a11y routine arm's `runs/a11y/<run_id>.jsonl` passes `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y`, and the record is uploaded as a CI artifact (per obs-plan §9 Telemetry artifact handling).
- (obs) Every live-drive scenario-result record in `runs/<run_id>.jsonl` carries all eleven envelope keys by presence, with null admitted on blocked rows. Each non-null `latency_ms` is asserted ≤ its `slo_tier` threshold at report-generation time (per obs-plan §6 Required fields; §10 Performance budgets).
