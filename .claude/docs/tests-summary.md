# Tests Summary — Conductor

_Distilled from `.andromeda/test-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Test tier
**Tier:** Minimal (0), at the upper boundary.
**Justification:** Minimal security tier + a single release-gate surface (cli; webview/IPC are convenience), no persistent user data — but 8 seam crates + 7 critical paths + a "production-grade verification rigor" mandate, so the Minimal baseline is augmented with Section 5 triggers (security-vector negative tests, determinism property/golden, MCP contract test, cross-surface parity, bounded fault-injection) WITHOUT escalating to Comprehensive.

## Harness contract (§3) — agent-driven, NO daemon
setup-project Phase 4 materializes `scripts/agent-run.{sh,ps1}`. Enforcement: `.claude/rules/verification-harness.md`.
- **Test runner:** cargo-nextest (+ `cargo test --doc`) — zero-retry `ci` profile in `.config/nextest.toml`; the version is a reference floor (test-plan §4).
- **5-command discipline:** boot (=MCP preflight gate) / run / status / cleanup / logs.
- **Status endpoint:** none — `status` reads `runs/<run_id>.jsonl` (`jq -e`) or the `runs.db` row (no HTTP/IPC listener).
- **PID file:** N/A — one-shot CLI, no daemon.
- **Log format:** JSONL emission journal under `runs/` (wall-clock `std::time` stamps) — bound to obs-plan §3.
- **Tempdir:** `assert_fs::TempDir` via `CONDUCTOR_RUNS_DIR` per cross-process test.

## E2E coverage (§6 — 7 critical paths)
- **Headless scenario + MCP read-back** (`error-baseline-spike`) — exit 0 + `runs.db` verdict/state + journal + read-back; same-seed replay.
- **Fingerprint-storm** — fingerprints populated; read-back within SLO.
- **Restart-suppression incl. bypass** — suppression is hard Pass/Fail; bypass case reports its distinct outcome.
- **Severity-lifecycle full pass** — auto-resolve + resolution summary; timing hard, severity choice = CalibrationRegion.
- **Known-residual (P-032)** — `state=KnownResidual` (not Fail) from a `degraded_mode` read-back.
- **Coverage-matrix completeness** — every capability in the SUT capability manifest, zero gaps (DoD).
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
