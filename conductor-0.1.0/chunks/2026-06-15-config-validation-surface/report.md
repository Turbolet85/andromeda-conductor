# Report — 2026-06-15-config-validation-surface

**Chunk:** Config-validation surface — garde Validate on scenario model, garde Report→CoreError bridge, CONDUCTOR_* path canonicalize
**Date:** 2026-06-15T15:45:15Z
**Commits:** none yet — this wrap creates the chunk commit (prior commit `bc2a9f0` was the previous chunk)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/scenario.rs`, `crates/conductor-core/src/error.rs`, `crates/conductor-core/src/config_path.rs` (new), `crates/conductor-core/src/lib.rs`, `crates/conductor-core/Cargo.toml`, `Cargo.toml` (workspace), `Cargo.lock`.
- **Symbols / APIs:**
  - `conductor-core`: new `pub fn resolve_under(base: &Path, candidate: &Path) -> Result<PathBuf>` (re-exported at crate root); `Scenario` + `PId` now `impl garde::Validate`; new `CoreError::Validation(#[from] garde::Report)` variant; private validators `pid_format`, `no_duplicate_pids`.
  - No new IPC methods / endpoints / event topics / ports / sockets.
  - No new env vars — the `CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_CONTRACT_MANIFEST` handles `resolve_under` guards are already in arch §Occupied Resources.
- **Crates / modules:** no new crate; new private module `conductor-core::config_path` (one `pub fn` re-exported). No cross-seam dependency edges added.
- **Dependencies:**
  - `garde` **downgraded 0.23.0 → 0.22.1** in workspace `[workspace.dependencies]` (user-authorized this session — `garde_derive 0.23.0` is absent from the registry, so garde 0.23.0 + the `derive` feature is unbuildable).
  - `garde` wired into `conductor-core` `[dependencies]` with `features = ["derive"]` (garde's `derive` is NOT a default feature).
  - Transitive (Cargo.lock): `garde_derive 0.22.1` added; `compact_str` pinned 0.9.1 → 0.8.2 (garde 0.22.1's requirement). No other lock drift.
- **Schema / config:** no DB schema, no config keys, no violation schemas. `Scenario`/`PId` gain garde rules: non-empty `p_ids` (`length(min=1)`), `P-NNN` with NNN ∈ 001..=060 (`pid_format` custom), no-duplicate `p_ids` (`no_duplicate_pids` field custom), non-empty `name`.
- **Coverage of new surfaces:**
  - `Scenario`/`PId` config deserialization → validation **garde✓** · instrumentation **n/a** (load-time, not one of the 7 must-trace ops; the tracing/logging stack is a later chunk) · PII **n/a** · tests **unit✓** (positive + negative matrices) · a11y **n/a** · tokens **n/a**
  - `resolve_under` CONDUCTOR_* path guard → validation **✓** (`std::fs::canonicalize` + traversal/absolute rejection, OUTSIDE garde per security §Input Validation) · instrumentation **n/a** · PII **n/a** (error messages carry no absolute paths / struct names) · tests **unit✓** (in-scope / `../` traversal / out-of-scope absolute) · a11y **n/a** · tokens **n/a**

## Deviations from intent
- **garde 0.23.0 → 0.22.1** (plan + arch §Stack assumed 0.23.0). Justification: `garde_derive 0.23.0` absent from the registry (latest `0.22.1`), so 0.23.0 + `derive` is unbuildable; user authorized the downgrade this session. **Spec impact (this wrap must amend): arch §Stack + §Established Decisions [Validation Library] + §Inherited Defaults garde version → 0.22.1; cascade `.claude/docs/stack.md`. security-plan §Dependency Security if it pins the garde version.**
- **`features = ["derive"]` required** on the garde edge (plan assumed `garde.workspace = true` sufficed). Minor; in-scope (`conductor-core/Cargo.toml`).
- **no-duplicate-P-IDs validator: struct-level → field-level `#[garde(custom)]`**. Justification: garde 0.22.1 has no container-level `custom` ("unrecognized attribute"); moved onto the `p_ids` field (functionally identical). Was the planned fallback.
- **Test runner: plain `cargo test` + `#[test]`**, not nextest/rstest. Justification: intent-consistent interim — the "Test framework + fixtures" chunk (later in Epoch 1) installs nextest/rstest; test-plan §2/§4 describes the *target* framework, not a per-chunk mandate before that chunk lands. (Recurring: the prior chunk did the same.)

## Decisions & corrections
- **User decision:** resolve the garde 0.23.0 spec↔reality gap by downgrading to garde 0.22.1 (latest with an available `garde_derive`) rather than repairing the registry or re-planning.
- **Recurring pattern:** early Foundation chunks use `cargo test` / `#[test]` until the test-framework chunk lands nextest/rstest → candidate `playbook.md` rule (so D-tests-framework stops flagging it).
- **garde API facts (verified against 0.22.1 source):** `fn validate(&self)` takes no context arg; error type is `garde::Report`; `custom` is field-level only (no container `custom`); `derive` is a non-default feature. These shaped the impl.
- **Forward note (Epoch-2 `Scenario-config model`):** genuine cross-*field* invariants (p50≤p95≤p99 across distinct fields) need garde's `Context` pattern or a manual `Validate` impl, since 0.22.1 has no container `custom`.

## Outcome
- **Acceptance criteria: met.** garde `Validate` on `Scenario`/`PId` (non-empty `p_ids`, `P-NNN`/001..060, no-duplicate, non-empty name); `CoreError::Validation(#[from] garde::Report)` bridge (validation failure = `Err`, `Verdict`/`ReportState` stay `Ok` values); `resolve_under` traversal guard (in-scope ✓, `../` traversal ✗, out-of-scope absolute ✗).
- **Gates green:** `cargo test -p conductor-core` → 20 passed · `cargo clippy --workspace --all-targets -- -D warnings` → clean · `cargo build --workspace` → green.
- **Smoke:** skipped — no boot-path change (conductor-core is a library; no binary/entry-point touched; no `agent-run.sh` in Test Commands).
