# tests extract

## Relevance
relevant: the chunk removes a whole test surface (the desktop-webview E2E leg, the Tauri mock-runtime integration tier, the `a11y` CI job, the `--e2e` and a11y run arms). It must also keep every remaining gate in the plan green over the smaller tree.

## Constraints
- The bundled `run` default must stay green in both shells with the crate gone. That default is workspace nextest (`ci` profile), then doctest, then workspace clippy `-D warnings`, then the two feature-gated lint lines (`-p conductor-verify --features stub-server`, `-p conductor-run --features live-pulse`). It stops at the first non-zero cargo line (per test-plan §3 `run`). Whether removing the crate turns any remaining item into `-D warnings` dead code is research's question (scope B1).
- The harness must keep exactly five commands, with identical `.sh`/`.ps1` semantics. The `run` selectors stay mutually exclusive, so removing `--e2e` must leave `--unit` / `--integration` / `--live` / `--live real-model` and the unknown-selector `usage:` exit 2 the same in both shells (per test-plan §3 `run`). §3 `run` defines `--integration` to include the `tauri::test` mock-runtime leg, and bare `--live` composes the driven a11y arm (`ensure_frontend` + release build + `npm run a11y:driven`) into the order `live_leg_order` owns. Research must read whether the shells encode either. The order comes from `live_leg_order`, never from a count.
- The suite must stay green under both runners. `cargo test -p <crate>` (full, non-doc) is a standing runner-portability gate beside nextest (per test-plan §4 Framework). A runner-dependent result is a determinism defect, never a reason to pin a runner, and nextest `retries` stays 0 (per test-plan §10 Zero-flakiness budget).
- The workspace line-coverage gate `--fail-under-lines 60` (cargo-llvm-cov over the whole workspace, exclusion `[\\/]tests[\\/]`) must still hold after the crate's lines leave the denominator (per test-plan §10 Coverage thresholds). Whether it holds is a measurement, not an argument.
- The supply-chain gate is bare `cargo audit` plus `cargo deny check` (advisories/licenses/bans/sources), both green over the new committed, un-drifted `Cargo.lock` (per test-plan §9 Supply-chain audit form). `cargo audit --deny warnings` is never the gate (per test-plan §9 Supply-chain audit form).
- Each committed fixture keeps its meaning-asserting round-trip test (per test-plan §7 Self-bootstrapping requirement). `crates/conductor-tauri/tests/fixtures/scenarios/` and its round-trip test (`commands::tests::the_committed_scenarios_fixture_stays_loadable`) leave with the crate. `crates/conductor-run/tests/fixtures/` with `lamps_fixture.rs` / `envelope_fixture.rs` are B1's and stay, with their round-trip tests green. The `CONDUCTOR_E2E_SEED_DIR` seeder's mandated coverage is its unset no-op path running in every ordinary suite run (per test-plan §1 Coverage triggers, Vector 1 third class).
- Live-Pulse legs, the `--live` suite included, are operator/local only and never a CI gate (per test-plan §9 Live-Pulse scenarios). Editing the `--live` arm in this chunk does not license running it: ports 4317/4318 are shared with Pulse's builder (scope B5).

## Patterns to follow
- Gate on the printed verdict and on an exit read from the BARE command, never through a pipe, the way `--e2e` asserted wdio's printed `Spec Files:` line (per test-plan §3 `run`). Apply this when proving that the remaining shell arms and gates are green.
- Cross-surface parity stays in `crates/conductor-cli/tests/cross_surface_parity.rs`. That package has no `tauri` dependency, and the leg drives `conductor_run::{preflight, drive_run}` directly with no `tauri::*` item (per test-plan §5 Cross-module patterns covered). The crate's removal must leave it untouched and green.
- The remaining static CI gates keep their step shape: their own named `rust`-job step, a presence guard that emits `::error::` and exits 1, no `continue-on-error`, no `if:` (per test-plan §6 Coverage-matrix completeness gate, repository-hygiene legs). Removing the `frontend` and `a11y` jobs must leave the `Coverage-completeness gate`, `Scenario-assertion audit gate`, `Secret-scan gate`, `Workflow env-context gate` and the `GITHUB_ENV context probe` pair intact.
- The coverage exclusion is one generic path regex, not a list (per test-plan §10 Coverage thresholds). The crate's `tests/` tree needs no exclusion edit when it leaves.
- Identify CI jobs by name (`rust`, `frontend`, `a11y`), never by `ci.yml` line coordinates (per test-plan §9 Matrix builds).

## Anti-patterns to avoid
- Never lower the coverage threshold to pass the build, and never skip a binding gate "just this once" (per test-plan §11 Quality).
- Never add nextest `retries`, and never merge or `cargo build --release` without `cargo audit` and `cargo deny check` green over a committed lock (per test-plan §11 CI).
- Never use a manual smoke step to stand in for proof (per test-plan §11 Universal). "No crate's graph holds the window toolkit" is shown by a `cargo tree` / `Cargo.lock` reading, never by inspection or argument.

## Contract bindings
- tests §3 `run` ↔ obs-plan §3: the `--e2e` leg wrote a separate `runs/a11y/<run_id>.jsonl` violation record, made of the eleven envelope fields plus the obs §9 resource tags. The `a11y` job's reused `journal_conformance` gate read that record over `runs/a11y` (per test-plan §9 CI Integration stage table, E2E row). Removing the leg removes that artifact. The `status` disk read and the envelope/JSONL shapes must stay untouched.
- tests §9 ↔ a11y §Bootstrap: the a11y CI gate lived in the same workflow (the `a11y` job), and it leaves with this chunk. The a11y masters' amendments belong to the wrap (scope B6).
- tests §9 ↔ security §Dependency Security: `cargo deny check` runs in the same workflow. The Tauri-tree `deny.toml` exceptions are decided against the post-removal graph, with both tools green.
- tests §1 Coverage triggers (Vector 1 harness-only handle classes `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, `CONDUCTOR_E2E_SEED_DIR`, `CONDUCTOR_A11Y_STRICT`; Vector 3 Tauri capabilities / `tauri` ≥ 2.10.3) ↔ security §Input Validation / §Tauri GUI and arch §Occupied Resources: each class's mandated test leaves only with its reader.
- tests §6 repository-hygiene legs ↔ security §Secrets: the secret-scan allowlist is an exact set graded in both directions, so an entry whose file this chunk deletes fails as "matching nothing". The workflow env gate checks every `env.X` read in `ci.yml` against that file's own declarations.

## Acceptance criteria contributions
- `bash scripts/agent-run.sh run` (bundled default) exits 0 with no window crate in the workspace, and `cargo test --workspace` (non-doc) is also green (per test-plan §3 `run`; test-plan §4 Framework).
- Bare `cargo audit` and `cargo deny check` each exit 0 over the new committed `Cargo.lock`, each exit read from the bare command (per test-plan §9 Supply-chain audit form).
- `cargo llvm-cov nextest` over the workspace with the shipped exclusion regex meets `--fail-under-lines 60` (per test-plan §10 Coverage thresholds).
- `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci` and `cargo nextest run -p conductor-core --test workflow_env_gate --profile ci` pass after the file and `ci.yml` removals, and CI reads green on the push that carries them (per test-plan §6 Coverage-matrix completeness gate, repository-hygiene legs).
