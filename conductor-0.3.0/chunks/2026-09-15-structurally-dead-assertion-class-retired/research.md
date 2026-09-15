# Codebase Research — 2026-09-15-structurally-dead-assertion-class-retired

## Scope
- **Depth:** deep · **Reads:** 16 files (12 in-repo, 4 in the SUT at Pulse HEAD `83d4060`) · **Globs/Greps:** 14
- **Harness rules consulted:** none — no live leg in this chunk (the chunk drives nothing; its evidence base
  is the already-committed live measurements plus the SUT re-derivation below).

## Files inspected
- `crates/conductor-core/src/scenario.rs` (`:1185-1250`, `:1296-1345`, `:1370-1460`, `:978-990`) — all five
  pinning tests in the inline `#[cfg(test)]` module, plus `expected_defaults_to_empty_when_omitted` (`:980`)
  proving `#[serde(default)]` admits a zero-`[[expected]]` scenario.
- `crates/conductor-core/src/drift.rs` (`:40-75`, `:140-175`) — `UNBACKED_AUTO` + `check_scenario_backing`;
  the gate compares `Auto` rows against `scenario_p_ids` (what the catalog NAMES, via `list_scenarios`),
  never against `expected` emptiness.
- `crates/conductor-core/src/coverage.rs` (structural read of `row(...)` entries) — the 82-row classification.
- `crates/conductor-verify/src/slo.rs` (`:45-60`, `:75-105`) — `compare` (`Contains` is case-SENSITIVE at
  `:51`) and `evaluate_check`'s floor routing.
- `crates/conductor-verify/src/extract.rs` (`:1-60`, `:80-160`) — how `Observation.text` and
  `evidence_count` are composed. **Load-bearing and initially mis-read:** the report markdown IS pushed into
  the graded text (`:101-103`), not only `list_text`'s status/severity/title (`:135-147`).
- `crates/conductor-run/src/execute.rs` (`:90-140`, `:160-200`, `:510-525`, `:640-660`) — the run seam's
  declare-only routing.
- `scenarios/cross-incident-recurrence.toml` (full) + the `[[expected]]` blocks of the other three members.
- `scripts/mutation-roster.toml` — `[[unit]]` members are `conductor-cli`, `conductor-verify`,
  `conductor-emit`; **`conductor-core` is not a member**, so test-plan §9's scoped mutation audit is not owed.
- `.andromeda/architecture.md:135` and `.andromeda/obs-plan.md:349` — read by OFFSET (multi-KB single lines).
- SUT at Pulse HEAD `83d4060`: `pulse-app/src/inference_runtime.rs`, `pulse-app/src/incidents_router.rs`,
  `crates/mcp-server/src/tools.rs` (`:425-440`), `crates/interpretation/src/markdown.rs`.

## Graph impact (rust plane, `db_state: fresh`)
Trace: `.andromeda/runs/2026-09-15T09-59-02-phase/tree-query-2026-09-15-structurally-dead-assertion-class-retired.json`
(5 queries; row counts below are the trace's `rows`, not a clipped view).

- **`ComparisonKind::CountAtLeast`** @ `crates/conductor-core/src/expected.rs:44` — **16 refs** across 6
  files: `expected.rs` 2 · `scenario.rs` 2 · `conductor-verify/src/extract.rs` 2 ·
  `conductor-verify/src/slo.rs` 7 · `conductor-verify/tests/expected_slo.rs` 2 ·
  `conductor-verify/tests/readback.rs` 1. The chunk removes no variant and changes no signature — every
  ref is a reader of the enum, unaffected by retiring instances in data.
- **`Scenario.expected`** @ `crates/conductor-core/src/scenario.rs:134` — **55 refs** across 8 files:
  `scenario.rs` 43 · `conductor-run/src/execute.rs` 4 · `conductor-run/tests/operator_pause_harvest.rs` 3 ·
  `error.rs` 1 · `load_envelope.rs` 1 · `conductor-run/tests/composition_root.rs` 1 ·
  `conductor-run/tests/dispatch_wire.rs` 1 · `conductor-timeline/src/convert.rs` 1. The four `execute.rs`
  reads are the run seam analysed below; none needs a change.
- No symbol's signature changes, so there is no caller-threading set to enumerate. The companion sweep (a
  NAME grep over the whole Rust tree, per the corrected rule) is the binding enumeration here, and it
  produced the five-test modify-set below.

## Patterns detected
- **Declare-only is routed, not special-cased** (`conductor-run/src/execute.rs:98,123`): `scenario.expected.is_empty()`
  is passed to `route_read_back` and gates an early branch. A declare-only scenario takes the
  `ReadBack::AutoResolved` arm on an empty read-back (→ `degraded: true`) where a checks-bearing one lands
  `Blocked`; it then raises an **operator-checklist hold** (`step: "operator-checklist"`, `allow_no_go: true`)
  and lands `verdict: null` / `state: KnownResidual`.
- **Zero check records by construction** (`execute.rs`, `ScenarioOutcome` doc + `:180`): the per-check map
  zips over `scenario.expected`, and `.expect("expected is non-empty")` sits on the non-empty path only.
  `checks` is empty for a declare-only scenario. The `:648` test comment states it directly: "an
  empty-`expected` scenario never reaches `evaluate_check`".
- **An unmet sample floor never hard-fails** (`conductor-verify/src/slo.rs`): `evaluate_check` overrides
  `class` to `ClaimClass::CalibrationRegion` when `matches!(check.kind, CountAtLeast) && !value_matched`.

## Conventions to follow
- **Retirement records its ground in the TOML header** — the shipped idiom, e.g.
  `scenarios/severity-tier-autonomous.toml:11` (`DECLARE-ONLY (2026-08-21-severity-lifecycle-live-proof) — …`)
  and `scenarios/restart-suppression.toml:40`.
- **Pinning tests stay inline** in `conductor-core/src/scenario.rs` (test-plan §4); do not migrate to `tests/`.
- **Set-valued, de-hardcoded assertions** over literals (test-plan §1 Critical Path 6) — the style governing
  any re-scoping of the two suite-shape tests.

## New files to create
- `conductor-0.3.0/chunks/2026-09-15-structurally-dead-assertion-class-retired/evidence/assertion-class-enumeration.md`
  — the committed "as a class" record: all 12 live `[[expected]]` blocks classified satisfiable-or-dead, each
  with its derivation command. Repo-relative paths only (security-plan §Anti-Patterns → Logging).

## Files to modify
- `scenarios/constellation-severity-live-wiring.toml` — remove the `[[expected]]` block (`:52-55`), add the
  retirement header comment. `p_ids`, `slo_tier` and phases untouched.
- `scenarios/findings-counter-refresh.toml` — same at `:48-51`.
- `scenarios/pulse-run-contract.toml` — same at `:57-60`.
- `scenarios/cross-incident-recurrence.toml` — **disposition pending the P4 fork** (Open question 1).
- `crates/conductor-core/src/scenario.rs` — the five pinning tests (measured, not inherited):

| Test | Line | Effect of retirement |
|---|---|---|
| `constellation_severity_live_wiring_asserts_incident_visibility_via_hard_count_floor` | `:1408` | RED — asserts P-079's `CountAtLeast`/`Hard`/`"1"` |
| `findings_counter_and_threshold_reload_carry_their_hard_checks` | `:1303` | RED on its first assert only; the P-056 `Absent` half stays live and correct |
| `in_lane_sut_suite_mixes_operator_checklist_and_auto` | `:1429` | RED — `shapes.iter().any(|empty| !*empty)` ("the suite carries an auto member"); constellation is the trio's only non-empty member |
| `cross_incident_recurrence_asserts_previously_seen_via_hard_contains` | `:1188` | RED **iff** member 4 is retired |
| `constellation_context_grounding_suite_has_operator_checklist_members` | `:1207` | RED **iff** member 4 is retired — asserts `!recurrence.expected.is_empty()` |

  `pulse-run-contract.toml` (P-073) has **no** pinning test — no test in `crates/` asserts its declaration.
  The `pulse-run-contract` name hits in `conductor-core/src/run_contract.rs` and
  `conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` reference `contracts/pulse-run-contract.toml`, a
  different artifact; read and excluded.

**Not in the modify-set, each measured:** `coverage.rs` / `drift.rs` (all seven P-IDs behind the five
already-retired declare-only scenarios remain `Auto`; `check_scenario_backing` keys on naming) · the garde
annotations and `Scenario::check_*` arms (no config rule changes; none of the four declares `[[checklist]]`,
so `check_checklist`'s trigger is untouched) · `conductor-run/src/execute.rs` (the seam already handles
zero-`expected` correctly) · any harvest test (no `conductor-run/tests/*_harvest.rs` names any of the four).

## Scope premise closure
- **Members 1–3 mechanism — VERIFIED at Pulse HEAD `83d4060`** (unmoved since the last wrap).
  `pulse-app/src/inference_runtime.rs:871` still writes `span_ids: Vec::new()`, and it is the SOLE non-test
  writer — every other writer in the Pulse tree sits inside a `#[cfg(test)]` module (markdown.rs:507 >
  cfg@376 · assembler.rs:802 > cfg@165 · retrieval.rs:170 > cfg@150 · persistence.rs:359 > cfg@335 ·
  registry.rs:400 > cfg@376). `incidents_router.rs:67` derives `evidence_count` from `.span_ids.len()`;
  `mcp-server/src/tools.rs:430-435` maps it 1:1 into `span_refs` (**measured coordinate — the residual cited
  `:430-439`**). `0 >= 1` / `0 >= 3` is false in every world, and `slo.rs` routes the unmet floor to
  `CalibrationRegion` regardless of the declared `Hard`. The `[inferred]` tag is dropped.
- **Member 4 ground — FALSIFIED** `[premise-corrected: the token IS emitted and DOES reach the graded text;
  the check fails on a one-character case mismatch]`. `interpretation/src/markdown.rs:221` pushes
  `"## Previously Seen"`; `conductor-verify/src/extract.rs:101-103` pushes each incident's `retrieve_report`
  markdown into `observation.text`; `slo.rs:51` makes `Contains` case-sensitive. The scenario asserts
  `"Previously seen"`. The live DEAD measurement stands; its recorded CAUSE does not. scope.md amended.
- **The residual's blind-spot hypothesis** stays `[inferred]` and explicitly unproven — recorded so a later
  reader does not mistake it for a finding.
- **Extract override:** the security, arch, obs and tests extracts each lean on "the four are structurally
  dead" as one class. That premise now holds for members 1–3 only. Any extract content treating member 4 as
  structurally dead is OVERRIDDEN by this closure for P4 synthesis.

## Two stale spec-master claims (verified by reading, not inherited)
Both extracts asserted these about their own sources; both reproduce at the cited artifact.
- `architecture.md:135` (offset 652): "the delegated-timing family (… 2026-08-21 — **`findings-counter-refresh`
  is NOT a member: it carries one `[[expected]]`**)". Retiring member 2 falsifies it.
- `obs-plan.md:349` (offset 1351): "**`findings-counter-refresh` is not declare-only (one `[[expected]]`)** and
  reaches KnownResidual by the same degraded read-back, its unmet `CountAtLeast` floor grading
  `CalibrationRegion`." Retiring member 2 falsifies it.
Both are `Expected amendments (wrap)`; neither is a touchpoint.

## Open questions
1. **Member 4's disposition** — retire to declare-only on the corrected ground, or correct the case
   (`"Previously Seen"`) and keep the check live? The route entry's wording ("one Hard Contains to
   declare-only") was authored on the now-disproved cause, and correcting the case makes the check
   *reachable* but not *proven* (the recurrence path's firing was never measured, and this chunk drives
   nothing). → blocks: **plan-decision** — resolved at P4 before synthesis.
2. **The two suite-shape tests' disposition** (`:1207`, `:1429`) — both assert a family "carries both
   shapes", a property the retirement dissolves rather than breaks. Re-scoping each to the family's new true
   shape preserves a guard where deleting removes one (test-plan §11 Quality). `:1207` is contingent on
   question 1. → blocks: **plan-decision**, dependent on 1.
