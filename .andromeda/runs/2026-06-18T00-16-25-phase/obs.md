# obs extract

## Relevance
Partial — exception events are instrumented spans; fingerprint computation is synthetic deterministic logic, not a telemetry signal itself; however, the `fingerprints` field in the run-report envelope is obs-owned and must be journaled per §6 Log Coverage + §3 JSON schema.

## Constraints
- **No OTel SDK for self-observation** (obs-plan §3 OTel SDK init; behavioral invariant clarified 2026-06-17): opentelemetry-proto is the PRODUCT (fault stream to Pulse `:4317`); Conductor self-obs is structured tracing JSON only — exception events attach OTel span-event syntax to the PRODUCT stream, but Conductor's instrumentation of its own exception handling is via `#[tracing::instrument]` + JSON log events, not OTel span exports.
- **Span naming convention** (obs-plan §4 + §2): exception-event spans follow `{module}.{operation}`; if exception detection spans emit, name as `emit.exception` (child of `emit.batch` per critical-path nesting in §4).
- **Fingerprint field in run-report envelope** (obs-plan §3 Log format JSON schema + §6 Log Coverage): `fingerprints` is an array field on the run-report JSONL line (per tests binding §3); the expected fingerprints, computed deterministically seeded (`ChaCha8Rng::seed_from_u64`), populate this field at journal-write time.
- **No absolute host paths or internal struct names** (obs-plan §11 PII Scrubbing + 2026-06-15 redaction amendment): exception `exception.stacktrace` content MUST be redacted at the tracing-subscriber layer — mask absolute file paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace paths) → `<redacted>`; preserve allowlisted `target` module path (e.g. `conductor_emit::...`).
- **Determinism under seed** (obs-plan §1 + obs-scope §1): fingerprint computation must be reproducible — same seed ⇒ same fingerprint, preserving journal-relative SLO math (wall-clock from `std::time` only per §3, no tokio virtual clock for fingerprints).
- **Agent-parseable fingerprint representation** (obs-plan §2 + §3): fingerprints array in JSONL must be parseable via `jq`/`serde_json`; bounded set per scenario P-IDs (no per-exception-ID cardinality); no transient fingerprints omitted from the journal.

## Patterns to follow
- Critical-path tracing (§4): exception spans are child spans of `emit.batch`; if exception happens in fault-injection, nesting is `timeline.execute` → `fault.*` → exception event on the fault span or its parent (clarify in impl).
- Fingerprint as a deterministic seal: compute via the same seeded RNG pattern as `conductor-timeline` (no wall-clock); store the expected fingerprint at emission time; later read-back verification (verify phase) compares Pulse's derived fingerprint to the expected value — the journal record is the ground truth.
- Exception-event attributes align to OTel semantic convention: `exception.type` / `exception.message` / `exception.stacktrace` per OTel spec; Conductor's deterministic control of stacktrace content drives the identical/path/line variants; redact stacktrace file paths at the processor stage to prevent host-path leaks.

## Anti-patterns to avoid
- Do NOT introduce OTel SDK + exporter for self-observation (obs-plan §11 + §3): exception events must not trigger SDK init or background batch exporters — breaks `current_thread` determinism + pollutes the PRODUCT stream. Stay within the `tracing` span + JSON log event model.
- Do NOT include high-cardinality exception identifiers in span names/attributes (obs-plan §11): no per-exception-ID span names; no unbounded labels on exception span events; `exception.type` + `exception.message` are bounded by scenario spec, not user input.
- Do NOT skip fingerprint journaling or emit partial fingerprints (obs-plan §6): if an exception is emitted, the corresponding run-report envelope line MUST carry the computed fingerprints array (empty array if no exceptions, still journaled).

## Contract bindings
- **obs ↔ tests harness** (obs-plan §3 + focus-guide cross-domain): tests bind to obs via the `fingerprints` field schema (tests assert presence/shape of the `fingerprints` array); obs provides the JSON schema + run-report envelope shape (tests verify conformance). Fingerprint computation is deterministic (tests verify same seed ⇒ same fingerprints).

## Acceptance criteria contributions
- (obs) Exception events conform to OTel semantic convention: every exception emitted via `opentelemetry-proto` exception span event carries `exception.type` / `exception.message` / `exception.stacktrace`; stacktrace is deterministic seeded (same seed ⇒ same content); wall-clock-unrelated.
- (obs) Fingerprints journaled deterministically: same seed + same exception spec ⇒ identical expected fingerprint via `ChaCha8Rng::seed_from_u64` (no randomness); fingerprints array populated in run-report JSONL (empty array still journaled when no exceptions).
- (obs) No host-path leaks in exception stacktrace: `exception.stacktrace` redacted at the tracing-subscriber processor stage; absolute file paths → `<redacted>` before JSON emission; allowlisted `target` module path preserved.
- (obs) Fingerprint variants prove identity/path/line semantics: three exception cases journal with expected fingerprints — (a) identical type+frames+line ⇒ same fingerprint; (b) different module/function path ⇒ different fingerprint; (c) same type+frames, different line ⇒ same fingerprint (line-insensitive). Pulse's derived fingerprints verified at read-back stage (out of scope here).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified Run-report envelope vs self-obs base line; `fingerprints` array is an envelope field (scenario-result record, populated at report-write time, Epoch 6 seam). This chunk must compute + store expected fingerprints in a form the report writer can later access + serialize.
- **2026-06-15-log-error-boundary-redaction:** redaction masks absolute host-FILE paths → `<redacted>`; internal struct names kept out by field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge; `target` module path preserved (NOT blanket `::`-token redaction). Applies to any stacktrace content carried into self-obs logs.
- **2026-06-16-emission-journal-writer:** added `read_back_observed_at` (null until read-back) to the JSONL envelope; `latency_ms` computed as `read_back_observed_at − journal_emitted_at` at report-generation time; fingerprints array is a sibling field, journaled together.
- **2026-06-17-raw-otlp-message-scaffold:** no-SDK invariant clarified as behavioral; opentelemetry-proto (the PRODUCT proto lib) is dormant-OK; follow-up to evaluate `default-features = false`. Exception events attach OTel span-event syntax to the PRODUCT stream, not self-obs.
