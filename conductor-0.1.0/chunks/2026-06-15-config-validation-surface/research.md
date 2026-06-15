# Codebase Research — 2026-06-15-config-validation-surface

## Scope
- **Depth:** moderate · **Reads:** 8 (dependency-tree, api-surface, workspace `Cargo.toml`, `conductor-core/Cargo.toml`, `scenario.rs`, `error.rs`, `lib.rs`, prior-chunk `scope.md`) · **Globs/Greps:** 1 (`crates/conductor-core/**/*.rs`)
- Codebase is early but NOT cold-start: `conductor-core` already holds the exact identity types this chunk validates. Reads were targeted at the four files the extracts flagged to modify.

## Files inspected
- `crates/conductor-core/src/scenario.rs` (full) — `PId(pub String)` (serde-`transparent`); `SloTier` (closed enum, type-validated already); `Scenario { name: String, p_ids: Vec<PId>, seed: u64, slo_tier: SloTier }`. **Doc-comments explicitly say garde validation "attaches in the config-validation chunk"** (lines 5-6, 33) — this is THE file for the garde derives. Existing serde round-trip tests must keep passing.
- `crates/conductor-core/src/error.rs` (full) — `CoreError` is `#[derive(Debug, Error)] #[non_exhaustive]` with one variant `Config(String)`. **Doc-comment (line 13) explicitly reserves "a `garde::Report` `#[from]` variant" for this chunk.** `pub type Result<T> = …<T, CoreError>` alias present. Tests assert Display + the Ok-carries-outcomes wall.
- `crates/conductor-core/src/lib.rs` (full) — module root: `mod {error, report_state, scenario, verdict}` + `pub use` re-exports. Advertises **"Plain data types: no async, no I/O."** A new path-guard module + re-export would be added here.
- `Cargo.toml` (workspace, full) — **`garde = "0.23.0"` is ALREADY in `[workspace.dependencies]` (line 46)** — declared by the scaffold, default features (so `derive` is available). serde/thiserror/anyhow/serde_json all present.
- `crates/conductor-core/Cargo.toml` (full) — deps: `serde.workspace`, `thiserror.workspace`; dev-dep `serde_json.workspace`. **No `garde` edge yet** — must add `garde.workspace = true`.
- `.andromeda/context/api-surface.md` — confirms current public surface; `conductor-cli`/`conductor-tauri` are **bin-only crates with no lib / no public API** (relevant to the path-guard placement question).
- `.andromeda/context/dependency-tree.md` — `conductor-core` depends only on serde + thiserror today; every seam depends on core (it is the dep root).

## Patterns detected
- **Per-concern module + re-export** (`lib.rs:9-17`): each type family is its own `mod` (`scenario`/`error`/`verdict`/`report_state`) with a flat `pub use`. A path-guard helper follows this as a new `mod config_path` + `pub use`.
- **In-file `#[cfg(test)] mod tests`** (`scenario.rs:46`, `error.rs:26`): plain `#[test]` unit tests co-located in the source file; `serde_json` is the only dev-dep. No `#[rstest]`/nextest yet (they land in the later "Test framework + fixtures" chunk).
- **`#[non_exhaustive]` extension-point discipline** (`error.rs:15`): `CoreError` was deliberately left open with a doc-comment naming the exact future variant — extend, don't restructure.
- **serde-`transparent` newtype** (`scenario.rs:13`): `PId` wraps a bare `String`; a garde derive on `PId` applies field rules to `.0`.
- **Workspace-inherited deps** (`conductor-core/Cargo.toml:8-9`): crates use `<dep>.workspace = true`; never a direct version pin in a member crate.

## Conventions to follow
- **Validation co-located with the serde structs** in `conductor-core` (arch §Validation Library; matches the existing one-mod-per-concern layout).
- **Add the dep via the workspace edge**: `garde.workspace = true` in `conductor-core/Cargo.toml` (mirrors `serde.workspace = true` at `:8`). Do NOT re-pin the version.
- **Extend `CoreError` in place** with `#[from] garde::Report` (`error.rs` — the reserved `#[non_exhaustive]` slot); keep `Config(String)`. garde's `Report` impls `Error` + `Display`, so `#[from]` + `#[error(...)]` compose.
- **Plain `#[test]` in-file tests** (not `#[rstest]`/nextest); run with `cargo test -p conductor-core` (handoff: nextest/rstest not installed until a later chunk).
- **Status/verdict wall untouched** — validation failure is `Err(CoreError)`, never a `Verdict`/`ReportState`.

## New files to create
- `crates/conductor-core/src/config_path.rs` — the `CONDUCTOR_*` path-handle guard: a pure `resolve_under(base, candidate) -> Result<PathBuf, CoreError>`-style helper (`std::fs::canonicalize` + bounds-check that the resolved path stays under the permitted base; rejects `../` traversal / out-of-scope absolutes). Sync, no env-reading (env wiring is the CLI-edge chunk). **(Placement pending the P4 open question below.)**

## Files to modify
- `crates/conductor-core/src/scenario.rs` — derive `garde::Validate` on `Scenario` + `PId`; `#[garde(length(min = 1))]` on `p_ids` + `#[garde(dive)]` into each `PId`; `PId` format/range rule (`P-NNN`, 001..060) via a custom validator (avoids the regex feature/dep); add the struct-level `#[garde(custom(…))]` worked example. Keep existing serde derives + tests.
- `crates/conductor-core/src/error.rs` — add the `#[from] garde::Report` variant (ConfigError class) to `CoreError`.
- `crates/conductor-core/src/lib.rs` — `mod config_path;` + `pub use` the resolver (if placed in core); update the "no I/O" doc-note to acknowledge the one sync-fs helper.
- `crates/conductor-core/Cargo.toml` — add `garde.workspace = true` to `[dependencies]`.

## Open questions
1. **Path-guard placement (P4 — AskUserQuestion candidate).** Security plan says canonicalize "at the `conductor-cli` edge"; arch says "core/CLI boundary"; but `conductor-core` advertises "no I/O" and `conductor-cli` is a bin-only stub (no lib to host a reusable helper). **Recommendation:** put the pure `resolve_under(base, candidate)` logic in `conductor-core::config_path` (reusable by cli + tauri + report, unit-testable now), *called* at the CLI edge later — honoring "validated at the edge" while keeping it DRY. The alternative is a cli-only helper (defers reusability, needs a cli lib).
2. **`canonicalize` requires existence.** A not-yet-created `runs/` dir can't be canonicalized directly — design the guard as "canonicalize the existing base, then verify the joined candidate stays under it," not "canonicalize the full target." (Impl note for andromeda-implement; shapes the `resolve_under(base, …)` signature.)
3. **Worked custom-validator example.** No *cross-field* invariant exists over the current identity fields (the real p50≤p95≤p99 / error-fraction rules need Epoch-2 emission fields). **Recommendation:** use **no-duplicate-P-IDs** as the `#[garde(custom)]` worked example — a real, useful collection invariant that authentically exercises the struct-level custom-validator mechanism the Epoch-2 rules will extend.
