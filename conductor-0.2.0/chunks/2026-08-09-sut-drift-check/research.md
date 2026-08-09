# Codebase Research — 2026-08-09-sut-drift-check

## Scope
- **Depth:** moderate · **Reads:** 7 files · **Globs/Greps:** 3 · **Graph queries:** 5 (trace: `.andromeda/runs/2026-08-09T11-52-59-phase/tree-query-2026-08-09-sut-drift-check.json`)

## Files inspected
- `crates/conductor-core/src/capability_manifest.rs` (full, 157 lines) — the whole consumed API. `load` (`:36`) reads → `toml::from_str` → `validate` → `tracing::info!(count = …)`; `accepts` (`:54`) is a linear `iter().any`. Error shape is `CoreError::Config(String)` with deliberate path hygiene (`:38-40`, `e.kind()` only). The test module carries the `manifest(&["P-001"])` fixture helper (`:93`) and `load_failure_message_never_contains_the_path` (`:120`) — the two shapes this chunk's tests should reuse.
- `crates/conductor-core/src/coverage.rs` (1-80 + tests grep) — `coverage_matrix() -> &'static [CapabilityRow]` (`:60`) over `static COVERAGE: [CapabilityRow; 60]` (`:73`). `CapabilityRow` is `&'static str`-backed, doc-stated "no allocation and no runtime IO" (`:46`). Module doc (`:3-4`) and the `p_id` field doc (`:49`) still say `P-001..P-060`.
- `crates/conductor-core/src/lib.rs` (full) — flat `mod` list + `pub use` block; adding a seam module is one `mod` line (`:16-31`) + one `pub use` (`:33-50`). `CoreError`/`Result` are re-exported from `error` (`:36`).
- `crates/conductor-core/src/redact.rs` (1-60) — **`ALLOWLISTED_FIELDS` (`:21-47`) confirmed first-hand:** it carries `p_ids`, `count`, `message`, `phase`, but **not** `sut_version`, `captured_at`, `missing_p_ids`, or `coverage_percent`. A non-allowlisted field name is dropped at the processor stage, so manifest metadata must ride inside `message` (or the allowlist is amended — an obs-plan §6 edit).
- `crates/conductor-report/src/coverage.rs` (1-110) — `CoverageMatrix::render()` (`:24`) is a pure function of `coverage_matrix()`; `summary_line` (`:62`) tallies from `rows.len()`, already count-agnostic. But two tests hardcode the current classification: `renders_title_summary_and_table_header` asserts the literal `"**Capabilities** 60 · 40 auto · 13 drive+observe · 7 static-only"` (`:89`) and `every_capability_renders_as_an_exact_row` asserts `.count(), 60` (`:104`).
- `crates/conductor-cli/src/paths.rs` (full) — the shipped manifest edge: `capability_manifest_path` resolved by `resolve_under` with **no** `CONDUCTOR_*` override (`:29`), and `capabilities()` → `CapabilityManifest::load` (`:34-36`). Any CLI-reachable drift check reuses this, adding no path input.
- `.github/workflows/ci.yml` (gate-step grep) — the single Rust job already runs build → flakiness budget → `agent-run` dogfood (test+lint) → `cargo audit` → `cargo deny` → coverage floor → obs conformance. A new gate step slots into this job; there is no second workflow.

## Graph impact
- **`coverage_matrix`** — **13 call rows**: `conductor-report/src/coverage.rs:24` (`CoverageMatrix::render`) + `:98` (test), `conductor-tauri/src/commands.rs:124` (+ `:276` test), `conductor-tauri/src/main.rs:23`, `conductor-cli/src/render.rs:162`. All are **read-only consumers of the classification** — a check that only reads it adds zero blast radius to them.
- **`CapabilityManifest`** — **75 reference rows**, concentrated at the three binary edges (`conductor-cli/src/paths.rs`, `conductor-tauri/src/commands.rs:67…111`, the catalog loader). Confirms the CARRY: the type is already the single source of the accepted set; a second reader would be the duplication to avoid.
- **`accepts`** — **10 call rows**; the sole production caller is `Scenario::check_capabilities` at `crates/conductor-core/src/scenario.rs:142` (the rest are that module's tests). Membership-checking is therefore an established, single-purpose path this chunk sits beside, not inside.
- **`%drift%`** — **0 rows.** Consulted-and-no-match: the name is free across the workspace; no existing drift concept to extend or collide with.
- **`crate_edges` for `conductor-core`** — **6 rows, all six inbound** (`conductor-run`, `conductor-cli`, `conductor-tauri`, `conductor-report`, `conductor-timeline`, `conductor-verify`); **zero outbound** — the base crate imports no workspace crate, so the query's bidirectional `OR` returned inbound rows only. Inbound-heavy, so a *breaking* core change is expensive — but an **additive** public fn has zero cross-crate impact, which is what this chunk should be.

## Patterns detected
- **Sanitized typed-fault construction** (`capability_manifest.rs:37-43`): every failure is `CoreError::Config(format!(…))` built from `e.kind()` or `crate::sanitize_error(&e)` — never the path, never `Debug`. The drift error should be minted the same way.
- **Static classification, zero IO** (`coverage.rs:46,60,73`): `coverage_matrix()` is infallible and allocation-free. A drift check comparing it against a *loaded* manifest is therefore fallible only on the manifest side — the classification side cannot fail.
- **Fixture-helper + hygiene-negative-test pair** (`capability_manifest.rs:93,120`): in-sync/drifted manifests are constructible via `manifest(&[…])` without touching the committed file; the path-leak negative test is the in-repo template for the artifact-hygiene assertion.
- **Count-agnostic renderer, count-hardcoded tests** (`conductor-report/src/coverage.rs:62-69` vs `:89,:104`): the render already scales to any classification size; only the *tests* freeze 60. Confirms the `v2-03` CARRY — the exactly-sixty assertions are that chunk's to move, not this one's.
- **Gate-step composition in one CI job** (`ci.yml:68-94`): each gate is a named step with its own `run:`; the `agent-run` dogfood step is the existing home for test+lint gating.

## Conventions to follow
- **Additive module in `conductor-core`**: one `mod` line + one `pub use` in `lib.rs:16-50`; no new crate, no new edge (crate_edges shows six inbound consumers that must stay unbroken).
- **Verdict/error wall**: `crate::Result<T>` = `Result<T, CoreError>`; drift is `Err`, never a `Verdict`/`ReportState` (`lib.rs:6-8` states the wall explicitly).
- **Deterministic ordering**: the drifted-id list must be stably ordered (manifest order is already deterministic — a `Vec<String>` read in file order) so the message is byte-stable under the zero-retry bar.
- **Obs field discipline**: only `p_ids`/`count`/`message` are available without an allowlist amendment (`redact.rs:21-47`).

## New files to create
- `crates/conductor-core/src/drift.rs` — the manifest↔classification comparison + its typed failure and unit tests. (Name confirmed free: 0 graph rows for `%drift%`.)

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod drift;` + `pub use drift::{…};`.
- *(conditional on the P4 surface decision)* `crates/conductor-cli/src/commands/coverage.rs` and/or `.github/workflows/ci.yml` — only if the check becomes operator/CI-reachable rather than test-only.

## Open questions
1. **Gate hardness.** On today's tree the check correctly reports 22 drifted ids (manifest 82 vs classification 60), so a hard gate lands RED and stays red until `v2-03`. The tests extract flags this against test-plan §10/§11 ("never skip quality gates just this once"). Hard-red-now, or surfaced-non-gating-then-hardened?
2. **Surface.** Test-only (`conductor-core` unit tier), or additionally reachable from the harness — extending the existing `conductor coverage` verb vs a `preflight`-style exit-code gate? This decides whether the design/layouts cli constraints bind at all.
3. **Direction of the comparison.** `v2-02`'s acceptance names one direction (manifest ids the classification lacks). Should the check also report the reverse (classification rows absent from the manifest — a capability Pulse *retired*)? Not required by the acceptance; cheap to include.
