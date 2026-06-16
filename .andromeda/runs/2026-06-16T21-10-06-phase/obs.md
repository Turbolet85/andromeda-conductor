# obs extract

## Relevance
Relevant — emission-journal writer is the core obs write-side artifact + binding contract.

## Constraints
1. Per obs-plan §3 "Observability Harness Contract / Log format JSON schema": journal lines use binding-contract JSONL schema with fields `journal_emitted_at` (ISO-8601 from `std::time::SystemTime`), `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`, plus scenario-specific fields (`bypass_triggered`, `lifecycle_phase`, `severity_choice_calibrated`, `degraded_mode_response`).
2. Per obs-plan §1 "Instrumentation scope / conductor-report": "Instrumentable — JSONL journal per-run emission stream; Markdown run report generation; rusqlite synchronous embedded SQLite (`runs.db`) for run-metadata; instrumentation of journal write, report render, DB insert operations."
3. Per obs-plan §4 "Span / Trace Coverage": root span `scenario.run` must track attributes `run_id`, `seed`, `scenario`, `p_ids` (array); child spans (`timeline.execute`, `emit.batch`, `verify.readback`, etc.) close at phase boundaries; no high-cardinality span names.
4. Per obs-plan §6 "Log Coverage / Boundary-call wrappers": DB insert (`db.insert_run`) must log row count (1) + `run_id` + `verdict` + `state`; redaction layer must scrub absolute host-file paths (drive-letter, `/home`, `/Users`, etc.) → `<redacted>`, not token-level `::`-redaction.
5. Per obs-plan-amendments 2026-06-15: two record shapes — self-obs base line (every line carries `timestamp_ms`, `level`, `target`, service identity, `run_id`) vs Run-report envelope (journal + `runs.db`, Epoch 6). Emission journal is the Run-report envelope form.
6. Per obs-plan §3 "Correlation": No W3C `trace_id`/`traceparent`; within a run, the `tracing` span hierarchy + the `run_id` field correlate lines. IPC envelope carries `run_id`, not a W3C context.
7. Per scope §Determinism invariant: "wall-clock stamps from `std::time` only; the virtual clock stays scheduling-only so journal-relative SLO math is sound."

## Patterns to follow
1. Per obs-plan §2 "Telemetry Strategy / Naming conventions": span naming `{module}.{operation}` (e.g., `scenario.run`, `timeline.execute`, `emit.batch`, `report.generate`, `db.insert_run`).
2. Per obs-plan §3 "Service identity": `service.name` hardcoded `"conductor"` (CLI) or `"conductor-tauri"` (Tauri); `service.version` compile-time `env!("CARGO_PKG_VERSION")`; `deployment.environment` runtime `$CONDUCTOR_ENV` (default `"local"`).
3. Per obs-plan §4 "Auto-instrumentation per surface / cli": no auto-instrumentation (CLI is not HTTP); manual `#[tracing::instrument]` on core scenario handlers.
4. Per scope "Timestamp contract": journal stored values are integer-millisecond journal offsets SLO math consumes; human-facing RFC-3339 is a render concern. `journal_emitted_at` originates here.

## Anti-patterns to avoid
1. Per obs-plan §11 (implied; see amendments): blanket `::`-token redaction that would remove the allowlisted `target` correlation field or mangle std type names in panics. Redaction anchors to absolute *file* paths only (drive-letter, `/home`, `/Users`, etc.); struct-name leaks are prevented via field-name allowlist + `Display`-not-`Debug` at the edge.
2. Per obs-plan §3 "Correlation / gRPC outbound": do not propagate W3C `traceparent` in the OTLP fault stream — the only OTLP Conductor speaks is the PRODUCT fault stream to Pulse on `:4317`; self-observation never exports OTLP.
3. Per scope "Verdict/error wall": journal I/O failure is a harness fault, never a verdict; treat write failures as `Result::Err`, not as test-level verdict changes.

## Contract bindings
- **tests ↔ obs** per obs-plan §3 "Log format JSON schema" / tests excerpt §5 (binding contract): tests consume the Run-report envelope JSONL schema (journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints).
- **obs ↔ SLO math** per obs-plan §5 "Metric Coverage": `latency_ms` is computed `read_back_observed_at - journal_emitted_at` (wall-clock milliseconds); SLO assertion at report-generation time.
- **timeline (Epoch 2, current chunk predecessor) ↔ emission-journal-writer** per obs-plan §1 "Instrumentation scope": "Deterministic seeded phase scheduler; tokio `current_thread` runtime ensures emission ordering is seed-deterministic; instrumentation spans must use wall-clock (`std::time::SystemTime`/`Instant`) not tokio's virtual clock."

## Acceptance criteria contributions
1. **(obs) Journal write binds to tests schema:** every line in `runs/<run_id>.jsonl` contains JSON object with exact fields from obs-plan §6 Log Coverage binding schema (journal_emitted_at ISO-8601, run_id filesystem-safe, seed u64, scenario string, p_ids array, verdict/state/latency_ms/slo_tier/fingerprints); scenario-specific fields (bypass_triggered, lifecycle_phase, severity_choice_calibrated, degraded_mode_response) present on applicable runs.
2. **(obs) Stamping uses std::time:** `journal_emitted_at` originates from `std::time::SystemTime` / `std::time::Instant` (wall-clock), never tokio's virtual clock; wall-clock-relative SLO math is sound.
3. **(obs) Run_id-keyed, never-overwritten:** files are created as `runs/{run_id}.jsonl`; subsequent emissions on same run append (never truncate/overwrite); filesystem-safe `run_id` format is YYYY-MM-DDTHH-MM-SS-<suffix> (hyphen-delimited per scope & timestamp-contract).
4. **(obs) Redaction at write:** no absolute host-file paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) leak into journal lines; `target` module field preserved; field-name allowlist applied per amendment 2026-06-15-log-error-boundary-redaction.

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified two record shapes — the emission journal is the Run-report envelope (result record from report seam Epoch 6), distinct from the foundational self-obs base-line format (every line carries `timestamp_ms`, `level`, `target`, service-identity, `run_id`). The journal's envelope schema is the binding contract from tests; self-obs lines are a lower-level stream. Cascaded to `.claude/rules/observability.md`.
- **2026-06-15-log-error-boundary-redaction:** reconciled redaction model — blanket `::`-token redaction is anti-pattern (would remove the allowlisted `target` field + mangle std type names). Actual redaction anchors to absolute *file* paths (drive-letter, `/home`, `/Users`, etc.) → `<redacted>`; struct-name leaks are kept out via field-name allowlist + `Display`-not-`Debug` at the edge. Cascaded to `.claude/rules/observability.md` + `.claude/docs/obs-summary.md`.