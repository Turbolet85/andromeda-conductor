# tests extract

## Relevance
Partial — Constellation + context-grounding scenarios (P-025..P-027, P-032, P-036) are new catalog entries exercising existing primitives and report states; test coverage is catalog + scenario-model guards, not infrastructure.

## Constraints
- Per §1, scenario catalog is partially-testable: static assertion that every scenario carries a P-ID and generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries (unit-assertable, never `missing P-XXX`); dynamic auto scenarios assert via MCP read-back (live Pulse, local gate); drive+observe scenarios (ManualCheck) are operator-checklist items, not agent-drivable — per §1 Untestable zones ("the operator confirms the visual/UX claim via the generated checklist"). P-032 known-residual is pre-accepted deviation — reported state, not hard-failed.
- Per §2 Test Strategy / agent-runnable invariants: "No human-in-loop verification" — ManualCheck visual claims do NOT block CI, they are operator-checklist artifacts (assertion-policy split — model-interpretive claims are "calibration-region checks + report-for-human, never hard-failed").
- Per §4 Unit Test Strategy (conductor-core bullet): scenario-config validation is unit-testable with valid/invalid fixtures under rstest (already established pattern); per-scenario class/state guards are loaded + asserted at unit level.
- Per §3 Test Harness Contract (bootstrap phases), scenario configs are declarative one-per-P-ID under `scenarios/` (serde + garde).
- Per scope §Boundaries, P-032/P-036 must run inside a real git workspace with known commits (operator/Epoch-10 precondition; declared intent in TOML).

## Patterns to follow
- Per §4, rstest `#[case]` table-driven rows over the P-001..P-060 catalog and garde-config matrices — apply to new P-IDs (P-025/026/027/032/036 catalog fixtures).
- Per §4 Unit Test Strategy (conductor-core bullet), "static assertion that every catalog scenario carries a P-ID and generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries" — test wiring must assert P-025/026/027/032/036 presence in coverage-matrix output.
- Per scope §Definition of done, scenario TOMLs must be "loadable + garde-valid" — validate via unit test against the new P-ID fixtures.
- Per §1 Critical paths item 6 (Known-residual classification path), P-032 produces `state=KnownResidual` (NOT `Fail`); a `degraded_mode` read-back maps to KnownResidual (arch Standard Contracts) — unit assertion that verdict/state mapping applies.

## Anti-patterns to avoid
- Do NOT unit-test ManualCheck visual/halo/constellation rendering claims — per §1 Untestable zones ("the operator confirms the visual/UX claim via the generated checklist") and §2 agent-runnable invariants ("no human-in-loop verification, no visual regression with human review"). ManualCheck scenarios go to operator-checklist artifacts, never hard-fail CI.
- Do NOT invent live Pulse read-back tests for P-025/026/027 ManualCheck scenarios — per §1 E2E gate, "full dynamic end-to-end (auto scenarios) requires a live Pulse" (CI unavailable); drive+observe scenarios are local-gate-only with operator verification.
- Do NOT duplicate static-only Pulse capabilities per §1 Untestable zones ("Pulse's static-only ... explicitly does NOT [be] duplicat[ed]").

## Contract bindings
**tests ↔ obs:** per amendment 2026-06-15-structured-logging-stack §3 Test Harness Contract / Log format, the per-run emission journal (`runs/<run_id>.jsonl`, SLO ground truth + Run-report envelope) is SEPARATE from the self-obs stream (stderr / `logs/agent-latest.jsonl`); Do Not conflate envelope schemas.

## Acceptance criteria contributions
- (tests) Scenario TOMLs exist for P-025, P-026, P-027, P-032, P-036 — each carrying P-ID, expected outcome, SLO tier; `cargo nextest run -p conductor-core` loads + guards them (valid/invalid fixtures via rstest).
- (tests) Coverage-matrix generated from the new scenarios: static assertion all 60 P-IDs present, zero gaps — `cargo test --doc` + doctest assertions green.
- (tests) P-032 known-residual path: unit assertion that a KnownResidual read-back state maps to `state=KnownResidual` in the envelope (NOT `Fail`); per §1 Critical paths item 6.
- (tests) P-036 fingerprint recurrence: unit assertion that cross-run `runs.db` fingerprint index correctly surfaces "Previously seen" on repeat incident (golden `retrieve_incident_list` envelope assertion, live read-back local-gate-only).

## Relevant amendment history
- 2026-06-16-emission-journal-writer (§4 conductor-report bullet): unit serialization goldens use exact-string `assert_eq!` at unit level (verdict/report_state/scenario canonical shapes); insta stays E2E journal-golden mechanism with `run_id`/timestamp redaction. **Applies:** P-032/P-036 envelope assertions must use exact-string goldens at unit, insta at E2E.
- 2026-06-16-test-framework-fixtures-coverage-tooling (§4): external-CLI tools (cargo-nextest, cargo-llvm-cov) are reference floors; dev-deps caret-resolved with `Cargo.lock` authoritative. **Applies:** new P-ID fixtures inherit the established nextest + insta mechanism (no version drift re-validation needed).