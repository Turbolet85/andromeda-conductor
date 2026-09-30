# obs extract

## Relevance
partial — the chunk's product work is the React webview's accessible structure (row names, status cells, heading/landmark structure) plus the `sr*` leg's row table and regrade record; no new span, metric, log line, Tauri command or telemetry surface is in scope, so obs binds only through the frontend telemetry posture, the host-path/redaction boundary on committed SR evidence, and the CI obs gates the "every existing test stays green" boundary keeps live.

## Constraints
- The webview frontend's telemetry stays console-JSON-only: obs-plan §2 (surface table, desktop-webview row) and §3 Logging stack require the frontend sink to be `console.log(JSON…)` with NO OTel JS SDK, NO browser OTLP exporter and NO `:4318` — any diagnostic a product a11y fix adds to the React tree must follow that sink, never a network export (per obs-plan §2; §3 Logging stack).
- The frontend's service identity is `conductor-ui` per obs-plan §3 Service identity; the a11y violation record that the routine `a11y` job writes carries that tag as a §9 resource tag on top of the eleven envelope keys, so a product change that moves selectors must leave that record's shape unchanged (per obs-plan §3 Log format JSON schema; §9 Telemetry artifact handling).
- A regrade record committed as evidence must carry no absolute host path: obs-plan §11 PII Scrubbing requires that the redaction enumeration is never assumed to bound every host-path channel, and names the SR leg's own `<host-path>` scrub + `security_finding` flag as the standing second line of defence for speech-log content (per obs-plan §11 PII Scrubbing; §11 Logs).
- If the chunk touches any Tauri command handler (research's question — the scope expects zero or near-zero Rust delta), each handler keeps its manual `tracing::info_span!("tauri.command.<name>").entered()` guard, never the `#[tracing::instrument]` attribute, and span names stay within §11's bounded set (per obs-plan §4 desktop-webview row; §11 Spans / Traces).
- The per-suite Tauri backend log directory rides `CONDUCTOR_RUNS_DIR` (`runs/sr-leg/logs/` for `sr`/`sr-error`, `runs/logs/` for routine `--e2e` + `sr-empty`, `runs/driven/logs/` for the driven arm); a leg-table change must not redirect or merge those sinks (per obs-plan §3 Log file location).
- The PREREQ's `--unit` run in full must satisfy the zero-unlogged-panics invariant — every panic captured by `std::panic::set_hook()` and logged as one-line structured JSON (per obs-plan §10 Always-required SLO invariant; §9 Zero-unlogged-panics gate).

## Patterns to follow
- Frontend diagnostics, if any are needed while confirming rows against the rendered DOM, go to `console.log(JSON.stringify(event))` as paste-to-AI output, with no file persistence and no export (per obs-plan §3 Logging stack; §3 Log file location).
- Host-path hygiene on third-party text entering committed evidence: scrub to the `<host-path>` placeholder and raise a `security_finding` row rather than dropping or echoing the match — the pattern §11 cites as the SR leg's scrub (per obs-plan §11 PII Scrubbing).
- The a11y violation record keeps the eleven-key envelope plus the §9 resource tags (`service.name`, `deployment.environment`, and `ci.run.id` + `git.commit.sha` under CI), with conformance asserted by `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y` (per obs-plan §3 Log format JSON schema; §9 Telemetry artifact handling).

## Anti-patterns to avoid
- NEVER introduce an OTel SDK, `web-vitals` export or any OTLP egress from the React frontend to instrument the a11y fixes — self-obs is console JSON only on every surface, including the browser (per obs-plan §11 Telemetry Strategy; §11 Universal).
- NEVER leak an absolute host path or internal struct name into a committed artifact (the regrade record, run-report, `runs.db`, JSONL) (per obs-plan §11 Logs; §11 PII Scrubbing).
- NEVER use unstructured stderr text or multi-line stack traces in any harness or product output the chunk adds (per obs-plan §11 Logs).

## Contract bindings
- obs ↔ a11y: the `a11y` job's violation record (`runs/a11y/<run_id>.jsonl`) is an obs-owned record shape consumed by the a11y gate; the chunk's product change to row names / heading structure must keep the routine axe/contrast arm green and the record conformant (per obs-plan §9 Telemetry artifact handling; §3 Log format JSON schema).
- obs ↔ security: the host-path redaction boundary on SR speech-log ingest (the `<host-path>` scrub + `security_finding`) is shared with security-plan's untrusted-third-party-text rule; obs-plan names it the second line of defence behind `conductor-core::redact` (per obs-plan §11 PII Scrubbing).
- obs ↔ tests: the PREREQ gates (`cargo clippy …` and `bash scripts/agent-run.sh run --unit`) feed the §9 log-conformance check (the §3 self-obs base-line schema on `logs/agent-latest.jsonl`) and the zero-unlogged-panics gate (per obs-plan §9 Log conformance check; §9 Zero-unlogged-panics gate).

## Acceptance criteria contributions
- (obs) The committed regrade record (`evidence/nvda-pass.json` or its successor) carries zero absolute host paths; any speech-log host path appears only as the `<host-path>` placeholder with a matching `security_finding` row, and the count of each is stated (per obs-plan §11 PII Scrubbing).
- (obs) The chunk's frontend delta adds no OTel/OTLP import or network export; a grep of the webview source delta for `opentelemetry`, `OTLP` and `4318` returns zero new hits (per obs-plan §11 Universal; §2 desktop-webview row).
- (obs) The routine `a11y` job's violation record stays conformant to the eleven envelope keys plus §9 resource tags after the row-name / heading changes, as asserted by `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y` (per obs-plan §9 Telemetry artifact handling).
- (obs) The PREREQ's full `--unit` run shows zero unstructured panic lines (`^thread.*panicked`) in `logs/agent-latest.jsonl` or stderr (per obs-plan §9 Zero-unlogged-panics gate; §10 Always-required SLO invariant).
