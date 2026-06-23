# obs extract

## Relevance
Partial — line-oriented output rendering is a CLI presentation layer (zero observability model changes); obs applies to the logging/tracing instrumentation of the render paths themselves.

## Constraints
- per §3 Logging stack: structured JSON sink (stderr pretty-print dev mode / file `logs/agent-latest.jsonl` agent mode) handles all CLI output; no unstructured stderr (§11 Logs anti-pattern)
- per §2 Telemetry Strategy / Naming conventions: span naming follows `{module}.{operation}` pattern; any new render-path spans (e.g., `report.render_table`, `report.render_lines`) must use bounded convention (§11 Spans — bounded span name set: `report.generate`, `db.insert_run`, etc.)
- per §3 Service identity: all JSONL lines carry flat `service.name` / `service.version` / `deployment.environment` fields; render-path instrumentation inherits parent span context + `run_id` correlation field
- per §1 Instrumentation scope: `conductor-cli` is Instrumentable; render seams are boundary calls (owo-colors formatting, indicatif progress updates, comfy-table construction) — low-overhead spans only per §11 "do not over-instrument hot paths"
- per §9 CI Integration: log file (`logs/agent-latest.jsonl`) + structured test output remain CI artifacts; render output contributes no new telemetry artifact
- per §11 Anti-Patterns / Logs: no raw escape sequences in piped/non-tty output (owo-colors `if_supports_color` auto-detection prevents this; agent-mode full JSON is ch5 deferred, §3 log file location)

## Patterns to follow
- per §3 Log format JSON schema + §6 Log Coverage: every self-obs line carries `timestamp_ms`, `level`, `target`, service-identity (`service.name` / `version` / `deployment.environment`), `run_id` (base set); scenario-result envelope fields (`verdict` / `state` / `latency_ms` / `slo_tier` / `fingerprints`) appear only on report-generation events, not on intermediate render-path lines
- per §4 Must-trace paths (Critical Path 1 headless scenario, Path 7 both-surface parity): render operations (`report.generate` closed span, `report.coverage_matrix_generate`) are children of the root `scenario.run` span; render output is post-verdict JSON population, not pre-verdict instrumentation
- per §2 Agent-readable invariants: all render output is machine-parseable (JSONL for logs; textual status lines with stable ASCII prefixes for piped consumers); color is optional tty-gated overlay, never the sole encoder

## Anti-patterns to avoid
- NEVER use unstructured stderr text (§11 Logs) — any render-path error or progress indication must be captured via `tracing::warn!` / `tracing::info!` with structured fields, or as part of the existing JSONL stream
- NEVER introduce high-cardinality span names (§11 Spans) — render operations are boundary calls, not per-element instrumentation (e.g., no span per table row; the `comfy-table` seam is one span with `row_count` attribute)
- NEVER lose the `run_id` correlation field (§11 Logs) — render-path spans and all self-obs lines inherit the parent scenario context; `run_id` stays on every line for correlation

## Contract bindings
- obs ↔ tests harness (§3 Observability Harness Contract): render-path output does not alter the JSONL schema; tests still consume the same log format + status envelope (no new telemetry artifact from rendering)
- obs ↔ a11y (scope §a11y): the "never color-alone" rule (ASCII prefix always present; color optional overlay) is the accessibility contract on line-oriented output (color is tty-gated; piped/non-tty lines remain unambiguous)

## Acceptance criteria contributions
- (obs) Render-path spans (`report.render_lines`, `report.render_table`) use bounded span name convention per §4 §11 and carry `row_count` / `table_section` attributes (no per-element cardinality)
- (obs) All render-path errors + progress feedback logged via `tracing::{warn,info}` with structured fields; no unstructured stderr (§11 Logs anti-pattern)
- (obs) JSONL log lines from render operations include `run_id` + `timestamp_ms` + service-identity fields (base set); no new envelope fields (scenario-result envelope populated only at report-generation, Epoch 6)
- (obs) Piped / non-tty output contains no raw escape sequences (owo-colors auto-detection via `if_supports_color`); agent-mode structured logging deferred to ch5

## Relevant amendment history
- 2026-06-15-structured-logging-stack (§3): self-obs base line (foundational, every line) = `timestamp_ms` + `level` + `target` + service-identity + `run_id`; Run-report envelope (verdict/latency/fingerprints) appears only on scenario-result events. Render-path lines follow the base set; envelope fields populated post-verdict by report seam (Epoch 6).
- 2026-06-18-severity-logs (§11): bounded span name set updated to include `emit.logs_batch` (low-cardinality egress spans). Render-path spans conform to same bounded set; no new high-cardinality names introduced.