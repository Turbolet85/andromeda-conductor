# obs extract

## Relevance
Relevant — this chunk is squarely an obs-domain chunk: it lands three of the six must-trace spans in obs-plan §4 Critical Path 1 (the root plus the terminal pair).

## Constraints
- Span names must be drawn from the bounded low-cardinality set; `scenario.run`, `report.generate`, `db.insert_run` are already members, so no name is invented and no run-scoped suffix may enter a name — identity goes in attributes (per obs-plan §11 Spans / Traces).
- Attribute sets are fixed by the plan, not by convenience: `scenario.run` = `run_id`/`seed`/`scenario`/`p_ids`; `report.generate` = `verdict`/`state`; `db.insert_run` = `row_count` (per obs-plan §4 "Scenario: Headless deterministic scenario run…" Required span attributes; mirrored in §1 must-trace table).
- Parenting is plain `tracing` span nesting only — no OTel SDK, no `tracing-opentelemetry`, no `traceparent`/`trace_id`; the correlation key stays `run_id` (per obs-plan §3 OTel SDK init + §3 Correlation).
- The new spans emit onto the **self-obs base line** record shape (`timestamp_ms`/`level`/`target`/service identity/`run_id` on every line); the Run-report envelope in `runs/<run_id>.jsonl` + `runs.db` is a distinct record and stays untouched (per obs-plan §3 Log format JSON schema — "Two record shapes").
- Every new span field must pass the processor-stage field-name allowlist in `conductor-core::redact` — non-allowlisted field names are DROPPED, so an unregistered attribute silently disappears rather than failing loudly (per obs-plan §11 PII Scrubbing + §11 Logs).
- Cleanup is specified: the root closes on `db.insert_run` completion and children close at their phase boundaries; no dangling spans (per obs-plan §4 CP1 Cleanup + §11 Spans / Traces).
- Instrumentation must not perturb `current_thread` determinism or journal timing — no background tasks, wall-clock (`std::time::SystemTime`) only, never tokio's virtual clock (per obs-plan §11 Project-specific bans + §1 conductor-timeline instrumentability row).

## Patterns to follow
- Manual `#[tracing::instrument]` at seams, `{module}.{operation}` naming — the shipped `timeline.execute` / `emit.batch` / `verify.readback.*` spans are the in-tree exemplar; §4's auto-instrumentation table already prescribes "manual span around rusqlite queries" for the report/DB seam (per obs-plan §4 Auto-instrumentation per surface + §2 Naming conventions).
- The custom `tracing-subscriber` layer that emits flat identity fields + `run_id` on every line — the root span's `run_id` attribute should reconcile with that existing per-line field rather than introduce a second spelling (per obs-plan §3, "two record shapes" note).
- Boundary-call must-log events already specified for exactly these two seams: `report.generate` logs verdict/state/fingerprint count, `db.insert_run` logs row count + `run_id` + verdict + state — the spans pair with these events at `info` (per obs-plan §6 Boundary-call wrappers + §6 log-levels table, conductor-report base level `info`).
- `db.insert_run` is a **Client**-kind span (rusqlite is a synchronous boundary off the async runtime); `scenario.run` / `report.generate` are Internal (per obs-plan §4 Span kinds).
- Verification reuses the shipped §9 log-conformance scaffold reading `logs/agent-latest.jsonl` rather than a new harness (per obs-plan §9 Log conformance check) — matches the scope's `[inferred]` verification call.

## Anti-patterns to avoid
- Reaching for an OTel SDK / `tracing-opentelemetry` / W3C `traceparent` to make the tree "real" — banned for self-obs on every surface, and no self-obs OTLP to `:4317` or `:4318` (per obs-plan §11 Telemetry Strategy + §11 Universal).
- High-cardinality span names (`scenario.run.<run_id>`, per-P-ID names) or renaming the shipped spans out of the bounded set (per obs-plan §11 Spans / Traces).
- Over-instrumenting the emit inner loop while chasing chain contiguity, or leaving a span unclosed on the error path (per obs-plan §11 Telemetry Strategy + Spans / Traces).

## Contract bindings
- **obs ↔ tests:** the emitted JSON line shape is the tests-owned harness contract; test-plan §3 owns the Run-report envelope schema, obs derives — this chunk may add self-obs span fields but must not re-author envelope fields (per obs-plan §3 Log format JSON schema, "obs aligns to tests").
- **obs ↔ CI:** the §9 `logs/agent-latest.jsonl` conformance gate + zero-unlogged-panics gate consume the stream these spans join; a new field that breaks the base schema or leaks a host path fails the build (per obs-plan §9).
- **obs ↔ security:** the redaction boundary is `conductor-core::redact` (field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge), binding security-plan §Error Handling / §Logging — new span fields cross it (per obs-plan §11 PII Scrubbing).
- **obs ↔ cross-surface parity:** placing the root at the shared composition root yields an identical tree under both bins, with `tauri.command.start_scenario` as the parent above `scenario.run` on the GUI path; parity itself is still asserted by the `runs.db` envelope (same seed ⇒ same verdict/state), never by trace correlation (per obs-plan §4 "Scenario: Both-surface parity").

## Acceptance criteria contributions
- A real agent-mode run's `logs/agent-latest.jsonl` shows `scenario.run` as an ancestor of `timeline.execute`, `emit.batch` and `verify.readback*`, carrying exactly `run_id`, `seed`, `scenario`, `p_ids` (per obs-plan §4 Scenario: Headless deterministic scenario run — Must-trace spans + Required span attributes).
- `report.generate` (`verdict`, `state`) and `db.insert_run` (`row_count` = 1) are emitted, and the root closes on `db.insert_run` completion with no dangling spans on either the success or error path (per obs-plan §4 CP1 Required span attributes + Cleanup).
- Every line the new spans produce still carries the §3 self-obs base set and passes the conformance gate: no absolute host-file paths in any field, allowlisted `target` preserved (per obs-plan §9 Log conformance check).
- No OTel SDK, exporter, or `traceparent` is introduced, and all emitted span names remain within the bounded span-name set (per obs-plan §11 Spans / Traces + §11 Universal).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Log format JSON schema) — established the two record shapes; the reason this chunk's span fields land on the self-obs base line and not on the Run-report envelope. Also the reason a custom subscriber layer exists (stock `fmt().json()` cannot emit constant identity fields flat).
- **2026-06-15-log-error-boundary-redaction** (§6 / §11 Logs · PII Scrubbing) — pinned the implemented redaction model: value scrub targets absolute host **file** paths, struct names are excluded by the field-name allowlist + `Display` at the `anyhow` edge, and `target` is explicitly preserved. Directly governs whether this chunk's new attributes survive the layer.
- **2026-06-18-severity-logs** (§11 bounded span-name set) — precedent for the only sanctioned way to grow the span vocabulary: a conforming `{module}.{operation}` member added by amendment. This chunk's three names are already in the set, so no amendment should be needed — if one is, that is the shape it takes.
- **2026-06-27-obs-ci-conformance-gate** (§9) — the shipped gate asserts the §3 base schema, not the §6 envelope; it is the verification scaffold this chunk reuses, and it dogfooded PASS-on-real-artifact / FAIL-on-missing-base-field.
- **2026-06-17-raw-otlp-message-scaffold** (§3 OTel SDK init) — the no-SDK invariant is *behavioral* (never initialized/used for self-obs); a dormant transitive `opentelemetry_sdk` in the dep tree is not a violation, so the D-obs-stack detector should not re-fire on that basis here.
