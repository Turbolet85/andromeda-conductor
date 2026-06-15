# obs extract

## Relevance
partial — dependency audit gate is a supply-chain policy chunk; obs contributes PII/path scrubbing anti-patterns and CI artifact handling, NOT observability instrumentation

## Constraints
- Per obs-plan §3 Service Identity: no hardcoded credentials or path leaks in resource attributes (applies to audit logging if any status is captured)
- Per obs-plan §6 Log Coverage: logs must match JSONL schema with required fields (journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints); audit gate produces no telemetry matching this shape — cargo-audit/cargo-deny output is pure config/status verification
- Per obs-plan §9 CI Integration: telemetry artifacts uploaded as CI artifacts with 14-day retention and agent-parseable format (applies ONLY when this chunk's results are consumed by downstream CI wiring, which is deferred to later chunk "Base CI + agent-run harness skeleton")
- Per obs-plan §11 Anti-Patterns (PII Scrubbing): NEVER leak absolute host paths or internal crate names in any log output; if cargo-audit/cargo-deny stderr is captured, sanitize via redaction layer (path::, module::, backtrace removal)
- Per obs-plan §1 Obs Scope Summary: this chunk does not produce any signal in the critical-path 7 scenarios (Section 4); audit gate is a **boundary-only** verification (not instrumentable)

## Patterns to follow
- Audit tooling status is **not a telemetry signal** — cargo-audit/cargo-deny exit codes are CI gate pass/fail, not span/log events
- If audit output is piped to logs (future CI wiring), sanitize stderr before storage: strip absolute filesystem paths (redaction layer §11)
- `--locked` build verification for Cargo.lock reproducibility aligns with determinism principle (tokio `current_thread` runtime requirement in obs-plan §1 Stack)

## Anti-patterns to avoid
- NEVER add an external audit-reporting platform (e.g., Snyk SDK) that would bypass cargo-audit/cargo-deny verification
- NEVER leak RustSec advisory details (paths, crate versions, host context) into unstructured stderr without sanitization
- NEVER introduce a metrics instrument for audit gate (e.g., "advisory count histogram") — gate is boolean (pass/fail), not a perf-budget SLO

## Contract bindings
(none) — this chunk is a supply-chain gate; no direct binding to obs harness (CLI logger, spans, PII scrubbing) is activated until CI wiring in later chunk

## Acceptance criteria contributions
(none from obs domain) — obs criteria apply ONLY when audit results are logged/exported; this chunk runs locally with cargo-audit/cargo-deny CLIs, producing no structured telemetry

## Relevant amendment history
(none) — no prior amendments to obs-plan touching dependency-audit or supply-chain audit scope