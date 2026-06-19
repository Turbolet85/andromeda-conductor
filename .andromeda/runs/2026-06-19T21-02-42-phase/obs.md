# obs extract

## Relevance
Partial — this chunk (P-015 gap/resume fault helper) adds a new bounded-profile instrumentation span, but does not itself drive the timeline/emit wiring or verification.

## Constraints
- **§3 Observability Harness Contract:** Self-observation is structured tracing logs only; no OTel SDK initialization for self-obs. The gap/resume helper produces NO telemetry (tracing span or log) internally; instrumentation binds to the timeline/emit boundary later (Epochs 7/8).
- **§4 Span / Trace Coverage, fault-injection spans:** `fault.silence` span (attributes: `fault_type`, `fault_duration_ms`, `fault_start_offset_ms`) belongs to the bounded span-name set; this helper seals the gap/resume *structure*, deferring the span emission to the timeline integration chunk.
- **§1 Instrumentation scope table, conductor-faults row:** Fault-injection helpers (ramps, silence, port-occupier, fingerprint generation) are instrumentable; telemetry surfaces as spans around each fault application phase (not the helper constructor itself).
- **§6 Log Coverage, per-module log levels:** `conductor-faults` base level is `info`; opt-in debug gating via `RUST_LOG=conductor_faults=info` (or debug if deeper tracing needed). Boundary-call wrappers must log MCP calls, gRPC emit, DB insert — but gap/resume (Epoch 4 primitive) does NOT call MCP/gRPC/DB; no boundary logging required here.
- **§11 Anti-Patterns (Spans/Traces — bounded span name set):** Only `fault.silence`, `fault.ramp`, `fault.port_occupier`, and `emit.logs_batch` are in the allowlisted span name set; high-cardinality fault identifiers (e.g., per-gap ID, per-instance index) are banned.
- **§4 Critical Path 3 (Restart-suppression scenario):** Observes the gap/resume via two `emit.batch` spans (canonical vs bypass paths); the gap/resume helper itself does NOT span at Epoch 4.

## Patterns to follow
- **Determinism contract preservation (§4 critical paths):** Same scenario+seed ⇒ identical gap/resume shape; respect the seeded `current_thread` tokio runtime (do not perturb exact gap duration with non-deterministic sleep/scheduling).
- **Verdict/error wall (§1 "Validation + typed errors"):** Invalid gap (≤ 20s floor) or malformed ordering ⇒ typed `FaultError::*`, never panic. Panics are unlogged-fault violations (§6 "zero-unlogged-panics gate" + §9 CI gate).
- **Agent-parseable error envelopes (§6 "Boundary-call wrappers"):** If the gap/resume helper surfaces an error (e.g., at timeline-integration time), it must be captured in a structured `tracing::error!(fault_silence_error=…)` log line matching the base JSON schema (service.name, run_id, timestamp_ms, level, target, message).

## Anti-patterns to avoid
- **Do NOT initialize an OTel SDK for self-observation** (§3 OTel SDK init, creator-explicit). The gap/resume helper is a value/constructor; it produces no telemetry itself. Instrumentation spans are emitted by the timeline/emit integration (Epochs 7/8).
- **Do NOT introduce high-cardinality span names or fault identifiers** (§11 bounded-set invariant). Span names must stay in `{fault.silence, fault.ramp, fault.port_occupier}`; do not create per-instance or per-fault-run span names.
- **Do NOT emit logs or spans from the Epoch 4 helper itself** — telemetry is deferred to the driven-under-timeline integration chunk (Epochs 7/8).

## Contract bindings
- **Obs ↔ Tests harness (§3 Observability Harness Contract → test-plan §3):** The emission-journal JSONL Run-report envelope (11-field schema per amendment 2026-06-16) is the binding surface; when the gap/resume fault is driven under the timeline, the `emit.batch` (canonical path, gap period) and `emit.batch` (bypass path) spans are correlated by `run_id` and emit structured log lines matching the base JSON schema. Tests harness will assert structured verdict/state fields on the scenario-result record (amendment 2026-06-16: `read_back_observed_at` is now present).
- **Obs ↔ Determinism contract (§4 critical path 3, restart-suppression scenario):** Exact gap length is reproducible (seed-stable); both-surface parity is asserted by `runs.db` envelope comparison (same seed ⇒ identical verdict/state), not by trace correlation.

## Acceptance criteria contributions
- **(obs) Exact gap/resume structure deterministic under seed:** Same scenario+seed ⇒ reproducible exact gap length; validated against the >20s floor guard.
- **(obs) No unlogged panics:** Invalid gap or ordering error ⇒ typed `FaultError` value with structured `tracing::error!(…)` log envelope (service.name, run_id, timestamp_ms, level, target); CI gate (§9) passes.
- **(obs) Bounded span name conformance (deferred to timeline integration):** When driven under timeline/emit (Epochs 7/8), gap/resume fault application MUST emit only `fault.silence` span name (from the bounded set §11), never a per-instance or high-cardinality variant.
- **(obs) JSON schema alignment on scenario-result:** When the restart-suppression scenario runs with this gap/resume helper, the JSONL Run-report envelope (11 fields per §3) and `runs.db` row MUST carry `bypass_triggered` field (boolean, amendment 2026-06-15 §6); latency_ms and verdict/state fields match the critical-path 3 binding.

## Relevant amendment history
- **2026-06-16-emission-journal-writer:** Run-report envelope now carries `read_back_observed_at` as the 2nd field (ISO-8601, null until read-back); schema is 11 fields. When the restart-suppression scenario runs with this fault helper, the `latency_ms` field is computed as `read_back_observed_at − journal_emitted_at` (wall-clock). This amendment realigned obs-plan §3 to the owner (test-plan) and the implemented `RunRecord`.
- **2026-06-15-structured-logging-stack:** Clarified that self-obs log lines (every `tracing` line from the helper's error path, if it fires) carry base set (timestamp_ms, level, target, service.name, run_id, deployment.environment) on every line; Run-report envelope fields (verdict, state, latency_ms, fingerprints) appear only on scenario-result records. The gap/resume helper Epoch 4 chunk does NOT emit Run-report records (no verdict/state yet); error handling uses the base self-obs line shape only.
- **2026-06-15-log-error-boundary-redaction:** Redaction model clarified: absolute host-FILE paths are scrubbed → `<redacted>`, NOT `::` tokens; internal struct names kept out by field-name allowlist + Display-not-Debug at `anyhow` edge. When gap/resume FaultError propagates to the timeline boundary, the error message MUST NOT leak absolute file paths or struct-name dumps; use `anyhow`'s Display impl or an allowlisted field name.
