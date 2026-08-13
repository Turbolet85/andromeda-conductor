# obs extract

## Relevance
Partial — the chunk is a test/golden surface plus a `check_load_envelope` re-scope; obs contributes determinism-of-instrumentation rules, the golden-hygiene bans (no wall-clock stamp, no host paths), hot-path logging discipline, and the CI/audit evidence channel. No new must-trace path.

## Constraints
- No OTel SDK / exporter / `tracing-opentelemetry` may be introduced to observe the dispatcher — the ban is *behavioral* (an SDK's background batch tasks are exactly what would break `current_thread` emission ordering, i.e. the property this chunk is freezing); a dormant transitive `opentelemetry_sdk` is not a violation (per obs-plan §3 OTel SDK init, §11 Telemetry Strategy).
- `conductor-timeline` instrumentation stamps wall-clock (`std::time`), never tokio's virtual clock; conversely a wall-clock stamp is non-reproducible, so a committed golden must project only seed-derived / virtual-offset fields (per obs-plan §1 instrumentation-scope `conductor-timeline` row, §11 Project-specific bans).
- Any span added, renamed or asserted on the dispatch path must come from the bounded low-cardinality set under `{module}.{operation}` — `timeline.execute*`, `emit.batch` are the members that apply here; no seed-, scenario- or emission-index-keyed span names (per obs-plan §4 Span naming convention, §11 Spans/Traces).
- The per-phase dispatch hook is an inner loop: never `info!` per emission; per-emission detail is `debug`/`trace` gated by `RUST_LOG=conductor_timeline=debug`, and hot paths are profiled before gaining spans (per obs-plan §6 Per-module log levels, §11 Telemetry Strategy + Logs).
- The re-scoped `check_load_envelope` keeps its logging inside the field-name allowlist and leaks no absolute host path (the module already withholds the path from its `io::Error` text and routes through `sanitize_error`); redaction is single-owner at the processor stage, not re-implemented locally (per obs-plan §11 Logs + PII Scrubbing, §9 Log conformance check).
- The emitting-phase-duration / sustained-rate terms stay plain computed field assertions — no meter, histogram or metrics backend may be added to express them (per obs-plan §5 Metric Coverage, §11 Metrics).
- The `cargo audit` / `cargo deny` result belongs in the CI-log + artifact stream as machine-readable evidence, and `cargo-audit` red is already a listed build-failure condition — this chunk records a result, it does not add, weaken or gate a new obs check (per obs-plan §9 Telemetry artifact handling, §10 Build / deploy failure conditions).

## Patterns to follow
- The shipped `pacing.rs` `stamped()` helper measures offsets from `tokio::time::Instant` under `start_paused` — virtual, seed-reproducible; the emission golden should capture in that idiom rather than any `SystemTime` reading (per obs-plan §1 `conductor-timeline` row; §11 Project-specific bans).
- Shape-projection goldens: where a value also carries wall-clock fields, snapshot a projection over identity + ordering + linkage and exclude the clock fields — the seed governs identity, not the clock (per obs-plan §5 measurement definition; §11 Spans/Traces "no dangling/high-cardinality" discipline).
- Self-obs spans materialize as JSON *lines* in two variants over one base set (event line; span-lifecycle line with `span` / `span_event` / optional `parent`) — if any golden or assertion touches log output on this path, that is the shape it sees, not exported traces (per obs-plan §3 Log format JSON schema, two record shapes).
- `LoadEnvelope::load`'s existing `tracing::info!(count = …, "…")` is the house style for the re-scoped check: allowlisted scalar field names, message text carrying no path or struct name (per obs-plan §6 Boundary-call wrappers, §11 Logs).
- Envelope breach is a returned VALUE on the `Ok` path (`EnvironmentSuspect`), surfaced as a reported run-level state — never a panic or a harness fault (per obs-plan §11 Error Reporting; §10 which does not list envelope breach as a build-failure condition).

## Anti-patterns to avoid
- Never add an OTel SDK, `tracing-opentelemetry`, `traceparent` or any self-obs OTLP export (`:4317` is the PRODUCT stream, `:4318` is dead) to instrument or verify the dispatcher (per obs-plan §11 Telemetry Strategy + Spans/Traces + Universal).
- Never log at `info` per dispatched emission, and never introduce a high-cardinality span name keyed on seed / emission index / scenario input to make the stream observable (per obs-plan §11 Logs + Spans/Traces).
- Never stamp the journal from tokio's virtual clock — and by the same invariant, never bake a `std::time` stamp into a committed golden (per obs-plan §11 Project-specific bans).

## Contract bindings
- **obs ↔ tests harness:** test-plan §3 OWNS the JSONL log line + Run-report envelope format; obs-plan §3/§6 only reproduce it. Any golden or assertion in this chunk that captures self-obs lines is governed by the owner, and a one-sided change here is precisely what the obs↔tests-harness bind guards (per obs-plan §3 Log format JSON schema; §6 Log Coverage).
- **obs ↔ CI (tests harness):** `cargo-nextest` JSON + `logs/agent-latest.jsonl` are uploaded CI artifacts; supply-chain audit output rides the CI-log channel — this is where the PREREQ's recorded `cargo audit` / `cargo deny` result lands (per obs-plan §9 CI Integration).
- **obs ↔ core/report:** the load-envelope verdict surfaces through the run-level `[ENVIRONMENT-SUSPECT]` envelope caption / report state, not through an exit code or a panic (per obs-plan §11 Error Reporting; §10).

## Acceptance criteria contributions
- (obs) Committed goldens contain no `std::time`/wall-clock stamp and no absolute host path (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`) — reproducible byte-identically on any host (per obs-plan §11 Project-specific bans + §11 Logs / §9 Log conformance check).
- (obs) Any span touched on the dispatch path stays in the bounded `{module}.{operation}` set (`timeline.execute*`, `emit.batch`); no seed/emission-index-keyed name is introduced (per obs-plan §11 Spans/Traces + §4 Span naming convention).
- (obs) No new `info!`-level or span emission inside the per-emission dispatch hook; per-emission detail remains `debug`/`trace` under `conductor_timeline` (per obs-plan §6 Per-module log levels + §11 Telemetry Strategy).
- (obs) The re-scoped `check_load_envelope` reports through structured, allowlist-conformant fields with no host path or internal struct name, and an envelope breach remains an `Ok` value rather than a panic (per obs-plan §11 Logs + Error Reporting).

## Relevant amendment history
- **2026-08-11-faithful-emission-dispatcher** (§11 Project-specific bans) — the never-skip-garde ban was restated to name the shipped `error_percent` ∈ 0..=100 encoding and to require a nested spec field `dive`, never `skip` (a skipped struct is never descended into). This is the immediately preceding chunk in the same area; any fixture authored or edited to drive the new emission golden must satisfy that ban, and the `EmissionSpec` nesting is exactly the case it names.
- **2026-08-10-scenario-run-root-span-tree** (§4 Cleanup; §3 two record shapes) — the root `scenario.run` was re-anchored to `conductor-run::execute_scenario` with `report.generate` / `db.insert_run` recorded as run-scoped SIBLINGS correlated by `run_id`, and the self-obs span-lifecycle line variant (`span` / `span_event` / `parent`) was recorded. Relevant because `timeline.execute` is the child of that root and this is the shape any span-aware assertion on the timeline path would observe.
- **2026-06-17-raw-otlp-message-scaffold** (§3 OTel SDK init) — the no-SDK invariant was clarified as behavioral, with the dormant transitive `opentelemetry`/`opentelemetry_sdk` (via `opentelemetry-proto`) accepted as non-violating and a `default-features = false` trim tracked as follow-up. Relevant because the ban's stated harm is precisely the `current_thread` determinism this chunk freezes — do not re-litigate the dormant dep as a finding here.
