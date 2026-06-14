## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Minimal
- **Justification:** "this is Minimal at the upper boundary: 8 workspace crates plus several capability surfaces (>5 entities), 7 critical paths, and a Creator-Brief mandate for 'production-grade verification rigor' with a determinism hard-bar and an assertion-policy split — so the Minimal baseline is augmented with the Section 5 coverage triggers ... without escalating to Comprehensive."

### Test Harness Contract Summary
- **5-command names:** boot (preflight readiness gate — no daemon to start), run, status, cleanup, logs
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
(No HTTP/IPC status endpoint exists — no inbound listener. This is the Run-report envelope serialized identically into the Markdown report, the `runs.db` row, and each JSONL journal line. Artifacts MUST NOT leak absolute host paths or internal seam-crate struct names.)

### Critical Paths (must-be-accessible)
- **Headless deterministic scenario run with MCP read-back (`error-baseline-spike`):** After `conductor run error-baseline-spike --seed <s>`, exit 0 AND `runs.db` row has `verdict=Pass` + `state=Pass` with `latency_ms` within `slo_tier` AND the per-run JSONL journal is written AND MCP read-back confirms Pulse's reaction; same seed ⇒ same stream shape on re-run.
- **Fingerprint-storm (fault-injection burst + read-back):** `conductor run fingerprint-storm` exits 0 with per-P-ID `verdict`/`state` rows recorded; `fingerprints` field populated in the envelope; read-back confirms Pulse fingerprint reaction within SLO.
- **Restart-suppression incl. one bypass case:** `conductor run restart-suppression` produces a hard pass/fail on suppression logic AND the bypass case reports its expected distinct outcome; journal + `runs.db` row assert the deterministic stream.
- **Severity-lifecycle full pass (auto-resolve + resolution summary):** One full `severity-lifecycle` pass observes Pulse auto-resolve and a resolution summary via MCP read-back; lifecycle timing asserted as hard pass/fail, severity choice asserted as CalibrationRegion (report-for-human, not hard-failed).
- **Known-residual classification path (P-032):** `conductor run` for P-032 produces `state=KnownResidual` (NOT `Fail`); a `degraded_mode` read-back result maps to `KnownResidual`; run report distinguishes it from pass/fail/manual-check.
- **Coverage-matrix completeness gate:** The generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries; a missing P-XXX fails the gate ("A P-XXX missing from the matrix is a defect"); this is the "0.1.0 done" definition.
- **Both-surface parity (control panel AND headless):** A scenario launched via the Tauri start command yields the same `runs.db` envelope (verdict/state/seed) as the headless `conductor run` for the same scenario+seed; emission journal written per run from both paths.

### Coverage Triggers Summary
- **Security-vector CLI input (Vector 1)** (compliance / negative-test) — a11y implication: scripted verification that path-traversal `CONDUCTOR_*` handles are rejected before any artifact write; no a11y-surface-specific check (CLI-layer).
- **Config-file parsing garde validation (Vector 2)** (compliance / property-test) — a11y implication: ensure config-validation error states surface through accessible error output (no color-only signaling of invalid-config rejection).
- **MCP STDIO injection / preflight downgrade (Vector 4)** (compliance / negative-test) — a11y implication: verify `blocked` preflight state is announced via text/`aria-live`, never a silent or color-only downgrade.
- **Tauri capabilities deny-by-default (Vector 3)** (compliance / negative-test) — a11y implication: confirm only the deny-by-default command set + live-counter Channel reach the webview surface under test (no remote-origin iframe affecting a11y selectors).
- **SQL bound-parameter `runs.db` access** (compliance / negative-test) — a11y implication: none (storage-layer, no accessible surface).
- **Pinned MCP contract manifest** (compliance / contract-test) — a11y implication: verify a contract-mismatch `blocked` outcome renders with a text-paired status label, not color alone.
- **Determinism replay (`conductor-timeline`)** (cognitive / property-test) — a11y implication: deterministic stream shape supports reliable scripted keyboard/selector verification across re-runs (stable status labels).
- **Cross-surface coordination (control panel + headless parity)** (multi-platform / cross-surface) — a11y implication: axe-core / pa11y run on the desktop-webview surface confirming the GUI-launched run renders the same accessible envelope as headless.
- **Bounded fault-injection (P-060 "typical/high")** (chaos / fault-injection) — a11y implication: focus retention and `aria-live` HOLD/verdict announcements survive silence/ramp/port-occupier fault states (bounded, not saturation).
- **Supply-chain audit (`cargo-audit` + `cargo-deny`)** (compliance / supply-chain) — a11y implication: none (build-gate, no accessible surface).

### Quality Gates Summary
- **Zero-flakiness statement:** "flaky tests are NOT tolerated. If a test flakes once, it gets quarantined immediately and fixed (root cause — not retry-once budget). Do NOT set cargo-nextest `retries` > 0."
- **Coverage thresholds:** Minimal tier (selected) — line ≥ 60%, branch ≥ 50%, function ≥ 70%; enforced via cargo-llvm-cov `--fail-under-lines 60` (line gate is the binding CI gate; branch coverage is informational on the stable toolchain).
