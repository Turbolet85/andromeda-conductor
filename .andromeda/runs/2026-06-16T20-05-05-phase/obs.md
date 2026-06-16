# obs extract

## Relevance
Relevant — the chunk defines declarative scenario-config model that feeds into timeline execution, which produces the instrumented critical paths required by obs.

## Constraints
- Per obs-plan §1 Instrumentation scope: conductor-core is instrumentable; scenario-config model extends conductor-core and must preserve its instrumentation surface (no breaking changes to existing spans).
- Per obs-plan §3 Logging stack: all config parsing/validation failures surface as `ConfigError`/`CoreError` (harness `Result::Err`), never as verdicts; the deterministic config→timeline conversion never panics.
- Per obs-plan §6 Log conformance: scenario-specific fields (`bypass_triggered`, `lifecycle_phase`, `severity_choice_calibrated`, `degraded_mode_response`) must carry their scenario identity in the config model to enable proper field schema emissions downstream.
- Per obs-plan §4 Span coverage: the `Scenario` → `PhaseTimeline` conversion must be order-preserving and total; phase sequence ordering directly affects the deterministic timeline execution that spans correlate with `run_id`.
- Per obs-plan §1 Telemetry triggers (creator-explicit-telemetry): the per-phase emission spec is declarative data — the spec shape must be forward-compatible with obs' field-allowlist redaction layer (`conductor-core::redact`; no absolute paths in phase identity/kind strings).
- Per obs-plan §2 Naming conventions: phase spans follow `{module}.{operation}` pattern; the scenario config model must not introduce high-cardinality phase names (no per-user-ID, per-trace-ID, or unbounded user-input strings in phase identity).

## Patterns to follow
- Per obs-plan §3 Logging stack: two-record-shape model (2026-06-15 amendment: base self-obs line carries `timestamp_ms`, `level`, `target`, service-identity, `run_id`; Run-report envelope carries verdict/latency/state fields only on scenario-result events). The phase config model must carry fields that enable the report seam (Epoch 6) to populate envelope fields (`scenario`, `p_ids`, `lifecycle_phase`, etc.) from the config + execution state.
- Per obs-plan §4 Must-trace scenarios: each critical path (fingerprint-storm, restart-suppression, severity-lifecycle, known-residual) specifies required span attributes and log fields; the phase model must preserve the `scenario` string identity and `p_ids` array so spans can tag them deterministically.
- Per obs-plan §2 Agent-readable invariants: the config model shape must enable downstream code to emit machine-parseable JSONL (no ambiguous phase sequence, no implicit state transitions unobservable in the phase spec).

## Anti-patterns to avoid
- Per obs-plan §11 Anti-Patterns (Logs): do not store absolute filesystem paths in phase identity/kind fields; do not expose internal struct names or Debug-format dumps in the scenario config schema (preserve `Display` trait discipline). The config model must play nicely with the `conductor-core::redact` redaction layer (2026-06-15 amendment: absolute file paths → `<redacted>`, struct-name guard via allowlist + Display).
- Per obs-plan §4 Span naming: avoid high-cardinality phase identifiers (e.g., do not parameterize phase names by user-supplied strings; keep phase kinds to a closed enum: "canonical" / "bypass" / "escalation" / "plateau" / "resolution" as required per critical paths).

## Contract bindings
- **conductor-timeline SpanContext:** the `Scenario` → `PhaseTimeline` conversion must maintain span correlation by preserving `run_id` as a field the timeline executor inherits from scenario config (per obs-plan §3 Correlation: IPC/internal async uses `run_id`, not W3C `traceparent`).
- **Verdict/error wall (tests harness):** obs-plan §3 mandates config parsing/garde-validation failures surface as typed `Err` (ConfigError/CoreError), never as panics — the harness will gate CI on zero-unlogged-panics (per obs-plan §10 SLO Invariants). The config model must support garde `Validate` impls that fail gracefully (no `panic!` on invalid bounds/ordering).
- **Log conformance CI gate (obs):** obs-plan §9 CI Integration specifies agent validates `logs/agent-latest.jsonl` for schema binding (journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints); the scenario-config model must carry `scenario` (string identity) and enable `p_ids` (array) to flow through config → timeline → emit → report → JSONL binding.

## Acceptance criteria contributions
- (obs) Config-layer scenario model loads and garde-validates without panic; invalid bounds/ordering rejected as typed `ConfigError`/`CoreError` (not verdict), per obs-plan §3 error wall and §10 zero-unlogged-panics invariant.
- (obs) Per-phase emission spec carries required identity fields (`scenario`, `p_ids`) so downstream report seam (Epoch 6) can populate JSONL envelope fields per obs-plan §6 Log Coverage schema binding (journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints).
- (obs) Scenario-specific extra fields (`bypass_triggered`, `lifecycle_phase`, `severity_choice_calibrated`, `degraded_mode_response`) are expressible in the per-phase declarative spec so report generation can emit them without rework, per obs-plan §4 Critical Paths 2–5 required log fields.
- (obs) Scenario → PhaseTimeline conversion is total and order-preserving; identical (scenario, seed) produce identical phase sequence, enabling deterministic span correlation via `run_id` and enabling the both-surface-parity gate (obs-plan §4 Critical Path 7, tests harness).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified two-record-shape model (self-obs base line vs Run-report envelope). Per-phase config must preserve `scenario` + `p_ids` identity so report seam can populate envelope fields.
- **2026-06-15-log-error-boundary-redaction:** reconciled redaction model (absolute file paths → `<redacted>`; struct names guarded by allowlist + Display). Phase config strings must avoid absolute paths and be compatible with Display-not-Debug discipline at the config boundary.
