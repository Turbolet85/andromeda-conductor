# obs extract

## Relevance
Relevant (partial) — no new critical path or envelope change, but the chunk adds an instrumented contract-load seam in `conductor-core` and extends the instrumented `verify.readback.preflight` gate with new named `Blocked` preconditions, both of which emit self-obs log lines subject to the redaction + conformance gates.

## Constraints
- New instrumentation must stay inside the bounded, low-cardinality span-name set; the preflight extension belongs under the existing `verify.readback*` family, and any new loader span must follow `{module}.{operation}` and be added to the bounded set by amendment — never a per-term or per-path name (per obs-plan §4 Span naming convention, §11 Spans / Traces).
- Every line this chunk emits is a **self-obs base line** carrying `timestamp_ms` / `level` / `target` / `service.{name,version,environment}` / `run_id` — NOT the Run-report envelope; the two record shapes are distinct and preflight/contract-load lines must not be stamped with envelope fields (per obs-plan §3 "two record shapes").
- Any structured field attached to a new line (term name, contract identity, count) must be in `conductor-core::redact`'s field-name allowlist or it is DROPPED at the processor stage; no Debug-dumped struct fields (per obs-plan §11 PII Scrubbing).
- Every new blocked-precondition string, contract-load error, and log value is host-path-free: the value scrub masks absolute host-FILE paths → `<redacted>`, and `data_dir` stays redacted in the readiness envelope; the allowlisted `target` module path is preserved (do not blanket-redact `::` tokens) (per obs-plan §11 Logs, §9 Log conformance check).
- An absent/malformed run-contract manifest or an unmet term must never panic and never surface an unsanitized error: typed `Ok(Blocked)` with a named precondition, or a sanitized `Err` at the `anyhow` edge (`Display`-not-`Debug`) (per obs-plan §10 zero-unlogged-panics, §11 Error Reporting).
- No OTel SDK, exporter, metric instrument, or `traceparent` may be introduced to observe or correlate the run contract — the contract concerns env on a process Conductor does not launch, and self-obs never exports OTLP (`:4317` is the PRODUCT stream, `:4318` is dead) (per obs-plan §3 OTel SDK init, §5, §11 Metrics / Universal).
- If P-073 retires from `UNBACKED_AUTO` in this commit, the coverage roll-up's derived `(N unbacked)` qualifier and `coverage_percent`'s in-scope denominator stay pin/manifest-derived — never literals, and unbacked is not a fifth mode (per obs-plan §4 Coverage-matrix completeness gate, denominator semantics).

## Patterns to follow
- Contract-loader boundary pattern — `D:\dev\projects\conductor\crates\conductor-core\src\capability_manifest.rs` (`default_path()` → `load(path)`: `io::Error.kind()` never the path, `crate::sanitize_error` on the toml parse error, then one `tracing::info!(count = …, "loaded …")` boundary line with the allowlisted `count` field); `D:\dev\projects\conductor\crates\conductor-core\src\load_envelope.rs` is the second instance (per obs-plan §6 boundary-call wrappers).
- Blocked-precondition logging pattern — `D:\dev\projects\conductor\crates\conductor-verify\src\preflight.rs` logs one `info!` line keyed on `state = ReportState::Blocked.label()` inside the `#[tracing::instrument(name = "verify.readback.preflight", skip_all)]` root; new terms extend the same cascade and the same single line, not a new sink (per obs-plan §6 log-level mapping: `info` = boundary summaries + state transitions).
- Seam-side `redact_value(...)` before any string enters the readiness envelope (`data_dir`, tool-list error, connect error in `preflight.rs`) — apply the same to term-derived strings (per obs-plan §11 PII Scrubbing: redact at processor/seam stage, not at the sink).
- Existing `verify.readback.*` span family in `D:\dev\projects\conductor\crates\conductor-verify\src\client.rs` (`connect*`, `list_tools`, `call_tool` with `skip_all` / `skip(self, …)`) — extend this family rather than minting a top-level name (per obs-plan §4 auto- vs manual-instrumentation table).

## Anti-patterns to avoid
- NEVER put the resolved contract file path, an absolute host path, or a raw `data_dir` into a precondition string, an error message, or a log field — the §9 conformance gate fails the build on it (per obs-plan §11 Logs, §9).
- NEVER let a missing/malformed contract file or an un-observable term panic, or forward an unsanitized panic/backtrace to stderr or artifacts (per obs-plan §11 Error Reporting, §10).
- NEVER introduce an OTel SDK / meter / `tracing-opentelemetry` / `traceparent` to "observe" deterministic-L4 or correlate Conductor with `pulse-app`; `run_id` is the only correlation key (per obs-plan §11 Telemetry Strategy / Spans / Universal).

## Contract bindings
- **obs ↔ tests harness** — the `ReadyState` readiness envelope + its named `blocked_precondition` set (today four) is the harness surface `agent-run boot` / `conductor preflight --json` emits and tests assert; adding terms extends that bound shape, and obs owns only its host-path-free/redacted form (per obs-plan §3 Observability Harness Contract; schema ownership stays with test-plan §3).
- **obs ↔ security** — host-path/PII scrubbing is single-location in `conductor-core::redact` (field-name allowlist + value scrub); a new allowlist entry is the only sanctioned way to surface a new field (per obs-plan §11 PII Scrubbing).
- **obs ↔ coverage gate (conditional)** — if the `UNBACKED_AUTO` pin shrinks for P-073, the not-yet-built Epoch-6 completeness gate's attributes (`p_id_count_expected`, `coverage_percent`, derived `(N unbacked)`) shift with the pin (per obs-plan §4 Coverage-matrix completeness gate).
- **Not bound** — the 11-field Run-report envelope / `runs.db` row is untouched by this chunk (per obs-plan §6).

## Acceptance criteria contributions
- (obs) Every new log line from the contract loader and the extended preflight validates against the §3 self-obs base schema (`timestamp_ms`/`level`/`target`/`service.{name,version,environment}`/`run_id`); the `logs/agent-latest.jsonl` conformance gate stays green (per obs-plan §9 Log conformance check).
- (obs) No absolute host path in any new blocked-precondition string, contract-load error, or log field, and `data_dir` remains redacted in the `ReadyState` envelope (per obs-plan §11 Logs / §9).
- (obs) Absent/malformed run-contract manifest and every unmet term yield a typed sanitized outcome (`Ok(Blocked)` with a named precondition, or `CoreError::Config` via `sanitize_error`) — zero unlogged panics; the CI panic-grep gate stays green (per obs-plan §10 SLO invariants / §11 Error Reporting).
- (obs) New instrumentation uses only bounded `{module}.{operation}` span names in the `verify.readback*` family, with allowlisted fields only — a non-allowlisted field is dropped, never leaked (per obs-plan §4 / §11 Spans / Traces + PII Scrubbing).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3) — established the two record shapes; this chunk's preflight/loader lines are self-obs base lines, so do not attach envelope fields (`verdict`/`latency_ms`/…) to them.
- **2026-06-15-log-error-boundary-redaction** (§6/§11) — pinned the redaction model to the implemented `conductor-core::redact`: host-FILE-path anchor + field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge, with the `module::`-shaped `target` explicitly preserved. Directly governs this chunk's contract-path and unmet-term strings; do not "fix" them with `::`-token redaction.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3) — reworded agent mode to a **read-only** env trigger Conductor reads but never writes. Precedent posture for this chunk's `ANDROMEDA_PULSE_*` terms: recorded/read conditions, never Conductor setting env on a process it does not own.
- **2026-06-27-obs-ci-conformance-gate** (§9) — the shipped gate asserts the §3 base schema against `logs/agent-latest.jsonl` (not the §6 envelope); it is the live gate that will validate this chunk's new lines.
- **2026-06-18-severity-logs** (§11 bounded span set) — precedent for adding a conforming, low-cardinality span name (`emit.logs_batch`) to the bounded set via amendment; the route to follow if a new contract-load span name lands.
- **2026-08-08-sut-capability-manifest · 2026-08-09-out-of-scope-classification-treatment · 2026-08-09-interpretation-correctness-posture** (§4 coverage gate) — successive de-hardcoding of the gate's counts: expected-count from the manifest, `coverage_percent` over the in-scope denominator, and the auto term's derived `(N unbacked)` qualifier read from `UNBACKED_AUTO`. Relevant only if this chunk shrinks the pin for P-073 — the counts stay derived.
