# Codebase Research — 2026-09-15-remaining-structurally-dead-declarations-retired

## Scope
- **Depth:** deep · **Reads:** 11 (4 offset-bounded into `scenario.rs`, 5 scenario TOMLs, 2 Pulse-side) · **Globs/Greps:** 14 · **Graph queries:** 2 (`rust` plane)
- **Harness rules consulted:** none — no live leg in this chunk (every gate is `cargo`-tier). Noted for the record: `.claude/rules/testing.md` and `observability.md` auto-loaded on reading `crates/conductor-core/src/scenario.rs`; `testing.md`'s 2026-09-10 addition independently states CARRY 2's wide-sweep rule and its measured `crates/**/tests` false negative.

## Files inspected
- `crates/conductor-core/src/scenario.rs` (`:545-640`, `:640-760`, `:1238-1363`, `:1360-1377`) — **the single check-membership pin site for all five scenarios.** Carries both the live pins and the four prior declare-only retirement precedents.
- `crates/conductor-core/src/drift.rs` (`:155-185`) — `check_scenario_backing`'s signature and body; settles the coverage-gate question.
- `scenarios/{activity-floor,service-went-silent,high-severity-log-capture,exception-event-capture,threshold-hot-reload}.toml` — the six `[[expected]]` blocks, read whole.
- `scenarios/cross-incident-recurrence.toml` (`:1-40`) — the DECLARE-ONLY header precedent from the prior chunk.
- `../andromeda-pulse` @ `83d4060`: `crates/interpretation/src/markdown.rs` (`:115`, `:249`, `:358-368`), `crates/mcp-server/src/tools.rs` (`:360-392`), `crates/triage/src/contract.rs` (`:181`), `crates/triage/src/lifecycle/mod.rs` (`:57`, `:103`), `pulse-app/src/deterministic_inference.rs` (`:42-43`).

## Graph impact
Plane `rust`, `db_state` warm, all `probe_hits` > 0 (both queries returned rows, so no 0-row interpretation is in play). Graph lines are 0-indexed; editor lines below are `line + 1`.
- **`check_scenario_backing` / `check_sut_drift`** — 19 caller rows, all tests: 15 in `crates/conductor-core/src/drift.rs`, 2 in `crates/conductor-report/tests/coverage_gate.rs` (`:42`, `:59`), 2 crate-root re-export rows at `crates/conductor-core/src/lib.rs:40`. **No production caller threads a value this chunk changes.**
- **`UNBACKED_AUTO`** — 14 references across `conductor-cli/src/render.rs`, `conductor-core/src/{drift,lib}.rs`, `conductor-report/src/coverage.rs`, `conductor-tauri/src/commands.rs`. All read the ledger as classification data; none reads a scenario's `expected`.

**Consequence (the load-bearing equality, verified at HEAD):** `check_scenario_backing(classification, scenario_p_ids, known_unbacked)` computes `auto.difference(&covered)` where `covered` is built **only** from `scenario_p_ids` (`drift.rs:165`). It keys on the P-IDs a scenario **names**, never on whether that scenario grades anything. This chunk changes no `p_ids` and deletes no scenario ⇒ `covered` is byte-identical ⇒ **the coverage-completeness gate is unaffected by construction.** This answers the tests extract's open question and confirms the arch extract's claim.

## Patterns detected
- **The declare-only retirement test shape** (`scenario.rs:569-611`, `:698-721`, `:1278-1295`) — a retired member gets its family loader's assertion INVERTED to `s.expected.is_empty()` with a message naming the declare-only standing, plus (where a tier was re-declared) an `slo_tier` pin. The measurement lives in the test's doc comment AND the scenario TOML header.
- **The DECLARE-ONLY TOML header shape** (`scenarios/cross-incident-recurrence.toml:24-36`) — marker-attributed header, numbered mechanism statements with `file:line` citations into the SUT, and an explicit record of what was deliberately NOT done and why. Confirmed as the prior chunk's uniform precedent.
- **Mixed-shape family guards** (`scenario.rs:1334-1368`, and the retired severity-lifecycle guard `testing.md` records) — a family carrying both shapes asserts `any(empty) && any(!empty)`. When a family's last non-empty member retires, the guard is RETIRED rather than weakened; the severity-lifecycle precedent replaced it with `is_empty()` on all members.

## Conventions to follow
- **Repo-root artifacts resolve via `env!("CARGO_MANIFEST_DIR")` + `../../`** (`scenario.rs:557` and every sibling) — a test binary's CWD is its own package root (`testing.md` 2026-09-06).
- **Pins assert the seam's public contract, never fixture internals** — every existing pin reads `Scenario::from_toml_str(...)` and asserts on `expected`/`p_ids`/`slo_tier` (`testing.md` §Patterns).
- **`all()` over an empty iterator is `true`** — an `all(class == Hard)` guard on a retired scenario passes vacuously and asserts nothing; the project's own vacuous-green discipline (`testing.md` 2026-06-22 ext. 2026-08-16) makes retiring it the correct move, not leaving it green.

## New files to create
- None.

## Files to modify
| Path | Change |
|---|---|
| `scenarios/activity-floor.toml` | remove the `Absent "ServiceWentSilent"` block (`:67-70`); add DECLARE-ONLY header |
| `scenarios/service-went-silent.toml` | remove the `Contains "ServiceWentSilent"` block (`:35-38`); add DECLARE-ONLY header |
| `scenarios/high-severity-log-capture.toml` | remove BOTH blocks (`:34-37` Contains `"ERROR"`, `:40-43` Absent `"WARN"`); add DECLARE-ONLY header |
| `scenarios/exception-event-capture.toml` | remove the `Contains "exception"` block (`:24-27`); add DECLARE-ONLY header |
| `scenarios/threshold-hot-reload.toml` | remove the `Absent "RetroactiveReeval"` block (`:37-40`); add DECLARE-ONLY header |
| `crates/conductor-core/src/scenario.rs` | **eight** test changes, enumerated below |

### The eight pin changes in `scenario.rs` (enumerated from the wide sweep + reads, not from memory)
1. `:551 hard_signal_fixtures_load_and_validate` — **must SPLIT.** Four `#[case]` rows; `exception-event-capture` and `high-severity-log-capture` retire while `span-status-error-detection` and `root-span-error-scope` survive. Its `!s.expected.is_empty()` (`:563-566`) holds for the survivors and fails for the two retirees.
2. `:631 p007_high_severity_log_capture_asserts_both_sides_of_the_boundary` — asserts `any(Contains)` + `any(Absent)`; both retired ⇒ re-shape to the declare-only form.
3. `:653 activity_floor_and_silence_fixtures_load_and_validate` — `!s.expected.is_empty()` ⇒ invert to `is_empty()`.
4. `:673 activity_floor_and_silence_checks_are_all_hard` — `all(class == Hard)` becomes **vacuously true** on an empty vec ⇒ retire (it would otherwise stay green while asserting nothing).
5. `:723 activity_floor_asserts_the_learned_quiet_via_absent` — asserts `any(Absent)` ⇒ retire.
6. `:740 presence_scenarios_assert_a_surfaced_incident_via_contains` — its only case is `service-went-silent` ⇒ retire.
7. `:1297 findings_counter_is_declare_only_and_threshold_reload_keeps_its_hard_absent` — asserts threshold-hot-reload's `Hard Absent` (`:1325-1331`) ⇒ re-shape **and rename** (its name asserts the surviving half this chunk removes).
8. `:1334 scrub_pipeline_degraded_suite_mixes_hard_and_declare_only` — **breaks.** `threshold-hot-reload` is that family's LAST non-empty member (measured: `grep -c '^\[\[expected\]\]'` over its six stems ⇒ only `threshold-hot-reload` non-empty), so `assert!(any(!empty))` (`:1364-1367`) becomes false ⇒ retire the mixed-shape guard per the severity-lifecycle precedent.

`:1239 scrub_pipeline_degraded_fixtures_load_and_validate` carries no blanket expected-assertion (`:1257-1258`) ⇒ **no change**.

### Companion sweep record (CARRY 2's wide form, whole-repo)
`grep -rln "<name>" .` excluding `.andromeda/` · `runs/` · `target/` · `node_modules/` · version dirs:
- `activity-floor`: **15 files · 1 changed** (`scenario.rs`) · **14 no-change** — 8 are doc comments describing the P-013/P-014 fault family (`conductor-faults/src/{train,silence,lib}.rs`, `conductor-core/src/phase_spec.rs`), 4 are load-envelope duration fixtures / envelope-exemption note strings (`conductor-core/src/load_envelope.rs`, `conductor-cli/src/render.rs`, `conductor-report/src/{report,db}.rs`, `contracts/pulse-load-envelope.toml`), 2 are curated prose corpora (`.claude/docs/session-learnings.md`, `.claude/rules/{testing,verification-harness}.md` — wrap's territory, never a phase edit).
- `service-went-silent` · `exception-event-capture` · `threshold-hot-reload`: **1 file each · 1 changed** (`scenario.rs`).
- `high-severity-log-capture`: **2 files · 1 changed** (`scenario.rs`) · **1 no-change** — `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md:73`, the `a11y:sr` fixture roster's "short third scenario, which never executes". A roster line, not a check-membership pin (a11y extract's acceptance contribution); the fixture needs the scenario committed and validated, which retirement preserves.

**The narrow form returns a false zero:** `grep -rln "<name>" crates/**/tests` finds nothing for all five, because every pin sits in an inline `#[cfg(test)]` module under `src/`. CARRY 2's insistence on the wide form is measured-correct.

### Mechanism-claim re-derivation (all six grounds, at Pulse HEAD `83d4060`)
The SUT is still at the commit the enumeration measured, so these are confirmation spot-checks, not re-measurements. All six **VERIFIED**; two scope premises resolved.
- **fact 2** (`CueKind` snake_case) — `crates/triage/src/contract.rs:181` still carries `#[serde(rename_all = "snake_case")]`. Grounds `activity-floor` + `service-went-silent`. ✓
- **fact 3** (deterministic-L4 constants + lowercase severity) — `pulse-app/src/deterministic_inference.rs:42-43` pins `"severity": "autonomous"` / `"title": "Deterministic verification incident"`; `interpretation/src/markdown.rs`'s `severity_label` maps to `"info"|"warn"|"error"|"critical"`, all lowercase. Grounds both `high-severity-log-capture` blocks. ✓
- **scope premise 1 — RESOLVED, concern dissolves.** The worry was that `exception`'s ground rested on one file where the claim spans every composed-text field. Re-derived: `dispatch_retrieve_report` (`mcp-server/src/tools.rs:360-387`) composes markdown through `assemble_report` → `serialize_report`, and **both live in `crates/interpretation/src/markdown.rs`** (`:249`, `:115`) — that one file IS the sole composer. `git grep -c --fixed-strings 'exception' -- 'crates/interpretation/src/markdown.rs'` ⇒ exit 1, no matches. The enumeration's basis was sufficient. ✓
- **scope premise 2 — RESOLVED, the tree-wide ground is the true one.** `git grep -c --fixed-strings 'RetroactiveReeval' -- '*.rs'` ⇒ **exit 1, no matches** (read from the BARE command). No snake_case sibling exists either; the only `retroactive` hits tree-wide are two doc comments at `crates/triage/src/lifecycle/mod.rs:57,103`, both stating Pulse applies threshold changes FORWARD ("no retroactive re-classification"). So the ground is a genuine inferred-token absence — no producer ever emits the token — and **not** the serde-rename mechanism that explains its siblings. The header must say so. ✓

## Open questions
1. **A committed contradiction inside the prior chunk's own output, which this chunk must resolve.** `scenario.rs:1299-1302` (authored 2026-09-15) states P-056's `Hard Absent` "is untouched — it grades composed read-back text, not the count, and **remains satisfiable**", while the same chunk's committed enumeration classifies `threshold-hot-reload`'s `Absent "RetroactiveReeval"` as structurally dead in the **vacuous-PASS** direction. Re-derivation above settles it in the enumeration's favour: the token has zero occurrences tree-wide, so the check can never fail and grades nothing. → blocks: **plan-decision** — the plan must retire the check AND correct that comment, and should not reproduce its "remains satisfiable" wording anywhere.
2. **Does retiring the `scrub_pipeline_degraded` mixed-shape guard need operator ratification?** The severity-lifecycle precedent (`testing.md` 2026-06-22, RETIRED 2026-08-21) replaced such a guard with `is_empty()` across all members when the family went wholly declare-only — the same situation. → blocks: **plan-decision** — proposed as a decisive material lean on that precedent, surfaced at the P5 review card rather than asked.
