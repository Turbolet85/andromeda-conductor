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
