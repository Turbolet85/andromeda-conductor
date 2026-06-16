# Codebase Research — 2026-06-16-test-framework-fixtures-coverage-tooling

## Scope
- **Depth:** moderate · **Reads:** 10 · **Globs/Greps:** 4
- Mature codebase (8 crates, 36 green tests, `deny.toml` live) — targeted reads, not blanket. Three path-scoped rules
  (`testing.md`, `observability.md`, `verification-harness.md`) auto-loaded and are authoritative for this chunk.

## Files inspected
- `Cargo.toml` (full) — workspace manifest; `resolver="3"`, `rust-version=1.94.1`, `publish=false`. `[workspace.dependencies]`
  carries tokio/opentelemetry-proto/tonic/rmcp/rusqlite/serde/garde/thiserror/anyhow/tracing/tauri — but **zero test crates**.
- `rust-toolchain.toml` (full) — `channel="1.95.0"`, `components=["clippy","rustfmt"]` — **no `llvm-tools-preview`** (cargo-llvm-cov requires it).
- `deny.toml` (full) — cargo-deny schema v2; `[licenses] allow = [MIT, Apache-2.0, Unicode-3.0, BSL-1.0]`, `confidence-threshold=0.8`;
  `[advisories] ignore=[]`; `[bans] multiple-versions="warn"`, `wildcards="deny"`. New dep licenses outside the allowlist fail the gate.
- `crates/conductor-core/Cargo.toml` (full) — `[dependencies]` only (serde/serde_json/thiserror/garde+derive/tracing/tracing-subscriber);
  **no `[dev-dependencies]` section yet**.
- `crates/conductor-cli/Cargo.toml` (full) — `[[bin]] name="conductor"`, dep `conductor-core` only; no dev-deps.
- `crates/conductor-cli/src/main.rs` (full) — `fn main() { conductor_core::init_observability("conductor", None); }` →
  bin `conductor` exists, a valid `assert_cmd::Command::cargo_bin("conductor")` target (exits 0; emits a tracing JSON startup line to stderr).
- `crates/conductor-core/src/config_path.rs` (full) — `resolve_under(base, candidate)` + 3 plain `#[test]` using `env!("CARGO_MANIFEST_DIR")`;
  representative of the current test idiom.
- `.andromeda/context/dependency-tree.md` — 8 crates; `conductor-core` is the hub every crate depends on; 5 seam libs (timeline/emit/faults/verify/report) are placeholders.
- `.andromeda/context/api-surface.md` — `conductor-core` public fns available to exercise: `Verdict`/`ReportState` (`label()`, `status_prefix()`),
  `Scenario`/`PId` (`garde::Validate`), `SloTier`, `resolve_under`, `redact_value`, `sanitize_error`, `init_observability`, `mint_run_id`, `ServiceIdentity`.

## Patterns detected
- **All 36 tests live in `conductor-core` `#[cfg(test)] mod tests`** (verdict.rs:46, obs.rs:238, report_state.rs:52, scenario.rs:78, error.rs:29,
  redact.rs:126, config_path.rs:47) — plain `#[test]`, std-only, **no external test crates, no `tests/` integration dirs, no `tokio::test`** (grep across `crates/**/*.rs`). This chunk must add the toolchain **without disturbing those 36**.
- **Workspace-dep inheritance idiom** (`crates/conductor-core/Cargo.toml:9-14`): every dep is `name.workspace = true` (features added per-crate, e.g. `garde = { workspace = true, features = ["derive"] }`). New test crates follow `[workspace.dependencies]` → per-crate `.workspace = true`.
- **conductor-core is the only API-bearing crate** (api-surface.md) — the 5 seam libs + cli/tauri bins are near-empty, so exemplar tests that exercise *real* public fns (Verdict labels, redact_value, resolve_under, Scenario garde) belong in conductor-core and add genuine value vs throwaway smoke.

## Conventions to follow
- **Zero-retry determinism** (`testing.md` §Quality gates + `verification-harness.md` §Determinism): NEVER nextest `retries > 0`; the `ci` profile emits JUnit XML; nextest exit 100 = hard fail.
- **Fixed tool set** (`testing.md` §Frameworks): rstest (`#[fixture]`/`#[rstest]`/`#[case]`); proptest (regressions persisted under `proptest-regressions/`, committed); insta (CI/assert mode — **fail, don't write**); assert_cmd + assert_fs for CLI E2E; cargo-llvm-cov (`cargo llvm-cov nextest --lcov --fail-under-lines 60`).
- **`run` harness command** (`verification-harness.md`): `cargo nextest run --workspace --profile ci` + `cargo test --workspace --doc` + `cargo clippy --workspace --all-targets -- -D warnings`.
- **Supply-chain gate** (security extract + `deny.toml`): after adding deps, `cargo audit` + `cargo deny check advisories bans sources licenses` must stay green; extend `[licenses] allow` only additively + with a justifying comment, never a silent skip; commit `Cargo.lock`.

## New files to create
- `.config/nextest.toml` — `[profile.ci]` with explicit `retries = 0` + `[profile.ci.junit] path = "junit.xml"` + status/output settings (resolves the `--profile ci` gotcha).
- `crates/conductor-cli/tests/cli_smoke.rs` — assert_cmd exemplar (`Command::cargo_bin("conductor")?.assert().success()`) + assert_fs `TempDir` exemplar.
- Insta snapshot file(s) under `crates/conductor-core/src/snapshots/` — generated when the insta exemplar is first accepted; committed.
- Possibly `proptest-regressions/` under conductor-core — generated on first counterexample; committed per `testing.md` (must NOT be gitignored).

## Files to modify
- `Cargo.toml` — add `rstest`, `proptest`, `insta`, `assert_cmd`, `assert_fs` (and likely `predicates` for assert_cmd/assert_fs content checks) to `[workspace.dependencies]`.
- `crates/conductor-core/Cargo.toml` — new `[dev-dependencies]`: rstest/proptest/insta (`.workspace = true`).
- `crates/conductor-cli/Cargo.toml` — new `[dev-dependencies]`: assert_cmd/assert_fs (+predicates) (`.workspace = true`).
- `crates/conductor-core/src/*` — add rstest `#[case]` + proptest + insta exemplars exercising existing public fns (a new `#[cfg(test)]` block or a small `tests/` file), leaving the 36 existing tests untouched.
- `rust-toolchain.toml` — add `"llvm-tools-preview"` to `components` (cargo-llvm-cov requirement).
- `deny.toml` — only if `cargo deny check licenses` flags a new license post-add: extend `[licenses] allow` additively with a justifying comment.
- `Cargo.lock` — regenerated by the dep adds; committed + un-drifted.

## Open questions
- **Exemplar placement** — recommend conductor-core (rstest/proptest/insta on real public fns) + conductor-cli (assert_cmd/assert_fs). Resolve at P4 (not a scope ambiguity).
- **`predicates` companion dep** — include for assert_cmd/assert_fs string/content assertions, or keep the exemplar predicate-free via `.success()`? Minor; decide at P4.
- **tokio `test-util`/`macros` dev-features** — DEFER to the Epoch-2 timeline chunk (the `start_paused` determinism harness owns it); NOT in this chunk's 5-tool scope. Confirm at P4.
