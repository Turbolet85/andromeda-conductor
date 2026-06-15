# Codebase Research — 2026-06-15-conductor-core-shared-types

## Scope
- **Depth:** minimal–moderate (early Foundation chunk; `conductor-core` is a 1-line placeholder, but the workspace + pins are real). · **Reads:** 6 (`conductor-core/{lib.rs,Cargo.toml}`, root `Cargo.toml`, two living docs, two sibling seam `lib.rs`) · **Greps:** 1 (test-plan §10 coverage) · **Globs/ls:** 2 (`scenarios/`, `contracts/`).

## Files inspected
- `crates/conductor-core/src/lib.rs` (full) — single `//!` doc-comment placeholder; **no items**. This chunk replaces it with the real module tree + re-exports.
- `crates/conductor-core/Cargo.toml` (full) — `[dependencies]` is **empty**. This chunk adds `serde` (workspace, derive) + `thiserror` (workspace) as deps, and `serde_json` as a **dev-dependency** (round-trip tests).
- `Cargo.toml` (root, full) — `[workspace.dependencies]` pins `serde = { version = "1.0", features = ["derive"] }`, `thiserror = "2.0.18"`, `garde = "0.23.0"`, `anyhow`, `tracing*`. **`serde_json` is NOT pinned** — must be added to `[workspace.dependencies]` (the report seam will reuse it for the JSONL journal in Epoch 6; adding it now is the single-source-of-truth pattern).
- `.andromeda/context/api-surface.md` (full) — confirms `conductor-core` currently exposes "no public items yet — placeholder; `Verdict`/`ReportState` + scenario model land in the next chunk." This chunk is exactly that handoff.
- `.andromeda/context/dependency-tree.md` (full) — all 7 seams + 2 bins already edge to `conductor-core`; `conductor-core` itself has **no** outgoing crate edges yet. Adding serde/thiserror keeps it a leaf (no seam→seam edge).
- `crates/conductor-timeline/src/lib.rs`, `crates/conductor-report/src/lib.rs` (full) — confirm the established placeholder convention: each seam `lib.rs` is one `//!` module doc-comment. `conductor-core` is the first to grow a real module tree.

## Patterns detected
- **Placeholder lib convention** (`crates/*/src/lib.rs:1`): every seam crate is a single `//!` doc-comment today; no `mod`/`pub` items. This chunk introduces the first real public surface under it.
- **Centralized version pins** (`Cargo.toml:19-54`): all external deps are pinned in `[workspace.dependencies]`; member crates reference `{dep}.workspace = true` (per the scaffold plan). Follow this — add `serde_json` centrally, reference it as `serde_json.workspace = true` in the core dev-deps.
- **Canonical serde names are the variant names themselves** (arch §Standard Contracts envelope `"verdict": "Pass"` / `"state": "Pass"`): `Verdict`/`ReportState` variants are already PascalCase, so serde's default variant serialization is already canonical — **no `rename_all` needed** for those two. Only `SloTier` needs per-variant `#[serde(rename = "<5s")]` etc. (wire form is not a Rust identifier).

## Conventions to follow
- **No nextest / rstest yet** (handoff + testing rule): cargo-nextest + rstest land in the later Foundation chunk "Test framework + fixtures". This chunk's tests therefore use plain `#[cfg(test)] mod tests { #[test] … }` and run via **`cargo test -p conductor-core`** (the gate the scaffold used), NOT `cargo nextest run` and NOT `#[rstest]`/`#[case]`. The extracts' `cargo nextest` / rstest phrasing is the steady-state convention; here it degrades to `cargo test` + hand-written cases.
- **serde round-trip needs a concrete format** (`crates/conductor-core/Cargo.toml`): use `serde_json` (dev-dep) for `to_string`/`from_str` assertions against the canonical names.
- **Runtime-agnostic core** (CLAUDE.md + arch §Design Philosophy): no tokio/async/I/O in `conductor-core`; these are plain `#[derive]` data types + accessor methods only.
- **No design-token values in core** (reconciles the design extract): the design extract maps each state to a color/glyph, but itself says "the enum carries no color values." Keep hex/CSS/ANSI **out** of `conductor-core`; expose only `label()` + `status_prefix()` (cli ASCII). The state→color/glyph mapping is captured in **doc-comments** for downstream auditing; actual color/glyph rendering is the cli/GUI surfaces' job (Epoch 8/9).

## New files to create
- `crates/conductor-core/src/verdict.rs` (or `outcome.rs`) — `Verdict { Pass, Fail, CalibrationRegion }` + `label()`/`status_prefix()`.
- `crates/conductor-core/src/report_state.rs` — `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` + accessors. (Kept a **separate enum** from `Verdict` per design anti-pattern "never flatten the two.")
- `crates/conductor-core/src/scenario.rs` — `Scenario` core struct (name, required `p_ids`, seed), `PId` newtype, `SloTier { <5s/<20s/<90s }` (serde-renamed). serde-derive only; **no garde**.
- `crates/conductor-core/src/error.rs` — `CoreError` (thiserror) harness-fault enum; establishes the verdict/error wall. Possibly a `pub type Result<T> = …` alias.
- *(exact module names/granularity are P4's call — could collapse `verdict`+`report_state` into one `outcome` module.)*

## Files to modify
- `crates/conductor-core/src/lib.rs` — replace the bare doc-comment with `mod`/`pub use` re-exports of the new modules.
- `crates/conductor-core/Cargo.toml` — add `serde.workspace = true` + `thiserror.workspace = true` to `[dependencies]`; add `[dev-dependencies]` `serde_json.workspace = true`.
- `Cargo.toml` (root) — add `serde_json = "1.0"` (exact pin) to `[workspace.dependencies]`.
- `Cargo.lock` — regenerate + commit (adds serde_derive/serde_json/thiserror-impl resolution).

## Open questions
1. **`CoreError` variant set, this chunk vs. next.** garde's validation `Report` `#[from]` (the `ConfigError` class) belongs to the NEXT chunk ("Config-validation surface"), and this chunk loads no files (no parse I/O yet). Recommendation: define a **minimal foundational `CoreError`** now (establish the wall + a `Result` alias) with a clear extension point, and let the validation chunk add the garde `#[from]` variant. P4 to fix the exact starter variants (avoid an empty/uninhabited enum).
2. **Module granularity** — four small modules vs. a consolidated `outcome.rs`+`scenario.rs`+`error.rs`. P4 picks; either satisfies the contract. (The tests extract's `conductor_core::verdicts::…` path is illustrative, not binding.)
3. **Coverage threshold** is 60% line (test-plan §10 / testing rule `--fail-under-lines 60`) but coverage tooling (cargo-llvm-cov) installs in a later Foundation chunk — so coverage is a **target, not an enforced gate** this chunk. (Resolves the tests extract's `{threshold from §10}` placeholder.)
