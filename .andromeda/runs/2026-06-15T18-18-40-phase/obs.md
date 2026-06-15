# obs extract

## Relevance — relevant
This chunk realizes the obs-plan's §3 pii-scrubbing-wire bootstrap phase: the artifact-hygiene redaction layer for self-obs logs + error-boundary sanitization.

## Constraints
- per obs-plan §3 Observability Harness Contract: custom `tracing-subscriber` layer emits constant identity fields (`service.name`, `service.version`, `deployment.environment`, `run_id`) flat on every JSON line; field-allowlist redaction filters non-allowlisted field names at the subscriber processor stage.
- per obs-plan §6 Log Coverage: allowlisted self-obs field set = `{ timestamp_ms, level, target, service.name, service.version, deployment.environment, run_id }`; envelope fields (`journal_emitted_at`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) appear only on scenario-result records (Epoch 6), not on every line.
- per obs-plan §11 PII Scrubbing: redaction applies via (a) tracing-subscriber field filter on JSON formatter, (b) report generation on JSONL write, (c) runs.db schema validation on insert; never scrub only at sink stage (failures upstream leak via stderr); hardcode scrubbing rules in a single location: preserve ONLY verdict/state/identity/count fields; remove all `path::`, `module::`, `backtrace` paths.
- per obs-plan §11 Anti-Patterns: NEVER use unstructured stderr text — all output MUST be structured JSON-per-line or sanitized stderr; NEVER leak absolute host paths or internal struct names in logs; NEVER use multi-line stack traces without one-line serialization — `std::panic::set_hook()` must JSON-serialize backtrace to a single field.
- per security-plan excerpt (scope.md): sanitize at the `anyhow` edge + the tracing-subscriber field-allowlist — no stack traces, absolute host paths, or internal struct/field names reach the operator; only the operator-facing `error:` / `hint:` shape.

## Patterns to follow
- Field-allowlist redaction at the `tracing-subscriber` processor layer (not sink-stage) so upstream failures don't leak via stderr; expressible as a `Layer<S: Subscriber>` composing the custom JSON formatter from the prior chunk.
- Redaction primitive reusable by future run-report writers (Epoch 6: `runs.db` insert, JSONL journal write, Markdown report) — share a single `Redactor` module/trait.
- `anyhow` edge sanitization at `conductor-cli` main() error handler + `conductor-tauri` command-handler error boundaries: convert typed enum errors to `anyhow::Error`, scrub payload via the shared redactor, emit only `error: <message>` / `hint: <suggestion>` (no backtrace/stack).
- Wall-clock timestamp (`std::time::SystemTime`, epoch millis as `timestamp_ms`) on every self-obs line; panic hook captures backtrace as one-line JSON field, redacted before sink.

## Anti-patterns to avoid
- Do NOT re-init the obs stack or panic hook — they exist (prior chunk); compose the redaction layer on top.
- Do NOT leave stack traces / host paths / internal struct names unmasked in self-obs JSON or error output (CLAUDE.md artifact-hygiene invariant).
- Do NOT apply redaction only at sink stage (failures leak via stderr before reaching sink).

## Contract bindings
- **obs ↔ Epoch-6 artifact writers**: this chunk builds the redaction primitive wired to existing surfaces (self-obs logs, cli/tauri error edges); Epoch 6 artifact writers reuse it.
- **obs ↔ tests harness §Log conformance gate**: agent reads CI artifact logs (`logs/agent-latest.jsonl`) post-redaction; validates no host-path prefixes (`path::`) or module prefixes (`module::`) in any output field.
- **obs ↔ security-plan §Error Handling**: artifact-hygiene invariant — never leak absolute host paths or internal struct/field names to operator or artifacts.

## Acceptance criteria contributions
- (obs) Redaction: self-obs JSON log lines carry only allowlisted fields (`timestamp_ms`, `level`, `target`, `service.*`, `run_id`); values matching `path::*` or `module::*` shapes are masked/dropped.
- (obs) Error boundary: cli/tauri error returns sanitized to operator-facing `error:` / `hint:` shape (no backtrace, host paths, or struct names); `std::panic::set_hook()` JSON line redacted before sink.
- (obs) Redaction primitive: reusable by future JSONL/runs.db/report writers (Epoch 6); CI log conformance gate detects any leaked host-path or module-name prefixes.
- (obs) Regression gate: `cargo nextest run --workspace` + clippy + audit/deny remain green; no new critical deps introduced.

## Relevant amendment history
**2026-06-15-structured-logging-stack:** clarified self-obs log schema (base set on every line: `timestamp_ms`, `level`, `target`, service-identity, `run_id`; envelope fields only on scenario-result records). This chunk inherits that clarification — allowlist redaction applies to the base field set on every line, with envelope-specific redaction deferred to Epoch 6 writers. No plan body edit; playbook rule added to prevent future Bootstrap-phase deferrals from escalating.
