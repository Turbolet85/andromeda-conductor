# obs extract

## Relevance
partial — the chunk adds no instrumentation of its own; obs binds through the real-model drive's envelope and self-obs shape, the harvest-tier grading boundary, host-path hygiene on committed evidence, and Pulse-side log lines read as evidence.

## Constraints
- The real-model posture adds NO critical path, span name or span attribute: the `real-model-interpretation` scenario rides Critical Path 1 with the standard eleven-field envelope, `verdict` null, a non-degraded read-back landing `ManualCheck` (a canary-blocked drive landing `Blocked`), and its three-storm canary adds only the existing message-borne `canary fingerprint computed` line (per obs-plan §4 Scenario: Headless deterministic scenario run — Real-model posture (2026-09-22)).
- Interpretation is graded at the harvest tier (`conductor-run/tests/real_model_harvest.rs`, over the scrubbed capture), never through the envelope — the envelope's `latency_ms`/`budget_ms` measure Conductor's journal-relative span, not anything Pulse-internal (per obs-plan §4 Real-model posture; §4 Known-residual classification path — Delegated-timing family).
- The journal carries two record shapes (the eleven-key envelope and the per-check `CheckRecord`); a declare-only scenario emits no check line, and envelope keys are required as PRESENT even when null (per obs-plan §3 Log format JSON schema "Two record shapes"; §6 Required fields).
- A span attribute or dedicated log field outside `conductor-core::redact::ALLOWLISTED_FIELDS` is dropped at the processor stage; anything new a harness or run-path change wants on a self-obs line rides the allowlisted `message` field (per obs-plan §4 Known-residual classification path — Required span attributes; §6 Boundary-call wrappers).
- Pulse's OWN tracing lines are the evidence surface at the harvest tier, and Pulse's default-deny allowlist has rendered newer fields as `"<redacted>"` live before — so whether `a2addb3`'s "no-incident L4 outcome is logged with its reason" line, and the `app.boot.workspace_key` line, carry their values unredacted at Pulse's committed HEAD is research's question, not an assumption (per obs-plan §4 Restart-suppression scenario — Required log fields; §4 Severity-lifecycle — Required log fields).
- Pulse coordinates cited in obs-plan are pinned to a HEAD and expire when it moves; this chunk's SUT HEAD (`a2addb3`) differs from every HEAD the plan names, so any Pulse-side line or field the capture or harvest reads is re-read at that HEAD (per obs-plan §4 Known-residual classification path — Delegated-timing family, closing sentence).
- A per-target `RUST_LOG` directive replaces rather than adds to the default and must be paired with the global level (`info,{crate}=debug`), never set for a run that also executes the test suite (per obs-plan §6 Per-module log levels).

## Patterns to follow
- Scenario-level posture mismatch is recorded inside `scenario.run` as a scenario `Blocked` with one `info` line on the allowlisted `message` field — never a sixth gate precondition (per obs-plan §4 Real-model posture).
- The 2026-09-29 series precedent: Stage B preflights reached ready, the graded drive's envelope landed `ManualCheck`/`verdict` null/eleven keys, canary-blocked drives landed `Blocked`, recorded in the chunk's `evidence/attempt-ledger.md` (per obs-plan §4 Real-model posture).
- Harvest-tier grading reads an exact leaf from the SUT's own lines and pins a retired or superseded reading as its own record rather than rewriting it (per obs-plan §4 Delegated-timing family — the `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency` pin).
- Key NAMES, never values, on the `message` field as the shape witness for read-back results, so a field-name divergence (e.g. a changed L4 schema) is distinguishable from an empty result (per obs-plan §6 Boundary-call wrappers — MCP readback).

## Anti-patterns to avoid
- NEVER leak absolute host paths or internal struct names in logs / run-report / `runs.db`, and never assume the three named redaction sites bound every host-path channel — the committed capture is a separate channel scrubbed by its own path (per obs-plan §11 Logs; §11 PII Scrubbing).
- NEVER widen the bounded span-name set or add a W3C trace context / OTel SDK for self-observation (per obs-plan §11 Spans / Traces; §11 Telemetry Strategy).
- NEVER add retry-once policies that mask real failures — the obs analogue of the chunk's no-retry-until-pass rule (per obs-plan §11 SLO; §10 Always-required SLO invariant).

## Contract bindings
- obs ↔ tests: the envelope's key presence is asserted by `conductor-run`'s `journal_conformance` (presence + closed sets + host-path freedom, never exclusivity), and the interpretation verdict belongs to the `real_model_harvest.rs` test tier (per obs-plan §3 Log format JSON schema; §4 Real-model posture).
- obs ↔ security: the capture enters committed `evidence/` only through the security-owned scrub chain; obs-plan's host-path ban and its "enumeration bounds the sites it names, never every channel" warning apply to that channel (per obs-plan §11 PII Scrubbing).
- obs ↔ SUT (Pulse): Pulse's tracing lines (`app.boot.workspace_key`, the no-incident-reason line) are read-only evidence at Pulse's committed HEAD; their redaction state under Pulse's allowlist governs what Conductor can grade (per obs-plan §4 Restart-suppression / Severity-lifecycle Required log fields).

## Acceptance criteria contributions
- Every drive's envelope record in `runs/<run_id>.jsonl` carries the eleven keys with `verdict` null and `state` `ManualCheck` (graded) or `Blocked` (canary-blocked), and no new envelope field, span name or span attribute is introduced by the chunk (per obs-plan §4 Scenario: Headless deterministic scenario run — Real-model posture; §6 Required fields).
- The series verdict is stated by the harvest test from the digest-pinned capture files, never from the envelope's `verdict`/`latency_ms` (per obs-plan §4 Real-model posture).
- No absolute host path appears in any committed artifact the chunk adds (captures, attempt ledger, contract section) or in the drives' self-obs lines (per obs-plan §9 Log conformance check; §11 Logs).
- Zero unstructured panic backtraces (`^thread.*panicked`) in `logs/agent-latest.jsonl` or stderr across the drives (per obs-plan §9 Zero-unlogged-panics gate; §10 Always-required SLO invariant).
