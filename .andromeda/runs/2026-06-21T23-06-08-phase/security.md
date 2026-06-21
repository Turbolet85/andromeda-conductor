# security extract

## Relevance
Relevant — hard-signals scenarios chunk introduces scenario config TOML files (P-005..P-008) that are untrusted boundaries per threat model.

## Constraints
1. All `Scenario` + `ExpectedCheck` TOML structs MUST derive `garde::Validate` with `range` rules on numeric fields + `#[garde(custom)]` cross-field assertions, collapsed to `ConfigError` via `#[from]` at parse time (per security-plan §Input Validation, four-boundary discipline).
2. P-008 scenario's `expected` checks MUST carry `class = "CalibrationRegion"` per the v2.1 amendment, never `class = "Hard"` — root-vs-deep weighting is model-side (P-020). Routes to `ManualCheck` via the default `Verdict → ReportState` mapping (per security-plan §Threat Model Summary).
3. No new inbound network listener or port bind introduced — the scope law "no new inbound bind" enforced (security-plan §Threat Model Summary attack-surface).
4. Bound parameters MUST be used for any `runs.db` writes of run_id/seed/fingerprints — no string interpolation, despite self-generated synthetic data (security-plan §Input Validation, rusqlite finding).
5. Error responses from scenario deserialization / garde validation failures MUST be sanitized (no absolute paths / internal struct names) at the `conductor-cli` / `#[tauri::command]` edges (security-plan §Error Handling).
6. Phase timeline construction MUST preserve determinism — same scenario+seed ⇒ same stream shape under `start_paused`; wall-clock stamps use `std::time::SystemTime`/`Instant`, not tokio virtual clock (security-plan §Logging & Monitoring; Conventions: Determinism discipline).

## Patterns to follow
1. Scenario TOML structure mirrors ch1 precedent: top-level `name · p_ids · seed · slo_tier · jitter_ms` + `[[phases]] {name, gap_ms}` + `[[expected]] {kind, class, expected}` (security-plan §Input Validation, Conventions: Config conventions).
2. Per-scenario SLO tier (<5s) aligns with hard-signal detection budget (500ms-p99 for P-005) — bounds-check duration fields via garde `range` (security-plan §Input Validation).
3. Fixture round-trip tests for each TOML prove deserialize + garde-validate + `PhaseTimeline` construction, mirroring ch1 fixture gate (security-plan §Bootstrap phases).
4. `ComparisonKind` selection reuses the existing enum — no new kind if the v2.1 amendment shape can be expressed declaratively (scope open questions #2, #4, #5).

## Anti-patterns to avoid
1. NEVER deserialize scenario config without garde validation at load — `#[derive(Validate)]` + cross-field rules ARE the trust boundary (security-plan §Input Validation, Security Anti-Patterns §Input).
2. NEVER interpolate scenario parameters (seed, p_ids, slo_tier) into SQL or argv — bound parameters for `runs.db`; config stays TOML-declarative (security-plan §Security Anti-Patterns §Input).
3. NEVER promote the P-003 port-occupier into a general inbound listener — this chunk is pure-egress only (security-plan §Threat Model Summary attack-surface).

## Contract bindings
- **security ↔ obs (log redaction)**: config deserialization / validation errors MUST NOT leak absolute path handles or internal struct names to `conductor-cli` stderr — sanitization coordinated at the error boundary (security-plan §Error Handling + §Logging & Monitoring; amendment 2026-06-15-structured-logging-stack).
- **security ↔ tests (fixture gate)**: scenario-config fixture round-trip tests MUST be present per the CI gate — no scenario TOML ships without a deserialize+validate+scheduler-route passing fixture (security-plan §Bootstrap phases; mirrors ch1).

## Acceptance criteria contributions
1. (security) All new scenario TOML files (P-005..P-008) deserialize + garde-validate + produce a valid `PhaseTimeline` — proven by fixture round-trip tests (green `cargo nextest`).
2. (security) P-008 scenario's `expected` checks carry `class = "CalibrationRegion"`, never `class = "Hard"` — verifiable across the TOML file.
3. (security) No new inbound listener or port bind introduced; loopback-egress-only attack surface preserved (security-plan §Threat Model Summary).
4. (security) Scenario deserialization errors surface as sanitized user messages (no absolute paths / struct names); `anyhow` edge collects detail (security-plan §Error Handling).

## Relevant amendment history
- **2026-06-15-config-validation-surface:** garde pinned 0.23.0 → 0.22.1 — validation contract unchanged; `#[derive(Validate)]` + `#[garde(custom)]` cross-field at load (the TOML boundary this chunk touches).
- **2026-06-15-structured-logging-stack:** `CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV` are non-path string labels requiring no validation — distinct from `CONDUCTOR_*` path handles that canonicalize+bounds-check at the CLI edge.
- **2026-06-21-runs-db-index:** bundled SQLite 3.50.4; audit-green with committed `Cargo.lock`. Relevant if fixture tests write to `runs.db` — bound-parameter discipline enforced (security-plan §Input Validation).
