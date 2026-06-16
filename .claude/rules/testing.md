---
paths:
  - "crates/**/tests/**"
  - "crates/**/src/**/*.rs"
  - "scenarios/**"
  - "**/snapshots/**"
  - "**/proptest-regressions/**"
---

# Testing Rules

Path-scoped rules for Rust test code (in-crate `#[cfg(test)]` modules + crate-local `tests/` + scenario fixtures). Loaded when working with matching files.

**Authoritative source:** `.andromeda/test-plan.md` — §3 Test Harness Contract, §4–7 strategy, §11 Anti-Patterns.

## Frameworks
- **Runner:** cargo-nextest (`cargo nextest run`; stable `NextestExitCode` 100/101/4 + `ci`-profile JUnit XML) + `cargo test --doc` for doctests nextest skips.
- **Fixtures:** rstest (`#[fixture]`/`#[rstest]`/`#[case]`/`#[once]`); **property:** proptest (`proptest-regressions/` persisted); **golden:** insta (CI/assert mode — fail, don't write); **CLI E2E:** assert_cmd + assert_fs; **coverage:** cargo-llvm-cov.

## Patterns
- Test the public seam API + observable behavior (envelope/verdict/state output) — never private fns or internal struct fields.
- Assert on the Run-report envelope contract, not fixture-internal values.
- Per-seam grouping (`cargo nextest run -p conductor-<seam>`); `#[rstest]` `#[case]` rows for the P-ID + garde valid/invalid matrices.
- E2E selectors: role / data-testid / text + brand anchors (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`, `role="alertdialog"`, `aria-live`, "HOLD — operator pause", P-ID tokens). Never xpath or hashed-CSS classes; never `sleep(N)` — wait for an explicit signal (envelope `state`, JSONL line, exit code).

## Determinism (the quality bar)
- Same scenario+seed ⇒ same stream shape. Drive `conductor-timeline` via `#[tokio::test(flavor="current_thread", start_paused=true)]` + `tokio::time::advance`; assert scheduled ordering/shape, never real wall-clock duration.
- Seed all randomness (`conductor-timeline` seed + proptest strategies). Journal/report stamps come from `std::time::SystemTime`/`Instant` — NEVER tokio's virtual clock (a golden test catches a paused-clock leak).

## Test data & isolation
- Self-bootstrapping via the seeded `conductor-timeline` generator — no developer-seeded DB; scenario config is committed serde+garde fixtures one-per-P-ID under `scenarios/`.
- DB at integration level: rusqlite `open_in_memory()` (per-test) or `assert_fs::TempDir` file DB (cross-process) — never a hand-rolled SQL fake/mock. Fresh DB per test (nextest per-process model); never share mutable state across parallel tests.
- Mock the live Pulse with an rmcp stub (in-process duplex + `TokioChildProcess` for the spawn/`.env()` path) — never fake Pulse's *reaction* as a CI verdict; the live leg is local/operator-gated only.

## Running
- Full: `scripts/agent-run.sh run` (or `cargo nextest run --workspace --profile ci`). Single seam: `cargo nextest run -p conductor-<seam>`. One scenario: `conductor run <P-ID> --seed <s>`. Coverage: `cargo llvm-cov nextest --lcov --fail-under-lines 60`.

## Quality gates
- Zero-flakiness: NEVER set nextest `retries` > 0 — a flake means a real determinism break; quarantine + fix. Never lower the coverage threshold to pass; never run the live-Pulse leg as a CI gate.
- `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported envelope states, NOT non-zero exits — only a hard `Fail` exits non-zero.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-06-16: When testing a seeded/deterministic component, assert BOTH directions — same seed ⇒ identical output (reproducibility) AND different seeds ⇒ different output (the seed materially drives the result). Reproducibility alone also passes if the seed is computed-but-never-applied (or applied identically regardless of value); the different-seeds-diverge test is what proves the seed is wired through to the output. Use ≥2 fixed seeds known to diverge over multiple draws — not a single-draw coincidence. (seeded-phase-scheduler chunk)
