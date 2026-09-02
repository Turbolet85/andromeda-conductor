# Test Plan — Amendments

_Append-only changelog of amendments to `test-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — self-obs stream noted as distinct from the emission journal
**Section:** §3 Test Harness Contract / Log format
**Change:** added a note that the `tracing` self-observation stream (stderr / `logs/agent-latest.jsonl`; per-line base fields incl service-identity + `run_id`, obs-plan §3) is a SEPARATE artifact from the per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth + Run-report envelope) — the two schemas must not be conflated.
**Why:** D-tests-obs-harness fired on the chunk's new flat `service.*` self-obs fields. Validated (main) that those belong to the self-obs stream, NOT the emission-journal envelope test-plan §3 owns — so the bound §3 ↔ obs-plan §3 envelope is unchanged; the amendment is a clarifying cross-reference, not a field addition to the envelope.

## 2026-06-15-design-token-typography-bundle — frontend (ui/) tests build-gated, unit deferred to Epoch 9
**Section:** §4 Unit Test Strategy (What unit tests cover)
**Change:** added a `conductor-tauri/ui` bullet — the React/Tailwind token bundle carries no Rust/nextest unit tests; it is build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke), and frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9.
**Why:** D-tests-coverage (warning) fired on the new frontend code paths having no unit tests; routine per playbook (a Foundation chunk defers a downstream-sequenced concern — §12 Decisions Log already records no JS/TS unit runner adopted, GUI convenience-only). No envelope/harness change; D-tests-framework + D-tests-obs-harness clear. No distillation cascade (tests-summary/testing.md describe the unchanged Rust test strategy).

## 2026-06-16-test-framework-fixtures-coverage-tooling — external-CLI tool versions reframed as floors
**Section:** §4 Unit Test Strategy
**Change:** added a Tool-version policy paragraph — the external-CLI tools (cargo-nextest, cargo-llvm-cov) named in §4 are reference floors (outside `Cargo.lock`; any green-running install satisfies the gate, per the cargo-audit/deny precedent); the crate dev-deps are caret-resolved with `Cargo.lock` authoritative.
**Why:** D-tests-framework (warning) fired — the chunk resolved cargo-nextest 0.9.133 · cargo-llvm-cov 0.8.5 · proptest 1.11.0 · insta 1.48.0 · assert_cmd 2.2.2 · assert_fs 1.1.4 · predicates 3.1.4 · rstest 0.26.1, differing from §4's named pins (0.9.137 / 0.8.7 / 1.9.0 / 2.1.2) while all gates ran green (46/46, 91.97%). Routine per playbook rules #2 (dev-CLI floors) + #5 (spec→sound-impl); reframing to floors stops the detector recurring. predicates was already named in §4 (test-runner-install). Cascaded to `.claude/docs/{stack,tests-summary}.md`.

## 2026-06-16-emission-journal-writer — unit serialization goldens use exact-assert; insta stays the E2E mechanism
**Section:** §4 Unit Test Strategy (conductor-report bullet)
**Change:** clarified the conductor-report golden — the canonical line shape is locked via exact-string `assert_eq!` at unit level (matching the `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern), with insta reserved as the E2E journal-golden mechanism (`run_id`/timestamp redaction, §6/§7).
**Why:** D-tests-framework (warning). The chunk's unit goldens used exact-string `assert_eq!` (dropping insta from `conductor-report` dev-deps), matching the established canonical-serialization golden pattern already in `conductor-core`; insta remains the E2E mechanism (real runs, redaction) and stays in the workspace dev-deps. Routine per playbook rule #5 (spec-illustration → sound-impl alignment; report proves the schema is golden-locked). No distillation cascade (testing.md / tests-summary.md describe insta for CI/E2E goldens, unchanged).

## 2026-06-17-raw-otlp-message-scaffold — OTLP-egress loopback gRPC stub (tokio-stream) registered in the integration mechanisms
**Section:** §2 Test Strategy (the test-pyramid Integration row)
**Change:** added the OTLP-egress loopback gRPC `TraceService` stub — a tonic server over `tokio-stream::wrappers::TcpListenerStream` on an ephemeral `127.0.0.1:0` (NEW dev-dep `tokio-stream`) — to the Integration row's mechanisms (alongside the rmcp stub + Tauri mock + in-memory rusqlite). Never binds the real `:4317` (reserved for the Epoch-4 port-occupier).
**Why:** D-arch-decisions (warning) flagged `tokio-stream` as a new dev-dep absent from any spec inventory; its correct home is the test-plan dev-test stack (where rstest/assert_cmd live), not arch §Stack (which doesn't enumerate dev utils). Registers the gRPC-stub mechanism + crate at the integration-test altitude. Routine; no harness/envelope change (D-tests-obs-harness clear). No distillation cascade — testing.md/tests-summary.md describe the unchanged Rust test strategy; the reusable loopback-gRPC-stub pattern is curated to testing.md §Session Additions (P3).

## 2026-06-24-frameless-window-shell — registered the Tauri-backend self-obs sink logs/conductor-tauri.jsonl
**Section:** §3 Test Harness Contract (Self-obs stream is a distinct artifact)
**Change:** added the Tauri backend's `logs/conductor-tauri.jsonl` to the self-obs stream sink list (was cli `logs/agent-latest.jsonl` only), matching obs-plan §3's dual-surface sink table now that the Tauri sink is live.
**Why:** D-tests-obs-harness (warning). The chunk made the Tauri backend self-obs sink LIVE (report §Changes — `ObsSink::File` → `logs/conductor-tauri.jsonl`); obs-plan §3 already documents both sinks, but test-plan §3 (the log-schema OWNER) listed only the cli sink. Routine §3↔§3 reconcile of the now-live half — distinct from the still-carried dual-RECORD-SHAPE reconcile. No schema change (same per-line base fields; only the sink-location list grew). No cascade — observability.md already references the conductor-tauri.jsonl sink; tests-summary.md is high-level.

## 2026-06-26-live-counter-channel-stream — deferred the Tauri-integration + GUI-parity tests to the GUI test-harness chunk
**Section:** §5 Integration Test Strategy (Read-back / boundary coverage list)
**Change:** noted that the `tauri::test` mock-runtime command/Channel assertion + the GUI leg of cross-surface parity defer to the Epoch-9 `tauri-driver` GUI test-harness chunk; the run logic is covered now at the conductor-run unit tier (drive_run stream+persist+abort + seed-stable Blocked envelope) + the conductor-cli cli_smoke parity E2E (the headless leg of Critical Path 7).
**Why:** D-tests-coverage (warning) — escalated + user-confirmed (2026-06-26, this wrap). The GUI start_run drives the run on a background thread streaming the Channel asynchronously, so an in-process frame-sequence assertion is thread-timing-dependent and would flake against the zero-retry bar; the run logic IS covered deterministically at the unit tier + the CLI parity E2E (byte-identical Blocked envelope post-extraction). The GUI integration leg lands with the sequenced tauri-driver harness chunk. New playbook rule appended to pre-empt re-fire on ch5/ch8. D-tests-framework + D-tests-obs-harness clean (nextest 402 +4; RunRecord envelope UNCHANGED, RunEvent is transient IPC not in the JSONL/obs schema).

## 2026-08-08-sut-capability-manifest — Capability-manifest coverage + de-hardcoded catalog assertions
**Section:** §1 Test Scope (Scenario catalog entity) · §4 Unit Test Strategy · §6 coverage-matrix completeness scenario · §7 fixtures
**Change:** Catalog/coverage assertions re-sourced from the SUT capability manifest instead of a hard-coded 60; capability-manifest loader/validator + membership-rejection coverage recorded (8 new unit tests, workspace nextest 420 → 428).
**Why:** The plan mandated a static assertion over an accepted set the code no longer defines.

## 2026-08-09-current-sut-coverage-classification — Critical Path 6 de-hardcoded; stale arch citation repaired
**Section:** §1 Test Scope Summary → Critical paths (Path 6, coverage-matrix completeness gate) · §1 Testable Entities (Scenario catalog, Source clause)
**Change:** Path 6's verification signal drops "the generated `coverage-matrix.md` enumerates all 60 P-IDs" for a manifest-relative assertion (set-equality against the accepted set, no literal count or P-ID range), and its mode list widens from three to four with `not-Conductor's`. Separately the Scenario-catalog entity's **Source:** clause, which quoted arch Project Intent as "60 claimed capabilities P-001..P-060", now quotes arch's current wording — the citation no longer resolved after arch was de-hardcoded on 2026-08-08. The adjacent Creator-Brief quotation is left verbatim: it cites frozen `input.md:160`, where that wording is still exactly what the brief says.
**Why:** The chunk added a fourth `CoverageMode` and replaced every literal-60 assertion with a manifest-derived one, so the plan's own mandated gate no longer described the code path it governs. The Source-clause repair is the cross-master citation edge of the cascade (a master citing another master's superseded wording), caught by grepping the six other masters for each amended passage's OLD wording rather than from the proposal set. **Deliberately NOT amended:** Path 6's reference to the `coverage-matrix.md` artifact — that file has never existed in the repo though arch §Occupied Resources registers it, a question of whether the artifact becomes real that belongs to the Epoch-6 coverage-completeness-gate chunk; routed to route-resolve as a CARRY on that entry, per operator direction.

## 2026-08-09-interpretation-correctness-posture — mock-runtime command tier live; coverage gate gains a scenario-backing leg
**Section:** §5 (GUI test-harness deferral) · §6 Scenario: Coverage-matrix completeness gate
**Change:** §5's deferral narrowed to the `Channel`-frame-sequence + GUI-parity leg — the mock-runtime command-dispatch half landed with 2026-06-27-desktop-a11y-harness-setup and is now the standing tier for each read-only command (`coverage_matrix`, `run_report`, `unbacked_auto`). §6's coverage gate gains the second-axis scenario-backing leg (exact-set equality in both directions over `UNBACKED_AUTO`; `Auto` only; unit-tier, CI-runnable).
**Why:** the chunk added a mock-runtime IPC test for `unbacked_auto`, which §5 still described as deferred; and it shipped the scenario-backing gate §6 did not describe. Self-raised — no detector proposed the §6 leg (the plan's Expected-amendments list is the coverage floor).

## 2026-08-09-in-lane-sut-scenarios — selector strategy: de-hardcode the P-ID range
**Section:** §6 E2E / Selector strategy
**Change:** the coverage-matrix/report row selector anchor now reads "mono P-ID tokens (the manifest's accepted set)" instead of the literal range "(`P-001`..`P-060`)".
**Why:** this chunk lands the first catalog entries above P-060 (P-067, P-072, P-079), so a selector strategy keyed on `P-001..P-060` no longer covers the coverage-matrix rows it creates. Named the SET rather than substituting `P-001..P-082`, per the de-hardcode precedent established by 2026-08-08-sut-capability-manifest — a new literal re-stales on the next SUT release. Self-raised at wrap validation (no detector covers the class; the fan-out's tests agent observed it and declined to raise it as outside its three invariants) — which is what motivated the two `D-*-derived-count` detectors added to drift-base this same chunk.

## 2026-08-09-sut-load-envelope — cli status-label enumerations name the set + the non-lamp qualifier
**Section:** §1 Surfaces under test (cli Signal) · §6 E2E cli driver row + Selector strategy
**Change:** All three cli status-label enumerations now present the six bracket labels as the closed per-P-ID lamp/report-state set and record `[ENVIRONMENT-SUSPECT]` as a run-level non-lamp qualifier cli selectors must expect.
**Why:** The chunk added a seventh bracket label to cli stdout while the three enumerations read as exhaustive; a selector strategy assuming only six would mis-parse an over-envelope run.

## 2026-08-10-pulse-run-contract — `boot` timeout restated under the run contract's floor
**Section:** §3 Test Harness Contract → 5-command implementation → `boot`
**Change:** the flat `Timeout: 30s` became `CONDUCTOR_PREFLIGHT_TIMEOUT` seconds (default 30) raised to the run contract's effective floor (`contracts/pulse-run-contract.toml` `[incident_formation].min_canary_poll_seconds`, which must outlast Pulse's L3 digest cadence), so the live-Pulse leg must allow at least that floor. The stub leg still completes in <1s because it drives `CanaryPoll::immediate()` — no real clock.
**Why:** the chunk made the floor real in `canary_poll()`, so a spec stating a flat 30s budget now describes a value the code will not honour for the live leg. One-sided by construction: obs-plan §3 carries no preflight budget, and the shared envelope/log-format sides are untouched (envelope still 11 fields, `ReportState` five, `LAMP_META` six). Detector-raised (D-tests-obs-harness).

## 2026-08-10-scenario-run-root-span-tree — the self-obs stream's two line variants recorded
**Section:** §3 Test Harness Contract → Log format → "Self-obs stream is a distinct artifact"
**Change:** the bullet now states the stream carries TWO line variants over its one base set — the event line, and the span-lifecycle line adding `span` (the bounded §4 span name), `span_event` (`new` | `close`), an optional `parent`, and the span's own allowlisted attributes on the `new` line — and that a span attribute is gated by the same `conductor-core::redact` allowlist as any event field. The Run-report envelope and `runs.db` columns are unchanged.
**Why:** this plan is the SOURCE OF TRUTH for the JSONL log format (obs-plan §3 derives), and the chunk changed that format: `JsonObsLayer` gained `on_new_span` / `on_close`, so spans now materialize as lines where previously the layer implemented only `on_event` and no span emitted anything. Detector D-tests-obs-harness — its precondition ("the report changes the log format") is met, so the 2026-06-21 pre-existing-bind dismiss rule does not apply. obs-plan §3 amended in lockstep.
## 2026-08-11-faithful-emission-dispatcher — scenario-config validation entity + trigger restated
**Section:** §1 Entity (scenario-config validation surface) · §1 Trigger (security-vector-coverage / property-test)
**Change:** Both now name the shipped error-fraction encoding (`error_percent` ∈ 0..=100) rather than an f64 `[0,1]` bound, add bounded per-phase occurrences and the per-shape emission rules to the validated set, and require the negative-test suite to catch a nested spec field annotated `skip` rather than `dive` — rules that exist but never execute.
**Why:** Cross-master citation fold from the architecture amendment, plus the chunk's finding that `PhaseSpec.emission` was `#[garde(skip)]`, which no existing negative test could have caught.

## 2026-08-13-dispatcher-determinism-goldens — golden inventory gains the seeded stream families
**Section:** §7 Test Data & Fixtures — Seed strategies, the `Golden artifacts` row
**Change:** The row now names two families: the envelope/journal goldens (unchanged, redacting `run_id`/`journal_emitted_at`/`read_back_observed_at`) and the seeded STREAM goldens committed per-crate under `<crate>/tests/snapshots/`, one file per test-file family × seed (`replay__*` · `pacing__*` · `dispatch_wire__*`). Records that stream goldens have no wall-clock field to redact — the dispatch-tier projection EXCLUDES every `*_time_unix_nano` rather than masking it — and makes the never-`cargo insta review` rule explicit in the row.
**Why:** The chunk committed 4 new stream goldens in two new families (`pacing__fixture_emission_stream_seed_{4317017,7}`, `dispatch_wire__storm_stream_seed_{4317017,7}`) while the row enumerated only the envelope/journal goldens. Stated as the family SET plus the per-family × seed rule rather than a file count, so it does not re-stale at the next seed or tier.

## 2026-08-13-first-live-green-preflight — boot Timeout: two derived budgets
**Section:** §3 Test Harness Contract — 5-command implementation → `boot` (Timeout)
**Change:** Timeout now names TWO budgets, both derived: the in-process canary poll (`CONDUCTOR_PREFLIGHT_TIMEOUT` raised to `[incident_formation].min_canary_poll_seconds`, unchanged) and the wall-clock wrapper `agent-run.{sh,ps1}` `boot` applies, which both shells now derive per invocation as `warmup_ms/1000 + poll + margin`. A missing contract term is a hard exit 2; a lowered env value clamps up to the floor. Stated as the derivation rule, no literal.
**Why:** The wrapper was a fixed 30s in `.sh` and absent in `.ps1`, while the in-process budget is ~135s (45s warm-up + >=90s poll) — the wrapper killed every live run before the warm-up finished, and the two shells disagreed. Predicted as an expected amendment in the chunk plan.

## 2026-08-14-canary-fingerprint-feed-capture — the event-line variant named by set, not by level literals
**Section:** §3 Test Harness Contract → Log format ("Self-obs stream is a distinct artifact")
**Change:** the event-line variant is now described as any `tracing` event record (`message` plus its
allowlisted fields) at whatever level obs-plan §11's policy assigns that call site, rather than as "an
`info!`/`error!` record".
**Why:** the chunk shipped the first `debug`-level self-obs line (the `emit.batch` wire-shape witness, at
`debug` because §11 bans `info` on a hot path), which the old two-level enumeration excluded — a one-sided
test-plan §3 ↔ obs-plan §3 divergence, since obs-plan §3 never enumerated levels. Naming the set rather than
substituting a fresh literal keeps it from re-staling.

## 2026-08-16-canary-fingerprint-derivation-aligned — canary boundary re-aimed to incident freshness
**Section:** §5 Integration Test Strategy (Cross-module patterns covered)
**Change:** Splits the old combined bullet: `query_incident_list` now carries the canary round-trip (an
incident opened after the emission stamp ⇒ proceed; empty corpus, only-older incidents, or a missing stamp ⇒
`blocked`), covered by the fresh / stale / stamp-absent stub legs; `retrieve_telemetry_slice` /
`retrieve_report` move to a separate per-check read-back bullet, which the canary no longer calls.
**Why:** the canary's tool and predicate both changed. The split is deliberate — the tools are still called by
per-check extraction, so removing them entirely would have introduced new drift rather than removing it.

## 2026-08-16-canary-fingerprint-derivation-aligned — ipc-internal Signal predicate updated
**Section:** §1 Test Scope Summary (Surfaces under test → ipc-internal)
**Change:** The Signal clause's "canary round-trip non-empty" becomes "canary round-trip observing an incident
opened after the emission stamp". The pinned required-tool list is left intact.
**Why:** same retired predicate restated in the surfaces table; non-emptiness is no longer sufficient, since a
corpus of only older incidents must block.

## 2026-08-16-canary-fingerprint-derivation-aligned — preflight state mapping widened
**Section:** §4 Unit Test Strategy (What unit tests cover → conductor-verify)
**Change:** "empty canary ⇒ `blocked`" widens to "an empty corpus OR a stale one — no incident newer than the
canary's emission stamp — ⇒ `blocked`".
**Why:** a non-empty but wholly pre-dating corpus now blocks too; the old mapping under-described the
assertion the stub legs make.

## 2026-08-16-fingerprint-storm-live-proof — fingerprint-storm verification signal re-based on the harvest
**Section:** §6 E2E Test Strategy (Scenario: Fingerprint-storm) + §1 Test Scope Summary (Critical paths, the
same restatement)
**Change:** the envelope's `fingerprints` field is PRESENT but expected `[]` under deterministic L4 (fed
solely from `fingerprint_refs`, which the L4 fixture pins empty); the scenario's read-back token checks are
DECLARE-ONLY because `retrieve_report` is permanently `degraded_mode` in this mode; the live proof is the
test-only harvest of Pulse's `triage.pattern.storm.detected` line asserted at the `conductor-run` unit tier.
**Why:** measured on two live legs (2026-08-16) — both returned `fingerprints: []`, `Contains "RetryStorm"`
always failed and `Absent "RetryStorm"` passed VACUOUSLY against text that cannot carry the token. Both
sections restated the identical claim, so a §6-only apply would have left it standing in the critical-paths
table.

## 2026-08-18-error-baseline-spike-live-proof — §6/§1 error-baseline-spike signal re-based; fingerprints [] pin retired
**Section:** §6 Scenario: error-baseline-spike + §1 Critical Path 1 · §6 Scenario: Fingerprint-storm + §1 Critical Path 2
**Change:** The error-baseline-spike verification signal re-based from exit-0/`[PASS]`/`verdict=Pass`/`state=Pass` to the declare-only reality: `[RESIDUAL]`, `verdict` null / `state=KnownResidual`, live claim graded at the harvest tier (`triage.cue.emit` via `baseline_harvest.rs`) — both sites. The `fingerprints` expected-`[]` pin retired at both sites: the field is SUT-populated under deterministic L4 (constant `det-*` triple, payload-invariant, measured 2026-08-18 at SUT HEAD `efabe8e`); the harvest, never the array, stays the proof.
**Why:** Measured on leg A (row KnownResidual; envelope carried the 3 det-* refs); the family's checks retired because no read-back surface can carry them (degraded report renders fixture constants; evidence count reads an unpopulated `span_ids`). Chunk report §Spec claims disproved.

## 2026-08-18-restart-suppression-live-proof — restart-suppression re-based to the harvest tier; harness seed precedence
**Section:** §1 Critical paths (Path 3) · §6 Scenario: Restart-suppression incl. one bypass case · §3 Test Harness Contract (`run` scenario invocation)
**Change:** Path 3 and its §6 twin move from an in-scenario hard pass/fail to the declare-only row (`verdict` null / `state=KnownResidual`, `<90s`) with the suppression/bypass claim graded as hard predicates at the harvest tier (`restart_harvest.rs`, verbatim leg captures; bypass set = absolute arm + suppressed case, relative arm recorded unreachable; `persistence_seconds` = cumulative samples; tick counters redacted live). §3's scenario invocation now states `--seed` rides only an explicitly set `SEED` (TOML-declared seed governs otherwise, `.sh`/`.ps1` parity).
**Why:** Both `[[expected]]` checks were structurally ungradeable under deterministic L4 and retired; the live leg (run 2026-08-18T21-40-46-519) proved every witness at the harvest tier; the harness previously fabricated `--seed 424242` over TOML-declared seeds (report §Symbols/APIs, §Spec claims disproved 1-3; the two-site rule).

## 2026-08-19-pii-scrub-live-proof — pii-scrub re-based to declare-only at both sites
**Section:** §6 Headless deterministic scenario run (Cleanup parenthetical) + §1 Critical paths closing note (the two-site rule)
**Change:** Both sites now split the parenthetical — connection-lifecycle stays an auto-scenario instance of Path 1; pii-scrub is DECLARE-ONLY (all five [[expected]] read-back checks retired, measurement in the scenario TOML header; row lands verdict null / state KnownResidual, non-Blocked) with the live scrub claim graded at the harvest tier in crates/conductor-run/tests/pii_harvest.rs (the storm/baseline/restart_harvest precedent).
**Why:** Report Counts bullet: [[expected]] 5 -> 0; a single-site apply would leave the twin asserting the retired read-back-graded shape (the two-site rule established at the error-baseline re-base). The other two pii-scrub mentions (:62 run-verb list, :148 scenario-invocation list) name it only as an invocable scenario and stay true — verified this wrap.

## 2026-08-19-connection-lifecycle-live-proof — connection-lifecycle re-based auto-scenario → DECLARE-ONLY (two sites)
**Section:** §1 Test Scope Summary (critical-paths closing note) · §6 E2E Scenario 1 (Cleanup parenthetical)
**Change:** Both sites now state the connection family as DECLARE-ONLY instances of Path 1's pattern — zero read-back-graded checks, rows landing `verdict` null / `state=KnownResidual` non-Blocked — with the live walk/conflict claims graded at the harvest tier in `crates/conductor-run/tests/connection_harvest.rs` (the fifth harvest).
**Why:** Measured this chunk (leg A structural fail; legs B1–B4 the shipped shape): connection state reaches no MCP read-back surface, so the family's four Contains checks retired. The two-site rule applied in one pass (the error-baseline precedent).

## 2026-08-20-latency-regression-re-proof — sidecar-spawn citation re-based
**Section:** §Test Anti-Patterns (stack-specific)
**Change:** The cross-master citation of security-plan's sidecar-spawn ban now reads "a fixed hard-coded program NAME resolved through the inherited `PATH`" instead of "a fixed hard-coded path".
**Why:** Cascade edge — test-plan cited security-plan as saying something it no longer says after the 2026-08-20 spawn-wording amendment (the shipped constant is a program NAME, PATH-resolved). The negative-test mandate itself is unchanged.

## 2026-08-20-verifier-self-hardening — cargo-mutants registered + runner portability made a gate

**Section:** §4 Unit Test Strategy (Framework · Mutation instrument · Tool-version policy)
**Change:** Registered cargo-mutants 27.1.0 as an operator/local mutation instrument with its run discipline (workspace-root `-f` paths, mandatory `--test-tool=nextest`, `Found 0 mutants to test` is a NO-OP never a pass, gitignored output) and added it to the floors-not-pins list. Also named `cargo test -p <crate>` a standing runner-portability gate beside nextest.
**Why:** The chunk used the instrument as its acceptance evidence and measured that the package-relative `-f` form reports zero mutants at exit 0 — indistinguishable from a clean run. Runner portability became an acceptance criterion because only the shared-process runner exposes a test relying on nextest's per-test process for isolation.

## 2026-08-20-verifier-self-hardening — scoped mutation recorded as an operator instrument

**Section:** §9 CI Integration (after Live-Pulse scenarios)
**Change:** Recorded the scoped mutation audit on the same footing as the live-Pulse leg — per-chunk, against the touched crates, never a blocking CI stage — and stated that the stage table is CI's complete inventory, not a chunk's.
**Why:** The chunk ran the instrument for acceptance while §9's stage table registers only Lint / Supply-chain / Unit / Doctest / Integration / E2E / Coverage / Quality, so the table could be read as the full set of checks a chunk owes.

## 2026-08-20-verifier-self-hardening — mutation-survivor disposition + runner-dependence as flakiness

**Section:** §10 Quality Gates & Coverage Targets (Zero-flakiness budget)
**Change:** Added mutation-survivor disposition as a non-blocking audit-tier rule — every named survivor ends killed OR classified accepted-deliberate against a cited rule — explicitly not a numeric threshold; and recorded that a runner-dependent result is a determinism break to fix at the cause.
**Why:** The chunk dispositioned 22 named survivors as 16 killed + 6 accepted-deliberate (the `declares` edge, per testing.md 2026-08-10), and the score moved only as a consequence.

## 2026-08-20-verifier-self-hardening — process-global singleton isolation ban

**Section:** §11 Test Anti-Patterns → Integration
**Change:** New ban — never rely on nextest's per-test process to isolate a test from a process-global first-install-wins singleton (`init_observability`); give it its own test binary rather than serializing the file.
**Why:** Measured: the wire-shape witness passed alone and under `--test-threads=1`, failing only alongside its four `emit_canary_storm` siblings — concurrent interference against the global subscriber, which a per-test temp FILE cannot isolate. Serializing was rejected as hiding the defect (§10).

## 2026-08-20-verifier-self-hardening — decisions-log entry

**Section:** §12 Test Decisions Log
**Change:** Added the 2026-08-20 entry recording the instrument adoption, the measured run-discipline findings, the `declares` accepted-deliberate classification, and the runner-portability gate.
**Why:** The tool's adoption and the classification ruling are decisions future chunks must not re-litigate.

## 2026-08-20-read-back-seam-survivors-closed — cargo-mutants exit code carries no verdict

**Section:** §4 Unit Test Strategy → Mutation instrument
**Change:** Recorded the second half of cargo-mutants' exit-code semantics — the exit code carries no verdict in EITHER direction: `Found 0 mutants to test` is a no-op at exit 0 (already recorded), and a NON-zero exit reflects surviving/timeout CLASSES rather than run failure. Gate on the tallies read out of `mutants.out/` (`missed.txt` empty + the named survivors present in `caught.txt`), never on the exit code.
**Why:** Measured this chunk: **exit 3** on a scoped run that fully met its acceptance — 51 mutants, 0 missed, all five named survivors killed — with the non-zero owed entirely to two pre-existing timeouts. §4 previously recorded only the exit-0 direction, which implies by omission that a non-zero exit IS a failure; a reader following §4 alone would have inverted this chunk's verdict.

## 2026-08-20-read-back-seam-survivors-closed — decisions-log exit-code restatement

**Section:** §12 Test Decisions Log → `2026-08-20` Mutation instrument adopted, Run-discipline bullet
**Change:** Extended the run-discipline bullet's exit-code parenthetical to carry both directions, naming the measured exit-3 case and its chunk.
**Why:** §12 restates §4's run discipline and carried only the exit-0 half, so a single-site apply to §4 would have left the one-sided claim standing in the decisions log — the duplicate-occurrence sweep the reconcile pass exists to prevent.
## 2026-08-21-severity-lifecycle-live-proof — severity-lifecycle re-based to declare-only + the harvest tier

**Section:** §6 E2E Test Strategy — Scenario: Severity-lifecycle full pass · §1 Test Scope Summary — Critical
Path 4 (the two-site rule) · §5 Integration Test Strategy — `mark_incident_resolved` bullet
**Change:** the family grades declare-only (five rows, `verdict` null / `state` "KnownResidual", exit 0) with
the live auto-resolve claim asserted HARD at the harvest tier in
`crates/conductor-run/tests/severity_harvest.rs` — on the instant Pulse's active set EMPTIES
(`incidents.list_active.request` `item_count`) against the 120s window + 30s observer tick, plus the
same-fingerprint retrigger reading `created=true, deduped=false`. The §6 Steps line no longer invokes
`conductor run severity-lifecycle --seed <s>`: no scenario carries that name (the family is five separately
named TOMLs) and `--seed` rides only an explicitly set `SEED` per §3. `mark_incident_resolved` no longer
claims a severity-lifecycle resolution-summary read-back — it has no production call site.
**Why:** measured across five fresh-dir live legs at SUT HEAD `efabe8e`. Two witnesses the doc mandated do not
work: `triage.incident.auto_resolve.tick` renders its counters as `"<redacted>"` (proving only that the
observer ran), and P-059's resolution summary is unreachable under deterministic L4 (zero
`DigestKind::ResolutionSummary` constructors; the canned fixture pins `is_resolution_summary` false) — zero
such lines appeared across all five legs.

## 2026-08-21-per-check-latency-measurement — Journal carries two report-seam line shapes
**Section:** Section 3 Test Harness Contract -> Log format (Agent parsing)
**Change:** The journal is recorded as carrying TWO report-seam line shapes — the envelope line (unchanged) and the per-check `CheckRecord` line (9 keys) — and a typed parse must DISCRIMINATE rather than assert every line deserializes as `RunReportEnvelope`.
**Why:** The chunk added `CheckRecord` lines to `runs/<run_id>.jsonl`, pinned by `journal.rs::check_line_is_its_own_parseable_shape_beside_the_envelope`. test-plan Section 3 OWNS this format, so leaving it would make the change one-sided against obs-plan Section 3.

## 2026-08-21-per-check-latency-measurement — Status endpoint shape scoped to the envelope grain
**Section:** Section 3 Test Harness Contract -> Status endpoint shape
**Change:** 'each JSONL journal line' narrowed to the envelope line / `runs` row, with per-check detail named as a separate finer grain (`run_check`, read via `RunsDb::checks_for`).
**Why:** The envelope field count is explicitly unmoved while a new grain now shares the journal.

## 2026-08-21-per-check-latency-measurement — Agent-runnable invariants qualified
**Section:** Section 2 Test Strategy -> Agent-runnable invariants
**Change:** The machine-parseable-output bullet qualified so the envelope assertion targets the envelope line specifically.
**Why:** Same retired claim as the Section 3 primary: a `CheckRecord` line will not deserialize as `RunReportEnvelope`.

## 2026-08-21-per-check-latency-measurement — E2E journal signal targets the envelope line
**Section:** Section 6 E2E -> Headless deterministic scenario run, Verification signal
**Change:** The journal assertion targets the envelope LINE, with any per-check lines parsing to the per-check shape.
**Why:** A file-wide parse-to-envelope assertion is now a false negative on any run that emits check rows.

## 2026-08-21-per-check-latency-measurement — Cleanup covers every table a run writes
**Section:** Section 3 Test Harness Contract -> cleanup (body + verification)
**Change:** Teardown extended past `runs` to `DELETE FROM run_check` and `DELETE FROM run_envelope` (bound parameters), with matching count-zero verifications.
**Why:** The chunk added `run_check`; Section 3 named only `runs`, so cleanup was neither complete nor verifiable for a run that wrote check rows. `run_envelope` was already uncovered — a pre-existing gap the contract closes at the same time; the CODE fix in scripts/agent-run.{sh,ps1} is carried to its owner entry at route-resolve.

## 2026-08-22-operator-pause-and-checklist-live-firing — the never-blocks property re-tiered from the cli E2E leg to the unit tier
**Section:** §1 Test Scope Summary → cli surface Notes · §6 E2E Test Strategy → drivers-per-surface, cli row
**Change:** The stdin-closed cli leg is now recorded as proving no-hang / exit-0 ONLY. With no live Pulse the preflight blocks and `execute_scenario` returns on the Blocked spine before the hold, so the leg never reaches an interactive prompt; the never-blocks property is attributed to the unit tier (`headless_never_blocks_under_paused_clock`; `resolve_kind_agent_mode_overrides_an_attended_tty`). Applied at both sites, which stated the claim identically.
**Why:** Measured false this chunk — a fresh `agent-latest.jsonl` carried ZERO resolution witnesses and ended at `preflight blocked: MCP read-back path unreachable`, so the leg's green proved something narrower than the doc claimed.

## 2026-08-31-p-075-assert-round — `mark_incident_resolved` gains a production caller and a live exercise
**Section:** §5 Integration Test Strategy → `mark_incident_resolved` bullet · §1 Critical Path 4 · §6 severity-lifecycle Surfaces line
**Change:** Retired "NO production call site and no live exercise" — both halves measured false. §5 now names three tiers: applied + declined arms stub-proven at the integration tier (`conductor-verify/tests/readback.rs`), the declined arm stub-ONLY and permanently so (Pulse's `DeclinedStale` is a monotonic-timestamp guard its own dispatch cannot trip), and the live write graded at the harvest tier on the pinned capture (`lifecycle_harvest.rs`) driven by the feature-gated operator leg. Verdict PROVEN-BY-LIVENESS — runtime-STATE fidelity, never payload. §1 Path 4 and §6's Surfaces line move with it (two-site rule): the caller exists but is exercised out-of-band, never inside `execute_scenario`, so neither path's grading changes.
**Why:** Report §Spec claims disproved #1 names this bullet explicitly; the live leg ran 2026-09-01 against Pulse HEAD `83d4060` with `idle_seconds_at_resolve = 0.0`.

## 2026-08-31-p-075-assert-round — Stub item-key fidelity: `incident_id`, not `id`
**Section:** §5 Integration Test Strategy → `query_incident_list` bullet
**Change:** Recorded as a mocking-discipline requirement: the stub's incident item key MUST be `incident_id`, the key the live sidecar emits. The shared stub emitted `id`, so a reader keyed on `id` extracted an empty active set from a POPULATED live corpus — the silently-empty class this bullet already bans, arriving through the mock rather than the data dir, and green against every stub leg on the way to failing live. Pinned by `lifecycle_harvest::the_live_item_key_is_incident_id`; readers accept either key.
**Why:** Report §Spec claims disproved #3. Cost four legs: three incidents were active 131-142s each while the leg logged an empty set; only dumping the raw wire value separated "Pulse never exposed it" from "our reader extracted nothing".

## 2026-08-31-p-075-assert-round — A cargo-feature gate is a third sanctioned live-leg path
**Section:** §9 CI Integration → Live-Pulse scenarios · §11 Test Anti-Patterns → CI (stack-specific live-Pulse ban)
**Change:** §9's invocation inventory now admits a cargo-feature-gated test file invoked directly — `conductor-run/tests/lifecycle_live.rs` behind `[features] live-pulse = []`, an EMPTY feature adding zero package nodes and leaving `Cargo.lock` byte-unchanged, which keeps the leg out of default `nextest`/`clippy`/release and out of both runner-portability runs. Because a gated file is invisible to the default lint pass, such a leg OWES its own `cargo clippy -p <crate> --features <feat> --all-targets -- -D warnings`. §11's ban is unchanged in force; its parenthetical widens past "workflow_dispatch only" so the enumeration cannot contradict §9.
**Why:** The chunk's live leg used this path and the report records the extra clippy gate; the `stub-server`-gated `preflight_spawn.rs` precedent existed in code but appeared nowhere in test-plan (grep: zero hits), so the convention was undocumented.

## 2026-09-01-webview-self-verify-windows-host — the webview E2E platform verdict, measured
**Section:** §1 Surfaces under test (:54) · §1 Coverage triggers (:85) · §2 Agent-runnable invariants (:121, :122) · §3 CI stage selectors (:152) · §3 Bootstrap 5-command-discipline-wire (:207) · §4 conductor-tauri/ui (:244) · §5 GUI-leg deferral (:287) · §6 Drivers per surface (:304) · §6 Both-surface parity (:369) · §9 pipeline table (:456) · §9 Matrix builds (:465) · §11 Universal bans (:575, :577) · §12 Decisions Log (:594)
**Change:** The "Linux + `xvfb` only" verdict is retired as a CAPABILITY claim across every site, replaced by the measured platform SET — Linux CI headless under `xvfb`, plus the Windows dev host headful via WebView2 + an operator-supplied `msedgedriver`; macOS alone stays driver-less, which was always the justification's true content. The CI ARRANGEMENT is unchanged (still an `ubuntu-latest` + `xvfb` job; this chunk made no CI edit). `--e2e` now names its measured preconditions (ensure-frontend → release build `--features tauri/custom-protocol` → `wdio run`), "headless only" becomes "non-interactive only" in both bans, the loopback allowlist admits the harness-lifetime WebDriver pair, §85's coverage trigger splits by who READS the handle, §4 records that the build gate does not prove ESM loadability, and §12's dated decision keeps its text with a dated correction appended.
**Why:** The chunk drove a real WebView2 session on Windows, which falsifies the platform verdict but not the driver choice or the macOS clause. The site list ran to EIGHT where the plan's expected-amendments list named six — `:287` and `:369` additionally deferred GUI legs to "Linux+xvfb" — which is why the fix names the set rather than substituting a fresh literal. §4's ESM clause is the sharper finding: `typecheck:e2e` passed for two months over a `wdio.conf.ts` that could not load on any host.

## 2026-09-01-desktop-a11y-sweep — webview E2E registered as two arms; the driven arm's firing form recorded
**Section:** §2 Agent-runnable invariants (determinism) · §3 CI stage selectors · §4 conductor-tauri/ui · §6 Drivers per surface (desktop-webview) · §9 Live-Pulse scenarios · §11 (CI ban, Universal real-network ban)
**Change:** (1) §6 records TWO arms over one stack and the driven arm's FULL FIRING FORM as part of the leg — a `PATH` prefix resolving `andromeda-pulse-mcp`, `ANDROMEDA_PULSE_MCP_ENABLED` + `_L4_DETERMINISTIC`, `ANDROMEDA_PULSE_DATA_DIR` **equal to the live Pulse's data dir**, and a QUIET WINDOW of ≥120s idle + a 30s resolver tick after any preflight canary. (2) §9 admits an npm-script wdio suite as a fourth sanctioned live-leg invocation path, §11's ban parenthetical follows. (3) §3 qualifies `--e2e` as the routine suite only, with the `cwd: repoRoot` spawn and the 60s → 15min mocha ceiling. (4) §2/§11 name both arms in the loopback allowlist and record the driven arm as a real-clock live-Pulse leg carried only as the operator-local-gate exception. (5) §4's ESM-loadability clause widens from `--e2e` to whichever wdio leg loads the member.
**Why:** each precondition was measured this chunk and each has a failure mode that misreads as something else — a bare invocation returns BLOCKED in ~2ms indistinguishable from a real gate failure (PATH); a default-dir sidecar reads a corpus Pulse never writes, giving `result_count: 0` forever (data dir); and `conductor preflight` fires its own canary, so Pulse's per-`(kind, scope, scope_id)` dedupe means a second canary inside the window forms no fresh incident (quiet window — two 12-minute runs lost to it). The ban on running any live leg as a CI gate is unchanged in force.

## 2026-09-01-live-per-p-id-verdict-lamps — the `--e2e` leg seeds its own subject
**Section:** 3 Test Harness Contract (CI stage selectors) + 5 Integration Test Strategy + 6 E2E drivers-per-surface (desktop-webview) + 7 Test Data and Fixtures + 9 CI Integration (E2E row)
**Change:** Recorded that `--e2e` now copies the committed `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` into a gitignored `runs/e2e-fixture/` and spawns tauri-driver with `CONDUCTOR_RUNS_DIR` pointing there (section 3, restated in the section 9 pipeline row). Section 6's ROUTINE arm is re-stated from "subject-absent specs context-skip" to fixture-seeded -- it now asserts the verdict lamps for real, and only the load-envelope banner still context-skips, that path carried at the unit + mock-runtime tiers. Section 7 names the committed-journal fixture family in the sanctioned provenance set and requires such a fixture's MEANING be pinned by a production-reader round-trip. Section 5's read-only-command tier now names the source SET rather than `conductor-core` alone.
**Why:** `/runs/` is gitignored, so on a clean tree the coverage rows had no run record to render and the populated-lamp assertion had no subject -- on a dev host it would have passed off local run residue instead. Section 5's `conductor-core` qualifier was falsified outright: `run_envelope` single-sources `conductor-report`'s `RunsDb::get_envelope` via `conductor_run::read_envelope`.
