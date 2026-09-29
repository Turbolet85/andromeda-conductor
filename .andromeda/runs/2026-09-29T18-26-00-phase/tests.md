# tests extract

## Relevance
partial — the chunk adds no Rust or TS logic (two license texts plus manifest metadata), so no new unit, integration or E2E surface applies; what binds is the supply-chain gate (`cargo deny check` licenses arm), lockfile hygiene, the frontend build gate over the edited `package.json`, and the no-regression run of the existing suite.

## Constraints
- The supply-chain stage runs `cargo audit` + `cargo deny check` across all four arms (advisories/licenses/bans/sources), and either being non-green is a build failure — so a manifest-only chunk still owes both gates green over the edited manifests, per test-plan §9 CI Integration (Pipeline structure: Supply-chain audit row; Build failure conditions) and §10 Quality Gates (Build failure conditions: Supply-chain).
- `Cargo.lock` must stay committed and un-drifted; drift is itself a build-failure condition, per test-plan §9 CI Integration (Build failure conditions) and §10 Quality Gates. Whether a `license` field edit moves `Cargo.lock` at all is research's question (the plan states the rule, not the lock's content model).
- The frontend package is build-gated rather than unit-tested (`tsc --noEmit` + `vite build` + `npm audit` + a `vite preview` render smoke); an edit to `crates/conductor-tauri/ui/package.json` falls under that gate, per test-plan §4 Unit Test Strategy (the `conductor-tauri/ui (frontend SPA)` bullet).
- Coverage must hold at `--fail-under-lines 60` and the threshold is never lowered; a zero-code chunk is expected to leave the measured figure unchanged, per test-plan §10 Quality Gates (Coverage thresholds) and §11 Test Anti-Patterns → Quality.
- Every gate result must be machine-parseable and read from the bare command's exit (nextest `NextestExitCode`, `jq -e`-style assertions), never a human read, per test-plan §2 Test Strategy (Agent-runnable invariants).
- Repository-hygiene gates (the secret-scan gate over the cached + untracked workspace listing) are build-failure conditions, so the two new root files enter that gate's scope, per test-plan §9 CI Integration (Build failure conditions: repository-hygiene) and §10 Quality Gates (Repository hygiene).
- The CI stage table is the complete inventory of gates CI enforces; the scoped mutation instrument is operator-local and per-chunk over crates the chunk touched — a chunk whose delta is manifest metadata only has no mutable code to audit, per test-plan §9 CI Integration (Scoped mutation audit) and §4 Unit Test Strategy.

## Patterns to follow
- Run the full supply-chain pair as the shipped gate runs it (`cargo audit` + `cargo deny check` over all arms), not the licenses arm alone, per test-plan §9 CI Integration (Supply-chain audit row) and §11 Test Anti-Patterns → CI (stack-specific bullet).
- Run the workspace suite through the harness form (`agent-run.{sh,ps1} run --unit` → `cargo nextest run --workspace --profile ci`) plus clippy `-D warnings` and `cargo fmt --check` as the no-regression proof, per test-plan §9 CI Integration (Pipeline structure: Lint and Unit rows).
- Prove the frontend manifest edit through its build gate (`tsc --noEmit` + `vite build` + `npm audit`), per test-plan §4 Unit Test Strategy (frontend SPA bullet).
- Assert metadata facts on machine-parseable output (e.g. a JSON manifest dump checked with `jq -e`) rather than by eyeballing files, per test-plan §2 Test Strategy (Agent-runnable invariants).

## Anti-patterns to avoid
- NEVER merge or `cargo build --release` without `cargo audit` + `cargo deny check` green and a committed un-drifted `Cargo.lock`, per test-plan §11 Test Anti-Patterns → CI.
- NEVER skip quality gates "just this once" on the grounds that the change is metadata-only — audit-green + coverage + clippy are binding, per test-plan §11 Test Anti-Patterns → Quality.
- NEVER include a manual smoke step ("developer verifies the license files") — every check is an agent-run command with a readable exit, per test-plan §11 Test Anti-Patterns → Universal (agent-driven specific).

## Contract bindings
- tests ↔ security: the Supply-chain audit CI stage (test-plan §9) is the executable form of security-plan §Dependency Security's `cargo deny` license allowlist; which of the P4 fork's arms holds (own crates still skipped via `deny.toml`'s `private` ignore, or now actually license-checked) changes what the licenses arm proves, and the test gate is the same command either way.
- tests ↔ security: `package-lock.json` must stay committed and un-drifted beside the frontend build gate's `npm audit` (test-plan §4 frontend SPA bullet), binding to security-plan §Dependency Security's frontend lockfile rule.

## Acceptance criteria contributions
- `cargo deny check` exits 0 over all arms (advisories/licenses/bans/sources) and `cargo audit` exits 0, each exit read from the bare command before any pipe (per test-plan §9 CI Integration).
- `cargo nextest run --workspace --profile ci`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` all exit 0 on the chunk's tree, with no nextest `retries` introduced (per test-plan §9 CI Integration; §10 Quality Gates, Zero-flakiness budget).
- The frontend build gate — `tsc --noEmit` + `vite build` + `npm audit --omit=dev` — exits 0 after the `package.json` edit, with `package-lock.json` regenerated by npm and committed (per test-plan §4 Unit Test Strategy, frontend SPA bullet).
- A machine-parseable probe asserts all nine member packages report the license expression `MIT OR Apache-2.0` (e.g. a JSON manifest dump checked with `jq -e`, zero exceptions) (per test-plan §2 Test Strategy, Agent-runnable invariants).
