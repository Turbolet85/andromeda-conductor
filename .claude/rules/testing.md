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
- 2026-06-17: Property-testing an async SUT (e.g. `run_timeline`) cannot use `#[tokio::test]` — that macro cannot wrap a `proptest!` block. Build the runtime inside each case (`tokio::runtime::Builder::new_current_thread().enable_time().start_paused(true).build()`), pull results out via `rt.block_on(...)`, then assert with `prop_assert*!` synchronously OUTSIDE the async block (`prop_assert` returns early on failure — keep it out of the future, or `?`-propagate a `Result<(), TestCaseError>`). (determinism-replay-harness chunk)
- 2026-06-17: An unpinned (entropy-seeded) `proptest` search is consistent with the zero-retry determinism bar — "seed all randomness" governs the SUT's seed, not proptest's case-search RNG. The search cannot flake when the property is universally true (∀ input); a discovered counterexample is a real determinism break, persisted to a committed `proptest-regressions/` and replayed deterministically thereafter. (determinism-replay-harness chunk)
- 2026-06-17: OTLP-egress tests use a loopback tonic gRPC **server** stub — a `TraceService` impl over `tokio-stream::wrappers::TcpListenerStream` bound to an ephemeral `127.0.0.1:0`, driven under `#[tokio::test(flavor="current_thread")]` (the spawned server is polled cooperatively while the client awaits, so no sleep is needed). NEVER bind the real `:4317` in a test — it is reserved for the Epoch-4 port-occupier fault. This is the OTLP-egress analogue of the rmcp Pulse stub (which is MCP read-back) — a distinct mechanism; live `:4317` egress stays a local/operator gate, not CI. (raw-otlp-message-scaffold chunk)
- 2026-06-17: When a seeded builder's output ALSO carries wall-clock fields (e.g. span `start/end_time_unix_nano` from `SystemTime::now()`), assert determinism on a *shape-projection* that excludes those fields (ids + linkage + status), NOT a whole-message byte golden — the seed governs identity, not the clock, so an `insta`/`assert_eq!` over the full payload would flake on the timestamps. Pair it with the both-directions check (same seed ⇒ identical projection; ≥2 seeds ⇒ divergent). (error-spans chunk)
- 2026-06-18: When the harness must reproduce a value the SUT *derives* from content (e.g. an exception fingerprint Pulse computes), compute it as a pure function of that content — NOT via the timeline/RNG seed. The seed governs span identity + timing, not derived-content values; "same seed ⇒ same fingerprint" then holds trivially because the fingerprint ignores the seed. Make the hash version/platform-STABLE: use a fixed algorithm (FNV-1a / sha2), NEVER `std::hash::DefaultHasher` (SipHash — not stable across Rust versions/platforms), so the expected value matches Pulse's derived one on any host. Mirrors the ChaCha8-over-StdRng cross-version-stability choice. (exception-events-fingerprint-control chunk)
- 2026-06-21: To integration-test a real CHILD process (not just an in-proc duplex stub), ship the child as a feature-gated `[[bin]]` with `required-features = ["<feat>"]`, have that feature pull the child's extra deps (e.g. `stub-server = ["rmcp/server", "tokio/io-std"]`), and discover the built exe in the test via `env!("CARGO_BIN_EXE_<bin_name>")`; gate the spawning test with `#[cfg(feature = "<feat>")]` so a default `nextest`/`clippy`/release never compiles the bin or the test. Use an underscore bin name (`stub_pulse_mcp`) so the `CARGO_BIN_EXE_*` var is clean. Run the leg explicitly with `--features <feat>`. This is the real-child analogue of the in-proc rmcp duplex stub. (preflight-readiness-gate chunk)
- 2026-06-21: Testing a bounded connect/deadline (e.g. tonic `Endpoint::connect_timeout`): you cannot deterministically trigger the timeout *elapse* on loopback — a refused localhost port (`127.0.0.1:1`) returns a fast RST `Err`, not a hang, and there is no portable black-hole route to force the wait. Cover the bound with (a) a const sanity-check on the public timeout (`0 < TIMEOUT ≤ ceiling`) and (b) an assertion that the refused/unreachable case returns `Err` promptly — never a real wall-clock wait (`sleep`/black-hole host), which flakes against the zero-retry bar. (otlp-egress-liveness-check chunk)
