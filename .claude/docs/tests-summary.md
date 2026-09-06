# Tests Summary — Conductor

_Distilled from `.andromeda/test-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Test tier
**Tier:** Minimal (0), at the upper boundary.
**Justification:** Minimal security tier + a single release-gate surface (cli; webview/IPC are convenience), no persistent user data — but 8 seam crates + 7 critical paths + a "production-grade verification rigor" mandate, so the Minimal baseline is augmented with Section 5 triggers (security-vector negative tests, determinism property/golden, MCP contract test, cross-surface parity, bounded fault-injection) WITHOUT escalating to Comprehensive.

## Harness contract (§3) — agent-driven, NO daemon
setup-project Phase 4 materializes `scripts/agent-run.{sh,ps1}`. Enforcement: `.claude/rules/verification-harness.md`.
- **Test runner:** cargo-nextest (+ `cargo test --doc`) — zero-retry `ci` profile in `.config/nextest.toml`; the version is a reference floor (test-plan §4). `cargo test -p <crate>` (full, non-doc) is a standing **runner-portability** gate beside it: only the shared-process runner exposes a test relying on nextest's per-test process for isolation.
- **Mutation instrument:** cargo-mutants — operator/local audit, never a CI stage; full-crate-path `-f`, `--test-tool=nextest` required; the exit code carries no verdict either way (`Found 0 mutants` is a NO-OP at exit 0; a non-zero exit reflects surviving/timeout classes) — gate on the tallies (`missed.txt` == exactly the run's accepted-deliberate survivors, empty only when none is accepted; every other named survivor in `caught.txt`). Named survivors end killed or accepted-deliberate against a cited rule, and the accepted roster is a SET recorded in test-plan §12 (test-plan §4/§9/§10).
- **5-command discipline:** boot (=precondition probe, then the MCP preflight gate — an unmet precondition short-circuits and SKIPS the preflight) / run / status / cleanup / logs. `cleanup` must delete from **all three `runs.db` tables** (`runs`, `run_check`, `run_envelope`) — teardown covers every table a run writes, or it leaves orphans — and is idempotent (zero-row `DELETE` succeeds; file-absence is not an error).
- **Status endpoint:** none — `status` reads the newest ENVELOPE line of `runs/<run_id>.jsonl` (`jq -e`; never the file's last line, which is a `CheckRecord` on a run that graded checks) or the `runs.db` row (no HTTP/IPC listener). The polled shape is the eleven-field Run-report envelope; **per-check detail is a separate finer grain, never an extension of it** — the `run_check` table keyed `(run_id, scenario, check_index)` (read via `RunsDb::checks_for`), its own journal lines, and an indented per-check line in the Markdown report.
- **PID file:** N/A — one-shot CLI, no daemon.
- **Log format:** JSONL emission journal under `runs/` (wall-clock `std::time` stamps) — bound to obs-plan §3. The journal carries **TWO report-seam record shapes**: the run **envelope line** (eleven fields, byte-unchanged) and the per-check **`CheckRecord` line** (`run_id` · `scenario` · `check_index` · `kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms`; `budget_ms` null when the check inherits its tier). A blocked or declare-only scenario emits ZERO check records. Every line is `jq -c`-parseable, and a TYPED parse discriminates the shape — `serde_json::from_str::<RunReportEnvelope>(line)` targets the envelope line specifically, not every line.
- **Per-check deadlines:** an `[[expected]]` check may declare `budget_ms`, a sub-tier deadline beneath the closed `slo_tier` set; it is graded against `ExpectedCheck::effective_deadline_ms(tier)` (the budget when declared, else the tier's). Because `observe()` runs ONCE per scenario, per-check `latency_ms` values are equal by construction — the DEADLINE is what separates two checks' timing verdicts.
- **Tempdir:** `assert_fs::TempDir` via `CONDUCTOR_RUNS_DIR` per cross-process test.

## E2E coverage (§6 — 7 critical paths)
- **Headless scenario + MCP read-back** (`error-baseline-spike`) — exit 0 + `runs.db` row (declare-only family: `verdict` null / `state=KnownResidual`; the live claim grades at the harvest tier, `baseline_harvest.rs` over `triage.cue.emit`) + journal + read-back; same-seed replay.
- **Fingerprint-storm** — envelope `fingerprints` present and SUT-populated under deterministic L4 (payload-invariant `det-*` fixture constants — never an identity carrier); read-back token checks declare-only (permanent `degraded_mode`); the live proof is the test-only harvest of Pulse's `triage.pattern.storm.detected` line at the `conductor-run` unit tier.
- **Restart-suppression incl. bypass** — declare-only row (`verdict` null, `state=KnownResidual`, `<90s`); the suppression/bypass claim grades as hard predicates at the harvest tier (`conductor-run/tests/restart_harvest.rs`, verbatim leg captures): the P-015 restart pair, the drop→keep crossing at the 30-sample persistence cutoff, and the P-057 absolute arm (the relative arm is unreachable at shipped SUT constants).
- **Severity-lifecycle full pass** — declare-only family of FIVE scenarios (no scenario is named `severity-lifecycle`); each row `verdict` null / `state=KnownResidual`. Auto-resolve grades HARD at the harvest tier (`severity_harvest.rs`) on the active-set-empties instant against the 120s window + 30s tick, plus the same-fingerprint retrigger's `created=true, deduped=false`; severity choice = CalibrationRegion at the cue tier. The resolution-summary half is unreachable under deterministic L4 — recorded, not asserted.
- **Known-residual (P-032)** — `state=KnownResidual` (not Fail) from a `degraded_mode` read-back.
- **Coverage-matrix completeness** — every capability in the SUT capability manifest, zero gaps (DoD). Second axis: `check_scenario_backing` asserts every `Auto`-classified capability has a scenario naming it, exact-set against the `UNBACKED_AUTO` pin (fails on a new unbacked claim, pin rot, or a pin that lost its `Auto` mode).
- **Cross-surface parity** — the in-process core arm (`conductor_run::{preflight, drive_run}`, the composition behind the Tauri `start_run` command — no `tauri::*` item) vs headless `conductor run`: identical envelope for the same seed, both into ONE `runs.db` under a non-default `CONDUCTOR_RUNS_DIR`, compared between the two persisted envelopes. Lives in `crates/conductor-cli/tests/cross_surface_parity.rs`, the package declaring the `conductor` bin. The control-panel-LAUNCHED half of this path is DEFERRED to the tauri-driver leg and still owed against the Creator Brief.

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
- **Per-surface drivers:** assert_cmd (cli release gate) · the hand-rolled JSON-RPC `stub_pulse_mcp` (MCP, CI — rmcp removed 2026-06-27; the stub's incident item key is `incident_id`, the live sidecar's) · tauri::test mock runtime (IPC) · tauri-driver/WebdriverIO (webview — three suite families over the one stack: routine `--e2e`, driven `a11y:driven`, screen-reader `a11y:sr*` with NVDA's speech log under `CONDUCTOR_NVDA`; Linux+xvfb in CI, headful on the Windows WebView2 host under `CONDUCTOR_MSEDGEDRIVER` — a cross-major driver/runtime pair measured working 2026-09-02, the refresh being the operator's host task; macOS driver-less). Screen-reader browse-mode reading is agent-untestable today (a missing key path, not a missing driver). A live-Pulse leg is operator/local only and may ride a cargo-feature gate (`--features live-pulse`), which owes its own clippy leg.
- **Open:** no JS/TS unit runner for the React webview (GUI is convenience; CLI is the gate) — re-run research if frontend unit coverage is later required.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md` + `verification-harness.md`.
