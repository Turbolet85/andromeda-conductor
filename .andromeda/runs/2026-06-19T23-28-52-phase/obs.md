# obs extract

## Relevance
relevant — chunk produces a permanent-stop fault helper (P-014) whose instrumentation is deferred to timeline integration; obs-plan governs the fault span pattern once driven

## Constraints
- per obs-plan §4: Fault-injection spans bounded to `{fault.silence, fault.ramp, fault.port_occupier}` set only; **no high-cardinality span names**
- per obs-plan §4 fault.silence span: attributes required = `fault_type` ("silence"), `fault_duration_ms` (integer), `fault_start_offset_ms` (journal offset)
- per obs-plan §1 instrumentation scope: conductor-faults is instrumentable; telemetry surfaces as **spans around each fault application phase** (not during helper construction — construction is scope-out per chunk boundary)
- per obs-plan §6 error capture: construction failure ⇒ typed `FaultError` value, never panic — aligns to "verdict/error wall" acceptance criterion
- per obs-plan §3 logging init: tracing JSON stack initialized at CLI/Tauri bootstrap; fault helper inherits structured logs via parent `timeline.execute` span context

## Patterns to follow
- per obs-plan §4 fault.silence span: span wraps fault application phase in conductor-faults; children of the `timeline.execute` span for scenario; span closes on fault release (duration expiry)
- per obs-plan §1 determinism contract: abrupt-silence helper must be **seed-independent / deterministic by construction** (same scenario ⇒ same stop); no seeded randomness
- per obs-plan §2 span naming: `{module}.{operation}` pattern — fault.silence spans follow `fault.silence` (fixed, low-cardinality)

## Anti-patterns to avoid
- per obs-plan §11 anti-patterns: **no unbounded span cardinality** — fault-name enum is closed (silence/ramp/port_occupier only); no per-user-ID or per-input span variants
- per obs-plan §3 no-OTel-SDK invariant: helper construction must not initialize or spawn OTel SDK tasks (behavioral guard on `current_thread` determinism)
- per obs-plan §6 error boundary: panic in construction ⇒ CI gate failure; only `FaultError` typed returns acceptable

## Contract bindings
obs ↔ tests: chunk's `FaultError` (if fallible) binds to tests' "verdict/error wall" assertion (tests excerpt §Acceptance accepts typed errors, no panics); obs ↔ timeline: fault.silence span is driven by timeline integration (Epochs 7/8, not this chunk); deferred to downstream consumer

## Acceptance criteria contributions
- **(obs) Determinism by construction:** abrupt-silence helper is seed-independent / deterministic; same scenario ⇒ same permanent stop (no seeded randomness; differs from `EmissionGap` which owns a finite `Duration`)
- **(obs) Fault-injection instrumentation ready:** helper type & attributes (fault_type, fault_duration_ms, fault_start_offset_ms) align to obs-plan §4 fault.silence span contract; ready for downstream timeline span wrapping (Epochs 7/8)
- **(obs) Verdict/error wall:** construction failure ⇒ typed `FaultError` value; infallible constructor (if chosen) documented as contrast to `EmissionGap` — zero panics on construction

## NOTE (orchestrator, carried to research/plan)
obs lists `fault_duration_ms` as a required `fault.silence` span attribute, but abrupt-silence is PERMANENT (no finite duration). The span attribute set assumes a bounded silence (fits EmissionGap/ramp). When the fault.silence span is wired under the timeline (Epoch 7/8), the permanent case will need a null/sentinel duration or a distinct attribute. Deferred (instrumentation is out of this chunk's scope) — flagged so it is not lost.

## Relevant amendment history
- 2026-06-18-severity-logs: bounded span-name set clarified; `fault.silence` confirmed as fixed member of `{fault.silence, fault.ramp, fault.port_occupier}` (no high-cardinality variants)