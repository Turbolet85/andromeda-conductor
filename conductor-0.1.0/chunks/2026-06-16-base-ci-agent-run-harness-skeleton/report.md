# Report — 2026-06-16-base-ci-agent-run-harness-skeleton

**Chunk:** Base CI + agent-run harness skeleton — GitHub Actions build/nextest(ci)/clippy/doctest + cargo-audit/deny + llvm-cov + frontend npm-audit/vite-build gate; agent-run.{sh,ps1} 5-command skeleton
**Date:** 2026-06-16T18:08:51Z
**Commits:** not yet committed (wrap P7 commits this chunk); prior wrap = 6ea15fe (test-framework-fixtures-coverage-tooling)

## Changes (structured — detectors read this)
- **Files:** NEW `.github/workflows/ci.yml` (base CI workflow). No Rust/TS source changed; `scripts/agent-run.{sh,ps1}` were verified to conform and run headlessly, **NOT modified** (verify-don't-rewrite).
- **Symbols / APIs:** none — no Rust/TS code added. No IPC method / endpoint / export / port / socket. No new app env var (the workflow sets `$PSNativeCommandUseErrorActionPreference` as a PowerShell step preference, not an application env var).
- **Crates / modules:** none added / removed / changed.
- **Dependencies:** none added / bumped — no `Cargo.toml` or `package.json` change. CI consumes already-pinned tools (cargo-nextest / cargo-audit / cargo-deny / cargo-llvm-cov, npm) via CI actions (`actions/checkout@v4`, `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`, `actions/setup-node@v4` — CI infrastructure, not project dependencies). Node pinned to `22` in CI.
- **Schema / config:** NEW CI workflow config (`.github/workflows/ci.yml`, jobs `rust` + `frontend`, runner `windows-latest`); no app config keys, no migrations, no violation schemas.
- **Coverage of new surfaces:** none — the only new artifact is a CI workflow (build infrastructure), not a runtime surface / hot-path op / UI element. validation n/a · instrumentation n/a · PII n/a · a11y n/a · design-tokens n/a · tests = the workflow *runs* the existing suite (no new code paths introduced).

## Deviations from intent
- **Fail-fast robustness placed in `ci.yml`, not the harness script.** The dogfooded `agent-run.ps1 run` can mask a mid-sequence failure under Windows PowerShell 5.1 (native non-zero doesn't propagate). Rather than edit the setup-project-owned `agent-run.ps1` (verify-don't-rewrite; would be clobbered on setup regen), the workflow sets `$PSNativeCommandUseErrorActionPreference=$true` before invoking it. Justified: makes the gate sound on all pwsh versions, zero script change.
- **Added `permissions: contents: read` + `concurrency` (cancel-in-progress).** CI hygiene the plan didn't enumerate; within the new file's scope.
- **Pinned `node-version: '22'`** (plan left it open) — satisfies vite 8's ≥22.12 floor (local validation ran on node 24).
- Otherwise aligned with the plan + scope definition-of-done.

## Decisions & corrections
- **Runner OS = `windows-latest`** (user decision, phase P4 AskUserQuestion) — fidelity to arch §CI/CD "the dev OS target" + this tool's Windows path / bundled-SQLite specifics.
- **Test leg = dogfood agent-run** (user decision, phase P4) — CI invokes `scripts/agent-run.ps1 run`; honors arch §Cross-cutting "agent-run is the source of truth + release gate."
- **agent-run.ps1 native fail-fast** relies on pwsh-7.4+ defaults; CI makes it explicit. The durable fix (explicit `$LASTEXITCODE` checks in the script) belongs in **setup-project's generator**, not authored here — SURFACED for awareness.
- **Pre-existing uncommitted stray (NOT this chunk's work):** `rust-toolchain.toml` adds the `rust-analyzer` component (modified before this session began). Needs a commit decision at P7 — include in the chunk commit (CI would then install `rust-analyzer`, minor cost) or restore it. (Phase open-Q #3.)
- **Deferred to later epochs (build-sequencing, not drift):** coverage `--fail-under` threshold + flakiness budget + JUnit/coverage artifact upload + obs-conformance grep + a11y axe/contrast gate → **Epoch 10**; `boot`/preflight + scenario `run` bodies → **Epoch 5 / Epoch 8** (the `conductor` bin is a no-op stub today). Live-Pulse dynamic leg stays operator/local (`workflow_dispatch` reserved).

## Outcome
- **All 12 plan acceptance criteria met.** Base CI correctly does NOT wire `boot`/preflight (deferred Epoch 5) and measures coverage with no threshold (deferred Epoch 10).
- **Gates (run locally — the workflow's exact commands, all green):** `cargo build --workspace --locked` ✓ (Cargo.lock un-drifted) · dogfood `agent-run.ps1 run` ✓ (46/46 nextest `--profile ci` zero-retry, doctests, clippy `-D warnings`) · `cargo audit` ✓ (143 crates, 0 vulns) · `cargo deny check` ✓ (advisories/bans/licenses/sources ok) · `cargo llvm-cov nextest --workspace --profile ci --summary-only` ✓ (**91.97%** line) · frontend `npm ci` / `npm audit` / `npm run build` ✓ (0 vulns, vite built `dist/`).
- **Smoke:** `agent-run.sh run` ✓ — cross-shell parity (both the `.ps1` CI path and the `.sh` Git-Bash path green); usage dispatch reachable.
- **Caveat:** gates green **locally** (plan Test Commands = local-equivalent). The workflow's behavior in the GitHub Actions runtime (action resolution, `rustup show` toolchain install, `taiki-e/install-action`, setup-node cache) is validated by inspection, not yet by a real Actions run — the first Actions run happens after this commit is pushed.
