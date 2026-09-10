# obs extract

## Relevance
Relevant — the chunk's load-bearing assertion is an obs contract (`fault.port_occupier` span + field-allowlist survival), and its fix touches the self-obs subscriber install.

## Constraints
- `fault.port_occupier` must carry exactly two attributes — `fault_type` = "port_occupier" and `port` — recorded on the `new` record alone, with the realized hold witnessed separately on the allowlisted `message` field at `debug` when the RAII guard releases (per obs-plan §4 Fault-injection spans). Any restructuring of the test must not induce a change to this attribute set.
- A span attribute must be a name in `conductor-core::redact::ALLOWLISTED_FIELDS` or the processor stage drops it — which is exactly the invariant the chunk's assertion exists to prove for `port` (per obs-plan §4 Required span attributes; §11 PII Scrubbing). Whether `port` is currently in that allowlist, and whether the current probe can actually fail on its removal, is research's question.
- The self-obs span-lifecycle line is required to carry `span` (bounded §4 name), `span_event` (`new` | `close`), an optional `parent`, and the span's allowlisted attributes on the `new` line (per obs-plan §3 Log format JSON schema — two record shapes; format owned by test-plan §3). A replacement probe must read this shape, not a synthesized one.
- `fault.port_occupier` is a member of the bounded span-name set (`scenario.run`, `timeline.execute*`, `emit.batch`, `emit.logs_batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `fault.silence/ramp/port_occupier`, `tauri.command.*`); the set may not be widened or a member renamed (per obs-plan §11 Spans / Traces).
- Fault-span lifecycle must emit at `info`, not `debug`, precisely so it is visible under the subscriber's default INFO filter without a `RUST_LOG` opt-in; only the realized-hold witness stays `debug` (per obs-plan §6 Log levels + "Why fault-span lifecycle is `info`").
- A per-target `RUST_LOG` directive REPLACES the default rather than adding to it, and obs-plan §6 (Per-module log levels) requires that neither form be set for a run that also executes the test suite — the environment reaches the runner's child processes. A fix that reaches for env-based filtering to de-race the test collides with this rule.
- Self-obs is `tracing` + `tracing-subscriber` only, one `JsonObsLayer` over every writer, with `ObsSink` limited to `{Stderr, File}` and logging init required to complete before any scenario logic (per obs-plan §3 Logging stack, OTel SDK init). Any new capture path must stay inside that stack — no OTel SDK, no second formatter.

## Patterns to follow
- The `message`-field escape hatch: when a needed value has no allowlisted field name, it rides the already-allowlisted `message` value rather than gaining a new attribute (per obs-plan §4 Coverage-matrix gate, §6 Boundary-call wrappers, §4 port-occupier realized-hold witness).
- Processor-stage redaction: the field allowlist and value scrub are applied in the `tracing-subscriber` layer, never at the sink, with single-location ownership in `conductor-core::redact` (per obs-plan §11 PII Scrubbing). A probe that reads lines downstream of that layer is what makes the allowlist assertion evidence at all.
- Attribute-recording placement: the layer records span attributes on the `new` record alone — spec'd constraints are chosen to be knowable at span open (per obs-plan §4 Fault-injection spans; the same rule retired `fault_duration_ms`/`fault_start_offset_ms` for the occupier).
- Parentage is asserted from the emitted line's `parent` field, not from macro-site structure — `fault.port_occupier` is spec'd as a child of `timeline.execute` (per obs-plan §4 "Where each span opens").

## Anti-patterns to avoid
- Never widen or rename the bounded span-name set to accommodate a test (per obs-plan §11 Spans / Traces).
- Never skip the field-allowlist / redaction layer, and never scrub only at the sink stage — a probe that bypasses the processor stage stops proving allowlist survival (per obs-plan §11 PII Scrubbing).
- Never add retry-once policies (per obs-plan §11 SLO / §10 zero-unlogged-panics), which is the obs-side twin of the chunk's test-plan §11 retry ban.

## Contract bindings
- obs ↔ tests: the self-obs JSONL line format (both the event and span-lifecycle variants) is owned by test-plan §3, with obs-plan §3 reproducing it — a change to the observed line shape is a two-sided change, never one-sided (per obs-plan §3 Log format JSON schema).
- obs ↔ conductor-core `init_observability` / `ObsSink`: obs-plan §3 (Logging stack, Sink configuration) is the authority for the sink set and the single-layer install; obs-plan mandates no test-isolation or per-test subscriber mechanism, so whether the crate already admits a non-global install is research's question, not a plan fact.
- obs ↔ security: processor-stage redaction and the host-path scrub bind to security-plan §Logging & Monitoring (per obs-plan §11 PII Scrubbing).
- obs ↔ the out-of-scope panic-hook carry: obs-plan §10 requires `std::panic::set_hook()` capture as the zero-unlogged-panics invariant — the process-global the chunk's Out-of-scope section defers; if P3 finds the two process-globals are one mechanism, that lands against this same §10 mandate.

## Acceptance criteria contributions
- The retained probe reads real emitted span-lifecycle lines for `fault.port_occupier` (`span`, `span_event` `new`/`close`, `parent`, attributes on `new`) and still fails if `port` stops surviving the field allowlist (per obs-plan §4 Fault-injection spans + §4 Required-span-attributes allowlist constraint).
- The span's attribute set remains exactly `fault_type` + `port`, and its name remains an unchanged member of the bounded span-name set (per obs-plan §11 Spans / Traces + §4 Fault-injection spans).
- Fault-span lifecycle remains at `info` so it is visible under the default INFO filter with no `RUST_LOG` opt-in; the realized-hold witness remains `debug` on the allowlisted `message` field (per obs-plan §6 Log levels).
- No part of the delta sets `RUST_LOG` (in either bare or additive form) for a run that also executes the test suite (per obs-plan §6 Per-module log levels).

## Relevant amendment history
- **2026-08-16-fault-application-spans** — narrowed `fault.port_occupier` to `fault_type` + `port` (duration and journal offset are unknowable at bind time and the layer reads attributes on `new` alone), moved fault-span lifecycle from `debug` to `info` because at `debug` the spans are invisible under the default INFO filter and the chunk's own acceptance — the spans appearing on emitted lines — could not hold, and recorded the occupier as the one fault span opening inside `conductor-faults` at its RAII bind site. This is the amendment that created the assertion the current chunk must preserve.
- **2026-08-19-connection-lifecycle-live-proof** — the occupier's `timeline.execute` parentage went CONDITIONAL → measured real, witnessed live as `span_event` `new`+`close` with `parent: "timeline.execute"` and attributes exactly `fault_type` + `port`, driven by `conductor-run`'s `phase_guard` with RAII release at the scheduler's boundary drop.
- **2026-08-10-scenario-run-root-span-tree (self-obs span-lifecycle variant)** — recorded that the custom layer emits the span-lifecycle line variant (`span` / `span_event` / optional `parent` + allowlisted attributes on `new`), so a §4 span materializes as real lines; before it, the layer implemented only `on_event` and no span emitted anything. This is the line shape the chunk's probe depends on, and the amendment notes the lateral test-plan §3 ↔ obs-plan §3 bind.
- **2026-08-13-per-check-read-back-extraction** — established the §4 Required-span-attributes constraint that an attribute name absent from `redact::ALLOWLISTED_FIELDS` is dropped at the processor stage and "would have emitted nothing"; this is the mechanism the chunk's assertion exists to prove for `port`.
- **2026-08-14-canary-fingerprint-feed-capture** — measured that a bare per-target `RUST_LOG` directive replaces rather than adds to the default and fails the CLI's own agent-mode self-obs test; the additive `info,{crate}=debug` form plus the "never set it for a run that also executes the test suite" rule date from here.
- **2026-08-22-operator-pause-and-checklist-live-firing** — corrected 21 sites to record that `build_subscriber` installs exactly one layer (`JsonObsLayer`) over every writer and that `ObsSink` has only `{Stderr, File}`, with agent mode a WRITER switch and never a format switch — the sink surface any de-racing fix must work within.
