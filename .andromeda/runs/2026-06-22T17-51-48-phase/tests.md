# tests extract

## Relevance
Partial — scenario-config TOML authoring (P-017/P-018) requires test validation via the established loader patterns, but delivers no new test infrastructure or framework changes.

## Constraints
- Per test-plan §1 testable-entities: scenario catalog (P-001..P-060) are statically assertable (P-ID keying completeness) and dynamically assertable via MCP read-back (live Pulse, local gate); this chunk's P-017/P-018 files participate in coverage-matrix completeness (Critical Path 6 / test-scope Sec 4) — per test-plan §6 E2E Scenario Coverage-matrix completeness gate.
- Per test-plan §1 security Vector 2: scenario-config serde+garde validation (error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99, severity-mix sums) is unit-testable with valid/invalid config fixtures; this chunk's fingerprint configuration (storm thresholds 6→Suggested / 12→Autonomous) must pass garde validation at load.
- Per test-plan §4 Unit Test Strategy (Scenario-config validation surface): garde `range` + `#[garde(custom)]` cross-field rules asserted via `Scenario::from_toml_str` with valid/invalid fixtures; established Epoch-7 pattern reused.
- Per test-plan §6 E2E Critical Path 2 (Fingerprint-storm scenario): cli + ipc-internal + persistent-storage surfaces exercise `conductor-faults` fingerprints; exit 0 with per-P-ID `verdict`/`state` rows in `runs.db`; `fingerprints` field populated in the envelope; read-back confirms Pulse fingerprint reaction within SLO (live leg).
- Per test-plan §1 Boundaries: all `class = "Hard"` — fingerprint identity is deterministic hash, storm-cue thresholds are deterministic counts (no CalibrationRegion); scenario.expected outcomes evaluate against one MCP read-back with no per-check window scoping (outcome-coherence file split — chunk scope: same-fp grouping vs distinct-fp vs 6/12 cue tiers).
- Per test-plan §3 Test Harness Contract status endpoint + §7 Test Data & Fixtures: Run-report envelope (verdict/state/fingerprints fields) serialized into `runs.db` row + JSONL journal; envelope golden-locked via insta (`run_id`/timestamp redaction); fingerprints field presence asserted in json schema.

## Patterns to follow
- **Established loader test pattern** (Epoch-7, conductor-core `scenario.rs`): extend per-family rstest `#[rstest]` `#[case]` rows to cover the new fingerprint-storm `.toml` files; `Scenario::from_toml_str` unit test asserts parse + garde validation (all-Hard / kind-usage guards); no production-code change in `scenario.rs` (tests-only).
- **Hardness classification** (test-plan §1): all P-017/P-018 expectations carry `class = "Hard"` (deterministic hash + threshold counts); assert hard pass/fail verdict, never CalibrationRegion.
- **Fingerprint envelope field golden** (test-plan §6 E2E): insta golden on the Run-report envelope includes `fingerprints: [...]` populated (non-empty array for storm scenarios); field presence + type asserted.
- **MCP read-back verification** (test-plan §6 Critical Path 2): the E2E fingerprint-storm scenario runs via assert_cmd; read-back via rmcp stub (CI) / live `andromeda-pulse-mcp` sidecar (local gate) confirms Pulse's fingerprint dedup and cue (Suggested ≥6 hits/30s, Autonomous ≥12 hits/30s) within SLO.

## Anti-patterns to avoid
- NEVER add a scenario without a P-ID (test-plan §11 Universal); P-017 and P-018 are already pinned in the scope — chunk must key each TOML file to these existing P-IDs.
- NEVER include `class != "Hard"` expectations for deterministic fingerprint logic (test-plan §1 Boundaries, test-plan §11 Quality); fingerprint identity (triple match) and storm-cue thresholds (6/12 counts) are hard checks, never model-interpretive.
- NEVER over-mock the MCP read-back leg in the unit loader test — the `Scenario::from_toml_str` unit test validates parse + garde only; the Pulse's fingerprint-dedup behavior is tested at E2E via the 7-critical-paths mechanism (test-plan §6 Critical Path 2, live leg local-gate-only), not faked at unit.

## Contract bindings
- **obs ↔ tests harness (log format):** the per-run emission journal (`runs/<run_id>.jsonl`) carries the envelope fields incl `fingerprints: [...]`; field redaction layer (no internal struct names) asserted by negative test (test-plan §3 Log format / §6 Run-report envelope golden).
- **verify (MCP read-back) ↔ tests:** the E2E fingerprint-storm critical path gates on successful `query_incident_list` canary round-trip + per-P-ID `verdict`/`state` mapping (test-plan §5 Integration, Cross-module patterns — MCP preflight + query_incident_list assertion); preflight must negotiate DOWN to `2024-11-05` protocol (not strict-newer), required-tool presence vs. pinned manifest (test-plan §1 coverage trigger contract-test).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes for new loader test cases (P-017 same-fp/distinct-fp, P-018 6→Suggested / 12→Autonomous thresholds); all `class = "Hard"` guards green.
- (tests) E2E path Critical Path 2 (fingerprint-storm scenario, §6) passes: `conductor run fingerprint-storm --seed <s>` exits 0; `runs.db` envelope has `verdict=Pass`, `state=Pass`, `fingerprints=[...]` populated; MCP read-back confirms Pulse cue mapping (live leg, local gate).
- (tests) Scenario-config garde validation: error fraction, durations, p50≤p95≤p99 ordering (if storm config includes timing), severity-mix sums (if applicable) all validated at load; invalid config matrix asserted to fail loading.
- (tests) Coverage-matrix completeness: generated `coverage-matrix.md` enumerates P-017 and P-018 with zero unclassified entries (static assertion, Critical Path 6); missing P-XXX fails the gate.

## Relevant amendment history
(none) — the test-plan amendments (2026-06-15 through 2026-06-17) touch logging, frontend build-gate deferral, tool-version policy, serialization goldens, and OTLP-egress loopback stubs; none directly amend scenario-catalog validation or the fingerprint-storm E2E path. The established Epoch-7 patterns (loader tests, all-Hard classification, envelope golden) remain stable; this chunk reuses them unchanged.