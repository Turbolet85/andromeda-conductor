# security extract

## Relevance
Relevant — chunk adds scenario catalogs exercising severity-lifecycle model features and timing requirements that touch input validation (TOML deserialize + garde), error outcomes (verdict/state routing), and determinism (seed reproducibility).

## Constraints
1. All `Scenario`/`Phase`/`ExpectedCheck` TOML structs deserializing from severity-lifecycle files MUST derive `garde::Validate` with `range` rules (error-fraction ∈ [0,1], non-negative durations, p50≤p95≤p99 cross-field) and route parse failures to `ConfigError` (per §Input Validation, four-boundary discipline).
2. Lifecycle-status assertions (P-022 "Resolved" detection, P-023 cool-down window) MUST use existing `ComparisonKind` enum (`Contains`/`Absent`/`CountAtLeast`) without adding lifecycle-status-specific comparison kinds unless expressly unreachable (per scope law: "zero model change").
3. All `CalibrationRegion` expected blocks (P-019/P-020/P-059 severity/interpretation legs) MUST route to `ManualCheck` verdict-state via the verdict-first lamp; `Hard` blocks route deterministically (per §Probabilistic-Assertion Policy, P-008 precedent).
4. Seed-named determinism goldens under `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_*.snap` MUST be re-baselined iff new TOML feeds them; grep the new seed values (expected UNCHANGED per Epoch-7 invariant).
5. No new emit/fault primitive, no new inbound listener bind, no new dependency; path canonicalization gate remains at the CLI edge for any `CONDUCTOR_*` overrides that load severity-lifecycle files (per §Input Validation, env-var bounds-check).
6. Deterministic lifecycle timing (`start_paused` fixture mode) covers P-022's 120-s cease window and P-023's 5-min cool-down; declare spec-faithful gap_ms (~120000 / ~300000 ms) with runtime compression deferred to Epoch-8 (per §Timing-Tolerance Model).

## Patterns to follow
1. Mixed `Hard` + `CalibrationRegion` class split — severity-choice and interpretation-continuity legs are `CalibrationRegion` (routed to `ManualCheck`); lifecycle-timing and tier-SLO-routing legs are `Hard` (per P-008 precedent + architecture §Probabilistic-Assertion Policy).
2. Trigger-stimulus shape reuse — tier-calibrated stimuli (error spans / error-rate / latency-shaping / fingerprint-storm) and cease-resume levers (abrupt-silence / emission-gap-resume) already exist from Epoch-3/4; new TOML consumes these via `Phase` (no new emit/fault primitive).
3. Guard-assertion for mixed-class coverage — test that both `Hard` and `CalibrationRegion` are exercised in the TOML suite (first mixed-class family); mirrors ch2 class-usage coverage pattern.

## Anti-patterns to avoid
1. NEVER deserialize severity-lifecycle TOML without garde validation — `#[derive(Validate)]` with `range` + custom cross-field rules ARE the trust boundary for operator-supplied config (§Input Validation).
2. NEVER add a new `ComparisonKind` variant for lifecycle-status assertions without exhausting `Contains` / `Absent` / `CountAtLeast` expressiveness first (zero-model-change bias per scope law).
3. NEVER let a lifecycle-timing mismatch (e.g. incident surfaces as `Resolved` in <120 s) silently become a `CalibrationRegion`/`ManualCheck` outcome — timing is deterministic (`Hard`); model-interpretive legs are calibration-region only (§Probabilistic-Assertion Policy).

## Contract bindings
- **security ↔ obs** — seed-named determinism goldens re-baseline iff new TOML feeds them.
- **security ↔ error-handling** — model-interpretive legs route to `ManualCheck`; deterministic spine surfaces as hard verdicts.
- **security ↔ scenario-config boundary** — garde validates P-IDs + lifecycle-timing params at load; path canonicalization gates TOML file reads at the CLI edge.

## Acceptance criteria contributions
- (security) All structs in new severity-lifecycle TOML files derive `garde::Validate` with `range` + custom cross-field rules; validation failures produce `ConfigError` (per §Input Validation; grep + `cargo build`).
- (security) Lifecycle-timing assertions (`Hard`) use existing `ComparisonKind` without new variants; model-interpretive assertions (`CalibrationRegion`) route to `ManualCheck` (per §Probabilistic-Assertion Policy; test + review).
- (security) No new inbound listener bind introduced; scenarios carry declared P-IDs only (per §Security Anti-Patterns; grep `scenarios/*.toml`).
- (security) Seed-named goldens unchanged unless new TOML feeds them (per §Dependency Security determinism gate; grep seed values).

## Relevant amendment history
- `2026-06-15-config-validation-surface` — garde pinned 0.23.0 → 0.22.1 (validation contract unchanged). Applies: severity-lifecycle scenarios use garde 0.22.1.
- `2026-06-15-dependency-audit-gate` — cargo-audit/cargo-deny reframed as minimum floors; toolchain bump done. Applies: dependency audit gate remains mandatory before merge; no new dependency this chunk (GUI dormant until Epoch 9).
