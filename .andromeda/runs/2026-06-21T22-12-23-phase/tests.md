# tests extract

## Relevance
Partial — covers the scenario catalog surface (TOML configuration + serde/garde validation) but excludes the live MCP read-back verification and CLI driver (deferred to Epoch 8 and 10).

## Constraints
- Per test-plan §1 test-scope entity "Scenario catalog (P-001..P-060...)" — P-ID keying + declarative config TOML validation is unit-testable (§1 Testability: partially-testable, static assertion on matrix completeness and per-scenario P-ID keying)
- Per test-plan §4 Unit test strategy — scenario-config validation surface uses garde `range` + `#[garde(custom)]` cross-field rules with valid/invalid fixtures; also property-tested per §7
- Per test-plan §1 coverage trigger "security-vector-coverage (negative-test)" — negative-tests asserting garde rejects out-of-range config (error fraction [0,1], durations non-negative, p50≤p95≤p99 ordering, severity-mix sums) at load (§1 Trigger type: security-vector-coverage / property-test)
- Per test-plan §3 Test Harness Contract — scenario catalog scenarios referenced by the `run` command (P-ID-driven invocation via `conductor run <P-ID>` or `conductor suite`) — file under `scenarios/` and deserialize via `Scenario::from_toml_str`
- Per test-plan §2 test pyramid + §4 — unit test framework is cargo-nextest 0.9.137 with rstest fixtures; table-driven `#[rstest]` `#[case]` rows over the P-ID catalog and garde-config matrices; the 5-command `run` semantics exit 0 on all checks Pass (§1 Coverage triggers: property-test + contract-test)
- Per test-plan §1 test-scope critical paths — "Coverage-matrix completeness gate ... generated `coverage-matrix.md` enumerates all 60 P-IDs ... zero unclassified entries ... zero gaps" (§1 Critical paths definition-of-done) — this chunk covers P-001..P-004 keying and SLO-tier expression

## Patterns to follow
- Declarative scenario config fixtures one-per-P-ID under `scenarios/` (test-plan §4 conventions) — P-001..P-004 TOML keyed by `name`, `p_ids = [P-XXX]`, `seed`, `slo_tier`, `[[phases]]`, and (new) `expected` block + optional `holds` for drive+observe expression (per scope.md model-extension follow-up)
- Fixture round-trip test (mirrors `error-baseline-spike.toml` precedent per scope.md definition-of-done) — deserialize via `Scenario::from_toml_str`, produce valid `PhaseTimeline` through existing scheduler, assert-green via rstest `#[fixture]` and `#[rstest]` table-driven rows
- Serde + garde validation on scenario TOML — valid/invalid `#[case]` rows for garde range/cross-field rules; negative tests asserting rejection on P-ID absence, out-of-range SLO fields, malformed phase blocks (per §1 security-vector-coverage / property-test trigger type)
- Determinism unit test — same scenario+seed ⇒ same stream shape under `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` (per test-plan §1 property-test trigger + §2 agent-runnable determinism invariant)

## Anti-patterns to avoid
- No developer-seeded database — self-bootstrapping synthetic generation via `conductor-timeline` seeded fixtures only (test-plan §3 test-data bootstrap)
- No unvalidated scenario config deserialization — all TOML loads must pass garde validation before `PhaseTimeline` construction (test-plan §1 security-vector-coverage / property-test and Vector 2; anti-pattern: "Deserializing scenario config without garde validation at load")
- No multi-line JSONL in the emission journal — tracing JSON flattening serializes one line per event (test-plan §3 Log format agent-parsing note)
- No interactive `cargo insta review` in CI — insta runs in assert/CI mode only; snapshots committed ahead of runs (test-plan §2 agent-runnable invariants)

## Contract bindings
**5-command discipline** (tests §3) binds to the scenario `run` command (tests §3 / §1): `conductor run <P-ID>` / `conductor run <P-ID> --seed <s>` invokes the catalog; the P-ID ↔ TOML file matching is resolved at the `run` surface (no new inbound binding; the `:4317` port-occupier for P-003/P-004 reuses Epoch-4 existing fault primitive per scope.md).

**Scenario catalog surface** (tests §1 entity + §4 unit coverage) binds to the scenario deserializer (conductor-core `Scenario::from_toml_str`) and the `run` command surface (Epoch-8 CLI driver, deferred).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes unit tests for P-001..P-004 scenario TOML serialization + garde validation + determinism round-trip (fixture test mirroring `error-baseline-spike` precedent, scope.md definition-of-done).
- (tests) Negative tests asserting garde rejection of out-of-range config (error fraction ∉ [0,1], negative durations, p50 > p95, severity-mix sum violations) per test-plan §1 security-vector-coverage / property-test trigger.
- (tests) Property test over `#[garde(custom)]` cross-field invariants on connection-lifecycle phases + expected block (same scenario+seed ⇒ deterministic stream shape per test-plan §1 property-test trigger + §2 determinism invariant).
- (tests) Coverage: new-code line coverage ≥ threshold from test-plan §10 (conductor-core scenario/phase_spec/expected modules are pure library, covered by nextest).

## Relevant amendment history
- 2026-06-16-test-framework-fixtures-coverage-tooling — §4 tool-version policy: external-CLI tools (cargo-nextest 0.9.137, cargo-llvm-cov 0.8.7) are reference floors outside `Cargo.lock`; any green-running install satisfies the gate (routine per playbook rules #2/#5; no envelope change — directly relevant to this chunk's nextest gate).
- 2026-06-16-emission-journal-writer — §4 conductor-report golden: canonical line shape is locked via exact-string `assert_eq!` at unit level (matching `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern); insta stays the E2E mechanism (predates this chunk but anchors the model-extension depth for the new `expected` block wiring, scope follow-up d).