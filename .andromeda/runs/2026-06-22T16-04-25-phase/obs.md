# obs extract

## Relevance
Partial — statistical-anomaly scenarios introduce error-rate and latency-baseline telemetry instrumentation at the emit and verify boundaries, but are primarily TOML declarative catalog wiring with no new emission primitive.

## Constraints
- Per obs-plan.md §1: instrumentation scope = conductor-emit (raw OTLP message boundary) + conductor-verify (MCP read-back boundary) + conductor-report (JSONL journal write). This chunk exercises both emit + verify for error-rate / latency signals.
- Per obs-plan.md §4 (Span / Trace Coverage): all scenarios must-trace the `scenario.run` root → `timeline.execute` → `emit.batch` (per emission) → `verify.readback` (MCP call) → `report.generate` → `db.insert_run` critical path; added P-009..P-012 emit/verify spans inherit the `scenario.run` parent.
- Per obs-plan.md §6 (Log Coverage): JSON envelope fields must include `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms` (computed: `read_back_observed_at - journal_emitted_at`), `slo_tier`, `fingerprints` on every log line; no absolute host paths / internal struct names (redaction per §11, amendments 2026-06-15).
- Per obs-plan.md §5 (Metric Coverage): no metrics backend; performance budget is JSON field assertion at report time (latency_ms vs SLO threshold per slo_tier).
- Per obs-plan.md §9 (CI Integration): log conformance gate validates JSONL schema + zero unstructured panics; log file = `logs/agent-latest.jsonl`.
- Per obs-plan.md §1 Telemetry Triggers "perf-budget-instruments": latency_ms bucketing per slo_tier ("<5s | <20s | <90s"); P-010 detection <2s p99 → `<5s`, P-012 (≥60s persistence) → tune per tier-scaling model (scope.md open q4).

## Patterns to follow
- Span naming `{module}.{operation}` (e.g., `emit.batch`, `verify.readback`) with low-cardinality attributes (fault_type, mcp_method, latency_ms, batch_index) — per obs-plan.md §4 + amendment 2026-06-18 (emit.logs_batch joined the bounded set).
- Log fields on every JSONL line: service-identity + run_id + verdict/state/latency_ms (from scenario-result envelope per obs-plan.md §3 "Two record shapes" amendment 2026-06-15); redaction applies on journal write via allowlist + Display-edge (amendment 2026-06-15: host-file-path masking, NOT token-level `::` removal).
- Scenario .toml fixtures round-trip via `Scenario::from_toml_str` with the existing ExpectedCheck/PhaseTimeline model (scope.md q5 zero model change); each scenario's expected checks all `class="Hard"`.

## Anti-patterns to avoid
- Per obs-plan.md §11: no per-user-ID / per-trace-ID / per-path-with-user-input high-cardinality span names (bounded set: `scenario.run`, `timeline.execute`, `emit.batch`, `emit.logs_batch`, `verify.readback`, `report.generate`, `db.insert_run` + per-fault spans; P-009..P-012 add no new span names).
- No OTel SDK init or batch-task background work that breaks `current_thread` determinism (obs-plan.md §3 / amendment 2026-06-17).
- No W3C trace context / distributed tracing; correlation is `run_id` field only (obs-plan.md §3 Correlation).

## Contract bindings
- **obs ↔ tests harness §3** — the envelope schema (JSONL with journal_emitted_at / read_back_observed_at / verdict / latency_ms / slo_tier) is the binding contract obs derives from; tests-plan owner (obs §3 read-only, realigned per amendment 2026-06-16).
- **obs ↔ emit primitives** — P-009/P-011 baseline convergence + P-010/P-012 detection ramp seeded from Epoch-3 emit layers (latency-shaping, error-spans, error-rate, traffic-rate ramps) with no new primitive; TOML declares intent, Epoch-8 evaluator owns ±10%/±15% tolerance math + persistence-window timing.
- **obs ↔ a11y-plan §3** — violation-schema reproduction references the JSONL envelope shape (amendment 2026-06-16 adds read_back_observed_at; no further action).

## Acceptance criteria contributions
- (obs) Error-rate and latency spans (`emit.batch` for error/latency signals) tagged with low-cardinality attributes (error_rate, baseline_level, ramp_factor, latency_percentile) — no per-P-ID high-cardinality names.
- (obs) Scenario-result JSONL envelope includes verdict, state, latency_ms (read_back_observed_at − journal_emitted_at), slo_tier, fingerprints per §3 schema; no absolute host paths (redaction via allowlist + Display-edge per amendment 2026-06-15).
- (obs) P-009..P-012 scenarios deserialize via `Scenario::from_toml_str` and round-trip with no model changes; each `[[expected]]` check is `class="Hard"` (gate: cargo nextest + log conformance CI check).
- (obs) Determinism: same scenario+seed ⇒ same latency/error-rate signal stream (driven under `start_paused`; deterministic seeded p50/p95/p99 sampling per Epoch-3 primitives).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** — self-obs lines carry timestamp_ms / level / target / service-identity / run_id; Run-report envelope adds verdict/state/latency_ms/slo_tier/fingerprints (§3 two record shapes).
- **2026-06-15-log-error-boundary-redaction** — redaction = allowlist (drops non-allowlisted field names) + Display-not-Debug at error edge; host-file-path anchor masks absolute paths; allowlisted `target` (module::path) preserved.
- **2026-06-16-emission-journal-writer** — Run-report envelope schema = 11 fields (added `read_back_observed_at`); latency_ms = read_back_observed_at − journal_emitted_at (wall-clock ms).
- **2026-06-17-raw-otlp-message-scaffold** — "no OTel SDK for self-observation" is a behavioral invariant; opentelemetry-proto's transitive OTel SDK crates are dormant/allowed.
- **2026-06-18-severity-logs** — `emit.logs_batch` span added to the bounded set; if P-009..P-012 emit error/latency as logs, the span-name set is already updated.
