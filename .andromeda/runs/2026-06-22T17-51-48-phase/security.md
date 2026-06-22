# security extract

## Relevance — relevant

## Constraints

- All scenario config files (`scenarios/*.toml`) MUST deserialize + validate via `Scenario::from_toml_str` with garde `#[derive(Validate)]` `range` + `#[garde(custom)]` cross-field rules; failed garde `Report` routes to `ConfigError` harness fault (security-plan §Input Validation, four-boundary discipline)
- Scenario P-IDs MUST map to known Pulse capabilities (P-017 identity, P-018 storm-cue thresholds); introducing a scenario without a P-ID violates architecture scope law (security-plan §Security Anti-Patterns § Universal)
- Fingerprint identity validation (triple collision: identical stack · path-variant · line-variant → ONE fingerprint; type/frame variance → DIFFERENT fingerprints) is deterministic; all `class="Hard"` per Probabilistic-Assertion Policy (scope.md § Boundaries; security-plan §Input Validation table)
- Storm-cue threshold counts (6 hits / 30s → "Suggested", 12 hits / 30s → "Autonomous") are deterministic counts verified at Epoch-8 runtime; this chunk declares expected outcomes only via `scenario.expected` — no live detection logic here (security-plan §Error Handling § Verdict wall)
- Run-report artifacts (`<run_id>.md`, `runs.db` rows) MUST NOT leak absolute host paths or internal seam-crate struct names when recording fingerprint/storm outcomes (security-plan §Error Handling § Run-report artifact sanitization)
- Scenario TOML loading test coverage MUST extend per-family loader `#[rstest]` `#[case]` rows to cover new files; all loader tests parse green, no golden re-baseline (scope.md § Boundaries)

## Patterns to follow

- Reuse pre-existing `ComparisonKind` / `ExpectedCheck` / `Scenario` serde + garde types (no model change); new seeds in `43170NN` family, distinct per file (scope.md § Surfaces / contracts)
- Outcome-coherence split: a scenario's `expected: Vec<ExpectedCheck>` evaluates against ONE read-back with no per-check window/service scoping — same token MUST NOT carry contradictory `Contains`/`Absent` (or contradictory threshold) outcomes in one file (scope.md § Boundaries)
- Declare only outcomes the single read-back evaluator can disambiguate; per-check window/service legs recorded in TOML comments + chunk notes for Epoch-8 evaluator (scope.md § Boundaries)

## Anti-patterns to avoid

- NEVER introduce a scenario without a valid Pulse P-ID (security-plan §Security Anti-Patterns § Universal; scope.md § Boundaries: "ZERO model change / ZERO new dependency")
- NEVER skip scenario config garde validation at load — garde `range` + `#[garde(custom)]` cross-field rules ARE the trust boundary (security-plan §Security Anti-Patterns § Input)
- NEVER let malformed scenario config parse silently — route garde `Report` to `ConfigError` harness-fault class, never a false pass (security-plan §Error Handling, §Security Anti-Patterns § Universal)

## Contract bindings

- Fingerprint emission primitive (conductor-emit, P-006/P-017/P-018) — already shipped 2026-06-18; this chunk declares expected outcomes (scope.md § What this builds)
- Epoch-8 runtime verification (MCP read-back against `scenario.expected`) — live storm/dedup detection + cue evaluation are Pulse's behavior; this chunk's outcomes feed the evaluator (scope.md § Boundaries)
- Runs.db + JSONL artifact sanitization (conductor-report storage seam) — fingerprint/storm outcomes recorded without leaking absolute paths or struct names (security-plan §Error Handling)

## Acceptance criteria contributions

- (security) Scenario config parse green: `Scenario::from_toml_str` succeeds + garde validation passes for all new `scenarios/*.toml` files
- (security) Fingerprint identity outcomes asserted: same-fp (identical/path/line) and distinct-fp (type/frame) both declared in scenario expected checks; line-insensitivity (clause (c)) reflected
- (security) Storm-cue thresholds asserted: 6→Suggested and 12→Autonomous threshold outcomes both declared, all `class="Hard"` per Probabilistic-Assertion Policy
- (security) Run artifacts sanitized: run reports + `runs.db` rows record fingerprint/storm outcomes without absolute paths or internal struct names (grep verifies no canonicalized `CONDUCTOR_*` paths or seam-crate type names leak)

## Relevant amendment history

2026-06-15-config-validation-surface — garde pinned 0.22.1 (from unreleased 0.23.0); scenario-config boundary validation contract unchanged (§Input Validation table); impacts this chunk's `Scenario::from_toml_str` garde call

2026-06-15-structured-logging-stack — clarified that `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path labels (no validation); does NOT impact fingerprint-storm scenarios (those are path-agnostic outcome declarations)