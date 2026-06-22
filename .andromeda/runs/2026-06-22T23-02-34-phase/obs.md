# obs extract

## Relevance
Partial — P-032 (known-residual) + P-036 (cross-incident fingerprint) are obs-instrumented scenarios; P-025/026/027 (constellation/hue/breathing) are visual operator-checklist (no obs instrumentation required per their ManualCheck verdict, but `run_id` correlation + JSONL envelope needed). Catalog only; no new emission primitives or instrumentation code.

## Constraints
- Per §3 OTel SDK init: no SDK initialization for self-observation; tracing-subscriber JSON formatter only (behavioral invariant, no batch tasks breaking `current_thread` determinism)
- Per §4 Must-trace spans: P-032 (known-residual path) requires `scenario.run` → `verify.readback_degraded_mode` → `report.classify_known_residual` → `db.insert_run` span chain with `degraded_mode_response` attribute
- Per §6 Log Coverage: Run-report envelope (11 fields: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) + P-032-specific `degraded_mode_response` field
- Per §4 Scenario P-025/026/027: spans close at phase boundary; no high-cardinality span names (bounded set: scenario.run, timeline.execute*, emit.batch, verify.readback*, report.generate, db.insert_run, tauri.command.*, fault.*)
- Per §10 SLO Invariants: zero unlogged panics (every panic captured + logged + converted to anyhow::Error at binary edge)

## Patterns to follow
- Scenario catalog: each TOML carries its P-ID, expected verdict (ManualCheck for P-025/026/027; KnownResidual for P-032; Auto pass for P-036), SLO tier (<5s / <20s / <90s); loadable + garde-valid (per §11 Anti-Patterns "garde validation on scenario config")
- P-032 verdict path: `state = "KnownResidual"` (never surprise Fail) with `degraded_mode_response` field on JSONL + runs.db; `verify.readback_degraded_mode` MCP call attribute logs method + latency_ms
- P-036 fingerprint recurrence: `runs.db` cross-run fingerprint index supplies "Previously seen" to `retrieve_report`; `fingerprints` array on JSONL must carry all touched fingerprints
- Run-id correlation: all P-IDs use `run_id` (YYYY-MM-DDTHH-MM-SS-<suffix>, filesystem-safe) on every JSONL line; no W3C trace context

## Anti-patterns to avoid
- NEVER add OTel SDK for self-observation (breaks `current_thread` determinism; creator-explicit ban §11 Telemetry Strategy)
- NEVER use high-cardinality span names — P-025/026/027 catalog must nest under `timeline.execute*` or `scenario.run`, not introduce per-scenario-variant span names
- NEVER leak absolute host paths or internal struct names in run-report JSONL — redaction layer masks `/home` / `/Users` / `%APPDATA%` / `~/.cargo` / `.rustup` / drive-letter paths; allowlisted `target` module path preserved (§11 PII Scrubbing)

## Contract bindings
- **Test harness § Emission Journal + Status Shape**: JSONL schema binding (11 fields); P-032 `degraded_mode_response` + P-036 `fingerprints` array must match test-plan §3 envelope expected on read-back
- **MCP read-back contract**: P-032 `retrieve_report` context section (expects run_id + degraded-mode flag in request; returns pre-accepted deviation indicator); P-036 `query_incident_list` recurrence check (expects cross-run `runs.db` fingerprint index populated)
- **Coverage matrix contract** (§4 Scenario 6): all five P-IDs (DriveObserve×3, Auto×2) must be queryable via `coverage.rs` class enum; no new `ComparisonKind` unless research proves minimal model addition needed (per scope "unless proven necessary")

## Acceptance criteria contributions
- (obs) All five scenario TOMLs declare expected verdict + slo_tier; P-032 carries `degraded_mode=true` flag; P-036 carries fingerprints list — per obs-plan §6 Run-report envelope.
- (obs) P-032 JSONL log includes `degraded_mode_response` field on report write; `state = "KnownResidual"` (enum validation) — per obs-plan §4 Must-trace spans + §6.
- (obs) P-036 JSONL log includes non-empty `fingerprints` array; cross-run `runs.db` fingerprint index queried at read-back time — per obs-plan §6 Log Coverage.
- (obs) `scenario.run` span root encompasses all five P-IDs; child spans close at phase boundary (no dangling spans per §11); no panic unlogged — per obs-plan §4 + §10.

## Relevant amendment history
**2026-06-15-structured-logging-stack** — clarified two record shapes: self-obs base line (`timestamp_ms`, `level`, `target`, service-identity, `run_id`) on every line; Run-report envelope (verdict, state, latency_ms, etc.) only on scenario-result events. **Impact on this chunk:** P-032/P-036 scenario results write to JSONL at report-seam (Epoch 6); envelope carries the scenario-specific fields (`degraded_mode_response`, etc.). Custom `tracing-subscriber` layer required to emit constant identity fields flat on every line (per §3 note).

**2026-06-15-log-error-boundary-redaction** — redaction model reconciled: absolute host-file paths → `<redacted>` (not `::`-token blanket); internal struct names kept out via field-name allowlist + Display-not-Debug at anyhow edge. **Impact on this chunk:** P-032 known-residual error context (e.g., "stub recent_commits producer") must NOT leak internal paths; allowlisted `target` module path preserved in all P-IDs' span attributes.

**2026-06-16-emission-journal-writer** — Run-report envelope gains `read_back_observed_at` (ISO-8601; null until read-back). Schema is now 11 fields. **Impact on this chunk:** P-032 `latency_ms = read_back_observed_at - journal_emitted_at` computed at report-write time; P-036 "Previously seen" logic waits for `read_back_observed_at` population before cross-run `runs.db` query (sequencing: emit → verify.readback → report.write envelope → db.insert with latency).