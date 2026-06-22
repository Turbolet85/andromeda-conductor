# security extract

## Relevance
Partial — this chunk authors scenario config (TOML files); Input Validation boundary rules apply; no new fault/emit primitives or network listeners; no secret/data/auth exposure.

## Constraints
1. All scenario config TOML files MUST deserialize via `garde::Validate` with `range` + `#[garde(custom)]` cross-field rules on the `Scenario`/`ExpectedCheck`/`Phase` structs (security plan §Input Validation, config-boundary discipline) — error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99, severity-mix sums.
2. Scenario TOMLs MUST NOT bind any new inbound listener or introduce any network/credential exposure — scope law holds: pure-egress (silence/gap/train are *stops/starts of egress*, not port binds; port-occupier is P-003/ch1) (security plan §Threat Model Summary attack surface).
3. The `CONDUCTOR_*` path handles in phase metadata (if any are extended) MUST canonicalize + bounds-check at the CLI edge, NOT in the TOML deserialize path (security plan §Input Validation, serde+garde boundary).
4. Run-report artifacts (`<run_id>.md`, `runs.db` rows) MUST NOT leak absolute host paths or internal struct/field names — suppression/absence checks MUST surface verdict/state/identity fields only (security plan §Error Handling, artifact sanitization).
5. If the P4 Q1 decision adds a negative `ComparisonKind` (e.g. `NotContains`) to express suppression absence checks, the TOML round-trip fixture tests MUST include valid/invalid garde `#[case]` rows for the new kind (security plan §Input Validation, fixture + acceptance criteria).

## Patterns to follow
1. Scenario TOML structure mirrors ch1–ch3 precedent: `name · p_ids · seed · slo_tier · jitter_ms · [[phases]]{name,gap_ms}` + `[[expected]]{kind,class,expected}` — schema already ingested by `Scenario::from_toml_str` (no model touch unless Q1 absence kind required) (security plan §Input Validation, scenario-config row + ch3 precedent).
2. All new checks carry `class = "Hard"` (per Probabilistic-Assertion Policy: "suppression/bypass logic" + timing are hard pass/fail) — no calibration-region tolerance bounds (security plan §Threat Model Summary, determinism discipline).
3. Seed-named determinism goldens: if either TOML feeds the replay snapshot tests, re-baseline by grepping the new **seed** values — same scenario+seed ⇒ same stream shape under `start_paused` (security plan §Error Handling, bounded recursion + determinism clause).

## Anti-patterns to avoid
1. NEVER introduce a `ComparisonKind` that mutates the existing `Contains`/`CountAtLeast` positive-only set without garde fixtures validating the new kind's bounds (security plan §Security Anti-Patterns § Input, input-boundary discipline).
2. NEVER let any phase/scenario name be interpolated into shell/argv or derived from untrusted config sources — phase names are TOML-author-declared only (security plan §Security Anti-Patterns § Code Patterns, spawn hardening).
3. NEVER allow a deferred suppression-absence check (Q1 option B) to evade the Hard class — if the decision defers absence legs to Epoch-8, document them as **explicit scope-out** for the evaluator, never silent/implicit (security plan §Threat Model Summary § Probabilistic-Assertion Policy).

## Contract bindings
**obs ↔ tests harness:** if `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` labels (obs-plan §3) are stamped into scenario metadata (non-path string labels), they require no validation — distinct from the `CONDUCTOR_*` path handles that canonicalize (per 2026-06-15-structured-logging-stack amendment). Guards against false Input-Validation escalation.

## Acceptance criteria contributions
1. (security) `cargo nextest` fixture round-trip tests pass (scenario TOML deserialize + garde-validate + valid `PhaseTimeline` build) — mirrors ch1–ch3 structure.
2. (security) All new `[[expected]]` checks carry `class = "Hard"`; valid/invalid garde `#[case]` rows exist if the P4 Q1 decision adds a negative `ComparisonKind`.
3. (security) Run-report artifacts (`<run_id>.md`, `runs.db` rows) record verdict/state/identity fields only (no internal struct names, no absolute paths leaked via scenario name/phase name).
4. (security) Scenario seed values differ from prior TOMLs; if snapshots feed replay tests, goldens are re-baselined by seed (determinism preserved).

## Relevant amendment history
**2026-06-15-structured-logging-stack:** obs identity env-handles `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` noted as non-path labels (no validation required) — distinct from `CONDUCTOR_*` path handles that canonicalize. Clarifies Input Validation boundary if scenario metadata references these labels (no escalation needed).