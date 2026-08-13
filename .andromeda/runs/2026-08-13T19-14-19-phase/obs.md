# obs extract

## Relevance
Relevant — the chunk lands directly on the `verify.readback*` span family, the envelope's `read_back_observed_at`/`latency_ms`/`verdict`/`state` fields, and the known-residual (`degraded_mode`) critical path.

## Constraints
- The read-back call must carry a span from the bounded set only — `verify.readback` on the canonical path, `verify.readback_degraded_mode` on the known-residual path; `{module}.{operation}`, no name derived per-check, per-P-ID or per-tool-result (per obs-plan §4 "Span naming convention" + §11 Spans/Traces bounded set).
- Required attributes on those spans are fixed: `verify.readback` → `mcp_method` (the tool name) + `latency_ms`; `verify.readback_degraded_mode` → `degraded_mode_requested` + `response_received`. Spans close on MCP response receipt (per obs-plan §4 Critical Path 1 + Known-residual classification path).
- `latency_ms` is `read_back_observed_at − journal_emitted_at` in wall-clock `std::time::SystemTime`/`Instant` — never tokio's virtual clock; `slo_tier` stays the closed enum; the SLO comparison is a JSON field assertion at report-generation time, not an instrument (per obs-plan §5 + §10 + §11 Project-specific bans).
- Envelope nullability is load-bearing as these fields become measured: `read_back_observed_at` null until read-back and null for blocked rows; `latency_ms` integer-or-null (null for blocked). The envelope stays 11 fields (per obs-plan §3 Log format JSON schema / §6 Log Coverage).
- Two record shapes stay distinct: the extraction seam's instrumentation emits **self-obs lines** (base set + span-lifecycle variant); the measured `verdict`/`state`/`latency_ms` land on the **Run-report envelope** (scenario-result record) — do not merge the two (per obs-plan §3 "Two record shapes").
- The read-back is a must-log boundary: log tool method name + `latency_ms` + error (if any) + canary-check result; `info` for completion, `warn` for degraded-mode response / timeout, `error` for protocol mismatch or transport refusal; conductor-verify's base level is `info` (per obs-plan §6 Boundary-call wrappers + Log levels + Per-module log levels).
- Malformed / absent / errored read-back must not panic and must not escape as an unstructured backtrace — typed `blocked` value plus a structured `tracing` error/warn line; panic hook + `anyhow` edge remain the only capture path (per obs-plan §11 Error Reporting + Project-specific bans; §10 zero-unlogged-panics).

## Patterns to follow
- Manual `#[tracing::instrument]` span wrapped around the rmcp seam in `conductor-verify` — the plan explicitly assigns conductor-verify "no auto-instrumentation, manual span around the MCP read-back call" (per obs-plan §4 Auto-instrumentation per surface).
- New span attributes materialize as real log lines only through the custom `tracing-subscriber` layer's span-lifecycle variant (`span` / `span_event: new|close` / optional `parent`, allowlisted attributes on `new`) — an attribute not in the field-name allowlist is silently dropped (per obs-plan §3 two record shapes; §11 PII Scrubbing allowlist).
- `run_id` is the only correlation key on every line; no `traceparent`/`trace_id`, and the new verify spans nest under the per-scenario `scenario.run` root opened in `conductor-run::execute_scenario` (per obs-plan §3 Correlation; §4 Critical Path 1 Cleanup).
- Redaction already sits at the processor stage in `conductor-core::redact` (host-file-path value scrub + field-name allowlist + `Display`-not-`Debug` at the anyhow edge) — extracted observed values route through it rather than gaining a new scrub site (per obs-plan §11 PII Scrubbing).

## Anti-patterns to avoid
- No high-cardinality span name or attribute derived from the extracted value (check index, expected token, corpus content) — the bounded span-name set is an invariant (per obs-plan §11 Spans / Traces).
- No OTel SDK, meter, or histogram introduced to measure the now-real `latency_ms` — Minimal tier has no metrics backend and an SDK breaks `current_thread` determinism (per obs-plan §11 Metrics + Telemetry Strategy).
- No corpus content, absolute host path, or internal struct name reaching a log field / journal / `runs.db` via the newly extracted observed value (per obs-plan §11 Logs + PII Scrubbing).

## Contract bindings
- **obs ↔ tests (§3 harness/envelope schema):** test-plan §3 OWNS the Run-report envelope; obs derives. This chunk makes `verdict`/`state`/`latency_ms`/`read_back_observed_at` measured rather than placeholder-derived — field names, count (11) and null semantics must remain the owner's (per obs-plan §3 / §6 "obs aligns to tests, not vice versa").
- **obs ↔ tests CI harness (§9):** the `logs/agent-latest.jsonl` conformance gate asserts the §3 self-obs **base** schema, not the envelope — any new self-obs field must not break base-schema validation, and the envelope's own gate is still unbuilt (per obs-plan §9 Log conformance check).
- **obs ↔ security:** the "no host paths / no internal struct names in artifacts" ban is realized by the single-owner `conductor-core::redact` layer, cited from security plan §2 (per obs-plan §1 creator-explicit-telemetry trigger; §11 PII Scrubbing).

## Acceptance criteria contributions
- The read-back call emits a bounded-set span (`verify.readback` / `verify.readback_degraded_mode`) with `mcp_method` + `latency_ms` attributes, visible as paired `span_event: new` / `close` self-obs lines carrying `run_id` (per obs-plan §4 Critical Path 1 + Known-residual classification path, §3 two record shapes).
- Envelope `latency_ms` equals `read_back_observed_at − journal_emitted_at` computed from wall-clock `std::time::SystemTime`, and both fields are null exactly on Blocked rows (per obs-plan §5 Metric Coverage + §3 Log format JSON schema).
- A degraded-mode read-back logs at `warn`, records `degraded_mode_response` on the scenario-result record, and writes `state = "KnownResidual"` (never `Fail`) (per obs-plan §4 Known-residual classification path + §6 Log levels).
- Zero un-redacted extracted values and zero unstructured panic text: no host path / internal struct name / corpus content in `agent-latest.jsonl`, `runs/<run_id>.jsonl` or `runs.db`, and no `^thread.*panicked` line for a malformed read-back (per obs-plan §11 PII Scrubbing + §9/§10 zero-unlogged-panics gate).

## Relevant amendment history
- **2026-06-16-emission-journal-writer** (§3 + §6) — envelope gained `read_back_observed_at` as field 2 (11 fields total), realigning obs to the owner test-plan §3 after obs-plan was the lone 10-field outlier; `latency_ms` pinned to `read_back_observed_at − journal_emitted_at`. Directly load-bearing: this chunk is the first to populate those fields for real, so the 11-field shape and null-on-blocked rule must not drift again.
- **2026-06-15-log-error-boundary-redaction** (§6 + §11) — redaction model pinned to `conductor-core::redact`: absolute host-**file**-path anchors → `<redacted>`, struct names excluded by field-name allowlist + `Display`-not-`Debug`, allowlisted `target` module path explicitly preserved; blanket `::`-token redaction was rejected as over-redaction. Governs the chunk's "redaction holds" boundary — reuse this layer, do not add a token-level scrub.
- **2026-06-15-structured-logging-stack** (§3) — established the two record shapes (self-obs base line vs Run-report envelope) and the custom subscriber layer, because stock `fmt().json()` cannot emit constant identity fields flat.
- **2026-08-10-scenario-run-root-span-tree** (§3 + §4) — recorded the span-lifecycle self-obs variant (spans materialize as real lines) and moved the `scenario.run` root to per-scenario in `conductor-run::execute_scenario`, with `report.generate`/`db.insert_run` reclassified as run-scoped siblings correlated by `run_id`. This is the exact function this chunk edits — a new verify span nests under that root, not under the report seam.
- **2026-06-18-severity-logs** (§11) — precedent for adding a conforming member (`emit.logs_batch`) to the bounded span-name set: a `{module}.{operation}`, low-cardinality addition is routine reconciliation, not an invariant breach — the path to follow if the extraction seam needs a span name beyond `verify.readback*`.
- **2026-06-27-obs-ci-conformance-gate** (§9) — the gate was pinned to the §3 self-obs base schema after it was found naming the §6 envelope fields; the envelope's own conformance gate remains unbuilt, so envelope-field correctness here is proven by tests, not by the CI log gate.
