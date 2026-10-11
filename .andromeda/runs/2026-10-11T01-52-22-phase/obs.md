# obs extract

## Relevance
partial — a removal chunk: obs adds no instrumentation here, but the plan's window-surface rows (desktop-webview / ipc-internal surfaces, `tauri.command.*` spans, the Tauri sinks, the a11y CI artifact, Critical Path 7's Tauri half) lose their subject, and the CLI-path obs gates must survive the removal intact.

## Constraints
- The bounded span-name set per obs-plan §11 Spans / Traces lists `tauri.command.*` beside the CLI-path names. Once the crate leaves, no `tauri.command.*` span site may survive anywhere under `crates/`, and no replacement name outside the bounded set may stand in. Whether any `tauri.command.*` site lives outside `crates/conductor-tauri` is research's question.
- Per obs-plan §4 Auto-instrumentation per surface (`cli` row), `conductor-cli` has no `tracing` dependency, so every CLI-path span and log comes from the libraries it drives. This chunk must not add that dependency edge just to keep a window-carried signal; giving functions their command-line entry is the next entry's job (scope B4).
- Per obs-plan §6 Boundary-call wrappers (DB READ row), the `conductor_run::read_envelope` boundary `info!` is specified as sitting INSIDE the caller's `tauri.command.*` span. The read stays (scope B1, the desktop envelope read), so that `info!` keeps the allowlisted `message` field and gains no `db.*` read span when its window caller goes. The §11 bounded set holds `db.insert_run` alone.
- The window log sink must not be removed or re-pointed here. The Tauri backend sink `<runs_dir.parent()>/logs/conductor-tauri.jsonl` (per obs-plan §3 Log file location, `.andromeda/registries/contracts/obs-plan/log-file-location.md`) is scope B1's "window log sink" and belongs to `Panel-shaped types retired`. If it becomes dead code under `-D warnings`, P4 names the move rather than silencing the lint (per obs-plan §10 Build / deploy failure conditions, the clippy-red row).
- Per obs-plan §9 Log conformance check / Zero-unlogged-panics gate, every CI run must keep uploading `logs/agent-latest.jsonl` (plus nextest output) and keep the log-conformance and zero-unlogged-panics checks. Removing the `frontend` / `a11y` jobs must not take either gate with them. Which job hosts each gate today is research's question.
- Per obs-plan §11 Universal (agent-driven specific), no self-observation OTLP may appear on `:4317` or `:4318`, and no new sink or log location may appear. The CLI sinks named in obs-plan §3 Logging stack (`.andromeda/registries/contracts/obs-plan/logging-stack.md`: stderr, or `logs/agent-latest.jsonl` under `--agent-mode` / `CONDUCTOR_AGENT_MODE`) stay byte-identical in behaviour.
- Per obs-plan §10 Always-required SLO invariant, the zero-unlogged-panics invariant keeps its CLI arm (exit 1 plus sanitized stderr). Only its Tauri arm ("error dialog + return error result") leaves with the crate. Whether the `std::panic::set_hook` installation is shared code or per-bin is research's question.

## Patterns to follow
- Removal leaves no half-instrumented seam. Per obs-plan §4 Must-trace path scenarios (Both-surface parity), the Tauri path's root `tauri.command.start_run` → `scenario.run` goes. The CLI path's `scenario.run` chain (Critical Path 1) stays unchanged, with `run_id` on every line.
- The a11y violation record `runs/a11y/<run_id>.jsonl` is the only exercised instance of the envelope's extension point: 13–15 keys, `service.name` `conductor-ui` (per obs-plan §3 Log format JSON schema, `.andromeda/registries/contracts/obs-plan/log-format-json-schema.md`). `journal_conformance` asserts key presence, never key exclusivity, so it needs no code change when that producer leaves. Keep it green over the scenario record's eleven keys.
- Retired surfaces leave through the wrap's amendments, never a phase edit (scope B6). The obs rows this removal makes stale: §1 telemetry surfaces (desktop-webview, ipc-internal) and service identity (`conductor-tauri` / `conductor-ui`, per obs-plan §3 Service identity), §3 Heartbeat ticks (Tauri backend/frontend `Channel` rows), §6 per-module levels (the `conductor-tauri` row) and sink config, §9 the a11y artifact row, §10 the `--e2e` clipless-arm sentence, and §11 the `tauri.command.*` member. P4 lists these as expected amendments.

## Anti-patterns to avoid
- Per obs-plan §11 CI: losing telemetry artifacts or skipping the zero-unlogged-panics gate. A job deletion that drops the `logs/agent-latest.jsonl` upload or the panic grep counts as this.
- Per obs-plan §11 Spans / Traces: widening the span-name set (for example a `db.*` read span or a CLI-side replacement for `tauri.command.*`) as a side effect of removing the window.
- Per obs-plan §11 Logs: leaking host paths. The removed a11y/SR legs' scrub (`<host-path>` plus `security_finding`) was a second line of defence. Their removal must not weaken the primary redaction layer `conductor-core::redact`, and the sidecar spawn's console suppression (§11 Project-specific bans) stays because it sits on the CLI path.

## Contract bindings
- obs ↔ tests §3: Critical Path 7 (both-surface parity, obs-plan §4) loses its Tauri half. The parity assertion and its test are test-plan-owned, and how they retire is decided there and at P4. `journal_conformance` (obs-plan §3 Log format JSON schema) loses its `runs/a11y` invocation and keeps the scenario-record one.
- obs ↔ a11y: the a11y violation-JSON record (obs-plan §9 Telemetry artifact handling, a11y row) retires with the `a11y` CI job. Its `conductor-ui` resource tag has no other producer named in the plan.
- obs ↔ security: the redaction layer (obs-plan §11 PII Scrubbing) is unchanged. The fetched-driver class and the SR speech-log ingest leave on security's side, and no obs gate keys on either.
- obs ↔ arch (scope B1): the window log sink path helper and the `read_envelope` boundary log stay until `Panel-shaped types retired`.

## Acceptance criteria contributions
- After removal, no crate under `crates/` opens a `tauri.command.*` span. Every remaining span name is in the bounded set, and none is added (per obs-plan §11 Spans / Traces).
- CI on the push that carries the removal still uploads `logs/agent-latest.jsonl` and nextest output, and its log-conformance and zero-unlogged-panics checks run and pass (per obs-plan §9 Log conformance check).
- `journal_conformance` stays green over the scenario record's eleven envelope keys with no key-exclusivity change (per obs-plan §3 Log format JSON schema).
- The CLI's self-obs sink behaviour is unchanged (stderr by default, `logs/agent-latest.jsonl` under agent mode), and no self-obs OTLP or new log location is introduced (per obs-plan §3 Logging stack).
