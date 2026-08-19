# obs extract

## Relevance
Relevant — the chunk wires the driver for a span obs-plan §4 already specifies (`fault.port_occupier`) and drives four live legs whose evidence is obs's journal/self-obs stream.

## Constraints
- `fault.port_occupier` is spec'd with **exactly two** attributes — `fault_type`="port_occupier" and `port` (4317) — and no duration/offset, because the layer records span attributes on the `new` record alone; the realized hold must be witnessed on the allowlisted `message` field at `debug` when the RAII guard releases (per obs-plan §4 Fault-injection spans). Whether the shipped span already conforms is research's question.
- §4 records the occupier's `timeline.execute` parentage as **CONDITIONAL** and names this chunk's entry (`verification-matrix.json#v2-15`) as the driver owner; the plan requires the span open at the RAII bind site inside `conductor-faults` while the run-path faults open from a caller-supplied per-phase hook, created-not-entered so `emit.batch` keeps `timeline.execute` as parent (per obs-plan §4 "Where each span opens").
- Any span attribute must be a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it — so a new occupier-outcome attribute (bind failed / port held) either earns allowlist membership or rides `message` (per obs-plan §4 Required-span-attributes constraint).
- The span name set is bounded; the driver may not introduce a name outside `{scenario.run, timeline.execute*, emit.batch, emit.logs_batch, verify.readback*, report.generate, db.insert_run, fault.silence/ramp/port_occupier, tauri.command.*}` (per obs-plan §11 Spans / Traces).
- Fault-span **lifecycle** must log at `info` (else invisible under the default INFO filter and the leg leaves no trace); fault **application** detail stays `debug`, and a per-target opt-in must be additive `RUST_LOG=info,conductor_faults=info` (per obs-plan §6 Log levels + §3 Per-module log levels).
- Each leg's scenario-result record must carry the eleven-field Run-report envelope with `latency_ms = read_back_observed_at − journal_emitted_at` from wall-clock `std::time`, never tokio's virtual clock, and `slo_tier` from the closed enum `<5s|<20s|<90s` (per obs-plan §3 Log format JSON schema + §11 Project-specific bans).
- The verdict/error wall is an obs invariant here: a failed bind or transport fault must be captured as a structured `tracing::error!`/typed `blocked`, never an uncaptured panic or unsanitized backtrace (per obs-plan §10 Zero unlogged panics + §11 Error Reporting).

## Patterns to follow
- RAII span lifecycle at `occupy()`/`release()`/`Drop` for the occupier, as §4 specifies and as `crates/conductor-faults/tests/port_occupier.rs` is cited by the scope to prove.
- Created-not-entered per-phase fault-span placement (`run_timeline_observed` / `PhaseWindow`, with `conductor-run` supplying fault semantics and the `std::time` journal basis) — the shape the driver wiring should mirror rather than entering the fault span (per obs-plan §4 Fault-injection spans).
- The span-lifecycle self-obs line variant (`span`, `span_event` new|close, optional `parent`, allowlisted attributes on `new`) is how a §4 span materializes as gradable lines — the evidence carrier for any harvest-tier grading of this leg (per obs-plan §3 two record shapes).
- The `message`-field witness pattern used for `verify.readback` key sets and the `emit.batch` wire-shape witness — precedent for recording facts that cannot legally be a dedicated attribute (per obs-plan §6 Boundary-call wrappers).
- Boundary must-log set for the live read-back legs: tool name + `latency_ms` + error + observed result key set (per obs-plan §6 Boundary-call wrappers).

## Anti-patterns to avoid
- No self-observation OTLP or W3C trace context anywhere near `:4317` — that port is the PRODUCT fault stream only; correlation is the `run_id` field (per obs-plan §11 Universal + Spans / Traces).
- No absolute host paths in logs / journal / runs.db — directly live here because the operator convention puts leg dirs under `%TEMP%/pulse-legs/`, a drive-letter path the redaction layer is required to mask (per obs-plan §11 Logs + PII Scrubbing).
- No dangling spans — the occupier span must close at its RAII boundary, and no `info` logging inside a per-record inner loop (per obs-plan §11 Spans / Traces + Logs).

## Contract bindings
- **obs ↔ tests harness:** the JSONL line format and the eleven-field envelope are OWNED by test-plan §3; obs derives (per obs-plan §3 Log format JSON schema). A harvest-tier test pinning verbatim lines for this family consumes obs's span-lifecycle line variant.
- **obs ↔ security:** redaction at the processor stage, no absolute host paths / internal struct names in run artifacts (per obs-plan §11 PII Scrubbing; §1 creator-explicit-telemetry trigger).
- **obs ↔ CI:** the log-conformance gate (§3 self-obs base schema) and the zero-unlogged-panics gate read `logs/agent-latest.jsonl` (per obs-plan §9).

## Acceptance criteria contributions
- The driven leg emits `fault.port_occupier` span-lifecycle lines (`span_event` `new`+`close`) at `info`, carrying `fault_type` + `port` and no other attributes, with the realized hold witnessed on `message` at `debug` on release (per obs-plan §4 Fault-injection spans + §6 Log levels).
- The occupier span is observed beneath `timeline.execute` on the driven leg, or §4's CONDITIONAL parentage wording is re-based on what the wiring actually produces — whether the new driver yields that parent line is research's question (per obs-plan §4 "Where each span opens").
- Each P-001..P-004 leg's scenario-result record carries the eleven-field envelope with wall-clock `latency_ms` and a closed-enum `slo_tier`, SLO asserted at report-generation time (per obs-plan §3 Log format JSON schema + §10 Performance budgets).
- Zero unlogged panics across the legs including the expected bind-refused condition (no `^thread.*panicked` in `agent-latest.jsonl`/stderr) and no absolute host path — `%TEMP%` leg dirs included — in any log, journal, or runs.db field (per obs-plan §9 CI gates + §11 Logs).

## Relevant amendment history
- **2026-08-16-fault-application-spans** — the direct precursor: narrowed `fault.port_occupier` to `fault_type`+`port` (duration/offset unknowable at bind time), moved the realized-hold witness to `message` at `debug`, moved fault-span lifecycle from `debug` to `info`, and recorded the `timeline.execute` parentage as CONDITIONAL because the code-graph measured `conductor-faults` with zero inbound crate edges. This chunk's CARRY is the missing driver that amendment identified.
- **2026-08-18-restart-suppression-live-proof** — retired a never-built family span chain and an envelope extra, re-basing the row onto the shipped declare-only instrumentation with harvest-tier grading over Pulse's own lines. Precedent the scope's "[inferred] harvest route" follows; expect the same re-basing discipline if P-001..P-004 read-back proves ungradeable.
- **2026-08-16-fingerprint-storm-live-proof** and **2026-08-18-error-baseline-spike-live-proof** — the two prior live-proof amendments: a must-trace field measured unsatisfiable gets corrected against measurement, and a superseded supporting fact does not automatically overturn the conclusion it supported.
- **2026-08-13-per-check-read-back-extraction** — established that a span attribute outside `ALLOWLISTED_FIELDS` emits nothing, retiring two attributes on that ground; governs any new occupier-outcome attribute this chunk contemplates.
- **2026-08-10-scenario-run-root-span-tree** — recorded the span-lifecycle self-obs line variant (so §4 spans materialize as real lines) and corrected a Cleanup claim of parentage the code could not produce; both bear on how this chunk evidences and words the occupier's placement.
- **2026-08-14-canary-fingerprint-feed-capture** — measured that a bare per-target `RUST_LOG` directive REPLACES the default; the additive `info,{crate}=debug` form applies to any `conductor_faults` opt-in on these legs.
