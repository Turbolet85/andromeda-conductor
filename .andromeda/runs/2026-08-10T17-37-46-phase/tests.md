# tests extract

## Relevance
Relevant — the chunk lands a new `conductor-verify` preflight readiness leg (`method: integration` per `verification-matrix.json#v2-17`), an artifact-hygiene assertion on an operator-facing string, and a live-Pulse probe whose gate placement my plan governs.

## Constraints
- The live two-launch `pulse-app` probe is an **operator/local gate** (`workflow_dispatch` / `scripts/agent-run.sh`), never a CI gate; CI runs the stub legs only (per test-plan §9 CI Integration → Live-Pulse scenarios).
- The stub proves MCP *wiring*, not Pulse's behavior — a simulated key-divergence leg may assert Conductor's detection, never Pulse's reaction as a CI verdict (per test-plan §11 Test Strategy, stack-specific).
- The detected divergence is a reported envelope **state**, not a process failure: `state="Blocked"` exits 0; only a hard `Fail` is non-zero, and a failed preflight is never silently downgraded (per test-plan §3 → `run` exit-code semantics + §11 E2E, stack-specific).
- The preflight seam's test tier is the **Module ↔ external (MCP sidecar)** row: in-process duplex stub for gate/verdict logic, child-binary/`TokioChildProcess` for the spawn + `.env(...)` path (per test-plan §5 Boundary types covered).
- The precondition string reaches the journal/report, so the no-absolute-host-paths / no-internal-struct-names invariant is asserted by a **negative test** on the artifact (per test-plan §3 Status endpoint shape + §3 Log format → Required fields).
- Zero-flakiness budget is binding: no nextest `retries`, quarantine-and-fix; the new leg must be deterministic (`#[tokio::test(flavor = "current_thread")]`, no `sleep`) (per test-plan §10 Zero-flakiness budget).
- Supply-chain stage stays a build-failure condition — `cargo audit --deny warnings` + `cargo deny check` green, committed un-drifted `Cargo.lock`, toolchain ≥ 1.94.1 (per test-plan §10 Build failure conditions + §9 Supply-chain audit); the chunk's PREREQ keeps this a *bounded wait*, so no floor raise, no `deny.toml` ignore, no CI edit.

## Patterns to follow
- The configurable line-delimited JSON-RPC duplex stub — `D:\dev\projects\conductor\crates\conductor-verify\tests\common\mod.rs` (`StubConfig` = one knob per readiness leg, `serve_stub`) driven by `drive_with(config, manifest) -> ReadyState` in `D:\dev\projects\conductor\crates\conductor-verify\tests\preflight.rs`. A divergence leg is a new `StubConfig` knob plus one `#[tokio::test]` case asserting `report_state() == ReportState::Blocked` and the named `blocked_precondition` (per test-plan §5 / §8 in-process duplex).
- Named-precondition constants rather than inline literals — `UNREACHABLE_PRECONDITION` at `D:\dev\projects\conductor\crates\conductor-verify\src\preflight.rs:99` with its mirror at `D:\dev\projects\conductor\crates\conductor-run\src\lib.rs:87`; tests bind to the constant, keeping assertions off fixture-internal values (per test-plan §11 Unit).
- The real-child-process leg — `D:\dev\projects\conductor\crates\conductor-verify\tests\preflight_spawn.rs`, gated `#![cfg(feature = "stub-server")]`, driving `preflight_boot` over the spawned `stub_pulse_mcp` binary; this is the tier where a `ANDROMEDA_PULSE_DATA_DIR`-keyed divergence crosses a true process boundary (per test-plan §5 Module ↔ external, `TokioChildProcess` test-binary).
- CLI-surface confirmation of the Blocked envelope — `D:\dev\projects\conductor\crates\conductor-cli\tests\cli_smoke.rs` (assert_cmd + `[BLOCKED]` label, `runs.db` row + JSONL journal, stdin closed, NO_COLOR-stable) (per test-plan §6 cli driver row + Selector strategy).
- Isolation/fixtures: `assert_fs::TempDir` with `.env("CONDUCTOR_RUNS_DIR", …)` for any cross-process leg; rstest `#[case]` rows for the readiness-leg matrix (per test-plan §3 Per-test isolation + §7 Seed strategies).

## Anti-patterns to avoid
- NEVER surface the divergence as `Result::Err`, a panic, or a non-zero exit — `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported envelope states (per test-plan §11 E2E, stack-specific).
- NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` (or any operator/workspace value) into sidecar argv or a shell — `.env(...)` only, after injection-metacharacter rejection, against a fixed hard-coded program path; the existing negative test covers this and must stay green (per test-plan §11 Mocking, stack-specific).
- NEVER let the two-launch probe become a real network call or a CI dependency, and NEVER `sleep(N)` to synchronize it — loopback stubs in CI; wait on the envelope `state` (per test-plan §11 Universal + §11 E2E).

## Contract bindings
- **tests ↔ obs (log format):** test-plan §3 Log format owns the emission-journal schema and obs derives from it; the field-allowlist/redaction layer (no host paths, no struct names) is the obs-owned half, and my plan asserts that boundary via a negative test — the new precondition string is exactly that boundary crossing (test-plan §3 → obs-plan §3).
- **tests ↔ arch (readiness result):** the `blocked_precondition` field and `state="Blocked"` are the Status-endpoint-shape fields the harness `status` command polls from the `runs.db` row / JSONL journal (test-plan §3 Status endpoint shape ↔ architecture §Standard Contracts).
- **tests ↔ security (CI gate):** the supply-chain stage (`cargo audit` / `cargo deny`) runs in the same workflow as the test gates and is a shared build-failure condition (test-plan §9 Supply-chain audit ↔ security-plan §Dependency Security).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify` passes with a new preflight leg in which the app-side workspace key diverges from the sidecar's `data_dir` key, asserting `report_state() == Blocked` with the distinct named precondition — not `Fail`, not `Err`, not a panic (per test-plan §5 Boundary types covered + §11 E2E).
- (tests) A negative test asserts the new precondition string — and the journal/report line carrying it — contains no absolute host path and no internal seam-crate struct name (per test-plan §3 Status endpoint shape + §3 Log format → Required fields).
- (tests) Workspace gate green with zero retries: `cargo nextest run --workspace --profile ci`, `cargo test --workspace --doc`, `cargo clippy --workspace --all-targets -- -D warnings`, coverage `--fail-under-lines 60`, and the sixth `cargo audit` re-check recorded with `cargo deny check` observed green (per test-plan §3 `run` + §10 Quality Gates).
- (tests) The two-launch live verdict is executed and recorded on the operator/local gate only — no CI job acquires a dependency on a live `pulse-app`/Pulse process (per test-plan §9 CI Integration → Live-Pulse scenarios).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Log format) — established that the `tracing` self-obs stream (`logs/agent-latest.jsonl`, `logs/conductor-tauri.jsonl`) is a *separate* artifact from the per-run emission journal (`runs/<run_id>.jsonl`, the envelope/SLO ground truth); the two schemas must not be conflated. Directly relevant: the operator-facing precondition lands in "the run report / logs", so the hygiene assertion must target the right artifact.
- **2026-06-16-emission-journal-writer** (§4, conductor-report bullet) — unit-tier serialization goldens use exact-string `assert_eq!` (matching the `verdict.rs` / `report_state.rs` canonical-serialization pattern); insta stays the E2E journal-golden mechanism with `run_id`/timestamp redaction. Relevant: a golden lock on the new precondition's serialized form belongs at the unit tier as an exact assert, not an insta snapshot.
- **2026-08-09-sut-load-envelope** (§1 cli Signal · §6 cli driver row + Selector strategy) — the six bracket labels are the closed per-P-ID lamp set, with `[ENVIRONMENT-SUSPECT]` recorded as a run-level non-lamp qualifier selectors must expect. Relevant if this chunk asserts cli stdout for `[BLOCKED]`: assert the named set plus the qualifier, never "stdout carries only these six".
