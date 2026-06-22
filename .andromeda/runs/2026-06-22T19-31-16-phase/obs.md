# obs extract

## Relevance — partial
This chunk authors severity-lifecycle scenario TOML using the EXISTING obs surface; the obs-plan's span/log instrumentation for Scenario 4 is Epoch-8 runtime work, NOT this chunk (this chunk emits no spans). The actionable obs slice here is the run-report-envelope schema + SLO-tier field + determinism/redaction discipline.

## Constraints
- **SLO tier assertion (§5, §10):** three-tier bucketing per P-060: `<5s` (Tier-1 hard signals, 5000 ms), `<20s` (Tier-2 medium cues conf 0.7–0.85, 20000 ms), `<90s` (Tier-3 baseline, 90000 ms); measurement = `read_back_observed_at − journal_emitted_at` (wall-clock `std::time::SystemTime`, never tokio virtual clock); asserted at report-generation (JSON field) (obs-plan §5/§10).
- **Log schema (§6):** every scenario-result JSONL row carries the base envelope fields (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) (obs-plan §6).
- **Verdict class split (§ aligned to arch Probabilistic-Assertion Policy):** CalibrationRegion for severity choice (P-019/P-020) + interpretation continuity (P-059) → `ManualCheck`; Hard for lifecycle timing (P-022 120 s, P-023 5-min) + tier→SLO routing (P-060) → Pass/Fail (obs-plan §6).
- **Span naming bounded set (§4) [Epoch-8, not this chunk]:** if/when runtime instrumentation is added, use the bounded set (`scenario.run`, `emit.batch`, `report.generate`, `db.insert_run`, the `verify.readback_*` family) — no per-P-ID or per-tier-value span names (cardinality discipline).
- **Redaction (§11):** no absolute host-file paths, no internal struct names in logs/journal — field-allowlist + Display-edge scrubbing (obs-plan §11).

## Patterns to follow
- Determinism preservation: same scenario + seed ⇒ same shape under `start_paused`; seed-named goldens re-baselined only if new TOML feeds them (grep the new seeds — expect UNCHANGED) (obs-plan §determinism alignment).
- Mixed-class TOML: `Hard` for timing + routing; `CalibrationRegion` for choice; expected checks reuse `Contains "Resolved"` / `CountAtLeast` (zero model change) (scope §Definition of done).
- SLO tier as a per-scenario field mapping P-060 Tier-1/2/3 → `<5s`/`<20s`/`<90s` (obs-plan §5).

## Anti-patterns to avoid
- High-cardinality span names (§11): NEVER per-severity-value span names; P-060 tier routing is a JSON field assertion, not a span-name variant.
- OTel SDK / metrics backend (§11): NEVER introduce an OTel meter/histogram for SLO tracking — Minimal tier; the budget is `latency_ms` + `slo_tier` JSON-field assertion only.
- W3C trace context for the multi-interpretation leg (§11): correlation is the `run_id` field; cross-surface parity is `runs.db` envelope comparison, NOT trace continuity.

## Contract bindings
- **obs ↔ tests harness (§3):** the run-report envelope (scenario-result record) matches the tests binding schema; `latency_ms` + `slo_tier` inherited from test-plan; runs.db + JSONL journal format binding-honored.
- **obs ↔ Epoch-8 evaluator:** the evaluator asserts `runs.db` envelope shape (verdict, state, latency_ms, slo_tier presence) at runtime; this chunk only declares the expected values in TOML.

## Acceptance criteria contributions
- (obs) Each severity-lifecycle scenario declares a `slo_tier` conforming to P-060's tier→budget mapping (`<5s`/`<20s`/`<90s`) (per obs-plan §5).
- (obs) Verdict class split honored in the TOML: CalibrationRegion → ManualCheck for P-019/P-020/P-059; Hard → Pass/Fail for P-022/P-023/P-060 (per obs-plan §6 + arch Probabilistic-Assertion Policy).
- (obs) No absolute host paths / internal struct names introduced via the new TOML or its test artifacts (per obs-plan §11 redaction).
- (obs) [Epoch-8 forward] runtime spans, if added, conform to the bounded span-name set (per obs-plan §4/§11).

## Relevant amendment history
- **2026-06-18-severity-logs:** `emit.logs_batch` added to the bounded span-name set (OTLP logs-egress self-observation, record_count attribute). Doc-reconciliation for P-007; bounded-set invariant preserved. Relevant only to Epoch-8 runtime instrumentation, not this TOML chunk.
