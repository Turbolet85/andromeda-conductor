# Scope — Base CI + agent-run harness skeleton

**Marker:** `2026-06-16-base-ci-agent-run-harness-skeleton`
**Version:** conductor-0.1.0
**Epoch:** 1 — Foundation (9th and final chunk; closes Epoch 1)
**Working entry:** Base CI + agent-run harness skeleton — GitHub Actions build/nextest/clippy, agent-run.{sh,ps1} stub

## What it builds
Two foundational surfaces that, for the first time, make the headless release-gate path real end-to-end and
move it into CI — wiring the Epoch-1 toolchain (workspace, nextest `ci` profile, audit/deny, llvm-cov,
frontend bundle) that earlier chunks stood up:

1. **Base GitHub Actions CI** (`.github/workflows/`) — build + test + lint + supply-chain gate on the dev-OS
   target. Wires the tools already pinned in Epoch 1:
   - `cargo build --workspace`
   - `cargo nextest run --workspace --profile ci` (the zero-retry CI profile from the prior chunk; emits JUnit)
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --doc` (the doctests nextest skips)
   - supply-chain: `cargo audit` (RustSec) + `cargo deny` (`deny.toml`) — green locally, now enforced in CI
   - coverage run via `cargo-llvm-cov` (`llvm-tools-preview`)
   - frontend gate for `crates/conductor-tauri/ui` (npm, NOT a cargo member): `npm audit` (0 vulns) +
     `vite build` — parallel to the cargo gate
   - toolchain pinned via `rust-toolchain.toml`; relies on a committed, un-drifted `Cargo.lock`

2. **agent-run harness (verify, don't rewrite)** (`scripts/agent-run.sh` + `scripts/agent-run.ps1`) — the
   **5-command discipline** (boot / run / status / cleanup / logs) was already generated to the test-plan §3
   contract by setup-project Phase 4 and conforms (confirmed in research — the scripts are NOT bare stubs). This
   chunk **verifies** they run headlessly under CI (the `run` leg exercises nextest+doctest+clippy; the scenario
   leg + `boot`/`preflight` stay inert until `conductor-cli` gains those subcommands in Epochs 2–6) — it does NOT
   rewrite them and adds no 6th command. CI dogfoods `agent-run` as the test+lint gate.

## Boundaries (what this chunk does NOT do)
- **NOT the advanced CI quality gates** — coverage-% thresholds, flakiness budget, JUnit/llvm-cov artifact
  upload, the obs-conformance gate, and the a11y gate are **Epoch 10** (CI quality-gate config · Obs CI
  conformance gate · A11y CI gate). This chunk wires the *runs*, not the *thresholds/uploads/conformance*.
- **NOT the full 5-command harness behavior** — `boot`=MCP-preflight, `run`=scenario timeline,
  `status`=runs.db/JSONL read, `cleanup`=idempotent teardown, `logs`=journal tail become live in **Epoch 8**
  (5-command agent-run harness), once `conductor-cli` gains the `preflight`/`run` subcommands (Epochs 2–6). The
  scripts already encode the contract, but the bin is a no-op stub today, so `boot` + the scenario leg are inert
  and base CI does not invoke them.
- **NOT live-Pulse dynamic proof** — error-baseline-spike / fingerprint-storm / restart-suppression require a
  live Pulse + MCP and remain an explicit local operator gate, **never** a CI gate (CI has no Pulse).
- No new crates, no scenario config, no production Rust logic, no release/bundle step.

## Surfaces / contracts touched
- **New:** `.github/workflows/*.yml` (the base CI workflow).
- **Verified (not rewritten):** `scripts/agent-run.sh`, `scripts/agent-run.ps1` — already conform to the
  5-command contract (setup-project Phase 4); CI dogfoods the `run` leg.
- **Consumed (read-only):** `Cargo.toml` workspace · `.config/nextest.toml` (`[profile.ci]`) ·
  `rust-toolchain.toml` · `deny.toml` · `crates/conductor-tauri/ui/package-lock.json`.
- **Invariants honored:** zero nextest retries (determinism — no retries mask flakes); CI is build+test
  gating only (dynamic proof is local/operator); never merge/`--release` without cargo-audit + cargo-deny
  green and an un-drifted `Cargo.lock`; agent-run is the headless source of truth (GUI is a thin shell).

## Definition of done (intent anchor for validation-1)
- A GitHub Actions workflow on `windows-latest` that runs build + nextest(`ci`) + clippy(`-D warnings`) +
  doc-tests + cargo-audit + cargo-deny + cargo-llvm-cov + the frontend `npm audit`/`vite build` gate — green on
  the current tree, with the test+lint leg dogfooding `scripts/agent-run.ps1 run`.
- `agent-run.sh` / `agent-run.ps1` confirmed to expose the 5 commands and run headlessly (`run` exit 0) on both
  shells — verified, not rewritten; no 6th command added.
