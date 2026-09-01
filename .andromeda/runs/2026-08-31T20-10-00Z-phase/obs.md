# obs extract

## Relevance
Relevant — the chunk fires an MCP read-back boundary call for the first time (`mark_incident_resolved`) and maps a JSON-RPC error arm to a typed verdict, both of which are instrumented surfaces under obs-plan §4/§6/§11.

## Constraints
- The lifecycle call is a read-back boundary, so it must carry a span from the **bounded** span-name set — `verify.readback*` under the `{module}.{operation}` convention; a new family-specific span name outside that set is banned (per obs-plan §4 "Span naming convention" and §11 Spans/Traces "bounded span name set"). Whether the shipped client already opens such a span at `client.rs:148` is research's question.
- A span attribute must be a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it — so any `incident_id` / applied-declined attribute is emitted only if allowlisted; otherwise the evidence rides the allowlisted `message` field (per obs-plan §4 Known-residual "Required span attributes" heading).
- §6 Boundary-call wrappers requires the MCP-readback must-log set — tool name (the enumerated list already names `mark_incident_resolved`), `latency_ms`, error if any, canary-check result, and the **observed key set** of the raw tool result (key NAMES only, never values, on `message`) (per obs-plan §6 "Boundary-call wrappers (must-log events)"). Whether the code already emits this for this tool is research's question.
- The declined arm is a recoverable boundary error, not a fault: it maps to `warn` level (§6 Log levels: "MCP call timeout, degraded-mode response") and MUST NOT panic or escape as an unstructured stderr line — malformed/refused read-back becomes a typed `Ok(...)` verification value (per obs-plan §11 Error Reporting, "verdict/error wall", and §10 zero-unlogged-panics).
- The journal envelope stays the eleven fields owned by test-plan §3; obs derives and does not re-author. A lifecycle extra (e.g. `resolve_applied`) would be a scenario-specific ENVELOPE extra written by the report seam, governed differently from a span attribute (per obs-plan §3 "two record shapes" + §6 "Additional scenario-specific fields").
- `latency_ms` is wall-clock `read_back_observed_at - journal_emitted_at` from `std::time::SystemTime`, never tokio's virtual clock, and it bounds Conductor's MCP round-trip — not any Pulse-internal duration (per obs-plan §5, §10 Performance budgets, §11 project-specific bans).
- Every self-obs line, including whatever the new leg emits, must carry the base set `timestamp_ms` · `level` · `target` · `service.{name,version,environment}` · `run_id`, with no absolute host paths (per obs-plan §3 Log format / §9 Log conformance check).

## Patterns to follow
- The `verify.readback.observe` pattern from the Known-residual path: ONE read-back pass whose result REPORTS the signal (Pulse computes it), observed rather than requested, `count` as the allowlisted attribute and the qualitative signal on a `warn`-level `message` line (per obs-plan §4 Known-residual classification path).
- The shipped read-back span field set is `mcp_tool` + `latency_ms` on `verify.readback` — reuse those names rather than minting new ones (per obs-plan §4 Critical Path 1 span attributes).
- Harvest-tier grading over Pulse's own tracing lines, each read from its OWN exact allowlist leaf, is the established home for evidence Conductor does not itself compute (per obs-plan §4 Known-residual "Delegated-timing family" and the restart/severity rows in §1 Critical paths).
- Declare-only / non-gradeable outcomes land as `verdict` null + `state` "KnownResidual" on the standard envelope, adding no critical path of their own (per obs-plan §1 Critical paths rows 3-4 and §4 Known-residual path).
- Per-check grain, if the leg emits checks, is the report seam's `CheckRecord` line (`run_id` · `scenario` · `check_index` · `kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms`) — a finer grain beneath the envelope, never an extension of it (per obs-plan §3 "two record shapes").

## Anti-patterns to avoid
- NEVER mint a family-specific span chain for this round (e.g. `verify.readback_resolve_lifecycle`, `verify.readback_declined`) — the bounded span-name set is the rule, and two prior families had exactly such never-built chains retired (per obs-plan §11 Spans/Traces).
- NEVER let a declined/malformed MCP response panic, become a harness `Err`, or reach stderr as an unstructured line — structured JSON only, typed verdict at the wall (per obs-plan §11 Error Reporting + §11 Logs "NEVER use unstructured stderr text").
- NEVER put an incident identifier into a span NAME, and never log raw tool-result VALUES (key names only) — high-cardinality span names and value leakage are both banned (per obs-plan §11 Spans/Traces + §6 Boundary-call wrappers).

## Contract bindings
- **obs ↔ tests:** the eleven-field Run-report envelope and the `CheckRecord` line shape are owned by test-plan §3; obs §3/§6 reproduce them and must not add fields unilaterally — the harvest-tier assertions in `conductor-run` consume these shapes (per obs-plan §3, §6).
- **obs ↔ security:** redaction is single-location in `conductor-core::redact` (field-name allowlist + host-path value scrub); any new field this leg wants emitted must be admitted there, and report-seam records do not pass through it (per obs-plan §11 PII Scrubbing, §3 two record shapes).
- **obs ↔ SUT (Pulse) telemetry:** harvested Pulse counters can read `"<redacted>"` live because Pulse's default-deny allowlist predates newer fields — an obs-plan-recorded measurement constraint that applies to any harvest-tier evidence this leg reads (per obs-plan §1 Critical paths rows 3-4).

## Acceptance criteria contributions
- The `mark_incident_resolved` call and the follow-up `query_incident_list` re-read are covered by a span whose name is inside the bounded `verify.readback*` set, with attributes drawn only from `ALLOWLISTED_FIELDS` (per obs-plan §4 Span naming + §11 Spans/Traces).
- The boundary log for each of those calls records tool name + `latency_ms` + error-if-any + the observed key set of the raw result, key names only on `message` (per obs-plan §6 Boundary-call wrappers).
- The declined arm emits a `warn`-level structured line and a typed verdict/state; no `^thread.*panicked` line and no unstructured stderr appears in `logs/agent-latest.jsonl` for the leg (per obs-plan §10 zero-unlogged-panics + §9 Zero-unlogged-panics gate).
- The leg's journal lines still validate against the self-obs base schema and the unchanged eleven-field envelope, with no absolute host paths and no new envelope extra unless the report seam deliberately writes one (per obs-plan §3 Log format + §9 Log conformance check).

## Relevant amendment history
- **2026-08-13-per-check-read-back-extraction** — retired `verify.readback_degraded_mode` for `verify.readback.observe`, renamed `mcp_method` → `mcp_tool`, and added the standing rule that a span attribute outside `ALLOWLISTED_FIELDS` emits nothing. Directly governs how this chunk's new read-back call may be attributed.
- **2026-08-13-first-live-green-preflight** — added the observed-KEY-SET shape witness to the read-back must-log set (names only, on `message`) because the extraction readers degrade to empty rather than erroring; the same ambiguity applies to a first-ever `mark_incident_resolved` result shape.
- **2026-08-18-restart-suppression-live-proof** and **2026-08-21-severity-lifecycle-live-proof** — both retired never-built family-specific span chains and their envelope extras (`bypass_triggered`, `lifecycle_phase`, `severity_choice_calibrated`) after measurement; the precedent is to instrument with the shipped generic chain and grade family evidence at the harvest tier, not to mandate new names.
- **2026-08-21-delegated-timing-budgets-proven** — recorded harvest-tier families under the Known-residual path with no new critical-path row, each reading its OWN exact allowlist leaf and never `budget_ms`/`effective_deadline_ms`; the same placement logic applies if this round's lifecycle leg lands as a harvest test.
- **2026-08-21-per-check-latency-measurement** — recorded the report seam's second journal line shape (`CheckRecord`) as a finer grain beneath the envelope, format owned by test-plan §3; relevant if the lifecycle leg produces per-check rows.
