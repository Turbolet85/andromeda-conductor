# Scope — CI quality-gate config

**Marker:** `2026-06-27-ci-quality-gate-config`
**Version:** conductor-0.1.0 · **Epoch:** 10 (Polish & ship) — pulled forward ahead of the display-gated Desktop a11y verification chunk (Windows-only session).
**Working entry:** _CI quality-gate config — coverage threshold + flakiness budget + nextest JUnit / llvm-cov artifact upload._

## Intent (one line)
Promote the existing GitHub Actions **build + test** pipeline into a **quality-gated** one: enforce a line-coverage floor, a zero-flakiness budget, and emit/upload machine-parseable test + coverage artifacts — all on the dev-OS CI target, with **no live Pulse / Linux+xvfb / webview dependency**.

## What it builds
Four CI-surface gates layered onto the current `cargo build` / `nextest (ci)` / `clippy` / `audit` / `deny` / doctest workflow (the `base-ci-agent-run-harness-skeleton` scaffold):

1. **Coverage threshold gate** — run the suite under `cargo-llvm-cov` (already in the stack; needs the `llvm-tools-preview` component) and **fail CI** when line coverage drops below a pinned floor. The threshold value + grain (workspace-wide vs per-crate) is a plan-time decision anchored in test-plan §10.
2. **Flakiness budget** — make the determinism invariant (`zero-flakiness — no nextest retries`) an enforced CI property, not just a local `.config/nextest.toml` setting: assert the `ci` profile stays zero-retry and surface any flake as a hard failure (the exact mechanism — retry=0 assertion vs a repeated-run flake-detection step — is a plan-time decision).
3. **nextest JUnit output** — configure the nextest `ci` profile to emit JUnit XML (`[profile.ci.junit]`) and wire CI to consume it as the machine-parseable test report.
4. **llvm-cov + JUnit artifact upload** — upload the coverage report (lcov/HTML) and the JUnit XML as GitHub Actions build artifacts (`actions/upload-artifact`), establishing the reusable upload pattern.

## Surfaces / contracts touched (to be confirmed in P3 research)
- `.github/workflows/*.yml` — the CI workflow job(s): add the llvm-cov run, the coverage-threshold gate step, the JUnit consumption, and the artifact-upload steps.
- `.config/nextest.toml` — extend the `ci` profile with JUnit emission (and confirm zero-retry is pinned + asserted).
- Possibly `scripts/agent-run.{sh,ps1}` and/or a CI invocation of `cargo llvm-cov` — only if the coverage run is shared between the local harness and CI; the harness `run` verb must stay green.
- **Authoritative spec:** `.andromeda/test-plan.md` §9 (CI integration) / §10 (quality gates); the determinism/zero-retry invariant in `architecture.md` §Cross-cutting Patterns + CLAUDE.md.

## Boundaries (explicitly NOT this chunk)
- **No engine/seam code change** — CI YAML + nextest config (+ optional harness wiring) only; zero change to any `conductor-*` library/bin behavior.
- **Not the live-Pulse dynamic proof** — error-baseline-spike / fingerprint-storm / restart-suppression E2E stays a local operator gate (Epoch-10 live chunks), never a CI gate (CI has no Pulse).
- **Not the Obs CI conformance gate** (`agent-latest.jsonl` upload + log-schema conformance + zero-unlogged-panics) — a **separate downstream chunk that builds on this scaffold**; this chunk establishes the artifact-upload + gate-step pattern it reuses, but does not implement obs conformance.
- **Not the A11y CI gate** (axe/contrast/keyboard → obs envelope) — separate, display-gated Epoch-10 chunk.
- **Not the coverage-matrix completeness gate** (60-P-ID zero-gap definition-of-done) — separate Epoch-10 chunk.
- **Not the release build / Tauri bundle** — the capstone, deliberately left last.

## Why it leads (reorder rationale)
Fully executable on the Windows dev host, and it is the CI quality-gate scaffold that the later Epoch-10 gates ("Obs CI conformance gate", "A11y CI gate + violation JSON") extend — so landing its gate-step + artifact-upload patterns first unblocks them. The display-gated Desktop a11y verification chunk + the live-Pulse Epoch-10 chunks stay deferred to a Linux+xvfb+live-Pulse environment.

## Acceptance shape (sharpened into criteria at P4)
- CI fails when line coverage is below the pinned floor; passes at/above it.
- The `ci` nextest profile emits JUnit XML and remains zero-retry; a flake surfaces as a hard failure.
- Coverage report + JUnit XML are uploaded as CI artifacts on every run.
- No change to local `agent-run.{sh,ps1} run` green status; `cargo audit` + `cargo deny` stay green; no engine/seam diff.
