# Codebase Research — 2026-08-09-in-lane-sut-scenarios

## Scope
- **Depth:** moderate · **Reads:** 9 file ranges · **Globs/Greps:** 6 · **Code-graph queries:** 2

## Files inspected
- `crates/conductor-core/src/drift.rs` (90–149, 295–378) — `check_scenario_backing`'s three disjoint fault conditions and its six tests. The live gate `the_committed_catalog_matches_the_unbacked_ledger` (line 304) builds `named` by `list_scenarios(&scenarios_dir(), &m)` → `flat_map(|s| s.p_ids)`, so **naming P-079 in a scenario's `p_ids` is exactly what backs it**. The five synthetic negative cases need no change.
- `crates/conductor-report/src/coverage.rs` (60–101, 175–200) — `summary_line(rows, unbacked)` takes the count as a **parameter** and writes `" ({unbacked} unbacked)"` only when `unbacked > 0` and only on the `Auto` term. `the_unbacked_qualifier_is_not_a_fifth_summand` asserts against `UNBACKED_AUTO.len()`, never a literal.
- `crates/conductor-cli/src/render.rs` (205–215) — same qualifier, derived inline from `UNBACKED_AUTO.len()`.
- `crates/conductor-tauri/src/commands.rs` (350–359) — `unbacked_auto_command_returns_the_core_ledger` asserts IPC equality with `conductor_core::UNBACKED_AUTO` (no count literal); the webview reads the ledger over IPC rather than mirroring it in TS.
- `crates/conductor-core/src/scenario.rs` (780–840) — the operator-checklist test idiom: an `#[rstest]` `#[case]` loader per stem asserting `s.p_ids`, plus `constellation_and_p032_are_operator_checklist_declare_only` asserting `s.expected.is_empty()`, plus the suite guard `constellation_context_grounding_suite_has_operator_checklist_members` asserting the family exercises BOTH shapes.
- `crates/conductor-core/src/expected.rs` (35–64) — `ComparisonKind` is closed: `Exact` · `Contains` · `Absent` · `CountAtLeast`; `ExpectedCheck` = `{kind, class, expected}` with `expected` garde-required non-empty.
- `scenarios/service-constellation-discovery.toml` (full) — the shipped `DriveObserve` precedent: header comment (P-ID + Pulse's claim + coverage mode + a *declare-only* list), `name`/`p_ids`/`seed`/`slo_tier`/`jitter_ms`, `[[phases]]` with `gap_ms`, and a closing comment stating **why** there is no `[[expected]]` block.
- `scenarios/fingerprint-storm.toml` (full) — the `Auto` precedent: same header discipline plus a single `[[expected]]` `kind = "Contains"` / `class = "Hard"` / `expected = "RetryStorm"`.
- `crates/conductor-timeline/tests/replay.rs` (grep) — the determinism-replay harness enumerates **only** `scenarios/error-baseline-spike.toml`; its two goldens are `replay__fixture_seed_424242.snap` / `replay__fixture_seed_7.snap`.

## Graph impact (code-graph, trace at `.andromeda/runs/2026-08-09T19-39-56-phase/tree-query-2026-08-09-in-lane-sut-scenarios.json`)
- **`UNBACKED_AUTO`** — 13 references: `conductor-core/src/lib.rs:36` (re-export) · `drift.rs:310` (the live gate) · `conductor-report/src/coverage.rs:13,28,164,178,189,191` · `conductor-cli/src/render.rs:17,208,209` · `conductor-tauri/src/commands.rs:140,357`. **Every consuming site reads `.len()` or the slice itself — none hardcodes a count**, so the 11 → 10 shrink propagates with no arithmetic edit.
- **`check_scenario_backing`** — 8 call sites, **all inside `conductor-core/src/drift.rs` tests** (310, 317, 328, 340, 351, 364, 370) plus the `lib.rs:36` re-export. **No production caller**: the gate is a unit test over the committed artifacts, which is why it runs in CI without a live Pulse.

## Patterns detected
- **Seed convention `4317<PPP>`** (`scenarios/*.toml`): the seed encodes the primary P-ID — `4317027` for P-027, `4317017` for P-017, `4317035` for P-035. Only `error-baseline-spike` deviates (`424242`, the arch-documented example seed). **`4317067` / `4317072` / `4317079` are all free** (verified — no collision).
- **Header-comment discipline** (`service-constellation-discovery.toml:1-11`): every scenario opens with the P-ID + Pulse's claim, then an explicit *Coverage mode* line, then a *Declare-only* list naming what is deliberately not asserted and who owns it later.
- **Empty `expected` is the DriveObserve carrier** (`scenario.rs:793-806`): `Scenario.expected` is `#[serde(default)]`, so omitting `[[expected]]` is valid and routes verdict `None` → `Lamp::Manual` (ManualCheck).
- **Qualifier is derived, never literal** (`coverage.rs:82-93`, `render.rs:209-210`): `summary_line` is kept a pure function of its arguments precisely so the Markdown format golden can fix a synthetic set independent of the real pin.
- **Suite-shape guard per family** (`scenario.rs:825`): each family adds one test asserting the family's mixed shape (`any(empty) && any(non-empty)`), beyond the per-stem loaders.

## Conventions to follow
- **Per-stem loader test**: `#[rstest]` `#[case]` rows in `crates/conductor-core/src/scenario.rs` `mod tests`, path via `format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"))`, asserting `s.name == stem` and `s.p_ids` (`scenario.rs:780-786`).
- **Mode-shape assertion**: `assert!(s.expected.is_empty())` for the DriveObserve stems (`scenario.rs:793`); a `Contains`/`Hard` membership assertion for the Auto stem (`scenario.rs:815-822` is the idiom).
- **`slo_tier` is a closed enum** over `<5s` / `<20s` / `<90s`, with an inline comment justifying the tier choice (`fingerprint-storm.toml:24`).
- **Token choice is substring-tolerant and flagged**: an inferred read-back token is declared via `Contains` and recorded in the header as an Epoch-8 calibration point (`fingerprint-storm.toml:16-18`).

## New files to create
- `scenarios/live-only-service-truth.toml` — P-067, DriveObserve, empty `expected` (operator-checklist).
- `scenarios/investigate-actions-functional.toml` — P-072, DriveObserve, empty `expected` (operator-checklist).
- `scenarios/constellation-severity-live-wiring.toml` — P-079, Auto, one `[[expected]]` block over the corpus read-back surface.

## Files to modify
- `crates/conductor-core/src/drift.rs` — remove `"P-079"` from `UNBACKED_AUTO` (11 → 10 entries) and update the doc comment's closing sentence, which currently reads that `P-073`/`P-074`/`P-079` "are already owned by the in-lane SUT-scenarios entry" (only P-073/P-074 remain owed after this chunk; their owners are the Epoch-2 "Pulse run contract" and Epoch-3 "fingerprint-storm live proof" entries).
- `crates/conductor-core/src/scenario.rs` — add the per-stem loader cases + the mode-shape assertions + a suite guard for the new in-lane family.

**No roll-up renderer change is needed** — `coverage.rs`, `render.rs` and `commands.rs` all derive the count. This corrects `scope.md` deliverable 3, which anticipated edits on three surfaces.

## Open questions
1. **What does P-079's `[[expected]]` assert?** The capability is "reconcile the incident workspace key so per-service severity + the incidents panel light up under a live storm". The read-back-observable consequence is that a storm's incident is visible to the **sidecar** at all (intent §4 F10: with the keys diverged, `query_incident_list` returns zero rows). Candidate shapes: `CountAtLeast "1"` over the incident list, or `Contains "<candidate token>"` mirroring the fingerprint-storm precedent. → P4.
2. **Does P-072 stay empty-`expected`?** Its classification is `DriveObserve` (⇒ empty, per the shipped precedent), but intent §4 F2 describes it as "candidate drive+observe **via report read-back**", which hints at a `retrieve_report` leg. Only `Auto` participates in `check_scenario_backing`, so either choice passes the gate. → P4.
