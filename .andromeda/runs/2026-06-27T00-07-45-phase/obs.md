# obs extract

## Relevance
Partial — chunk builds desktop VIEW of coverage-matrix data; observability surface depends on data-source decision (static vs live backend command).

## Constraints
- Per §1: Minimal obs tier — if a backend command is added, keep instrumentation focused, no background batch tasks breaking `current_thread` determinism.
- Per §3: If a new `#[tauri::command]` is added to fetch live coverage, instrument via `#[tracing::instrument]` (plain `tracing` spans, no OTel SDK); use `CONDUCTOR_AGENT_MODE` (read-only trigger) to gate JSON-only logging to file.
- Per §4 Path 6 (Coverage-matrix completeness gate): span names follow `{module}.{operation}` pattern; if sourcing coverage, span would be `report.query_coverage_matrix` with attributes `p_id_count`, `missing_count`.
- Per §6: Log level = `info` for boundary-call summaries (e.g., coverage query result); JSON schema is binding contract from tests (11-field envelope: journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints).
- Per §11 Anti-Pattern (Logs): no absolute host-file paths, no internal struct names; redaction via field-allowlist + `Display` (not `Debug`) at error edge; `target` module path preserved as correlation field.

## Patterns to follow
- Span naming: `{module}.{operation}` (e.g., `report.query_coverage_matrix` if a read command is added).
- Command handler instrumentation: `#[tracing::instrument]` on the `#[tauri::command]` fn (Tauri commands are IPC boundaries; span is parent of core-operation spans).
- Log output: JSONL via `tracing-subscriber`; service-identity fields (`service.name`, `service.version`, `deployment.environment`) on every line.
- Coverage vocabulary: reuse `Lamp::for_record` (verdict-first semantics) + `LAMP_META` from `ui/src/lamp.ts` — mirror `conductor-core::lamp.rs` spellings, never re-spell (per chunk scope CARRY).

## Anti-patterns to avoid
- No high-cardinality span names: don't name spans after individual P-IDs or result cardinality (e.g., `coverage.p_id_<N>` banned).
- No panics unlogged: if coverage data fetch fails, capture via `std::panic::set_hook()` + `tracing::error!`.
- No re-spelling lamp palette/glyph/label logic: consume `LAMP_META`/`Lamp::for_record` as-is; divergence must be escalated to plan.

## Contract bindings
- **Test harness contract** (tests §3): if a backend command is added, its JSONL output must match binding schema (11-field envelope per §3); tests consume structured log format and status-endpoint shape.
- **Critical Path 6** (Coverage-matrix completeness gate, §4): CLI coverage-matrix generation already traces `report.coverage_matrix_generate` → `db.query_all_p_ids` → `report.validate_coverage`; desktop VIEW consumes that data; if live-fetch command is added, span naming/attributes must align to the already-traced path.
- **Lamp engine** (conductor-core): `Lamp::for_record` verdict-first precedence and glyph/label/color binding is observable in logs via span attributes + coverage result fields; desktop view must preserve that vocabulary (binding honored).

## Acceptance criteria contributions
- (obs) If backend command added: `#[tracing::instrument]` on the `#[tauri::command]` handler; span attributes include `p_id_count`, `missing_count`.
- (obs) Coverage query logged at `info` level with result summary (count of covered/uncovered P-IDs) + latency_ms computed from wall-clock `std::time`.
- (obs) No PII in coverage display (synthetic P-ID/verdict data only); verify via field-allowlist redaction.
- (obs) UI-only path: no new instrumentation required; coverage-matrix generation observability already present via §4 Path 6.

## Relevant amendment history
- **2026-06-15 (structured-logging-stack)**: Two record shapes clarified — self-obs base line (timestamp_ms, level, target, service-identity, run_id) vs Run-report envelope (11 fields including verdict/state/latency). If backend command added, ensure log lines carry base set on every line; envelope fields appear only on coverage-result event (per §3 refinement).
- **2026-06-24 (agent-mode logging)**: Agent mode = `--agent-mode` flag OR `CONDUCTOR_AGENT_MODE` env (read-only trigger, never written by Conductor). If command is added and logs to file, respect this trigger for JSON-only sink (no pretty-print in agent mode).
- **(none from amendments to lamp/verdict/state vocabulary)**: §4 Path 6 (coverage completeness) and §4 Fault-injection spans remain stable; no recent amendments to the coverage-matrix critical path instrumentation.