# Obs Plan — Amendments

_Append-only changelog of amendments to `obs-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — two record shapes clarified (self-obs base line vs Run-report envelope)
**Section:** §3 Observability Harness Contract / Log format JSON schema
**Change:** the §3 schema block is the Run-report envelope (scenario-result record — emission journal + `runs.db`, report seam Epoch 6); the foundational self-obs log line carries a base set on every line — `timestamp_ms` (epoch millis, `std::time`), `level`, `target`, service-identity, `run_id` — with envelope/result fields only on scenario-result events. The implementation uses a custom `tracing-subscriber` layer (stock `fmt().json()` cannot emit constant identity fields flat).
**Why:** the chunk implemented the self-obs stream, whose line schema justifiably differs from the envelope the §3 block showed (foundational lines cannot carry verdict/latency yet), so §3 was reconciled. Self-obs is tracing JSON with no OTel SDK. Redaction was deferred to `pii-scrubbing-wire` (the next chunk; synthetic-only, no leak), ratified by the user as correct sequencing with no obs body edit; a `playbook.md` rule now keeps Foundation deferrals of separately-sequenced concerns from escalating.
**Ref:** .andromeda/runs/2026-06-15T17-46-44-wrap/

## 2026-06-15-log-error-boundary-redaction — redaction model reconciled (host-file-path anchor; struct-name guard = allowlist + Display; `target` preserved)
**Section:** §6 Log conformance check (obs CI gate) · §11 Anti-Patterns (Logs · PII Scrubbing)
**Change:** the redaction model matches the implemented `conductor-core::redact`: the value scrub masks absolute host-FILE paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>`, NOT `::`-type tokens; internal struct names are kept out by the field-name allowlist (drops non-allowlisted, incl. Debug-dumped, field names) + `Display`-not-`Debug` at the `anyhow` edge; the allowlisted `target` module path (`module::`-shaped by design) is explicitly preserved. The §6 CI conformance gate no longer treats a `module::` prefix as a struct-name leak (was: `path::`/`module::`/`backtrace` listed as token-level redaction targets).
**Why:** the redaction layer deferred from `2026-06-15-structured-logging-stack` landed. Blanket `::`-redaction would gut the allowlisted `target` correlation field and mangle std type names in panics (`Option::unwrap`); §11's "paths" means absolute *file* paths, and the host-path anchor + allowlist + Display edge realize "no host paths / no struct names" without that breakage. Ratified by the user at the 2026-06-15 wrap. `run_id` + `std::time` preserved; no OTel.
**Ref:** .andromeda/runs/2026-06-15T18-54-57-wrap/

## 2026-06-16-emission-journal-writer — Run-report envelope gains `read_back_observed_at` (realign to owner test-plan §3)
**Section:** §3 Logging stack + §3 Harness Contract / Log format JSON schema + §6 Log Coverage (all four envelope reproductions)
**Change:** `read_back_observed_at` (ISO-8601 from `std::time`; null until read-back, null for blocked rows) is the 2nd field of the JSONL Run-report envelope, after `journal_emitted_at` — the schema is now 11 fields (was 10). `latency_ms` stays `read_back_observed_at − journal_emitted_at`.
**Why:** obs-plan §3 was the lone doc omitting the field, while the schema OWNER test-plan §3, arch §Standard Contracts and the `runs.db` data model all carry it; the chunk had implemented obs-plan's 10-field outlier. Resolved with the user (Option A): obs §3 realigned to the owner and the chunk's `RunRecord` fixed to 11 fields.
**Ref:** .andromeda/runs/2026-06-16T21-43-46-wrap/

## 2026-06-17-raw-otlp-message-scaffold — no-SDK invariant clarified as behavioral; dormant transitive OTel SDK noted
**Section:** §3 OTel SDK init
**Change:** the "no OTel SDK for self-observation" invariant is BEHAVIORAL — no SDK is ever initialized/used for self-obs (the harm is batch tasks breaking `current_thread` determinism + polluting the PRODUCT stream). opentelemetry-proto's default features transitively pull `opentelemetry` + `opentelemetry_sdk` (dormant, never initialized, audit/deny-green); their mere presence is not a violation. A follow-up is recorded to evaluate `default-features = false` on the opentelemetry-proto dep.
**Why:** opentelemetry-proto (the PRODUCT proto lib) pulls the OTel SDK crates transitively while self-obs stays tracing JSON with no SDK init, so the behavioral invariant holds. Resolved with the user on 2026-06-17 (accept + document + follow-up); a `playbook.md` rule keeps a dormant transitive SDK from re-escalating.
**Ref:** .andromeda/runs/2026-06-17T23-06-06-wrap/

## 2026-06-18-severity-logs — `emit.logs_batch` added to the bounded span-name set
**Section:** §11 Anti-Patterns (Spans / Traces — bounded span name set)
**Change:** added `emit.logs_batch` (the OTLP logs-egress self-observation span on `LogsEmitter::export`, `record_count` attribute) to the bounded span-name set, alongside `emit.batch` (trace egress) — distinguishing logs egress from trace egress in self-obs.
**Why:** the severity-logs chunk (P-007) shipped a new `LogsEmitter` whose egress is instrumented with a new low-cardinality span name following the `{module}.{operation}` convention; routine spec-to-sound-impl alignment — a conforming member is added, so the bounded-set invariant is preserved.
**Kept:** test-plan §3 unedited — the harness, envelope and log format are unchanged, so the bind stays consistent.
**Ref:** .andromeda/runs/2026-06-18T16-49-03Z-wrap/

## 2026-06-24-sanitized-stderr-agent-mode-logging — §3 agent-mode flag reworded to the read-only CONDUCTOR_AGENT_MODE trigger (D4)
**Section:** §3 Observability Harness Contract (Logging stack — agent-mode flag, both the summary + detailed sink blocks)
**Change:** was "`--agent-mode` … sets `CONDUCTOR_AGENT_MODE=1`"; now agent mode is triggered by the flag OR the `CONDUCTOR_AGENT_MODE` env, a **read-only trigger** Conductor reads (`agent_mode = flag || env-set`) and never WRITES (avoids edition-2024 `unsafe std::env::set_var`; the harness/operator exports it). Observable mode is identical.
**Why:** routine spec-wording to sound-impl reconciliation (the 2026-06-15 playbook rule); the observable invariant is proven by the agent-mode E2E + the obs file-sink units. The new file sink REUSES the unchanged field-allowlist + `redact_value`, and consuming unchanged hardened infra is not a new boundary; it is std-only, no OTel SDK.
**Kept:** the pre-existing "pretty-print in dev" wording — not this chunk's drift (JSON-to-stderr in dev predates it), out of scope this wrap.
**Ref:** .andromeda/runs/2026-06-24T20-38-37-wrap/

## 2026-06-27-obs-ci-conformance-gate — §9 conformance check asserts the §3 self-obs base schema (not the §6 envelope)
**Section:** §9 CI Integration — Log conformance check
**Change:** the `logs/agent-latest.jsonl` conformance gate validates the §3 self-obs base-line schema (`timestamp_ms`/`level`/`target`/`service.name`/`service.version`/`deployment.environment`/`run_id`), NOT the §6 run-report ENVELOPE (was: §9's field list named the §6 envelope); the envelope (`runs/<run_id>.jsonl`) gets its own, not-yet-built gate. The two record shapes are distinct.
**Why:** this chunk operationalized the §9 gate for the first time, and the shipped gate correctly asserts the §3 base schema (PASS on the real artifact, FAIL on a missing base field); §9 had mis-referenced §6, the wrong record shape for the artifact it reads, while test-plan §3 (the envelope-schema owner) was already correct. Confirmed with the user on 2026-06-27; a `playbook.md` rule keeps future first-operationalization gate-wording reconciles from re-escalating.
**Kept:** the §9 zero-unlogged-panics gate (greps `agent-latest.jsonl` + stderr) — it already matched the impl.
**Ref:** .andromeda/runs/2026-06-27T15-18-37-wrap/

## 2026-08-08-sut-capability-manifest — De-hardcoded coverage-gate span attributes
**Section:** §4 Span/Trace Coverage (coverage-matrix completeness gate) · §1 must-trace table
**Change:** `db.query_all_p_ids`'s `p_id_count_expected` and the `p_ids` log field name the manifest's capability count/set rather than the literal 60 (was: a fixed 60).
**Why:** a SUT-advance reversal — the span spec asserted a fixed 60 the accepted set no longer defines.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-09-out-of-scope-classification-treatment — coverage_percent's denominator pinned to the in-scope count
**Section:** §4 Span/Trace Coverage — Scenario: Coverage-matrix completeness gate
**Change:** the gate's attributes carry denominator semantics: `p_id_count_expected` is the FULL manifest capability count, while `coverage_percent` is computed over the **in-scope** count (the manifest set minus the out-of-Conductor's-remit rows). Both stay manifest-derived, never literals.
**Why:** the chunk's roll-up renders that split on all three surfaces, which pre-binds `coverage_percent`'s meaning for the not-yet-built Epoch-6 completeness gate; unrecorded, the gate's 82-row expectation and the shipped 66-row in-scope denominator could diverge unnoticed. The pre-binding was an operator decision at /andromeda-phase P4.
**Ref:** .andromeda/runs/2026-08-09T15-56-31-wrap/

## 2026-08-09-interpretation-correctness-posture — coverage roll-up auto term carries a derived unbacked qualifier
**Section:** §4 (coverage-matrix completeness gate — denominator semantics)
**Change:** the roll-up's auto term carries a derived `(N unbacked)` qualifier read from `conductor_core::UNBACKED_AUTO`; it qualifies the auto count rather than joining the breakdown, so the per-mode summands still sum to the row count and a gate reading these fields must not treat it as a fifth mode.
**Why:** the chunk shipped the qualifier on all three roll-up surfaces the gate's denominator semantics already govern.
**Ref:** .andromeda/runs/2026-08-09T19-30-00-wrap/

## 2026-08-10-scenario-run-root-span-tree — the root's cleanup no longer claims the report-seam spans as descendants
**Section:** §4 Span/Trace Coverage — Critical Path 1 Cleanup · Known-residual classification path Cleanup
**Change:** both Cleanup lines dropped "Root `scenario.run` closes on `db.insert_run` completion". The root now opens per-scenario at the composition root (`conductor-run::execute_scenario`) and closes when that scenario returns; `report.generate` / `db.insert_run` are run-scoped SIBLINGS correlated by `run_id`, not descendants.
**Why:** the retired claim was unachievable — `persist` is called ALONGSIDE `execute_scenario` at all three production sites (run, suite, `drive_run`), and under a suite one `persist` serves N scenarios whose roots have already closed, so no single `scenario.run` can contain `db.insert_run`. Root placement was an operator decision at /andromeda-phase P4.
**Ref:** .andromeda/runs/2026-08-11T15-45-25-wrap/

## 2026-08-10-scenario-run-root-span-tree — Tauri command-span names reconciled to the shipped handlers
**Section:** §1 Harness contract (desktop-webview row) · §4 Both-surface parity path · §4 cross-surface correlation note
**Change:** `start_scenario()` / `stop_scenario()` / `get_run_report()` / `operator_pause_go_no_go()` → the shipped `start_run()` / `stop_run()` / `run_report()` / `resolve_operator_hold()`, and the two `tauri.command.start_scenario` span references → `tauri.command.start_run`.
**Why:** pre-existing drift this chunk did not introduce: the illustrative names matched no shipped handler. §11's bounded set uses the `tauri.command.*` wildcard, so no invariant was violated — only the illustrative names were false. Fixed here because it sits in the sections this chunk amended, under an explicit operator directive.
**Ref:** .andromeda/runs/2026-08-11T15-45-25-wrap/

## 2026-08-10-scenario-run-root-span-tree — the self-obs line's span-lifecycle variant recorded
**Section:** §3 Log format JSON schema (two record shapes)
**Change:** the custom layer emits the self-obs line in two variants over the same base set — the event line, and the span-lifecycle line adding `span` / `span_event` (`new` | `close`) / optional `parent` plus the span's allowlisted attributes on `new` — so a §4 span materializes as real lines. The two-record-shapes split (self-obs line vs Run-report envelope) is unaffected.
**Why:** the lateral test-plan §3 ↔ obs-plan §3 bind: test-plan §3 OWNS the JSONL format and gained the variant this chunk shipped, so leaving obs §3 unamended would be a one-sided change. Before this chunk the layer implemented only `on_event`, so no span emitted anything at all.
**Ref:** .andromeda/runs/2026-08-11T15-45-25-wrap/
## 2026-08-11-faithful-emission-dispatcher — scenario-config validation ban restated
**Section:** §11 Anti-Patterns (project-specific bans)
**Change:** the never-skip-garde-validation ban names the shipped error-fraction encoding (`error_percent` ∈ 0..=100; was `error fraction ∈ [0,1]`) and adds that a nested spec field must `dive`, never `skip` — a skipped struct is never descended into, so its rules never run.
**Why:** the ban quoted a bound the shipped model does not carry in that form, and the chunk's `#[garde(skip)]` finding gave the ban a concrete failure mode it did not previously name.
**Ref:** .andromeda/runs/2026-08-13T16-43-31-wrap/

## 2026-08-13-per-check-read-back-extraction — degraded read-back is OBSERVED, not requested
**Section:** §1 Critical paths (Known-residual row) · §4 Span/Trace Coverage (CP1 `verify.readback`,
Fingerprint-storm `verify.readback_fingerprints`, CP5 Known-residual classification path)
**Change:**
- CP5's must-trace span `verify.readback_degraded_mode` (MCP call with `degraded_mode=true`) → `verify.readback.observe` (the read-back pass whose `retrieve_report` result REPORTS `degraded_mode`), at both the §1 table row and the §4 scenario.
- Its required attributes `degraded_mode_requested` / `response_received` are retired: the degraded signal is observed, not requested, and rides a `warn` line on the allowlisted `message` field.
- §4's Required-span-attributes heading states that a span attribute must be a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it.
- `mcp_method` → `mcp_tool` on both `verify.readback` and `verify.readback_fingerprints`.
- CP5's Required log fields gained the missing `read_back_observed_at` (the eleven-field envelope every other critical path carries) and keep the scenario extra `degraded_mode_response`, noting the two record shapes are governed differently — an envelope extra by the report seam, a span attribute by the allowlist.
**Why:** the chunk is `retrieve_report`'s FIRST caller in the workspace, so it operationalizes this path. Pulse computes `degraded_mode` as `parsed_l4.is_none()` and returns it in the result, so `degraded_mode_requested` describes a call Conductor cannot make; and neither retired attribute name is in `ALLOWLISTED_FIELDS`, so a built attribute would emit nothing. The shipped span field is `mcp_tool`. Routine spec-illustration → sound-impl alignment, and operationalizing surfaced the spec's own stale field list.
**Ref:** .andromeda/runs/2026-08-13T19-58-45-wrap/

## 2026-08-13-first-live-green-preflight — read-back boundary log gains the shape witness
**Section:** §6 Boundary-call wrappers (must-log events)
**Change:** the MCP-readback must-log set gains the observed KEY SET of each raw tool result, and the tool list is completed with `retrieve_telemetry_slice`. Key names only, never values, carried on the `message` field because a field name outside the allowlist is dropped at the processor stage.
**Why:** the extraction readers degrade to empty on an unrecognized shape rather than erroring, so without the witness a live field-name divergence is indistinguishable from an empty corpus. The first live leg confirmed the witness works, matching the committed baseline exactly.
**Ref:** .andromeda/runs/2026-08-13T22-48-53-wrap/

## 2026-08-14-canary-fingerprint-feed-capture — emit.batch wire-shape witness + the additive RUST_LOG form
**Section:** §6 Log Coverage (Boundary-call wrappers) · §3 Harness Contract (Per-module log levels) · §11 Anti-Patterns
**Change:** §6's `emit.batch` must-log list records the wire-shape witness — observed span count, spans a receiver would skip for a missing `trace_id`/`span_id`, and the distinct event / event-attribute KEY NAMES of the outbound request (names only, ordered), emitted at `debug` inside the existing `#[instrument]` span and carried on the already-allowlisted `message` field, so it needs no new allowlist entry. Separately, the §3 per-module level table and the §11 hot-path rule state the ADDITIVE form `RUST_LOG=info,{crate}=debug`: a bare per-target directive replaces the default rather than adding to it.
**Why:** the witness is the emitting-side twin of the `verify.readback` key-set witness — a receiver that degrades to empty on an unrecognized shape makes a divergence indistinguishable from emptiness. The RUST_LOG correction was measured: the bare form fails the CLI's own agent-mode self-obs test while `info,conductor_emit=debug` passes, and the old table was internally inconsistent under literal use (it would silence the very crates it sets to `info`).
**Ref:** .andromeda/runs/2026-08-14T16-51-43-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — fingerprint match count is not computable
**Section:** §4 Span / Trace Coverage (Fingerprint-storm scenario → required span attributes)
**Change:** `verify.readback_fingerprints`'s `fingerprints_matched_count` → `fingerprints_read_back_count` — a count of what read-back returned, with a note that an emitted-vs-read-back MATCH is not computable because Pulse exposes no read-back surface carrying its own computed fingerprint (`fingerprint_refs` is L4-authored and `[]` under deterministic L4; `span_events.fingerprint` has zero reads in `mcp-server`).
**Why:** a required span attribute defined as a match count between two values that never meet is uninstrumentable as written; measured 2026-08-16 and confirmed live.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-fault-application-spans — fault spans open where the faults actually are
**Section:** §4 Span / Trace Coverage (Fault-injection spans — heading, the three attribute lists, and the
placement paragraph) · §1 Obs Scope Summary (instrumentation-scope `conductor-faults` row · telemetry-trigger
`chaos-instrumentation` row) · §6 Log Coverage (log-levels table `info` + `debug` rows, plus a why-note)
**Change:** four corrections.
- (a) PLACEMENT — "Each fault span wraps the fault application phase in conductor-faults" is retired. The run-path faults are declarative phase data (`EmissionSpec { occurrences: 0 }` and `EmissionShape::Ramp`), so `fault.silence` / `fault.ramp` open from a caller-supplied per-phase hook (`run_timeline_observed` / `PhaseWindow`, fault semantics + the `std::time` journal basis supplied by `conductor-run`), created but NOT entered so `emit.batch` keeps `timeline.execute` as its parent. Only `fault.port_occupier` opens inside `conductor-faults`, at its RAII bind site. The §1 crate row realigned with it.
- (b) `ramp_factor` — range was `0.0-1.0`; now **−1.0..1.0**, the normalized signed slope `(to_rate − from_rate) / max(from_rate, to_rate)`, so a rise and a fall are distinguishable.
- (c) `fault.port_occupier` — attributes narrowed to `fault_type` + `port`; the realized hold is witnessed on the allowlisted `message` field at `debug` on release; its `timeline.execute` parentage recorded as CONDITIONAL — no run path drives the occupier yet. The §1 trigger row realigned with it.
- (d) LOG LEVEL — fault-span lifecycle moved from the `debug` row to `info`, with the reasoning beneath the table.
**Why:** (a) `conductor-faults` has no crate edges in either direction and no outside references to its fault types, so a span placed there fires on no scenario run. (b) the shipped `EmissionShape::Ramp` carries `from_rate`/`to_rate`/`windows` integers — no 0.0-1.0 quantity exists; the signed form was the operator's decision at phase P4. (c) neither duration nor journal offset is knowable at bind time and the layer records attributes on `new` alone, so an attribute uncomputable then is uninstrumentable; claiming a parent the code cannot produce repeats what the 2026-08-10 root-span cleanup corrected. (d) at `debug` the spans are invisible under the default INFO filter without a `RUST_LOG` opt-in; every sibling span is `info`, and one span per phase is not the hot path §11's ban targets.
**Ref:** .andromeda/runs/2026-08-16T11-53-35-wrap/

## 2026-08-18-error-baseline-spike-live-proof — fingerprints MAY-BE-EMPTY-under-L4 superseded (three sites)
**Section:** §4 Fingerprint-storm Required log fields · §4 verify.readback_fingerprints bullet · §1 critical-paths table Fingerprint-storm row
**Change:** "MAY BE EMPTY / pinned `[]` under deterministic L4" retired at all three sites — the fixture's `evidence_refs` now populates `fingerprint_refs` with a constant `det-*` triple (measured 2026-08-18, SUT HEAD `efabe8e`); the emitted-vs-read-back match stays non-computable (payload-invariant fixture constants), and `fingerprints_read_back_count` + the harvest surface stay the storm-identity evidence.
**Why:** the live envelope measured the 3 `det-*` refs; the 2026-08-16 amendment's supporting fact (emptiness) is superseded while its conclusion (no identity carrier) stands.
**Supersedes:** 2026-08-16-fingerprint-storm-live-proof — fingerprint-storm `fingerprints` no longer "populated"
**Ref:** .andromeda/runs/2026-08-18T19-10-05-wrap/

## 2026-08-18-restart-suppression-live-proof — restart-suppression row retargeted to the shipped declare-only instrumentation
**Section:** §1 Critical paths table (Restart-suppression row) · §4 Scenario: Restart-suppression scenario incl. bypass case · §6 Log Coverage (scenario-specific fields) · §3 Log format JSON schema (additional-fields example)
**Change:** the never-built family span chain (`timeline.execute_restart_suppression` / per-path `emit.batch` `path_type` / `verify.readback_suppression_bypass`) and the `bypass_triggered` envelope extra are retired at all four statement sites. The row records the shipped instrumentation — `scenario.run` → `timeline.execute` + two `fault.silence` children → `emit.batch` → `verify.readback*` → `report.generate`, the standard eleven-field envelope — plus the measured constraints: suppression/bypass evidence grades at the harvest tier over Pulse's own lines, the `triage.cue.tick` counters read `"<redacted>"` live, and `persistence_seconds` is the service's cumulative sample count.
**Why:** the scenario landed declare-only (both `[[expected]]` checks structurally ungradeable under deterministic L4), and the live leg measured the actual span/field surface.
**Ref:** .andromeda/runs/2026-08-18T21-55-43-wrap/

## 2026-08-19-connection-lifecycle-live-proof — `fault.port_occupier` parentage CONDITIONAL → measured real
**Section:** §4 Span / Trace Coverage — Fault-injection spans ("Where each span opens")
**Change:** the occupier span IS a child of `timeline.execute` (was: parentage CONDITIONAL) — measured live on the conflict leg (span_event `new`+`close`, `parent: "timeline.execute"`, attributes exactly `fault_type` + `port`); `conductor-run`'s `phase_guard` drives it for a fault-declared phase's window and the RAII release is the scheduler's boundary drop.
**Why:** the 2026-08-16 amendment recorded the parentage CONDITIONAL because no driver existed; the v2-15 driver shipped this chunk and the live leg measured the condition real.
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/

## 2026-08-20-latency-regression-re-proof — sidecar-spawn citation re-based
**Section:** §Obs Anti-Patterns
**Change:** the cross-master citation of security-plan's sidecar-spawn ban reads "fixed hard-coded program NAME resolved through the inherited `PATH` + `.env(...)` only" (was "fixed program path + `.env(...)` only").
**Why:** obs-plan cited security-plan's retired wording after security-plan's 2026-08-20 spawn-wording amendment. The ban's substance (never a shell with operator-supplied input) is unchanged.
**Ref:** .andromeda/runs/2026-08-20T17-10-10-wrap/
## 2026-08-21-severity-lifecycle-live-proof — CP4's family-specific span chain retired as never-built
**Section:** §4 Span / Trace Coverage — Scenario: Severity-lifecycle full pass · §1 Obs Scope Summary —
critical-paths row 4 · §6 Log Coverage — additional scenario-specific fields · §3 Harness Contract — the
additional-fields example
**Change:** the mandated chain (`timeline.execute_severity_lifecycle` / per-transition `emit.batch` `severity_level`+`phase` / `verify.readback_auto_resolve` / `verify.readback_resolution_summary`) is RETIRED as never-built and replaced by the shipped one (`scenario.run` → `timeline.execute` → `emit.batch` → `verify.readback*` → `report.generate`); the envelope is the standard eleven fields with `verdict` null and `state` "KnownResidual", with the `lifecycle_phase` / `severity_choice_calibrated` extras removed at all three sites that named them. The lifecycle evidence lives in Pulse's own lines at the harvest tier.
**Why:** measured 2026-08-21 — all four span names plus `auto_resolve_triggered` / `summary_received` have ZERO occurrences in `crates/`; read-back is one `verify.readback.observe` pass, not a per-claim call; and the family landed declare-only, so no per-phase or severity-calibration extra is written. The auto-resolve witness had to move because `triage.incident.auto_resolve.tick`'s counters read `"<redacted>"` on the wire (Pulse's default-deny allowlist predates them). Same retirement class as the restart-suppression re-base.
**Ref:** .andromeda/runs/2026-08-21T09-50-00-wrap/

## 2026-08-21-per-check-latency-measurement — Second report-seam journal line shape
**Section:** Section 3 Harness Contract -> Log format (two record shapes)
**Change:** the report seam's SECOND journal line shape — the per-check `CheckRecord` — is recorded as a finer GRAIN beneath the envelope, never an extension of it; its format is owned by test-plan Section 3; neither report-seam shape passes through the span-attribute allowlist.
**Why:** obs Section 3 reproduces the owner's journal format and the test-plan ↔ obs-plan bind is two-sided, so the owner's amendment must land here too. `redact.rs` needed no admission: the field allowlist governs the self-obs line only.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-delegated-timing-budgets-proven — Delegated-timing budgets recorded as a harvest-tier path
**Section:** §4 -> Scenario: Known-residual classification path (detail block; the §4 critical-path TABLE is unchanged)
**Change:** the delegated-timing family is recorded under the Known-residual path it reaches: the three declare-only members add NO critical path of their own; their budgets grade at the harvest tier over Pulse's own tracing lines, each from its OWN exact allowlist leaf and never through `budget_ms`/`effective_deadline_ms`; the duration field is `duration_ms` on three leaves but `value` on `metric.report.render_ms`; and `findings-counter-refresh` (one `[[expected]]`) reaches the same state by the degraded read-back with its unmet `CountAtLeast` floor grading `CalibrationRegion`.
**Why:** no detector covers adding a harvest family to §4, so the orchestrator raised it. The `budget_ms` distinction was confirmed live (envelope `latency_ms` of seconds vs a ~2ms median in the same leg).
**Kept:** the §4 critical-path table — its per-scenario blocks are the fixed 7-path set §4's Minimal-tier justification rests on (obs-plan and `.claude/rules/observability.md` both state 7), so an eighth row would move a tier-justification input this chunk never measured.
**Ref:** NOT DERIVED

## 2026-08-22-operator-pause-and-checklist-live-firing — the self-obs sink corrected: stderr, never stdout; JSON, never pretty
**Section:** §1 Obs Scope Summary (telemetry-surfaces table cli + desktop-webview rows; logging-stack sinks; log-file location; multi-platform-exporter-compat) · §3 Logging stack (sinks, agent-mode flag, log-file location, paste-to-AI, correlation) · §6 Log Coverage (sink configuration) · §11 Anti-Patterns (three Universal bans) · §12 Decisions Log (dated annotation)
**Change:** 21 sites corrected across THREE stale wordings of one false premise. `build_subscriber` installs exactly one layer, `JsonObsLayer`, over every writer, and `ObsSink` has only `{Stderr, File}` — so (a) "pretty-print in dev" is retired everywhere (no pretty layer exists on any path), (b) every self-obs "stdout" is now stderr (nothing in the self-obs stack writes stdout), and (c) the Tauri backend's "stderr (dev only)" dual-sink framing is retired: the file sink is UNCONDITIONAL with stderr only as an open-failure fallback. Agent mode is a WRITER switch, not a format switch. §12 is history, so it carries a dated correction rather than a rewrite.
**Why:** the chunk measured the claim false at source on every path (the obs module, the CLI and Tauri mains) and corrected the source twin in `obs.rs` in the same pass. The 2026-06-24 amendment had already flagged the pretty wording as unreconciled.
**Kept:** §1's "structured output to stdout/stderr" — it is the CLI PRODUCT render and stays true.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-08-31-p-075-assert-round — The MCP boundary carries a WRITE; §6 must-log set names it
**Section:** §6 Log Coverage → Boundary-call wrappers · §1 Obs Scope Summary → conductor-verify row · §1 → Pulse MCP server row
**Change:** §6 gains an explicit lifecycle-write bullet: the same bounded `verify.readback.call_tool` span and allowlisted `mcp_tool` attribute as the read-back calls, plus the applied/declined outcome, with the INCIDENT ID carried on the allowlisted `message` field because `incident_id` is absent from `redact::ALLOWLISTED_FIELDS` and a dedicated attribute would emit nothing; the declined arm is recorded stub-only and permanently so. §1's two table rows lose their observation-only framing. The conductor-verify row was "MCP read-back client via rmcp 1.7.0" (rmcp removed 2026-06-27); now the hand-rolled line-delimited JSON-RPC transport.
**Why:** `conductor_run::probe_resolve_lifecycle` is the first production caller of `mark_incident_resolved`; `ALLOWLISTED_FIELDS` is unchanged. The rmcp clause sat inside the very sentence this amendment rewrites, so leaving it would preserve a known-false claim in an authored line; the operator approved the incidental correction and declined dismissing it as pre-existing drift.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — the command-span MECHANISM, and the handler count
**Section:** §4 Span/Trace Coverage → auto-instrumentation table, desktop-webview row · §1 Obs Scope Summary → desktop-webview row · §1 → ipc-internal row
**Change:** all three sites retire `#[tracing::instrument]` as the Tauri command-handler mechanism in favour of the manual `tracing::info_span!("tauri.command.<name>").entered()` guard, because the ATTRIBUTE does not stack with `#[tauri::command]` (the command macro rewrites the fn signature for IPC arg-extraction). The §1 desktop-webview row's handler enumeration was four; now the measured **seven** — `list_scenarios` · `coverage_matrix` · `unbacked_auto` · `run_report` · `start_run` · `stop_run` · `resolve_operator_hold`.
**Why:** the code carries the prescribed manual span at 7/7 sites, re-verified by the operator — coverage was never missing; the DOC was the divergent side, still prescribing an attribute that cannot stack (an early "zero `#[tracing::instrument]`" obs-gap finding was retracted). Span names stay inside §11's bounded `tauri.command.*` set, and `.claude/rules/observability.md` had carried the correct mechanism since 2026-06-26.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — eighth handler, the read-path convention, and a self-contradiction retired
**Section:** 1 Obs Scope Summary (telemetry-surfaces desktop-webview row + Notes + rusqlite instrumentation-scope row) + 4 Span/Trace Coverage (desktop-webview and conductor-report rows) + 6 Log Coverage (Boundary-call wrappers)
**Change:**
- (a) Instrumented `#[tauri::command]` handlers 7 → 8 with `run_envelope()` in the enumeration (section 1) and the site count 7/7 → 8/8 (section 4).
- (b) Deleted section 1's trailing "instrument via `#[tracing::instrument]`" clause, which contradicted the same sentence's statement that the attribute does not stack with `#[tauri::command]`.
- (c) The READ-path convention at three sites: a rusqlite read inside a `tauri.command.*` span logs a section-6 Boundary-call `info!` carrying `run_id` plus the outcome on the allowlisted `message` field and mints NO `db.*` read span; section 4's conductor-report row narrowed to WRITE queries; section 1's rusqlite row split into write-span / read-log.
**Why:** (a) the chunk registered an eighth command. (b) a measured-false spec claim. (c) the operator ruled that the bounded span-name set is a deliberate contract and is NOT widened with a `db.*` wildcard; the in-repo DB-span convention is an attribute on `RunsDb` methods in `conductor-report`, so a `db.*` read span stays a design option for the entry that next touches that crate.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — the Tauri sink is runs-dir-relative; a measured host-path channel outside the redaction sites
**Section:** 1 Obs Scope Summary (telemetry-surfaces desktop-webview row · Logging stack sink · Log file location) + 3 Log file location + Logging stack sink + 6 Sink configuration + 11 PII Scrubbing + 11 Project-specific bans + 12 Decisions Log (Exporter)
**Change:**
- (1) Every `logs/conductor-tauri.jsonl` restatement qualified: the file NAME is fixed, the DIRECTORY rides `CONDUCTOR_RUNS_DIR` (`runs_dir.parent()/logs`) — the project-root `logs/` only with the handle unset; `runs/logs/` · `runs/driven/logs/` · `runs/sr-leg/logs/` on the a11y suites; §12's Exporter bullet gains a dated correction.
- (2) §11 PII Scrubbing records the measured gap outside the three application sites — the flagless sidecar spawn's console pane publishes the sidecar's absolute exe path as a foreground window title — and the sidecar ban gains "never with a visible console window"; the `CREATE_NO_WINDOW`-class fix is route-owned, not shipped.
**Why:** (1) the chunk moved the driven arm's runs dir and three landing sites were measured in one day; the bare literal pointed at a file that exists on no a11y suite. "Never the project-root `logs/`" was over-general, so it applies with the unset-handle scope. (2) NVDA spoke the pane's title (`security_finding`); only the leg's own scrub kept it out of the committed record. Resolved by operator directive as a Conductor spawn DEFECT, a route candidate of its own.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-02-cross-surface-envelope-parity — §4's two rmcp attributions retired
**Section:** §4 Span / Trace Coverage — Span kinds (Client) · the auto-instrumentation per-surface table (conductor-verify row)
**Change:** the Client span-kind line reads "MCP readback over the hand-rolled line-delimited JSON-RPC client"; the auto-instrumentation table's conductor-verify cell reads "None (the hand-rolled line-delimited JSON-RPC MCP client is a seam)" (both were rmcp attributions). The manual-span cell is unchanged.
**Why:** a measured-false spec claim; these two sites were missed by the 2026-08-31 sweep of the same defect class, which corrected only §1's conductor-verify row, so this completes that sweep.
**Kept:** §11's `CVE-2026-30623 rmcp STDIO flaw` mention — a vulnerability-CLASS citation — and §1's already-reconciled "rmcp removed 2026-06-27" notes.
**Ref:** .andromeda/runs/2026-09-02T14-34-37Z-wrap/

## 2026-09-04-sidecar-spawn-without-a-console-window — the host-path channel this plan enumerated is closed
**Section:** §11 Obs Anti-Patterns — PII Scrubbing (primary) · §11 Obs Anti-Patterns — Project-specific bans
**Change:** both sites retire the "route-owned, not shipped" status of the `CREATE_NO_WINDOW`-class fix and record the channel as CLOSED, citing symbol names instead of a line locator. §11 PII Scrubbing keeps its standing warning intact — the enumeration bounds the sites it names, never every channel — while recording that this gap is closed and that the a11y leg's `<host-path>` scrub is a second line of defence rather than the only one. §11 Project-specific bans keeps the ban and its "never with a visible console window" clause, restating the suppression as shipped and test-held, and records that the APPLIED flag is not assertable at the unit tier (no creation-flags getter on `std::process::Command`), so its effect is SR-leg evidence.
**Why:** the chunk shipped the fix, and the SR leg measured the channel closed (no host path in the pane's utterances, no `security_finding`); the 2026-09-02 reading measured the pre-fix state. Routine: a suppression flag admits nothing new across the boundary, so it is not boundary widening. The obs↔security lateral bind holds — security-plan §Anti-Patterns carries the identical retirement, and this cross-master citation must move on both sides together.
**Kept:** §3/§4/§6 untouched — the fix adds no span, no log line and no field.
**Ref:** .andromeda/runs/2026-09-04T20-15-00-wrap/

## 2026-09-06-operator-gated-live-suite — `degraded_mode_response` retired as never implemented
**Section:** §3 Observability Harness Contract — Log format JSON schema · §4 Span/Trace Coverage — Known-residual classification path (Required log fields) · §6 Log Coverage — Additional scenario-specific fields
**Change:** All three sites retire the `degraded_mode_response` envelope extra. §3 states the per-scenario extension point as an extension point with no known instance (its only ever-named one retired); §4's known-residual path carries the eleven-field envelope ALONE, keeping the note that an envelope extra is report-seam-written while a span attribute must be allowlisted; §6's field entry is struck through and marked retired. The extension POINT itself stands — only the named field is withdrawn.
**Why:** The field was measured absent from `crates/` — no scenario emits it, so a §4 required field on a must-trace path was never satisfiable. Scope held to the measurement: the generalisation "no scenario extra is implemented" was NOT applied, because only this one field was measured; the text says only that nothing is KNOWN to exercise the extension point. The same field in `.claude/rules/observability.md`'s Format bullet was re-derived in the same pass.
**Ref:** .andromeda/runs/2026-09-06T09-37-04-wrap/

## 2026-09-06-run-report-envelope-conformance-gate — conductor-cli raises no spans of its own; the teardown WRITE is deliberately uninstrumented; the envelope list restored to eleven
**Section:** 4. Span / Trace Coverage (Auto-instrumentation per surface: cli row, conductor-report row) + 1. Obs Scope Summary (rusqlite row) + 6. Log Coverage (boundary-call wrappers, per-module levels, Required fields)
**Change:** SIX sites.
- (1) §4 cli row: `#[tracing::instrument]` narrowed to the core scenario handlers the CLI drives — NOT `fn main()`, because `conductor-cli` declares no `tracing` dependency and carries no `tracing::` call site.
- (2) §6 per-module levels, conductor-cli row: qualified to the same effect.
- (3) §4 conductor-report row: the manual-span rule narrowed to the run-persist WRITE; the teardown WRITE (`RunsDb::delete_run`) mints NO span and no boundary log, deliberately — off the must-trace paths, and §11's bounded span-name set stays `db.insert_run` alone.
- (4) §1 rusqlite row: the same split.
- (5) §6 boundary-call wrappers: a DB DELETE entry recording the deliberate non-logging.
- (6) §6 Required fields: the list restored from TEN to the ELEVEN §3 fields (`read_back_observed_at` was missing), the heading corrected from "every log line" to "every ENVELOPE record", and required restated as KEY PRESENCE rather than non-null.
**Why:** Sites 1-5 were escalated and operator-ratified at this wrap. Site 1: the `fn main()` mandate sat in a SECOND "Auto-instrumentation per surface" table, so the playbook rule dismissing it as a misattribution did not apply. Sites 3-5: the teardown span is never-minted by design (the operator closed §11's set against a `db.*` wildcard on 2026-09-01), not a span deferred to a later epoch. Site 6: the count was settled at eleven by `2026-06-16-emission-journal-writer` and a11y-plan already carries the eleven-key form, so this doc was the lone outlier.
**Ref:** .andromeda/runs/2026-09-06T13-07-09-wrap/

## 2026-09-06-coverage-completeness-gate — Critical Path 6 re-based onto the allowlisted `message` field
**Section:** §4 Span/Trace Coverage → Scenario: Coverage-matrix completeness gate · §1 Obs Scope Summary → Critical paths (must-trace) table, Coverage-matrix row
**Change:** Both sites re-based onto what shipped. Must-trace spans: NONE — the gate mints no span; its observable is ONE boundary `info!` from `conductor_report::coverage_rollup` (the function every surface reaches; deliberately not `CoverageMatrix::render`, which is pure/no-IO and unreached without `--write`). Required span attributes: NONE — the counts ride the already-allowlisted `message` field. Required log fields: none beyond the §3 base set. The retired chain (`report.coverage_matrix_generate` → `db.query_all_p_ids` → `report.validate_coverage`) and its five attribute/field names are recorded as never-built and unbuildable as written: none of the three names is in §11's bounded span-name set, the `conductor-report` row keeps that set at `db.insert_run` with no `db.*` widening (which `db.query_all_p_ids` contradicted), and the five names are absent from `ALLOWLISTED_FIELDS` so they would have been dropped at the processor stage. The §1 row's surface cell was corrected: no `runs.db` query — the classification is a code-native static, not an aggregation over runs. The Denominator-semantics paragraph is UNCHANGED; the gate adopted its in-scope denominator and `(N unbacked)` auto qualifier verbatim.
**Why:** A chunk that operationalizes a spec'd gate for the first time surfaces that the spec's own description names a stale field-list — routine, scoped to the operationalizing doc's own sections. Operator-ratified at the chunk's P4 as the `message`-field re-base.
**Ref:** .andromeda/runs/2026-09-06T15-59-32-wrap/

## 2026-09-06-halo-hue-budget-re-driven — P-025's hue bound is unmeasurable through its leaf, and the cause is quantization
**Section:** 4. Span / Trace Coverage — Known-residual classification path, Delegated-timing family
**Change:** The P-025 entry's recorded CAUSE for the over-budget `metric.constellation.hue_update_ms` reading was STALENESS; now TICK QUANTIZATION, and the <=2s bound is recorded UNMEASURABLE through that leaf: `ServiceRegistryEntry.last_seen_unix_nano` has no ingest-path writer and is stamped only by Pulse's 15s lifecycle tick, so the leaf reports `t_sample - t_last_refreshing_tick`, U(0,15s) and independent of dispatch rate. P-025 grades as a mechanism pin with no pass arm; the other three delegated leaves keep their harvest-tier budgets unchanged. The passage carries its measurement pointer and its SCOPE (this leaf, Pulse HEAD 83d4060; a SUT writer change lifts it, recorded as Pulse intake).
**Why:** The re-driven leg removed the only competing explanation: `halo-hue-encoding` emitted 360 dispatches at 2/s so the service never went quiet, yet its in-window sample still read 14525.9ms against 2000ms, matching its offset to the preceding tick; all 7 samples fit duration = offset + k*15000, k in {0,1,2}, within 2ms. Staleness alone would have predicted ~0 for an emitting service.
**Ref:** .andromeda/runs/2026-09-07T08-32-30-wrap/

## 2026-09-07-dependency-polish — §3 Transitive note: the default-features follow-up closed, its premise measured false
**Section:** §3 Observability Harness Contract → OTel SDK init
**Change:** The note recorded the default-features trim as the way to shed the dormant footprint; it now records that the trim shipped at the WORKSPACE opentelemetry-proto entry covering BOTH dep sites, and that it CANNOT drop `opentelemetry` / `opentelemetry_sdk` — at 0.32.0 the `trace` and `logs` features gate both the generated message modules conductor-emit imports and the SDK transform modules, and the crate declares `trace = [opentelemetry/trace, opentelemetry_sdk/trace]`. What the trim drops is `metrics`/`zpages`/`with-serde`/`internal-logs`, of which only `const-hex` left the tree. The behavioural invariant is untouched.
**Why:** The premise was measured false against the vendored `opentelemetry-proto-0.32.0` source, with both crates still in `Cargo.lock` after the trim.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-dependency-polish — Tier-justification crate count and stack version reconciled
**Section:** §1 Obs Scope Summary · §12 Obs Decisions Log
**Change:** "8 workspace crates" → 9 at both tier-justification sites; `tokio 1.48.x` → `1.52.3` at both instrumentation-scope table rows.
**Why:** Pre-existing stale literals; the workspace roster is measured unchanged at 9. Raised by the orchestrator because obs-plan's own detectors are scoped to instrumentation, stack and redaction, so neither literal falls inside one.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-a11y-ci-gate — the envelope extension point is EXERCISED; the a11y violation artifact registered
**Section:** §3 Harness Contract → Log format JSON schema (the extension-point paragraph) · §9 CI Integration (artifact table)
**Change:** Retired "The extension point itself stands; nothing is known to exercise it today" — it IS exercised as of 2026-09-07 by the a11y CI gate's violation record at `runs/a11y/<run_id>.jsonl`, which carries the eleven envelope keys PLUS the §9 resource tags `service.name` (`conductor-ui`) and `deployment.environment`: 13 top-level keys locally and 15 under CI, where `ci.run.id` + `git.commit.sha` join them. The paragraph states what the extras ARE — resource TAGS on a harness artifact, not scenario-specific fields, so the scenario record at `runs/<run_id>.jsonl` still carries the eleven alone — and why the superset is admitted: `journal_conformance` asserts key PRESENCE, closed sets and host-path freedom, never key exclusivity, which is what lets ONE gate serve both shapes. §9's artifact table gains the record's row (uploaded via `actions/upload-artifact@v4`, conformance asserted in-job under `CONDUCTOR_RUNS_DIR=runs/a11y`).
**Why:** A chunk operationalizing a spec'd CI gate for the first time surfaces that the same spec's description of it is stale; the gate passes over the real record. The §9 row was raised by the orchestrator because no detector proposed it.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — the fmt gate joins the build-time verification set and the Lint row
**Section:** §1 Obs Scope Summary (instrumentation-scope table, `cargo build / CI/CD pipeline` row) · §9 CI Integration (Pipeline integration, Lint / typecheck row)
**Change:** TWO edits.
- (1) §1's Not-instrumentable row listed build-time verification as "(`cargo build`, cargo-nextest, cargo clippy)"; `cargo fmt --all --check` joins it — the fmt gate is build-time and runs at no runtime, so its classification is unchanged.
- (2) §9's Lint / typecheck row named only `cargo clippy` warnings to stderr; it now also names the fmt gate's unified diff to stdout, records that it ships as the `rust` job's own early step at index 2 rather than colocated with clippy, and states that the diff's `Diff in {file}` lines are agent-readable from the job log but are NOT written into any telemetry artifact — they carry absolute host paths, which §9's own log-conformance check rejects.
**Why:** Edit (2) was raised by the orchestrator because no detector proposed it — the plan's expected-amendment list is the chunk's coverage floor and may not under-run silently. Edit (1) came from the cross-master sweep for the gate-set enumeration. The host-path clause is stated because the gate is classed `instrumentation n/a`, and §9's log-conformance check would fail on such a line if it were ever routed into `agent-latest.jsonl`.
**Ref:** .andromeda/runs/2026-09-09T13-20-08-wrap/

## 2026-09-10-live-pulse-in-lane-scenario-round — the latency delta is the whole emission window, not the MCP round-trip
**Section:** §4 (Delegated-timing family note)
**Change:** The parenthetical glossing `read_back_observed_at - journal_emitted_at` as "(Conductor's MCP round-trip)" is retired. It now reads as Conductor's journal-relative span covering the scenario's WHOLE emission window, not the MCP call, with its measurement pointer: three legs of 18s / 35s / 30s of declared phases recorded `latency_ms` 18169 / 35120 / 30212, while the sidecar's own tool calls logged `duration_ms` 0–22 in the same legs. The passage's CONCLUSION is unchanged and strengthened — the budgets still bound a Conductor-side wall-clock span "rather than a Pulse-internal duration".
**Why:** No detector covers latency semantics; the orchestrator raised it and it applied as routine — reconciling a spec's illustrative wording to the sound shipped implementation where the invariant still holds. Only this site glossed the delta as a round-trip; other statements of the formula were correct. Recorded, not established: this gloss is a plausible origin of `constellation-severity-live-wiring`'s unattainable `<20s` tier — a tier chosen believing the delta ≈ a round-trip would be far too tight for a 30s emission window — which is why the residual carries both defects under one owner.
**Ref:** .andromeda/runs/2026-09-10T15-55-02-wrap/

## 2026-09-13-p-025-measurement-contract-for-pulse — the two named fixes measured insufficient, and the SCOPE clause repointed
**Section:** §4 Span / Trace Coverage → Scenario: Known-residual classification path → Delegated-timing family (the SCOPE clause closing the passage)
**Change:** The SCOPE clause's prediction that "a SUT change that stamps `last_seen` from span arrival (or copies the activity floor's `last_observed_unix_nanos`) would lift it" is RETIRED as measured false. The body now states that neither candidate lifts the bound — each addresses the STALENESS term alone, so with `last_seen` ingest-stamped a 2/s emitter reports ~500 ms and the 2000 ms budget starts PASSING while the leaf still emits the age of the newest span at paint time (a green grade over the wrong quantity) — and that the other two fire-site terms survive either change: SLOWEST-WINS (the maximum across every dot whose tier changed, on a leaf carrying no service identifier) and the tick QUANTIZATION the passage already establishes. It names what WOULD lift the bound: Pulse emitting a different QUANTITY, per the new SUT-facing `contracts/pulse-p025-measurement-contract.md` — EXPOSE a start instant Pulse already holds where it computes the tier (`opened_at_unix_nano` on a rise, `transitioned_at_unix_nano` on a fall, the two cases exhaustive because an incident's `priority_tier` is immutable after opening), never mint one, and never a journal-relative term (`latency_ms` / `budget_ms` / `effective_deadline_ms`). The Pulse-intake ownership is unchanged; the clause carries its measurement pointer and its expiry (HEAD `83d4060`, 2026-09-13).
**Why:** The chunk measured the named change WOULD BE INSUFFICIENT — a different event from the change occurring, so no playbook rule matched (the nearest fails its DIRECTNESS condition). Applied under recorded direction: the plan's expected amendment, ratified at the phase P5 review and reaffirmed by the operator's wrap directive. A direction settles the proposal, never the class: the adjacent class (a master's stated MECHANISM falsified by measuring its named precondition insufficient) was put to the operator and DEFERRED at n=1 — no generator requires a master to state a predicted fix — and is to be re-raised on recurrence. The amended passage has no distillation in any leaf, so no leaf re-derivation is owed.
**Ref:** .andromeda/runs/2026-09-14T16-05-00-wrap/

## 2026-09-15-structurally-dead-assertion-class-retired — findings-counter-refresh restated as declare-only (both statements on the line)
**Section:** §4 Span / Trace Coverage → Known-residual classification path → Delegated-timing family — both occurrences applied.
**Change:**
- The family's declare-only roster widened three → four: `halo-hue-encoding` · `service-constellation-discovery` · `report-render-surface`, joined by `findings-counter-refresh` on 2026-09-15.
- The exclusion clause `findings-counter-refresh is not declare-only (one [[expected]]) … its unmet CountAtLeast floor grading CalibrationRegion` is retired. The body now states that it IS declare-only as of 2026-09-15, that its single `CountAtLeast` floor was retired as structurally dead (the floor graded `evidence_count`, summed from a `span_refs` vector whose only non-test writer in the SUT sets it empty), and that it reaches `KnownResidual` by the same degraded read-back with the declare-only envelope shape the section already specifies — `verdict` null, no `CalibrationRegion` grade from that floor. Its harvest-tier leaf `metric.findings.counter_refresh_ms` is unchanged.
**Why:** The chunk retired that scenario's only `[[expected]]` block, falsifying both halves of the §4 sentence. The claim sat TWICE in one paragraph — the roster and the exclusion clause — so a single-statement apply would have left the enumeration contradicting the fix. No other obs-plan site and no obs leaf carried the retired claim.
**Ref:** .andromeda/runs/2026-09-15T12-44-09-wrap/

## 2026-09-22-interpretation-proven-live — the real-model posture on Critical Path 1, the fingerprint-storm read-back re-based, parity scoped
**Section:** §1 Must-trace table, the Fingerprint-storm row · §4 Critical Path 1 (Headless deterministic scenario run) — NEW "Real-model posture" bullet · §4 Fingerprint-storm (`verify.readback_fingerprints` attribute · required log fields) · §4 Critical Path 7 required log fields
**Change:**
- Critical Path 1 records the real-model posture: the declare-only `real-model-interpretation` scenario adds NO critical path, span name or span attribute — it rides this chain with the eleven-field envelope, `verdict` null, a non-degraded read-back landing `ManualCheck`; its interpretation is graded at the harvest tier, never through the envelope; `execute_scenario`'s posture-mismatch `Blocked` (before the ready check) is one `info` line on the allowlisted `message` field — never a sixth gate precondition. The one drive was blocked at the preflight, so the chain was not observed live under that posture.
- The Fingerprint-storm row and §4 block: was "no read-back surface carries Pulse's computed fingerprint" (true at `efabe8e`); now at `83d4060` `fingerprint_refs` also carries each incident's triggering-cue fingerprint (the grounded union, measured 2026-09-10), so an emitted-vs-read-back match is computable, though no span attribute or shipped check computes it; `fingerprints_read_back_count` stays a COUNT.
- Critical Path 7's parity (identical `verdict`/`state` for the same seed) is stated for DETERMINISTIC-posture scenarios; a real-model scenario is outside parity by design (the Tauri path keeps the deterministic gate and records it scenario-level `Blocked`).
**Why:** The chunk added the mismatch `Blocked` and its message-borne line but no envelope key, verdict word, state, span name or attribute. The fingerprint and parity edits were escalated and operator-ratified ("amend now, as measured"; "narrow + record").
**Kept:** Parity's MECHANISM statements (envelope comparison, not trace correlation) are left standing — the scope lives in Critical Path 7 alone. The plan's "§5/§10 — the pickup figure" amendment took NO edit: no figure was measured and obs-plan states none to correct.
**Ref:** .andromeda/runs/2026-09-23T08-03-55-wrap/

## 2026-09-24-secret-scanning-ci-gate — the repository-hygiene gates in §1, §9 and §10
**Section:** §1 instrumentation-scope table (the `cargo build / CI/CD pipeline` Not-instrumentable row) · §9 Pipeline integration (new row) · §10 Build / deploy failure conditions
**Change:** The Not-instrumentable build-time row names the static gates, including the repository-hygiene secret-scan and workflow env-context gates. §9 gains a row for them: each hit is one line naming a repo-relative `path:line` with its rule, never the matched text, or else the workflow file, line and key. The probe prints its own verdict line. All of it is readable from the job log only and NEVER written into a telemetry artifact (the fmt-row discipline). §10 lists a repository-hygiene gate red, including the `GITHUB_ENV context probe (assert)` step.
**Why:** Raised by the orchestrator following the 2026-09-09 fmt-gate precedent, since no obs detector covers a gate-enumeration update. The gates are instrumentation n/a — build-time; no telemetry schema, artifact or span changes.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model posture chain observed live
**Section:** §4 Span / Trace Coverage → the headless deterministic scenario run → Real-model posture
**Change:** was "The one drive (2026-09-23) was blocked at the preflight, so this chain was not observed live under the real-model posture". Now the 2026-09-29 series observed it live: both Stage B preflights reached ready, the graded drive's envelope landed `ManualCheck` with `verdict` null and the eleven keys, and canary-blocked drives' envelopes landed `Blocked`. The three-storm canary adds no span or field; each storm logs the existing `canary fingerprint computed` line.
**Why:** the series superseded the claim that the chain was never observed. Instrumentation is unchanged: the diff adds no span name and no allowlisted field.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/

## 2026-09-29-hue-shift-budget-graded-hard — P-025 graded hard; the retired instrument kept as history
**Section:** §4 Span / Trace Coverage → Known-residual classification path → Delegated-timing family (the P-025 passage)
**Change:** was "P-025's ≤2s bound is UNMEASURABLE through `metric.constellation.hue_update_ms` … TICK QUANTIZATION … P-025 therefore grades as a MECHANISM PIN with no pass arm". It also said the bound "lifts only if Pulse emits a DIFFERENT QUANTITY … `transitioned_at_unix_nano` when it falls … Still Pulse intake", with every coordinate as measured at `83d4060`.
Now P-025 grades HARD at the harvest tier: PASS, worst 684.98 ms, from one graded leg against a checkout carrying `e98d838`. That leg's in-window samples were a rise of 684.98 ms (anchored to its incident's creation line at 29.98 ms) and a fall to `none` of 430.79 ms.
- `duration_ms` is paint minus `tier_effective_at_unix_nano`. The rise source is `opened_at_unix_nano`; the fall source is `resolved_at_unix_nano`, not `transitioned_at_unix_nano`. Acknowledgement does NOT lower the tier. Pulse emits one sample per changed service, and only for changes the canvas witnessed.
- The grade is taken by `grade_in_window` under the contract's §The grading rule: every sample in [phase-2 start, `scenario.run` close], worst observation, inclusive, absence UNGRADED, a breach a hard Fail relayed to Pulse. All four delegated-timing bounds now grade hard.
- SCOPE: deterministic L4, the widget visible, the rise poll-bounded.
- The tick-quantization reading at `83d4060`, and its test, stay as the retired instrument's record. So does the finding that neither named SUT candidate would have lifted the bound; the shipped fix is neither.
- Pulse coordinates are read at `226554a` unless dated `83d4060`.
**Why:** the passage named its own precondition (Pulse emitting the contracted quantity) and its own expiry, and the graded leg is that precondition, measured. The fall source and acknowledgement were measured wrong at `226554a`, which is the CARRY's two premises, now verified.
**Ref:** .andromeda/runs/2026-09-29T21-30-19-wrap/

## 2026-10-02-p-075-assert-round-against-pulse — a test-tier fingerprint-membership check; delegated timing re-graded at S
**Section:** §4 Fingerprint-storm → `verify.readback_fingerprints` · §4 Delegated-timing family
**Change:**
- Fingerprint-storm: was "no span attribute or shipped check computes it" (the emitted-vs-read-back match). Now no span attribute computes it, and one test-tier check does (2026-10-02). `lifecycle_harvest::p075_round_assertion_1_read_back_content_fidelity` grades the emitted fingerprint's membership in `fingerprint_refs` at Pulse S `03ec944`, over the P-075 round's digest-pinned capture. The match is computed in-process by the `live-pulse`-gated `p075_round_live` leg, never through a span or the envelope, so `fingerprints_read_back_count` stays a count.
- Delegated-timing family: re-graded 2026-10-02 at Pulse S `03ec944`, and all four PASS from each leaf's own field under the same rule:
  - P-025 worst 478.56ms in window, the rise anchored at 38.24ms;
  - P-027 worst 605.26ms;
  - P-037 0ms, one sample, fired with no desktop input;
  - P-045 worst 5.0ms of 163.

  Cited as measured at the chunk's `evidence/round-ledger.md` and held by `delegated_timing_harvest::tests::p075_round_assertion_{3..6}_*`.
**Why:** the P-075 assert round added the first check computing the match and re-graded the four bounds at a newer Pulse HEAD.
**Kept:** "no shipped check keys on the envelope array" stands, because the new check reads the leg capture, not the envelope. The 2026-09-29 P-025 PASS record and the `226554a` coordinate note stand beside the re-grade.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/
