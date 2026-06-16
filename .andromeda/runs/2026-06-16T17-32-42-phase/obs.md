# obs extract

## Relevance
Partial — CI harness skeleton / agent-run command dispatch has no *observability signal production* yet (spans/logs/metrics); obs bindings appear only at integration seams (logs sink configuration, agent-mode flag forwarding, CI artifact handling).

## Constraints
- Per §1 Obs Scope Summary: CLI is Instrumentable; agent-mode flag gates dual sink (stderr pretty-print dev, file `logs/agent-latest.jsonl` agent mode). This chunk formalizes the flag without emitting signals. (§1 Telemetry surfaces: cli row, Sink column)
- Per §3 Observability Harness Contract: `--agent-mode` CLI flag sets `CONDUCTOR_AGENT_MODE=1` internally; forces JSON-only to file, no pretty-print. No logging init yet (lands in subsequent epoch chunks that add scenario logic). (§3 Log file location; agent-mode flag)
- Per §9 CI Integration: `logs/agent-latest.jsonl` artifact uploaded per CI job; GitHub Actions run ID + git commit SHA wired as `ci.run.id` + `git.commit.sha` resource attributes at logging init (deferred). (§9 Telemetry artifact handling, CI-specific resource attributes)
- Per §9: `cargo-nextest` JSON output (`--message-format libtest-json`), `cargo clippy` warnings, `cargo-llvm-cov` coverage artifacts uploaded; agent parses these (not signal emission; infrastructure only). (§9 Pipeline integration)
- Per §11 Logs anti-pattern: CI must never lose telemetry artifacts; upload with 14-day retention. This chunk establishes the CI job stubs; retention is GitHub Actions default. (§11 CI / NEVER lose telemetry artifacts)

## Patterns to follow
- Per §2 Telemetry Strategy / Naming conventions: span naming `{module}.{operation}` (e.g., `scenario.run`, `timeline.execute`, `emit.batch`). This chunk has no spans yet; the pattern is a reserve for Epoch 2+. (§2 Naming conventions)
- Per §3 Service identity: `service.name` hardcoded `"conductor"` (CLI) or runtime `$CONDUCTOR_SERVICE_NAME` override; version via `env!("CARGO_PKG_VERSION")`; `deployment.environment` via `$CONDUCTOR_ENV` (default `"local"`). The script stubs must preserve env var forwarding (no shadowing). (§3 Service identity)
- Per §1 Telemetry surfaces table / CLI row: dual sink pattern (dev + agent mode); scripts must not assume log file existence during stub phase (downstream epochs may pre-create or omit). (§1 cli surface, Sink column)
- Per §9 Zero-unlogged-panics gate: CI greps `logs/agent-latest.jsonl` + stderr for unstructured panic backtraces; scripts must not swallow exit codes (early seams will inherit this). (§9 Zero-unlogged-panics gate)

## Anti-patterns to avoid
- Per §11 Logs: NEVER use unstructured stderr text — agent can't parse; all output MUST be structured JSON-per-line or sanitized stderr. This chunk's stub commands must not emit arbitrary debug text; any logging is deferred. (§11 Logs / NEVER use unstructured stderr text)
- Per §11 CI: NEVER lose telemetry artifacts (log file + status) — CI workflows must upload as artifacts. Stub workflows must reserve the artifact upload path (even if source is empty at this stage). (§11 CI / NEVER lose telemetry artifacts)
- Per §11 Universal: NEVER export any self-observation OTLP — not to `:4317` (PRODUCT fault stream) and not to `:4318` (dead port); self-obs is stdout/file/console JSON only. Scripts must not initialize any OTLP exporter or background task for self-observation. (§11 Universal / NEVER export any self-observation OTLP)

## Contract bindings
- **Tests harness** (per focus guide cross-domain bindings): tests consume structured log format + status endpoint shape; this chunk establishes CI artifact export paths but defers log format schema emission (logs §5 "emit JSON schema file" in bootstrap phases) to Epoch 2+. Contract is read-only at this stage.
- **Security § Logging & Monitoring** (per focus guide): PII scrubbing binds to logs via field-allowlist redaction layer (`conductor-core::redact`); this chunk has no logs yet. Binding is reserve.

## Acceptance criteria contributions
- (obs/CI) `logs/agent-latest.jsonl` upload configured in GitHub Actions workflow with 14-day retention; path is agent-parseable (even if empty at stub phase).
- (obs/agent-run) `--agent-mode` flag accepted by both `agent-run.sh` and `agent-run.ps1`; forwards to any downstream scenario runner as `CONDUCTOR_AGENT_MODE=1` env var without modification.
- (obs/agent-run) Exit code preserved across command dispatch (no swallowing of harness panics or subprocess failures) to enable CI zero-unlogged-panics gate.

## Relevant amendment history
- **2026-06-15-structured-logging-stack**: clarified two record shapes (self-obs base line vs Run-report envelope). Impacts this chunk: the foundational self-obs log line carries `timestamp_ms`, `level`, `target`, service-identity, `run_id` on every line; envelope fields only on scenario-result events. Scripts must not assume envelope shape until scenario logic lands. (No edit to obs-plan body; note for implementation.)
- **2026-06-15-log-error-boundary-redaction**: reconciled redaction model (host-file-path anchor; struct-name guard via allowlist + Display). Impacts this chunk: CI conformance gate §6 no longer treats a `module::` prefix in `target` field as a struct-name leak; `target` is allowlisted and preserved. Scripts must forward the `target` field without modification. (No edit; implementation clarification.)