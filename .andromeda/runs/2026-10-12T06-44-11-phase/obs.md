# obs extract

## Relevance
partial — the chunk retires one named self-obs sink (the Tauri backend's) and re-shapes or removes `drive_run`, the in-process arm of a must-trace critical path. It also adds command-line entries that run through the self-obs stream. Instrumentation depth stays Minimal tier (per obs-plan §1 Obs Scope Summary).

## Constraints
- obs-plan §3 Logging stack requires the CLI sink to be a WRITER choice over one `JsonObsLayer` (stderr in non-agent mode, `logs/agent-latest.jsonl` under `--agent-mode` or the read-only `CONDUCTOR_AGENT_MODE` trigger). Retiring the Tauri backend sink must leave that CLI file sink (`ObsSink::File`) and its agent-mode trigger intact. The scope names this sink as shared and staying. Whether any code path besides the CLI's still constructs the file sink is research's question.
- obs-plan §3 Log file location names `<runs_dir.parent()>/logs/conductor-tauri.jsonl` as having "no writer since 2026-10-11", and it names this entry as the place the sink goes. Its master rows (§3 Logging stack, §3 Log file location, §6 Sink configuration) retire through the wrap's amendments, never through a phase edit (scope item 5). `operator_pause_harvest.rs`'s read of that path belongs to `Window grading retired` and is not this chunk's.
- obs-plan §4 Scenario "In-process ↔ headless parity" names `conductor_run::{preflight, drive_run}` → `scenario.run` as the in-process arm's must-trace chain. That chain must carry the same `scenario.run` attributes (`run_id`, `seed`, `scenario`, `p_ids`) as the CLI arm. If `drive_run` is deleted or re-shaped, the parity arm still needs a `scenario.run` root with those attributes. Otherwise the critical-path row loses its referent, which is a wrap amendment and never a silent gap. Whether the CLI `run` / `suite` paths already share `drive_run`'s instrumentation is research's question.
- obs-plan §6 Boundary-call wrappers requires the DB READ (`conductor_run::read_envelope` over `RunsDb::get_envelope`) to log one `info!` line carrying `run_id` and the outcome on the allowlisted `message` field, inside the caller's span, and to mint no `db.*` read span. If the read is re-homed to the CLI for a past run's load-envelope standing (scope item 10), the boundary log comes with it.
- obs-plan §6 Per-module log levels records that `conductor-cli` declares no `tracing` dependency. New verbs and flags (catalogue listing, graceful stop, envelope standing) therefore emit self-obs only through the library targets they drive. Adding a `tracing` edge to the CLI is the point where §6's deferred DB-DELETE boundary log and the CLI's own gating come due.
- obs-plan §10 SLO Invariants requires zero unlogged panics. A graceful mid-run stop (scope item 9) must end through the typed path, never a panic or an abort that bypasses `std::panic::set_hook()`, and it must keep the finished rows' wall-clock `journal_emitted_at` / `read_back_observed_at` intact.
- obs-plan §11 Spans / Traces bans dangling spans. A mid-run stop must close `scenario.run`, `timeline.execute` and any open fault span at the stop boundary. That includes `fault.port_occupier`, whose close is its RAII release (§4 Fault-injection spans).

## Patterns to follow
- Correlation is the `run_id` field on every self-obs line, and parity is matched by `(scenario, seed)` over the `runs.db` envelope, never by trace continuity (per obs-plan §3 Correlation (no distributed tracing)).
- A self-obs line has a base set (`timestamp_ms`, `level`, `target`, service identity, `run_id`) plus span-lifecycle variants (`span`, `span_event` new|close, `parent`). Envelope fields belong to the scenario-result record alone (per obs-plan §3 Log format JSON schema, "Two record shapes").
- Derived counts or outcomes for a new read-only verb ride the allowlisted `message` field rather than a new attribute name, the same as the coverage roll-up and the MCP key-set witness (per obs-plan §4 Scenario "Coverage-matrix completeness gate" and §6 Boundary-call wrappers).
- Pair a per-target `RUST_LOG` directive with the global level (`info,{crate}=debug`), and never set either for a run that also executes the suite (per obs-plan §6 Per-module log levels).

## Anti-patterns to avoid
- Never leak absolute host paths or internal struct names into logs, the run report or `runs.db`. This covers the catalogue listing's output and the envelope-standing read (per obs-plan §11 Logs). Whether `ScenarioSummary` or the listing's read carries a filesystem path is research's question.
- Never use unstructured stderr text for a new verb's self-obs. Every line must be structured JSON or sanitized stderr with machine-parseable hints (per obs-plan §11 Logs).
- Never add a span name outside the bounded set (scenario.run, timeline.execute*, emit.batch, emit.logs_batch, verify.readback*, report.generate, db.insert_run, fault.*), for example a `db.*` read span or a per-verb span (per obs-plan §11 Spans / Traces).

## Contract bindings
- obs ↔ tests: the parity critical path. `crates/conductor-cli/tests/cross_surface_parity.rs` drives `drive_run` as its in-process arm, and obs-plan §4 Scenario "In-process ↔ headless parity" names that arm's spans. Re-shaping `drive_run` binds to test-plan §1 Critical Path 7, whose panel-launched half was already retired, per the handoff.
- obs ↔ tests: the `--agent-mode` file sink feeds the CI log-conformance and zero-unlogged-panics gates over `logs/agent-latest.jsonl` (obs-plan §9 CI Integration), and the CLI's agent-mode self-obs test reads it (§6 Per-module log levels).
- obs ↔ security: the redaction / field-allowlist layer (`conductor-core::redact`) governs any new verb's self-obs (obs-plan §11 PII Scrubbing). The `MAX_CHECKLIST_TEXT` garde bound is a trust-boundary rule that must not be skipped (obs-plan §11 Project-specific bans), whatever happens to its panel justification.

## Acceptance criteria contributions
- No writer for, and no reference to, `conductor-tauri.jsonl` remains under `crates/` except `operator_pause_harvest.rs`, which `Window grading retired` owns. The `--agent-mode` run still writes `logs/agent-latest.jsonl`, and the CLI's agent-mode self-obs test stays green (per obs-plan §3 Log file location).
- If `read_envelope` survives or is re-homed, a past run's envelope-standing read emits exactly one `info!` boundary line carrying `run_id` and no `db.*` span line (per obs-plan §6 Boundary-call wrappers).
- After a graceful mid-run stop under `--agent-mode`, `logs/agent-latest.jsonl` meets three checks. Every `span_event: new` has a matching `close`. Every line carries `run_id`. No line matches `^thread.*panicked` (per obs-plan §11 Spans / Traces).
- The parity test still asserts identical `verdict` / `state` for the same deterministic scenario and seed across its two arms through the `runs.db` envelope, whatever `drive_run` becomes (per obs-plan §4 Scenario "In-process ↔ headless parity").
