## 5. Creator Brief Excerpt

### Must-Work Scenarios

- "End-to-end proof against a real Pulse: at minimum `error-baseline-spike`, `fingerprint-storm`, `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass observes auto-resolve + resolution summary."
- "Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed; emission journal written per run."
- "Run report distinguishes pass / fail / manual-check / known-residual; manual checklist renders with induced-state context; P-032 reports as known-residual (not as a surprise failure)."
- "`coverage-matrix.md` complete: all 60 P-IDs classified auto / drive+observe / static-only, zero gaps." ("A P-XXX missing from the matrix is a defect. '0.1.0 done' requires the matrix complete with zero unclassified entries.")
- Coverage classification the harness must honor: "**auto** — Conductor drives AND asserts (journal + MCP read-back + timing)"; "**drive+observe** — Conductor induces the state; the operator confirms the visual/UX claim via the generated checklist"; "**static-only** — no dynamic telemetry dimension ... these stay with Pulse's own test matrix — Conductor explicitly does NOT duplicate them."

### Risk Tolerance Hints

- Scope-law rigor (no feature creep, no untested claims): "Conductor implements **exactly enough** fault injection and scenario surface to verify each P-XXX claim — no more (feature creep), no less (untested claims). Every scenario cites the P-IDs it verifies; a scenario with no P-ID does not exist."
- Assertion-policy split (THE rigor boundary, creator-mandated): "deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) = hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting — model-side per the amended spec) = calibration-region checks + report-for-human, never hard-failed on exact values."
- Determinism as a hard quality bar: "Deterministic under a seed: same scenario + seed ⇒ same stream shape. Wall-clock-stamped **emission journal** per run = ground truth of what was sent when (the left side of every SLO check)."
- Audience / scale (solo local, MVP-version 0.1.0 but production-grade verification rigor): "The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host."
- Harness-execution mandate: "Development Style: **agent-driven** (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin shell over the same commands)."
- MCP read-back prerequisite that must never be silently assumed: "Operator prerequisite (documented, never silently assumed): Pulse built with the non-default `mcp-server` cargo feature AND runtime env `ANDROMEDA_PULSE_MCP_ENABLED` AND the sidecar spawned with `ANDROMEDA_PULSE_DATA_DIR` equal to the live Pulse's data-dir ... mismatch ⇒ read-back silently empty; preflight must canary-round-trip a known incident before any scenario trusts read-back."

### Test Anti-Patterns (creator's explicit asks)

- "NOT a load-tester: bounded 'typical / high' load profiles only (P-060 SLO checks); 50k+ spans/sec saturation regimes are explicitly out (the spec excludes them from Conductor scenarios; Pulse's own `perf_load_profiles.rs` suite owns saturation)."
- "NO Pulse process management — Conductor never starts/stops/restarts Pulse or edits its config; those are operator-pause steps. (Keeps Conductor honest: it validates Pulse from the outside, as a real workload.)"
- "NO UI automation of Pulse (Playwright/axe live in Pulse's own suites) — visual claims are operator checklist items."
- "NO scenario DSL — declarative config files + the built-in catalog; new behavior = new P-XXX first."
- "NO multi-target / distributed generation; no cloud; local dev host only."
- Reference prior art read-only (do not vendor): "Reference prior art in the Pulse repo (read, don't vendor): `crates/ingest/examples/inject_demo.rs` (minimal injector) and `crates/ingest/examples/load_profiles.rs` ... — proven emission patterns to mine."
- Environment requirement for two scenario families: "P-032/P-036 scenarios run inside a real git workspace with known recent commits (documented per-scenario environment requirements)."
