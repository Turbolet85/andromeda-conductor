# obs extract

## Relevance
Partial — this chunk adds two scenario TOMLs (activity-floor + restart-suppression) that feed critical-path instrumentation (§4 Scenarios 3-7), but contains no new telemetry signal or infrastructure; existing spans/logs apply.

## Constraints
- Per obs-plan §4: restart-suppression scenario spans MUST include `bypass_triggered` (boolean) field in JSONL log + `path_type` ("canonical" / "bypass") on emit.batch spans (Scenario 3); all outcomes are Hard class (§11 Anti-Patterns: "suppression/bypass logic" named explicitly as hard pass/fail).
- Per obs-plan §4 Scenario 2 + §6: activity-floor scenario (fingerprint-storm shape P-013) produces multiple emit.batch spans per P-ID cohort; verify span MUST emit `fingerprints_matched_count` attribute; fingerprints array MUST populate in JSONL.
- Per obs-plan §1: no new instrumentation depth added at Minimal tier — existing span names (`scenario.run`, `timeline.execute*`, `emit.batch`, `verify.readback*`, `report.generate`) cover the two families; bounded span-name set §11 enforcement (no high-cardinality per-seed or per-phase names).
- Per obs-plan §10: SLO tiers apply to positive/surfacing legs (P-014 cue ≈30s = `<90s`, P-015 restart ≤2s = `<5s`, P-016/P-057 surfacing ≈30s = `<5s`/`<20s`); absence legs (P-013 lunch non-alarm, P-016(a) 15s suppressed, P-057(a) suppressed) carry NO positive-event timestamp, so `latency_ms` is null for absence assertions — resolved by Epoch-8 evaluator comparison, not by SLO field.
- Per obs-plan §3 service-identity + §6 required fields: every JSONL line carries `run_id`, `seed`, `scenario` (scenario name per TOML), `service.name`/`service.version`/`deployment.environment` (inherited from CLI/Tauri bootstrap); no new identity wiring required.

## Patterns to follow
- Per obs-plan §4 span-coverage pattern: each must-trace scenario (3-7) specifies span hierarchy (root `scenario.run` → timeline.execute_<variant> → emit.batch (per cohort/path) → verify.readback_<variant> → report.generate); the two TOMLs' phases emit sequential emit/verify spans as siblings under the root.
- Per obs-plan §1 § Telemetry Strategy: fault-injection spans (conductor-faults) wrap each fault application (silence, ramp, port-occupier) as children of `timeline.execute`; the activity-floor/restart-suppression TOMLs compose existing fault references (no new fault primitives).
- Per obs-plan §9 CI Integration: both TOMLs feed `cargo-nextest` with artifact upload (`logs/agent-latest.jsonl`); determinism golden re-baseline triggered if seed values change (grep for new seed in the two TOMLs and reconcile against `crates/conductor-timeline/tests/snapshots/`).

## Anti-patterns to avoid
- Per obs-plan §11 Spans: NEVER introduce high-cardinality span names (e.g., per-scenario-instance, per-seed). The two TOMLs are config only — span names remain bounded (`timeline.execute_restart_suppression`, `timeline.execute_fingerprint_storm`), not parameterized per scenario load.
- Per obs-plan §11 Logs: NEVER leak absolute host paths or internal struct names in the TOML or any generated journal. The scenario names (P-013, P-015, etc.) and p_ids arrays are allowlisted identity fields — safe to emit. Phase names (train/lunch/gap/resume/suppress/bypass) are semantic descriptors, not Rust struct names.
- Per obs-plan §11 SLO: NEVER define soft SLO budgets. The chunk scope (Open Q5 in chunk scope.md) flags the absence-SLO ambiguity; Minimal tier closure: absence legs defer the SLO assertion to Epoch-8 evaluator (the timing window is the scenario phase duration, not a runtime measurement).

## Contract bindings
Obs ↔ tests harness: both-surface parity (CLI vs Tauri) verified by `runs.db` envelope comparison (same seed ⇒ same verdict/state); test-plan §5 Critical Path 7 binds the two surface logs to identical JSONL schema (per obs-plan §3 Harness Contract / Log format); the `run_id` correlation field on every JSONL line satisfies the tests binding (no W3C trace context).

## Acceptance criteria contributions
- (obs) Restart-suppression scenario JSONL includes `bypass_triggered` field (boolean) + `path_type` ("canonical" / "bypass") on emit.batch spans; verified by parsing `logs/agent-latest.jsonl` or CI artifact and asserting field presence + enum values.
- (obs) Activity-floor scenario (P-013/P-014) emits activity-floor-shaped timeline + emit/verify spans under existing hierarchy; fingerprints array populates for P-013 (fingerprint-storm variant per Scenario 2); verified by snapshot round-trip test (same seed ⇒ same span + log sequence).
- (obs) No new span names introduced; both TOMLs consume only bounded set (scenario.run, timeline.execute*, emit.batch, verify.readback*); verified by grepping `obs-plan §11` bounded span-name set against implementation.
- (obs) SLO tiers assigned per-scenario per obs-plan §10 (P-013 lunch leg = non-event ⇒ no SLO assertion; P-014 cue ≈30s = `<90s`; P-015 = `<5s`; P-016/P-057 surfacing = `<5s`/`<20s`); verified by asserting scenario.slo_tier field in TOML matches the tier-assignment table.

## Relevant amendment history
**2026-06-18-severity-logs:** `emit.logs_batch` added to bounded span-name set (obs-plan §11 Anti-Patterns / Spans). *Reason:* severity-logs chunk (P-007) shipped LogsEmitter egress instrumentation; routine doc-reconciliation. — *Applies here:* restart-suppression scenario may emit severity-transition logs (P-016/P-057 error-burst magnitude); if so, the `emit.logs_batch` span name is conforming (no further amendment needed).

---
ORCHESTRATOR NOTE (not part of the distiller return): the "fingerprint-storm shape P-013 / Scenario 2" framing
is a distiller confusion — P-013 is the activity-floor (bursty-train) capability, NOT exception fingerprinting
(P-017/P-018). Treat the `fingerprints_matched_count` / fingerprint-array claims for P-013 as NOT applicable;
the sound obs constraints for this chunk are: bounded span-name set, `run_id`/`seed`/`scenario` on every JSONL
line, no high-cardinality names, and SLO tiers govern only the positive/surfacing legs (absence legs ⇒ null
`latency_ms`). Verify the `bypass_triggered`/`path_type` field claims against obs-plan §4 in P3 before adopting.