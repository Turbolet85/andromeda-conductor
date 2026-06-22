# Codebase Research — 2026-06-22-severity-lifecycle-scenarios

## Scope
- **Depth:** moderate · **Reads:** 6 (`scenario.rs` full, `expected.rs` full, 3 precedent TOMLs, spec §5) · **Globs/Greps:** 3 (core src list, timeline goldens grep, code-graph query)

## Files inspected
- `crates/conductor-core/src/scenario.rs` (full) — the `Scenario`/`PId`/`SloTier` model + the `from_toml_str` loader + **the entire per-family test pattern** (the rstest catalog of `*_fixtures_load_and_validate` + class-guard + token-assertion tests I extend).
- `crates/conductor-core/src/expected.rs` (full) — `ExpectedCheck {kind, class, expected}`, `ClaimClass {Hard, CalibrationRegion}`, `ComparisonKind {Exact, Contains, Absent, CountAtLeast}`. `expected` is garde `length(min=1)` (non-empty); `kind`/`class` are closed enums (`#[garde(skip)]`).
- `scenarios/root-span-error-scope.toml` — **the CalibrationRegion precedent** (P-008): `kind="Contains"`, `class="CalibrationRegion"`, `expected="error"`; the relative/tendency comparison is Epoch-8's, the TOML declares a representative-token calibration marker only.
- `scenarios/fingerprint-storm.toml` + `scenarios/activity-floor.toml` — the Contains/Absent outcome-coherence precedent + the phase-name-encodes-semantics convention + the `4317NNN` seed family + comment-block house style.
- `crates/conductor-timeline/tests/replay.rs` + `determinism.rs` (grep) — both reference **only** `error-baseline-spike`; no other TOML feeds a golden.

## Graph impact (code-graph query → `tree-query-2026-06-22-severity-lifecycle-scenarios.json`)
- **`ExpectedCheck`** — referenced only in `conductor-core` (`expected.rs` def, `lib.rs:32` re-export, `scenario.rs:12` use + tests) and consumed downstream by the verify evaluator. This chunk adds **new TOML data + new test `#[case]` rows** against the existing type — **no signature change, additive, zero cross-crate blast radius**. (Same holds for `Scenario`/`ComparisonKind`/`ClaimClass`/`SloTier`: consumed, not modified.)

## Patterns detected
- **Per-family test triplet** (`scenario.rs:300-516`): for each catalog family — (1) a `*_fixtures_load_and_validate` rstest with `#[case](stem, p_ids)` asserting `name`, `p_ids`, and non-empty `expected`; (2) a class guard (`*_checks_are_all_hard`); (3) targeted token assertions (e.g. `Contains "RetryStorm"` / `Absent "RetryStorm"`). I replicate this, but the class guard for THIS family asserts **both** classes (see Conventions).
- **CalibrationRegion encoding** (`root-span-error-scope.toml`): a model-interpretive leg is `kind="Contains"` + `class="CalibrationRegion"` + a representative token; the *tendency* (b≥a, or "the model chose tier X") is the Epoch-8 evaluator's, NOT encoded in `ComparisonKind`. → severity-tier legs (`Contains "Suggested"` etc.) follow this exactly.
- **Phase semantics live in phase NAMES** (`scenario.rs:131` note + both TOMLs): `PhaseSpec = {name, gap_ms, emission(default)}` — no occurrence-count / variant / magnitude / tier field. The Epoch-8 driver realizes counts, variants, tier-calibration, ack actions, and cease windows FROM the phase names. So my phases encode `trigger` / `sustain` / `cease-120s` / `retrigger-within` / `retrigger-after` / `autonomous-stimulus` / `suggested-stimulus` / `curious-stimulus` as NAMES.
- **Outcome-coherence** (`scenario.rs:431-516` + testing.md): `Absent` + `Contains` of the SAME token contradict (one read-back, no per-check scoping) → split files. DIFFERENT tokens via `Contains` coexist (`query_incident_list` returns a LIST — P-007's `Contains "ERROR"` + `Absent "WARN"` precedent).

## Conventions to follow
- **Seed family `4317NNN`**, one distinct seed per file (e.g. `4317019`/`4317022`/`4317023`/`4317060`), tracking the lead P-ID (precedent: `4317008`/`4317013`/`4317017`).
- **`slo_tier` is SCENARIO-level** (`scenario.rs:80-82`) — exactly ONE per `Scenario`. P-060's Tier-1/2/3 (`<5s`/`<20s`/`<90s`) therefore CANNOT coexist in one scenario; the family demonstrates the mapping by the per-scenario tier spread (and/or dedicated per-tier scenarios). The latency *measurement* is Epoch-8 (`SloTier::deadline_ms` exists; the TOML only declares the tier).
- **`expected` is a non-empty String**; `CountAtLeast` uses a decimal-string floor (e.g. `"2"`).
- **Class guard for a MIXED family (new):** every prior family guard asserts `all(... == Hard)`. This family is the first to carry CalibrationRegion AND Hard, so its guard asserts `any(Hard)` **and** `any(CalibrationRegion)` — not `all(Hard)`. This is the one genuinely new test shape.
- **`conductor-verify::slo` softens an UNMET `CountAtLeast` to calibration-region at eval time** (testing.md) — so declaring a met-floor `CountAtLeast` as `class="Hard"` is correct (the unmet case is the evaluator's).
- House-style TOML comment block: P-ID recipe + declare-only severity + coverage-mode line + Epoch-8 calibration points (mirror `fingerprint-storm.toml:1-21`).

## New files to create
- `scenarios/*.toml` — the severity-lifecycle catalog file(s); **count + grouping is the P4 headline AskUserQuestion** (recommendation: 4, mapping the working-entry's own decomposition — tiered-inputs / auto-resolve / ack-cooldown / per-tier-SLO — adjusted for the slo_tier-is-scenario-level + Absent/Contains-coherence constraints below).

## Files to modify
- `crates/conductor-core/src/scenario.rs` (**tests only**, `#[cfg(test)] mod tests`) — add the per-family `#[rstest]` load+validate cases, the **mixed-class** guard (`any(Hard)` && `any(CalibrationRegion)`), and targeted token assertions (`Contains "Resolved"` Hard; `Contains "Suggested"`/`"Autonomous"`/`"Curious"` CalibrationRegion; the P-023 `Absent`/`CountAtLeast` legs per the resolved split). No production code changes.

## Open questions
- **P-060 vs scenario-level `slo_tier`:** three tiers cannot share one scenario → resolve at P4 whether P-060 gets dedicated per-tier scenarios or is satisfied by the family's slo_tier spread (recommend: the spread + one explicit per-tier demonstration; latency is Epoch-8).
- **P-023 within/after split + ack drivability (scope Q2a/Q4):** the within-cooldown (no-new) vs after-cooldown (new) outcomes contradict on the incident-kind token → split files or declare-only the within leg; AND "acknowledge" has no clear tool in the 4-tool MCP contract (`mark_incident_resolved` = resolve, not ack) → the ack step may be operator/declare-only. Resolve at P4 + flag as an Epoch-8 calibration point.
- **Lifecycle-status kind (scope Q3):** confirmed expressible with the existing set — `Contains "Resolved"` (status) + `CountAtLeast "2"` (new-not-reopen) — **no new `ComparisonKind` needed** (zero model change holds).
