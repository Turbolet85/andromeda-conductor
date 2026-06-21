# security extract

## Relevance
Relevant — chunk introduces scenario TOML config with new serde+garde-validated fields (`expected`/`holds` outcomes) and references existing port-occupier fault primitive.

## Constraints
1. All new scenario-config TOML fields (`expected`, optional `holds`) MUST deserialize via serde + validate via garde (security-plan §Input Validation; config-validation-surface amendment — garde 0.22.1); cross-field invariants (`p50≤p95≤p99` if applicable, outcome enum bounds) validated at load.
2. Port-occupier bind on `:4317` MUST remain loopback-only and intentional-fault scoped — never promoted to a general-purpose inbound listener (security-plan §Anti-Patterns § API, §Threat Model Summary § Attack surface — port-occupier exception).
3. Scenario TOML files must carry `p_ids` fields; no scenario without a P-ID (security-plan §Anti-Patterns § Universal — "NEVER add a scenario without a Pulse P-ID").
4. `CONDUCTOR_*` path env overrides (if used during scenario testing) MUST canonicalize + bounds-check at `conductor-cli` edge before any `runs.db`/manifest access (security-plan §Input Validation, §Anti-Patterns § Input).
5. Run-report artifacts (`<run_id>.md`, `runs.db` rows) MUST NOT leak absolute paths (canonicalized `CONDUCTOR_*` dirs, `ANDROMEDA_PULSE_DATA_DIR`) or internal struct names (security-plan §Error Handling).
6. Scenario-config serde deserialize MUST NOT panic on malformed input — route to `ConfigError` (harness fault via garde `#[from]`), never a panic (security-plan §Error Handling, §Anti-Patterns § Universal).

## Patterns to follow
1. Reuse existing `scenarios/error-baseline-spike.toml` TOML shape (`name · p_ids · seed · slo_tier · jitter_ms · [[phases]]`) and extend it with serde+garde `expected`/`holds` fields (scope.md § existing precedent).
2. Scenario payload validated entirely at load via `Scenario::from_toml_str` → `PhaseTimeline`; timing windows (Receiving ≤1s, ReceiverFailed ≤2s, Idle/Stalled boundaries) expressed as declarative `expected` blocks with SLO tiers (scope.md § requirement source + drive+observe split).
3. Type-erased `anyhow` errors at the `conductor-cli` binary edge collapse from internal `thiserror` / `garde::Report` — sanitize stack traces/paths before stderr output (security-plan §Error Handling).

## Anti-patterns to avoid
1. NEVER interpolate scenario-config strings into argv or shell when spawning the port-occupier or MCP child; pass all operator input via `.env(...)` builder or direct enum/type-safe fields (security-plan §Anti-Patterns § Input, Code Patterns — rmcp STDIO design flaw CVE-2026-30623).
2. NEVER deserialize scenario TOML without garde validation at load — the `expected` outcome fields and any cross-field SLO invariants are the trust boundary (security-plan §Anti-Patterns § Input).
3. NEVER expose scenario-specific error detail (P-ID mismatch reasons, validation failure stack traces, internal `Scenario` struct names) in run-report artifacts or `conductor-cli` stderr (security-plan §Anti-Patterns § Logging, § Error Handling).

## Contract bindings
- **obs ↔ tests harness:** scenario TOML fixture round-trip test (scope.md definition-of-done) MUST exercise garde validation (valid + invalid `#[case]` rows); test fixtures must carry synthetic telemetry only (no real PII) — tests-harness §CI Integration (cross-cutting PII scrubbing rule).
- **design ↔ scenario:** lifecycle badge / hue visual verdict routes to operator checklist (drive+observe split); design contracts on ManualCheck routing apply.

## Acceptance criteria contributions
1. (security) All new `Scenario`/`PhaseSpec` TOML serde fields include `#[serde(...)]` + `#[garde(...)]` derives; garde `#[case]` fixture tests verify both valid + invalid payloads reject/accept as expected (grep for `#[garde(...)]` on new fields).
2. (security) Port-occupier bind remains scoped to `conductor-faults` fault primitive (P-003 ReceiverFailed leg) — no new inbound listeners introduced; loopback-only comment + verify via grep/static analysis.
3. (security) Scenario TOML files contain `p_ids` field; audit `scenarios/connection-lifecycle*.toml` for presence + non-empty vec.
4. (security) Run-report artifacts (`<run_id>.md` row, `runs.db` verdict rows for this scenario) do NOT contain absolute host paths or internal struct names — audit fixture golden output vs. canonicalized path inspection.

## Relevant amendment history
- **2026-06-15-config-validation-surface:** garde pinned 0.22.1 (was proposed 0.23.0, unbuildable); affects this chunk's `#[derive(Validate)]` syntax + dependency pinning.
- **2026-06-15-structured-logging-stack:** obs identity env-handles (`CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV`) are non-path string labels, requiring no validation — distinct from `CONDUCTOR_*` *path* handles (canonicalize-and-bounds-check remain the boundary).