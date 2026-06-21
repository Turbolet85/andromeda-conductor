# obs extract

## Relevance
Partial — hard-signals scenarios (P-005..P-008) are pure-egress declarative TOML config; instrumentation for error-spans, exception-events, severity-logs, and root-vs-child linkage already exists from Epoch 3 (not reimplemented here).

## Constraints
- **Span naming:** `{module}.{operation}` pattern; only the bounded named set in §4 (`emit.batch`, `emit.logs_batch`, `verify.readback`, `report.generate`, etc.) — no high-cardinality span names (obs-plan §4 Anti-Patterns; 2026-06-18-severity-logs amendment bounds `emit.logs_batch`).
- **Hard vs CalibrationRegion class mapping:** P-005..P-007 carry `class = "Hard"`; P-008 carries `class = "CalibrationRegion"` due to the v2.1 amendment (root-vs-deep weighting is model-side per P-020) — pulse-capability-spec §2 amendments + obs-plan §4.
- **No OTel SDK init:** self-observation is structured tracing JSON only (JSONL via `tracing-subscriber`); the emit primitives are the PRODUCT sent to Pulse on `:4317`, NOT self-instrumentation (obs-plan §3; 2026-06-17-raw-otlp-message-scaffold amendment).
- **Run-report envelope fields:** every scenario result line carries the 11-field envelope (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) — obs-plan §3 Log format + 2026-06-16-emission-journal-writer amendment.
- **Scenario-specific extensions:** P-008 adds NO new envelope field (root-vs-deep is encoded in relative tendency, not an explicit field); resolve scope open question #4 before implementation — obs-plan §6.
- **No new emit primitive:** all hard-signal primitives already exist from Epoch 3 (scope "What it builds").

## Patterns to follow
- **Scenario TOML grammar:** `name · p_ids · seed · slo_tier · jitter_ms · [[phases]] {name, gap_ms} · [[expected]] {kind, class, expected}` — mirror ch1, distinct TOML per P-ID (scope "Existing precedent" + open question #1).
- **Fixture round-trip tests:** prove each TOML deserializes + garde-validates + builds a valid `PhaseTimeline` (no scheduler changes); mirror ch1 (scope "Definition of done").
- **Determinism:** `start_paused` gating ⇒ same seed ⇒ same stream shape; SLO tier `<5s` fits the 500ms-p99 budget (scope "Boundaries").

## Anti-patterns to avoid
- **High-cardinality span names:** no per-scenario/per-P-ID dynamic span names; use the bounded set (obs-plan §4 Anti-Patterns; 2026-06-18 amendment).
- **OTel SDK init in self-observation:** PRODUCT fault telemetry is OK; self-obs stays tracing JSON only (obs-plan §3; 2026-06-17 amendment).
- **Inventing new envelope fields:** the 11-field schema is fixed; scenario-specific extensions are pre-enumerated in obs-plan §6 — no new fields without a plan amendment.

## Contract bindings
- **obs ↔ tests harness**: the emission-journal JSONL + runs.db envelope fields (journal_emitted_at, read_back_observed_at, p_ids, verdict, state, latency_ms, slo_tier) are binding contracts from the test-plan owner; every hard-signals result must emit the 11-field envelope (obs-plan §3 binding to test-plan §3 + 2026-06-16 amendment).
- **obs ↔ pulse capability spec**: P-005..P-008 normative clauses (pulse-capability-spec §2) define the verification recipe + detection budget/boundary per P-ID; scenario TOML must respect phase gaps + SLO tier to achieve the budget (scope "Requirement source of truth").

## Acceptance criteria contributions
- (obs) Hard-signal scenarios (P-005..P-008) TOML deserialize + garde-validate + produce a valid `PhaseTimeline`; no dynamic span names outside the bounded set.
- (obs) P-005..P-007 expected checks carry `class = "Hard"`; P-008 carries `class = "CalibrationRegion"`; P-008 routes to `ManualCheck`; root-vs-deep tendency is NOT a new envelope field.
- (obs) Scenario result envelope (11 fields) emitted for each hard-signals scenario; no additional scenario-specific fields (P-007 two-sided boundary encoded within the existing `expected` block).
- (obs) SLO tier `<5s` respects the 500ms-p99 budget; determinism preserved under `start_paused`.

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified two record shapes (self-obs base line vs Run-report envelope); envelope is the §3 schema block.
- **2026-06-16-emission-journal-writer:** added `read_back_observed_at` to envelope (ISO-8601, null until read-back) — results carry both `journal_emitted_at` and `read_back_observed_at`.
- **2026-06-17-raw-otlp-message-scaffold:** no-SDK invariant clarified as behavioral; emit primitives are PRODUCT, not self-obs.
- **2026-06-18-severity-logs:** `emit.logs_batch` added to the bounded span-name set (P-007 severity-logs egress) — P-007 emission uses `emit.logs_batch`, no dynamic names.
