# security extract

## Relevance
Partial — chunk is scenario catalog config (P-009..P-012 baseline/detection), carries security constraints on input validation (scenario TOML deserialization + garde validation) and error/report sanitization, but does not touch auth, secrets, dependency management, or network surfaces directly.

## Constraints
1. All scenario config TOML structs in `scenarios/*.toml` (error-baseline-spike.toml, latency-regression.toml) **MUST** deserialize via `Scenario::from_toml_str` and derive `garde::Validate` with `#[garde(range)]` + `#[garde(custom)]` cross-field rules before producing a `PhaseTimeline` (security-plan.md §Input Validation, boundary: Scenario config files).
2. Each baseline-math expected-check assertion (P-009/P-011 baseline-match within ±10%/±15%) and detection-assertion (P-010/P-012 candidate emission) **MUST** be marked `class = "Hard"` in the `[[expected]]` TOML blocks; no `CalibrationRegion` exception except at floor boundaries (security-plan.md §Input Validation + Threat Model Summary § Attack surface, config-boundary discipline).
3. Run-report artifacts (`<run_id>.md`, `runs.db` rows) written by any scenario **MUST NOT** leak canonicalized `CONDUCTOR_*` env paths or internal seam-crate struct/field names to the operator (security-plan.md §Error Handling § Run-report artifact sanitization).
4. All referenced emit primitives (error-spans, error-rate, latency-shaping, traffic-rate ramps) **MUST** continue using bound SQL parameters for any `runs.db` writes (security-plan.md §Input Validation § `runs.db` writes, anti-patterns § Input).
5. No new inbound network listener may be added by scenario config phases — pure-egress only per architecture scope law (security-plan.md § Threat Model Summary § Attack surface vectors).
6. Scenario phase timelines must preserve determinism: identical `scenario+seed` must produce identical event streams under `start_paused` (security-plan.md § Threat Model Summary § Infrastructure).

## Patterns to follow
1. Scenario TOML structure mirrors ch1/ch2 precedent: `name · p_ids · seed · slo_tier · jitter_ms · [[phases]]{name,gap_ms}` + `[[expected]]{kind,class,expected}` (scope.md precedent; security-plan.md §Input Validation validates at config deserialize).
2. Baseline-convergence + ramp-hold-past-persistence timeline structure (P-009→P-010 and P-011→P-012 dependent pairs) are **declare-only** in TOML — the ±N% tolerance math and persistence-window evaluation defer to Epoch-8 evaluator (security-plan.md § Threat Model Summary § Attack surface + scope.md q2 & q5).
3. Expected-check clauses mark intent deterministically without encoding runtime math — runtime verification happens in the `slo`/evaluator seam, not in the TOML (consistency with ch2 SLO-timing deferral, scope.md q5).

## Anti-patterns to avoid
1. NEVER deserialize scenario config without garde validation at load — the `#[derive(Validate)]` + `range`/`custom` rules ARE the trust boundary (security-plan.md § Security Anti-Patterns § Input).
2. NEVER let run-report artifacts or `runs.db` rows expose absolute host paths (canonicalized `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) — sanitize all path references (security-plan.md § Error Handling + Anti-Patterns § Logging).
3. NEVER add a phase or expected-check that implicitly requires a scenario to bind a new inbound port or spawn an outbound listener beyond the loopback OTLP/gRPC/stdio MCP surface (security-plan.md § Anti-Patterns § Universal).

## Contract bindings
- **security ↔ tests:** roundtrip fixture tests prove each scenario deserializes + garde-validates (scope.md DoD).
- **security ↔ obs:** run-report envelope binds `p_ids`, `slo_tier`, `verdict`; PII scrubbing at logger config, no direct binding here.
- **security ↔ Epoch-8 evaluator:** runtime verdict/state evaluation against baseline tolerance + persistence thresholds (deferred; scope.md q2/q5 confirm evaluate logic is Epoch-8, not TOML).

## Acceptance criteria contributions
1. (security) Scenario TOML files (`error-baseline-spike.toml`, `latency-regression.toml`) deserialize via `Scenario::from_toml_str` and derive `garde::Validate` — confirm `#[derive(Validate)]` + `#[garde(range)]`/`#[garde(custom)]` on scenario boundary structs (per security-plan.md §Input Validation).
2. (security) All P-009/P-010/P-011/P-012 expected-check blocks carry `class = "Hard"` (no `CalibrationRegion` except boundary floors) — TOML audit confirms (per security-plan.md §Input Validation).
3. (security) Fixture round-trip tests confirm `PhaseTimeline` builds without panic and determinism holds (`start_paused` identical seed ⇒ identical event stream) (per security-plan.md § Threat Model Summary).
4. (security) Run-report artifact path sanitization — `<run_id>.md` / `runs.db` rows omit absolute paths + internal struct names (per security-plan.md § Error Handling).

## Relevant amendment history
1. **2026-06-15-config-validation-surface** (§Input Validation) — garde pinned 0.23.0 (unbuildable) → 0.22.1; validation contract unchanged (`#[derive(Validate)]` + `#[garde(custom)]` at load).
2. **2026-06-15-dependency-audit-gate** (§Dependency Security) — cargo-audit/cargo-deny floors confirmed; toolchain bumped to 1.95.0; tauri ≥2.10.3 dormant until Epoch-9 (no impact here).
3. **2026-06-15-structured-logging-stack** (§Input Validation) — obs env-handles (`CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`) are non-path labels (no validation), distinct from `CONDUCTOR_*` path handles; impacts run-report artifact sanitization.
