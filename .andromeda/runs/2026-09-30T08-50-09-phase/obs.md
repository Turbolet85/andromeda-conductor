# obs extract

## Relevance
partial — the chunk is webview keyboard/focus behaviour plus routine `--e2e` a11y assertions; obs binds only at the frontend telemetry posture, any Tauri command a shortcut reaches, the a11y violation record, and the per-suite backend log sink.

## Constraints
- The webview frontend's only permitted telemetry is browser console JSON (`service.name` = `conductor-ui`): obs-plan §3 Logging stack and §1 Telemetry surfaces (desktop-webview row) require no OTel JS SDK, no `auto-instrumentations-web` and no browser OTLP exporter. A keyboard or shortcut handler that logs anything must go through `console.log(JSON…)` alone.
- obs-plan §4 Auto-instrumentation per surface (desktop-webview row) requires every `#[tauri::command]` handler to open a MANUAL `tracing::info_span!("tauri.command.<name>").entered()` guard, never the `#[tracing::instrument]` attribute. This binds only if the chunk adds or changes a command. A start/stop/proceed/abort shortcut that reuses an existing command (`start_run`, `stop_run`, `resolve_operator_hold`) inherits that command's span. Whether the shortcuts need any backend change at all is research's question.
- The span-name set is bounded: obs-plan §11 Spans / Traces caps it at `tauri.command.*` on the GUI surface. Any new name must stay inside that family, and nothing keyboard-derived (key name, row index, scenario name) may appear in a span name.
- The a11y CI gate's violation record `runs/a11y/<run_id>.jsonl` is the envelope's one exercised extension point, per obs-plan §3 Log format JSON schema. It carries the eleven envelope keys plus the §9 resource tags: 13 keys locally, 15 under CI. It is asserted by presence and host-path freedom, not exclusivity. New routine-arm assertions that feed this record must keep that shape. Whether they write to it at all is research's question.
- Per obs-plan §3 Log file location, the routine `--e2e` suite's Tauri backend log is `runs/logs/conductor-tauri.jsonl`: the file name is fixed, and the directory follows `CONDUCTOR_RUNS_DIR`. Evidence that cites backend lines from the leg must cite that per-suite directory, not the project-root `logs/`.
- Per obs-plan §10 Always-required SLO invariant and §9 Zero-unlogged-panics gate, the PREREQ's workspace nextest and clippy runs, and every run over the chunk's delta, must leave no unstructured `^thread.*panicked` line in `logs/agent-latest.jsonl` or stderr.
- Per obs-plan §11 Logs and §9 Log conformance check, committed artifacts and logs from the leg must not contain an absolute host path. That covers evidence files, the violation record and the backend log.

## Patterns to follow
- A command handler opens `tracing::info_span!("tauri.command.<name>").entered()`. obs-plan §4 records this as measured at 8/8 sites. Research must confirm which of the eight commands the shortcuts dispatch to.
- Frontend telemetry uses the console JSON sink (`console.log(JSON.stringify(event))`) per obs-plan §3 Logging stack. There is no network egress.
- In the violation record, resource tags (`service.name` = `conductor-ui`, `deployment.environment`, plus `ci.run.id` and `git.commit.sha` under CI) sit beside the eleven envelope keys, per obs-plan §3 Log format JSON schema and §9 CI-specific resource attributes.
- A value outside the field allowlist rides the allowlisted `message` field instead of becoming a new attribute. obs-plan §6 Boundary-call wrappers uses this for key-set and incident-id witnesses, and it applies if a command gains an outcome to log.

## Anti-patterns to avoid
- Never add an OTel SDK, `tracing-opentelemetry`, a `traceparent` or any OTLP export for self-observation. That includes a browser-side SDK or exporter added to "observe" keyboard events, whether to `:4317` or `:4318` (obs-plan §11 Telemetry Strategy, Spans / Traces, Universal).
- Never put `#[tracing::instrument]` on a `#[tauri::command]`. It does not stack with the command macro (obs-plan §4 Auto-instrumentation per surface).
- Never put a high-cardinality or input-derived token (key code, row index, scenario name) in a span name (obs-plan §11 Spans / Traces).

## Contract bindings
- obs ↔ a11y: the routine `--e2e` arm's violation record at `runs/a11y/<run_id>.jsonl` uses the obs-plan §3 envelope-superset shape with the §9 resource tags. The four claims' new assertions land on this arm.
- obs ↔ tests: the violation record's conformance is asserted in-job by `conductor-run`'s `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y` (obs-plan §9 Telemetry artifact handling). That check covers key presence, closed sets and host-path freedom.
- obs ↔ security: the host-path hygiene of committed evidence and logs follows obs-plan §11 Logs and PII Scrubbing, the redaction layer in `conductor-core::redact` and the leg's own scrub.

## Acceptance criteria contributions
- (obs) The chunk adds no OTel or OTLP package, import or network telemetry call to `crates/conductor-tauri/ui`. Any logging a new keyboard or shortcut handler does is `console.log` JSON only. Pass/fail check: the frontend dependency and import diff against `7ee2fea` gains no `@opentelemetry/*` entry (per obs-plan §3 Logging stack; §11 Universal).
- (obs) Any `#[tauri::command]` the chunk adds or changes opens a manual `tauri.command.<name>` span guard and has no `#[tracing::instrument]` attribute. The span name stays inside the bounded `tauri.command.*` set. If the chunk has no backend delta, record that explicitly as not applicable (per obs-plan §4 Auto-instrumentation per surface; §11 Spans / Traces).
- (obs) The routine arm's `runs/a11y/<run_id>.jsonl` violation record still passes `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y` after the new assertions: the eleven envelope keys are present, the resource tags are present, and there is no absolute host path (per obs-plan §3 Log format JSON schema; §9 Telemetry artifact handling).
- (obs) Zero unlogged panics across the PREREQ gate runs and the delta runs: no `^thread.*panicked` line in `logs/agent-latest.jsonl` or stderr (per obs-plan §10 Always-required SLO invariant; §9 Zero-unlogged-panics gate).
