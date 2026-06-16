# Codebase Research — 2026-06-16-base-ci-agent-run-harness-skeleton

## Scope
- **Depth:** moderate · **Reads:** 8 · **Globs/Greps:** 4
- Key surprise: the agent-run scripts already exist and are **contract-complete** (setup-project Phase 4 generated them); `.github/workflows/` does **not** exist. So base CI is the only genuinely new artifact; the harness is a verify-don't-rewrite.

## Files inspected
- `scripts/agent-run.sh` (full) — already implements all 5 commands to the test-plan §3 contract: `boot`→`cargo run -p conductor-cli --bin conductor -- preflight --json` (30s timeout); `run`→`nextest run --profile ci` + `test --doc` + `clippy -- -D warnings` + optional `SCENARIO=`-gated scenario leg; `status`→`jq` on `runs/<id>.jsonl`; `cleanup`→`rm` + `sqlite3 DELETE` (idempotent); `logs`→`cat`. `set -euo pipefail`.
- `scripts/agent-run.ps1` (full) — faithful mirror, PowerShell idioms (`$ErrorActionPreference='Stop'`, `Set-StrictMode`, `ConvertFrom-Json`). Same 5 commands, same semantics.
- `.config/nextest.toml` (full) — `[profile.ci]` `retries=0` + `failure-output="immediate-final"`; `[profile.ci.junit] path="junit.xml"` → written to `target/nextest/ci/junit.xml`; `[profile.default] retries=0`. JUnit is emitted automatically whenever `--profile ci` runs.
- `rust-toolchain.toml` (full) — `channel="1.95.0"`, `components=["clippy","rustfmt","llvm-tools-preview","rust-analyzer"]`. **`rust-analyzer` is an uncommitted local addition** (committed set is the first three).
- `Cargo.toml` (full) — `resolver="3"`, 8 members, `publish=false`, `version="0.1.0"`, `rust-version="1.94.1"`, `tauri="2.10.3"` (unused until Epoch 9), dev-deps rstest/proptest/insta/assert_cmd/assert_fs/predicates in `[workspace.dependencies]`.
- `crates/conductor-cli/src/main.rs` (full, 5 lines) — **bare stub**: `fn main(){ conductor_core::init_observability("conductor", None); }`. No clap, **no `preflight`, no `run` subcommand.**
- `crates/conductor-tauri/ui/package.json` (full) — `scripts.build="tsc --noEmit && vite build"`, `typecheck`, `preview`, `dev`; `vite ^8.0.16`; `private:true`. `package-lock.json` present (npm ci works).
- `.andromeda/context/dependency-tree.md` — current graph: every seam → `conductor-core`; `conductor-cli` carries assert_cmd/assert_fs/predicates dev-deps; the `tauri` crate is NOT yet in the tree (`conductor-tauri` deps only `conductor-core`).
- `.claude/rules/verification-harness.md` — authoritative 5-command contract: "CI uses the rmcp stub (returns `ready:true`); the live-Pulse leg is `workflow_dispatch`/local only"; **"Do not add a 6th command without a test-plan amendment."**

## Patterns detected
- **Harness already conforms** (`scripts/agent-run.sh:23-73`, `.ps1:17-65`): the 5-command discipline + envelope + no-PID-file invariant are implemented. This chunk verifies and CI-wires them; it does NOT rewrite them.
- **JUnit is free under `--profile ci`** (`.config/nextest.toml:11-13`): base CI gets `target/nextest/ci/junit.xml` automatically — no extra flag. (Uploading/annotating it is Epoch 10.)
- **`conductor` bin is a no-op** (`main.rs:3-5`): `cargo run -- preflight --json` / `-- run <scenario>` are accepted-but-ignored (no clap to reject/route them). So `agent-run boot` "exits 0" vacuously today, and `run` works only because the scenario leg is `SCENARIO=`-gated. Real `preflight` = Epoch 5, real `run <scenario>` = Epochs 2–3.
- **Frontend gate shape** (`package.json:8`): `npm ci` → `npm audit` → `npm run build` (= `tsc --noEmit && vite build`), run inside `crates/conductor-tauri/ui/`.

## Conventions to follow
- 5-command harness is frozen — never add a 6th command absent a test-plan amendment (`verification-harness.md:24`).
- CI is build+test gating only; dynamic/live-Pulse proof is `workflow_dispatch`/local, never an unconditional CI gate (`verification-harness.md:18`; arch §CI/CD).
- Supply-chain gate must be green before merge: `cargo audit` + `cargo deny` + committed, un-drifted `Cargo.lock` (security-plan §Dependency Security).
- Toolchain installs from `rust-toolchain.toml` (pin 1.95.0 ≥ MSRV 1.94.1) — CI should honor it, not hardcode a version.
- Windows-safe artifacts (`run_id` hyphen-stamp, no colons) — relevant only if the runner is Windows (arch §Conventions / §Standard Contracts).

## New files to create
- `.github/workflows/ci.yml` — the base CI workflow. Gates (all green on the current tree): `cargo build --workspace` · `cargo nextest run --workspace --profile ci` (emits JUnit) · `cargo test --workspace --doc` · `cargo clippy --workspace --all-targets -- -D warnings` · `cargo audit` · `cargo deny check` · `cargo llvm-cov` (run/report, **no** threshold gate yet) · frontend `npm ci` + `npm audit` + `npm run build` in `crates/conductor-tauri/ui/`. Toolchain via `rust-toolchain.toml`; `Swatinem/rust-cache` recommended (bundled SQLite + tonic/prost cold builds are slow). Triggers: `push` + `pull_request` (+ optional `workflow_dispatch` reserved for the future live-Pulse leg).

## Files to modify
- `scripts/agent-run.{sh,ps1}` — **expected: none** (already contract-complete). Touch only if CI dogfooding surfaces a portability defect; any change is a fix, never a rewrite, and must not add a 6th command.

## Open questions
1. **CI runner OS** — `windows-latest` (fidelity to arch's "dev OS target" + Windows path/SQLite specifics) vs `ubuntu-latest` (faster/cheaper, the Rust-CI default; determinism is logic-not-OS) vs a matrix of both. → resolve at P4 (AskUserQuestion).
2. **CI test/lint leg: dogfood vs granular** — invoke `scripts/agent-run.{sh,ps1} run` so CI and local share one source-of-truth gate, vs call `cargo nextest`/`test --doc`/`clippy` as separate steps for per-tool annotations. Recommend: dogfood `run` for test+doctest+clippy; build/audit/deny/coverage/frontend as sibling steps. → confirm at P4.
3. **Uncommitted `rust-analyzer` toolchain component** — CI installs per `rust-toolchain.toml`, so it would pull `rust-analyzer` (minor install-time cost, no functional need in CI). Keep (dev convenience) or drop? Pre-existing local edit, arguably outside this chunk — flag at P5.
