# Scope — Test framework + fixtures + coverage tooling

**Marker:** `2026-06-16-test-framework-fixtures-coverage-tooling`
**Version:** conductor-0.1.0 · Epoch 1 (Foundation) · chunk 8 of 9
**Working entry:** Test framework + fixtures + coverage tooling — cargo-nextest, rstest, proptest, insta, assert_cmd/fs, cargo-llvm-cov

## Intent
Stand up the workspace-wide **test runner + fixture + coverage toolchain** so every later chunk inherits one
deterministic, machine-parseable test harness. This is pure Epoch-1 foundation: it wires the dev-dependencies,
the nextest profile config, and the coverage tool, and proves each is functional with one minimal exemplar
test apiece — it does **not** write the domain tests (timeline/emission/verify/report), which land with their
own epochs.

## What it builds
- **`.config/nextest.toml`** — the nextest profile config, including the **`ci` profile** that
  `cargo nextest run --workspace --profile ci` references. This file is currently absent, so `--profile ci`
  errors (the active forward gotcha from the prior session); creating it is the gating deliverable. The `ci`
  profile must be machine-parseable (JUnit output for the agent-run `status`/`run` commands) and
  **zero-retry** — the determinism / zero-flakiness invariant forbids nextest retries (retries that mask flakes
  would violate "same scenario+seed ⇒ same stream shape").
- **Dev-dependency wiring** (workspace `[workspace.dependencies]` + per-crate `[dev-dependencies]`):
  - `rstest` — fixtures + parametrized table cases.
  - `proptest` — property-based tests (the Epoch-2 determinism-replay harness builds on it).
  - `insta` — snapshot / golden tests (later: stream-shape + run-report-envelope goldens).
  - `assert_cmd` + `assert_fs` — CLI-binary + filesystem assertions for the `conductor-cli` / agent-run paths.
- **`cargo-llvm-cov` coverage tooling** — config/wiring + the documented invocation, so the Epoch-10 CI
  coverage-threshold gate has a tool to call.
- **Exemplar / smoke tests** — one minimal test per tool, proving the wiring compiles and runs green under
  `cargo nextest run --workspace` (default profile) and `--profile ci`.

## Boundaries (NOT in this chunk)
- **No GitHub Actions / CI wiring** — that is the *next* chunk ("Base CI + agent-run harness skeleton"); this
  chunk only makes the tools locally runnable and leaves the `--profile ci` gotcha resolved.
- **No domain tests** — determinism replay, OTLP emission goldens, MCP verdict logic, run-report envelopes all
  ship with Epochs 2–10; here only trivial exemplars exist.
- **No frontend test stack** (`crates/conductor-tauri/ui`, npm/Vitest) — deferred to Epoch 9 per the recorded
  test-plan §4 amendment.
- **Read-only on specs / `state.yaml`**; no production code beyond the minimum an exemplar test needs.

## Surfaces / contracts touched
- `.config/nextest.toml` (new) — the **`ci` profile name** is the contract that `--profile ci` + agent-run depend on.
- Workspace `Cargo.toml` `[workspace.dependencies]` + each crate's `[dev-dependencies]`.
- **Determinism invariant:** `ci` profile `retries = 0` (no flakiness masking; no nextest retries).
- **Test-plan §3 5-command harness:** the `run = nextest + scenarios` command + machine-parseable (JUnit) output.
- **Coverage:** `cargo-llvm-cov` output is consumed later by the Epoch-10 coverage-threshold CI gate.
- **Supply chain:** any new dev-deps must keep `cargo-audit` / `cargo-deny` green and `Cargo.lock`
  committed + un-drifted (the universal release-gate invariant).

## Definition of done (scope-level — the val-1 anchor)
- `cargo nextest run --workspace --profile ci` runs **green** (no longer errors on a missing `ci` profile).
- Each tool — `rstest`, `proptest`, `insta`, `assert_cmd` + `assert_fs` — has a green exemplar test.
- `cargo llvm-cov` produces a coverage report locally.
- `cargo-audit` + `cargo-deny` still green; `Cargo.lock` updated + committed; `cargo clippy -D warnings` clean.
