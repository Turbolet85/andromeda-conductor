# tests extract

## Relevance
Relevant — the chunk changes what fills `verdict`/`state`/`latency_ms` in the Run-report envelope and adds a read-back extraction seam whose only deterministic proof surface is the stub MCP leg tests own.

## Constraints
- Proof must run on the CI-runnable stub leg only: the read-back boundary is exercised via the in-process JSON-RPC/duplex stub server (+ the child-process spawn leg); the live-Pulse read-back is `workflow_dispatch`/local-gate, never a CI gate (per test-plan §5 Boundary types → Module ↔ external, §9 Live-Pulse scenarios).
- The four state mappings are unit-tier assertions on `conductor-verify`: `ready:false ⇒ Blocked`, `degraded_mode ⇒ KnownResidual`, empty/malformed canary ⇒ `Blocked` (never false-pass-as-empty) — plus `retrieve_report(degraded_mode)` and `query_incident_list`/`retrieve_telemetry_slice` round-trip coverage at integration tier (per test-plan §4 conductor-verify bullet, §5 Cross-module patterns).
- Reported states are envelope values, not process failures: `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` must still exit 0; only a hard `Fail` is non-zero (per test-plan §3 `run` exit-code semantics).
- Envelope/journal shape is golden-locked: canonical line shape via exact-string `assert_eq!` at unit tier, insta reserved for the E2E journal golden with `run_id`/`journal_emitted_at`/`read_back_observed_at` redaction; committed goldens are CI fail-don't-write, never `cargo insta review` (per test-plan §7 Golden artifacts, §4 conductor-report bullet).
- Extraction output must survive the sanitization negative test: no absolute host paths, no internal seam-crate struct names, no corpus content in the journal/`runs.db`/report (per test-plan §3 Log format → Required fields, §5 Run report envelope).
- Zero-flakiness / determinism bar: no nextest `retries`; same scenario+seed ⇒ identical envelope; scheduling under `current_thread` (+ `start_paused` where a clock is involved), ground-truth stamps from `std::time` (per test-plan §10 Zero-flakiness budget, §2 Agent-runnable invariants).
- Workspace line coverage stays ≥ 60% under `cargo llvm-cov nextest --fail-under-lines 60` (per test-plan §10 Coverage thresholds).

## Patterns to follow
- Extend the existing hand-rolled JSON-RPC duplex stub rather than introducing a new mock: `D:\dev\projects\conductor\crates\conductor-verify\tests\common\mod.rs` (`StubConfig` — one knob per readiness leg, RAW Pulse result shapes, no MCP `{content:[…]}` envelope), driven from `D:\dev\projects\conductor\crates\conductor-verify\tests\readback.rs` via `tokio::io::duplex` under `#[tokio::test(flavor = "current_thread")]`. A `degraded_mode` / malformed-payload leg is a new `StubConfig` knob (per test-plan §5 Driver(s), §8 Mocking libraries).
- Keep the spawn/`.env(...)` leg on the committed test binary `D:\dev\projects\conductor\crates\conductor-verify\src\bin\stub_pulse_mcp.rs` exercised by `D:\dev\projects\conductor\crates\conductor-verify\tests\preflight_spawn.rs` — no new spawn code, per the chunk's boundary (per test-plan §5 Module ↔ external, §8 What to mock).
- Per-seam split: pure extraction/mapping functions get crate-local `#[cfg(test)] mod tests` (the pattern already in `D:\dev\projects\conductor\crates\conductor-run\src\lib.rs:491`), cross-boundary legs get crate-local `tests/`; table-drive the `ComparisonKind` × tool matrix with rstest `#[case]` rows (per test-plan §4 Conventions, §7 Fixture library).
- Committed goldens under `<crate>/tests/snapshots/` are the regression fence — `D:\dev\projects\conductor\crates\conductor-run\tests\dispatch_wire.rs` + its two seed snapshots must stay byte-identical (the extraction changes graded values, not the dispatched stream) (per test-plan §7 Golden artifacts).
- Headless envelope parity/E2E goes through `D:\dev\projects\conductor\crates\conductor-cli\tests\cli_smoke.rs`: `Command::cargo_bin` + `CONDUCTOR_RUNS_DIR` → `assert_fs::TempDir`, stdin closed, asserting exit code + the closed bracket-label set + the `runs.db` row (per test-plan §6 cli driver row).

## Anti-patterns to avoid
- NEVER let the stub's canned result stand in for Pulse's *reaction* as a CI verdict — the stub proves extraction wiring/shape only; Pulse's actual behavior stays a local/operator-gate assertion (per test-plan §11 Test Strategy).
- NEVER assert a non-zero exit for a `Blocked`/`KnownResidual`/`CalibrationRegion` outcome, and never let a malformed read-back surface as `Result::Err`/panic in an assertion path (per test-plan §11 E2E, stack-specific).
- NEVER derive the now-measured `latency_ms` / `read_back_observed_at` from tokio's virtual clock — ground-truth stamps are `std::time::SystemTime`/`Instant`, and a paused-clock leak must be caught by the golden (per test-plan §11 Test Data, stack-specific).

## Contract bindings
- tests ↔ obs: test-plan §3 Log format is the source of truth for the JSONL journal envelope that obs-plan §3 derives from; the field-allowlist/redaction layer (`conductor-core::redact`) is obs-owned and tests assert that boundary with a negative test on the journal — extracted observed values pass through it.
- tests ↔ harness/status: `status` has no endpoint; assertions read the `runs.db` row via bound-parameter SQL or `jq -e`/`serde_json::from_str::<RunReportEnvelope>` on `runs/<run_id>.jsonl` (test-plan §3 Status endpoint shape) — the envelope's 11 fields stay unchanged by this chunk.
- tests ↔ security: the absorbed PREREQ (`cargo audit` re-run + `cargo deny check` green, un-drifted `Cargo.lock`) is also a test-plan §10 build-failure condition, so it is a gate this chunk must re-verify against its own lockfile delta.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify -p conductor-run -p conductor-cli` and the workspace `--profile ci` run pass with zero retries configured (per test-plan §3 `run` / §10 Zero-flakiness budget).
- (tests) Against the in-process stub: a `degraded_mode` read-back yields `state=KnownResidual`, an empty/malformed/errored read-back yields a typed `Blocked` value (no `Result::Err`, no panic), and each case still exits 0 (per test-plan §5 Cross-module patterns / §3 `run` exit-code semantics).
- (tests) Envelope + journal goldens hold: canonical line shape via exact-string `assert_eq!`, insta E2E journal golden with `run_id`/timestamp redaction, existing `dispatch_wire__*` seed snapshots unchanged, and a same-seed re-run produces a byte-identical envelope (per test-plan §7 Golden artifacts).
- (tests) Sanitization negative test passes on the extracted observed values — no host paths, internal struct names, or corpus content in journal/`runs.db`/report — and workspace line coverage stays ≥ 60% (per test-plan §3 Log format / §10 Coverage thresholds).

## Relevant amendment history
- `2026-08-10-pulse-run-contract` (§3 `boot`): the preflight timeout became `CONDUCTOR_PREFLIGHT_TIMEOUT` raised to `contracts/pulse-run-contract.toml` `[incident_formation].min_canary_poll_seconds`; the stub leg still completes in <1s via `CanaryPoll::immediate()`. Why it matters here: read-back tests on the stub must not inherit a real-clock floor.
- `2026-06-26-live-counter-channel-stream` (§5): the GUI `Channel`-frame + GUI-parity leg is deferred to the Epoch-9 tauri-driver chunk; run logic is covered at the `conductor-run` unit tier (`drive_run` + seed-stable `Blocked` envelope) plus the `conductor-cli` `cli_smoke` parity E2E. Why it matters: this chunk changes what fills that envelope — the seed-stable `Blocked` envelope assertion is the standing regression fence, and no GUI-tier test is owed.
- `2026-06-16-emission-journal-writer` (§4 conductor-report bullet): unit serialization goldens use exact-string `assert_eq!`; insta stays the E2E mechanism. Why it matters: new/changed envelope values must be golden-locked in that style, not by adding insta to a unit crate.
- `2026-08-13-dispatcher-determinism-goldens` (§7 Golden artifacts): the golden inventory now names the seeded stream families (`replay__*` · `pacing__*` · `dispatch_wire__*`), one file per family × seed, with `*_time_unix_nano` excluded rather than masked. Why it matters: this is the immediately-prior chunk (and this chunk's PREREQ source) — its committed stream goldens must stay untouched by the extraction work.
