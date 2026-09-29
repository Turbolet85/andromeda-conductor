# tests extract

## Relevance
relevant — the chunk's deliverable is a hard grade at the `conductor-run` HARVEST tier over a committed live-leg capture, driven by an operator-gated `--live` leg; test-plan names neither `halo-hue-encoding`'s grade nor `delegated_timing_harvest.rs` (its only hue mentions are leg H in §3 `run` and the `hue-verdict.md` evidence pointer in §9), so whether the plan should name the new grade is a wrap-amendment question, not an edit here.

## Constraints
- A live-Pulse claim grades HARD at the HARVEST tier: a `conductor-run` test over verbatim leg captures, never a read-back `[[expected]]` check; the family precedent is `storm_harvest.rs` / `baseline_harvest.rs` / `restart_harvest.rs` / `severity_harvest.rs` (per test-plan §1 Critical paths; §6 Critical path scenarios). The hue grade follows that shape; whether `delegated_timing_harvest.rs` already runs in the default suite and what it asserts today is research's question.
- The live leg is an operator/local gate only — `scripts/agent-run.{sh,ps1} run --live`, a cargo-feature-gated test invoked directly, or `workflow_dispatch` — and never a CI gate; CI runs stub legs only (per test-plan §9 Live-Pulse scenarios; §11 CI). `halo-hue-encoding` is leg H of the bare `--live` suite, and the leg order and per-leg budget are read from the shells' `live_leg_order`, never restated from a count in prose (per test-plan §3 `run`).
- A committed fixture must carry its MEANING under test, not just parseability — a Rust round-trip through the production reader asserting the properties the consumer depends on, and it must come from the sanctioned provenance set (crate-local `tests/fixtures/` trees; no developer-seeded `runs.db`) (per test-plan §7 Self-bootstrapping requirement). This binds both the new capture and the old-mechanism fixture if it is kept beside it: a retained fixture whose instrument is retired must not stay green while semantically inert. Where the existing hue fixture lives today is research's question.
- Assertion-policy split: only a hard `Fail` is a failing outcome; `CalibrationRegion` / `ManualCheck` / `KnownResidual` / `Blocked` are reported states, never a non-zero exit (per test-plan §3 `run`; §11 E2E). Moving P-025 from a calibration region to a hard grade is exactly a change of which side of this split the claim sits on, so a breach of `duration_ms ≤ 2000` must surface as a test failure, not a recorded state.
- Zero-flakiness: no nextest `retries`, no retry-once, and a result that differs by runner is a shared-state defect (per test-plan §10 Zero-flakiness budget). The harvest test over a committed capture must be deterministic under both `cargo nextest` and `cargo test`.
- A feature-gated target is invisible to the default lint pass, so any `live-pulse`-gated file the chunk touches (e.g. `live_suite.rs`) owes its own `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` (per test-plan §9 Live-Pulse scenarios).
- Coverage stays `--fail-under-lines 60`, and every crate-local `tests/` tree is excluded by the one generic path regex, so a new harvest test or fixture needs no exclusion entry and adds no measured lines (per test-plan §10 Coverage thresholds).

## Patterns to follow
- Harvest-tier grading with synthetic arms that cover every outcome plus the live capture pinned per section, as in `real_model_harvest.rs` running in the default suite over an operator-driven capture (per test-plan §6 Critical path scenarios, the real-model paragraph).
- The `--live` suite freezes each leg's `logs/agent-latest.jsonl` to `runs/live-suite/{leg}.jsonl` because that sink truncates per invocation (per test-plan §3 `run`). Whether Pulse's own `metric.constellation.hue_update_ms` lines reach that frozen file, or must come from Pulse's log instead, is research's question.
- A harness-level wait BETWEEN legs that reproduces a documented SUT-side precondition is part of the leg's firing form and is not a banned sleep (per test-plan §11 E2E). The compact-widget-mounted-before-the-tier-flips precondition belongs in the leg's firing form on that basis, never as a sleep inside a test.
- Separate the stub/synthetic proof from the live proof: stubs prove the grader's wiring, and the live capture is the only evidence of Pulse's behaviour (per test-plan §8 What NOT to mock).

## Anti-patterns to avoid
- NEVER fake Pulse's reaction as a CI verdict: a synthetic arm proving the ≤2000 ms grader is not evidence that Pulse meets it (per test-plan §11 Test Strategy).
- NEVER run the live-Pulse leg as a CI gate or wire it into any default path (per test-plan §11 CI).
- NEVER add a retry policy or read a breach as a flake: a sample over 2000 ms is a real `Fail` (per test-plan §11 Quality; §10 Zero-flakiness budget).

## Contract bindings
- tests ↔ obs: the harvest reads SUT-emitted log lines out of a leg capture, so the capture path binds to the `--live` freeze in §3 `run` and to obs-plan's log format and sink. obs-plan §4 holds the delegated-timing budget wording, which is a wrap amendment per scope.
- tests ↔ contracts: the bound the harvest asserts (`duration_ms ≤ 2000`, hard `Fail`, and which samples are attributable) is owned by `contracts/pulse-p025-measurement-contract.md` §The hard grade, not by test-plan. The harvest must cite that document's re-verified terms.
- tests ↔ security: a committed capture enters the repo as a fixture, so the artifact-hygiene rule applies (no absolute host paths, no internal struct names; security-plan §Error Handling). Whether the hue metric lines carry any host path is research's question.
- tests ↔ harness shells: leg order and budget are owned by `live_leg_order` / `live_leg_budget_sec` in `scripts/agent-run.{sh,ps1}` (test-plan §3 `run`). A change to the drive shape must land in both shells with identical semantics.

## Acceptance criteria contributions
- `cargo nextest run -p conductor-run --profile ci` is green, including a harvest target that grades every attributable `metric.constellation.hue_update_ms` sample in the committed capture at `duration_ms ≤ 2000`. A synthetic over-bound arm must prove that a breach fails the test (per test-plan §6 Critical path scenarios; §1 Critical paths).
- The committed capture, and any retained old-mechanism fixture, is asserted for its MEANING through a round-trip test. That test must fail if the fixture stays parseable but becomes semantically inert (per test-plan §7 Self-bootstrapping requirement).
- The live leg is reachable only from the operator-gated `run --live` path and from no CI stage. If a `live-pulse`-gated file changed, `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` must be green (per test-plan §9 Live-Pulse scenarios; §11 CI).
- `cargo nextest run --workspace --profile ci` and `cargo clippy --workspace --all-targets -- -D warnings` must be green with nextest `retries` unset (per test-plan §10 Build failure conditions; §10 Zero-flakiness budget).
