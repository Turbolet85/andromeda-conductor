## 5. Test Plan Excerpt

### Tests Tier

- **Tier:** Minimal (0)
- **Justification:** The passed `security_tier` is Minimal (0) (single-developer, local-only, no-cloud, no-auth, zero network exposure, only self-generated synthetic data) and the product is effectively a single primary surface (the cli release gate) with no persistent USER data — the lone store (`runs.db` + JSONL) holds only synthetic telemetry classified `low`. These place it below Standard. However, this is Minimal at the upper boundary: 8 workspace crates plus several capability surfaces (>5 entities), 7 critical paths, and a Creator-Brief mandate for "production-grade verification rigor" with a determinism hard-bar and an assertion-policy split — so the Minimal baseline is augmented with the Section 5 coverage triggers (security-vector negative-tests, deterministic-replay property/golden tests, MCP contract test, cross-surface parity, bounded fault-injection) without escalating to Comprehensive (which the project disqualifies itself from: NOT a load-tester, no multi-target/distributed, local-only, mobile N/A).

### Test Harness Contract Summary

- **5-command names:** boot, run, status, cleanup, logs
- **Status JSON shape:**
```json
{
  "run_id": "string", "seed": 0, "scenario": "string", "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0, "slo_tier": "string",
  "journal_emitted_at": "ISO-8601", "read_back_observed_at": "ISO-8601",
  "fingerprints": []
}
```
- **Log format JSON schema:**
```jsonl
{
  "journal_emitted_at": "ISO-8601 from std::time::SystemTime",
  "run_id": "string",
  "seed": 0,
  "scenario": "string",
  "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0,
  "slo_tier": "string",
  "fingerprints": []
}
```
(One JSON object per line; no absolute host paths, no internal struct names; agent-parseable with jq / serde_json.)

### Critical Paths (must-trace)

- **Headless deterministic scenario run with MCP read-back verification:** after `conductor run error-baseline-spike --seed <s>`, exit code 0 AND `runs.db` row has `verdict=Pass` + `state=Pass` with `latency_ms` within `slo_tier` AND the per-run JSONL journal is written AND MCP read-back (`query_incident_list`/`retrieve_report`) confirms Pulse's reaction; same seed ⇒ same stream shape on re-run.
- **Fingerprint-storm scenario:** `conductor run fingerprint-storm` exits 0 with per-P-ID `verdict`/`state` rows recorded; fingerprints field populated in the envelope; read-back confirms Pulse fingerprint reaction within SLO.
- **Restart-suppression incl. one bypass case:** `conductor run restart-suppression` produces a hard pass/fail on suppression logic AND the bypass case reports its expected distinct outcome; journal + `runs.db` row assert the deterministic stream.
- **Severity-lifecycle full pass observing auto-resolve + resolution summary:** one full `severity-lifecycle` pass observes Pulse auto-resolve and a resolution summary via MCP read-back; lifecycle timing asserted as hard pass/fail, severity *choice* asserted as CalibrationRegion (report-for-human, not hard-failed).
- **Known-residual classification path:** `conductor run` for P-032 produces `state=KnownResidual` (NOT `Fail`); a `degraded_mode` read-back result maps to `KnownResidual`; run report distinguishes it from pass/fail/manual-check.
- **Coverage-matrix completeness gate:** the generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries; a missing P-XXX fails the gate ("A P-XXX missing from the matrix is a defect").
- **Both-surface parity:** a scenario launched via the Tauri start command yields the same `runs.db` envelope (verdict/state/seed) as the headless `conductor run` for the same scenario+seed; emission journal written per run from both paths.

### Coverage Triggers Summary

- **security-vector-coverage / negative-test (Vector 1)** — obs implication: "negative-test asserting `std::fs::canonicalize` + type/existence check rejects path-traversal `CONDUCTOR_*` handles BEFORE any `runs.db`/journal write"
- **security-vector-coverage / property-test (Vector 2)** — obs implication: "negative-tests asserting garde rejects out-of-range config (error fraction outside [0,1], negative durations, p50>p95>p99 ordering violations, severity-mix sum violations) at load"
- **security-vector-coverage / negative-test (Vector 4)** — obs implication: "negative-tests asserting (a) `ANDROMEDA_PULSE_DATA_DIR` with injection metacharacters is rejected; (b) sidecar program path is fixed/hard-coded; (c) empty/malformed canary round-trip becomes `blocked` (never false pass-as-empty); (d) bounded prost recursion does not panic on malformed child stdout"
- **security-vector-coverage / negative-test (Vector 3)** — obs implication: "negative-test/contract assertion that Tauri capabilities file is deny-by-default; no `shell-open` plugin and no remote-origin iframe; dependency-pin assertion (`tauri` ≥ 2.10.3)"
- **security-vector-coverage / negative-test (SQL injection)** — obs implication: "negative/static assertion that all `runs.db` access uses rusqlite bound parameters (no string-built SQL)"
- **contract-test** — obs implication: "preflight gate negotiates DOWN to `2024-11-05` (not strict-newer) and required-tool set matches pinned `contracts/` manifest; mismatch ⇒ `blocked`"
- **property-test / determinism discipline** — obs implication: "property/golden test asserting a fixed scenario+seed reproduces an identical emission-journal stream shape across runs"
- **cross-surface-coordination** — obs implication: "Tauri-launched run and headless `conductor run` produce identical envelope verdict/state for the same scenario+seed"
- **chaos-test / fault-injection** — obs implication: "fault-injection scenario tests asserting Pulse's reaction to silence/ramp/port-occupier faults within SLO — bounded 'typical/high' profiles only (P-060); explicitly NOT saturation"
- **supply-chain audit** — obs implication: "`cargo-audit` + `cargo-deny` green over bundled-SQLite-from-C + OTLP/gRPC/MCP tree; committed `Cargo.lock`; toolchain pinned ≥ 1.94.1"

### Quality Gates Summary

- **Zero-flakiness statement:** "flaky tests are NOT tolerated. If a test flakes once, it gets quarantined immediately and fixed (root cause — not retry-once budget). Do NOT set cargo-nextest `retries` > 0. Agent-driven dev cannot distinguish flake from real bug; retry policies mask actual failures. Determinism is enforced upstream (tokio `start_paused` virtual clock, seeded `conductor-timeline`, proptest `proptest-regressions/` persistence) so flakes indicate a real determinism break."
- **Coverage thresholds:** Minimal tier: ≥ 60% line, ≥ 50% branch, ≥ 70% function (enforced by cargo-llvm-cov 0.8.7 `--fail-under-lines 60`)
