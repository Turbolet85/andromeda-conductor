# tests extract

## Relevance
Relevant — this chunk directly implements test-plan.md §1's "Scenario catalog + coverage-matrix" testable entity and §6's coverage-matrix completeness gate (Critical Path 6), and it moves multiple existing literal-60 assertions across five crates.

## Constraints
1. **Zero-unclassified is the assertion, and the manifest is its source** — per test-plan.md §4 (What unit tests cover → "Scenario catalog + coverage-matrix") and §1 (Scenario catalog entity), the static assertion is that the matrix "enumerates every capability in the **SUT capability manifest** with zero unclassified entries". After this chunk no coverage test may assert a literal count; the expected set must be derived from `contracts/pulse-capabilities.toml`. This is already the amended wording (see amendment history 2026-08-08).
2. **A missing P-XXX is a hard `Fail` (non-zero exit)** — per §6 Scenario "Coverage-matrix completeness gate": assert via assert_cmd exit code + an insta golden / static row-count check. The completeness gate is one of the few things that IS a hard non-zero exit, unlike reported states.
3. **The fourth `CoverageMode` variant must not become a `Blocked`/`Fail` state** — per §11 E2E (stack-specific) the envelope's `state`/`verdict` vocabulary is fixed (`Pass|Fail|ManualCheck|KnownResidual|Blocked` / `Pass|Fail|CalibrationRegion`, §3 Status endpoint shape). `CoverageMode` is a classification axis, not a verdict axis; adding a variant must not widen the envelope enums, and the envelope golden tests must stay byte-stable (§3 log-format binding: "format break = harness break").
4. **Unit tier owns this, per-seam** — per §2 pyramid + §4 Conventions: crate-local `#[cfg(test)] mod tests` in `conductor-core`, run by `cargo nextest run -p conductor-core`; the E2E leg is only the existing CLI completeness scenario (§6). No new E2E path is warranted (§1 caps at 7 critical paths).
5. **Serialization goldens use exact-string `assert_eq!` at unit level** — per §4 conductor-report bullet (as amended 2026-06-16): the new serde wire spelling for the fourth variant gets an exact-string canonical-serialization golden matching the `verdict.rs`/`report_state.rs`/`scenario.rs` pattern; insta stays the E2E journal mechanism only.
6. **Zero-flakiness / no lowered bar** — per §10: `--fail-under-lines 60`, `cargo audit --deny warnings` + `cargo deny check` green, committed un-drifted `Cargo.lock`, clippy `-D warnings`; no nextest `retries`. The chunk's DoD gate set matches §9's pipeline stages.
7. **Doc/artifact assertions must not leak internals** — per §3 Status endpoint shape + §11 Test Data: artifacts (incl. `coverage-matrix.md` and drift messages) carry identity fields only, no absolute host paths / internal struct names; `drift.rs`'s existing `drift_message_names_identity_without_host_paths_or_type_names` is the enforcing test and must keep passing with an empty ledger.

## Patterns to follow
1. **Manifest-relative de-hardcoding assertion** — `crates/conductor-core/src/scenario.rs:267` already establishes it: `assert!(!err.contains("001..=060"), "must not name a hardcoded range: {err}")`. Reuse this shape so a re-hardcoded range fails loudly.
2. **The full literal-60 assertion inventory is larger than scope.md states** (scope names only the two in `coverage.rs`). All of these move with the change:
   - `crates/conductor-core/src/coverage.rs:141` `matrix_has_exactly_sixty_capabilities` (`len(), 60`)
   - `crates/conductor-core/src/coverage.rs:146` `p_ids_are_contiguous_p001_to_p060_zero_gaps_no_dups` (`(1..=60).collect()`)
   - `crates/conductor-report/src/coverage.rs:89` exact-string footer golden `"**Capabilities** 60 · 40 auto · 13 drive+observe · 7 static-only"`
   - `crates/conductor-report/src/coverage.rs:104` `assert_eq!(md.lines().filter(|l| l.starts_with("| \`P-")).count(), 60)`
   - `crates/conductor-cli/src/render.rs:288` `coverage_table_renders_all_sixty_pids` (`for n in 1..=60`)
   - `crates/conductor-cli/tests/cli_smoke.rs:189` `coverage_lists_all_sixty_pids` (asserts stdout contains `P-001` and `P-060`)
   - `crates/conductor-tauri/src/commands.rs:317` `coverage_matrix_command_returns_all_sixty_pids`
3. **Drift ledger tests already carry the empty-ledger case** — `crates/conductor-core/src/drift.rs:171` `in_sync_with_an_empty_known_gap_passes` is the target end-state; `committed_artifacts_match_the_known_gap` (`:138`) degenerates to the plain zero-drift assertion, and `a_known_gap_entry_that_is_now_classified_is_drift` (`:153`) must keep using a synthetic ledger fixture, not the (now empty) `KNOWN_UNCLASSIFIED` constant.
4. **Table-driven rstest for the classification matrix** — per §4 Fixture pattern / §7: `#[rstest]` + `#[case]` rows over the manifest's accepted set is the sanctioned grouping for per-P-ID matrices, incl. the 22 new ids and their mode assignments.
5. **CLI assertions are NO_COLOR/pipe-stable text** — per §6 Selector strategy + the existing `piped_coverage_output_carries_no_ansi_escapes` (`crates/conductor-cli/tests/cli_smoke.rs:211`); any new mode label must assert on text, never color.

## Anti-patterns to avoid
1. **NEVER re-hardcode a count or a range** in any coverage assertion — §4/§1 mandate the manifest-derived accepted set; a literal `60`, `82`, or `1..=N` reintroduces exactly the defect the 2026-08-08 amendment recorded.
2. **NEVER test implementation details** (§11 Unit) — assert the public seam API (`coverage_matrix()`, `CoverageMode::ALL`/`label`/serde wire, the rendered artifact), not the private `static COVERAGE` layout.
3. **NEVER lower the coverage threshold or skip a gate** (§11 Quality) — the +22 rows and new variant arms must be covered by the table-driven cases rather than by relaxing `--fail-under-lines 60`.

## Contract bindings
- **tests §3 Log format ↔ obs §3** — the Run-report envelope / JSONL journal schema is owned by test-plan §3 and derived downstream by obs. This chunk must NOT alter envelope fields or enum vocabularies; if the fourth `CoverageMode` ever surfaces in the journal/envelope, that is a §3 change requiring an obs reconcile (the 2026-06-15 and 2026-06-24 amendments show this binding fires on any schema-shaped drift).
- **tests §6 completeness gate ↔ arch definition-of-done** — `coverage-matrix.md` is the artifact both sides key on; the CLI `conductor coverage [--write]` path (`crates/conductor-cli/src/commands/coverage.rs`) is the gate's driver.
- **tests §9/§10 ↔ security §Dependency Security** — `cargo audit`/`cargo deny` run in the same workflow the chunk's DoD gate set names.

## Acceptance criteria contributions
1. `(tests)` `cargo nextest run --workspace --profile ci` is green, and no coverage/classification test asserts a literal capability count or P-ID range — every expected set derives from `contracts/pulse-capabilities.toml` (§1 Scenario-catalog entity, §4).
2. `(tests)` All seven literal-60 assertions listed above (across `conductor-core`, `conductor-report`, `conductor-cli` unit + `cli_smoke`, `conductor-tauri`) are converted manifest-relative; `conductor-core::drift` passes with `KNOWN_UNCLASSIFIED == []` including `committed_artifacts_match_the_known_gap` and `drift_message_names_identity_without_host_paths_or_type_names` (§4, §11 Test Data).
3. `(tests)` The fourth `CoverageMode` variant has an exact-string serde-wire golden and a `label` assertion at unit level (§4 conductor-report bullet, per the 2026-06-16 amendment), and the envelope enums (`verdict`/`state`) are unchanged (§3).
4. `(tests)` Full gate set green: `cargo test --workspace --doc`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo audit --deny warnings`, `cargo deny check`, coverage `--fail-under-lines 60`, zero nextest retries, and `agent-run.sh run` (§9, §10).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest — Capability-manifest coverage + de-hardcoded catalog assertions** (§1 Scenario catalog entity · §4 · §6 completeness scenario · §7 fixtures). Catalog/coverage assertions were re-sourced from the SUT capability manifest instead of a hard-coded 60; a manifest loader/validator + membership-rejection coverage was added (8 new unit tests, workspace nextest 420 → 428). *Why:* "the plan mandated a static assertion over an accepted set the code no longer defines." This is the direct predecessor of this chunk — that amendment de-hardcoded the *scenario/catalog* side; the `coverage.rs` classification side is the remaining half, and the seven literal-60 assertions above are the residue that sweep did not reach.
- **2026-06-16-emission-journal-writer — unit serialization goldens use exact-assert; insta stays the E2E mechanism** (§4 conductor-report bullet). Governs how the new `CoverageMode` variant's wire spelling must be golden-locked (exact-string `assert_eq!`, not insta).
- **2026-06-15-structured-logging-stack — self-obs stream noted as distinct from the emission journal** (§3 Log format). Relevant as a guardrail: classification/rendering changes must not bleed into the §3-owned envelope schema, which is the obs-bound contract.
