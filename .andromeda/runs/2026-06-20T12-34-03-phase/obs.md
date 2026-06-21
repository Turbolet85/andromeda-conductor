# obs extract

## Relevance
Partial — `conductor-faults` is instrumentable (obs-plan §1), but actual span instrumentation is deferred to Epoch 7/8.

## Constraints
1. **Descriptor-only (no spans yet):** The `BurstyTrain` type is a deterministic fault-helper primitive with no span instrumentation in this chunk (per obs-plan §4 bounded span-name set + chunk scope "Boundaries — explicitly NOT in this chunk"); spans will be added when timeline wiring occurs (obs-plan §4 Fault-injection spans, subsection acknowledges `fault.*` spans are Epoch 7/8-sequenced).
2. **Zero panics:** Any validation of duty-cycle bounds must return `Result<Self, FaultError>`, never panic (per obs-plan §10 zero-unlogged-panics invariant + §11 Error Reporting anti-pattern on malformed input).
3. **Determinism:** The `BurstyTrain` descriptor owns exact, un-jittered duty-cycle values (active/quiet durations); no seed in the helper; same inputs ⇒ same descriptor (parity with `EmissionGap` per chunk scope).
4. **No new observability deps:** The primitive adds no tracing crate, no OTel SDK, no logger init; instrumentation is added by the timeline/scheduler caller (obs-plan §2 Telemetry Strategy § Signal pyramid: chaos-instrumentation trigger handled at module boundary).
5. **Module naming conformance:** When span instrumentation arrives (Epoch 7), spans MUST use the `fault.{...}` naming convention (obs-plan §4 Fault-injection spans: `fault.silence`, `fault.ramp`, `fault.port_occupier` are the current bounded set; a bursty-train span will follow that pattern, if added).
6. **Field allowlist preservation:** No absolute host paths, no internal struct names in any future run-report fields; the redaction layer (`conductor-core::redact`) will apply allowlist + value scrub at report-generation time (obs-plan §11 PII Scrubbing).

## Patterns to follow
1. **Descriptor pattern:** Match the `EmissionGap` / `AbruptSilence` constructor shape (validated `Result<Self, FaultError>` vs. fixed canonical marker; chunk scope notes this is open, resolved at plan time).
2. **Duty-cycle accessor semantics:** Expose `*_ms` accessors and a within-cycle phase query (e.g., `is_active_at(offset) -> bool`) matching the timeline/`PhaseSpec` boundary consumption (obs-plan §1 Instrumentability table: the `*_ms` accessors the timeline/`PhaseSpec` boundary will consume).
3. **Doctest + unit coverage:** All public methods tested; doctests validate bounds and invariants (gates: nextest, clippy, doctest, llvm-cov per chunk scope Definition of Done).

## Anti-patterns to avoid
1. **NEVER panic on bounds violation:** Invalid duty cycles (zero or exceeding sanity ceiling) are `FaultError`, not panic (obs-plan §11 Error Reporting: malformed input ⇒ `blocked`, never panic).
2. **NEVER hardcode span instrumentation:** Spans are Epoch 7/8; this chunk is descriptor only (chunk scope explicitly defers `fault.*` spans).
3. **NEVER introduce OTel SDK, tracing init, or logger config:** The primitive itself is unobserved; the timeline/scheduler (Epoch 7) will wrap its use with spans (obs-plan §11 anti-pattern: "NEVER introduce an OTel SDK + exporter").

## Contract bindings
**None** — the `BurstyTrain` descriptor has no direct observability harness binding in this chunk. Timeline/scheduler wiring (Epoch 7) and span instrumentation (Epoch 7/8) will establish bindings downstream: `BurstyTrain` will be accessed as a `PhaseSpec` parameter, and fault spans will decorate the timeline's application of the pattern.

## Acceptance criteria contributions
1. **(obs-deferred)** When timeline wiring occurs (Epoch 7), a `fault.*` bursty-pattern span (per obs-plan §4 bounded set) wraps pattern application — NOT this chunk's responsibility, recorded as a forward binding.
2. **(obs-now) Zero-unlogged-panics:** `BurstyTrain` constructor returns `Result<Self, FaultError>` on invalid duty cycles; zero panics in bounds validation (per obs-plan §10).
3. **(obs-later) Run-report envelope fields:** any future bursty-train scenario field added to the JSONL envelope must pass the redaction layer + field-allowlist (per obs-plan §11 PII Scrubbing: no absolute paths, internal struct names dropped).

## Relevant amendment history
1. **2026-06-18-severity-logs** — `emit.logs_batch` added to the bounded span-name set alongside `emit.batch`. Relevant as precedent for bounded-set augmentation: when bursty-train fault instrumentation ships (Epoch 7), a new `fault.*` span name follows the same conforming-member pattern (spec → sound impl → bounded-set add, no high-cardinality names).