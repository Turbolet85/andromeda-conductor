# obs extract

## Relevance
Partial — renders the run-report envelope, which includes obs fields (verdict, state, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints) but does NOT implement instrumentation/spans/metrics; rendering only.

## Constraints
- §3 Log format JSON schema: run-report envelope carries 11 fields including `verdict` (enum: Pass|Fail|CalibrationRegion), `state` (enum: Pass|Fail|ManualCheck|KnownResidual|Blocked), `latency_ms` (wall-clock integer or null for blocked rows), `slo_tier` (closed enum: <5s|<20s|<90s), `read_back_observed_at` (ISO-8601 or null until read-back), and `fingerprints` (array or empty).
- §3 Service identity: emit `service.name` (conductor/conductor-tauri/conductor-ui), `service.version`, `deployment.environment` as flat fields on every telemetry signal; render into run-report metadata section.
- §6 Log Coverage: required fields on every log line include `run_id`, `seed`, `scenario`, `p_ids`, plus envelope/result fields — ensure Markdown render surfaces all identity and measurement fields per design.
- §11 PII Scrubbing / Logs: no absolute host paths (drive-letter, /home, /Users, %APPDATA%, ~/.cargo, .rustup, backtrace file paths), no internal struct names (Design: field-allowlist + Display-not-Debug at edge, preserve allowlisted `target`); sanitize at render edge per artifact-hygiene invariant.
- §4 Must-trace paths: Markdown renders per-scenario detail from `RunRecord`s carrying all critical-path measurement fields (latency_ms computed wall-clock, slo_tier, fingerprints array) — ensure render does not invent or transform measurement values.

## Patterns to follow
- Verdict-first lamp precedence (scope §design, carried follow-up (c)): lamp selection is verdict-first (Pass/Fail/CalibrationRegion/blocked hierarchy), rendered as ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` — never color alone (scope §status-never-color-alone invariant); CalibrationRegion renders as `[HOLD]`, not `[MANUAL]`.
- Blocked-row null rule (scope §design): a blocked row (verdict = None, state = Blocked) renders identity + slo_tier only; the five never-measured fields (latency_ms, read_back_observed_at, journal_emitted_at, fingerprints) render as em-dash / absent marker, never the literal `null` or struct name.
- Run-report envelope mirroring JournalWriter/RunsDb: writer takes resolved `runs_dir` path, writes `<run_id>.md`, never overwrites (artifact-hygiene pattern from JSONL/DB sibling chunks).
- Deterministic output for fixed record set: unit + golden (`insta`) tests; only wall-clock generated-at stamp (if any) injected/excluded from golden.

## Anti-patterns to avoid
- NEVER log in hot path at `info` level (§11 Logs) — N/A to render, but respect log-level context when rendering from `tracing` events.
- NEVER leak absolute host paths or internal struct names in run-report artifacts (§11 Logs / PII Scrubbing) — redaction layer masks file paths → `<redacted>`; allowlist+Display edge keeps struct names out; preserve allowlisted `target` module path.
- NEVER use unstructured stderr text (§11 Logs) — render must be machine-parseable JSON schema (run-report is Markdown, but envelope data sources are JSON JSONL/runs.db — preserve shape fidelity).

## Contract bindings
- **Obs ↔ Tests harness** (scope §design, obs §3): run-report Markdown renders the JSONL Run-report envelope fields (binding §3 Log format JSON schema) — tests consume the structured log format + status envelope shape; Markdown is the *human-readable derivation*, not the telemetry source-of-truth.
- **Verdict-first lamp precedence ↔ downstream surfaces** (scope §design, carry forward to ch4/cli/desktop): coverage-matrix, CLI output, desktop GUI reuse the same lamp-selection helper; Markdown establishes the shared rule.

## Acceptance criteria contributions
- (obs) Run-report Markdown renders each measured record with the correct verdict-first lamp (`[PASS]`/`[FAIL]`/`[HOLD]` for CalibrationRegion/`[BLOCKED]`); CalibrationRegion rows show `[HOLD]`, never `[MANUAL]`.
- (obs) A blocked row renders identity fields (scenario, p_ids, seed, run_id) + `slo_tier` only; measurement fields (`latency_ms`, `read_back_observed_at`, `journal_emitted_at`, `fingerprints`) render as em-dash, never `null`.
- (obs) No absolute host path or internal struct name appears in the rendered Markdown; redaction layer applied at render edge (output deterministic for fixed record set).
- (obs) Run-report envelope fields (verdict, state, latency_ms, slo_tier, read_back_observed_at) are faithfully rendered from source `RunRecord`s without transformation or invention; latency_ms is never recomputed (use source wall-clock value only).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** clarified §3 two record shapes — self-obs log line vs Run-report envelope. Envelope carries 11 fields (added context for Markdown render: envelope is scenario-result record, NOT foundational log-line); self-obs base line carries smaller set (timestamp_ms, level, target, service-identity, run_id) — rendering uses envelope shape.
- **2026-06-16-emission-journal-writer:** added `read_back_observed_at` (ISO-8601, null until read-back; null for blocked rows) to envelope schema — second field after `journal_emitted_at`. Markdown render must include this field in detail table and honor null-rendering rule for blocked rows.
- **2026-06-18-severity-logs:** added `emit.logs_batch` span name to bounded set (routine doc-reconciliation for P-007 severity-logs chunk). No impact to Markdown render (span names not rendered as telemetry; render consumes `RunRecord` envelope only).