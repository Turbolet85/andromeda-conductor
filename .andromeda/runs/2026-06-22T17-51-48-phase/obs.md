# obs extract

## Relevance
Partial — the chunk declares scenario configs (TOML), not Rust instrumentation or backend harness; OBS provides span/log schema bindings for the scenarios' must-trace paths.

## Constraints
- Per obs-plan §1 Obs Scope Summary: Minimal tier (0), structured tracing logs via `tracing`/`tracing-subscriber` JSONL format only (no OTel SDK for self-observation, determinism constraint)
- Per obs-plan §4 Span / Trace Coverage: must-trace the fingerprint-storm scenario path — `scenario.run` (root) → `timeline.execute_fingerprint_storm` → `emit.batch` (per P-ID cohort, multiple) → `verify.readback_fingerprints` → `report.generate`; span attributes required: `run_id`, `seed`, `scenario` = "fingerprint-storm", `p_ids`, `batch_index`, `fingerprints_in_batch`
- Per obs-plan §6 Log Coverage: JSONL records must carry required base fields — `journal_emitted_at` (ISO-8601), `run_id`, `seed`, `scenario`, `p_ids` (array), `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` (array, populated for this scenario); additional scenario-specific fields per §4 (none defined for fingerprint-storm beyond base)
- Per obs-plan §1 Telemetry triggers / "creator-explicit-telemetry": field allowlist + redaction layer applied on journal write — no absolute host paths, no internal struct names
- Per obs-plan §4 Must-trace paths / Fingerprint-storm: `slo_tier` field populated; parity = fingerprints array populated (non-empty) versus distinct-fp scenarios (empty); outcome-coherence (same-fp vs distinct-fp not contradictory within single read-back) asserted via `expected` checks

## Patterns to follow
- Scenario TOML files declare `[[expected]]` checks keyed to `verify.readback_fingerprints` outcome (query_incident_list MCP call) — use `kind: "Contains" | "Absent"` with `class: "Hard"` (deterministic fingerprint identity, not model-interpretive)
- Deterministic counting: storm cue thresholds (6 hits / 30s → "Suggested", 12 hits / 30s → "Autonomous") are hard checks; captured in log envelope via `fingerprints` array size or canary MCP read-back field
- Per obs-plan §4 Fault-injection spans: not applicable to this chunk (fingerprint-storm is a scenario config, not a fault-injection scenario); emit spans close on batch flush; no chaos instrumentation needed

## Anti-patterns to avoid
- Per obs-plan §11 (Anti-Patterns / Spans): high-cardinality span names — do not emit per-fingerprint-id or per-user-id span names; use low-cardinality `emit.batch` with `fingerprints_in_batch` attribute instead
- Per obs-plan §3 OTel SDK init: no OTel SDK initialization for self-observation; determinism constraint precludes batch-exporting background tasks
- Per obs-plan §6 Log conformance: do not include absolute host file paths (drive letters, `/home`, `/Users`, `%APPDATA%`) or internal struct names in log fields; apply redaction layer via `conductor-core::redact`

## Contract bindings
- **obs ↔ tests harness**: JSONL Run-report envelope (§3 / §6 Log format binding contract from upstream test-plan §3) — logs.agent-latest.jsonl artifact parsed by CI gate; per obs-plan §9 CI Integration, agent reads JSON, validates schema presence (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints), asserts no host paths, gates build
- **obs ↔ architecture / Pulse**: MCP read-back verb `retrieve_telemetry_slice` or `query_incident_list` (Pulse's living storm/dedup detection); fingerprints field in the envelope corresponds to MCP response's incident.fingerprints array; canary checks vs live Pulse behavior verified at Epoch-8 runtime

## Acceptance criteria contributions
- (obs) Scenario TOML `[[expected]]` checks for fingerprint-storm path are all `class="Hard"` (deterministic identity + counting, no CalibrationRegion)
- (obs) JSONL log records populate `fingerprints` array (non-empty for same-fp and distinct-fp scenarios, empty for non-fingerprint scenarios); schema validation gates CI
- (obs) Span nesting: `scenario.run` → `timeline.execute_fingerprint_storm` → `emit.batch` (per cohort) → `verify.readback_fingerprints` spans close on MCP response; no per-fingerprint-id high-cardinality names
- (obs) Redaction layer applied: no absolute paths or internal struct names in log output; `target` module paths preserved per 2026-06-15-log-error-boundary-redaction amendment

## Relevant amendment history
- **2026-06-15-log-error-boundary-redaction** (§6 Log conformance check, §11 Anti-Patterns): redaction model anchored to absolute *file* paths (not `::`-token redaction); struct names kept out by field allowlist + Display-edge; `target` module paths explicitly preserved. Relevant: the chunk's JSONL output will apply this redaction (no host paths leaked in any field).
- **2026-06-18-severity-logs** (§11 Anti-Patterns / Spans): added `emit.logs_batch` span name to bounded set, alongside `emit.batch`. Not directly applicable to fingerprint-storm (which uses `emit.batch` for trace egress, not logs), but documents the bounded-set invariant that fingerprint-storm spans must conform to (low-cardinality span names only).

## NOTE (orchestrator, build-sequencing per playbook D-obs-instrumentation)
The §4 must-trace path (`timeline.execute_fingerprint_storm` / `verify.readback_fingerprints` spans) is the Epoch-8 driver's traced OPERATION, NOT this scenario-config chunk's deliverable. Per the playbook rule confirmed 2026-06-22 (generalizes the seam-primitive deferred-span dismiss to scenario-config chunks), this chunk adds CONFIG, not the runtime op — the §4 spans land in the wiring epoch. Carried as an Epoch-8 obs touchpoint, not an obligation of this chunk.