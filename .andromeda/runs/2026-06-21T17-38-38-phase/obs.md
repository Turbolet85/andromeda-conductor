# obs extract

## Relevance
Partial — Run-report envelope serializer touches log-format schema + redaction boundaries; does NOT touch harness init, SLO assertion, or CI gates.

## Constraints
- Per obs-plan §3 Logging stack: envelope JSONL schema is the binding contract from tests; 11-field shape per §6 amendment `2026-06-16-emission-journal-writer` (adds `read_back_observed_at`)
- Per obs-plan §1 Instrumentation scope: conductor-report is instrumentable; journal write + report render + DB insert operations are boundary-call candidates
- Per obs-plan §6 Log Coverage: required fields on every scenario-result record — `journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`; scenario-specific fields per critical path (e.g., `bypass_triggered`, `lifecycle_phase`, `degraded_mode_response`)
- Per obs-plan §11 Anti-Patterns / Logs: envelope must NOT leak absolute host paths or internal struct names; redaction layer applies field-allowlist (drops non-allowlisted Debug-dumped fields) + value scrub (host-file-path anchor, NOT blanket `::` token redaction); allowlisted `target` module path preserved
- Per obs-plan §3 Blocked-row null rule: a `Blocked` state envelope populates identity fields only (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`); `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` serialize as JSON `null`

## Patterns to follow
- JSONL serialization via serde with canonical field naming (no absolute paths / internal names)
- Per obs-plan §3 Timestamp encodings: `run_id` stays hyphen-delimited (`YYYY-MM-DDTHH-MM-SS-<suffix>`); in-payload instants use RFC-3339 colon format (envelope `journal_emitted_at` / `read_back_observed_at` ISO-8601 strings)
- Per obs-plan §6 Boundary-call wrapper for `report.generate`: log final verdict + state + fingerprint count at span close
- Per obs-plan §1 Telemetry triggers / creator-explicit-telemetry: span for `redaction.apply_field_allowlist()` on journal write; removal of non-allowlisted fields

## Anti-patterns to avoid
- NEVER leak absolute host paths or internal struct names in serialized form (obs-plan §11 Logs anti-pattern); use field-allowlist + `Display`-not-`Debug` at edge
- NEVER use multi-line stack traces in envelope fields (obs-plan §11 Logs); JSON-serialize to single field if included
- NEVER hardcode redaction rules in a single location — redaction layer in `conductor-core::redact` is the owner (obs-plan §11 PII Scrubbing)

## Contract bindings
**Run-report envelope ↔ tests harness:** envelope shape is binding contract from upstream test-plan §3 (status-shape + polled fields); obs-plan §3 reconciled on `2026-06-16` amendment to align all four schema reproductions (obs §3 + §6 + arch §Standard Contracts + runs.db golden rows).

## Acceptance criteria contributions
- (obs) Run-report envelope serializes to JSONL with schema matching obs-plan §6: 11-field shape (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) + scenario-specific fields.
- (obs) Blocked state: identity fields + `slo_tier` only; `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `fingerprints` are JSON `null`.
- (obs) No absolute host paths or internal struct names in serialized envelope (redaction layer applies field-allowlist + host-file-path value scrub per obs-plan §11).
- (obs) Timestamp fields use RFC-3339 format in JSON (ISO-8601); `run_id` is hyphen-delimited stem for runs.db PK + artifact naming.

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified self-obs base-line shape vs Run-report envelope; the §3 schema block is the envelope (scenario-result record, Epoch 6 report seam); custom `tracing-subscriber` layer emits constant identity fields flat on every line.
- **2026-06-15-log-error-boundary-redaction:** redaction model reconciled — value scrub masks absolute host-FILE paths → `<redacted>` (NOT `::` tokens); struct names kept out by field-allowlist + `Display`-not-`Debug` at edge; allowlisted `target` module path preserved (obs-plan §6 conformance gate clarified).
- **2026-06-16-emission-journal-writer:** Run-report envelope gains `read_back_observed_at` (ISO-8601, null until read-back, null for blocked rows); schema now 11 fields, aligns obs-plan §3 to test-plan §3 (owner) + arch §Standard Contracts + chunk implementation (`RunRecord`).
