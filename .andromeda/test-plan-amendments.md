# Test Plan — Amendments

_Append-only changelog of amendments to `test-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — self-obs stream noted as distinct from the emission journal
**Section:** §3 Test Harness Contract / Log format
**Change:** added a note that the `tracing` self-observation stream (stderr / `logs/agent-latest.jsonl`; per-line base fields incl service-identity + `run_id`) is a SEPARATE artifact from the per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth + Run-report envelope) — the two schemas must not be conflated.
**Why:** the chunk's new flat `service.*` self-obs fields belong to the self-obs stream, not the emission-journal envelope §3 owns, so the bound §3 ↔ obs-plan §3 envelope is unchanged; the amendment is a clarifying cross-reference, not an envelope field addition.
**Ref:** .andromeda/runs/2026-06-15T17-46-44-wrap/

## 2026-06-15-design-token-typography-bundle — frontend (ui/) tests build-gated, unit deferred to Epoch 9
**Section:** §4 Unit Test Strategy (What unit tests cover)
**Change:** added a `conductor-tauri/ui` bullet — the React/Tailwind token bundle carries no Rust/nextest unit tests; it is build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke), and frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9.
**Why:** the new frontend code paths had no unit tests; a Foundation chunk defers a downstream-sequenced concern, and §12 Decisions Log already records no JS/TS unit runner adopted (GUI convenience-only). No envelope/harness change.
**Ref:** .andromeda/runs/2026-06-15T22-05-00-wrap/

## 2026-06-16-test-framework-fixtures-coverage-tooling — external-CLI tool versions reframed as floors
**Section:** §4 Unit Test Strategy
**Change:** added a Tool-version policy paragraph — the external-CLI tools (cargo-nextest, cargo-llvm-cov) named in §4 are reference floors (outside `Cargo.lock`; any green-running install satisfies the gate, per the cargo-audit/deny precedent); the crate dev-deps are caret-resolved with `Cargo.lock` authoritative.
**Why:** the chunk resolved tool and dev-dep versions differing from §4's named pins while every gate ran green; dev-CLI tools are floors and a spec illustration aligns to the sound implementation, so reframing to floors stops the drift recurring.
**Ref:** .andromeda/runs/2026-06-16T16-46-23-wrap/

## 2026-06-16-emission-journal-writer — unit serialization goldens use exact-assert; insta stays the E2E mechanism
**Section:** §4 Unit Test Strategy (conductor-report bullet)
**Change:** clarified the conductor-report golden — the canonical line shape is locked via exact-string `assert_eq!` at unit level (matching the `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern), with insta reserved as the E2E journal-golden mechanism (`run_id`/timestamp redaction, §6/§7).
**Why:** the chunk's unit goldens used exact-string `assert_eq!` (dropping insta from `conductor-report` dev-deps), matching the established canonical-serialization golden pattern in `conductor-core`; insta remains the E2E mechanism (real runs, redaction) and stays in the workspace dev-deps. A spec illustration aligns to the sound implementation.
**Ref:** .andromeda/runs/2026-06-16T21-43-46-wrap/

## 2026-06-17-raw-otlp-message-scaffold — OTLP-egress loopback gRPC stub (tokio-stream) registered in the integration mechanisms
**Section:** §2 Test Strategy (the test-pyramid Integration row)
**Change:** added the OTLP-egress loopback gRPC `TraceService` stub — a tonic server over `tokio-stream::wrappers::TcpListenerStream` on an ephemeral `127.0.0.1:0` (NEW dev-dep `tokio-stream`) — to the Integration row's mechanisms (alongside the rmcp stub + Tauri mock + in-memory rusqlite). Never binds the real `:4317` (reserved for the Epoch-4 port-occupier).
**Why:** `tokio-stream` was a new dev-dep absent from any spec inventory; its home is the test-plan dev-test stack (where rstest/assert_cmd live), not arch §Stack, which does not enumerate dev utilities. No harness/envelope change.
**Ref:** .andromeda/runs/2026-06-17T23-06-06-wrap/

## 2026-06-24-frameless-window-shell — registered the Tauri-backend self-obs sink logs/conductor-tauri.jsonl
**Section:** §3 Test Harness Contract (Self-obs stream is a distinct artifact)
**Change:** added the Tauri backend's `logs/conductor-tauri.jsonl` to the self-obs stream sink list (was cli `logs/agent-latest.jsonl` only), matching obs-plan §3's dual-surface sink table now that the Tauri sink is live.
**Why:** the chunk made the Tauri backend self-obs sink live; obs-plan §3 already documented both sinks but test-plan §3, the log-schema owner, listed only the cli sink. A §3↔§3 reconcile of the now-live half — distinct from the still-carried dual-RECORD-SHAPE reconcile. No schema change (same per-line base fields; only the sink list grew).
**Ref:** NOT DERIVED

## 2026-06-26-live-counter-channel-stream — deferred the Tauri-integration + GUI-parity tests to the GUI test-harness chunk
**Section:** §5 Integration Test Strategy (Read-back / boundary coverage list)
**Change:** noted that the `tauri::test` mock-runtime command/Channel assertion + the GUI leg of cross-surface parity defer to the Epoch-9 `tauri-driver` GUI test-harness chunk; the run logic is covered now at the conductor-run unit tier (drive_run stream+persist+abort + seed-stable Blocked envelope) + the conductor-cli cli_smoke parity E2E (the headless leg of Critical Path 7).
**Why:** escalated and user-confirmed. The GUI start_run drives the run on a background thread streaming the Channel asynchronously, so an in-process frame-sequence assertion is thread-timing-dependent and would flake against the zero-retry bar; the run logic is covered deterministically at the unit tier + the CLI parity E2E (byte-identical Blocked envelope). The GUI integration leg lands with the sequenced tauri-driver harness chunk; a playbook rule was added to pre-empt re-firing on later chunks. RunRecord envelope unchanged — RunEvent is transient IPC, not in the JSONL/obs schema.
**Ref:** .andromeda/runs/2026-06-26T22-49-24-wrap/

## 2026-08-08-sut-capability-manifest — Capability-manifest coverage + de-hardcoded catalog assertions
**Section:** §1 Test Scope (Scenario catalog entity) · §4 Unit Test Strategy · §6 coverage-matrix completeness scenario · §7 fixtures
**Change:** catalog/coverage assertions re-sourced from the SUT capability manifest instead of a hard-coded 60 (was 60; now the manifest's accepted set); capability-manifest loader/validator + membership-rejection coverage recorded (8 new unit tests, workspace nextest 420 → 428).
**Why:** the plan mandated a static assertion over an accepted set the code no longer defines.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-09-current-sut-coverage-classification — Critical Path 6 de-hardcoded; stale arch citation repaired
**Section:** §1 Test Scope Summary → Critical paths (Path 6, coverage-matrix completeness gate) · §1 Testable Entities (Scenario catalog, Source clause)
**Change:** Path 6's verification signal was "the generated `coverage-matrix.md` enumerates all 60 P-IDs"; now a manifest-relative assertion (set-equality against the accepted set, no literal count or P-ID range), and its mode list widens from three to four with `not-Conductor's`. The Scenario-catalog entity's **Source:** clause, which quoted arch Project Intent as "60 claimed capabilities P-001..P-060", now quotes arch's current wording (the old citation stopped resolving once arch was de-hardcoded on 2026-08-08). The adjacent Creator-Brief quotation stays verbatim — it cites frozen `input.md`, where that wording is still what the brief says.
**Why:** the chunk added a fourth `CoverageMode` and replaced every literal-60 assertion with a manifest-derived one, so the plan's mandated gate no longer described the code path it governs. The Source-clause repair is the cross-master citation edge of the cascade (a master citing another master's superseded wording), found by grepping the other masters for each amended passage's OLD wording rather than from the proposal set.
**Kept:** Path 6's reference to the `coverage-matrix.md` artifact — that file has never existed in the repo though arch §Occupied Resources registers it; whether it becomes real belongs to the Epoch-6 coverage-completeness-gate chunk (routed as a CARRY on that entry, per operator direction).
**Ref:** .andromeda/runs/2026-08-09T14-17-38-wrap/

## 2026-08-09-interpretation-correctness-posture — mock-runtime command tier live; coverage gate gains a scenario-backing leg
**Section:** §5 (GUI test-harness deferral) · §6 Scenario: Coverage-matrix completeness gate
**Change:** §5's deferral narrowed to the `Channel`-frame-sequence + GUI-parity leg — the mock-runtime command-dispatch half landed with 2026-06-27-desktop-a11y-harness-setup and is now the standing tier for each read-only command (`coverage_matrix`, `run_report`, `unbacked_auto`). §6's coverage gate gains the second-axis scenario-backing leg (exact-set equality in both directions over `UNBACKED_AUTO`; `Auto` only; unit-tier, CI-runnable).
**Why:** the chunk added a mock-runtime IPC test for `unbacked_auto`, which §5 still described as deferred, and shipped the scenario-backing gate §6 did not describe. Self-raised — the plan's Expected-amendments list is the coverage floor, not the ceiling.
**Ref:** .andromeda/runs/2026-08-09T19-30-00-wrap/

## 2026-08-09-in-lane-sut-scenarios — selector strategy: de-hardcode the P-ID range
**Section:** §6 E2E / Selector strategy
**Change:** the coverage-matrix/report row selector anchor was the literal range "(`P-001`..`P-060`)"; now "mono P-ID tokens (the manifest's accepted set)".
**Why:** this chunk lands the first catalog entries above P-060 (P-067, P-072, P-079), so a selector keyed on `P-001..P-060` no longer covers the rows it creates. The SET is named rather than a new literal range (`P-001..P-082`), per the de-hardcode precedent of 2026-08-08-sut-capability-manifest — a new literal re-stales on the next SUT release. No detector covered the class, which motivated the two `D-*-derived-count` detectors added to drift-base the same chunk.
**Ref:** .andromeda/runs/2026-08-09T20-30-00-wrap/

## 2026-08-09-sut-load-envelope — cli status-label enumerations name the set + the non-lamp qualifier
**Section:** §1 Surfaces under test (cli Signal) · §6 E2E cli driver row + Selector strategy
**Change:** all three cli status-label enumerations now present the six bracket labels as the closed per-P-ID lamp/report-state set and record `[ENVIRONMENT-SUSPECT]` as a run-level non-lamp qualifier cli selectors must expect.
**Why:** the chunk added a seventh bracket label to cli stdout while the three enumerations read as exhaustive; a selector strategy assuming only six would mis-parse an over-envelope run.
**Ref:** .andromeda/runs/2026-08-10T15-43-07-wrap/

## 2026-08-10-pulse-run-contract — `boot` timeout restated under the run contract's floor
**Section:** §3 Test Harness Contract → 5-command implementation → `boot`
**Change:** the flat `Timeout: 30s` became `CONDUCTOR_PREFLIGHT_TIMEOUT` seconds (default 30) raised to the run contract's effective floor (`contracts/pulse-run-contract.toml` `[incident_formation].min_canary_poll_seconds`, which must outlast Pulse's L3 digest cadence), so the live-Pulse leg must allow at least that floor. The stub leg still completes in <1s because it drives `CanaryPoll::immediate()` — no real clock.
**Why:** the chunk made the floor real in `canary_poll()`, so a flat 30s budget described a value the code will not honour for the live leg. One-sided by construction: obs-plan §3 carries no preflight budget, and the shared envelope/log-format sides are untouched (envelope still 11 fields, `ReportState` five, `LAMP_META` six).
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/

## 2026-08-10-scenario-run-root-span-tree — the self-obs stream's two line variants recorded
**Section:** §3 Test Harness Contract → Log format → "Self-obs stream is a distinct artifact"
**Change:** the bullet now states the stream carries TWO line variants over its one base set — the event line, and the span-lifecycle line adding `span` (the bounded §4 span name), `span_event` (`new` | `close`), an optional `parent`, and the span's own allowlisted attributes on the `new` line — and that a span attribute is gated by the same `conductor-core::redact` allowlist as any event field. The Run-report envelope and `runs.db` columns are unchanged.
**Why:** this plan is the source of truth for the JSONL log format (obs-plan §3 derives), and the chunk changed that format: `JsonObsLayer` gained `on_new_span` / `on_close`, so spans now materialize as lines where previously the layer implemented only `on_event` and no span emitted anything. Because the log format itself changed, the pre-existing-bind dismiss rule does not apply. obs-plan §3 amended in lockstep.
**Ref:** .andromeda/runs/2026-08-11T15-45-25-wrap/
## 2026-08-11-faithful-emission-dispatcher — scenario-config validation entity + trigger restated
**Section:** §1 Entity (scenario-config validation surface) · §1 Trigger (security-vector-coverage / property-test)
**Change:** both now name the shipped error-fraction encoding (`error_percent` ∈ 0..=100) — was an f64 `[0,1]` bound — add bounded per-phase occurrences and the per-shape emission rules to the validated set, and require the negative-test suite to catch a nested spec field annotated `skip` rather than `dive` (rules that exist but never execute).
**Why:** cross-master citation fold from the architecture amendment, plus the chunk's finding that `PhaseSpec.emission` was `#[garde(skip)]`, which no existing negative test could have caught.
**Ref:** .andromeda/runs/2026-08-13T16-43-31-wrap/

## 2026-08-13-dispatcher-determinism-goldens — golden inventory gains the seeded stream families
**Section:** §7 Test Data & Fixtures — Seed strategies, the `Golden artifacts` row
**Change:** the row now names two families: the envelope/journal goldens (unchanged, redacting `run_id`/`journal_emitted_at`/`read_back_observed_at`) and the seeded STREAM goldens committed per-crate under `<crate>/tests/snapshots/`, one file per test-file family × seed (`replay__*` · `pacing__*` · `dispatch_wire__*`). Stream goldens have no wall-clock field to redact — the dispatch-tier projection EXCLUDES every `*_time_unix_nano` rather than masking it — and the never-`cargo insta review` rule is explicit in the row.
**Why:** the chunk committed 4 new stream goldens in two new families (`pacing__fixture_emission_stream_seed_{4317017,7}`, `dispatch_wire__storm_stream_seed_{4317017,7}`) while the row enumerated only envelope/journal goldens. Stated as the family SET plus the per-family × seed rule rather than a file count, so it does not re-stale at the next seed or tier.
**Ref:** .andromeda/runs/2026-08-13T18-25-00-wrap/

## 2026-08-13-first-live-green-preflight — boot Timeout: two derived budgets
**Section:** §3 Test Harness Contract — 5-command implementation → `boot` (Timeout)
**Change:** Timeout now names TWO budgets, both derived: the in-process canary poll (`CONDUCTOR_PREFLIGHT_TIMEOUT` raised to `[incident_formation].min_canary_poll_seconds`, unchanged) and the wall-clock wrapper `agent-run.{sh,ps1}` `boot` applies, which both shells derive per invocation as `warmup_ms/1000 + poll + margin`. A missing contract term is a hard exit 2; a lowered env value clamps up to the floor. Stated as the derivation rule, no literal.
**Why:** the wrapper was a fixed 30s in `.sh` and absent in `.ps1`, while the in-process budget is ~135s (45s warm-up + >=90s poll) — the wrapper killed every live run before the warm-up finished, and the two shells disagreed.
**Ref:** .andromeda/runs/2026-08-13T22-48-53-wrap/

## 2026-08-14-canary-fingerprint-feed-capture — the event-line variant named by set, not by level literals
**Section:** §3 Test Harness Contract → Log format ("Self-obs stream is a distinct artifact")
**Change:** the event-line variant is now any `tracing` event record (`message` plus its allowlisted fields) at whatever level obs-plan §11's policy assigns that call site; was "an `info!`/`error!` record".
**Why:** the chunk shipped the first `debug`-level self-obs line (the `emit.batch` wire-shape witness, at `debug` because §11 bans `info` on a hot path), which the old two-level enumeration excluded — a one-sided test-plan §3 ↔ obs-plan §3 divergence, since obs-plan §3 never enumerated levels. Naming the set rather than a fresh literal keeps it from re-staling.
**Ref:** .andromeda/runs/2026-08-14T16-51-43-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — canary boundary re-aimed to incident freshness
**Section:** §5 Integration Test Strategy (Cross-module patterns covered)
**Change:** splits the old combined bullet: `query_incident_list` now carries the canary round-trip (an incident opened after the emission stamp ⇒ proceed; empty corpus, only-older incidents, or a missing stamp ⇒ `blocked`), covered by the fresh / stale / stamp-absent stub legs; `retrieve_telemetry_slice` / `retrieve_report` move to a separate per-check read-back bullet, which the canary no longer calls.
**Why:** the canary's tool and predicate both changed. The split is deliberate — the tools are still called by per-check extraction, so removing them entirely would have introduced new drift rather than removing it.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — ipc-internal Signal predicate updated
**Section:** §1 Test Scope Summary (Surfaces under test → ipc-internal)
**Change:** the Signal clause was "canary round-trip non-empty"; now "canary round-trip observing an incident opened after the emission stamp".
**Why:** the same retired predicate restated in the surfaces table; non-emptiness is no longer sufficient, since a corpus of only older incidents must block.
**Kept:** the pinned required-tool list.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — preflight state mapping widened
**Section:** §4 Unit Test Strategy (What unit tests cover → conductor-verify)
**Change:** was "empty canary ⇒ `blocked`"; now "an empty corpus OR a stale one — no incident newer than the canary's emission stamp — ⇒ `blocked`".
**Why:** a non-empty but wholly pre-dating corpus now blocks too; the old mapping under-described the assertion the stub legs make.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-fingerprint-storm-live-proof — fingerprint-storm verification signal re-based on the harvest
**Section:** §6 E2E Test Strategy (Scenario: Fingerprint-storm) + §1 Test Scope Summary (Critical paths, the
same restatement)
**Change:** the envelope's `fingerprints` field is PRESENT but expected `[]` under deterministic L4 (fed solely from `fingerprint_refs`, which the L4 fixture pins empty); the scenario's read-back token checks are DECLARE-ONLY because `retrieve_report` is permanently `degraded_mode` in this mode; the live proof is the test-only harvest of Pulse's `triage.pattern.storm.detected` line asserted at the `conductor-run` unit tier.
**Why:** measured on two live legs — both returned `fingerprints: []`, `Contains "RetryStorm"` always failed and `Absent "RetryStorm"` passed VACUOUSLY against text that cannot carry the token. Both sections restated the identical claim, so a §6-only apply would have left it standing in the critical-paths table.
**Ref:** .andromeda/runs/2026-08-16T14-06-03-wrap/

## 2026-08-18-error-baseline-spike-live-proof — §6/§1 error-baseline-spike signal re-based; fingerprints [] pin retired
**Section:** §6 Scenario: error-baseline-spike + §1 Critical Path 1 · §6 Scenario: Fingerprint-storm + §1 Critical Path 2
**Change:** the error-baseline-spike verification signal was exit-0/`[PASS]`/`verdict=Pass`/`state=Pass`; now the declare-only reality: `[RESIDUAL]`, `verdict` null / `state=KnownResidual`, live claim graded at the harvest tier (`triage.cue.emit` via `baseline_harvest.rs`) — both sites. The `fingerprints` expected-`[]` pin is retired at both sites: the field is SUT-populated under deterministic L4 (constant `det-*` triple, payload-invariant, at SUT HEAD `efabe8e`); the harvest, never the array, stays the proof.
**Why:** measured live (row KnownResidual; envelope carried the 3 `det-*` refs); the family's checks retired because no read-back surface can carry them (the degraded report renders fixture constants; the evidence count reads an unpopulated `span_ids`).
**Ref:** .andromeda/runs/2026-08-18T19-10-05-wrap/

## 2026-08-18-restart-suppression-live-proof — restart-suppression re-based to the harvest tier; harness seed precedence
**Section:** §1 Critical paths (Path 3) · §6 Scenario: Restart-suppression incl. one bypass case · §3 Test Harness Contract (`run` scenario invocation)
**Change:** Path 3 and its §6 twin move from an in-scenario hard pass/fail to the declare-only row (`verdict` null / `state=KnownResidual`, `<90s`) with the suppression/bypass claim graded as hard predicates at the harvest tier (`restart_harvest.rs`, verbatim leg captures; bypass set = absolute arm + suppressed case, relative arm recorded unreachable; `persistence_seconds` = cumulative samples; tick counters redacted live). §3's scenario invocation now states `--seed` rides only an explicitly set `SEED` (the TOML-declared seed governs otherwise, `.sh`/`.ps1` parity); the harness previously fabricated `--seed 424242` over TOML-declared seeds.
**Why:** both `[[expected]]` checks were structurally ungradeable under deterministic L4 and retired; the live leg proved every witness at the harvest tier. Both sites were applied together per the two-site rule.
**Ref:** .andromeda/runs/2026-08-18T21-55-43-wrap/

## 2026-08-19-pii-scrub-live-proof — pii-scrub re-based to declare-only at both sites
**Section:** §6 Headless deterministic scenario run (Cleanup parenthetical) + §1 Critical paths closing note (the two-site rule)
**Change:** both sites now split the parenthetical — connection-lifecycle stays an auto-scenario instance of Path 1; pii-scrub is DECLARE-ONLY (all five `[[expected]]` read-back checks retired, `[[expected]]` 5 → 0, measurement in the scenario TOML header; row lands verdict null / state KnownResidual, non-Blocked) with the live scrub claim graded at the harvest tier in `crates/conductor-run/tests/pii_harvest.rs` (the storm/baseline/restart_harvest precedent).
**Why:** a single-site apply would leave the twin asserting the retired read-back-graded shape (the two-site rule established at the error-baseline re-base).
**Kept:** the two other pii-scrub mentions (the run-verb list and the scenario-invocation list) name it only as an invocable scenario and stay true.
**Ref:** .andromeda/runs/2026-08-19T21-00-45-wrap/

## 2026-08-19-connection-lifecycle-live-proof — connection-lifecycle re-based auto-scenario → DECLARE-ONLY (two sites)
**Section:** §1 Test Scope Summary (critical-paths closing note) · §6 E2E Scenario 1 (Cleanup parenthetical)
**Change:** both sites now state the connection family as DECLARE-ONLY instances of Path 1's pattern (was an auto-scenario) — zero read-back-graded checks, rows landing `verdict` null / `state=KnownResidual` non-Blocked — with the live walk/conflict claims graded at the harvest tier in `crates/conductor-run/tests/connection_harvest.rs` (the fifth harvest).
**Why:** measured live: connection state reaches no MCP read-back surface, so the family's four Contains checks retired. The two-site rule applied in one pass (the error-baseline precedent).
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/

## 2026-08-20-latency-regression-re-proof — sidecar-spawn citation re-based
**Section:** §Test Anti-Patterns (stack-specific)
**Change:** the cross-master citation of security-plan's sidecar-spawn ban was "a fixed hard-coded path"; now "a fixed hard-coded program NAME resolved through the inherited `PATH`".
**Why:** cascade edge — test-plan cited security-plan as saying something it no longer says after the 2026-08-20 spawn-wording amendment (the shipped constant is a program NAME, PATH-resolved). The negative-test mandate itself is unchanged.
**Ref:** .andromeda/runs/2026-08-20T17-10-10-wrap/

## 2026-08-20-verifier-self-hardening — cargo-mutants registered + runner portability made a gate
**Section:** §4 Unit Test Strategy (Framework · Mutation instrument · Tool-version policy)
**Change:** registered cargo-mutants 27.1.0 as an operator/local mutation instrument with its run discipline (workspace-root `-f` paths, mandatory `--test-tool=nextest`, `Found 0 mutants to test` is a NO-OP never a pass, gitignored output) and added it to the floors-not-pins list. Also named `cargo test -p <crate>` a standing runner-portability gate beside nextest.
**Why:** the chunk used the instrument as its acceptance evidence and measured that the package-relative `-f` form reports zero mutants at exit 0 — indistinguishable from a clean run. Runner portability became a gate because only the shared-process runner exposes a test relying on nextest's per-test process for isolation.
**Ref:** .andromeda/runs/2026-08-20T21-45-00-wrap/

## 2026-08-20-verifier-self-hardening — scoped mutation recorded as an operator instrument
**Section:** §9 CI Integration (after Live-Pulse scenarios)
**Change:** recorded the scoped mutation audit on the same footing as the live-Pulse leg — per-chunk, against the touched crates, never a blocking CI stage — and stated that the stage table is CI's complete inventory, not a chunk's.
**Why:** the chunk ran the instrument for acceptance while §9's stage table registers only Lint / Supply-chain / Unit / Doctest / Integration / E2E / Coverage / Quality, so the table could be read as the full set of checks a chunk owes.
**Ref:** .andromeda/runs/2026-08-20T21-45-00-wrap/

## 2026-08-20-verifier-self-hardening — mutation-survivor disposition + runner-dependence as flakiness
**Section:** §10 Quality Gates & Coverage Targets (Zero-flakiness budget)
**Change:** added mutation-survivor disposition as a non-blocking audit-tier rule — every named survivor ends killed OR classified accepted-deliberate against a cited rule — explicitly not a numeric threshold; and recorded that a runner-dependent result is a determinism break to fix at the cause.
**Why:** the chunk dispositioned 22 named survivors as 16 killed + 6 accepted-deliberate (the `declares` edge, per testing.md 2026-08-10), and the score moved only as a consequence.
**Ref:** .andromeda/runs/2026-08-20T21-45-00-wrap/

## 2026-08-20-verifier-self-hardening — process-global singleton isolation ban
**Section:** §11 Test Anti-Patterns → Integration
**Change:** new ban — never rely on nextest's per-test process to isolate a test from a process-global first-install-wins singleton (`init_observability`); give it its own test binary rather than serializing the file.
**Why:** the wire-shape witness passed alone and under `--test-threads=1`, failing only alongside its four `emit_canary_storm` siblings — concurrent interference against the global subscriber, which a per-test temp FILE cannot isolate. Serializing was rejected as hiding the defect (§10).
**Ref:** .andromeda/runs/2026-08-20T21-45-00-wrap/

## 2026-08-20-verifier-self-hardening — decisions-log entry
**Section:** §12 Test Decisions Log
**Change:** added the 2026-08-20 entry recording the instrument adoption, the measured run-discipline findings, the `declares` accepted-deliberate classification, and the runner-portability gate.
**Why:** the tool's adoption and the classification ruling are decisions future chunks must not re-litigate.
**Ref:** .andromeda/runs/2026-08-20T21-45-00-wrap/

## 2026-08-20-read-back-seam-survivors-closed — cargo-mutants exit code carries no verdict
**Section:** §4 Unit Test Strategy → Mutation instrument
**Change:** recorded the second half of cargo-mutants' exit-code semantics — the exit code carries no verdict in EITHER direction: `Found 0 mutants to test` is a no-op at exit 0 (already recorded), and a NON-zero exit reflects surviving/timeout CLASSES rather than run failure. Gate on the tallies read out of `mutants.out/` (`missed.txt` empty + the named survivors present in `caught.txt`), never on the exit code.
**Why:** a scoped run exited 3 while fully meeting its acceptance (0 missed, all named survivors killed), the non-zero owed entirely to two pre-existing timeouts. §4 previously recorded only the exit-0 direction, implying by omission that a non-zero exit IS a failure; a reader following §4 alone would have inverted the verdict.
**Ref:** .andromeda/runs/2026-08-20T23-05-37-wrap/

## 2026-08-20-read-back-seam-survivors-closed — decisions-log exit-code restatement
**Section:** §12 Test Decisions Log → `2026-08-20` Mutation instrument adopted, Run-discipline bullet
**Change:** extended the run-discipline bullet's exit-code parenthetical to carry both directions, naming the measured exit-3 case and its chunk.
**Why:** §12 restates §4's run discipline and carried only the exit-0 half, so a single-site apply to §4 would have left the one-sided claim standing in the decisions log.
**Ref:** .andromeda/runs/2026-08-20T23-05-37-wrap/
## 2026-08-21-severity-lifecycle-live-proof — severity-lifecycle re-based to declare-only + the harvest tier
**Section:** §6 E2E Test Strategy — Scenario: Severity-lifecycle full pass · §1 Test Scope Summary — Critical
Path 4 (the two-site rule) · §5 Integration Test Strategy — `mark_incident_resolved` bullet
**Change:** the family grades declare-only (five rows, `verdict` null / `state` "KnownResidual", exit 0) with the live auto-resolve claim asserted HARD at the harvest tier in `crates/conductor-run/tests/severity_harvest.rs` — on the instant Pulse's active set EMPTIES (`incidents.list_active.request` `item_count`) against the 120s window + 30s observer tick, plus the same-fingerprint retrigger reading `created=true, deduped=false`. The §6 Steps line no longer invokes `conductor run severity-lifecycle --seed <s>`: no scenario carries that name (the family is five separately named TOMLs) and `--seed` rides only an explicitly set `SEED` per §3. `mark_incident_resolved` no longer claims a severity-lifecycle resolution-summary read-back — it has no production call site.
**Why:** measured live at SUT HEAD `efabe8e`, two mandated witnesses do not work: `triage.incident.auto_resolve.tick` renders its counters as `"<redacted>"` (proving only that the observer ran), and P-059's resolution summary is unreachable under deterministic L4 (zero `DigestKind::ResolutionSummary` constructors; the canned fixture pins `is_resolution_summary` false).
**Ref:** .andromeda/runs/2026-08-21T09-50-00-wrap/

## 2026-08-21-per-check-latency-measurement — Journal carries two report-seam line shapes
**Section:** Section 3 Test Harness Contract -> Log format (Agent parsing)
**Change:** the journal is recorded as carrying TWO report-seam line shapes — the envelope line (unchanged) and the per-check `CheckRecord` line (9 keys) — and a typed parse must DISCRIMINATE rather than assert every line deserializes as `RunReportEnvelope`.
**Why:** the chunk added `CheckRecord` lines to `runs/<run_id>.jsonl`, pinned by `journal.rs::check_line_is_its_own_parseable_shape_beside_the_envelope`. test-plan Section 3 OWNS this format, so leaving it would make the change one-sided against obs-plan Section 3.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Status endpoint shape scoped to the envelope grain
**Section:** Section 3 Test Harness Contract -> Status endpoint shape
**Change:** was "each JSONL journal line"; now narrowed to the envelope line / `runs` row, with per-check detail named as a separate finer grain (`run_check`, read via `RunsDb::checks_for`).
**Why:** the envelope field count is explicitly unmoved while a new grain now shares the journal.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Agent-runnable invariants qualified
**Section:** Section 2 Test Strategy -> Agent-runnable invariants
**Change:** the machine-parseable-output bullet qualified so the envelope assertion targets the envelope line specifically.
**Why:** the same retired claim as the Section 3 primary: a `CheckRecord` line will not deserialize as `RunReportEnvelope`.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — E2E journal signal targets the envelope line
**Section:** Section 6 E2E -> Headless deterministic scenario run, Verification signal
**Change:** the journal assertion targets the envelope LINE, with any per-check lines parsing to the per-check shape.
**Why:** a file-wide parse-to-envelope assertion is now a false negative on any run that emits check rows.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Cleanup covers every table a run writes
**Section:** Section 3 Test Harness Contract -> cleanup (body + verification)
**Change:** teardown extended past `runs` to `DELETE FROM run_check` and `DELETE FROM run_envelope` (bound parameters), with matching count-zero verifications.
**Why:** the chunk added `run_check`; Section 3 named only `runs`, so cleanup was neither complete nor verifiable for a run that wrote check rows. `run_envelope` was already uncovered — a pre-existing gap the contract closes at the same time; the matching CODE fix in `scripts/agent-run.{sh,ps1}` is carried to its owner route entry.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-22-operator-pause-and-checklist-live-firing — the never-blocks property re-tiered from the cli E2E leg to the unit tier
**Section:** §1 Test Scope Summary → cli surface Notes · §6 E2E Test Strategy → drivers-per-surface, cli row
**Change:** the stdin-closed cli leg is now recorded as proving no-hang / exit-0 ONLY (was the never-blocks property). With no live Pulse the preflight blocks and `execute_scenario` returns on the Blocked spine before the hold, so the leg never reaches an interactive prompt; the never-blocks property is attributed to the unit tier (`headless_never_blocks_under_paused_clock`; `resolve_kind_agent_mode_overrides_an_attended_tty`). Applied at both sites, which stated the claim identically.
**Why:** measured false — a fresh self-obs log carried ZERO resolution witnesses and ended at a preflight block (MCP read-back path unreachable), so the leg's green proved something narrower than the doc claimed.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-08-31-p-075-assert-round — `mark_incident_resolved` gains a production caller and a live exercise
**Section:** §5 Integration Test Strategy → `mark_incident_resolved` bullet · §1 Critical Path 4 · §6 severity-lifecycle Surfaces line
**Change:** retired "NO production call site and no live exercise" — both halves false. §5 now names three tiers: applied + declined arms stub-proven at the integration tier (`conductor-verify/tests/readback.rs`), the declined arm stub-ONLY and permanently so (Pulse's `DeclinedStale` is a monotonic-timestamp guard its own dispatch cannot trip), and the live write graded at the harvest tier on the pinned capture (`lifecycle_harvest.rs`) driven by the feature-gated operator leg. Verdict PROVEN-BY-LIVENESS — runtime-STATE fidelity, never payload. §1 Path 4 and §6's Surfaces line move with it (two-site rule): the caller exists but is exercised out-of-band, never inside `execute_scenario`, so neither path's grading changes.
**Why:** the chunk added the production caller and a live leg exercised the write against Pulse HEAD `83d4060`.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-08-31-p-075-assert-round — Stub item-key fidelity: `incident_id`, not `id`
**Section:** §5 Integration Test Strategy → `query_incident_list` bullet
**Change:** recorded as a mocking-discipline requirement: the stub's incident item key MUST be `incident_id`, the key the live sidecar emits (the shared stub emitted `id`). Pinned by `lifecycle_harvest::the_live_item_key_is_incident_id`; readers accept either key.
**Why:** a reader keyed on `id` extracted an empty active set from a POPULATED live corpus — the silently-empty class this bullet already bans, arriving through the mock rather than the data dir, green against every stub leg on the way to failing live. It cost four live legs; only dumping the raw wire value separated "Pulse never exposed it" from "our reader extracted nothing".
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-08-31-p-075-assert-round — A cargo-feature gate is a third sanctioned live-leg path
**Section:** §9 CI Integration → Live-Pulse scenarios · §11 Test Anti-Patterns → CI (stack-specific live-Pulse ban)
**Change:** §9's invocation inventory now admits a cargo-feature-gated test file invoked directly — `conductor-run/tests/lifecycle_live.rs` behind `[features] live-pulse = []`, an EMPTY feature adding zero package nodes and leaving `Cargo.lock` byte-unchanged, which keeps the leg out of default `nextest`/`clippy`/release and out of both runner-portability runs. Because a gated file is invisible to the default lint pass, such a leg OWES its own `cargo clippy -p <crate> --features <feat> --all-targets -- -D warnings`. §11's ban is unchanged in force; its parenthetical widens past "workflow_dispatch only" so the enumeration cannot contradict §9.
**Why:** the chunk's live leg used this path with the extra clippy gate; the `stub-server`-gated `preflight_spawn.rs` precedent existed in code but appeared nowhere in test-plan, so the convention was undocumented.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — the webview E2E platform verdict, measured
**Section:** §1 Surfaces under test (:54) · §1 Coverage triggers (:85) · §2 Agent-runnable invariants (:121, :122) · §3 CI stage selectors (:152) · §3 Bootstrap 5-command-discipline-wire (:207) · §4 conductor-tauri/ui (:244) · §5 GUI-leg deferral (:287) · §6 Drivers per surface (:304) · §6 Both-surface parity (:369) · §9 pipeline table (:456) · §9 Matrix builds (:465) · §11 Universal bans (:575, :577) · §12 Decisions Log (:594)
**Change:** the "Linux + `xvfb` only" verdict is retired as a CAPABILITY claim across every site, replaced by the measured platform SET — Linux CI headless under `xvfb`, plus the Windows dev host headful via WebView2 + an operator-supplied `msedgedriver`; macOS alone stays driver-less, which was always the justification's true content. The CI ARRANGEMENT is unchanged (still an `ubuntu-latest` + `xvfb` job; no CI edit). `--e2e` now names its measured preconditions (ensure-frontend → release build `--features tauri/custom-protocol` → `wdio run`); "headless only" becomes "non-interactive only" in both bans; the loopback allowlist admits the harness-lifetime WebDriver pair; the §1 coverage trigger splits by who READS the handle; §4 records that the build gate does not prove ESM loadability; §12's dated decision keeps its text with a dated correction appended.
**Why:** the chunk drove a real WebView2 session on Windows, which falsifies the platform verdict but not the driver choice or the macOS clause. More sites than the plan expected carried the claim (the §5 and §6 GUI-leg deferrals also pointed at "Linux+xvfb"), which is why the fix names the set rather than a fresh literal. §4's ESM clause is the sharper finding: `typecheck:e2e` passed for two months over a `wdio.conf.ts` that could not load on any host.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-desktop-a11y-sweep — webview E2E registered as two arms; the driven arm's firing form recorded
**Section:** §2 Agent-runnable invariants (determinism) · §3 CI stage selectors · §4 conductor-tauri/ui · §6 Drivers per surface (desktop-webview) · §9 Live-Pulse scenarios · §11 (CI ban, Universal real-network ban)
**Change:**
- §6 records TWO arms over one stack and the driven arm's FULL FIRING FORM as part of the leg — a `PATH` prefix resolving `andromeda-pulse-mcp`, `ANDROMEDA_PULSE_MCP_ENABLED` + `_L4_DETERMINISTIC`, `ANDROMEDA_PULSE_DATA_DIR` **equal to the live Pulse's data dir**, and a QUIET WINDOW of ≥120s idle + a 30s resolver tick after any preflight canary.
- §9 admits an npm-script wdio suite as a fourth sanctioned live-leg invocation path; §11's ban parenthetical follows.
- §3 qualifies `--e2e` as the routine suite only, with the `cwd: repoRoot` spawn and the 60s → 15min mocha ceiling.
- §2/§11 name both arms in the loopback allowlist and record the driven arm as a real-clock live-Pulse leg carried only as the operator-local-gate exception.
- §4's ESM-loadability clause widens from `--e2e` to whichever wdio leg loads the member.
**Why:** each precondition has a failure mode that misreads as something else — a bare invocation returns BLOCKED in ~2ms, indistinguishable from a real gate failure (PATH); a default-dir sidecar reads a corpus Pulse never writes, giving `result_count: 0` forever (data dir); and `conductor preflight` fires its own canary, so Pulse's per-`(kind, scope, scope_id)` dedupe means a second canary inside the window forms no fresh incident (quiet window). The ban on running any live leg as a CI gate is unchanged in force.
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — the `--e2e` leg seeds its own subject
**Section:** 3 Test Harness Contract (CI stage selectors) + 5 Integration Test Strategy + 6 E2E drivers-per-surface (desktop-webview) + 7 Test Data and Fixtures + 9 CI Integration (E2E row)
**Change:** `--e2e` now copies the committed `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` into a gitignored `runs/e2e-fixture/` and spawns tauri-driver with `CONDUCTOR_RUNS_DIR` pointing there (section 3, restated in the section 9 pipeline row). Section 6's ROUTINE arm was "subject-absent specs context-skip"; now fixture-seeded — it asserts the verdict lamps for real, and only the load-envelope banner still context-skips, that path carried at the unit + mock-runtime tiers. Section 7 names the committed-journal fixture family in the sanctioned provenance set and requires such a fixture's MEANING be pinned by a production-reader round-trip. Section 5's read-only-command tier now names the source SET rather than `conductor-core` alone.
**Why:** `/runs/` is gitignored, so on a clean tree the coverage rows had no run record to render and the populated-lamp assertion had no subject — on a dev host it would have passed off local run residue instead. Section 5's `conductor-core` qualifier was false outright: `run_envelope` single-sources `conductor-report`'s `RunsDb::get_envelope` via `conductor_run::read_envelope`.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — the `sr*` screen-reader leg registered; browse-mode reading flagged; the cross-major driver pair recorded
**Section:** 1 Test Scope Summary (Untestable zones · Vector-1 coverage trigger) + 2 Test Strategy (Deterministic invariant) + 4 Unit Test Strategy (`conductor-tauri/ui` prover set) + 6 E2E drivers-per-surface (desktop-webview: Mode cell + Notes) + 11 Test Anti-Patterns (network ban)
**Change:**
- §6 registers the third suite family, the operator-local `sr` / `sr-empty` / `sr-error` screen-reader leg. Firing form: `CONDUCTOR_NVDA`; NVDA started before the driver, ready on `NVDA initialized`; the leg-owned `nvda.ini`; `activate-window.ps1`; per-suite runs / scenarios dirs; `subject.txt`; the `sr` subject in the live-Pulse form. Stop form: `nvda -q`, census, `tauriDriver.kill()`. Also the driven arm's own `runs/driven/runs` and the routine arm's clean re-seed. "TWO arms" became "THREE suite families" (§6; §11 "ANY webview arm"; §2's carve-out names the operator-local set, driven + `sr*`, both wall-clock).
- Retired: §6's "msedgedriver matching the host WebView2 Runtime major" requirement. msedgedriver 151.0.4129.101 drove WebView2 152.0.4191.53 green: unsupported by the driver's own line, working by measurement. Refreshing it is the operator's host task before the next leg.
- §1 Untestable zones gains "screen-reader BROWSE-MODE reading (by agent, today)". This is a missing key path, not a missing driver; OS-level key injection is the route-owned CARRY.
- §1's Vector-1 trigger and §4's prover set name both host-tool handles and all three firing paths.
**Why:** the chunk's leg measured it (3/3 sessions, the skip arm with the handle unset, across a mid-day WebView2 update). The operator's wrap directive set the registration wording and the driver-pair record. The browse-mode zone is the operator's review ruling: such rows are findings, never passes, and never a manual arm.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-02-cross-surface-envelope-parity — the rmcp payload retired, the parity deferral discharged, and the seeder re-stated
**Section:** §1 (Entity · Surfaces · Coverage triggers) · §2 (pyramid) · §3 (`boot` · CI stage selectors) · §4 (unit coverage · external services) · §5 (Driver(s) · boundary table · lifecycle · cross-module patterns · the GUI deferral) · §6 (driver table · scenario steps · routine-arm paragraph) · §7 (provenance set · meaning-under-test) · §8 (mocking) · §9 (pipeline) · §10 (coverage exclusions) · §11 (anti-patterns) · §12 (decisions log)
**Change:**
- Every site that described the CURRENT read-back client or test stub as rmcp now names the shipped hand-rolled line-delimited JSON-RPC client (`jsonrpc.rs` + `client.rs`) and the hand-rolled `stub_pulse_mcp`. The rmcp token count fell 24 → 5, and each survivor is deliberate: two name the rmcp STDIO injection vulnerability CLASS; one keeps the quoted anti-pattern TITLE (its mechanism clause amended); one reads "rmcp removed 2026-06-27" as a historical note; the decisions-log entry is corrected in place with a dated 2026-09-02 bracket.
- Retired: the "negotiates DOWN to `2024-11-05`" mechanism, at both sites that stated it.
- §5's GUI-parity deferral narrowed. The mock-runtime ↔ CLI half is DISCHARGED by an equal-envelope test that drives both surfaces into one `runs.db` under a non-default `CONDUCTOR_RUNS_DIR` and compares the two persisted envelopes, not each against a literal. The `Channel`-frame-sequence and webview-render halves stay deferred.
- §3/§6 re-state the `--e2e` seeder: `seedFixtureRuns()` invokes the `conductor-run` `envelope_fixture` test target under `CONDUCTOR_E2E_SEED_DIR`, writes the subject through the production writer, and throws on a non-zero exit.
- §6's routine-arm paragraph retires the load-envelope banner's context-skip: the banner now asserts (label + axe). The RULE stays: an unproducible DOM state context-skips, and a skip is never a pass.
- §7's provenance SET admits a fixture seeded through the production WRITER, and the meaning-under-test proof is a SET (`lamps_fixture.rs` + `envelope_fixture.rs`).
- §1's Vector-1 trigger gained the third per-reader handle class and its mandated tier.
**Why:** the chunk's expected amendments, plus sites the fan-out found beyond them. Trap for later chunks: one retired-mechanism site carries no `rmcp` token, so a token-keyed sweep cannot reach it. Sweep for what the claim says, and re-derive the site count instead of trusting an enumeration.
**Ref:** .andromeda/runs/2026-09-02T14-34-37Z-wrap/

## 2026-09-02-mutation-tier-restored-for-conductor-tauri — the parity leg's first arm was never the mock runtime
**Section:** §2 Critical Path 7 (`:80`) · §2 trigger table (`:93`) · §5 Driver(s) (`:264`) · §5 Cross-module
patterns → Cross-surface parity (`:287`) · §5 deferred-leg bullet (`:288`) · §6 Both-surface parity Steps
step 2 (`:370`) + Verification signal (`:371`) — SEVEN sites, all applied.
**Change:**
- Cross-surface parity: retired "the Tauri mock-runtime run" as the leg's first arm. The measured arm SET is an in-process call to `conductor_run::{preflight, drive_run}` (the composition the Tauri `start_run` command's thread runs, with no `tauri::*` item) plus the assert_cmd CLI subprocess arm. The leg now lives in `crates/conductor-cli/tests/cross_surface_parity.rs`, a package with no `tauri` dependency in any section.
- The deferred-leg bullet's landed half is renamed from mock-runtime ↔ CLI to in-process-core ↔ CLI.
- §6 step 2's GUI leg is now the in-process drive. `get_ipc_response()` stays as the driver of the mock-runtime command-dispatch tier.
- §5 Driver(s): the parity CLI arm takes its binary path from the declared build-graph edge `assert_cmd::Command::new(env!("CARGO_BIN_EXE_conductor"))`, not `Command::cargo_bin`'s runtime `target/debug` fallback. assert_cmd stays the assertion library, and `cli_smoke`, inside the owning package, keeps `cargo_bin`.
- Critical Path 7's Verification signal, its required-test-type twin and §6's Verification signal were SPLIT, not rewritten. The envelope-equality half is PROVEN by the in-process arm ↔ CLI subprocess pair. The control-panel-LAUNCHED half is DEFERRED to the tauri-driver leg (§5) and still OWED against the Creator Brief.
**Why:** the chunk measured that the parity leg's first arm uses no `tauri::*` item. The relocated leg compiles and passes in a package with no `tauri` dependency, so the mock-runtime attribution was false. The first four changes are routine spec-illustration → sound-impl alignment: the parity invariant holds and only the description of its mechanism moved. The split was escalated, and the operator chose it. The alignment rule applies only when the contract is preserved, and Critical Path 7's signal comes from the Creator Brief Must-Work ("from the control panel AND headless"). Rewriting it to the measured mechanism would have retired a brief-sourced requirement to match the evidence. Trap: two of the sites carry none of the mechanism tokens, and a third surfaced only in the post-edit sweep. Find sites by sweeping what the claim says, not by token.
**Ref:** .andromeda/runs/2026-09-03T05-25-00Z-wrap/

## 2026-09-03-conductor-tauri-survivors-dispositioned — mutation read-out gate, accepted-deliberate roster, and the committed-fixture sets
**Section:** §4 Unit Test Strategy — Mutation instrument (`:226`) · §10 Quality Gates — Mutation-survivor disposition (`:500`) · §12 Test Decisions Log — `2026-08-20` entry (`:607`) · §7 Test Data & Fixtures — Self-bootstrapping requirement (`:398`, `:399`)
**Change:**
- §4's read-out gate was "`missed.txt` empty". It now requires `missed.txt` to hold EXACTLY the run's accepted-deliberate survivors, with every other named survivor in `caught.txt`. `missed.txt` is empty only when nothing is accepted, since an accepted survivor by construction SURVIVES (§10).
- §10's "the `declares` env-reading edge is the shipped case" is replaced by the roster named as a SET. The set is recorded in §12, each member against its own cited rule.
- §12's `2026-08-20` entry gains a bullet for `conductor-tauri`'s accepted-deliberate TRIPLE (`main:18` · `run_thread:296` · `start_run:263:8`), with each citation and its evidence home.
- §7's committed-fixture provenance and meaning-pinner SETs widen from `conductor-run`-only literals to the set of crate-local `tests/fixtures/` trees, with one round-trip per committed fixture.
**Why:**
- §4 contradicted §10, and this chunk was the first to exercise that visibly: its accepted run left 3 survivors in `missed.txt` while meeting its acceptance. The rule was reconciled to §10, the governing rule. The new condition is stricter than "free of the dispositioned names" because it also fails an unexpected survivor. The chunk escalated it and the operator resolved it at wrap: a gate's pass condition is not a wrap-time judgment call.
- The roster gained its second member (22 standing survivors → 3), so a single-case literal under-stated it.
- A committed fixture landed outside `crates/conductor-run/tests/fixtures/` for the first time: `crates/conductor-tauri/tests/fixtures/scenarios/`. It is read-only, consumed in-test through the production validator `Scenario::from_toml_str_with`, and pinned by `commands::tests::the_committed_scenarios_fixture_stays_loadable`.
**Ref:** .andromeda/runs/2026-09-03T09-10-06-wrap/

## 2026-09-03-conductor-run-composition-root-survivors-dispositioned — the `declares` class shrinks, its crate is corrected, and `conductor-run`'s roster member lands
**Section:** §10 Quality Gates — Mutation-survivor disposition (`:500`) · §12 Test Decisions Log — the `declares` bullet (`:607`) and a new `conductor-run` bullet (`:609`)
**Change:**
- §12's `declares` bullet drops the `x6` literal and names the SET (`lib.rs:358:5` → `false`, `:360:11`, `:360:21`, `:360:26`). It records that the class SHRANK on 2026-09-03 from the six members ratified 2026-08-21: `observe_run_contract:346` and `declares:358 → true` both proved killable with no env mutation and fell to `observe_run_contract_names_the_term_this_environment_does_not_declare`.
- §12 gains the roster's third member, `conductor-run`'s accepted-deliberate classes. Class B (`lib.rs:518:27`, `:559:25` ×2) is unreachable past the fixed `http://127.0.0.1:4317` egress gate. Class C (`lib.rs:66:8`) is unreachable without the fixed-name `andromeda-pulse-mcp` sidecar on the inherited `PATH`. Each carries its cited rules, with the chunk's disposition ledger as citation home.
- §10's inline roster reference is re-attributed and de-counted. It was "`conductor-verify`'s `declares` env-reading edge ×6"; it now names `conductor-run`'s `declares` env-read arms plus the two 2026-09-03 sets, "each enumerated there rather than carried as a count here".
**Why:** the tier re-run showed the roster was wrong in two ways.
- COUNT: the env read lives in `declares`, not in its caller. The cited env-at-the-caller rule (`.claude/rules/testing.md` 2026-08-10) therefore never covered the enclosing function's mutant or the always-true arm. The roster was also 4 short of the 8 accepted-deliberate survivors the tier reports (118 mutants: 8 missed / 82 caught / 28 unviable / 0 timeout; `missed.txt` matches the accepted set exactly).
- CRATE: `fn declares` has never been in `conductor-verify`; it is in `conductor-run`.
Both §12 bullets follow the derived-count discipline: name the SET, never a fresh literal that re-stales.
**Kept:** security-plan already states the attribution correctly. The cascade leaves (`.claude/rules/testing.md`, `.claude/docs/tests-summary.md`) carry the rule and "the accepted roster is a SET recorded in test-plan §12", with no count and no crate attribution, so they stay unchanged.
**Ref:** .andromeda/runs/2026-09-03T12-19-05-wrap/

## 2026-09-03-live-pulse-preconditions-probed — `boot` is probe-then-preflight; the runs-dir sandbox qualified
**Section:** §1 (Surfaces under test cli · 5-command `boot` requirement) · §2 (Self-bootstrapping fixtures) · §3 (intro · `boot` body · Per-test isolation · 5-command-discipline-wire) · §6 (cli driver row · Selector strategy)
**Change:**
- `boot` is now a precondition probe THEN a preflight readiness gate. This is recorded at all four sites that called it a gate alone (§1 requirement, §3 intro, §3 body, §3 wiring). The leading arm short-circuits on an unmet subject: it skips the preflight invocation and its two derived budgets and emits no `ReadyState` JSON, identically in `.sh` and `.ps1`. The command count stays FIVE, and the envelope, status-read and JSONL shapes are untouched, so §3 ↔ obs-plan §3 still agree.
- All three cli status-label enumerations (§1 Signal, §6 driver row, §6 Selector strategy) name the run-level non-lamp caption SET.
- The `CONDUCTOR_RUNS_DIR` sandbox is qualified, at both §3 Per-test isolation and §2 fixtures, to cross-process tests THAT WRITE run artifacts. The handle is repo-relative, and `resolve_under` rejects an absolute value at `Paths::resolve()` before the verb dispatches. A write-nothing verb's edge test does not set it at all.
**Why:** the runs-dir qualification resolves a contradiction inside §3: its e2e-seed bullet already recorded the handle as repo-relative, while Per-test isolation stated the sandbox unconditionally. Measured: with an absolute temp path set, the probe printed nothing and both new edge tests failed on empty stdout.
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-sr-findings-remediation — `boot` unreached through the leading probe; the `sr` leg's fifth handle
**Section:** §3 Test Harness Contract → `boot` (primary) · §1 Test harness requirements → `boot` · §6 E2E critical-path scenarios (five step-1 lines) · §6 Drivers per surface → desktop-webview
**Change:**
- §3's `boot` command body is marked not currently reached through `boot`. The leading probe's `handles-declared` subject cannot be satisfied, so the 2026-09-03 "exit 1 in both shells" note is unconditional. The body and its two derived timeout budgets have been unreached since `480bc66`. §1's probe-then-gate bullet carries the same qualifier.
- The five §6 critical-path step-1 lines no longer assert that the `boot` verb returns `ready:true`. They name the direct `conductor preflight --json` invocation instead.
- §6 desktop-webview drivers row: only `sr-empty` / `sr-error` carry a `scenarios` field at the one spawn site. The live `sr` subject therefore takes a FIFTH env handle from the shell, `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` (repo-relative). Without it the app loads the full catalog and the walk dies at S0-04 on option order.
**Why:** spec claims disproved by measurement, applied as routine spec alignment. The fifth handle is also an operator directive.
**Kept:** test-plan states nowhere that a last-scenario Stop reports `Done`, so the plan's expected §6 amendment had nothing to retire.
**Ref:** .andromeda/runs/2026-09-04T07-33-12Z-wrap/

## 2026-09-04-preconditions-probe-reads-path-handles-by-presence — boot reaches the gate; the declares roster shrinks
**Section:** §1 Test Scope Summary — harness `boot` · §3 Test Harness Contract — `boot` Leading arm + Command body · §6 E2E Test Strategy — the five critical-path step-1 lines · §12 Test Decisions Log — the `declares` accepted-deliberate entry and the classes B/C entry
**Change:**
- Retires the earlier claim that `boot`'s command body is unreached (was `not currently reached`, `cannot currently be met`, `both shells exit 1`). §1 and §3 now record that `conductor_core::handle_declared` grades each `ANDROMEDA_PULSE_*` name by its own KIND. `handles-declared` can therefore be satisfied, the short-circuit depends on the environment, and the command body and its two derived timeout budgets are REACHED and paid through `boot`.
- The five §6 step-1 lines drop the "invoked directly, not via `boot`" / "not through `boot`" workaround. The gate is reachable through `scripts/agent-run.{sh,ps1} boot`; a direct `conductor preflight --json` remains an option.
- §12: the `declares` accepted-deliberate SET is re-derived to **`lib.rs:362:5` (→ `false`) ALONE** (it had four members). The three operator arms moved into `conductor_core::flag_declared`, where its rstest matrix catches them. `declares -> true` is caught by `declares_rejects_a_name_this_environment_does_not_declare`.
- §12 classes B/C coordinates are re-measured (B `:559:27` + `:600:25` ×2, C `:68:8`), with identity confirmed by function and COLUMN rather than line. The crate total is re-derived 8 → 5.
**Why:** both shells' `boot` emits `ReadyState` JSON and exits 0 with no `skipped preflight` line. The `boot` sites are routine spec alignment. The two count sites held a stale literal enumerating a set the code owns. `.claude/rules/testing.md` (2026-09-03) required re-running the tier rather than carrying the count forward.
**Kept:** §10's roster prose names the SET rather than a count and stays correct after these edits.
**Ref:** .andromeda/runs/2026-09-04T17-15-00-wrap/

## 2026-09-04-sidecar-spawn-without-a-console-window — a zone that cannot be unit-asserted, and the roster's fourth member
**Section:** §1 Test Scope Summary — Untestable zones · §12 Test Decisions Log — mutation-instrument roster · §10 Quality Gates — Mutation-survivor disposition (inline sample) · §6 E2E Test Strategy — Drivers per surface (desktop-webview row)
**Change:**
- §1 gains an untestable zone for the APPLIED Windows creation flag. `std::process::Command` exposes exactly five getters and none for creation flags (rustc 1.95.0), so `build_command`'s `.creation_flags(...)` cannot be asserted through `as_std()`. The flag VALUE is unit-covered; its APPLICATION is SR-leg evidence, never a manual smoke step.
- §12's roster gains its FOURTH member: `conductor-verify`'s accepted-deliberate pair at `spawn.rs:116:5` (`sidecar_resolves_on_path` → `true` / `false`), recorded as the enumerated SET with its rejection rationale.
- §10's inline sample of the roster is extended to four, so it does not list three against a four-member set.
- §6's desktop-webview driver row widens the measured msedgedriver/WebView2 working SET to include Runtime 152.0.4191.62.
**Why:**
- The roster is written as a SET, never a count, because a count re-stales on the next addition.
- §6 names a runtime PAIR rather than substituting `.62` for `.53`: the driver was not refreshed across the host bump, so the pairing widened rather than moved.
- The §1 zone follows the doc's agent-driven invariant.
- §12 records that an agreement test for the pair was CONSIDERED and REJECTED. It would kill exactly one arm, depending on the host, so the score would mean different things on different hosts (§10 zero-flakiness). The rejection is part of the disposition.
**Kept:**
- §9 Matrix builds and its Pipeline E2E row (the 3-OS matrix and the separate `ubuntu-latest` + `xvfb` webview job) stay standing. This was escalated and the operator resolved it: they describe an UNIMPLEMENTED PLAN owned by the *A11y CI gate* route entry, not drift, and retiring them would delete a plan rather than correct a falsehood.
- `TokioChildProcess` in the §12 Decisions Log stays as written. It already carries an inline 2026-09-02 correction, and a decisions-log record is annotated, never rewritten.
**Ref:** .andromeda/runs/2026-09-04T20-15-00-wrap/

## 2026-09-05-audit-corrective — the supply-chain gate form, and the accepted-deliberate roster's fifth member
**Section:** §3 Bootstrap phases (quality-gate-config-emit) · §4 Unit Test Strategy (Mutation instrument) · §9 CI Integration (stage table + build-failure conditions) · §10 Quality Gates (roster enumeration + build-failure conditions) · §11 Test Anti-Patterns (CI) · §12 Test Decisions Log (mutation-instrument entry, four bullets + a new one)
**Change:**
- `cargo audit --deny warnings` became the bare `cargo audit` that CI actually runs, at all SIX sites.
- §12's accepted-deliberate coordinates were re-pointed by function + COLUMN after the `conductor-run/src/lib.rs` 1944 → 36-line split and the `conductor-tauri` `resolve_run_target` hoist:
  - `declares` `lib.rs:362:5` → `preconditions.rs:50:5`
  - class B `lib.rs:559:27` + `lib.rs:600:25` ×2 → `execute.rs:95:27` + `execute.rs:136:25` ×2
  - class C `lib.rs:68:8` → `canary.rs:58:8`
  - `run_thread` → `commands.rs:308:5`
  The present-tense "the set is now" restatement in §12's `declares` entry was also dated.
- A FIFTH roster member: `conductor-cli`'s tty-gate wrapper PAIR (`render.rs:161:5` / `:171:5`), cited against the 2026-09-04 `sidecar_resolves_on_path` host-dependence rule. The rejected `CONDUCTOR_COLOR`-class override is recorded as part of the disposition, and §10's enumeration is extended to match.
- §4's and §12's exit-3 samples are dated to the 2026-08-20 run. This chunk killed that run's two `conductor-verify` timeouts (timeouts 2 → 0).
**Why:** the `--deny warnings` form exits 1 by construction against the 18 `deny.toml`-adjudicated allowed warnings, while CI runs the bare form and passes. The site count is SIX, not the five a `cargo audit` grep finds: §12's Trigger-tooling entry writes the command hyphenated (`cargo-audit 0.22.2`) and has no `cargo audit` token. Standing rule: sweep a claim about a FLAG on the flag, not on the command name.
**Ref:** .andromeda/runs/2026-09-05T21-09-11Z-wrap/

## 2026-09-06-operator-gated-live-suite — the `--live` stage flag, and the `sleep(N)` ban's scope
**Section:** §2 Test Strategy — Agent-runnable invariants (determinism) · §3 Test Harness Contract — CI stage selectors · §9 CI Integration — Live-Pulse scenarios · §11 Test Anti-Patterns — CI · §11 Test Anti-Patterns — E2E
**Change:**
- §3's stage-selector bullet names `--live` beside `--unit` / `--integration` / `--e2e`. It gives the composition (leading non-priming `conductor preconditions` → B1 → B2 → 150s quiet window → A → the driven a11y arm) and the per-leg freeze to `runs/live-suite/{leg}.jsonl`. `--live` is a sibling `case` branch, so the 5-command discipline holds. It adds no port, env var or `CONDUCTOR_*` handle, and leaves the envelope, `status` read and JSONL shapes untouched, so §3 ↔ obs-plan §3 still agree.
- §9's sanctioned operator/local live-leg SET gains `scripts/agent-run.{sh,ps1} run --live` as the composed invocation. `[features] live-pulse = []` now gates a SET of targets (`lifecycle_live.rs` + `live_suite.rs`), and each still owes its own per-feature clippy.
- §11's CI ban repeats the SET; the ban itself (never a CI gate) is unchanged.
- §2's determinism bullet adds the `--live` suite to the enumerated operator-local real-wall-clock exceptions, which had two members.
- §11's `sleep(N)` ban gains a scope clarification. The ban's subject is in-test synchronisation. The quiet window is named as a firing-form precondition reproducing a SUT-side dedupe window that emits no signal to wait on. The closing sentence re-asserts that nothing inside a test may sleep to synchronise.
**Why:** the chunk landed the flag in both shells with identical semantics. The §2 change is required because a new non-deterministic real-wall-clock leg outside the enumerated exception set reads as a §10 zero-flakiness break. The `sleep(N)` change was deliberately NARROWED: the proposed carve-out permitting a fixed wall-clock wait was REJECTED. The 150s quiet window runs in the harness shell BETWEEN legs, not inside a test, so it never violated the ban and needed no permission. A ban is not weakened to accommodate something outside it.
**Ref:** .andromeda/runs/2026-09-06T09-37-04-wrap/

## 2026-09-06 0-pending adaptation (subject: `2026-09-06-operator-gated-live-suite`) — the `--live` auto-resolve leg is not run-stable
**Section:** §9 CI Integration — Live-Pulse scenarios (the composed `run --live` stage sentence)
**Change:** The §9 passage recording the composed stage now states the leg's SUT-UPTIME precondition. The auto-resolve leg needs an EMPTY active set at read-back, which holds only while Pulse is inside the emitting service's one-hour bootstrap window. Past that window, `service_went_silent` cues raise new autonomous incidents during the leg's own silent phase, so no window length reaches the arm. Inside it, the window must still clear 150 s from the LAST incident the leg's own preflight formed, and there can be more than one. This is explicitly NOT a source-delta fact: an unchanged tree grades differently by SUT uptime alone, so the leg must not be read as run-stable. Observables 2 and 3 and the driven a11y arm are unaffected.
**Why:** a 0-pending adaptation over a fact this session measured. Three runs of one unchanged scenario graded two ways, and the two failures had DIFFERENT causes. One failed inside the window with no silence cues, because the later of two preflight incidents was only ~122 s old at read-back (the margin). The other failed past the window on silence cues. A retry tuned for only one cause would mis-tune, which is the operator-ratified reason both are stated. The SUT basis was re-verified at HEAD `83d4060`. §9 is the home because it owns the sanctioned live-leg SET and the CI ban that make this leg operator-gated.
**Kept:** the other `run --live` sites register the flag or its exit rule and do not assert the leg's reachability, so they needed no change. The §3 ↔ obs-plan §3 bind is untouched: no envelope, status-read or JSONL shape changed.
**Ref:** NOT DERIVED

## 2026-09-06-run-report-envelope-conformance-gate — cleanup teardown re-homed to the binary; status reads the newest ENVELOPE line
**Section:** 3. Test Harness Contract - 5-command implementation (`cleanup` body, `status` body) + 1. Test Scope Summary (5-command `status`)
**Change:**
- §3 `cleanup` body: the shells keep only `rm -f runs/<run_id>.{jsonl,md}` and then call `conductor cleanup <run_id>`. The shells issue NO SQL. The three bound-parameter deletes run inside the binary as literal statements in one transaction (`RunsDb::delete_run`). The `sqlite3` CLI could never have honoured the prescribed `?1` form (it has no bind facility), and it is installed neither on this host nor on any CI runner.
- §3 `status` body: the journal read targets the NEWEST ENVELOPE line, never the file's last line. Both shells select it by the envelope-only `seed` key.
- §1 5-command `status`: its Mechanism clause carries the same qualification.
**Why:** the `status` defect was measured. On a seeded mixed-shape journal the verb reported a `CheckRecord`'s fields as the envelope (`seed: null`, `slo_tier: null`, exit 0); after the fix both shells print the envelope's values. The §1 site has no `**status**` bolding, so the locating grep could not see it, and a single-site apply would have left the unqualified-read claim standing. §3 ↔ obs-plan §3 still agree: obs-plan owns no status mechanism, and the envelope/JSONL shapes are untouched.
**Ref:** .andromeda/runs/2026-09-06T13-07-09-wrap/

## 2026-09-06-coverage-completeness-gate — Path 6 gate surface shipped; CI matrix re-based on measurement
**Section:** §1 Critical Path 6 · §4 Unit Test Strategy (Scenario catalog + coverage-matrix bullet) · §6 E2E → Coverage-matrix completeness gate scenario · §9 CI Integration → Matrix builds
**Change:**
- §1 Path 6's surface qualifier names the shipped gate: the crate-local `conductor-report --test coverage_gate` target, enforced in CI by the `ci.yml` "Coverage-completeness gate" step over the COMMITTED `coverage-matrix.md`, LF-pinned by `.gitattributes`.
- §4's bullet split. Per-scenario P-ID keying stays at unit tier. The zero-unclassified assertion moves to the crate-local `tests/` tier, because it must reach both `conductor_core::check_sut_drift` and `conductor_report::CoverageMatrix::render`, and only that side of the dependency edge can.
- §6: the `assert_cmd` exit-code + insta-golden / static row-count form is retired, superseded by the three shipped arms:
  - a runtime manifest load via `CARGO_MANIFEST_DIR`;
  - a negative arm that removes each classified row IN TURN and asserts `Err(CoreError::SutDrift)` naming the dropped id;
  - byte-equality to `CoverageMatrix::render()`.
  A row count is the literal-count form this path's own Verification signal forbids.
- §9 Matrix builds: the three-OS matrix and the separate `ubuntu-latest`+`xvfb` webview-E2E job are re-stated as the TARGET arrangement, owned by the *A11y CI gate* route entry. The SHIPPED arrangement, as measured, is two `windows-latest` jobs. The leg's platform CAPABILITY is untouched.
**Why:** the first three changes reconcile the spec's own stale description, as a chunk that first BUILDS a spec'd gate must. The §9 change was escalated, because the gap predates this chunk and the *A11y CI gate* entry owns the CI-job work. The operator's resolution was NARROW: retire only false PRESENT-TENSE claims and leave target-state descriptions standing.
**Kept:** no §9 stage-table row for this gate. The proposal's rationale cited a §9 "complete inventory of gates" claim that does not exist, and the prior chunk's equivalent CI gate step took no row; the operator ruled to follow that precedent. Six further platform-claim sites in this master stay standing under the same narrow ruling.
**Ref:** .andromeda/runs/2026-09-06T15-59-32-wrap/

## 2026-09-06-halo-hue-budget-re-driven — --live composition re-anchored to live_leg_order, leg H first
**Section:** 3. Test Harness Contract — `run` CI stage selectors, the `--live` paragraph
**Change:** The four-leg list (B1 -> B2 -> quiet window -> A -> a11y arm) is replaced by the shipped ordered leg SET, with leg H (`halo-hue-encoding`, budget `live_leg_budget_sec 180`) first. The order is anchored to `live_leg_order` in `scripts/agent-run.{sh,ps1}` instead of being restated as a count. Unchanged: no port, env var or `CONDUCTOR_*` handle is added; section 3 <-> obs-plan section 3 still agree; legs still freeze to `runs/live-suite/{leg}.jsonl`.
**Why:** the chunk added leg H ahead of B1 in both shells, taking the leg count from 4 to 5. It is applied as SET-naming, not a fresh literal, because a literal re-stales at the next leg; the doc had already gone stale once, one chunk after the composition was first written.
**Ref:** .andromeda/runs/2026-09-07T08-32-30-wrap/

## 2026-09-06-halo-hue-budget-re-driven — The auto-resolve leg's not-run-stable verdict is conditioned on the DEFAULT window
**Section:** 9. CI Integration — Live-Pulse scenarios
**Change:** The verdict is now scoped: the leg is not run-stable UNDER THE DEFAULT one-hour window.
- That default is a boot-time posture. `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is resolved once by `Thresholds::from_env` at Pulse's boot, and Conductor never sets or reads it. The arm's reachability is therefore bounded by the window IN FORCE AT BOOT.
- The suite launches Pulse under a stretched boot-wide window. The stretch removes cause (a) ONLY, and the leg stays not-run-stable under it.
- The booted posture is confirmed by grepping Pulse's log for the TARGET `triage.baseline.bootstrap_window.override` (level WARN, reason="env_override", resolved_seconds). Never grep for the emitting function name, which appears nowhere in the log.
**Why:**
- Under that posture, leg A reached `KnownResidual` via the `ReadBack::AutoResolved` arm on 2026-09-07, after failing to reproduce on 2026-09-06; this discharged the chunk's CARRY.
- The verdict was NARROWED at the light gate. The wrap's own re-run graded the SAME tree under the SAME posture `ManualCheck`, with the evaluator provably disarmed in both runs (`silence_cues_emitted: 0`). What separated the two runs was when the window's last incident formed (+45s vs +137s).
- The prior wording named the emitting function (`warn_bootstrap_window`), which no log line contains, so a light-gate grep could never hit.
**Ref:** .andromeda/runs/2026-09-07T08-32-30-wrap/

## 2026-09-07-dependency-polish — Tier-justification crate count, stack version and the baked lock literal reconciled
**Section:** §1 Test Scope Summary · §8 Mocking & Stubbing Discipline · §9 CI Integration → Live-Pulse scenarios · §12 Test Decisions Log
**Change:**
- Crate count: "8 workspace crates" → 9 (§1) and "8 seam crates" → 9 (§12).
- `tokio 1.48.x` → `1.52.3` at both sites (§1 coverage-scope entity, §8 time-mocking row).
- §9's "the lock held at 564 packages" is replaced by the PROPERTY it stood for: the empty `live-pulse` feature adds zero package nodes. The lock total is read from `Cargo.lock`, never pinned in prose.
**Why:** the lock moved 564 → 562. The §12 site says "seam crates", not "workspace crates", so the report's own grep missed it, and a single-site apply would have left the retired count in the decisions log. The §9 literal was removed rather than replaced with the new number, because a fresh literal re-stales; naming the property does not.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-a11y-ci-gate — the webview-E2E CI job shipped (Windows, not Linux); printed-verdict assertion; a fourth handle class
**Section:** §1 Test Scope Summary (desktop-webview Driver · Vector-1 coverage trigger) · §2 Agent-runnable invariants · §3 Test Harness Contract (`--e2e` clause) · §6 Drivers-per-surface (desktop-webview Mode) · §6 Both-surface parity · §9 CI Integration (Matrix builds · Pipeline E2E row)
**Change:**
- Seven platform sites retired the Linux-CI/`xvfb` ARRANGEMENT. The measured host SET is Windows WebView2: the dev host plus CI's `windows-2025` `a11y` runner. Linux+`xvfb` stays as unrun target-state, and macOS has no driver.
- §9 Matrix builds records that the webview-E2E job SHIPPED as the `a11y` job, and that CI jobs are identified BY NAME, not by `ci.yml` line coordinates, which move as the workflow grows.
- §3's `--e2e` clause records the printed-verdict assertion (both shells, identical semantics). The `runs/a11y/<run_id>.jsonl` violation artifact is a SEPARATE shape outside the harness's non-recursive glob. The five-command surface, the `status` disk read and the envelope/JSONL shapes are unchanged, so §3 ↔ obs-plan §3 still agree.
- §1's Vector-1 trigger gains `CONDUCTOR_A11Y_STRICT` as the fourth per-reader class, with e2e-tier coverage.
**Why:** the master's own §9 named the *A11y CI gate* route entry as owner of the arrangement this chunk shipped. When a chunk ships a fix the master names as route-owned, the change is routine. The boundary-widening rule was checked first and does not apply: nothing new crosses a hardened boundary, the live-Pulse exclusion is intact, and the strict flag only changes what an unresolved handle costs.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-07-sr-findings-fixed — printed-verdict contract conditioned; a11y job measured red; step set opened
**Section:** §3 `run` → `--e2e` CI stage selectors · §3 Bootstrap phases → `5-command-discipline-wire` · §6 Drivers per surface (desktop-webview row) · §9 Pipeline structure (E2E webview row) · §9 Matrix builds (OS bullet)
**Change:**
- §3 `--e2e`: the printed-verdict contract holds only where the INVOKING environment does not preempt the capture-then-print. A caller that sets `$PSNativeCommandUseErrorActionPreference = $true` turns wdio's non-zero exit into a terminating error before the script prints or asserts; the output is one `NativeCommandExitException` line and no verdict. This chunk removed that setting from the CI gate step; both shells are byte-unchanged.
- §3 `5-command-discipline-wire`: the identical-semantics contract binds the SCRIPTS together with the invoking environment, because such a caller makes the `.ps1` leg diverge with ZERO script change.
- §6 desktop-webview row: the baked `10 passing / 2 skipped` sample is replaced by the SET it evidences: fully green, with only the two live-hold subjects skipped. The pass tally is a moving count, 12 as of 2026-09-07.
- §9 E2E row: the a11y job's step set is no longer closed. Two `continue-on-error` diagnostic steps and an `if: always()` `a11y-session-diag` upload joined it. Rule: the GATE step itself never carries `continue-on-error`.
- §9 Matrix builds: the job's first push-triggered run is still owed. The job HAS run in CI (four `ci-probe/` runs) and is RED at WebView2 session creation; the *Hosted-runner WebView2 session* route entry owns that.
**Why:** the sample count was missed by the orchestrator's own search. The search keyed on a proxy spelling of the count and found nothing, but the doc says `10 passing`. Standing rule: search for what the doc says, not a proxy for it. The sample was de-literalized, because naming the set stops it carrying a value that ages every time the arm grows.
**Ref:** .andromeda/runs/2026-09-07T21-30-50-wrap/


## 2026-09-08-hosted-runner-webview2-session — driver↔runtime pair SET; hosted-runner capability bounded to the measured runtime
**Section:** §1 Surfaces under test (desktop-webview Driver) · §2 Agent-runnable invariants · §6 Drivers-per-surface table (desktop-webview Mode cell) · §6 cross-surface parity step 2 · §9 CI Integration Matrix builds
**Change:** Seven amendments across five sections.
- (1) §6 Mode cell: the pairing clause names the measured set of PAIRS, split by the dev host's driver refresh at 2026-09-02 14:45 local — passing = {runtime 152.0.4191.53 × driver 151.0.4129.101, the 2026-09-02 MORNING window only} ∪ {runtime 152.0.4191.x × driver 152.0.4191.53, every run since}; failing = {runtime 151.0.4129.101 × driver 151.0.4129.101} on the hosted image. Retired: the 2026-09-04 runs were "un-refreshed" (they were not).
- (2)–(6) the five sites stating the hosted `windows-2025` runner as a proven headful WebView2 host (§9 OS bullet primary; §1, §2, §6 parity, §6 Mode opening clause dependents) now name the dev host as the only member measured to DRIVE a session, the runner wired-but-non-driving. The retirement names the hosted image AS SHIPPED (WebView2 Runtime 151.0.4129.101), not a flat incapacity; whether an in-job runtime-152 install changes it is OPEN, owned by the follow-up route entry.
- (7) §6 Mode cell: the mechanism behind the driver's silence about the profile path — msedgedriver creates its own `%TEMP%\scoped_dir<pid>_<rand>\EBWebView` (two siblings per session, one holding the profile), reports `"userDataDir": ""` in a SUCCEEDED session's capability response, and overrides an inherited `WEBVIEW2_USER_DATA_FOLDER`; the path exists while the driver's output names it nowhere.
**Why:** The driver attribution was measured false at this chunk: the dev-host driver was refreshed 2026-09-02 14:45, and the driver-alone probe read msedgedriver 152.0.4191.53 against browser 152.0.4191.66. The morning half was verified TRUE rather than retired (its evidence predates the refresh), so the claim is split, not discarded. Scope bounded on operator ruling: the probe measured that image at that runtime and nothing wider. Edit (7) was operator-directed at this wrap.
**Kept:** a11y-plan's matching pair reading was verified TRUE and left standing.
**Ref:** .andromeda/runs/2026-09-08T14-20-00-wrap/

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — CI arrangement re-based to the measured job set; the runtime × driver PAIR retired as a discriminator
**Section:** §1 Surfaces under test (desktop-webview Driver note) · §6 E2E Test Strategy — Drivers per surface (the Failing entry) · §9 CI Integration — Matrix builds (OS bullet, two distinct claims on one line)
**Change:** Four edits on three sections.
- (1) §9 OS bullet: was "the SHIPPED CI arrangement is measured as two jobs, both `windows-latest`"; now THREE jobs — `rust` and `frontend` on `windows-latest`, `a11y` on `windows-2025` — named BY JOB, not by line coordinate (which the same bullet forbids).
- (2) The same line's second claim, "Whether installing WebView2 Runtime 152+ in-job changes it is OPEN", retired as measured: run `34280136892` installed Evergreen 152.0.4191.66 and the bare-app probe still reported `DevToolsActivePort first seen: never within 90s`.
- (3) §1 desktop-webview Driver note drops "a runtime-152 install in-job is untested" for the measured result.
- (4) §6 Failing entry re-keyed from the `{151 × 151}` pair to the hosted IMAGE at both measured runtimes, stating the PAIR is not the discriminator: 152 × 151 PASSES on the dev host (152.0.4191.53 × 151.0.4129.101) and FAILS on the hosted image (152.0.4191.66 × 151.0.4129.101) — same runtime major, same driver, opposite outcomes. What differs is the runtime PATCH (53 vs 66, unmeasured as a cause and named as such) and the image; the open variable is the hosted image itself, a policy or session property, owned by the `v2-24` deferral note.
**Why:** Both §9 claims were measured false this chunk (the workflow parsed; the in-job install run). Edit (4)'s framing is the operator's correction at escalation: extending the measured set alone would have left the table implying the pair still discriminates at major granularity, which the dev-host PASS at the same majors falsifies. Edits (2)–(4) were escalated and approved with a playbook rule minted. Trap: one line carried two distinct claims needing two separate edits — a single verbatim apply would have left one standing.
**Ref:** .andromeda/runs/2026-09-09T13-20-08-wrap/
## 2026-09-10-release-build-and-bundle — singleton remedy widened to a SET; tauri-cli joins the tool-floor roster
**Section:** §11 Test Anti-Patterns — Integration (process-global singleton) · §4 Unit Test Strategy — Tool-version policy
**Change:** Two edits.
- (1) §11 process-global-singleton remedy: was exclusive ("The remedy is a test binary of its own"); now a SET of two with the discriminator named — (a) an own test binary, the default; (b) a shared in-module guard serializing ONLY the tests that touch the singleton, the answer when the tests are INLINE over crate-private helpers and (a) would require `pub` on them, widening the crate's public API for tests alone (forbidden by architecture.md §Established Decisions [Module Boundaries]). Both arms carry the same discriminator: green under BOTH runners. The runner-knob ban is byte-unchanged and restated: serializing the FILE with `--test-threads=1` or a profile setting still hides the defect, and an in-code guard is not that. The evidence is marked `as measured 2026-09-10`.
- (2) §4 Tool-version policy roster gains `tauri-cli` beside cargo-nextest / cargo-llvm-cov / cargo-mutants: it is the BUNDLER; neither it nor `tauri-bundler` appears in any lockfile (the `tauri` CRATE the lock resolves is a different artifact); installed `cargo install tauri-cli --locked`, floor measured 2.11.4.
**Why:** (1) escalated and resolved with the operator: test-plan's exclusive remedy collided with architecture's visibility boundary, and the chunk resolved it by measurement — the `conductor-core` panic-hook pair took the guard, green under nextest and `cargo test -p conductor-core` plus 12/12 under forced contention. Leaving the exclusivity would have left the doc contradicting shipped, measured-sound code. (2) is the chunk plan's own expected amendment, a pure addition with no prior occurrence in any master.
**Ref:** .andromeda/runs/2026-09-10T20-36-29-wrap/

## 2026-09-11-hosted-runner-endpoint-cause-probed — the same candidate pair retired at both its sites; the a11y diagnostic-step literal de-literalized to a SET
**Section:** §9 CI Integration — Matrix builds · §6 E2E Test Strategy — Drivers per surface (desktop-webview row) · §9 CI Integration — Pipeline structure (E2E-webview row)
**Change:** Three edits on three sections.
- (1) §9 Matrix builds: retired "with a hosted-image policy or session property the leading unmeasured candidate" — CI run `34586959536` measured policy keys ABSENT on the hosted runner AND the dev host, runner `SessionId 2` / `UserInteractive: True`. No replacement candidate is stated; the elevation difference is recorded as a difference, not a cause; the third probe returned no reading.
- (2) §6 desktop-webview driver row restated the same claim ("The open variable is therefore the hosted image itself — a policy or session property —"); it now says the image remains the open variable while both named candidates are measured and retired, leaving no named candidate. The "not a runtime × driver pairing" half is unchanged.
- (3) §9 Pipeline-structure E2E-webview row: was the literal "two `continue-on-error` DIAGNOSTIC steps" plus a two-member enumeration; now names the SET {driver+runtime versions · session isolation probe · cause probes, invoking `scripts/webview2-cause-probe.ps1`}. The GATE-step-never-carries-`continue-on-error` clause is byte-unchanged.
**Why:** Both candidates were measured absent. (3) follows the rule that a stale literal enumerating a SET the code owns is applied as set-naming, never as a fresh count, so a fourth probe cannot re-stale it. (2) is a duplicate occurrence: a single-site apply at §9 would have left the disproved candidate pair alive in the §6 driver table.
**Ref:** .andromeda/runs/2026-09-11T10-27-30-wrap/

## 2026-09-11-hosted-runner-endpoint-cause-closed — the endpoint cause ESTABLISHED as elevation at both its sites
**Section:** §9 CI Integration — Matrix builds · §6 E2E Test Strategy — Drivers per surface (desktop-webview row)
**Change:** Three edits on two sections, one claim.
- (1) §9: was "No replacement candidate is named: the run recorded one difference (the runner is elevated, the dev host is not) which is a difference and not a demonstrated cause, and the third candidate — the module version the host processes actually load — returned NO reading, its probe's step running `0.645` s after the isolation step stops the app"; now the measured set — the module reading TAKEN in the isolation step's live window (CI run `34645345201`); driver/runtime major SKEW retired as a cause by direct variation with a control (CI run `34654076633` — install floor-conditional, `msedgedriver`/Edge/runtime all `152.0.4191.66`, processes loading 152 modules, every isolation reading byte-identical, endpoint still absent); ELEVATION varied on the known-good host and established as the cause, REMEDIABLE IN PRINCIPLE, the MECHANISM recorded-not-established, scope bounded to the two measured platforms, `v3-02` named as the remedy's owner.
- (2) "the cause stays OPEN and is owned by the `v2-24` deferral note" re-dated to "stayed OPEN until 2026-09-12".
- (3) §6 desktop-webview row: was "leaving the image open with no named candidate"; now names ELEVATION as the measured candidate and records the skew retirement. The "not a runtime × driver pairing" half is unchanged and remains correct.
**Why:** Routine under the playbook rule for retiring a platform claim: the retired sentence named its own preconditions (the reading absent because the probe ran after the app stopped; the cause unestablished because nothing varied it), and both arms removed them; what remains OPEN is stated with its owner (mechanism unmeasured, endpoint still closed, remedy owned by `v3-02`). Trap: the §6 site carried none of the swept tokens and was found by reading for the claim's meaning — the second consecutive chunk where this row needed the duplicate-occurrence mechanism.
**Ref:** .andromeda/runs/2026-09-12T10-08-17-wrap/

## 2026-09-13-audit-debt-retired-before-epoch-1-closes — roster identity moved off `file:line:col`; the tally gate's committed form; coverage exclusions re-stated as shipped
**Section:** 1. Test Scope Summary · 4. Unit Test Strategy — Mutation instrument · 9. CI Integration — Scoped mutation audit · 10. Quality Gates — Mutation-survivor disposition · 10. Quality Gates — Coverage thresholds · 12. Test Decisions Log
**Change:**
- (A) §12: the five accepted-deliberate members are keyed by mutation DESCRIPTION + `scripts/mutation-roster.toml`'s `member` key, not `file:line:col`: `member-1` `replace declares -> bool with false`; `member-2` `replace run_thread with ()`, `delete ! in start_run`; `member-3` class B `delete field degraded from struct Observation expression in execute_scenario`, `replace - with + in execute_scenario`, `replace - with / in execute_scenario`, class C `delete ! in preflight`; `member-4` `replace sidecar_resolves_on_path -> bool with {true,false}`; `member-5` `replace stdout_color -> bool with false`, `replace stderr_color -> bool with false`. Retired: "identity re-measured by function + COLUMN, never by line arithmetic" (quoted in a closing bullet) and "identity confirmed by function and COLUMN rather than by line"; coordinates stay as dated HISTORY. Rows PER GATED UNIT: `member-4`/`member-5` now, `member-1`–`member-3` once their crate's tier re-runs.
- (B) §12 `conductor-verify`: POPULATION fell by one as `ContractManifest::default_path` was REMOVED (its mutant left, not moved to `caught.txt`); read from the run, never restated.
- (C) §4: `python -X utf8 scripts/mutation-gate.py {unit}` → fresh gitignored `target/mutation-gate/{unit}-{stamp}/`, gating the tally as a MULTISET over `(file, mutation)`; `cargo mutants` exited **2** on both PASSING runs; `--output`: `os error 3` if the parent is missing — the stable parent is created, the leaf never.
- (D) §9 per-chunk runs use it; non-CI footing unchanged.
- (E) §10: two forms — §12 the citation home, the roster TOML the coordinate-free executable one — joined on `member`, neither restating the other.
- (F) §10 Coverage: exclusion list → one `--ignore-filename-regex '[\\/]tests[\\/]'`, broader for any crate-local `tests/` tree (`conductor-emit/tests/common` needs no entry), NARROWER in one way: `conductor-verify/src/bin/stub_pulse_mcp.rs` (under `src/`) is NOT excluded.
- (G) §1: `conductor-core`'s "runtime-agnostic engine library that every other crate depends on" → the declared-edge SET, `conductor-emit` excepted.
**Why:** (A) operator-resolved, no playbook rule governing (fresh literals re-staled 2026-09-04, 2026-09-05): no fourth re-point; one-time, no rule. (B)–(G) routine, measured; (G) lateral, arch owns the wording.
**Kept:** a §4 duty to verify committed non-Rust instruments — operator-dismissed as undecided policy.
**Ref:** .andromeda/runs/2026-09-13T19-15-00-wrap/

## 2026-09-14-emit-scrubber-and-percentile-math-under-test — the roster's sixth member, accepted for EQUIVALENCE; registration moved off the row count
**Section:** 10. Quality Gates — Mutation-survivor disposition (acceptance basis + member enumeration) · 12. Test Decisions Log — Mutation instrument (new `conductor-emit` member bullet + the identity note's per-unit clause)
**Change:** Four edits across two sections.
- (A) §10 acceptance BASIS, one ground → two: was accepted-deliberate only "when a standing rule prescribes the untested shape"; now also a mutant proven EQUIVALENT — no input in the domain distinguishes it, so the acceptance rests on a measurement. Such an acceptance covers the individual mutant, never the comparison class its siblings share.
- (B) §10 member enumeration five → six with `conductor-emit`'s 2026-09-14 equivalent mutant; "a new accepted disposition EXTENDS the roster rather than contradicting a single-case literal" unchanged.
- (C) §12 bullet `member-emit-1`, `replace < with <= in quantile` in `latency.rs` (the `u < 0.5` guard): the branches meet at `u = 0.5` with one value (`u / 0.5` exactly `1.0`, `(u - 0.5) / 0.45` exactly `0.0`, both exactly `p50`), the only input the flip re-routes, so every output is a bit-identical `f64` — EXACT `f64` comparison, 5 profiles × 10 008 points, 0 differing. Population: the two files WHOLE, 221 mutants (1 missed / 202 caught / 15 timeouts / 3 unviable), not the audit's `--shard 1/4` (115 of 227, unre-measurable once the accessor retirement shifts later indices; its 57 a strict subset, not a baseline). Scope: four same-shape siblings ARE killable (a five-equivalent prediction rested on a `1e-9` epsilon swallowing ~7e-15 IEEE754 differences). Rejected: a tolerance band; restructuring `quantile` (a production change for a tool, §11 forbids). The fifteen timeouts are classified in the citation home and rostered nowhere — `scripts/mutation-gate.py` reads `missed.txt`/`caught.txt`, never `timeout.txt`.
- (D) §12 identity note: three units carry rows (`conductor-cli` `member-5`, `conductor-verify` `member-4`, `conductor-emit` `member-emit-1`), replacing the dated two; REGISTRATION is the `[[unit]]` declaration, never the row count.
**Why:** The plan's sole expected amendment; the operator's wrap directive settled content and the equivalence basis, so routine, the class's playbook rule proposed, not minted. `scripts/mutation-roster.toml` declares a unit by a `[[unit]]` table (optional `files` scope via `-f`): a declared unit with all survivors killed passes on an empty `missed.txt` (the old rule failed it as "not a gated unit"), and a scoped tier generating 0 mutants is an explicit FAIL, not the exit-0 clean sweep `cargo mutants` reports.
**Ref:** .andromeda/runs/2026-09-15T05-14-43-wrap/

## 2026-09-16-scenario-assertion-audit-gate — the second static gate over committed data
**Section:** §6 (the static-gate scenario) · §4 (Scenario catalog + coverage-matrix) · §7 (Self-bootstrapping requirement, two sites)
**Change:**
- §6's `Coverage-matrix completeness gate (static)` scenario gains a sibling leg for the scenario-assertion audit gate — `crates/conductor-core/tests/scenario_audit_gate.rs`, asserted by `cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci`, CI-enforced by the presence-guarded `Scenario-assertion audit gate` step — with its two axes, both-directions exact-set grading (`unpinned` / `rotted` / `lost_subject`, no literal-count predicate), six in-suite negative arms, a closed-set arm and a sweep control. A ledger row's `discriminates` flag is RECORDED, never graded: whether a token depends on the scenario's own stimulus is a SUT-behaviour claim §11 routes to the live/operator gate. Retired: §6 naming one static gate where the tree runs two.
- §4's scenario-catalog bullet names the catalog half's own crate-local `tests/`-tier gate beside the coverage-matrix half's; it no longer reads as if P-ID presence were the only static assertion over the catalog.
- §7's committed-fixture family gains its third tree (`crates/conductor-core/tests/fixtures/`), and its per-fixture round-trip enumeration gains the matching round-trip (was 2 of 3 trees).
**Why:** The chunk shipped the gate. Recorded as a sibling LEG, not an eighth `#### Scenario`, so §6's "(7 scenarios — the test-scope Section 4 maximum.)" qualifier holds, and with no §9 stage-table row, per the `2026-09-06-coverage-completeness-gate` operator resolution. Trap: §7's tree set and its round-trip enumeration are separate sites; the body edit reached only the first.
**Kept:** the other `coverage_gate` mentions (obs-plan's roll-up log field, §1 Critical Path 6, §6's coverage-gate verification signal) describe the coverage gate as its own subject and none claims it is the only static gate.
**Ref:** .andromeda/runs/2026-09-16T08-43-09-wrap/

## 2026-09-16 — a11y-ci-gate-at-an-honest-terminal
**Section:** §9 CI Integration (the E2E (webview) stage row · the hosted-runner reading)
**Change:** Two edits.
- (1) §9 E2E (webview) stage row: the `continue-on-error` DIAGNOSTIC step SET stays named as a SET and gains its fourth member, `Remote-debugging-pipe route` (`if: always()` + `continue-on-error`, 2026-09-16); ordering restated — cause probes between the isolation probe and the pipe-route step, pipe-route before the upload. The asserting step is recorded as launched through `scripts/a11y-limited-token-launch.ps1` rather than invoking `agent-run.ps1` directly. Retired: three diagnostic steps and a direct gate invocation.
- (2) §9 hosted-runner reading: the remedy CLASS (a limited-token launch + a re-measurement, owned by `v3-02`) is recorded MEASURED INSUFFICIENT, with the three-leg basis, the two mechanisms named, and the successor entry as owner.
**Why:** The row named three diagnostic steps where `ci.yml` ships four, and a direct gate invocation the job no longer makes. (2) is a cascade-found duplicate of the owned-by-`v3-02` remedy claim retired in architecture.md; no fan-out agent proposed it, and a single-site apply would have left the retired claim standing in a second master.
**Kept:** §9 Matrix builds' dated citation of CI run `34586959536` and the dated 2026-09-07 shipping-history clause naming `agent-run.ps1 run --e2e` — both true as history.
**Ref:** NOT DERIVED

## 2026-09-17 — 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm
**Section:** §1 Surfaces under test · §2 Agent-runnable invariants · §6 Drivers per surface · §6 Both-surface parity · §9 Matrix builds — 4 sites
**Change:** The hosted-runner endpoint verdict is retired as UNCONDITIONAL and restated as configuration-bound. At CI run 35192876641, hosted `windows-2022` at a coherent 131.0.2903.86 msedgedriver + WebView2-runtime pair, High integrity: `DevToolsActivePort` in 1 s, WebDriver session created, routine arm 11 passing / 1 failing / 2 skipped, SC 2.4.3 passing, the single red a counting-basis defect in the assertion (12 visits / 6 distinct, bracket lists identical to the character). The endpoint still does not open on `windows-2025` at runtime 152/153. Corroborated externally by actions/runner-images#14738 (byte-identical image and runtime, a plain Tauri/wry app with no token work). Retired wordings, corrected at every site: "opens no remote-debugging endpoint", "RED at WebView2 session creation", "never been green", "Still never a green run", "endpoint remains CLOSED", "runnability is measured-unproven". Integrity's SIGN is configuration-bound: Medium helped at runtime 153 on the dev host; High is REQUIRED at 131 on `windows-2022`. The medium-integrity launcher is recorded MEASURED-INSUFFICIENT: it lowers the label as designed (parent `S-1-16-12288` → child `S-1-16-8192`, confirmed inside the leg) and did not open the endpoint.
**Why:** A session measured green on `windows-2022` falsified the unconditional verdict. The 2026-09-16 legs A/B/C are BOUNDED by this, never retired — they were correctly measured on what they measured.
**Kept:** Four escalations reached no operator ruling this wrap, so the probe-scoped `ci.yml` surfaces that would have motivated further amendments — the `windows-2022` label, the `≥152` floor bypass, the `msedgedriver.microsoft.com` egress, and the launcher's removal from the asserting step — were REMOVED instead of ratified. The shipped arrangement is unchanged and no arrangement row moved; those amendments stay owed.
**Ref:** NOT DERIVED

## 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration — hosted-runner capability restated as the measured SET; a11y job arrangement re-stated
**Section:** §1 Test Scope Summary (Surfaces under test, desktop-webview Driver) · §2 Test Strategy (Agent-runnable invariants) · §6 E2E Test Strategy (Drivers per surface · Both-surface parity step 2) · §9 CI Integration (Pipeline structure table, E2E-webview row · Matrix builds)
**Change:**
- Five restatements of one capability verdict — "the dev host, the only member measured to DRIVE a session; CI's `windows-2025` `a11y` runner is wired but drives none on the image as shipped" — retired for the measured SET: the dev host AND the hosted `windows-2022` runner at a coherent 131.0.2903.86 driver/runtime pair at High integrity, the arm green at 12 passing / 0 failing / 2 skipped (run 35208593666); hosted `windows-2025` at runtime 151/152/153 drives none. "That capability does NOT extend to the GitHub-hosted image AS SHIPPED" is re-scoped to the `windows-2025` image.
- §6 "The named candidate is now ELEVATION, and it is the established cause" is bounded to the configurations that measured it and does NOT generalize — integrity's SIGN is configuration-bound (the Medium drop helping at runtime 153 on the dev host, breaking the session at 131). §9 "REMEDIABLE IN PRINCIPLE — the app must not run elevated" is retired as a general remedy.
- §9 stage row and Matrix-builds prose restate the job's arrangement by SET, not count: `windows-2022`; the driver pinned by the job's own gate and published as `EDGEWEBDRIVER`, not taken from the image; the asserting step invoking `scripts/a11y-token-witness.ps1` directly at native High integrity; the diagnostic step set minus the removed version-diagnostic.
- The 11 passing / 1 failing / 2 skipped probe reading stays dated to run 35192876641, marked superseded by the green, its red assertion recorded fixed rather than open.
**Why:** A platform verdict is retired by naming the measured SET, never by a fresh single-platform literal — which is also why the job step count (17 → 16) is recorded as a step SET. Driver/runtime major SKEW stays retired as the cause of the `windows-2025` missing endpoint without contradicting the coherence finding: an incoherent pair is REFUSED with an explicit version error, while the `windows-2025` failure was the silent absence of `DevToolsActivePort` under a pair that already cohered (stated in security-plan.md where the coherence claim is introduced).
**Kept:** the remaining `windows-2025` mentions (dated measurements or bound to that image at its runtime), `Evergreen` / `floor-conditional` (the 2026-09-12 experimental condition under which skew was retired), `limited-token` / `runas` (the now-moot remedy-class history), and `must not run elevated`, quoted as retired.
**Ref:** .andromeda/runs/2026-09-17T10-34-20-wrap/

## 2026-09-17-keyboard-and-focus-order-coverage-ownership — §4's ui/ proving mechanism named as the SET of executing legs
**Section:** §4 Unit Test Strategy — the `conductor-tauri/ui (frontend SPA)` bullet
**Change:** The bullet was closed at the wdio legs — "proven only by EXECUTING the wdio leg that LOADS them", enumerating `--e2e`, `npm run a11y:driven` and the three `a11y:sr*` legs, with anything reachable from none of them "as unproven as it was under the build gate". It now names the proving mechanism as the SET of npm legs that EXECUTE a TS member: those four plus the driver-free, Pulse-free static checker `npm run a11y:ownership` (`tsx test/a11y/check-claim-ownership.ts`), asserted on its printed last line beside its exit. The head clause is corrected from "EXECUTING the wdio leg" to "EXECUTING a leg". Qualifiers: the leg runs no test framework (`tsx` is a TypeScript executor, not a runner), so §12's decision that no JS/TS unit runner is adopted stands unchanged; and it is operator-local like the §4 mutation gate, invoked by no CI step and adding no sixth `agent-run.{sh,ps1}` command.
**Why:** The chunk committed a gate, into the one surface §4 declares build-gated-plus-wdio-legs, whose runner the plan named nowhere; a closed set would have made a shipped, re-runnable gate read as an unproven member by the doc's own rule. Naming the SET rather than re-closing the list, per the de-literalization discipline, keeps the next executing leg from re-staling it. Trap: the retired head clause survived inside the very line just edited, contradicting the addition later in the sentence.
**Kept:** the `JS/TS unit runner` invariant at each of its sites (`tsx` carries no test framework; §12's subject is a component/unit runner such as vitest); the `a11y:driven` mentions, which name the arm without restating the closed set.
**Ref:** .andromeda/runs/2026-09-17T17-09-36-wrap/

## 2026-09-22-interpretation-proven-live — the real-model selector and harvest leg, parity scoped to the deterministic posture, and three stale claims corrected
**Section:** §1 (Coverage scope catalog entity `:39` · untestable zone `:44` · test data strategy `:71` · Critical Path 2 `:76` · Critical Path 7 `:81` · the cross-surface trigger `:94`) · §2 (deterministic-tier exceptions `:124` · directory pattern `:128`) · §3 (`run` scenario invocation `:151` · test selection `:154` · CI stage selectors `:155` · bootstrap mechanism `:201` · bootstrap phase `:214`) · §6 (Fingerprint-storm verification signal `:335` + NEW sibling leg `:336` · skip note `:309` · Both-surface parity signal `:375`) · §7 fixture-files row `:395` · §9 Live-Pulse paragraph `:466`
**Change:**
- §3 `--live` gains an optional selector: bare = the suite; `--live real-model` = `preconditions --for real-model-interpretation` → capture pre-build → non-recursive clear of `rm.jsonl` / `rm-capture.{txt,err}` → the rule record BEFORE the leg → the leg → the capture (`RUST_LOG` removed) → reminders; unknown selector exits 2 in both shells. §9: drives only `real_model_live`, never `a11y:driven`; fired once; pickup never budgeted; formation AND pickup unmeasured; `live-pulse` gains `real_model_live.rs`. §2: a real-wall-clock exception.
- §6 real-model harvest leg: a sibling of Fingerprint-storm (P-018), no eighth scenario, no §9 row, graded in `real_model_harvest.rs` against the pre-drive rule (`Identified → (Pass, Pass)`, `NotIdentified → (CalibrationRegion, ManualCheck)`, nothing to grade → `(null, Blocked)`); the 2026-09-23 drive: `NoAttributableIncident → Blocked`. P-018 scenarios: 1 → 2.
- Critical Path 7, the cross-surface trigger and the §6 parity signal scoped to DETERMINISTIC-posture scenarios; real-model is outside parity by design (only headless `conductor run` evaluates its posture), narrowing the Creator Brief Must-Work.
- "one-per-P-ID" retired at six sites: a P-ID may be named by several scenarios (P-017 · P-019 · P-020 · P-021 · P-022 · P-060, now P-018). `run <P-ID>` / `SCENARIO=<P-ID>` resolve to the first in directory order (`find_by_pid`); name the scenario to be determinate.
- Critical Path 2 and the §6 fingerprint-storm signal: at `83d4060` `fingerprints` carries each incident's triggering-cue fingerprint (payload-varying) beside the constant `det-*` refs; no shipped check keys on it.
- "encrypted `corpus.db` (P-049)" → plaintext; §1 drops a quote security-plan no longer carries.
**Why:** Four operator decisions (2026-09-23): narrow parity; correct P-ID and route; amend fingerprints as measured; fix plaintext now. A `run` refusing an ambiguous P-ID is route-owned (CARRY).
**Kept:** `run P-032` (one scenario). Unowned here: §6 still says `retrieve_report` is "permanently `degraded_mode`" under deterministic L4, a universal retired 2026-09-06.
**Ref:** .andromeda/runs/2026-09-23T08-03-55-wrap/

## 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected — Vector 1 gains the test-binary reader class
**Section:** §1 Test Scope Summary → Coverage triggers → security-vector-coverage, Vector 1 (`:87`)
**Change:** Added a FIFTH per-reader class after `CONDUCTOR_A11Y_STRICT`: the TEST-binary readers of `CONDUCTOR_RUNS_DIR` (`journal_conformance.rs`, `real_model_live.rs`, `live_suite.rs`) plus the `ANDROMEDA_PULSE_DATA_DIR` value read in `real_model_live.rs::pulse_log`, resolved through the shared `conductor-run/tests/capture_paths` guard, with the default-suite target `capture_paths_guard` (8 cases, both runners) as their mandated negative test.
**Why:** Additive coverage: the row enumerates each reader class of the Vector-1 handles and its mandated coverage, and this chunk added a guarded reader class and its test. No stale claim was corrected.
**Ref:** .andromeda/runs/2026-09-23T20-49-35-wrap/

## 2026-09-24-secret-scanning-ci-gate — the repository-hygiene gates join the static-gate family by kind
**Section:** §4 Unit Test Strategy (the Scenario catalog + coverage-matrix bullet) · §6 the static-gate scenario (new Repository-hygiene legs bullet) · §9 and §10 Build failure conditions
**Change:** §4 keeps the "static gates over committed data" definition and adds the second kind, the crate-local repository-hygiene gates (`secret_scan_gate.rs`, `workflow_env_gate.rs`), named by kind, never by a count. §6 adds a sibling bullet for the two legs: their `nextest --test` commands, step names, presence guards, no `continue-on-error`/`if:`, the exact-set allowlist, the never-echo hit shape, the `GITHUB_ENV context probe` pair, in-suite fail-proof on inputs built at test time with no committed fixture, and no §9 stage-table row. Both build-failure lists gain the hygiene-gate and probe-step line.
**Why:** The literal rule and arm counts were dropped under the name-the-set rule: the gates are named as a set, never counted. The build-failure lines were owed because the probe's assert step is a shell failure, not a nextest failure, so the existing "any test fails" line does not cover it.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model leg is a pre-stated series, with the diagnostic-quality grades
**Section:** §2 Test Strategy (the determinism exceptions) · §3 5-command implementation (`--live real-model`) · §6 Real-model interpretation leg · §9 Live-Pulse scenarios
**Change:**
- The single-drive claim is retired at every site. It was:
  - §9: "fired once and never re-driven (only a Conductor-side environment fault may be re-fired)".
  - §6: "operator-gated, fired once".
  - §2: "fired once and graded".
  - §3: "so the one drive is never spent".
- Now the selector is driven only as the pre-stated series of `contracts/pulse-real-model-leg-posture.md`:
  - The series is fixed before its first drive; every drive is recorded and never replaced.
  - Only a Conductor-side environment or pipeline fault is re-fired, and only once.
  - Each counted drive is graded against a rule recorded before it fires; §9's `rule_record` precedes each drive's leg.
- §6:
  - It names the P-ID set: P-018 · P-031 · P-033 · P-034 · P-044.
  - It records the P-031 `structure`, P-034 `steps`, P-044 `retrieval` and `canary:` grades, appended below the recorded 2026-09-23 rule (asserted to be a byte-exact prefix), and every committed capture pinned byte-equal.
  - It records the 2026-09-29 outcome: five counted drives, one graded `NotIdentified → (CalibrationRegion, ManualCheck)`, `v3-09` not met.
**Why:** overseer ruling D1 (founder-delegated) replaced the single drive with a fixed, pre-stated series; the chunk added the three grades and extended the harvest.
**Kept:** §9's "real-model formation AND pickup both still UNMEASURED" stands: the report records no formation or pickup figure for the series.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/

## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — digest pins, the fifth-class read, the moving tally
**Section:** §1 Test Scope Summary (desktop-webview driver; the fifth env-handle class) · §6 E2E Test Strategy (the real-model leg; the surface/driver table) · §7 Test Data & Fixtures (the provenance SET)
**Change:**
- §6 real-model leg:
  - Every committed capture is held by a sha256 digest pin (the test-only `sha2`) and graded from the file after the digest matches (was "pinned byte-equal to its section").
  - A tamper arm and a no-capture-text source arm join, as do the workspace-key mask arms.
  - The graded 2026-09-23 capture is an elided copy; the frozen 2026-09-22 file keeps its prefix.
  - The 2026-09-30 series graded none, so `v3-09` stays not met.
- §1 fifth class: `real_model_live.rs` reads `ANDROMEDA_PULSE_DATA_DIR` by `pulse_log` and now also by `workspace_key()`, both through the same guard.
- §1 and §6 driver: CI's routine arm reads "0 failing, only the expected-skip SET skipped (run 35208593666, and since, e.g. CI#36681853843)" (was "12 passing / 0 failing / 2 skipped").
- §7: the real-model harvest's committed capture EVIDENCE (a chunk's `evidence/`, read-only, digest-checked before grading) is named beside the fixture family.
**Why:**
- The chunk replaced the literal pins with digest pins and added one spec to the routine arm (tally 12 → 13 on the dev host and in CI#36681853843).
- The set form follows this plan's own moving-count rule at §6.
**Kept:** the "11 passing / 1 failing" probe record, the "(12 as of 2026-09-07)" moving-count record, and the coverage-matrix byte-equality at §6.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/

## 2026-09-30-mutation-gate-grades-every-tally-it-rests-on — the gate grades every tally; mutation runs at the epoch-boundary audit only
**Section:** §4 Unit Test Strategy — Mutation instrument · §7 Test Data & Fixtures — the committed fixture family and its meaning pin · §9 CI Integration — Scoped mutation audit · §10 Quality Gates — Mutation-survivor disposition · §12 Test Decisions Log — the `conductor-emit` member and Roster member IDENTITY
**Change:**
- §4: the gate grades EVERY tally, in order — the four tally files present (absent fails closed), each parsed count equal to `outcomes.json`'s, `missed + caught + timeout + unviable == total_mutants`, then `missed.txt` and `timeout.txt` as multisets over `(file, mutation)` against the roster rows of that required `tally` class. `unviable` is counted, never rostered. A `selftest` verb proves the grading over `scripts/fixtures/mutation-gate/` with no mutation run (was: compares `missed.txt` alone against the unit's expected SET).
- §9: the scoped mutation audit runs at the epoch-boundary code audit ONLY, never per chunk (was: per chunk, against the crates that chunk touched). §10 names the audit as the run that dispositions survivors, and says the gate grades `missed.txt` and `timeout.txt` by each row's `tally`.
- §12 `conductor-emit`: the fifteen timeouts have no roster rows yet. The gate can hold them since 2026-09-30, but they cannot be keyed from the ledger, so the epoch-boundary code audit seeds them from its measured `timeout.txt`. Until then `mutation-gate.py conductor-emit` fails closed, one `TIMED OUT, not in roster` line per timeout (was: rostered nowhere because the gate never read `timeout.txt`; that is kept as dated history).
- §12 identity: rows match `missed.txt` or `timeout.txt` per `tally`. A zero-row unit passes when every graded tally agrees (was: on an empty `missed.txt`). Unrostered members gain rows when the epoch-boundary audit re-runs their crate (was: when a chunk touching it re-runs the tier).
- §7: the fixture family is the SET of committed fixture trees, now including `scripts/fixtures/mutation-gate/`, its first member outside a crate. Its meaning is pinned by `selftest`, an operator-local Python entry (was: crate-local trees only, each pinned by a Rust round-trip).
**Why:** the chunk made the gate grade all four tallies and proved it on fixture tallies. The founder ruled at take-up that mutation testing runs at the epoch-boundary code audit only, and the overseer (founder-delegated) named that audit the owner of seeding conductor-emit's timeout rows. Timeout identity collides within a helper, so a roster row rests on a measured tally, never on a reconstruction from the ledger.
**Ref:** .andromeda/runs/2026-09-30T08-33-23-wrap/

## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the driven arm is one live run over its own two-scenario catalog
**Section:** §2 Agent-runnable invariants (Deterministic) · §6 desktop-webview row (driven arm · SR firing-form clause)
**Change:**
- §6: the driven arm is ONE live run over a harness-seeded catalog — `wdio.conf.ts` re-creates `runs/driven/scenarios` clean per run, copies exactly the two `[[checklist]]` scenarios (`halo-breathing-encoding`, `halo-hue-encoding`), sets `CONDUCTOR_SCENARIOS_DIR=runs/driven/scenarios` beside `CONDUCTOR_RUNS_DIR=runs/driven/runs`; two holds behind ONE canary. Real keypresses: Ctrl+Enter from a coverage-matrix row starts it, ArrowDown/ArrowUp while live, hold 1 trap / containment / Space / `role=status` / Escape→NoGo / restoration to the invoking ROW (by P-ID), Ctrl+. stops it, hold 2 Ctrl+Enter proceeds; decisions graded from the backend log's added lines (one `: No-Go (`, one `: Go (`, `run aborted by the operator`), never from the dialog closing. (Was: trap / containment / Space / announce / Escape→NoGo / restoration only.)
- §6 SR clause: "ONLY `sr-empty` and `sr-error` carry a `scenarios` field" is scoped to the SR suites — the driven suite also sets it since 2026-09-30.
- §2: the driven arm's wall-clock sample is `1 passing (4m 2.1s)` for the two-hold run, measured 2026-09-30, a dated sample, not a bound (was "~54s").
**Why:** one run with two holds carries all four shortcuts without a second canary (back-to-back runs dedupe on the open canary incident).
**Kept:** the 15-minute mocha ceiling; the live `sr` subject's shell-supplied `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`.
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/

## 2026-09-30-the-sr-cause-isolated-on-this-host — the 153 control pair joins the measured pair set
**Section:** §6 E2E Test Strategy → Drivers per surface → the desktop-webview (Tauri 2 bundled webview) row, the measured working PAIR set
**Change:** The Passing pair set gains {WebView2 Runtime 153.0.4234.48 × msedgedriver 153.0.4234.48}: the dev host's `sr-empty` leg on 2026-09-30, a one-off CONTROL. The runtime was selected per command with the loader's `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` over the on-disk Evergreen folder (no committed reader); the driver was operator-supplied on `CONDUCTOR_MSEDGEDRIVER` and Authenticode-verified before it executed; the session was created and its focus rows graded `not-announced` as on 154. The founder's ratification is quoted in the body. The set stays named as a set of pairs; "a same-major driver is the supported pairing" holds.
**Why:** The harness drove a session on a pair the row did not list. The runtime selection is a founder-ratified control crossing (ratified live, relayed by the overseer), recorded as a dated one-off, never a standing arrangement.
**Kept:** No arch registration of the loader variable: the operator (founder-delegated) ruled the arch registry carries standing committed readers only; a successor that commits a reader registers it then.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/

## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — the browse-mode zone narrowed; the SR firing form gains the OS key path
**Section:** §1 Test Scope Summary → Untestable zones (screen-reader browse mode); §6 E2E Test Strategy → desktop-webview row, the screen-reader family
**Change:**
- §1 was "screen-reader BROWSE-MODE reading … 14 rows `not-run-here` … not a missing driver but a missing key path — OS-level key injection would make these rows agent-driven (a route-owned CARRY)". Now the zone covers only browse rows whose window carries NO OS-level key:
  - since 2026-09-30 the `sr*` legs send Tab, Shift+Tab and `h` / `d` / ArrowDown through `send-keys.ps1` (`SendInput`, foreground-guarded), so every browse row driven by an OS key is agent-graded (`input` = `os`);
  - the rest stay `not-run-here` on the operator arm, named as a set, not a count;
  - injected keys still never deliver browse commands.
- §6 was "the browse-mode rows stay findings pending OS-level key injection". Now the rows it drives with an OS key are agent-graded and the rest stay findings. The firing form gains a per-key `send-keys.ps1` (`powershell.exe -File`, a closed `-Key` constant, sent only while `conductor-tauri` holds the foreground, exit 4 = nothing sent, a refusal throws with no injected fallback). The reset cycle and every other key stay injected, and each row records its input path.
**Why:** the key path the zone named as its remedy now exists and was measured on all three subjects (every OS browse key in NVDA's `Input:` log).
**Kept:** the zone itself, narrowed rather than deleted; the `focus` / `live` agent-driven sentence.
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/

## 2026-09-30-the-screen-reader-content-findings-fixed — the browse zone empties; T-01's quiet window and the SR clock calibration
**Section:** §1 Untestable (browse-mode zone) · §2 Deterministic bullet (the `sr*` real-wall-clock exceptions) · §6 E2E desktop-webview row (the screen-reader leg) · §11 E2E no-sleep ban
**Change:**
- §1: the zone's definition kept; it holds NO row as of the 2026-09-30 regrade (`not-run-here 0` over 51 rows).
- §2: the live `sr` suite's real-wall-clock list gains a 170 s quiet window inside the walk before T-01's second, un-stopped run (a Pulse-side precondition, not synchronisation); the grading timeline calibrates NVDA's log clock per session from the stimulus pairs.
- §6: every browse row now drives an OS key (a bounded walk keyed on speech, a miss recorded, never thrown); the record is graded with the per-session clock calibration — only a stretch more than 300 ms over the session baseline is shifted back, a non-step stretch voids the session, the calibration rides each subject's `clock` with its pair table; known limitation: one slow key send can cross 300 ms alone (357 ms on one key, no grade changed). Was "one preflight canary" for the live subject; now TWO — the stopped run, then T-01's second run after the quiet window; spec wall time `00:05:58` a dated sample, not a bound.
- §11: the carve-out also names the live `sr` leg's in-session 170 s quiet window (the same Pulse dedupe class, a guard throwing if the run settles before its hold); the ban on sleeping to synchronise unchanged.
**Why:** the regrade drove every browse row and T-01; the live leg measured NVDA's log clock stepping ~2.5 s ahead mid-session, moving rows onto their successors' speech; the operator ruled the in-session quiet window the same class as the `--live` suite's and recorded the 357 ms limitation.
**Ref:** .andromeda/runs/2026-09-30T20-18-56-wrap/

## 2026-09-30-full-gate-regression-over-the-moved-surfaces — an ambiguous P-ID target is refused
**Section:** §3 (`run` scenario invocation · Test selection)
**Change:**
- `run <P-ID>` (and the harness's `SCENARIO=<P-ID>`, passed straight through) REFUSES a P-ID that several scenarios name, before any scenario load: a harness fault naming the P-ID, the count and the matching scenario stems (never a host path), `error:` + `hint: pass one of the named scenarios instead of the P-ID`, exit 1, held by `cli_smoke::run_refuses_a_p_id_named_by_several_scenarios`. A single-owner P-ID still resolves; seven P-IDs are multiply named at 2026-09-30.
- Was: "`run` (and the harness's `SCENARIO=`) take the first scenario naming the P-ID in unsorted directory order … making `run` do the same is route-owned (CARRY)", per "2026-09-22-interpretation-proven-live — the real-model selector and harvest leg, parity scoped to the deterministic posture, and three stale claims corrected"; Test selection's "selects determinately only where a single scenario names it" retired with it.
**Why:** The route CARRY landed: a silent first-match pick is a guess, and `preconditions --for` already refused a P-ID for the same reason. The refusal narrows what the CLI admits; overseer direction at P5 recorded it needs no founder word, and it is named operator-visible.
**Kept:** every "a P-ID may be named by several scenarios" site (`:39`, `:71`, `:128`, `:201`, `:396`) — still true; `:124`'s 50 ms utterance-to-stamp window — the parser is unchanged.
**Ref:** .andromeda/runs/2026-10-01T00-02-40-wrap/

## 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix — the 2026-10-01 series and its skip witness arms
**Section:** §6 E2E Test Strategy → Scenario: Fingerprint-storm → Real-model interpretation leg
**Change:**
- The dated series list gains the 2026-10-01 series: three drives on one fresh letters-only data dir against Pulse `a2addb3`, the rule byte-identical; two graded — d1 `Identified`, d3 `NotIdentified`; d2's scenario spans refused by Pulse (`append_failed`, its span identity replaying d1's inside Pulse's buffer window) — so `v3-09` stays not met. Beside the verdict, never softening it: the real model surfaced 6 of 6 canary digests and the scenario's digest both times it formed.
- The synthetic arms gain, since 2026-10-01, the pairing of Pulse's `interpretation.incident.skipped` line: its `skip_reason` rides each `canary:` line as a trailing field, recorded and never graded (the rule reads the first token).
- The residual clause gains the 2026-10-01 d3 capture's all-digit `fingerprint_hex` prefix (the canary's synthetic storm), counted exactly by the harvest — overseer-ruled, founder ratification pending.
**Why:** the chunk ran the third pre-registered series and extended the capture outside the rule markers. The list names the set by its dated entries, never a total count that would re-stale.
**Ref:** .andromeda/runs/2026-10-01T20-39-22-wrap/

## 2026-10-01-per-run-span-identity-in-the-real-model-harness — per-execution span identity and its two-tier proof
**Section:** §2 Test Strategy (operator-local real-wall-clock list) · §4 Unit Test Strategy → conductor-emit · §6 E2E → Real-model interpretation leg · §7 Golden artifacts row · §8 What to mock → Random sources · §9 Live-Pulse scenarios (the `live-pulse` gated SET) · §11 E2E (the SUT-side quiet-window class)
**Change:**
- §6: the 2026-10-01 d2 replay is repaid. The production path re-keys every trace export's span identity under `execute_scenario`'s `std::time` salt (`Dispatcher::connect(…, Some(_))` → `conductor_emit::rekey_trace_identity`), and content stays a pure function of seed. Two tests hold it:
  - `dispatch_wire`'s `two_same_seed_drives_inside_one_window_share_no_span_identity`, which goes RED with the re-key removed;
  - live, the `span_landing_live` witness: 253 507 ms apart inside Pulse's 600 s retention, zero refused appends.
- §2 and §9: the operator-gated `span_landing_live` witness joins the operator-local real-wall-clock list and the `live-pulse` gated target SET. It is auto-discovered, carries its own clippy line, mints no harness verb, and is never a CI leg.
- §11: the witness pass's 180 s window between its two same-seed drives joins the SUT-side quiet-window class.
- §4: the conductor-emit unit tier names `rekey_trace_identity`: a pure per-salt bijection that preserves linkage, keeps an empty id empty and moves no content byte.
- §7: the `dispatch_wire__*` goldens drive the UNSALTED dispatcher, the seed-pure identity tier; a salted test passes a fixed salt.
- §8: the identity salt is injected (`identity_salt: Option<u64>`: `None` / a fixed `Some(N)` in tests, `emitted_ms` in production). The primitive sits at the unit tier and the two-drive property at the dispatcher integration tier.
**Why:** the chunk ships per-execution span identity with its CI and live proofs. These are the plan's three expected test-plan amendments, all carried.
**Kept:** `:328` "same-seed re-run yields identical stream shape" (shape stays seed-pure) · §12 `:616` (Decisions Log history).
**Ref:** .andromeda/runs/2026-10-01T23-55-00-wrap/

## 2026-10-02-p-075-assert-round-against-pulse — the P-075 round leg and graders; `degraded_mode` per-read-back
**Section:** §2 Test Strategy → Deterministic · §5 → `mark_incident_resolved` · §6 Fingerprint-storm → Verification signal · §9 → Live-Pulse scenarios
**Change:**
- §2:
  - The real-wall-clock operator-local exception list gains `p075_round_live` (2026-10-02). It drives one `canary_spec` storm against one live Pulse and reads back through the four registered MCP tools, resolving through the shipped `probe_resolve_lifecycle`. It compares the emitted fingerprint in-process, prints only integers, booleans and closed words, and adds no `agent-run` verb or selector. Its grades sit at the harvest tier over digest-pinned evidence (`p075_round_assertion_*`).
  - The `span_landing_live` description gains its `runs/span-landing/` journals, cleared by name before drive A, and its stale-pair refusal.
- §5: `mark_incident_resolved` was re-graded at Pulse S `03ec944`, giving `ProvenByLiveness` again (idle 12 ms) through the unchanged probe and `attribute_by_liveness`. This is held by `lifecycle_harvest::p075_round_assertion_2_runtime_state_fidelity`.
- §6: the token checks stay declare-only.
  - Was: "because `retrieve_report` is permanently `degraded_mode` in this mode". Now: retired 2026-08-18, when the read-back was degraded. `degraded_mode` is per-read-back and never mode-wide.
  - Evidence: an incident read back `false` under deterministic L4 at S (`evidence/p075-leg.txt`).
  - Added: since 2026-10-02, `lifecycle_harvest::p075_round_assertion_1_read_back_content_fidelity` grades the emitted fingerprint's membership in `fingerprint_refs`.
- §9: the `live-pulse` gated SET gains `conductor-run/tests/p075_round_live.rs` under the same clippy line. It is named as a set, with no count literal.
**Why:** the round measured `degraded_mode: false`. That falsifies the mode-wide clause, which tests history had already flagged stale and which arch's per-read-back clause contradicted. The chunk also added a new gated live leg and its graders.
**Kept:** :323 / :324's degraded read-back is the CI stub path of `error-baseline-spike`, which is not a live-mode claim. :284's "DECLINED arm stub-ONLY and permanently so" is unrelated. No §9 or §11 site stated the span-landing input path.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/

## 2026-10-02-captured-fingerprint-values-elided — the real-model leg's fingerprint residuals fixed; the population arm
**Section:** §6 E2E Test Strategy → Fingerprint-storm → Real-model interpretation leg
**Change:**
- Was: the frozen 2026-09-22 file keeping its prefix as a stated residual, and the 2026-10-01 d3 capture's one all-digit `fingerprint_hex` prefix that the harvest counts exactly, overseer-ruled, founder ratification pending (per the 2026-10-01 entry "the 2026-10-01 series and its skip witness arms").
- Now: the frozen file was elided in place on 2026-10-02 (a recorded exception) and is byte-identical to the graded copy.
- `elide_fingerprints` elides every `fingerprint_hex=` value whatever its class. The d3 prefix was re-elided with its pin moved, and the 2026-10-01 arm asserts zero for every drive.
- A population arm (`every_committed_capture_carries_no_un_elided_fingerprint_value`) holds every committed `rm-capture*.txt` at zero un-elided keyed values and a fixed point of the elision, the population pinned by count, beside its inverse control (`un_elided_keyed_values_counts_a_planted_value`).
**Why:** The founder ruled the residuals FIXED, never ratified (2026-10-02, relayed by the overseer). An exact-count residual pin is a brittle fixture assertion, so it is re-expressed as an observable property (zero), never re-pinned. The population arm was observed red on the untouched captures, naming exactly the two residual files, before the fix.
**Ref:** .andromeda/runs/2026-10-02T16-24-28-wrap/

## 2026-10-03-p-075-re-round-on-incident-events — the pinned tool set named, and the incident-events read's tiers
**Section:** §1 Test Scope Summary → ipc-internal (MCP read-back client) · §2 Test Strategy → Deterministic (the `p075_round_live` clause) · §5 Integration Test Strategy → Cross-module patterns covered · §6 E2E Test Strategy → desktop-webview driver row (the driven arm's firing form)
**Change:**
- §1's RAW-result schema assertion was over a four-name list; it is over the pinned required-tool set (`required_tools` = `READBACK_TOOLS`), which the `retrieve_incident_events` read joined 2026-10-03.
- §2's `p075_round_live` leg was "read back through the four registered MCP tools (the resolve through the shipped `probe_resolve_lifecycle`)"; it now reads back through the pinned set, reading the incident's lifecycle events before and after a resolve that runs through `probe_resolve_lifecycle_timed` (the shipped probe delegates to it), with window-relative nanosecond offsets on stdout; its harvest ids are `p075_round_assertion_*` (Pulse `03ec944`) and `p075_reround_assertion_*` (Pulse S2 `cdb6c1e`).
- §5 gains a `retrieve_incident_events` bullet: the stub tier (raw S2 keys in order; empty `events` reads `total: 0`; an unknown id is a typed `VerifyError::JsonRpc`, never `Ok`), the missing-tool preflight arm on the existing precondition (in-process and against the child stub), the operator-gated live leg, the harvest-tier assertion-7 grader with its synthetic arms, and the two inline `ResolveWindow` unit tests.
- §6's bare-invocation sentence was "all four tools `absent`"; it is "every pinned required tool `absent`", its 2026-09-01 date kept.
**Why:** The pinned set grew to five this chunk; naming the set keeps these sites from re-staling, and the new read needed its tier recorded.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/

## 2026-10-04T01-45-46-wrap — registry migration (U35): the test-plan Decisions Log leaves the body
**Section:** ## 12. Test Decisions Log · ## 4. Unit Test Strategy · ## 9. CI Integration · ## 10. Quality Gates & Coverage Targets
**Change:** the log moved verbatim to test-plan-amendments-archive.md (2 entries: `2026-06-14` initial plan, `2026-08-20` mutation instrument adopted). Four lifts:
- §4, after **Mutation instrument:** — roster member identity (mutation DESCRIPTION joined on `member`, never `file:line:col` or function + column; multiset because two members may share a `(file, mutation)` pair), per-gated-unit population (`member-1`/`-2`/`-3` carry no roster row until an epoch-boundary re-run), `[[unit]]` registration with an optional `-f`-scoping `files` list, zero-row units passing, and a 0-mutant run an explicit FAIL.
- §4, after **Coverage target:** — no frontend unit runner is adopted for the React webview logic (GUI convenience-only, CLI the release gate); frontend unit coverage, if ever needed, starts from re-run tool research. This is the decision the plan's `§12` citations name.
- §9, after **Scoped mutation audit:** — the supply-chain gate is the bare `cargo audit` beside `cargo deny check`, never `--deny warnings`, which exits 1 by construction against the `deny.toml`-adjudicated allowed set; the tool floors belong to the security plan.
- §10, after **Mutation-survivor disposition:** — the accepted-deliberate roster's citation home (§10 named §12 for it), with each member's set, cited rule, rejected alternatives and evidence pointer: `member-1` (`declares`), `member-2` (the `conductor-tauri` triple), `member-3` (`conductor-run` classes B and C), `member-4` (`sidecar_resolves_on_path` pair), `member-5` (tty-colour wrapper pair), `member-emit-1` (the equivalent `quantile` mutant, plus the rule that its timeouts carry no rows and the gate fails closed until the epoch audit seeds them).
Already in the body, not lifted: the Minimal tier and its justification (§1), cargo-nextest + doctest fallback and runner portability (§4), the per-surface drivers incl. the hand-rolled MCP stub and the tauri-driver platform set (§2/§6), cargo-llvm-cov + `--fail-under-lines` (§4/§10), trigger tooling and optional turmoil (§2/§8), the cargo-mutants registration, run discipline and epoch-boundary-only footing (§4/§9/§10).
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-10-04T01-45-46-wrap/

## 2026-10-04T01-45-46-wrap — citations of the retired Test Decisions Log re-pointed to the body
**Section:** §1 Test Scope Summary (the Vector 1 trigger) · §4 Frontend unit runner · §4 conductor-tauri/ui · §10 Mutation-survivor disposition
**Change:** the body's `§12` citations now name the sections holding that truth after U35. The no-frontend-unit-runner decision is §4 Frontend unit runner — was "(§4, §12)" in §1's Vector 1 trigger and "§12's decision … §12 Decisions Log" in §4's conductor-tauri/ui entry, and the lift's own "the decision this plan's `§12` citations name" clause is dropped. The accepted-deliberate roster's citation home is §10's Accepted-deliberate roster paragraph — was "§12's mutation-instrument entry"; "§12 carries the reasoning" now reads "the citation home carries the reasoning". A blank line now separates the roster paragraph from the disposition paragraph above it, and the Supply-chain audit form paragraph from the Scoped mutation audit paragraph.
**Why:** U35 moved §12 verbatim to the archive, leaving a stub; a `§12` citation would name a stub, and the roster citation would contradict the lifted §10 home (the founder-delegated overseer directed the re-point).
**Ref:** .andromeda/runs/2026-10-04T01-45-46-wrap/

## 2026-10-04-real-model-test-surface-corrective — the feature-gated lint lines bundled into `run`, and the secret-scan skip where no `.git` exists
**Section:** §3 → 5-command implementation (`run` command body) · §9 CI Integration → Lint row · §9 → Live-Pulse scenarios · §6 E2E Test Strategy → Repository-hygiene legs · §4 Unit Test Strategy → Mutation instrument
**Change:**
- §3 → 5-command implementation: the bundled `run` body was nextest → doctest → workspace clippy; it adds `cargo clippy -p conductor-verify --features stub-server --all-targets -- -D warnings` and `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` after the workspace clippy, in both shells (compile + lint only, never run; CI reaches them through the dogfood step).
- §9 Lint row names the two feature lines beside the workspace clippy.
- §9 Live-Pulse scenarios: "out of default `nextest` / `clippy` / release builds" and "invisible to the default lint pass … `preflight_spawn.rs` is the in-repo precedent" now read the workspace `clippy` pass; both owed lines run in the bundled default (as measured at CI#37201730301), the gated targets are still never RUN in CI, and a new feature owes its own line in the bundle.
- §6 Repository-hygiene legs (secret-scan leg): the gate skips with one path-free line where the workspace root holds no `.git` entry, decided by `Path::exists` before any spawn; a listing failure inside a repository still fails it; both arms named.
- §4 Mutation instrument (an addition): `conductor-core`'s tier no longer needs `--copy-vcs true` — measured by a `.git`-less copy run, never a mutation run; the next epoch-boundary audit confirms it.
**Why:** the CARRY (Epoch 5 diagnosis P14(b)), widened by the overseer to both feature sets, puts the gated targets under a gate CI reaches; the secret-scan gate panicked in cargo-mutants' default copy and left `conductor-core` unmeasurable (the Epoch 5 code audit).
**Ref:** .andromeda/runs/2026-10-04T12-32-24-wrap/

## 2026-10-04-second-test-surface-corrective — the bundled default stops at its first red line in both shells
**Section:** §3 → 5-command implementation (`run` command body; exit code semantics); §1 → 5-command requirements (`run`)
**Change:**
- `run` body: the bundled default stops at its FIRST non-zero cargo line in both shells — `agent-run.sh` by `set -euo pipefail` (as measured: a planted clippy-only `-D warnings` lint ended `run` at exit 101 on the workspace clippy line, after green nextest and doctest lines); `agent-run.ps1` by an explicit `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` after each of its five cargo lines (since 2026-10-04). It no longer depends on its caller setting `$PSNativeCommandUseErrorActionPreference`, and the script never sets it. A caller that does set it (CI's dogfood step) still sees the red line throw first (step exit 1, CI#34689135760); one that does not gets that line's own cargo exit. The ps1 green path is proven by CI#37209452847; its red path is read from the script, unmeasured on the Linux dev host (no `pwsh`).
- Exit code semantics (key file and §1): was "non-zero = at least one hard `Fail`"; now also a red build, doctest or `-D warnings` lint line in the bundled default, which ends the run at that line (measured exit 101 with every test green). The reported-states clause is unchanged.
**Why:** before this chunk the ps1 bundled default blocked on a red cargo line only when its caller set the native-command preference; the measured sh behaviour already contradicted the hard-`Fail`-only exit wording.
**Kept:** the `--e2e` capture-then-print caveat (a caller setting the preference preempts the printed verdict) stands — the `--e2e` arm is unchanged.
**Ref:** .andromeda/runs/2026-10-04T14-44-02-wrap/

## 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09 — the real-model leg records the fourth series' verdict
**Section:** §6 E2E Test Strategy → Scenario: Fingerprint-storm → Real-model interpretation leg
**Change:** The bullet's series record ended at the 2026-10-01 series; it now closes with the 2026-10-06 series: three drives in one sitting, Pulse `5f77859`, its shipped model `gemma-4-E4B-it-Q4_K_M`, the rule byte-identical, the first series on the Linux dev host; all three graded — d1 `Identified`, d2 `Identified`, d3 `NotIdentified` — so `v3-09` stays not met (`v3_09_is_not_met_by_the_2026_10_06_series`; no ref test, no fourth drive). Beside the verdict: 6 of 6 classified canary digests and the scenario's digest in all three drives surfaced, and every rank-1 statement carried a retry token. A closing sentence names the series set as the posture contract's dated sections, never a count restated in this bullet.
**Why:** the fourth pre-registered series ran and the bullet's enumeration no longer matched the harvest, which now states a fourth verdict. d3's rank 1 names the retry storm and the service only inside the hyphenated canary identity, which the unchanged rule separates from `conductor`. The founder ruled the same day (his own word, relayed by the overseer) that `v3-09` is neither relaxed nor deferred: Pulse is fixed, then a fifth series runs.
**Kept:** the bullet's arm description is unchanged — it names the host-path-mask arm set, never the mask's roots, so the two temp-root arms needed no edit here.
**Ref:** .andromeda/runs/2026-10-06T21-10-08-wrap/

## 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09 — the 2026-10-07 series' dated verdict
**Section:** §6 E2E Test Strategy → Scenario: Fingerprint-storm, the `Real-model interpretation leg` bullet
**Change:** one dated record added after the 2026-10-06 series' and before the closing sentence that names the series set: the 2026-10-07 series (three drives in one sitting, Pulse `f70be92`, prompt `v2.6`, the same model, the design and the rule byte-identical) graded all three — d1 `Identified`, d2 `Identified`, d3 `NotIdentified` — so `v3-09` stays not met (`v3_09_is_not_met_by_the_2026_10_07_series`; no ref test, no fourth drive). It is the same split on the same drive as the 2026-10-06 series at prompt `v2.5`, which three drives per series do not separate; the real model again surfaced 6 of 6 classified canary digests and the scenario's digest in all three drives, and every rank-1 statement carried a retry token. The 2026-10-06 record stands as written.
**Why:** the bullet's per-series record stopped at 2026-10-06 while the harvest gained the 2026-10-07 series' module and verdict test. A second not-met on the same drive, after the Pulse sentence written for it, is the fact the next reader of this leg needs first.
**Kept:** no count entered the body. The series set is still named as the posture contract's dated sections, and the capture and test populations stay unstated.
**Ref:** .andromeda/runs/2026-10-07T09-46-39-wrap/
