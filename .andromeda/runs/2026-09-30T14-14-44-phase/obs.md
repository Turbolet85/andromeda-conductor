# obs extract

## Relevance
partial — the chunk is an a11y/screen-reader harness and evidence chunk with no planned self-obs instrumentation; obs binds only where each arm launches the Tauri bundle (the backend log sink and the panic invariant) and where untrusted speech-log text and host paths enter committed evidence.

## Constraints
- The Tauri backend log file name is fixed, and its directory follows `CONDUCTOR_RUNS_DIR` (`<runs_dir.parent()>/logs/conductor-tauri.jsonl`). Each a11y suite has its own measured directory (`runs/logs/` for `sr-empty`, `runs/sr-leg/logs/` for `sr`/`sr-error`). Any new arm that launches the bundle (step 1's driver-launched OS-key control, or a by-path launch) therefore writes its backend log to whatever directory that arm's `CONDUCTOR_RUNS_DIR` names, or to the project-root `logs/` when the variable is unset (per obs-plan §3 Log file location; §6 Sink configuration).
- The Tauri frontend's self-obs is `console.log` JSON only. It has no OTel JS SDK and no network OTLP export to `:4317` or `:4318`. A harness change that injects into or drives the webview must not add a telemetry export path (per obs-plan §3 Logging stack; §11 Universal).
- No committed artifact may carry an absolute host path. This covers every record this chunk writes into its own `evidence/` (a regraded NVDA record, arm write-ups, the configuration strings named in each verdict). The drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo` and `.rustup` forms are the ones §9's conformance check rejects (per obs-plan §11 Logs; §9 Log conformance check).
- The three redaction sites in `conductor-core::redact` do not cover every host-path channel. The a11y leg's own scrub (untrusted speech text → `<host-path>` placeholder + a `security_finding` flag) is the standing second line of defence for anything NVDA speaks. The speech of a new launch posture (driver-launched with OS keys) is a channel no prior measurement bounded (per obs-plan §11 PII Scrubbing, the standing-warning bullet).
- Zero unlogged panics is the one always-required SLO invariant. Every Conductor launch in a leg must leave only structured `tracing::error!(panic=…)` JSON events, never a raw `thread … panicked` line (per obs-plan §10 Always-required SLO invariant; §9 Zero-unlogged-panics gate).
- If the harness change reaches Rust (e.g. a Tauri command or the spawn path), any new span must stay inside the bounded span-name set (`tauri.command.*` etc.) and use a manual `info_span!(…).entered()` guard, never `#[tracing::instrument]`, on a `#[tauri::command]`. Whether the chunk touches Rust at all is research's question (per obs-plan §4 Auto-instrumentation per surface; §11 Spans / Traces).

## Patterns to follow
- Use the extension point of the harness-artifact record shape: a harness record may add resource tags (`service.name`, `deployment.environment`, plus `ci.run.id`/`git.commit.sha` under CI) beside the eleven envelope keys. `journal_conformance` asserts key presence, never exclusivity. If a regrade record goes through that gate, naming runtime × driver × NVDA × OS build × input path as tags fits this shape. Whether the SR record is subject to `journal_conformance` at all is research's question (per obs-plan §3 Log format JSON schema, extension-point paragraph; §9 CI-specific resource attributes).
- Treat the per-suite `conductor-tauri.jsonl` as the agent-readable witness that the app launched and served its commands under a given posture. Read it from the suite's own runs-dir `logs/` rather than inferring launch state (per obs-plan §3 Log file location; §3 Snapshot / paste-to-AI integration).
- Carry the cause of a closed host-path channel as a measured before/after reading of the committed NVDA record (count of `heard` placeholders and `security_finding` rows). That is the measured shape the sidecar-console closure used (per obs-plan §11 PII Scrubbing, standing-warning bullet).

## Anti-patterns to avoid
- NEVER add a network OTLP exporter or OTel SDK on any surface, the webview included, to observe focus or input events. Self-obs stays stderr/file/console JSON only (per obs-plan §11 Universal; §11 Telemetry Strategy).
- NEVER commit untrusted speech-log text or harness output that carries absolute host paths, and NEVER assume the named redaction sites bound every channel (per obs-plan §11 Logs; §11 PII Scrubbing).
- NEVER use unstructured stderr as the only record of a verdict-bearing observation. Every signal the regrade rests on must have a machine-parseable, agent-readable form (per obs-plan §11 Logs; §11 Universal).

## Contract bindings
- obs ↔ a11y: the `sr*` leg's speech-log ingest is the second host-path defence (`<host-path>` + `security_finding`), and obs-plan §11 PII Scrubbing relies on it. A new input path or launch posture must keep that scrub in the ingest path.
- obs ↔ tests: the per-suite backend-log directories (obs-plan §3 Log file location) follow the `CONDUCTOR_RUNS_DIR` each suite sets in `wdio.conf.ts`. Test-plan §3 owns the record formats, and `journal_conformance` owns key-presence and host-path assertions on harness records (obs-plan §3 extension-point paragraph; §9).
- obs ↔ security: the host-path hygiene on committed evidence (obs-plan §9 Log conformance check / §11 Logs) is the same boundary as security-plan §Error Handling.

## Acceptance criteria contributions
- Every arm that launches the Conductor bundle does so with `CONDUCTOR_RUNS_DIR` set to a known suite or arm directory. The resulting `conductor-tauri.jsonl` lands under that directory's `logs/`, and none lands stray in the project-root `logs/` (per obs-plan §3 Log file location).
- Each launched arm's `conductor-tauri.jsonl` contains zero lines matching `^thread.*panicked` (per obs-plan §9 Zero-unlogged-panics gate; §10 Always-required SLO invariant).
- Every record committed under the chunk's `evidence/` contains zero absolute host-path runs. Each `letter:slash` match is enumerated and shown not to be a host path. Any speech-derived host path appears only as `<host-path>` with a matching `security_finding` row (per obs-plan §9 Log conformance check; §11 PII Scrubbing).
- No new self-obs export path appears in the diff against the chunk base `ff4f571`: no OTel/OTLP exporter, no `:4318`, no browser telemetry beyond `console.log` JSON (per obs-plan §11 Universal).
