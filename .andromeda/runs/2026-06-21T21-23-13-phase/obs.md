# obs extract

## Relevance — partial

This chunk delivers the **coverage-matrix.md artifact** (completeness gate + P-ID classification), which is an *output boundary* requiring structured logging per obs-plan. The chunk itself does NOT produce logs/spans/metrics directly, but its artifact (the Markdown file) is part of the observability *surface* and must participate in the schema contract.

> ORCHESTRATOR NOTE (not part of the raw extract): the obs span instrumentation below (`report.coverage_matrix_generate`, runs.db `coverage_percent`/`missing_p_ids` columns) is SPECULATIVE and conflicts with the handoff's DEFERRED `report.generate` obs span (→ Epoch 8 caller). Treated as deferred at synthesis, not this chunk's work. Retained verbatim for the trace.

## Constraints — domain rules that apply

1. **Coverage-matrix completeness is gated by observability compliance (per obs §1 Instrumentation scope):** The artifact is a static classification of 60 P-IDs; no dynamic telemetry is emitted by the generator itself, but the artifact's write path must be logged to `logs/agent-latest.jsonl` (per §6 Log Coverage, Section 6 boundary-call wrappers: "Report generation (`report.generate`): log final verdict + state").
2. **No OTel SDK for this chunk's generation (§3 OTel SDK init):** The coverage-matrix generator must NOT initialize or use OpenTelemetry SDK; it is a pure/clock-free render operation, eligible for golden-test assertion.
3. **Run-report envelope binding (§3 Harness Contract):** The generator's output lifecycle (render → artifact write) must emit structured logs matching the JSONL schema binding contract (§6: `journal_emitted_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`). For the coverage-matrix gate (§4 Critical Path 6), `report.coverage_matrix_generate` is a bounded span; the coverage-matrix generator is the implementation of that span.
4. **PII scrubbing: no absolute paths in artifact (§11 PII Scrubbing):** The coverage-matrix.md must NOT contain absolute host-file paths or internal struct names; artifact write path must apply `conductor-core::redact` field allowlist.
5. **Agent-readable output (§2 Telemetry Strategy, agent-readable invariants):** The coverage-matrix.md artifact must be machine-parseable (markdown table with stable column headers) so downstream agents can validate completeness (`all 60 P-IDs present` + `zero unclassified`).
6. **CI artifact upload and conformance gate (§9 CI Integration):** The coverage-matrix.md is a CI artifact; its presence + completeness is asserted by the zero-unlogged-panics + log conformance gate. No panic on missing P-IDs — emit `state: Blocked` to JSONL if completeness fails, never crash silently.
7. **Span naming convention bounded set (§11 Anti-Patterns, Spans / Traces):** The generator's instrumentation must use the bounded span-name set; `report.coverage_matrix_generate` and `db.query_all_p_ids` and `report.validate_coverage` are the conforming names per §4 Critical Path 6.

## Patterns to follow — existing patterns relevant to implementation

1. **Pure/clock-free render with golden-test assertion (per scope §Definition of done):** The coverage-matrix generator follows the `RunReport::render` pattern (already in conductor-report crate) — pure function, no side effects during render, then artifact write is a separate step guarded by `std::fs::write(..., safe_overwrite_mode)`. This aligns with §3 "Logging stack" which requires render + write as distinct seams (render = logic, write = boundary call logged).
2. **Structured JSONL boundary-call logging at artifact write (§6 Log Coverage, boundary-call wrappers):** When the generator writes `coverage-matrix.md`, emit a `report.coverage_matrix_generate` span with attributes: `coverage_percent` (0-100), `missing_count` (integer), `p_id_count_found`, `p_id_count_expected` (60). Log the write event at `info` level with fields matching §6 envelope binding.
3. **`runs.db` envelope row for coverage-matrix gate (§4 Critical Path 6, obs-scope §5 Coverage triggers):** The coverage gate is an output report seam; store the completeness result (final `coverage_percent` + `missing_p_ids` array) in the runs.db row (the report envelope) so downstream queries can enforce "all runs must have `coverage_percent == 100`" at CI time.
4. **Lamp reuse for status rendering (if live-status column is decided in open question 1, scope.md):** If the Epoch-6 matrix renders a status column via `Lamp::for_record`, the rendering must reuse `conductor-core::Lamp::for_record` (not re-derive lamp precedence) per scope's "Lamp::for_record reuse" contract. The span attributes and logged fields must preserve the lamp-rendered state enum values (Pass / Fail / ManualCheck / KnownResidual / Blocked) without re-encoding.

## Anti-patterns to avoid — domain bans that apply

1. **Do NOT crash silently on missing P-ID (§11 Error Reporting):** If any P-ID is missing or unclassified, emit `state: Blocked` to JSONL (never a panic). The CI gate (`report.validate_coverage`) asserts `missing_p_ids` array is empty on pass; a missing ID is a defect, not a runtime crash.
2. **Do NOT hardcode artifact path or skip redaction at write (§11 PII Scrubbing):** All artifact writes must apply `conductor-core::redact` field allowlist. Do not emit internal struct names (e.g., `RowData { coverage_percent: … }` debug-dumped) to the Markdown; only the classification and count summaries go to the artifact.
3. **Do NOT use tokio virtual clock for `journal_emitted_at` field (§11 Project-specific bans):** The coverage-matrix generator's JSONL span events must use `std::time::SystemTime` (not tokio `Instant`), preserving journal-relative SLO math for any timeout assertions on the validation span.

## Contract bindings — where your domain ties into another

- **obs ↔ tests harness (§3 Observability Harness Contract):** The coverage-matrix generator is part of the report seam (Epoch 6); its JSONL output (with `report.coverage_matrix_generate` span, `coverage_percent`, `missing_count` attributes) is consumed by the tests harness contract (test-plan §3 Status shape, polled fields) to validate completeness. The runs.db row schema must include `coverage_percent` and `missing_p_ids` as queryable fields for CI agent assertion.
- **obs ↔ design system (cli + desktop-webview surfaces, §1 Telemetry surfaces):** The coverage-matrix.md artifact is consumed by CLI and Tauri desktop UI in downstream epochs (8/9); the artifact must be machine-parseable (markdown table) with stable column headers (`P-ID`, `Title`, `Category`, `Mode`, `State` or similar) so both surfaces can format/display it consistently.
- **(none)** for PII scrubbing domain (security plan §Logging & Monitoring) — Conductor owns no PII; the redaction layer is defensive, already implemented per 2026-06-15-log-error-boundary-redaction amendment.

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. **(obs) Coverage-matrix completeness gate:** All 60 P-IDs (P-001..P-060) emitted in `coverage-matrix.md`, each classified into exactly one of {auto, drive+observe, static-only}, zero unclassified/missing. Asserted by: `report.validate_coverage` span closes with `missing_count == 0` + `coverage_percent == 100`; runs.db row has `coverage_percent == 100`.
2. **(obs) Golden-test exact-string render:** `RunReport::render_coverage_matrix()` pure function produces byte-for-byte identical Markdown on deterministic inputs (same P-ID set, same classification order); golden test asserts render artifact against a committed `coverage-matrix.golden.md` fixture.
3. **(obs) Boundary-call logging with structured fields:** `report.coverage_matrix_generate` span attributes populated (`coverage_percent`, `missing_count`, `p_id_count_found` = 60, `p_id_count_expected` = 60); JSONL event logged at `info` level with `run_id`, `seed` (zero for static artifact generation), `verdict` (Pass if `coverage_percent == 100`, Fail or Blocked otherwise), `state` (Pass/Blocked), `latency_ms` (render + write duration, wall-clock).
4. **(obs) Artifact write is loud + safe:** File write uses `std::fs::write(...)` with exact-overwrite semantics (no append, no partial writes); on write failure, emit `tracing::error!(...)` with context (disk full, permissions) before panic/error propagation.

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why

(none) — The coverage-matrix generator is a new artifact introduced at Epoch 6; no prior amendments to obs-plan touch this specific output seam. The foundational amendments (2026-06-15 structured-logging-stack, 2026-06-15 log-error-boundary-redaction, 2026-06-16 emission-journal-writer, 2026-06-17 raw-otlp-message-scaffold, 2026-06-18 severity-logs) establish the JSONL envelope binding and redaction model that this chunk must conform to on artifact write — but no prior changes directly amend the coverage-matrix scope or bind additional instrumentation rules.
