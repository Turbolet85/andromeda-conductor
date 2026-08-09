# tests extract

## Relevance
Relevant — the chunk adds no test code, but its definition of done *is* a test-plan quality gate (supply-chain green + un-drifted lock) re-proven through the 5-command harness, and the crossbeam-epoch bump lands in the dev/fixture tree (`assert_fs`) that my domain owns.

## Constraints
- The supply-chain gate is a **binding build-failure condition**, not advisory: `cargo audit --deny warnings` *and* `cargo deny check` (advisories/licenses/bans/sources) must be green with a committed, un-drifted `Cargo.lock` — per test-plan §9 Build failure conditions + §10 Build failure conditions. Both tools, not one: an `unsound` advisory (RUSTSEC-2026-0190/anyhow) is exit-0 under audit and denied by deny, exactly the dual-tool case §9 encodes.
- External-CLI tool versions (cargo-audit, cargo-deny, cargo-nextest, cargo-llvm-cov) are **reference floors, not exact pins** — any install that runs the gate green satisfies it; crate dev-deps are caret-resolved with `Cargo.lock` authoritative (per test-plan §4 Tool-version policy). Do not "fix" a version mismatch against the prose.
- Re-proof runs through the harness contract, not ad-hoc commands: `run` = `cargo nextest run --workspace --profile ci` → `cargo test --workspace --doc` → `cargo clippy --workspace --all-targets -- -D warnings`, with doctest and clippy as **separate stages, never folded into `--unit`** (per test-plan §3 `run` / CI stage selectors + §9 pipeline table).
- Zero-flakiness budget: if any test flakes after the bumps it is quarantined and root-caused, **never retried** — do not set nextest `retries` > 0 (per test-plan §10 Zero-flakiness budget + §11 CI). A post-bump flake is a real determinism break, not noise.
- The version floors asserted by CI must not regress: toolchain ≥ 1.94.1 (`rust-toolchain.toml`), `tauri` ≥ 2.10.3, committed lock (per test-plan §9 Matrix builds + §10). A lock bump that pushes any of these down fails the gate independently of the four advisories.
- Fixture-layer bumps are in-domain risk: `assert_fs::TempDir` + `CONDUCTOR_RUNS_DIR` per-test isolation and in-memory `runs.db` are the load-bearing mechanisms (per test-plan §3 Test data bootstrap, §5 Boundary types, §7 Seed strategies) — crossbeam-epoch reaches them via `assert_fs` → globwalk → ignore, so `conductor-cli` / `conductor-run` / `conductor-report` / `conductor-tauri` dev-test paths are the exercise surface.
- Coverage threshold stays fixed at `--fail-under-lines 60`; it may not be lowered to make a stage pass (per test-plan §10 + §11 Quality).

## Patterns to follow
- Harness-first verification: drive the re-proof via `scripts/agent-run.{sh,ps1} run` (aggregate) and the `--unit` / `--integration` / `--e2e` stage selectors rather than bespoke cargo invocations, so CI and local agree (per test-plan §3 5-command implementation + §9).
- Machine-parseable signals only: nextest `ci`-profile JUnit XML + stable `NextestExitCode` (100 test-fail / 101 build-fail / 4 NO_TESTS_RUN) as the pass/fail read (per test-plan §2 Agent-runnable invariants + §9 Test report format). Exit-4 (NO_TESTS_RUN) is a failure signal, not a pass — relevant if a bump breaks target compilation.
- Fast-iteration narrowing on compile fallout: `cargo nextest run -p conductor-<seam>` for the single affected seam (quick-xml 0.41 lands in `tauri-build`'s build script, so `conductor-tauri` first) before re-running the workspace (per test-plan §3 Test selection + §4 Conventions).
- Golden artifacts as the "nothing changed" tripwire: insta snapshots run CI/assert (fail-don't-write) over the Run-report envelope + JSONL journal, with `run_id`/timestamp redaction (per test-plan §7 Golden artifacts). A lock-only change must leave every snapshot untouched — any snapshot diff means the bump reached behaviour.
- CI stage already exists — the Supply-chain audit stage with its RustSec advisory-db cache is in the §9 pipeline table; restore it green rather than adding a step, matching the scope's "no new step expected."

## Anti-patterns to avoid
- NEVER merge or `cargo build --release` without `cargo audit --deny warnings` + `cargo deny check` green and a committed un-drifted `Cargo.lock` (test-plan §11 CI, stack-specific) — this ban is the reason the chunk exists; it also forbids "green enough" partial proof.
- NEVER skip a quality gate "just this once", nor lower `--fail-under-lines 60` or ignore a flake to get the pipeline through (test-plan §11 Quality + §10).
- NEVER drop the `~/.cargo` + `target/` caching keyed on `Cargo.lock` (test-plan §11 CI) — the lock change invalidates the cache once by design; removing the cache to "force freshness" is the banned shortcut.

## Contract bindings
- **tests ↔ security (dependency security):** test-plan §4 Tool-version policy explicitly derives its floor logic from the security plan §Dependency Security precedent for cargo-audit/cargo-deny; test-plan §9/§10/§11 own *enforcement in CI*, while advisory triage, `deny.toml` ignore-list membership, and the RUSTSEC-2024-0429/glib open item are security's call. My domain contributes only the pass/fail gate semantics.
- **tests ↔ obs (log format / envelope):** test-plan §3 Log format is the source of truth for the JSONL journal + Run-report envelope from which obs §3 derives; a lock-only bump must produce zero change here — the §7 insta goldens and the §3 negative test (no host paths / struct names) are the binding assertion that the boundary held.
- **tests ↔ arch (toolchain/tauri floors):** §9 Matrix builds pins toolchain ≥ 1.94.1 and `tauri` ≥ 2.10.3 as CI-asserted floors; the scope's "no Tauri bump" boundary must not fall below them.

## Acceptance criteria contributions
- (tests) `cargo nextest run --workspace --profile ci` passes at the 428-test baseline with zero retries, zero new skips, and no `NO_TESTS_RUN` (exit 4) target — plus `cargo test --workspace --doc` and `cargo clippy --workspace --all-targets -- -D warnings` green as separate stages (test-plan §3 `run`, §9, §10).
- (tests) `cargo audit --deny warnings` exit 0 **and** `cargo deny check advisories bans sources licenses` exit 0, with `Cargo.lock` committed and `git diff --exit-code Cargo.lock` clean after the run (test-plan §9 + §10 Build failure conditions).
- (tests) Fixture/dev-tree bump is exercised, not assumed: the `assert_fs::TempDir` + `CONDUCTOR_RUNS_DIR` isolation paths in `conductor-cli` (assert_cmd subprocess), `conductor-run`, `conductor-report`, and `conductor-tauri` all pass post-bump (test-plan §3 Test data bootstrap, §5, §7).
- (tests) No insta snapshot rewrite and no envelope/journal shape change — goldens pass in CI fail-don't-write mode, confirming the change is lock-only at the artifact boundary (test-plan §7 Golden artifacts, §3 Log format).

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** (§4 Unit Test Strategy) — reframed external-CLI tool versions as *floors, not pins*, explicitly citing "the same floor logic the security-plan applies to cargo-audit / cargo-deny." Directly governs this chunk: whatever cargo-audit / cargo-deny / cargo-nextest versions are installed locally satisfy the gate if they run it green; a version mismatch against the plan's named numbers is not a finding, and D-tests-framework should not re-fire on it.
- **2026-08-08-sut-capability-manifest** (§1/§4/§6/§7) — moved the workspace nextest count 420 → 428 via the capability-manifest loader/validator tests. That 428 is the baseline this chunk's scope re-proves against; any deviation is bump fallout, not drift from an older number.
- **2026-06-15-design-token-typography-bundle** (§4 Unit Test Strategy) — recorded that the `conductor-tauri/ui` frontend is **build-gated** (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` smoke) with no Rust/nextest unit tests. Supports the scope's exclusion of the `npm audit --omit=dev` frontend gate: it is a separate, separately-gated tree, and the Rust supply-chain gate does not cover or depend on it.
