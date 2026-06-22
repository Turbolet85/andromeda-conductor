# Codebase Research — 2026-06-22-constellation-context-grounding-scenarios

## Scope
- **Depth:** moderate-deep · **Reads:** 7 (expected.rs, lamp.rs, scenario.rs, coverage.rs grep, 2 fixture TOMLs, cookbook) · **Globs/Greps:** 4

## Files inspected
- `crates/conductor-core/src/scenario.rs` (full) — the `Scenario` model + ALL scenario-catalog test wiring. **Key: `expected: Vec<ExpectedCheck>` is `#[serde(default)]`** (line 94) and its doc comment (line 91-93) explicitly anticipates THIS chunk: *"defaults to none — a drive+observe scenario may assert only via the operator checklist."* Test `expected_defaults_to_empty_when_omitted` (line 660) proves an empty-`expected` scenario validates. Every existing loader test asserts `!s.expected.is_empty()` ("declares at least one expected check") — the constellation operator-checklist scenarios INVERT that. Class-guard idiom: `s.expected.iter().all(|c| c.class == ClaimClass::Hard)` / `.any(|c| c.class == CalibrationRegion)`; suite-mixed guard `any(Hard) && any(CalibrationRegion)` (line 605).
- `crates/conductor-core/src/expected.rs` (full) — `ExpectedCheck { kind: ComparisonKind, class: ClaimClass, expected: String }`. `ClaimClass ∈ {Hard, CalibrationRegion}` ONLY; `ComparisonKind ∈ {Exact, Contains, Absent, CountAtLeast}`. **No `state`/known-residual field anywhere** — an ExpectedCheck yields a *verdict*, never a ReportState.
- `crates/conductor-core/src/lamp.rs` (full) — `Lamp::for_record` (line 37) is the decisive mechanism: `RunRecord.verdict` is `Option<Verdict>`, `state` independent. `(ReportState::ManualCheck, None) => Lamp::Manual` (operator-checklist = **verdict None**); `(ReportState::KnownResidual, _) => Lamp::Residual` (state-driven, precedence). `(_, Some(CalibrationRegion)) => Lamp::Hold` — confirms severity-lifecycle's CalReg path is DISTINCT from the operator-checklist Manual path.
- `crates/conductor-core/src/coverage.rs` (grep) — P-025 "Halo Hue Encoding", P-026 "Halo Breathing Encoding", P-027 "Service Constellation Auto-Discovery" = `CoverageMode::DriveObserve`; P-032 "Project Context Grounding", P-036 "Cross-Incident Pattern Reference" = `CoverageMode::Auto`. Already classified — no coverage.rs change.
- `scenarios/severity-tier-autonomous.toml` + `scenarios/incident-auto-resolution.toml` (full) — the TOML idiom: comment header (P-IDs + spec § + capability prose + declare-only notes + **Epoch-8 calibration points** + **coverage mode** line) → `name`/`p_ids`/`seed`/`slo_tier`(inline comment)/`jitter_ms` → `[[phases]]` (descriptive `name` the Epoch-8 driver realizes + `gap_ms`) → `[[expected]]` (kind/class/expected, each commented). Seed convention `4317{NNN}`; spec-faithful `gap_ms` with "Epoch-8 owns compression" note.

## Graph impact (code-graph query — trace at `tree-query-2026-06-22-constellation-context-grounding-scenarios.json`)
- **`crate_edges` for `conductor-core`** — 5 inbound consumers (conductor-cli, -report, -tauri, -timeline, -verify); zero outbound. conductor-core is the base lib. BUT this chunk adds **no production symbol** (data TOMLs + `#[cfg(test)]` fns only) → **zero production blast radius**; the 5 edges are untouched. (Consulted: the change modifies no public surface, so no impact/caller query is decision-relevant.)

## Patterns detected
- **Empty-`expected` = the drive+observe/operator-checklist path** (`scenario.rs:91-94` + `lamp.rs:44`): a scenario that omits `[[expected]]` produces no verdict → `Lamp::Manual`. This IS the P-025/026/027 mechanism — built-in, zero change.
- **Declare-only Epoch-8 legs** (`incident-auto-resolution.toml` comments; severity-lifecycle within-cooldown; activity-floor suppressed legs): a leg the current model can't assert (environment-dependent token, model-mediated, or no state field) is documented in TOML comments + asserted later. P-032's KnownResidual routing fits this exactly (no scenario `state` field; the expected-commit token is live-environment-specific).
- **Inferred read-back token via `Contains`** (RetryStorm, LatencyRegression, Resolved): a non-canonical detector token is declared substring-tolerant + flagged an Epoch-8 calibration point. P-036's "Previously seen" recurrence token is the same.
- **Seed = `4317{NNN}`** per primary P-ID; goldens are seed-named timeline snapshots (`replay__fixture_seed_<N>.snap`) that these scenario seeds do not feed.

## Conventions to follow
- One scenario TOML per catalog entry under `scenarios/`, serde+garde-validated via `Scenario::from_toml_str` (`scenario.rs:107`).
- Test wiring lives in `scenario.rs` `#[cfg(test)] mod tests` — a per-family `#[rstest]` loader (`#[case(stem, &[p_ids])]`) + per-class/per-kind guards + a suite guard (mirror `severity_lifecycle_*` at line 518-632).
- Status never color-alone; `ReportState`/`Lamp` already render `[MANUAL]`/`[RESIDUAL]` (`lamp.rs:52`) — no new lamp work this chunk.

## New files to create
- `scenarios/halo-hue-encoding.toml` — P-025, DriveObserve, empty `expected` (operator-checklist), seed 4317025, `slo_tier="<5s"` (hue shift ≤2s).
- `scenarios/halo-breathing-encoding.toml` — P-026, DriveObserve, empty `expected`, seed 4317026, rate-ramp/breathing phases.
- `scenarios/service-constellation-discovery.toml` — P-027, DriveObserve, empty `expected`, seed 4317027, multi-service topology phases (new dot ≤5s, stable-across-restart, >20 cap — declare-only sub-claims).
- `scenarios/project-context-grounding.toml` — P-032, Auto→KnownResidual, empty `expected` (declare-only; recent_commits stub-until-v0.3.0 documented), seed 4317032.
- `scenarios/cross-incident-recurrence.toml` — P-036, Auto, `[[expected]]` Hard `Contains "Previously seen"` (inferred token), seed 4317036; same-fingerprint phases (recurrence realized by running twice → runs.db index).
- *(File count/granularity for the constellation trio is P4-Q1; the above is the recommended 5-file split.)*

## Files to modify
- `crates/conductor-core/src/scenario.rs` — TESTS ONLY: a `constellation_context_grounding_fixtures_load_and_validate` `#[rstest]` loader; a guard asserting P-025/026/027/032 are operator-checklist/declare-only (`s.expected.is_empty()` — inverts the existing non-empty guard); a guard asserting P-036 is Hard `Contains "Previously seen"`; a suite note. No production code.

## Open questions
- **Q1 (P4 AskUser):** constellation-trio file granularity — 3 per-P-ID files (recommended; matches per-value precedent + 1-TOML-per-P-ID) vs 1 grouped `service-constellation.toml` (matches input.md family digest). Context-grounding is NOT a question (P-032 vs P-036 must split — different outcomes/classes).
- **Q2 (P4 AskUser):** P-032 KnownResidual expression — declare-only/empty + Epoch-8 routing (recommended; zero model change, precedent-consistent) vs a minimal scenario-level state/known-residual marker (model change; contradicts "state is producer-assigned"). Refines scope.md DoD "P-032 expresses KnownResidual" → "declares + documents; routing Epoch-8."
