# Tests Summary — Conductor

_Distilled from `.andromeda/test-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Test tier
**Tier:** Minimal (0), at the upper boundary.
**Justification:** Minimal security tier + a single release-gate surface (cli; webview/IPC are convenience), no persistent user data — but 8 seam crates + 7 critical paths + a "production-grade verification rigor" mandate, so the Minimal baseline is augmented with Section 5 triggers (security-vector negative tests, determinism property/golden, MCP contract test, cross-surface parity, bounded fault-injection) WITHOUT escalating to Comprehensive.

## Harness contract (§3) — agent-driven, NO daemon
setup-project Phase 4 materializes `scripts/agent-run.{sh,ps1}`. Enforcement: `.claude/rules/verification-harness.md`.
- **Test runner:** cargo-nextest (+ `cargo test --doc`) — zero-retry `ci` profile in `.config/nextest.toml`; the version is a reference floor (test-plan §4). `cargo test -p <crate>` (full, non-doc) is a standing **runner-portability** gate beside it: only the shared-process runner exposes a test relying on nextest's per-test process for isolation.
- **Mutation instrument:** cargo-mutants — operator/local audit, never a CI stage; full-crate-path `-f`, `--test-tool=nextest` required; the exit code carries no verdict either way (`Found 0 mutants` is a NO-OP at exit 0; a non-zero exit reflects surviving/timeout classes) — gate on the tallies. Named survivors end killed or accepted-deliberate against a cited rule (test-plan §4/§9/§10).
- **5-command discipline:** boot (=MCP preflight gate) / run / status / cleanup / logs.
- **Status endpoint:** none — `status` reads `runs/<run_id>.jsonl` (`jq -e`) or the `runs.db` row (no HTTP/IPC listener).
- **PID file:** N/A — one-shot CLI, no daemon.
- **Log format:** JSONL emission journal under `runs/` (wall-clock `std::time` stamps) — bound to obs-plan §3.
- **Tempdir:** `assert_fs::TempDir` via `CONDUCTOR_RUNS_DIR` per cross-process test.

## E2E coverage (§6 — 7 critical paths)
- **Headless scenario + MCP read-back** (`error-baseline-spike`) — exit 0 + `runs.db` row (declare-only family: `verdict` null / `state=KnownResidual`; the live claim grades at the harvest tier, `baseline_harvest.rs` over `triage.cue.emit`) + journal + read-back; same-seed replay.
- **Fingerprint-storm** — envelope `fingerprints` present and SUT-populated under deterministic L4 (payload-invariant `det-*` fixture constants — never an identity carrier); read-back token checks declare-only (permanent `degraded_mode`); the live proof is the test-only harvest of Pulse's `triage.pattern.storm.detected` line at the `conductor-run` unit tier.
- **Restart-suppression incl. bypass** — declare-only row (`verdict` null, `state=KnownResidual`, `<90s`); the suppression/bypass claim grades as hard predicates at the harvest tier (`conductor-run/tests/restart_harvest.rs`, verbatim leg captures): the P-015 restart pair, the drop→keep crossing at the 30-sample persistence cutoff, and the P-057 absolute arm (the relative arm is unreachable at shipped SUT constants).
- **Severity-lifecycle full pass** — declare-only family of FIVE scenarios (no scenario is named `severity-lifecycle`); each row `verdict` null / `state=KnownResidual`. Auto-resolve grades HARD at the harvest tier (`severity_harvest.rs`) on the active-set-empties instant against the 120s window + 30s tick, plus the same-fingerprint retrigger's `created=true, deduped=false`; severity choice = CalibrationRegion at the cue tier. The resolution-summary half is unreachable under deterministic L4 — recorded, not asserted.
- **Known-residual (P-032)** — `state=KnownResidual` (not Fail) from a `degraded_mode` read-back.
- **Coverage-matrix completeness** — every capability in the SUT capability manifest, zero gaps (DoD). Second axis: `check_scenario_backing` asserts every `Auto`-classified capability has a scenario naming it, exact-set against the `UNBACKED_AUTO` pin (fails on a new unbacked claim, pin rot, or a pin that lost its `Auto` mode).
- **Cross-surface parity** — Tauri-launched vs headless identical envelope for the same seed.

## Quality gates (§10)
| Gate | Threshold | Tool |
|---|---|---|
| Coverage (line) | ≥60% (binding) | cargo-llvm-cov `--fail-under-lines 60` |
| Coverage (branch / fn) | ≥50% / ≥70% (branch informational on stable) | cargo-llvm-cov |
| Flakiness | zero — quarantine + fix, NO retries | nextest `retries=0` |
| Performance | N/A — not a load-tester (50k+ spans/s is Pulse's) | — |
| Supply chain | audit/deny green + `Cargo.lock` un-drifted | cargo-audit / cargo-deny |

## Universal anti-patterns
- No real network/time in tests (loopback stubs; `start_paused` scheduling; `std::time` only for journal stamps).
- No Percy/Chromatic/visual-regression-with-human-review; no `cargo insta review` interactive accept in CI.
- No scenario without a P-ID; no inbound listener (the `:4317` port-occupier releases on cleanup).
- `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported states, not non-zero exits.

## Critical decisions
- **cargo-nextest** (stable exit codes + JUnit) is the agent-readable runner the `run` command wraps.
- **Per-surface drivers:** assert_cmd (cli release gate) · rmcp stub (MCP, CI) · tauri::test mock runtime (IPC) · tauri-driver/WebdriverIO (webview, Linux+xvfb).
- **Open:** no JS/TS unit runner for the React webview (GUI is convenience; CLI is the gate) — re-run research if frontend unit coverage is later required.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md` + `verification-harness.md`.
