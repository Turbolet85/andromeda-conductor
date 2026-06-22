# arch extract

## Relevance
Partial — crate workspace boundaries + Standard Contracts apply; scenario config model is in-scope, but runtime evaluation is Epoch-8 responsibility.

## Constraints
- Code lives in `scenarios/` keyed to P-IDs per §Inherited Defaults workspace boundary; no new crate or cross-crate dependency (architecture §Module Boundaries: crate-per-seam Cargo workspace enforces forbidden cross-seam deps at compile time; per §Project Intent: "new scenarios are added as declarative config keyed to a Pulse P-ID").
- Scenario config surface (`Scenario`/`ExpectedCheck`/`Phase` TOML models) consumed unchanged via `Scenario::from_toml_str` per §Established Decisions [Scenario Config Format] (toml 0.9 + serde 1.0.x + garde 0.22.1); validation co-located with structs in conductor-core per §Conventions config-conventions.
- All checks must assert `class = "Hard"` per §Probabilistic-Assertion Policy ("suppression/bypass logic" + activity-floor/lifecycle timing are deterministic hard pass/fail, never calibration-region).
- Verdict/error wall (§Cross-cutting Patterns): verification outcomes are typed values (`enum Verdict { Pass, Fail, CalibrationRegion }`), never caught errors; `Result::Err` reserved for harness faults only (config parse → `ConfigError`, runtime errors only at binary edges).
- No new inbound listener — "no scenario without a P-ID" (§Project Intent scope law); port-occupier P-003 is prior art (Epoch 1, not this chunk); silence/gap/train are *stops/starts of egress*, never port binds.
- If Q1 (Open Question 1 in scope.md) resolves to adding a `ComparisonKind` negative variant for suppression absence checks, that single minimal addition is the only permissible model change; confirm zero-change possibility first.

## Patterns to follow
- Two-TOML structure per family grouping (activity-floor.toml P-013/P-014 + restart-suppression.toml P-015/P-016/P-057) mirrors ch1–ch3 precedent (receiver-failed-port-conflict.toml / receiver-lifecycle-state.toml — prior fault-referencing scenarios define how/whether a scenario names a fault).
- Multi-phase timelines within one file express opposing outcomes (e.g. activity-floor's P-013 no-alarm vs P-014 alarm, restart-suppression's suppress vs surface vs triple-bypass) via sequential `[[phases]]` + multiple `[[expected]]` checks with distinct `kind` (presence/absence) and `class` (all Hard).
- Fixture round-trip tests per phase (same pattern: deserialize TOML → garde-validate → `PhaseTimeline` construction) prove model-applicability without runtime MCP; seed-named goldens re-baselined iff new seeds feed the determinism snapshots.
- Determinism under seed is preserved by `start_paused` + `current_thread` tokio runtime (per §Design Philosophy / §Established Decisions); same scenario+seed ⇒ same emission-stream shape is the architectural invariant (no wall-clock in the timeline).

## Anti-patterns to avoid
- Do not emit `Result::Err` for verification outcomes (error-handling §Error Handling: `Verdict`/`ReportState` are values, never caught exceptions; `tonic::Status` / MCP error responses are first-class verification inputs, not panics).
- Do not introduce new crate dependencies outside the workspace or cross-seam (architecture forbids forbidden cross-seam deps at compile; all fault/emit levers already exist in conductor-faults/conductor-emit from Epoch 3/4).
- Do not bind a new port or expose an inbound listener (Conductor is gRPC/MCP client only, not server; port-occupier P-003 is prior art; all timing phases are egress stops/starts via fault helper parameters).

## Contract bindings
- **Workspace crate naming** binds to ALL domains: conductor-core (`Scenario`/`ExpectedCheck`/`Phase`) and conductor-verify (verdict logic) own the shapes; artifact lives under `scenarios/` per §Inherited Defaults.
- **Verdict/error wall** binds to tests (E2E fixtures consume `Verdict` + `ReportState` enums as return values, never `Err`-matching) and obs (error-logging distinguishes `Result::Err`-harness-only from `Ok(Verdict::Fail)`-verification-outcome).
- **Standard Contracts envelope** (run-report shape with `verdict`/`state` + `class` field) binds to report-seam (serialization/storage) + Epoch-8 evaluator (verdict-to-state mapping downstream).
- **Scenario-config TOML format** ties to conductor-core serde/garde stack (§Stack): no new JSON/YAML/DSL introduced; hand-authored TOML is the locked format per user P4 decision (scope.md Open Q2 recommends 2 files, multi-phase — confirm at P4).

## Acceptance criteria contributions
- "(arch) Scenario TOMLs deserialize + garde-validate via `Scenario::from_toml_str` (per §Established Decisions §Scenario Config Format); fixture tests round-trip each `[[phases]]` block to `PhaseTimeline` construction per conductor-timeline API contract."
- "(arch) All checks carry `class = \"Hard\"` (§Probabilistic-Assertion Policy: suppression/bypass logic + activity-floor/lifecycle timing are deterministic, never calibration-region)."
- "(arch) Verdict/error wall: `enum Verdict { Pass, Fail, CalibrationRegion }` + `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` remain the return shape; no new error variants in conductor-core per the verdict-as-value discipline (§Error Handling)."
- "(arch) Suppression absence checks express the Q1-decision outcome: either a minimal negative `ComparisonKind` lands (tracked in amendment history on Q1 resolution), or absence legs are declare-only-positive with Epoch-8 ownership documented — confirm zero-change possibility before any model touch."

## Relevant amendment history
**2026-06-16-scenario-config-model:** toml 0.9 pinned in §Stack + [Scenario Config Format] decision locked declarative TOML (serde + garde 0.22.1) for scenario config, chosen over JSON for hand-author ergonomics across 60 per-P-ID files; this chunk's two TOMLs inherit that locked format and validation surface.

**2026-06-15-config-validation-surface:** garde 0.22.1 pinned (was 0.23.0, which is unbuildable due to missing derive crate); no container-level custom validators — cross-field invariants (p50≤p95≤p99, severity-mix sums) use garde's `Context` pattern; this chunk's hardness/magnitude/tier constraints follow the same pattern per conductor-core's existing `Scenario` struct.

**2026-06-21-run-report-envelope-serializer:** `Verdict::default_report_state` mapping codified (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`); all hard checks in this chunk produce `Pass`/`Fail` verdicts directly (no calibration-region routing), so the Hard classification and envelope shape stay aligned.