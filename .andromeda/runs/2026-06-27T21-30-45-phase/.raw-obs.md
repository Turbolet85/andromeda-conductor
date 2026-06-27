# obs extract

## Relevance
relevant — The chunk drives critical paths (7 scenarios) against live Pulse, exercises must-trace spans (scenario.run → timeline.execute → emit.batch → verify.readback → report.generate), logs structured JSONL journal + runs.db + Markdown report, and verifies read-back fidelity via fingerprint match. All existing obs patterns (tracing, logging, PII scrubbing, error capture) apply in full, with no SDK escalation.

## Constraints

1. **Obs tier stays Minimal (per obs-plan §1 Obs Scope Summary):** no OTel SDK for self-observation, no metrics backend, no dashboards. Self-obs is tracing JSON only; OTLP is the PRODUCT (emit to Pulse), not the obs mechanism.

2. **No OTel SDK initialization (per obs-plan §3 OTel SDK init):** the behavioral invariant holds — self-obs never initializes the SDK. Emission (conductor-emit → Pulse) uses raw `opentelemetry-proto` (PRODUCT), not the SDK; verify (hand-rolled JSON-RPC) uses no SDK.

3. **Logging stack is tracing + tracing-subscriber JSONL (per obs-plan §3 Logging stack, amendment 2026-06-15-structured-logging-stack):** CLI dual sink (stderr + file `logs/agent-latest.jsonl` in `--agent-mode`); foundational §3 base-line schema on every line (`timestamp_ms`, `level`, `target`, service.{name,version,environment}, `run_id`); Run-report envelope (11 fields: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) on scenario-result records only.

4. **Must-trace path spans for all 5 live families (per obs-plan §4 Span / Trace Coverage, must-trace scenarios, amendment 2026-06-18-severity-logs):** scenario.run (root) → timeline.execute → emit.batch (per emission) → verify.readback (MCP call) → report.generate → db.insert_run; fault-injection spans (port-occupier in connection-lifecycle) with attributes `fault_type`, `fault_duration_ms`, `fault_start_offset_ms`.

5. **Verdict-first SLO latency is journal-relative wall-clock (per obs-plan §10 SLO Invariants, §5 Metric Coverage, amendment 2026-06-16-emission-journal-writer):** `latency_ms = read_back_observed_at − journal_emitted_at`; tiers <5s, <20s, <90s; hard Pass/Fail for deterministic claims, CalibrationRegion for model-interpretive; `Blocked` state for live transport/MCP/canary precondition failures (no panics).

6. **Error capture via std::panic::set_hook() + anyhow edge bridging (per obs-plan §7 Error Capture & Reporting):** live MCP/canary failures become typed Blocked/Fail log entries, never panics; CLI exit code 1 + sanitized error to stderr on unrecoverable harness faults.

7. **PII redaction: host-file-path anchor + allowlist + Display-not-Debug (per obs-plan §8 PII Scrubbing, amendment 2026-06-15-log-error-boundary-redaction):** pii-scrub scenario exercises redaction; no absolute host paths or internal struct names in logs; `target` module path preserved; allowlist enforced at logger config.

## Patterns to follow

1. **Span-attribute pattern (per obs-plan §4 Span / Trace Coverage, must-trace scenarios):** `scenario.run` root carries `run_id`, `seed`, `scenario`, `p_ids` (array); child spans (timeline.execute, emit.batch, verify.readback, etc.) carry operation-specific attributes; all spans closed on phase boundary.

2. **Read-back-as-observability-surface pattern (per obs-plan §1 Critical Path 1, §3 Correlation, scope Canary design):** MCP read-back call wrapped in `verify.readback` span; response latency recorded as span attribute; canary-fidelity check (fingerprint match in `retrieve_telemetry_slice` or incident fingerprint field) logged as span attribute or info-level boundary summary.

3. **Verdict-first state-transition logging (per obs-plan §6 Log Coverage, Boundary-call wrappers):** every state transition (run start, emission batch, read-back completion, verdict classification) logs at info level with verdict + state enum; final report logs complete 11-field envelope; no intermediate partial envelopes.

4. **Run-report envelope on every scenario-result (per obs-plan §3 Log format JSON schema, §6 Log Coverage):** JSONL journal `runs/<run_id>.jsonl` and runs.db row both carry full 11-field envelope; blocked rows carry `state: "Blocked"`, `latency_ms: null`, `read_back_observed_at: null` (scope Blocked-row NULL rule).

5. **Agent-mode flag as read-only trigger (per obs-plan §3 Observability Harness Contract § Agent-mode flag, amendment 2026-06-24):** `agent_mode = --agent-mode flag || CONDUCTOR_AGENT_MODE env-set` (read-only); file sink to `logs/agent-latest.jsonl` in agent mode.

## Anti-patterns to avoid

1. **No unlogged panics (per obs-plan §1 telemetry-triggers error-budget-SLO, §7 Error Capture, amendment 2026-06-15):** every panic captured via `std::panic::set_hook()`; structured error log with backtrace (file paths redacted); convert to `anyhow::Error` at binary edge (CLI exit code 1).

2. **No high-cardinality span names (per obs-plan §4 Span naming convention, §11 Anti-Patterns):** span names must be low-cardinality `{module}.{operation}` only; no per-P-ID, per-seed, per-user-input variants (e.g., NOT `emit.batch.P-001` or `verify.readback.canary-xyz`).

3. **No OTel SDK bootstrap or exporter (per obs-plan §3 OTel SDK init, amendment 2026-06-17):** no-SDK invariant is behavioral; never initialize OTel SDK for self-observation (batch tasks break `current_thread` determinism). OTLP to Pulse is raw `opentelemetry-proto` + tonic only; self-obs uses file/stderr sink only.

## Contract bindings

- **Readiness-gate contract (per obs-plan §3 Bootstrap phases, scope §1 canary bridge):** canary round-trip is a NEW readiness-gate predicate; `ready: true` on pass, or distinct `Blocked` with named precondition on fail (version mismatch, missing tool, canary fingerprint not found, timeout) — binds to test-harness readiness assertions.
- **Verdict-first SLO contract (per obs-plan §10 SLO Invariants, test-harness binding):** runs.db envelope carries `verdict` + `state` enum; tests assert verdict + tier.
- **Canary-fidelity verification (scope Canary design):** fingerprint match in `retrieve_telemetry_slice` or incident fingerprint is proof; logged as span attribute or boundary-call summary.
- **PII scrubbing contract (per obs-plan §8 PII Scrubbing, security-plan §Logging & Monitoring):** pii-scrub scenario verifies redaction layer removes absolute paths and struct names.
- **Agent-mode file-sink contract (per obs-plan §3 Logging stack, amendment 2026-06-24):** file sink to `logs/agent-latest.jsonl` carries §3 base-line schema on every line + §6 envelope on scenario-result records.

## Acceptance criteria contributions

1. (obs) Canary bridge emits fingerprint-storm, reads back via `retrieve_telemetry_slice`, verifies fingerprint match, logs fidelity as span attribute + info-level boundary summary (per obs-plan §4 verify.readback span, §6 Boundary-call wrappers).

2. (obs) Preflight readiness gate reaches `ready: true` after canary pass; or returns distinct `Blocked` state with named precondition (version / tool-missing / fingerprint-not-found / timeout) on any failure — no generic fail, no panic (per obs-plan §3 readiness, §7 error-capture, scope §1).

3. (obs) Live-run JSONL journal + runs.db row + Markdown report each carry the full 11-field envelope + scenario-specific fields; blocked rows have state='Blocked', latency_ms=null, read_back_observed_at=null (per obs-plan §3 §6 envelope, scope Blocked-row NULL rule).

4. (obs) All must-trace spans (scenario.run, timeline.execute, emit.batch, verify.readback, report.generate, db.insert_run, fault.port_occupier) carry correct attributes; span-name cardinality is low ({module}.{operation} only); all spans closed on phase boundary (per obs-plan §4 must-trace scenarios).

## Relevant amendment history

- **2026-06-15-structured-logging-stack:** clarified two record shapes (self-obs base line vs Run-report envelope). RELEVANT: this chunk persists BOTH — the self-obs stream (`logs/agent-latest.jsonl` with §3 base fields) AND the scenario-result envelope (runs.db + JSONL report with §6 full envelope).

- **2026-06-16-emission-journal-writer:** Run-report envelope gains `read_back_observed_at`. RELEVANT: live read-back transitions `read_back_observed_at` from null to MCP response timestamp; `latency_ms` computation depends on this field.

- **2026-06-24-sanitized-stderr-agent-mode-logging:** agent-mode flag reworded to read-only `CONDUCTOR_AGENT_MODE` trigger. RELEVANT: chunk uses `--agent-mode` or env var to force file sink to `logs/agent-latest.jsonl`.

- **2026-06-27-obs-ci-conformance-gate:** §9 conformance gate asserts §3 self-obs base schema (not §6 envelope). RELEVANT: if evidence artifacts are uploaded to CI, gate validates `logs/agent-latest.jsonl` carries §3 base fields, not §6 envelope fields (two record shapes are distinct).
