# security extract

## Relevance — partial

The chunk adds scenario catalog entries (TOML deserialization boundary) and scenario-model test wiring; no runtime engine code or new emission primitives. Input validation at the TOML deserialization boundary applies directly (per security-plan §Input Validation scenario-config row). Error handling for failed scenario loads applies. Dependency security remains in scope (no new dependencies expected). Secrets, auth, and API security remain out-of-scope (architecture unchanged).

## Constraints — domain rules that apply

1. Every scenario TOML carries a P-ID and must deserialize + garde-validate without error (security-plan §Input Validation, scenario-config boundary row: garde `#[derive(Validate)]` `range` rules on error fraction ∈ [0,1], non-negative durations, sane ramp factors; `#[garde(custom)]` cross-field p50≤p95≤p99, severity-mix sums).

2. Failed scenario config load (serde/garde) surfaces as `ConfigError` harness fault via `#[from]`, never panics; error returns `Result::Err` to indicate load failure (security-plan §Error Handling, Conventions: Error handling).

3. New scenario catalog entries must not introduce untrusted data paths or CLI-argument injection vectors beyond what the existing architecture defines (security-plan §Threat Model Summary attack vectors, scope law § "Conductor opens no new listener"; security-plan §Anti-Patterns § Input — "NEVER add a scenario without a Pulse P-ID").

4. Run-report artifacts (`<run_id>.md`, `runs.db` rows for P-032/P-036) must not leak absolute host paths (canonicalized `CONDUCTOR_*` directories, `ANDROMEDA_PULSE_DATA_DIR`) or internal seam-crate struct names; ManualCheck (P-025/026/027) and KnownResidual (P-032) states are agent-parseable ground truth (security-plan §Error Handling, run-report artifact sanitization).

5. Cross-incident fingerprint recurrence (P-036) stored in `runs.db` must use rusqlite bound parameters (never string-formatted SQL), even though fingerprints are self-generated synthetic data (security-plan §Input Validation, `runs.db` writes row; §Anti-Patterns § Input — "NEVER use string concatenation / `format!` to build SQL").

6. The determinism constraint on seed selection (scope § Determinism: "any seeds chosen must not collide with or perturb existing golden-fed seeds"; goldens are expected UNCHANGED) maps to scenario-seeding validation — if the plan requires a minimal scenario-model addition, it must not alter existing verdict/state expressions (security-plan §Threat Model Summary, Input Validation cross-field rules).

## Patterns to follow — existing patterns relevant to implementation

1. Scenario TOML structure: declarative, serde-deserialized, garde-validated at load (Epoch-7 established pattern; see `scenarios/*.toml` from severity-lifecycle chunk). Validation rules co-locate with seam-crate struct definitions (Input Validation § scenario-config boundary).

2. Error handling at catalog load: `serde::Deserialize` + `garde::Validate` failures collapse to `ConfigError` via `#[from]` thiserror enum (existing Epoch 7 error-wall pattern; Error Handling § error format, Conventions: Error handling).

3. Test wiring for scenario catalog: rstest parameterized loader (per-scenario class/state guards, suite-level guard for report-state purity) — mirrors severity-lifecycle scenario test additions (scope § Definition of done: "scenario test wiring loads + guards the new family").

4. Run-report artifact structure: identity fields (`run_id`, `seed`, `scenario`, `p_ids`, slo_tier, latency_ms, fingerprints) are agent-parseable; measurement fields are JSON null for blocked/failed states (Error Handling § run-report artifact sanitization).

## Anti-patterns to avoid — domain bans that apply

1. NEVER add a scenario without a Pulse P-ID or introduce an inbound network listener (security-plan §Anti-Patterns § Universal: scope law violation; this chunk declares all five P-IDs upfront).

2. NEVER let scenario-config validation bypass garde at load — all new TOML structs MUST `#[derive(Validate)]` with the cross-field rules co-located (security-plan §Anti-Patterns § Input: "NEVER deserialize scenario config without garde validation at load").

3. NEVER expose internal error details (stack traces, file paths, struct names) in run-report artifacts or `conductor-cli` stderr for failed scenarios — sanitize at the `anyhow` edge (security-plan §Anti-Patterns § API / Logging: error disclosure ban; Error Handling § run-report artifact sanitization).

## Contract bindings — where your domain ties into another

- **Scenario model ↔ observation contract:** ManualCheck (P-025/026/027) and KnownResidual (P-032) scenario expected-state expressions bind to the ReportState `enum` lamp.rs already present; new states are first-catalog exercise, no new lamp.rs entries needed unless minimal model addition is required (scope § Definition of done, Boundaries).
- **Fingerprint recurrence (P-036) ↔ storage seam:** cross-run fingerprint index in `runs.db` declared intent; MCP read-back contract (`query_incident_list` / fingerprint "Previously seen" field) is Epoch-10 live exercise; this chunk gates test-wiring only (scope § Surfaces / contracts touched).
- **Determinism ↔ golden test suite:** seed selection must not collide with or perturb existing golden-fed seeds (severity-lifecycle used 4317019..023); goldens remain UNCHANGED (scope § Determinism; security-plan §Threat Model Summary input-validation cross-field rules).

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. (security) All new scenario TOML structs carry `#[derive(Validate)]` with garde `range` rules (error fraction, durations, ramp factors) + `#[garde(custom)]` cross-field (p50≤p95≤p99, severity-mix sums); `cargo clippy -D` green (grep verifies no unvalidated Deserialize) — per security-plan §Input Validation scenario-config row.

2. (security) Failed scenario-config load surfaces as `ConfigError` harness fault (serde/garde errors routed via `#[from]`), never panic; error-wall test asserts the route (no `unwrap` on load) — per security-plan §Error Handling.

3. (security) Run-report artifacts for P-032 (KnownResidual context section) and P-036 (fingerprint recurrence) do not leak absolute paths or internal struct names; identity fields only (smoke test reads `<run_id>.md` and asserts no canonicalized paths) — per security-plan §Error Handling artifact sanitization.

4. (security) Fingerprint writes to `runs.db` (P-036) use rusqlite bound parameters; `cargo clippy -D` + grep confirm no `format!` or string concatenation in SQL builder — per security-plan §Input Validation runs.db row.

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why

**2026-06-15-config-validation-surface** — garde pinned 0.23.0 → 0.22.1
§Input Validation (scenario-config boundary) + §Bootstrap phases (input-validation-library-install). Validation contract unchanged (garde `#[derive(Validate)]` + `#[garde(custom)]` at load); version downgrade only. Applies: this chunk loads new scenario TOMLs with garde 0.22.1 (already in `[workspace.dependencies]`; no new version bump needed).

**2026-06-15-structured-logging-stack** — obs identity env-handles noted as non-path
§Input Validation clarification: `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs-plan §3) are non-path string labels, no validation required. Applies: P-032/P-036 scenario definitions may reference these env labels in their TOML; they are benign (not validation boundaries).