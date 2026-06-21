# obs extract

## Relevance — partial

Connection-lifecycle scenarios (P-001..P-004) author declarative TOML catalog entries with `expected` outcome blocks expressing timing assertions (Receiving ≤1s, ReceiverFailed ≤2s, Idle/Stalled boundaries) per obs-plan §1 critical paths + §5 perf-budget-instruments trigger. The chunk extends `Scenario`/`PhaseSpec` model to wire these assertions; live MCP read-back verification and CLI runs are Epoch 8/10 out-of-scope.

## Constraints — domain rules that apply

1. **Scenario model carries `expected` block (obs-plan §3 harness contract):** Every scenario TOML must serialize to a `Scenario` struct with an optional `expected: ExpectedOutcome` field binding the timing windows + SLO tier; this enables report-generation to validate latency assertions at the JSON envelope level (`latency_ms <= slo_tier_threshold`).

2. **SLO tiers match capability spec thresholds (obs-plan §5 perf-budget-instruments + scope.md requirement source):** P-001 (Receiving) ≤1s, P-002 (Idle) 10–60s degrading to P-003 (Stalled) >60s, P-004 (ReceiverFailed) ≤2s — map these windows to `<5s` / `<20s` / `<90s` SLO tier enums per the capability-spec.md "Conductor verification" clauses.

3. **Critical path instrumentation preserved (obs-plan §4 must-trace scenarios):** Every scenario route—whether P-001 or P-004—traverses `scenario.run` (root) → `timeline.execute` → `emit.batch` (or bypass for P-004 port-occupier) → `verify.readback` → `report.generate`. Model extension must not alter span hierarchy or lose correlation via `run_id` field on every JSONL line.

4. **Redaction layer permits only allowlisted fields (obs-plan §11 PII Scrubbing anti-pattern):** Scenario TOML fields and any `expected` block attributes must not introduce absolute host paths, internal struct names, or Debug-dumped fields; the field-name allowlist in `conductor-core::redact` (value anchor: drive-letter / `/home` / `/Users` → `<redacted>`) governs report output — keep the model extension lightweight to avoid redaction surprises.

5. **Zero unlogged panics on model validation (obs-plan §10 SLO invariants):** Any `garde` validation error on scenario deserialization must be captured as `tracing::error!(...)` + converted to `anyhow::Error` at the CLI edge, never left as an unstructured panic; the phased execution (start_paused fixture test per scope.md DoD) proves determinism (`same scenario + seed => same PhaseTimeline shape`).

6. **Port-occupier fault span attributes (obs-plan §4 fault-injection spans):** The P-004 ReceiverFailed leg reuses `conductor-faults` port-occupier; its instrumentation emits a `fault.port_occupier` span with attributes `fault_type`, `fault_duration_ms`, `fault_start_offset_ms` (journal offset) — the model extension must not suppress or duplicate this span.

7. **No W3C trace context in envelope (obs-plan §3 correlation / §11 anti-patterns):** Scenario config carries `run_id` for correlation (same seed => same verdict/state across surfaces); do NOT wire a `traceparent` field or OTel SDK into the model — determinism requires `tracing` span hierarchy + `std::time` wall-clock only, no tokio virtual clock.

## Patterns to follow — existing patterns relevant to implementation

1. **Scenario TOML shape precedent (scope.md existing precedent):** `scenarios/error-baseline-spike.toml` establishes the pattern — `name · p_ids · seed · slo_tier · jitter_ms · [[phases]] {name, gap_ms}`. Extend this shape by adding an optional `[expected]` block (timing window + verdict enum, per scope.md model-extension depth question) and optional `[holds]` iff a lifecycle scenario needs operator go/no-go (likely not for drive+observe timing, resolve in planning). Follow the same serde + garde validation path (`Scenario::from_toml_str` → `PhaseTimeline`).

2. **Binding schema for JSONL envelope fields (obs-plan §6 log coverage):** The Run-report envelope is the authority — `journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`, plus scenario-specific fields (`bypass_triggered`, `lifecycle_phase`, `severity_choice_calibrated`, `degraded_mode_response`). The connection-lifecycle P-001..P-004 family does not add new envelope fields; the `expected` block mirrors what report-generation already validates.

3. **Determinism-preserving fixture test (scope.md DoD: "fixtures mirror error-baseline-spike"):** Round-trip the connection-lifecycle TOML(s) through `Scenario::from_toml_str` → `timeline.schedule_under_seed(start_paused=true)` → assert the `PhaseTimeline` shape matches golden (determinism gate); no MCP execution or live read-back, just phase ordering + timing invariants.

## Anti-patterns to avoid — domain bans that apply

1. **Never introduce OTel SDK or metrics backend for scenario instrumentation (obs-plan §11 anti-patterns / §6 SLO):** The `expected` block is a TOML-level assertion schema, not an instrument or exporter. Do NOT wire `opentelemetry::metrics`, histogram creation, or any batch-task machinery into scenario deserialization or timeline execution. Performance budget is the JSON field assertion (`latency_ms <= slo_tier_threshold`) at report time, not a live histogram.

2. **Never skip garde validation or allow unlogged panics on model deserialization (obs-plan §10 zero-unlogged-panics + §11 error-reporting bans):** If scenario TOML parsing fails (e.g., `expected` field malformed, `slo_tier` out of range, cross-field invariant violation), the `garde` validation must fire as `tracing::error!(...)` + `anyhow::Error`, not bubble as a raw panic. The fixture test exercises invalid `#[case]` rows to prove this.

3. **Never leak absolute paths or struct names when serializing `expected` block to logs (obs-plan §11 logs anti-pattern):** The `expected` block fields (e.g., `verdict: Pass`, `latency_ms_threshold: 5000`) must survive redaction because they are allowlisted envelope fields. Do NOT use `Debug` derive on `ExpectedOutcome` or any sub-field; use `Display` at the `anyhow` edge and rely on the field-name allowlist to drop non-standard fields.

## Contract bindings — where your domain ties into another

**obs ↔ tests harness (obs-plan §3 observability harness contract / tests excerpt §5):** The fixture round-trip test (scope.md DoD) produces a deterministic `PhaseTimeline` that the tests harness consumes to validate log schema conformance + SLO assertion. The JSONL envelope emitted by report-generation for a successful P-001..P-004 run must carry `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms` (computed `read_back_observed_at - journal_emitted_at`), `slo_tier` (mapped from `expected.latency_ms_threshold`), `fingerprints` (empty for lifecycle), and scenario-specific fields if any (e.g., no `bypass_triggered` for connection-lifecycle; that is restart-suppression only). The tests harness reads this schema from runs.db / `logs/agent-latest.jsonl` and asserts its shape matches the binding contract.

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. **(obs) Scenario TOML deserializes with `expected` block wired:** The connection-lifecycle TOML(s) for P-001..P-004 carry an optional `[expected]` block (or equivalent inline field) that round-trips through `Scenario::from_toml_str` + `garde` validation without panic; fixture test `#[case]` rows cover valid and invalid states (e.g., `slo_tier` out of range, latency threshold negative).

2. **(obs) SLO tiers chosen per capability-spec windows:** Each P-ID maps to a tier — P-001 (Receiving ≤1s) and P-004 (ReceiverFailed ≤2s) use `<5s` tier; P-002 (Idle) and P-003 (Stalled long windows) map per the spec's thresholds (documented in scope.md open question #1; resolve in planning). The fixture test asserts the tier enum values round-trip correctly.

3. **(obs) JSON report envelope conforms to binding schema:** A fixture scenario run (no live MCP; `start_paused=true` determinism mode) generates a `runs.db` row with all 11+ envelope fields (`journal_emitted_at`, `read_back_observed_at` [null for blocked], `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`); no additional scenario-specific fields introduced for P-001..P-004 (those land in other families). The `read_back_observed_at` field is populated by a dry-run verify mock in the fixture (not live MCP; per scope.md boundaries).

4. **(obs) Critical path spans instrumented:** The fixture timeline execution emits `scenario.run` (root, tagged with `run_id`, `seed`, `scenario`, `p_ids`) → `timeline.execute` → `emit.batch` spans (possibly with distinct `path_type` attribute for P-004 bypass) → `report.generate` → `db.insert_run` (with `state_written` attribute). These are present in the `tracing` call hierarchy and render in `logs/agent-latest.jsonl` as JSON events; agent can parse and verify presence via `jq` filtering for span names.

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why

**(none)** — The amendment log (obs-plan-amendments.md) documents changes to the obs-plan body post-freeze; no amendments touch scenario authoring or model extension specifically. The three amendments (structured-logging-stack 2026-06-15, log-error-boundary-redaction 2026-06-15, emission-journal-writer 2026-06-16, raw-otlp-message-scaffold 2026-06-17, severity-logs 2026-06-18) all clarify upstream infrastructure (logger stack, schema shape, redaction model, SDK invariant, span naming) that this chunk inherits but does not amend. The connection-lifecycle scenarios chunk lands in Epoch 7 (after all Phase 2 amendments closed); it consumes the stable obs-plan, not modifying it.