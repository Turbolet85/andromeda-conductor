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
