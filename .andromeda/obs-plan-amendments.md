# Obs Plan — Amendments

_Append-only changelog of amendments to `obs-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — two record shapes clarified (self-obs base line vs Run-report envelope)
**Section:** §3 Observability Harness Contract / Log format JSON schema
**Change:** clarified that the §3 schema block is the Run-report envelope (scenario-result record — emission journal + `runs.db`, report seam Epoch 6); the foundational self-obs log line carries a base set — `timestamp_ms` (epoch millis, `std::time`), `level`, `target`, service-identity, `run_id` — on every line, with envelope/result fields only on scenario-result events. Noted the implementation uses a custom `tracing-subscriber` layer (stock `fmt().json()` can't emit constant identity fields flat).
**Why:** the structured-logging-stack chunk implemented the self-obs stream; its line schema differs from the envelope the §3 block showed. Justified divergence (foundational lines can't carry verdict/latency yet) → §3 reconciled. D-obs-stack cleared (tracing JSON, no OTel SDK). D-obs-redaction (escalate) resolved WITH the user as correct sequencing (redaction = `pii-scrubbing-wire`, the next chunk; synthetic-only, no leak) — NO obs body edit; a `playbook.md` rule was added so Foundation deferrals of separately-sequenced concerns no longer escalate. Cascaded to `.claude/rules/observability.md` + `.claude/docs/obs-summary.md`.

## 2026-06-15-log-error-boundary-redaction — redaction model reconciled (host-file-path anchor; struct-name guard = allowlist + Display; `target` preserved)
**Section:** §6 Log conformance check (obs CI gate) · §11 Anti-Patterns (Logs · PII Scrubbing)
**Change:** reconciled the redaction model to the implemented `conductor-core::redact`: the value scrub masks absolute host-FILE paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>`, NOT `::`-type tokens; internal struct names are kept out by the field-name allowlist (drops non-allowlisted, incl. Debug-dumped, field names) + `Display`-not-`Debug` at the `anyhow` edge; the allowlisted `target` module path (`module::`-shaped by design) is explicitly preserved. The §6 CI conformance gate no longer treats a `module::` prefix as a struct-name leak.
**Why:** the redaction layer (deferred from `2026-06-15-structured-logging-stack`) landed this chunk. The plan/§11 wording listed `path::`/`module::`/`backtrace` as if all are token-level redaction targets; blanket `::`-redaction would gut the allowlisted `target` correlation field and mangle std type names in panics (`Option::unwrap`) — the over-redaction the plan flagged. §11's "paths" = absolute *file* paths; the implemented host-path anchor + allowlist + Display-edge realizes "no host paths / no struct names" without that breakage (report deviation #1). D-obs-redaction (escalate) resolved WITH the user (2026-06-15 wrap). D-obs-stack / D-obs-instrumentation cleared (tracing JSON, no OTel; `run_id` + `std::time` preserved). Cascaded to `.claude/rules/observability.md` + `.claude/docs/obs-summary.md`.

## 2026-06-16-emission-journal-writer — Run-report envelope gains `read_back_observed_at` (realign to owner test-plan §3)
**Section:** §3 Logging stack + §3 Harness Contract / Log format JSON schema + §6 Log Coverage (all four envelope reproductions)
**Change:** added `read_back_observed_at` (ISO-8601 from `std::time`; null until read-back, null for blocked rows) as the 2nd field of the JSONL Run-report envelope, after `journal_emitted_at` — the schema is now 11 fields.
**Why:** D-tests-obs-harness escalation (reclassified from the fan-out's warning). obs-plan §3 was the LONE doc omitting `read_back_observed_at`, while the schema OWNER test-plan §3 (status-shape + polled fields + §6/§7 journal goldens that *redact* it ⇒ it is present) + arch §Standard Contracts + the `runs.db` data model all carry it. The emission-journal-writer chunk had implemented obs-plan's 10-field outlier. Resolved WITH the user (Option A): the JSONL carries `read_back_observed_at`; obs §3 realigned to the owner and the chunk's `RunRecord` fixed to 11 fields (re-ran nextest 66/66). `latency_ms` stays `read_back_observed_at − journal_emitted_at`. Cascaded to `.claude/rules/observability.md` + `.claude/docs/obs-summary.md`; bind-honored in a11y-plan §3 (violation-schema reproduction).

## 2026-06-17-raw-otlp-message-scaffold — no-SDK invariant clarified as behavioral; dormant transitive OTel SDK noted
**Section:** §3 OTel SDK init
**Change:** clarified the "no OTel SDK for self-observation" invariant is BEHAVIORAL — no SDK is ever initialized/used for self-obs (the harm is batch tasks breaking `current_thread` determinism + polluting the PRODUCT stream). Noted that opentelemetry-proto's default features transitively pull `opentelemetry` + `opentelemetry_sdk` (dormant, never initialized, audit/deny-green) into the tree; their mere presence is not a violation. Recorded a follow-up to evaluate `default-features = false` on the opentelemetry-proto dep.
**Why:** D-obs-stack (escalate) fired on the conductor-emit chunk — opentelemetry-proto (the PRODUCT proto lib) pulls the OTel SDK crates transitively. Self-obs is unchanged (tracing JSON only; no SDK init), so the behavioral invariant holds. Resolved WITH the user on 2026-06-17 (accept + document + follow-up); a `playbook.md` rule was added so a dormant transitive SDK no longer re-escalates. Cascaded to `.claude/rules/observability.md` (Hard bans clarified); obs-summary.md untouched (its "no SDK for self-obs" statements stay true + its header reserves it from wrap).

## 2026-06-18-severity-logs — `emit.logs_batch` added to the bounded span-name set
**Section:** §11 Anti-Patterns (Spans / Traces — bounded span name set)
**Change:** added `emit.logs_batch` (the OTLP logs-egress self-observation span on `LogsEmitter::export`, `record_count` attribute) to the bounded span-name set, alongside `emit.batch` (trace egress) — distinguishing logs egress from trace egress in self-obs.
**Why:** the severity-logs chunk (P-007) shipped a new `LogsEmitter` whose egress is instrumented with a new, low-cardinality span name following the `{module}.{operation}` convention. Routine doc-reconciliation: the report pre-flagged it; the two warning-severity detectors (D-obs-instrumentation + the D-tests-obs-harness bind-view) agree non-contradictorily; the spec→sound-impl-alignment playbook rule applies (the bounded-set invariant is preserved — a conforming member is added, not a high-cardinality name). Cascaded to `.claude/rules/observability.md` (bounded set); obs-summary.md untouched (enumerates no span names); test-plan §3 needs no edit (the harness/envelope/log-format are unchanged — the bind stays consistent).

## 2026-06-24-sanitized-stderr-agent-mode-logging — §3 agent-mode flag reworded to the read-only CONDUCTOR_AGENT_MODE trigger (D4)
**Section:** §3 Observability Harness Contract (Logging stack — agent-mode flag, both the summary + detailed sink blocks)
**Change:** reworded "`--agent-mode` … sets `CONDUCTOR_AGENT_MODE=1`" → agent mode is triggered by the flag OR the `CONDUCTOR_AGENT_MODE` env, a **read-only trigger** Conductor reads (`agent_mode = flag || env-set`) and never WRITES (avoids edition-2024 `unsafe std::env::set_var`; the harness/operator exports it). Observable mode is identical.
**Why:** report Deviation #3 (D4, pre-decided P4) flagged the spec wording as off from the shipped read-only trigger. Routine spec-wording → sound-impl reconciliation (the 2026-06-15 generalized playbook rule): the report proves the observable invariant holds (the agent-mode E2E + the obs file-sink units). D-obs-redaction clean — the new file sink REUSES the unchanged processor-stage field-allowlist + `redact_value` (report Coverage PII redacted✓, verified by `agent_file_sink_writes_redacted_json_to_the_file`); consuming unchanged hardened infra is not a new boundary. D-obs-instrumentation clean (the chunk adds the logging SINK itself, not a new must-trace op). D-obs-stack clean (no OTel SDK; std-only file sink). The pre-existing "pretty-print in dev" wording is NOT this chunk's drift (JSON-to-stderr in dev predates it — structured-logging-stack; out of scope this wrap). Cascade: `.claude/rules/observability.md` + obs-summary.md already state "cli → `logs/agent-latest.jsonl` (`--agent-mode`)" correctly (no "sets" wording) → no-op.

## 2026-06-27-obs-ci-conformance-gate — §9 conformance check asserts the §3 self-obs base schema (not the §6 envelope)
**Section:** §9 CI Integration — Log conformance check
**Change:** the `logs/agent-latest.jsonl` conformance gate validates the §3 self-obs base-line schema (`timestamp_ms`/`level`/`target`/`service.name`/`service.version`/`deployment.environment`/`run_id`), NOT the §6 run-report ENVELOPE; the envelope (`runs/<run_id>.jsonl`) gets its own, not-yet-built gate. The two record shapes are distinct (§3 line 229).
**Why:** this chunk OPERATIONALIZED the §9 conformance gate for the first time; the shipped gate correctly asserts the §3 base schema (verified by the dogfood — PASS on the real artifact + FAIL on a missing base field), revealing that §9's field-list named the §6 envelope — the wrong record shape for the artifact it reads. Intra-obs-plan reconciliation: test-plan §3 (the envelope-schema OWNER) was already correct (the test-plan detector confirmed it); obs §9 merely mis-referenced §6. Escalated (record-shape sensitivity — the long-flagged §3↔§3 follow-up) → confirmed WITH the user on 2026-06-27; a `playbook.md` rule was added so future first-operationalization gate-wording reconciles don't re-escalate. The §9 zero-unlogged-panics gate (greps `agent-latest.jsonl` + stderr) already matched the impl — no edit. Cascade: `.claude/rules/observability.md` + obs-summary.md carry the "two record shapes" correctly + no §9-conformance-field-list detail → no-op.

## 2026-08-08-sut-capability-manifest — De-hardcoded coverage-gate span attributes
**Section:** §4 Span/Trace Coverage (coverage-matrix completeness gate) · §1 must-trace table
**Change:** `db.query_all_p_ids`'s `p_id_count_expected` and the `p_ids` log field name the manifest's capability count/set rather than the literal 60.
**Why:** Same SUT-advance reversal: the span spec asserted a fixed 60 the accepted set no longer defines.

## 2026-08-09-out-of-scope-classification-treatment — coverage_percent's denominator pinned to the in-scope count
**Section:** §4 Span/Trace Coverage — Scenario: Coverage-matrix completeness gate
**Change:** added denominator semantics to the gate's attributes: `p_id_count_expected` is the FULL manifest capability count, while `coverage_percent` is computed over the **in-scope** count (the manifest set minus the out-of-Conductor's-remit rows). Both stay manifest-derived, never literals.
**Why:** the chunk's roll-up now renders that split on all three surfaces, which pre-binds `coverage_percent`'s meaning for the not-yet-built Epoch-6 completeness gate; without recording it the gate's 82-row expectation and the shipped 66-row in-scope denominator could diverge unnoticed. Detector D-obs-instrumentation; the pre-binding was an explicit operator decision at /andromeda-phase P4.

## 2026-08-09-interpretation-correctness-posture — coverage roll-up auto term carries a derived unbacked qualifier
**Section:** §4 (coverage-matrix completeness gate — denominator semantics)
**Change:** recorded that the roll-up's auto term additionally carries a derived `(N unbacked)` qualifier read from `conductor_core::UNBACKED_AUTO`; it qualifies the auto count rather than joining the breakdown, so the per-mode summands still sum to the row count and a gate reading these fields must not treat it as a fifth mode.
**Why:** the chunk shipped the qualifier on all three roll-up surfaces the gate's denominator semantics already govern. Self-raised — the obs detector returned clean.

## 2026-08-10-scenario-run-root-span-tree — the root's cleanup no longer claims the report-seam spans as descendants
**Section:** §4 Span/Trace Coverage — Critical Path 1 Cleanup · Known-residual classification path Cleanup
**Change:** both Cleanup lines dropped "Root `scenario.run` closes on `db.insert_run` completion". The root now opens per-scenario at the composition root (`conductor-run::execute_scenario`) and closes when that scenario returns; `report.generate` / `db.insert_run` are recorded as run-scoped SIBLINGS correlated by `run_id`, not descendants. Both sections landed.
**Why:** the chunk built the chain and the code-graph proved the claim unachievable — `persist` is called ALONGSIDE `execute_scenario` at all three production sites (`run.rs:23`/`:25`, `suite.rs:30`/`:36`, `drive_run() lib.rs:481`/`:485`), and under a suite one `persist` serves N scenarios whose roots have already closed, so no single `scenario.run` can contain `db.insert_run`. Detector D-obs-instrumentation; root placement was an operator decision at /andromeda-phase P4.

## 2026-08-10-scenario-run-root-span-tree — Tauri command-span names reconciled to the shipped handlers
**Section:** §1 Harness contract (desktop-webview row) · §4 Both-surface parity path · §4 cross-surface correlation note
**Change:** `start_scenario()` / `stop_scenario()` / `get_run_report()` / `operator_pause_go_no_go()` → the shipped `start_run()` / `stop_run()` / `run_report()` / `resolve_operator_hold()`, and the two `tauri.command.start_scenario` span references → `tauri.command.start_run`.
**Why:** PRE-EXISTING drift this chunk did not introduce, surfaced at /andromeda-phase P2 when the obs distiller faithfully reproduced a name that matches no shipped handler. §11's bounded set uses the `tauri.command.*` wildcard, so no invariant was violated — only the illustrative names were false. Fixed here because it sits in the same sections this chunk amended; raised by the orchestrator (no detector proposed it) under the plan's Expected-amendments list and an explicit operator directive.

## 2026-08-10-scenario-run-root-span-tree — the self-obs line's span-lifecycle variant recorded
**Section:** §3 Log format JSON schema (two record shapes)
**Change:** recorded that the custom layer emits the self-obs line in two variants over the same base set — the event line, and the span-lifecycle line adding `span` / `span_event` (`new` | `close`) / optional `parent` plus the span's allowlisted attributes on `new` — so a §4 span materializes as real lines. The two-record-shapes split (self-obs line vs Run-report envelope) is unaffected.
**Why:** the lateral test-plan §3 ↔ obs-plan §3 bind. test-plan §3 OWNS the JSONL format and gained the variant this chunk shipped; leaving obs §3 unamended would be exactly the one-sided change D-tests-obs-harness guards. Before this chunk the layer implemented only `on_event`, so no span emitted anything at all.
## 2026-08-11-faithful-emission-dispatcher — scenario-config validation ban restated
**Section:** §11 Anti-Patterns (project-specific bans)
**Change:** The never-skip-garde-validation ban now names the shipped error-fraction encoding (`error_percent` ∈ 0..=100) and adds that a nested spec field must `dive`, never `skip` — a skipped struct is never descended into, so its rules never run.
**Why:** Cross-master citation fold: the ban quoted `error fraction ∈ [0,1]`, a bound the shipped model does not carry in that form, and the chunk's `#[garde(skip)]` finding gave the ban a concrete failure mode it did not previously name.

## 2026-08-13-per-check-read-back-extraction — degraded read-back is OBSERVED, not requested
**Section:** §1 Critical paths (Known-residual row) · §4 Span/Trace Coverage (CP1 `verify.readback`,
Fingerprint-storm `verify.readback_fingerprints`, CP5 Known-residual classification path)
**Change:** CP5's must-trace span `verify.readback_degraded_mode` (MCP call with `degraded_mode=true`) →
`verify.readback.observe` (the read-back pass whose `retrieve_report` result REPORTS `degraded_mode`), at both
the §1 table row and the §4 scenario. Its required attributes `degraded_mode_requested` / `response_received`
are retired: the degraded signal is observed, not requested, and rides a `warn` line on the allowlisted
`message` field. §4's Required-span-attributes heading now states the constraint that a span attribute must be
a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it. `mcp_method` →
`mcp_tool` on both `verify.readback` and `verify.readback_fingerprints`. CP5's Required log fields gained the
missing `read_back_observed_at` (the eleven-field envelope every other critical path carries) and keep the
scenario extra `degraded_mode_response`, with a note that the two record shapes are governed differently — an
envelope extra by the report seam, a span attribute by the allowlist.
**Why:** the chunk is `retrieve_report`'s FIRST caller in the workspace (code-graph: 0 prior call sites), so it
is the chunk that operationalizes this path and whose current truth now includes its real behavior. Two
artifact-verified disqualifiers for the old wording: Pulse computes `degraded_mode` as `parsed_l4.is_none()`
and returns it in the result (`andromeda-pulse/crates/mcp-server/src/tools.rs:372,380`), so
`degraded_mode_requested` describes a call Conductor cannot make; and neither retired attribute name is in
`ALLOWLISTED_FIELDS`, so a built attribute would emit nothing. The shipped span field is `mcp_tool`
(`conductor-verify/src/client.rs:125`). Routine per playbook:28 (spec-illustration → sound-impl) and
playbook:88 (operationalizing surfaces the spec's own stale field-list).

## 2026-08-13-first-live-green-preflight — read-back boundary log gains the shape witness
**Section:** §6 Boundary-call wrappers (must-log events)
**Change:** The MCP-readback must-log set gains the observed KEY SET of each raw tool result, and the tool list is completed with `retrieve_telemetry_slice`. Key names only, never values, carried on the `message` field because a field name outside the allowlist is dropped at the processor stage.
**Why:** The extraction readers degrade to empty on an unrecognized shape rather than erroring, so without the witness a live field-name divergence is indistinguishable from an empty corpus. First live leg confirmed the witness works: `query_incident_list returned keys [items, next_cursor, total]`, matching the committed baseline exactly.

## 2026-08-14-canary-fingerprint-feed-capture — emit.batch wire-shape witness + the additive RUST_LOG form
**Section:** §6 Log Coverage (Boundary-call wrappers) · §3 Harness Contract (Per-module log levels) · §11 Anti-Patterns
**Change:** §6's `emit.batch` must-log list now records the wire-shape witness — observed span count, spans a
receiver would skip for a missing `trace_id`/`span_id`, and the distinct event / event-attribute KEY NAMES of
the outbound request (names only, ordered), emitted at `debug` inside the existing `#[instrument]` span and
carried on the already-allowlisted `message` field, so it needs no new allowlist entry. Separately, the §3
per-module level table and the §11 hot-path rule now state the ADDITIVE form `RUST_LOG=info,{crate}=debug`:
a bare per-target directive replaces the default rather than adding to it.
**Why:** the chunk added the witness as the emitting-side twin of the `verify.readback` key-set witness — a
receiver that degrades to empty on an unrecognized shape makes a divergence indistinguishable from emptiness,
which is the ambiguity this chunk existed to resolve. The RUST_LOG correction was measured, not inferred: the
bare form fails the CLI's own agent-mode self-obs test while `info,conductor_emit=debug` passes, and the old
table was internally inconsistent under literal use (it would silence the very crates it sets to `info`).

## 2026-08-16-canary-fingerprint-derivation-aligned — fingerprint match count is not computable
**Section:** §4 Span / Trace Coverage (Fingerprint-storm scenario → required span attributes)
**Change:** `verify.readback_fingerprints`'s `fingerprints_matched_count` becomes
`fingerprints_read_back_count` — a count of what read-back returned, with a note that an emitted-vs-read-back
MATCH is not computable because Pulse exposes no read-back surface carrying its own computed fingerprint
(`fingerprint_refs` is L4-authored and `[]` under deterministic L4; `span_events.fingerprint` has zero reads
in `mcp-server`).
**Why:** a required span attribute defined as a match count between two values that never meet is
uninstrumentable as written; measured 2026-08-16 and confirmed live (`retrieve_telemetry_slice` returned
`result_count: 0` on a healthy leg, envelope `fingerprints: []`).

## 2026-08-16-fault-application-spans — fault spans open where the faults actually are
**Section:** §4 Span / Trace Coverage (Fault-injection spans — heading, the three attribute lists, and the
placement paragraph) · §1 Obs Scope Summary (instrumentation-scope `conductor-faults` row · telemetry-trigger
`chaos-instrumentation` row) · §6 Log Coverage (log-levels table `info` + `debug` rows, plus a why-note)
**Change:** four corrections, all measured this chunk.
(a) PLACEMENT — "Each fault span wraps the fault application phase in conductor-faults" is retired. The
run-path faults are declarative phase data (`EmissionSpec { occurrences: 0 }` and `EmissionShape::Ramp`), so
`fault.silence` / `fault.ramp` open from a caller-supplied per-phase hook (`run_timeline_observed` /
`PhaseWindow`, fault semantics + the `std::time` journal basis supplied by `conductor-run`), created but NOT
entered so `emit.batch` keeps `timeline.execute` as its parent. Only `fault.port_occupier` opens inside
`conductor-faults`, at its RAII bind site. The §1 crate row was realigned in the same pass.
(b) `ramp_factor` — range corrected from `0.0-1.0` to **−1.0..1.0**, defined as the normalized signed slope
`(to_rate − from_rate) / max(from_rate, to_rate)`, so a rise and a fall are distinguishable.
(c) `fault.port_occupier` — attribute set narrowed to `fault_type` + `port` (neither the duration nor the
journal offset is knowable at bind time, and the layer records attributes on `new` alone), the realized hold
witnessed on the allowlisted `message` field at `debug` on release, and its `timeline.execute` parentage
recorded as CONDITIONAL — no run path drives the occupier yet. The §1 trigger row was realigned with it.
(d) LOG LEVEL — fault-span lifecycle moved from the `debug` row to `info`, with the reasoning recorded
beneath the table.
**Why:** (a) the code-graph returns 0 crate edges for `conductor-faults` in either direction and 0 references
to its fault types from outside the crate, so a span placed there fires on no scenario run. (b) the shipped
`EmissionShape::Ramp` carries `from_rate`/`to_rate`/`windows` integers — no 0.0-1.0 quantity exists to read;
the signed form was the operator's decision at the chunk's phase P4. (c) an attribute that cannot be computed
at the only moment the layer reads attributes is uninstrumentable as written (the 2026-08-16 precedent one
entry above), and claiming a parent the code cannot produce repeats what the 2026-08-10 root-span cleanup
corrected. (d) at `debug` the spans are invisible under the default INFO filter, so the chunk's own
acceptance — the spans appearing on emitted lines — could not hold without a `RUST_LOG` opt-in; every sibling
span is `info`, and one span per phase is not the hot path §11's ban targets.

## 2026-08-16-fingerprint-storm-live-proof — fingerprint-storm `fingerprints` no longer "populated"
**Section:** §4 Span/Trace Coverage (Fingerprint-storm Required log fields) + §1 Obs Scope Summary
(critical-paths table, the same restatement)
**Change:** `fingerprints` (populated) → present, MAY BE EMPTY: under deterministic L4 it is fed solely from
`fingerprint_refs`, which the fixture pins `[]`. Storm identity is evidenced by `fingerprints_read_back_count`
and the test-only harvest surface, never by a populated envelope array.
**Why:** measured 2026-08-16 — both live legs returned `fingerprints: []`. §4's own
`verify.readback_fingerprints` bullet already recorded `fingerprint_refs` as pinned empty, so the
Required-log-fields line contradicted the bullet above it and demanded an unsatisfiable field on a must-trace
path.

## 2026-08-18-error-baseline-spike-live-proof — fingerprints MAY-BE-EMPTY-under-L4 superseded (three sites)
**Section:** §4 Fingerprint-storm Required log fields · §4 verify.readback_fingerprints bullet · §1 critical-paths table Fingerprint-storm row
**Change:** "MAY BE EMPTY / pinned `[]` under deterministic L4" retired at all three sites — the fixture's `evidence_refs` now populates `fingerprint_refs` with a constant `det-*` triple (measured 2026-08-18, SUT HEAD `efabe8e`); the emitted-vs-read-back match stays non-computable (payload-invariant fixture constants), `fingerprints_read_back_count` + the harvest surface stay the storm-identity evidence.
**Why:** Leg A's envelope measured the 3 det-* refs live; the 2026-08-16 amendment's supporting fact (emptiness) is superseded while its conclusion (no identity carrier) stands. Chunk report + leg-verdict.

## 2026-08-18-restart-suppression-live-proof — restart-suppression row retargeted to the shipped declare-only instrumentation
**Section:** §1 Critical paths table (Restart-suppression row) · §4 Scenario: Restart-suppression scenario incl. bypass case · §6 Log Coverage (scenario-specific fields) · §3 Log format JSON schema (additional-fields example)
**Change:** The never-built family span chain (`timeline.execute_restart_suppression` / per-path `emit.batch` `path_type` / `verify.readback_suppression_bypass`) and the `bypass_triggered` envelope extra are retired at all four statement sites; the row now records the shipped instrumentation — `scenario.run` → `timeline.execute` + two `fault.silence` children (measured offsets 7006ms/76436ms) → `emit.batch` → `verify.readback*` → `report.generate`, the standard eleven-field envelope — plus the measured constraints: suppression/bypass evidence grades at the harvest tier over Pulse's own lines, the `triage.cue.tick` counters read `"<redacted>"` live, and `persistence_seconds` is the service's cumulative sample count.
**Why:** The scenario landed declare-only (both `[[expected]]` checks structurally ungradeable under deterministic L4); the live leg (run 2026-08-18T21-40-46-519) measured the actual span/field surface (report §Spec claims disproved 1, 3; §Coverage of new surfaces).

## 2026-08-19-connection-lifecycle-live-proof — `fault.port_occupier` parentage CONDITIONAL → measured real
**Section:** §4 Span / Trace Coverage — Fault-injection spans ("Where each span opens")
**Change:** The occupier span IS a child of `timeline.execute` — measured live on the conflict leg (span_event `new`+`close`, `parent: "timeline.execute"`, attributes exactly `fault_type` + `port`); `conductor-run`'s `phase_guard` drives it for a fault-declared phase's window and the RAII release is the scheduler's boundary drop.
**Why:** The 2026-08-16 amendment recorded the parentage CONDITIONAL because no driver existed; the v2-15 driver shipped this chunk and leg B4 measured the condition real (report §Spec claims disproved item 3; verbatim lines in the chunk leg-verdict).

## 2026-08-20-latency-regression-re-proof — sidecar-spawn citation re-based
**Section:** §Obs Anti-Patterns
**Change:** The cross-master citation of security-plan's sidecar-spawn ban now reads "fixed hard-coded program NAME resolved through the inherited `PATH` + `.env(...)` only" instead of "fixed program path + `.env(...)` only".
**Why:** Cascade edge — obs-plan cited security-plan's retired wording after the 2026-08-20 spawn-wording amendment. The ban's substance (never a shell with operator-supplied input) is unchanged.
## 2026-08-21-severity-lifecycle-live-proof — CP4's family-specific span chain retired as never-built

**Section:** §4 Span / Trace Coverage — Scenario: Severity-lifecycle full pass · §1 Obs Scope Summary —
critical-paths row 4 · §6 Log Coverage — additional scenario-specific fields · §3 Harness Contract — the
additional-fields example
**Change:** the mandated chain (`timeline.execute_severity_lifecycle` / per-transition `emit.batch`
`severity_level`+`phase` / `verify.readback_auto_resolve` / `verify.readback_resolution_summary`) is RETIRED
as never-built and replaced by the shipped one (`scenario.run` → `timeline.execute` → `emit.batch` →
`verify.readback*` → `report.generate`); the envelope is the standard eleven fields with `verdict` null and
`state` "KnownResidual", with the `lifecycle_phase` / `severity_choice_calibrated` extras removed at all three
sites that named them. The lifecycle evidence is recorded as living in Pulse's own lines at the harvest tier.
**Why:** measured 2026-08-21 — all four span names plus `auto_resolve_triggered` / `summary_received` have
ZERO occurrences in `crates/`; read-back is one `verify.readback.observe` pass, not a per-claim call; and the
family landed declare-only so no per-phase or severity-calibration extra is written. The auto-resolve witness
had to move because `triage.incident.auto_resolve.tick`'s counters read `"<redacted>"` on the wire (Pulse's
default-deny allowlist predates them). Same retirement class as the restart-suppression re-base.

## 2026-08-21-per-check-latency-measurement — Second report-seam journal line shape
**Section:** Section 3 Harness Contract -> Log format (two record shapes)
**Change:** Recorded the report seam's SECOND journal line shape — the per-check `CheckRecord` — as a finer GRAIN beneath the envelope, never an extension of it; format owned by test-plan Section 3; neither report-seam shape passes through the span-attribute allowlist.
**Why:** Raised by the orchestrator at validate check 5 (the plan's expected-amendment floor). obs Section 3 reproduces the owner's journal format and the D-tests-obs-harness bind is two-sided, so the owner's amendment must land here too. It also records why `redact.rs` needed no admission: the field allowlist governs the self-obs line only.

## 2026-08-21-delegated-timing-budgets-proven — Delegated-timing budgets recorded as a harvest-tier path
**Section:** §4 -> Scenario: Known-residual classification path (detail block; the §4 critical-path TABLE is unchanged)
**Change:** Recorded the delegated-timing family under the Known-residual path it reaches: the three declare-only members add NO critical path of their own; their budgets grade at the harvest tier over Pulse's own tracing lines, each from its OWN exact allowlist leaf and never through `budget_ms`/`effective_deadline_ms`; the duration field is `duration_ms` on three leaves but `value` on `metric.report.render_ms`; and `findings-counter-refresh` (one `[[expected]]`) reaches the same state by the degraded read-back with its unmet `CountAtLeast` floor grading `CalibrationRegion`.
**Why:** Orchestrator-raised under the Expected-amendments floor (no detector covers adding a harvest family to §4). Placed in the Known-residual detail block rather than the critical-path table DELIBERATELY: the table and its per-scenario blocks are the fixed 7-path set that §4's Minimal-tier justification rests on (obs-plan :25 / :647 / :575 and `.claude/rules/observability.md:26` all state 7), so an eighth row would have moved a tier-justification input this chunk never measured. The `budget_ms` distinction was confirmed live on leg D (envelope `latency_ms: 6139` vs 1.9ms median in the same leg).

## 2026-08-22-operator-pause-and-checklist-live-firing — the self-obs sink corrected: stderr, never stdout; JSON, never pretty
**Section:** §1 Obs Scope Summary (telemetry-surfaces table cli + desktop-webview rows; logging-stack sinks; log-file location; multi-platform-exporter-compat) · §3 Logging stack (sinks, agent-mode flag, log-file location, paste-to-AI, correlation) · §6 Log Coverage (sink configuration) · §11 Anti-Patterns (three Universal bans) · §12 Decisions Log (dated annotation)
**Change:** 21 sites corrected across THREE stale wordings of one false premise. `build_subscriber` installs exactly one layer, `JsonObsLayer`, over every writer, and `ObsSink` has only `{Stderr, File}` — so (a) "pretty-print in dev" is retired everywhere (no pretty layer exists on any path), (b) every self-obs "stdout" is now stderr (nothing in the self-obs stack writes stdout), and (c) the Tauri backend's "stderr (dev only)" dual-sink framing is retired: the file sink is UNCONDITIONAL with stderr only as an open-failure fallback. Agent mode is recorded as a WRITER switch, not a format switch. §12 is history, so it carries a dated correction rather than a rewrite. §1 line 37's "structured output to stdout/stderr" was deliberately left — it is the CLI PRODUCT render and stays true.
**Why:** The chunk measured the claim false at source on every path (`obs.rs:63-66`, `:92-100`, `:148-159`; `conductor-cli/src/main.rs:44-50`; `conductor-tauri/src/main.rs:38-40`) and corrected the source twin `obs.rs:3` in the same pass. The 2026-06-24 amendment had already flagged the pretty wording as unreconciled.

## 2026-08-31-p-075-assert-round — The MCP boundary carries a WRITE; §6 must-log set names it
**Section:** §6 Log Coverage → Boundary-call wrappers · §1 Obs Scope Summary → conductor-verify row · §1 → Pulse MCP server row
**Change:** §6 gains an explicit lifecycle-write bullet: the same bounded `verify.readback.call_tool` span and allowlisted `mcp_tool` attribute as the read-back calls, plus the applied/declined outcome, with the INCIDENT ID carried on the allowlisted `message` field because `incident_id` is absent from `redact::ALLOWLISTED_FIELDS` and a dedicated attribute would emit nothing; the declined arm is recorded stub-only and permanently so. §1's two table rows lose their observation-only framing. INCIDENTAL CORRECTION applied in the same edit (operator-approved): the conductor-verify row still read "MCP read-back client via rmcp 1.7.0" — rmcp was removed 2026-06-27 — now the hand-rolled line-delimited JSON-RPC transport.
**Why:** `conductor_run::probe_resolve_lifecycle` is the first production caller of `mark_incident_resolved`; the report's Coverage bullet records span-yes/log-yes with the message-borne id, and `ALLOWLISTED_FIELDS` is unchanged. The rmcp clause sat inside the very sentence this amendment rewrites, so leaving it would have preserved a known-false claim in an authored line — dismissing it as pre-existing drift was the alternative the operator declined.

## 2026-09-01-webview-self-verify-windows-host — the command-span MECHANISM, and the handler count
**Section:** §4 Span/Trace Coverage → auto-instrumentation table, desktop-webview row · §1 Obs Scope Summary → desktop-webview row · §1 → ipc-internal row
**Change:** All three sites retire `#[tracing::instrument]` as the Tauri command-handler mechanism in favour of the manual `tracing::info_span!("tauri.command.<name>").entered()` guard, because the ATTRIBUTE does not stack with `#[tauri::command]` (the command macro rewrites the fn signature for IPC arg-extraction). The §1 desktop-webview row's handler enumeration is corrected from four to the measured **seven** — `list_scenarios` · `coverage_matrix` · `unbacked_auto` · `run_report` · `start_run` · `stop_run` · `resolve_operator_hold`.
**Why:** This chunk's own research first reported the opposite — "zero `#[tracing::instrument]` across 7 sites" as an obs GAP — and that finding was retracted: the code carries the prescribed manual span at 7/7 sites (`commands.rs` ×6, `pause.rs` ×1), independently re-verified by the operator. Coverage was never missing; the DOC was the divergent side, still prescribing an attribute that cannot stack. Span names stay inside §11's bounded `tauri.command.*` set, and `.claude/rules/observability.md` already carried the correct mechanism since 2026-06-26 — obs-plan had simply never been reconciled to it.

## 2026-09-01-live-per-p-id-verdict-lamps — eighth handler, the read-path convention, and a self-contradiction retired
**Section:** 1 Obs Scope Summary (telemetry-surfaces desktop-webview row + Notes + rusqlite instrumentation-scope row) + 4 Span/Trace Coverage (desktop-webview and conductor-report rows) + 6 Log Coverage (Boundary-call wrappers)
**Change:** (a) Instrumented `#[tauri::command]` handlers 7 -> 8 with `run_envelope()` in the enumeration (section 1) and the site count 7/7 -> 8/8 (section 4). (b) Deleted section 1's trailing "instrument via `#[tracing::instrument]`" clause, which contradicted the same sentence's own correct statement that the attribute does not stack with `#[tauri::command]`. (c) Recorded the READ-path convention at three sites: a rusqlite read inside a `tauri.command.*` span logs a section-6 Boundary-call `info!` carrying `run_id` plus the outcome on the allowlisted `message` field and mints NO `db.*` read span; section 4's conductor-report row narrowed to WRITE queries; section 1's rusqlite row split into write-span / read-log.
**Why:** (a) the chunk registered an eighth command. (b) the report's one measured-false spec claim. (c) the operator's ruling at this wrap: the bounded span-name set is a deliberate contract and is NOT widened with a `db.*` wildcard; the in-repo DB-span convention is an attribute on `RunsDb` methods in `conductor-report`, so a `db.*` read span stays a design option for the entry that next touches that crate. The two extra sites are the same retired wording restated, fixed in the same pass per the duplicate-occurrence rule.

## 2026-09-02-screen-reader-manual-spec — the Tauri sink is runs-dir-relative; a measured host-path channel outside the redaction sites
**Section:** 1 Obs Scope Summary (telemetry-surfaces desktop-webview row · Logging stack sink · Log file location) + 3 Log file location + Logging stack sink + 6 Sink configuration + 11 PII Scrubbing + 11 Project-specific bans + 12 Decisions Log (Exporter)
**Change:** (1) Every `logs/conductor-tauri.jsonl` restatement qualified: the file NAME is fixed, the DIRECTORY rides `CONDUCTOR_RUNS_DIR` (`runs_dir.parent()/logs`) -- the project-root `logs/` only with the handle unset (measured 2026-09-01); `runs/logs/` · `runs/driven/logs/` · `runs/sr-leg/logs/` on the a11y suites (each measured 2026-09-02); §12's Exporter bullet gains a dated correction. (2) §11 PII Scrubbing records the measured gap outside the three application sites -- the flagless sidecar spawn's console pane publishes the sidecar's absolute exe path as a foreground window title -- and the sidecar ban gains "never with a visible console window"; the `CREATE_NO_WINDOW`-class fix is route-owned, not shipped.
**Why:** (1) the chunk moved the driven arm's runs dir and the wrap measured three landing sites in one day; the bare literal pointed at a file that exists on no a11y suite. The detector's "never the project-root `logs/`" was over-general and was applied with the unset-handle scope. (2) NVDA spoke the pane's title inside the S1-01 window (`security_finding`); only the leg's own scrub kept it out of the committed record. Escalate-class, resolved on the operator's WRAP directive item 1 (a Conductor spawn DEFECT, a route candidate of its own).

## 2026-09-02-cross-surface-envelope-parity — §4's two rmcp attributions retired
**Section:** §4 Span / Trace Coverage — Span kinds (Client) · the auto-instrumentation per-surface table (conductor-verify row)
**Change:** The Client span-kind line now reads "MCP readback over the hand-rolled line-delimited JSON-RPC
client"; the auto-instrumentation table's conductor-verify cell now reads "None (the hand-rolled
line-delimited JSON-RPC MCP client is a seam)". The manual-span cell is unchanged. §11's
`CVE-2026-30623 rmcp STDIO flaw` mention is a vulnerability-CLASS citation and was deliberately left
standing, as were §1's already-reconciled "rmcp removed 2026-06-27" notes.
**Why:** D-obs-instrumentation, from the report's *Spec claims disproved by measurement* item 2. These two
sites were never enumerated by the 2026-08-31 CARRY that swept the same defect class; §1's conductor-verify
row had been corrected at that wrap while §4's two were missed, so this is the completion of that sweep.

## 2026-09-04-sidecar-spawn-without-a-console-window — the host-path channel this plan enumerated is closed
**Section:** §11 Obs Anti-Patterns — PII Scrubbing (primary) · §11 Obs Anti-Patterns — Project-specific bans
**Change:** Both sites retire the "route-owned, not shipped" status of the `CREATE_NO_WINDOW`-class fix and record the channel as CLOSED, with the stale `spawn.rs:80` locator dropped in favour of the symbol names. §11 PII Scrubbing keeps its standing warning intact — the enumeration bounds the sites it names, never every channel — while recording that this particular gap is now closed and that the a11y leg's `<host-path>` scrub is a second line of defence rather than the only one. §11 Project-specific bans keeps the ban and its "never with a visible console window" clause, restating the suppression as shipped and test-held, and records that the APPLIED flag is not assertable at the unit tier (no creation-flags getter on `std::process::Command`) so its effect is SR-leg evidence.
**Why:** This plan named the fix route-owned at two sites; the chunk shipped it. As measured at `conductor-0.2.0/chunks/2026-09-04-sidecar-spawn-without-a-console-window/evidence/nvda-pass.json`: SR row S1-01's `heard` lost `<host-path>`, `<host-path> terminal blank` and `pane`; the committed record carries 0 `security_finding` rows and 0 `<host-path>` placeholders (the prior record carried both), and 0 host paths appear inside any `Speaking [...]` line of the raw speech log — the three drive-letter hits there are NVDA's own startup config lines, outside any utterance. This SUPERSEDES the 2026-09-02 reading, which measured the pre-fix state. Validated ROUTINE under playbook `:134`, with its mandatory precedence check performed: rule `:124` (Boundary widening) does NOT match, since a suppression flag admits nothing new across the boundary. The obs↔security lateral bind was honoured in the same pass — security-plan §Anti-Patterns (a) carries the identical retirement, and the 2026-08-20 amendment had already established that this cross-master citation must move on both sides together. **Search performed:** grep of obs-plan.md for `route-owned`, `not shipped`, `not yet shipped`, `spawn.rs:80`, `CREATE_NO_WINDOW`, `console pane`, `console window`, `foreground`, plus §11 read in full for the mechanism however worded — 2 sites carried it, both amended; post-edit re-grep returns 0 for `route-owned, not shipped`, 0 for `route-owned fix, not yet shipped` and 0 for the `spawn.rs:80` locator. No §3/§4/§6 site asserts the claim (the fix adds no span, no log line and no field, so the §4 must-trace table and the §3 sink table are untouched).

## 2026-09-06-operator-gated-live-suite — `degraded_mode_response` retired as never implemented
**Section:** §3 Observability Harness Contract — Log format JSON schema · §4 Span/Trace Coverage — Known-residual classification path (Required log fields) · §6 Log Coverage — Additional scenario-specific fields
**Change:** All three sites retire the `degraded_mode_response` envelope extra. §3 now states the per-scenario extension point as an extension point with no known instance (its only ever-named one retired); §4's known-residual path carries the eleven-field envelope ALONE, keeping the note that an envelope extra is report-seam-written while a span attribute must be allowlisted; §6's field entry is struck through and marked retired. The extension POINT itself stands — only the named field is withdrawn.
**Why:** `2026-09-06-operator-gated-live-suite` measured the field absent: **zero occurrences anywhere under `crates/`** (`grep -c 'degraded_mode_response' crates/`), so no scenario emits it and a §4 required-field on a must-trace path was never satisfiable. The chunk's plan carried this as an Expected amendment; the report's Expected-amendments bullet locates the three sites (`:225`, `:347`, `:442`) with per-master hit counts. **Scope held to the measurement:** the detector's proposals generalised to "no scenario extra is implemented" — that universal was NOT applied, because only this one field was measured; the applied text says nothing is KNOWN to exercise the extension point. **Search performed:** `grep -c 'degraded_mode_response'` over all seven masters → obs-plan 3, every other master 0; over `crates/` → 0 at implement time, the single later hit being that chunk's own `live_suite_harvest.rs:62` doc comment ASSERTING the absence (a self-referential mention, not an implementation). Cascade: `.claude/rules/observability.md:15` carried the same field in its Format bullet and was re-derived in the same pass.

## 2026-09-06-run-report-envelope-conformance-gate — conductor-cli raises no spans of its own; the teardown WRITE is deliberately uninstrumented; the envelope list restored to eleven
**Section:** 4. Span / Trace Coverage (Auto-instrumentation per surface: cli row, conductor-report row) + 1. Obs Scope Summary (rusqlite row) + 6. Log Coverage (boundary-call wrappers, per-module levels, Required fields)
**Change:** SIX sites. (1) SS4 cli row: `#[tracing::instrument]` narrowed to the core scenario handlers the CLI drives - NOT `fn main()`, because `conductor-cli` declares no `tracing` dependency and carries no `tracing::` call site. (2) SS6 per-module levels, conductor-cli row: qualified to the same effect. (3) SS4 conductor-report row: the manual-span rule narrowed to the run-persist WRITE; the teardown WRITE (`RunsDb::delete_run`) mints NO span and no boundary log, deliberately - off the must-trace paths, and SS11's bounded span-name set stays `db.insert_run` alone. (4) SS1 rusqlite row: the same split. (5) SS6 boundary-call wrappers: a DB DELETE entry recording the deliberate non-logging. (6) SS6 Required fields: the list restored from TEN to the ELEVEN SS3 fields (`read_back_observed_at` was missing), the heading corrected from "every log line" to "every ENVELOPE record", and required restated as KEY PRESENCE rather than non-null.
**Why:** Sites 1-5 proposed by D-obs-instrumentation and ESCALATED, then operator-ratified at this wrap. Site 1 did not match playbook :109 on inspection: that rule dismisses the misattribution at line 48, and the :48 table's cli row indeed names only the tracing crate + subscriber - but a SECOND "Auto-instrumentation per surface" table at :280-288 carries the `fn main()` mandate, so the rule's qualifier failed and its note's claim "the mandate is NOT on the cli row" is incomplete. Sites 3-5 did not match playbook :40 either: that rule covers a span DEFERRED to a later epoch, whereas this one is never-minted by design (the operator closed SS11's set against a `db.*` wildcard on 2026-09-01). Site 6 is report expected amendment E1, which NO detector proposed - raised by the orchestrator under Validate check 5, routine because the report substantiates it; the count was settled at eleven by `2026-06-16-emission-journal-writer` and a11y-plan already carries the eleven-key form at :96-109 and :233-246, so this doc was the lone outlier.

## 2026-09-06-coverage-completeness-gate — Critical Path 6 re-based onto the allowlisted `message` field

**Section:** §4 Span/Trace Coverage → Scenario: Coverage-matrix completeness gate · §1 Obs Scope Summary → Critical paths (must-trace) table, Coverage-matrix row
**Change:** Both sites re-based onto what shipped. Must-trace spans: NONE — the gate mints no span; its observable is ONE boundary `info!` from `conductor_report::coverage_rollup` (the function every surface reaches; deliberately not `CoverageMatrix::render`, which is pure/no-IO and unreached without `--write`). Required span attributes: NONE — the counts ride the already-allowlisted `message` field. Required log fields: none beyond the §3 base set. The retired chain (`report.coverage_matrix_generate` → `db.query_all_p_ids` → `report.validate_coverage`) and its five attribute/field names are recorded as never-built and unbuildable as written: none of the three names is in §11's bounded span-name set, this section's own `conductor-report` row keeps that set at `db.insert_run` with no `db.*` widening (which its own `db.query_all_p_ids` contradicted), and the five names are absent from `ALLOWLISTED_FIELDS` so they would have been dropped at the processor stage. The §1 row's surface cell was also corrected: no `runs.db` query — the classification is a code-native static, not an aggregation over runs. The Denominator-semantics paragraph is UNCHANGED; the gate adopted its in-scope denominator and `(N unbacked)` auto qualifier verbatim.
**Why:** Playbook: a chunk that OPERATIONALIZES a spec'd gate for the FIRST time surfaces that the spec's own description names a stale field-list — routine, scoped to the operationalizing doc's own sections; that rule's note explicitly pre-empts this re-fire. Report Changes → Symbols/APIs, Coverage of new surfaces, and Expected amendments (1), which located both sites (`:135` and `:354-360`; 2 sites, 1 master). Operator-ratified at the chunk's P4 as the `message`-field re-base.

## 2026-09-06-halo-hue-budget-re-driven — P-025's hue bound is unmeasurable through its leaf, and the cause is quantization
**Section:** 4. Span / Trace Coverage — Known-residual classification path, Delegated-timing family
**Change:** The P-025 entry's recorded CAUSE for the over-budget `metric.constellation.hue_update_ms` reading is corrected from STALENESS to TICK QUANTIZATION, and the <=2s bound is recorded UNMEASURABLE through that leaf: `ServiceRegistryEntry.last_seen_unix_nano` has no ingest-path writer and is stamped only by Pulse's 15s lifecycle tick, so the leaf reports `t_sample - t_last_refreshing_tick`, U(0,15s) and independent of dispatch rate. P-025 now grades as a mechanism pin with no pass arm; the other three delegated leaves keep their harvest-tier budgets unchanged. The passage carries its measurement pointer and its SCOPE (this leaf, Pulse HEAD 83d4060; a SUT writer change lifts it, recorded as Pulse intake).
**Why:** The 2026-09-07 re-driven leg removed the only competing explanation: `halo-hue-encoding` emitted 360 dispatches at 2/s so the service never went quiet, and its one in-window sample still read 14525.9ms against 2000ms, matching its offset to the preceding tick (14527ms) to 1.1ms; all 7 samples in the capture fit duration = offset + k*15000, k in {0,1,2}, within 2ms. Staleness alone would have predicted ~0 for an emitting service.

## 2026-09-07-dependency-polish — §3 Transitive note: the default-features follow-up closed, its premise measured false
**Section:** §3 Observability Harness Contract → OTel SDK init
**Change:** The note now records that the trim shipped at the WORKSPACE opentelemetry-proto entry covering BOTH dep sites, and that it CANNOT drop `opentelemetry` / `opentelemetry_sdk` — at 0.32.0 the `trace` and `logs` features gate both the generated message modules conductor-emit imports and the SDK transform modules, and the crate declares `trace = [opentelemetry/trace, opentelemetry_sdk/trace]`. What the trim drops is `metrics`/`zpages`/`with-serde`/`internal-logs`, of which only `const-hex` left the tree. The behavioural invariant is untouched.
**Why:** Report Changes → Spec claims disproved by measurement #1: this section recorded the trim as the way to shed the dormant footprint, measured false against the vendored `opentelemetry-proto-0.32.0` source with both crates confirmed still in `Cargo.lock` post-trim.

## 2026-09-07-dependency-polish — Tier-justification crate count and stack version reconciled
**Section:** §1 Obs Scope Summary · §12 Obs Decisions Log
**Change:** "8 workspace crates" → 9 at both tier-justification sites; `tokio 1.48.x` → `1.52.3` at both instrumentation-scope table rows.
**Why:** Pre-existing stale literals the chunk's folded CARRY routes here; the workspace roster is measured unchanged at 9 (report Changes → Crates / modules). Raised by the orchestrator under Validate check 5 — obs-plan's own detectors are scoped to instrumentation, stack and redaction, so neither literal falls inside one.

## 2026-09-07-a11y-ci-gate — the envelope extension point is EXERCISED; the a11y violation artifact registered
**Section:** §3 Harness Contract → Log format JSON schema (the extension-point paragraph) · §9 CI Integration (artifact table)
**Change:** Retired "The extension point itself stands; nothing is known to exercise it today" — it IS exercised as of 2026-09-07 by the a11y CI gate's violation record at `runs/a11y/<run_id>.jsonl`, which carries the eleven envelope keys PLUS the §9 resource tags `service.name` (`conductor-ui`) and `deployment.environment`, measured at 13 top-level keys locally and 15 under CI where `ci.run.id` + `git.commit.sha` join them. The paragraph now states what the extras ARE — resource TAGS on a harness artifact, not scenario-specific fields, so the scenario record at `runs/<run_id>.jsonl` still carries the eleven alone — and why the superset is admitted: `journal_conformance` asserts key PRESENCE, closed sets and host-path freedom, never key exclusivity, which is exactly what lets ONE gate serve both shapes. §9's artifact table gains the record's row (uploaded via `actions/upload-artifact@v4`, conformance asserted in-job under `CONDUCTOR_RUNS_DIR=runs/a11y`).
**Why:** D-obs-instrumentation proposed the §3 retirement from the report's measured key counts; the §9 row was the plan's own `Expected amendments` entry, raised by the orchestrator under Validate check 5 because no detector proposed it (obs proposed §3 instead). Rule `:88` governs the §3 half — a chunk operationalizing a spec'd CI gate for the first time surfaces that the same spec's description of it is stale — and the report demonstrates the gate passes over the real record (8/8). Basis: report Changes → Schema/config (13/15 measured) and Outcome (the UNMET-as-worded criterion routed here rather than reported met).

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — the fmt gate joins the build-time verification set and the Lint row
**Section:** §1 Obs Scope Summary (instrumentation-scope table, `cargo build / CI/CD pipeline` row) · §9 CI Integration (Pipeline integration, Lint / typecheck row)
**Change:** TWO edits. (1) §1's Not-instrumentable row listed build-time verification as "(`cargo build`, cargo-nextest, cargo clippy)"; `cargo fmt --all --check` joins it — the fmt gate is build-time and runs at no runtime, so its classification is unchanged. (2) §9's Lint / typecheck row named only `cargo clippy` warnings to stderr; it now also names the fmt gate's unified diff to stdout, records that it ships as the `rust` job's own early step at index 2 rather than colocated with clippy, and states that the diff's `Diff in {file}` lines are agent-readable from the job log but are NOT written into any telemetry artifact — they carry absolute host paths, which §9's own log-conformance check rejects.
**Why:** Edit (2) is the plan's `Expected amendments (wrap)` entry; **no detector proposed it** (obs-plan returned a bare `proposals: []`), so it was raised by the ORCHESTRATOR under Validate check 5 — the plan's list is the chunk's coverage floor and may not under-run silently. Edit (1) came from the cascade's cross-master sweep for the gate-set enumeration (`grep -nE 'cargo build.*(nextest|clippy)|…'`, control fired at `architecture.md:37`). The host-path clause in (2) is stated because the report's own *Coverage of new surfaces* row classes the gate `instrumentation n/a`, and §9's log-conformance check would fail on such a line if it were ever routed into `agent-latest.jsonl`.
