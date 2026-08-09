# obs extract

## Relevance
Partial — the drift check emits no telemetry of its own beyond a failure surface, but its error/log line, span naming (if instrumented), and artifact hygiene fall squarely under obs.

## Constraints
- Obs tier is Minimal (0): this chunk adds NO new telemetry infrastructure — no OTel SDK, no exporter, no metrics instrument for the drift check; self-observation stays structured `tracing` JSON only (per obs-plan.md §1, §3 "OTel SDK init", §5).
- If the drift check is instrumented, span names must come from the bounded `{module}.{operation}` set and reuse the coverage-gate names (`report.coverage_matrix_generate` → `report.validate_coverage`); no new high-cardinality name, and no per-P-ID span for the 22 drifted ids (per obs-plan.md §4, §11 "Spans / Traces").
- Coverage-universe telemetry must name the manifest's capability count/set, never a literal: `p_id_count_expected` and the `p_ids` field are manifest-derived (per obs-plan.md §4 "Coverage-matrix completeness gate", as amended 2026-08-08).
- Redaction is single-location in `conductor-core::redact` — the drift message must pass through the existing field-name allowlist + host-file-path value scrub; the check must not hand-roll a scrubber or a second allowlist (per obs-plan.md §11 "PII Scrubbing").
- Error surfacing is `tracing::error!` for an unrecoverable harness fault (`warn`/`info` for a non-gating surfaced signal) plus the `anyhow` binary edge with `Display`-not-`Debug`; no external error-reporting platform (per obs-plan.md §6 log-levels mapping, §11 "Error Reporting").
- Output must be structured JSON-per-line (or sanitized machine-parseable stderr) carrying `run_id` — never unstructured stderr prose, even for a human-facing "loud failure" (per obs-plan.md §11 "Logs", §3 self-obs base line).
- If this lands as a CI gate, it inherits the artifact rules: `logs/agent-latest.jsonl` uploaded and agent-readable, and a conformance/gate FAIL is a build-gating condition (per obs-plan.md §9, §10 "Build / deploy failure conditions").

## Patterns to follow
- The §4 Coverage-matrix completeness gate is the nearest existing template for this exact artifact-vs-universe assertion: `p_ids` (manifest set), `missing_p_ids` (empty on pass, populated on fail), `coverage_percent`, `missing_count` — reuse those field names rather than inventing `drifted_ids`-style synonyms (obs-plan.md §4).
- Reuse the shipped redaction infra at `crates/conductor-core/src/redact.rs` (field-name allowlist + `redact_value`), as the file-sink chunk did — consuming unchanged hardened infra is not a new boundary (obs-plan.md §11 "PII Scrubbing").
- Follow the existing self-obs layer at `crates/conductor-core/src/obs.rs` for line shape (identity fields + `run_id` flat on every line) rather than a bespoke formatter (obs-plan.md §3 "two record shapes").
- Emit manifest metadata (`sut_version`, `captured_at`) as structured fields, not interpolated prose, so the red gate is `jq`-parseable — the paste-to-AI consumption path (obs-plan.md §3 "Snapshot / paste-to-AI integration", §2 agent-readable invariants).
- Wall-clock `std::time` only if the check stamps anything; never tokio's virtual clock (obs-plan.md §11 "Project-specific bans").

## Anti-patterns to avoid
- No OTel SDK, meter, exporter, or `traceparent` introduced to report drift — correlation is the `run_id` field (obs-plan.md §11 "Telemetry Strategy", "Metrics", "Spans / Traces").
- No absolute host paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`) and no internal struct names in the drift message, `runs.db`, or run-report artifacts (obs-plan.md §11 "Logs").
- Never let drift detection (or a malformed/missing manifest on the read path) panic or emit an unstructured `thread ... panicked` line (obs-plan.md §11 "Error Reporting", §10).

## Contract bindings
- **obs ↔ tests harness:** the drift check's structured failure record is consumed by the tests harness; field names must not collide with or shadow the Run-report envelope (`verdict`/`state`/`latency_ms`), whose schema is OWNED by test-plan §3 — obs derives, never re-authors (obs-plan.md §3, §6).
- **obs ↔ security:** artifact hygiene on the failure message is the redaction layer's contract (no host paths / struct names in run-report artifacts) (obs-plan.md §11 "PII Scrubbing", security plan §2 anti-pattern cited there).
- **obs ↔ core error surface:** drift is a harness-side error at the `anyhow` edge, never a `Verdict`/`ReportState` value — matching the chunk's verdict/error-wall invariant (obs-plan.md §11 "Error Reporting").

## Acceptance criteria contributions
- (obs) The drift failure output is structured JSON-per-line carrying `run_id` and the service-identity fields — no unstructured stderr prose, parseable by `jq`/`serde_json` (per obs-plan.md §11 "Logs" + §3 self-obs base-line schema).
- (obs) The failure record contains no absolute host path and no internal struct name — manifest metadata (`sut_version`, `captured_at`) and capability ids only, via the existing `conductor-core::redact` allowlist and `Display`-not-`Debug` at the `anyhow` edge (per obs-plan.md §11 "Logs" / "PII Scrubbing").
- (obs) Any span or log field the check adds stays inside the bounded span-name set and reuses the coverage-gate field names (`p_ids`, `missing_p_ids`, `coverage_percent`), with `p_id_count_expected` sourced from the manifest and no hardcoded 60/82 literal (per obs-plan.md §4 "Coverage-matrix completeness gate" + §11 "Spans / Traces").
- (obs) Drift never panics: it is a typed harness-side error, and `logs/agent-latest.jsonl` + stderr contain zero unstructured `^thread.*panicked` lines when the check fires (per obs-plan.md §10 "Zero unlogged panics" + §9 zero-unlogged-panics CI gate).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest** (§4 Span/Trace Coverage — coverage-matrix completeness gate; §1 must-trace table): de-hardcoded the gate's span attributes so `p_id_count_expected` and the `p_ids` log field name the manifest's capability count/set instead of the literal 60. Why: the same SUT-advance reversal this chunk detects — the span spec asserted a fixed 60 the accepted set no longer defines. Directly governs how this chunk may describe the classified universe in telemetry.
- **2026-06-15-log-error-boundary-redaction** (§6 CI conformance check; §11 Logs / PII Scrubbing): fixed the redaction model to host-FILE-path masking + field-name allowlist + `Display`-not-`Debug`, explicitly NOT `::`-token redaction, and preserved the allowlisted `target` module path. Why: blanket `::` scrubbing would gut correlation fields. Relevant because this chunk's failure message must satisfy "no internal struct names" through that allowlist, not through token stripping.
- **2026-06-27-obs-ci-conformance-gate** (§9 Log conformance check): established that `logs/agent-latest.jsonl` is validated against the §3 self-obs base schema, not the §6 Run-report envelope; the envelope gets its own not-yet-built gate. Why: first operationalization of the §9 gate revealed §9 named the wrong record shape. Relevant if this drift check lands as a CI gate — it must not be conflated with the envelope conformance gate.
