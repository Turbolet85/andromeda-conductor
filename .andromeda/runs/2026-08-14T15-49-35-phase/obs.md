# obs extract

## Relevance
Relevant — the chunk's deliverable is an observability capture artifact on the self-obs surface, produced at the `conductor-emit` OTLP boundary.

## Constraints
- Self-observation is `tracing` JSON only; no OTel SDK is ever initialized or used for self-obs, and the only OTLP Conductor speaks is the PRODUCT fault stream at `:4317` (per obs-plan §1 Obs Scope Summary, §3 OTel SDK init). A capture channel may not be an SDK, exporter, or receiver.
- NEVER export self-observation OTLP on any port — not `:4317` (PRODUCT), not `:4318` (dead/unused); recursion guard (per obs-plan §11 Universal). This is the obs-side twin of the scope's "Conductor opens no inbound listener".
- `conductor-emit` is instrumentable at the **outbound gRPC boundary only** — the emission itself, not Pulse-side receipt; Pulse is boundary-only / not-instrumentable (per obs-plan §1 Instrumentation scope, conductor-emit + Pulse MCP server rows). Conductor-side evidence stops at what left the socket; the Pulse-appender half is a hand-off, not an instrumentation target.
- A span attribute must be a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it **silently** (per obs-plan §4 Required-span-attributes constraint). The shipped list has no exception/event-count field — a new attribute would emit nothing, i.e. a second zero indistinguishable from the one under investigation.
- Any capture span must be a `{module}.{operation}` member of the bounded span-name set (`emit.batch`, `emit.logs_batch`, …); never a per-fingerprint / per-span-id / payload-derived name (per obs-plan §11 Spans / Traces).
- Every line written to `logs/agent-latest.jsonl` must carry the §3 self-obs base set (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`); the CI conformance gate fails the build on a bare record shape (per obs-plan §3 two record shapes, §9 Log conformance check).
- Redaction applies at the **processor** stage, not the sink: no absolute host-file paths, no internal struct names in the capture artifact; `Display`-not-`Debug` at the `anyhow` edge (per obs-plan §11 Logs, §11 PII Scrubbing). Timestamps are wall-clock `std::time::SystemTime`, never tokio's virtual clock (per obs-plan §11 project-specific bans).

## Patterns to follow
- **Shape witness on `message`** — the MCP read-back boundary log already records the observed KEY SET of each raw tool result, key names only and never values, on the allowlisted `message` field, precisely because a reader that degrades to empty makes divergence indistinguishable from emptiness (per obs-plan §6 Boundary-call wrappers). Same failure shape as this chunk's zero; reuse the pattern for the emitted payload's event/attribute key set.
- **Extend the existing `emit.batch` seam** rather than adding a parallel one: its must-log set is batch index + emission count + result status, with `emission_count` / `p_id_count` and (storm path) `batch_index` / `fingerprints_in_batch` as attributes (per obs-plan §6, §4 Fingerprint-storm scenario).
- **Span-lifecycle self-obs lines** — the custom layer materializes a §4 span as real JSONL lines (`span`, `span_event` = `new` | `close`, optional `parent`, allowlisted attributes on `new`) (per obs-plan §3 two record shapes). That is the existing readout channel for "what the nine emit spans actually carried".
- **Adding a conforming member to the bounded set** is the sanctioned way to name a new egress observation (precedent: `emit.logs_batch`) (per obs-plan §11 Spans / Traces).
- **Boundary-only for the far side** — observe the response/receipt, never assert Pulse internals (per obs-plan §1 conductor-verify + Pulse rows; consistent with the scope's finding that the four live-buffer read-back tools return empty cross-process).

## Anti-patterns to avoid
- NEVER stand up an OTel SDK, exporter, or self-obs OTLP path (any port) to take the capture — determinism + recursion guard (per obs-plan §11 Universal, §11 Telemetry Strategy).
- NEVER put fingerprint values or raw payload values into span names or attributes — high-cardinality span names are banned and non-allowlisted field names are dropped without warning (per obs-plan §11 Spans / Traces, §4).
- NEVER route the capture around the redaction boundary or scrub only at the sink — no absolute host paths, no internal struct names in the artifact (per obs-plan §11 Logs, §11 PII Scrubbing).

## Contract bindings
- **obs ↔ tests:** test-plan §3 OWNS the JSONL self-obs line format and the run-report envelope; obs aligns to tests, not vice versa (per obs-plan §3 Log format JSON schema). A new capture record shape or new self-obs field is a two-sided change (the one-sided drift D-tests-obs-harness guards).
- **obs ↔ security:** the field-name allowlist + host-path value scrub in `conductor-core::redact` is the single-location realization of security's artifact-hygiene rule (no absolute host paths in run-report artifacts) (per obs-plan §11 PII Scrubbing, §1 creator-explicit-telemetry).
- **obs ↔ CI:** the §9 log-conformance gate and the zero-unlogged-panics gate both read `logs/agent-latest.jsonl`; any new capture line flows through both (per obs-plan §9, §10).
- **obs ↔ arch / conductor-emit:** the `:4317` OTLP stream is the PRODUCT, owned by conductor-emit — it is never an obs transport (per obs-plan §1 Telemetry surfaces, §3 Correlation).

## Acceptance criteria contributions
- Every capture line in `logs/agent-latest.jsonl` carries the §3 self-obs base set (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`); a missing base field is a CI FAIL (per obs-plan §9 Log conformance check).
- Each field the capture records is either an `ALLOWLISTED_FIELDS` name or carried as key-NAMES-only text on `message`, and a dogfood read of the real artifact proves the witness actually appears rather than being silently dropped (per obs-plan §4 Required span attributes + §6 Boundary-call wrappers).
- No OTel SDK is initialized and no self-observation OTLP is exported on any port, including `:4318` (per obs-plan §3 OTel SDK init + §11 Universal).
- The capture artifact contains no absolute host-file paths and no internal struct names, with redaction applied at the processor stage; any new span name is a `{module}.{operation}` member of the bounded set (per obs-plan §11 Logs / PII Scrubbing / Spans · Traces).

## Relevant amendment history
- **2026-08-13-first-live-green-preflight** (§6 Boundary-call wrappers) — the immediately prior leg: the read-back boundary log gained the observed KEY SET witness (names only, on `message`) because the extraction readers degrade to empty on an unrecognized shape, making a field-name divergence indistinguishable from an empty corpus. That is structurally the same ambiguity this chunk's `tracked_fingerprints_count: 0` presents, and the amendment is the sanctioned resolution pattern.
- **2026-08-13-per-check-read-back-extraction** (§1 Critical paths, §4) — established that a span attribute outside `ALLOWLISTED_FIELDS` emits nothing, and retired two attributes (`degraded_mode_requested` / `response_received`) that could never have emitted; also fixed `mcp_method` → the shipped `mcp_tool`. This is the exact trap a new exception-event attribute would fall into.
- **2026-06-18-severity-logs** (§11 bounded span set) — precedent for adding a conforming `{module}.{operation}` egress span (`emit.logs_batch`, `record_count`) to the bounded set without violating the invariant; the route to take if the capture needs a new span name.
- **2026-06-17-raw-otlp-message-scaffold** (§3 OTel SDK init) — the no-SDK invariant is BEHAVIORAL; `opentelemetry-proto`'s dormant transitive `opentelemetry`/`opentelemetry_sdk` are not a violation, and a `default-features = false` follow-up is on record. Relevant because this chunk works inside conductor-emit's OTLP payload code.
- **2026-06-27-obs-ci-conformance-gate** (§9) — the gate asserts the §3 self-obs base schema against `agent-latest.jsonl`, not the §6 run-report envelope; a capture written to that file is judged by the base schema, and the envelope gate remains unbuilt.
- **2026-06-15-log-error-boundary-redaction** (§6 / §11) — the implemented redaction model: absolute host-FILE path anchor → `<redacted>`, struct names excluded by the field-name allowlist + `Display`-at-the-`anyhow`-edge, with the `module::`-shaped `target` explicitly preserved (no blanket `::`-token redaction).
