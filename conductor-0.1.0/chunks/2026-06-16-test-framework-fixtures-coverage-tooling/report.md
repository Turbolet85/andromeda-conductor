# Report — 2026-06-16-test-framework-fixtures-coverage-tooling

**Chunk:** Test framework + fixtures + coverage tooling — cargo-nextest profiles (.config/nextest.toml, zero-retry ci) + rstest/proptest/insta fixtures + assert_cmd/assert_fs + cargo-llvm-cov
**Date:** 2026-06-16T16:46:23Z
**Commits:** none yet — this wrap creates the chunk commit (the work was implemented uncommitted)

## Changes (structured — detectors read this)
- **Files:**
  - New: `.config/nextest.toml`, `crates/conductor-core/tests/toolchain_smoke.rs`, `crates/conductor-cli/tests/cli_smoke.rs`
  - Modified: `Cargo.toml`, `Cargo.lock`, `crates/conductor-core/Cargo.toml`, `crates/conductor-cli/Cargo.toml`, `rust-toolchain.toml`
- **Symbols / APIs:** none — no public fns / IPC methods / endpoints / exports / ports / env vars added or changed. Test code + config + dev-deps only; the 36 existing `conductor-core` unit tests are untouched.
- **Crates / modules:** no crates added/removed. Two integration-test targets added: `conductor-core/tests/toolchain_smoke.rs` (rstest/proptest/insta exemplars over conductor-core's public API) and `conductor-cli/tests/cli_smoke.rs` (assert_cmd/assert_fs over bin `conductor`). No `src/` modules changed.
- **Dependencies (dev-only) added** to `[workspace.dependencies]` + inherited per-crate (`conductor-core`: rstest/proptest/insta; `conductor-cli`: assert_cmd/assert_fs/predicates): **rstest 0.26.1 · proptest 1.11.0 · insta 1.48.0 · assert_cmd 2.2.2 · assert_fs 1.1.4 · predicates 3.1.4**. External cargo subcommands exercised (not crates): cargo-nextest 0.9.133, cargo-llvm-cov 0.8.5. Toolchain component added: `llvm-tools-preview` (rust-toolchain.toml — cargo-llvm-cov requirement). `Cargo.lock` regenerated (143 deps); `cargo audit` + `cargo deny` green.
  - **NOTE vs test-plan §4:** the spec listed slightly different pins (cargo-nextest 0.9.137, proptest 1.9.0, cargo-llvm-cov 0.8.7) and did not name `predicates`; actual resolved/installed versions differ as above — a candidate routine reconciliation for the tests detector.
- **Schema / config:** `.config/nextest.toml` — `[profile.ci]` (`retries = 0`, `failure-output = "immediate-final"`), `[profile.ci.junit] path = "junit.xml"`, `[profile.default] retries = 0`. No DB migrations, no `CONDUCTOR_*` config keys, no violation schemas.
- **Coverage of new surfaces:** no new PRODUCT surface / hot-path op / UI element — test tooling only.
  - `conductor-cli/tests/cli_smoke.rs` fixture → validation n/a · instrumentation n/a · PII redacted✓ (synthetic only: `assert_fs::TempDir` + a synthetic `runs/424242.jsonl` path; no real secrets/PII/Pulse-corpus) · tests = the exemplar itself · a11y n/a · tokens n/a
  - `.config/nextest.toml` `ci` profile → determinism invariant `retries = 0` ✓ (zero-flakiness; test-plan §2/§10; arch §Async Runtime Flavor)

## Deviations from intent
- **Exemplars placed in integration `tests/`, not `conductor-core/src/*` `#[cfg(test)]` blocks** (plan "Files to modify" listed `src/*`). Justification: testing.md mandates testing the *public* API only — `tests/` enforces it and leaves the 36 src unit tests byte-for-byte untouched (a plan goal); the cli exemplar must be an integration test regardless (assert_cmd drives the built bin). A cleaner realization of the same intent.
- **insta exemplar uses an inline snapshot (`@"[PASS] [FAIL] [HOLD]"`), not a committed `.snap` under `src/snapshots/`** (plan/research anticipated a `.snap`). Justification: a deterministic single-line inline snapshot is green on first run with no `INSTA_UPDATE` seeding and nothing extra to commit — strictly more robust for an exemplar. File-snapshots (`tests/snapshots/`) remain for later epochs' real stream-shape goldens.
- **insta `json` feature not enabled** (plan step 1 mentioned it). Justification: the string `assert_snapshot!` needs no json feature — keeps the dep lean.
- **`deny.toml` not modified** (plan made it conditional). `cargo deny` reported `licenses ok` — the 6 crates introduced no license outside the MIT/Apache-2.0/Unicode-3.0/BSL-1.0 allowlist; the conditional resolved to a no-op.
- **No `proptest-regressions/`** — idempotence held for all generated inputs; proptest persisted no counterexample (expected).
- **`predicates` dev-dep added** (plan flagged it optional). Justification: used for the assert_fs `predicate::path::exists()` assertion — the canonical assert_fs pattern later CLI E2E reuses.

## Decisions & corrections
- Plan approved as-is (user "yes" at /andromeda-phase P5); no corrections during implement.
- Decision: exemplars exercise **real** `conductor-core` public fns (Verdict/ReportState `status_prefix`, `redact_value` idempotence) + the `conductor` bin — not throwaway smoke; light genuine coverage while proving the tools.
- Decision: **inline insta snapshot** chosen over a seeded `.snap` file for exemplar robustness (no `INSTA_UPDATE` dance) — reusable test-convention candidate.
- The carried-forward "`--profile ci` errors until `.config/nextest.toml` exists" gotcha is now **RESOLVED** — `cargo nextest run --workspace --profile ci` and `bash scripts/agent-run.sh run` both green.

## Outcome
- **All 8 plan acceptance criteria met.** Gates green, zero fix-loop iterations:
  - `cargo nextest run --workspace --profile ci` → **46/46** (36 existing + 10 exemplars); JUnit at `target/nextest/ci/junit.xml`
  - `cargo test --workspace --doc` → ok (0 doctests)
  - `cargo clippy --workspace --all-targets -- -D warnings` → clean
  - `cargo llvm-cov nextest --summary-only --fail-under-lines 60` → **91.97%** line coverage (passes)
  - `cargo audit` → clean (143 deps) · `cargo deny check advisories bans sources licenses` → advisories/bans/licenses/sources ok
- **Smoke:** `bash scripts/agent-run.sh run` → exit 0 — the harness drove `nextest --profile ci` + doctest + clippy through the real agent-driven entrypoint (headline deliverable proven end-to-end).
