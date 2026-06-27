# obs extract

## Relevance
Relevant — chunk closes the live Pulse E2E loop for the first time, exercising critical paths, canary bridge instrumentation, and all evidence artifacts.

## Constraints
- **No OTel SDK for self-observation** (per §3 OTel SDK init) — behavioral invariant; determinism via `tokio::current_thread` requires no background batch tasks
- **Wall-clock timing only** (§1 Instrumentability: conductor-timeline) — wall-clock `std::time::SystemTime` / `Instant`, never tokio virtual clock; essential for seeded emit ordering
- **11-field Run-report envelope** (per §3/§6 + amendment 2026-06-16) — `journal_emitted_at`, `read_back_observed_at` (ISO-8601, null until read-back / null for blocked rows), `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`; schema is binding contract from test-plan §3
- **Canary bridge must complete round-trip** (scope requirement) — emit known incident, verify Pulse ingests it into corpus.db, retrieve via `query_incident_list`, assert content fidelity; failure modes become distinct `Blocked` classifications
- **Host-path / struct-name redaction** (§6 + amendment 2026-06-15) — absolute filesystem paths (drive-letter `X:\`, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`); allowlisted `target` module path preserved; internal struct names kept out by field-name allowlist + `Display`-not-`Debug` at `anyhow` edge
- **No W3C trace context** (§3 Correlation) — `run_id` field is sole correlation key within a run; no `traceparent` / `trace_id` (Conductor is local single-process)
- **Agent-mode read-only trigger** (amendment 2026-06-24) — agent mode triggered by `--agent-mode` flag OR `CONDUCTOR_AGENT_MODE` env var; Conductor **reads** (`agent_mode = flag || env-set`), never **writes** the env var

## Patterns to follow
- **Critical path span nesting** (§4 Headless deterministic scenario run): `scenario.run` (root) → `timeline.execute` → `emit.batch` (per emission) → `verify.readback` (MCP call) → `report.generate` → `db.insert_run` (final verdict write); must-trace attributes per span per §4
- **Boundary-call logging** (§6 Boundary-call wrappers): MCP readback (`verify.readback`) logs method name + latency_ms + error; emit batch logs batch index + emission count; DB insert logs row count + run_id + verdict/state; report generation logs final verdict + fingerprint count
- **Span naming convention** (§2/§4): `{module}.{operation}` (low-cardinality; e.g., `scenario.run`, `verify.readback`, `emit.batch`, `report.generate`); no per-user-ID or per-path variability
- **Self-obs base schema on every line** (§3 amended 2026-06-27 clarification): `timestamp_ms` (epoch millis), `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id` — foundational fields on every `tracing` JSON output before scenario-result envelope fields
- **Latency measurement** (§5): `latency_ms` computed as `read_back_observed_at - journal_emitted_at` (wall-clock milliseconds); SLO assertion at report-generation time (`latency_ms <= threshold ? Pass : Fail`)

## Anti-patterns to avoid
- **High-cardinality span names** (§4) — no per-user-ID, per-trace-ID, or per-path-with-user-input spans; `emit.batch` and `verify.readback` are bounded (safe); avoid unbounded labels
- **OTel SDK initialization** (§3 + amendment 2026-06-17 behavioral invariant) — no SDK init for self-observation; dormant transitive opentelemetry + opentelemetry_sdk in deps (from opentelemetry-proto) is acceptable if never initialized; the harm is batch tasks + determinism break
- **Absolute paths or struct names in log output** (§6 + amendment 2026-06-15) — redact drive-letter paths, `/home`, `/Users`, `%APPDATA%` as `<redacted>`; do not blanket-redact `::`-tokens (would gut allowlisted `target` field + break panic context); keep internal backtrace file paths scrubbed but preserve type names in panics

## Contract bindings
- **Test harness contract** (focus guide §Cross-domain bindings) — tests consume structured log format (JSONL, §3 self-obs base schema + §6 envelope) + status endpoint shape + heartbeat presence
- **Preflight readiness gate** (scope requirement) — the canary round-trip must pass before dependent scenarios run; `ready: true` outcome gates all 5 family runs
- **MCP read-back client (conductor-verify boundary)** (§1 Instrumentability) — rmcp over `TokioChildProcess` stdio; instrument the `verify.readback` span with `mcp_method` + `latency_ms` attributes; protocol pinned to `2024-11-05`

## Acceptance criteria contributions
- **(obs) Canary bridge wired and instrumented** — `verify.readback` span completes round-trip (emit → Pulse corpus ingest → read-back → content-fidelity assertion); preflight gate reaches `ready: true`; distinct `Blocked` on failure (version mismatch / missing tool / empty canary / keychain conflict)
- **(obs) Live run journal + envelope + report** — each of 5 family runs writes JSONL journal (self-obs base + scenario-result records), `runs.db` row, Markdown report with `journal_emitted_at` / `read_back_observed_at` timestamps and tier-scaled SLO assertion; verdict-first classification (Pass / Fail / ManualCheck / KnownResidual)
- **(obs) No unlogged panics; determinism/error-wall intact** — all panics captured via `std::panic::set_hook()` as structured error logs; determinism invariant (same scenario+seed ⇒ same emission-stream shape) preserved; verdict/error-wall (transport/MCP/canary problems become typed `Blocked`/`Fail`, never panics)
- **(obs) Artifact redaction: no host-path / struct-name leakage** — JSONL journal, runs.db rows, Markdown report scrub absolute filesystem paths → `<redacted>`; preserve allowlisted fields (service.name, run_id, target); verify via agent-mode conformance gate (§9 CI Integration)

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 / §6) — two record shapes clarified: self-obs base-line (every line: `timestamp_ms`, `level`, `target`, service-identity, `run_id`) vs Run-report envelope (scenario-result: 11-field result record with verdict/latency/SLO). This chunk writes both shapes; §3 reconciliation documents the distinction. Custom `tracing-subscriber` layer required to emit constant identity fields flat on every line.
- **2026-06-16-emission-journal-writer** (§3/§6) — Run-report envelope field #2 is now `read_back_observed_at` (ISO-8601, null until read-back / null for blocked rows); schema is 11 fields. Realigned to test-plan §3 (envelope-schema owner) and `runs.db` data model. Canary round-trip completion populates `read_back_observed_at`; blocked rows carry null.
- **2026-06-15-log-error-boundary-redaction** (§6 / §11) — redaction model: absolute host-**file** paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>` (not `::`-token blanket redaction); internal struct names kept out by field-name allowlist + `Display`-not-`Debug` at `anyhow` edge; allowlisted `target` field (`module::`-shaped) explicitly preserved. Affects CI conformance gate (§9) which validates no absolute paths.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3) — `--agent-mode` CLI flag OR `CONDUCTOR_AGENT_MODE` env var (read-only trigger; Conductor reads, never writes via `unsafe std::env::set_var`). Sink: stderr pretty-print (dev) → JSON-only to `logs/agent-latest.jsonl` (agent mode). Scope uses `--agent-mode` for Windows headless runs; ANDROMEDA_PULSE_DATA_DIR propagated via hardened `.env(...)` (no argv/shell injection).
- **2026-06-27-obs-ci-conformance-gate** (§9) — operationalized for first time; conformance gate validates **§3 self-obs base-line schema** (`timestamp_ms`/`level`/`target`/`service.name`/`service.version`/`deployment.environment`/`run_id`), NOT §6 envelope. Envelope (`runs/<run_id>.jsonl`) gets its own, not-yet-built gate. Scope's evidence artifacts (JSONL + `runs.db` + `.md`) must pass both if run in CI. Zero-unlogged-panics gate (greps `agent-latest.jsonl` + stderr) unchanged.
