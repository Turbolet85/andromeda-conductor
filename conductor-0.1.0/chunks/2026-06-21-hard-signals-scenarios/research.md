# Codebase Research — 2026-06-21-hard-signals-scenarios

## Scope
- **Depth:** moderate · **Reads:** 8 (3 core src + 5 scenario TOMLs) · **Globs/Greps:** 3 · **Code-graph:** 1 query (adoption trace `tree-query-2026-06-21-hard-signals-scenarios.json`)

## Files inspected
- `crates/conductor-core/src/expected.rs` (full) — `ComparisonKind { Exact, Contains, Absent, CountAtLeast }`, `ClaimClass { Hard, CalibrationRegion }`, `ExpectedCheck { kind, class, expected }`. **`Absent` = "observed does NOT contain the expected token"** (expected.rs:41) — the negative-assertion primitive. `expected` is garde `length(min=1)` (expected.rs:62). All variants serialize to canonical PascalCase (locked by tests).
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario { name, p_ids, seed, slo_tier, phases, jitter_ms, expected }`; `expected: Vec<ExpectedCheck>` is `#[serde(default)]` + `#[garde(dive)]` (scenario.rs:94-96, the ch1 wiring). `from_toml_str` (scenario.rs:107) → `CoreError::Config` on parse / `CoreError::Validation` on garde. **Fixture rstest precedent:** `connection_lifecycle_fixtures_load_and_validate` (scenario.rs:300-312) — `#[case(stem, p_id)]` per file, reads `../../scenarios/{stem}.toml`, asserts `name`, `p_ids`, `!s.expected.is_empty()`.
- `crates/conductor-core/src/verdict.rs` (full) — `Verdict { Pass, Fail, CalibrationRegion }`; `default_report_state` maps `CalibrationRegion → ManualCheck` (verdict.rs:52-58). Confirms P-008's `CalibrationRegion` class lands in `ManualCheck`.
- `scenarios/error-baseline-spike.toml` + the 4 ch1 files (`receiver-lifecycle-state`, `last-span-ago-tracking`, `receiver-failed-port-conflict`, `orthogonal-health-domains`) — the live TOML shape (see Patterns).
- `scripts/code-graph-views.sql` (head) — query schema (`defs`/`occ`/`symbol`/`refs`/`calls_m`/`crate_edges`).

## Graph impact (from the code-graph query)
- **`ExpectedCheck` / `ClaimClass`** — consumed by `conductor-core` (`expected.rs`, `scenario.rs`, `lib.rs:32` export) and the **`conductor-verify` evaluator**: `slo::evaluate_check` (`crates/conductor-verify/src/slo.rs:79`), `record.rs`, `verdict.rs`. The evaluator test `exact_contains_absent_compare_as_expected` (`slo.rs:99-103`) proves **`Exact`/`Contains`/`Absent` are already implemented + tested**, and `unmet_count_floor_routes_to_calibration_region_even_when_declared_hard` (`slo.rs:139`) proves the `CountAtLeast` calibration-routing.
- **This chunk modifies no Rust symbol's signature** → zero blast radius. The new `scenarios/*.toml` are consumed only by (a) the new in-crate fixture test cases (calling existing `Scenario::from_toml_str`), and (b) later, the existing `conductor-verify::slo::evaluate_check` evaluator (Epoch-8 live run).

## Patterns detected
- **TOML shape** (`scenarios/*.toml`): `name · p_ids · seed · slo_tier · jitter_ms` + ordered `[[phases]] {name, gap_ms}` + zero-or-more `[[expected]] {kind, class, expected}`. Header comment cites the spec § + the coverage mode (e.g. `receiver-lifecycle-state.toml:1-4`).
- **Multiple `[[expected]]` blocks per scenario** (`orthogonal-health-domains.toml:20-29`): one `Hard`+`Contains "Idle"` and one `CalibrationRegion`+`Contains "error"` coexist — the precedent for P-007 (Contains + Absent) and for mixing classes.
- **CalibrationRegion precedent** (`orthogonal-health-domains.toml:26-29`) — exactly the P-008 shape.
- **Descriptive kebab filenames, NOT the P-ID** (ch1: `receiver-lifecycle-state`, etc.); the P-ID lives in `p_ids` + the header comment.
- **`jitter_ms = 0`** when emission timing is a ground truth the check depends on (`last-span-ago-tracking.toml:9`, `receiver-failed-port-conflict.toml:9`); `50` otherwise.

## Conventions to follow
- Every `[[expected]].expected` is non-empty (garde `length(min=1)`, `expected.rs:62`) — pick a concrete read-back token per check.
- P-007 negative side ("WARN < 17 does NOT contribute") → `kind = "Absent"` (`expected.rs:41`; evaluator-supported per `slo.rs:99`). Positive side ("ERROR/FATAL ≥ 17 contributes") → `kind = "Contains"`.
- `slo_tier = "<5s"` for hard signals (deadline 5000ms, `scenario.rs:54`) — covers the P-005 <500ms-p99 detection budget; the exact timing math is the Epoch-8 evaluator's (numeric tolerance is deferred, per `last-span-ago-tracking.toml:4`).
- Fixture test mirrors `connection_lifecycle_fixtures_load_and_validate` (`scenario.rs:300`): a parallel `#[rstest]` with one `#[case]` per new file.

## New files to create
- `scenarios/span-status-error-detection.toml` (P-005) — phases emit ERROR-status spans; `[[expected]]` `Contains`/`Hard` on the error/severity-candidate read-back token. `slo_tier = "<5s"`.
- `scenarios/exception-event-capture.toml` (P-006) — phases emit spans with `exception` events; `[[expected]]` `Contains`/`Hard` on the captured exception token (type/message). `slo_tier = "<5s"`.
- `scenarios/high-severity-log-capture.toml` (P-007) — phases emit logs across the 17 boundary; **two** `[[expected]]`: `Contains`/`Hard` (ERROR/FATAL ≥17 contributes) + `Absent`/`Hard` (WARN <17 absent from the hard-signal layer). `slo_tier = "<5s"`.
- `scenarios/root-span-error-scope.toml` (P-008) — phases emit trace (a) child-only ERROR / root OK, then trace (b) child+root ERROR; `[[expected]]` `Contains`/**`CalibrationRegion`** on the root-error/severity token (the relative b≥a tendency is the Epoch-8 evaluator's interpretation, not encoded in `ComparisonKind`). `slo_tier = "<5s"`.

## Files to modify
- `crates/conductor-core/src/scenario.rs` — add a `#[rstest]` `hard_signal_fixtures_load_and_validate` (4 `#[case]` rows, P-005..P-008) mirroring scenario.rs:300; **plus an assertion that the P-008 fixture's `expected` carries `ClaimClass::CalibrationRegion`** (the v2.1-amendment guard — the one behavioral check distinct from ch1).

## Open questions
- Exact filenames + the per-check `expected` target tokens (which read-back string each `Contains`/`Absent` matches) — tokens are ultimately the Epoch-8 evaluator's concern; choose declarative-but-plausible values now. **File granularity + names → likely AskUserQuestion at P4** (mirrors ch1's "4 files, one per P-ID" decision).
- P-007 encoding: confirm one file carrying both the `Contains` (ERROR) and `Absent` (WARN) blocks is the cleanest expression (vs two narrower checks). Leaning: one file, two blocks.
