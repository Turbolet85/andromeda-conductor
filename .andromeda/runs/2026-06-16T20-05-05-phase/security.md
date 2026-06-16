# security extract

## Relevance
Partial — the chunk models scenario config declaratively with serde + garde validation; three security boundaries apply (config parse, env-var path handles, deterministic conversion).

## Constraints
- All serde-deserialized scenario phase structs MUST derive `garde::Validate` with `range` rules (non-negative durations, sane bounds) and `#[garde(custom)]` cross-field invariants (e.g., p50≤p95≤p99 if applicable) (security plan §Input Validation, four-boundary discipline).
- The `CONDUCTOR_*` env-var path handles (`CONDUCTOR_SCENARIOS_DIR`) MUST canonicalize + bounds-check at the `conductor-cli` edge BEFORE any scenario file read, outside garde's struct validation (security plan §Input Validation; per security-research.md serde+garde finding).
- Failed garde `Report` on phase validation MUST surface as `ConfigError` via `#[from]`, routed to the harness-fault `Result::Err` wall, never a verdict or panic (security plan §Error Handling).
- The `Scenario → PhaseTimeline` conversion MUST be order-preserving, total, and deterministic (same scenario + seed ⇒ same shape) so that run-report artifacts remain reproducible ground truth without leaking internal struct names (security plan §Error Handling, determinism invariant).
- All new `serde::Deserialize` structs in the phase model MUST use bound parameters if they ever persist to `runs.db` (no string-formatted SQL); this chunk is config-only, but the constraint applies transitively if the model is reused (security plan §Input Validation, rusqlite row).

## Patterns to follow
- Extend the existing conductor-core scenario model; do not duplicate the phase model.
- Garde validation co-located with serde structs in the owning seam crate (conductor-core); reuse the existing `garde::Report`→`CoreError`/`ConfigError` bridge.
- Config-parse errors surface deterministically as `Err` (harness fault), never a verdict, never a panic; the verdict/error wall is inviolate (security plan §Error Handling).

## Anti-patterns to avoid
- NEVER deserialize scenario config without garde validation at load — the `range` + `#[garde(custom)]` cross-field rules ARE the trust boundary (security plan §Security Anti-Patterns § Input).
- NEVER expose absolute file paths (canonicalized `CONDUCTOR_SCENARIOS_DIR`) or internal struct/field names in run-report artifacts or error messages (security plan §Error Handling; logs § anti-pattern).
- NEVER let a malformed scenario config panic — deterministic conversion routed through typed errors, not `unwrap()` (security plan §Security Anti-Patterns § Universal).

## Contract bindings
- **obs ↔ scenario config:** if the phase emission spec ever includes observability labels (service/env identity), they are non-path env-string labels (no validation required) per §Logging & Monitoring note (security plan amendment 2026-06-15-structured-logging-stack).
- **tests § CI security gate:** `cargo audit` + optional `cargo deny check` must run green before merge (security plan §Dependency Security § CI integration).

## Acceptance criteria contributions
- (security) All serde scenario-config phase structs carry `#[derive(Validate)]` with `range` + cross-field rules; `cargo build` enforces (grep verifies: `derive.*Validate` on `Phase`-like types).
- (security) Scenario config-parse failures become `ConfigError` / `CoreError` `Err`, never panic or verdict; `nextest` runs green.
- (security) `Scenario → PhaseTimeline` conversion is deterministic (same scenario + seed produces same shape); a basic assertion in the integration test verifies reproducibility.
- (security) `CONDUCTOR_SCENARIOS_DIR` canonicalizes + bounds-checks at the CLI edge before scenario file read (read-only check in `conductor-cli` bootstrap, before seam instantiation).

## Relevant amendment history
`2026-06-15-config-validation-surface` — garde pinned 0.23.0 (unavailable) → 0.22.1. The validation contract (serde + `#[derive(Validate)]` at load) unchanged; config boundary row in §Input Validation + bootstrap phase reflect the 0.22.1 floor. (This amendment applies directly — the scenario model IS the config validation boundary.)
