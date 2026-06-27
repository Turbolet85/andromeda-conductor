# obs extract

## Relevance — relevant

This chunk adds a GitHub Actions CI gate for observability schema validation and zero-panic assertions on the self-observation artifact. It directly asserts the binding obs schema and redaction boundary at CI time without engine/seam change.

## Constraints

- Per obs-plan §1: obs tier is Minimal (0), driving that self-observation is `tracing` logs only (no OTel SDK for self-obs, hardcoded creator ban)
- Per obs-plan §3: self-obs harness contract specifies `tracing` 0.1.44 + `tracing-subscriber` 0.3.23 for JSON logging; agent-mode flag OR `CONDUCTOR_AGENT_MODE` env (read-only trigger) forces file sink `logs/agent-latest.jsonl`
- Per obs-plan §6: log schema binding contract — self-obs base lines carry `timestamp_ms`, `level`, `target`, `service.{name,version,environment}`, `run_id` on every line; Run-report envelope adds `journal_emitted_at`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints` on scenario-result records
- Per obs-plan §9: CI log conformance gate must validate every line parses as JSON, carries required base fields, contains no leaked absolute host paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`), preserves allowlisted `target` module path
- Per obs-plan §10: zero-unlogged-panics is always-required SLO invariant; every panic MUST be captured via `std::panic::set_hook()` + logged as structured `tracing::error!(panic=…)` JSON
- Per obs-plan §11 (Logs): NEVER leak absolute host paths or internal struct names — redaction layer masks absolute *file* paths to `<redacted>`, NOT blanket `::` token redaction; allowlisted `target` module path is preserved as documented identity field
- Per obs-plan §9 (CI Integration): telemetry artifact handling table specifies upload `logs/agent-latest.jsonl` with `if: always()` + 14-day retention

## Patterns to follow

- Per obs-plan §3 (amended 2026-06-24): invoke CLI with `--agent-mode` flag (or read `CONDUCTOR_AGENT_MODE` env set by operator) to produce `logs/agent-latest.jsonl` without live Pulse (Blocked preflight still emits self-obs lines with service identity + `run_id`)
- Per obs-plan §9: conformance gate exits non-zero on schema violation (missing required field, leaked absolute path, struct-name leak); zero-unlogged-panics gate greps `^thread.*panicked` from stderr + JSONL and exits non-zero on match
- Per obs-plan §6/§9: when validating envelope fields (journal_emitted_at, seed, verdict, state, latency_ms, slo_tier, fingerprints), gate must reconcile whether to assert self-obs base lines (timestamp_ms, level, target, service.*, run_id) or envelope schema against `agent-latest.jsonl` (expected: base set, per amendment 2026-06-15); open whether gate also checks `runs/<run_id>.jsonl` envelope schema separately

## Anti-patterns to avoid

- Per obs-plan §11 (Logs): NEVER use unstructured stderr text — all gate output must be structured JSON or sanitized with machine-parseable hints
- Per obs-plan §11 (Logs): NEVER use blanket `::` token redaction — allowlisted `target` module path (e.g. `conductor_core::obs`) is explicitly preserved; do NOT flag it as struct-name leak
- Per obs-plan §11 (CI): NEVER lose telemetry artifacts — always upload with `if: always()` and specified retention
- Per obs-plan §11 (CI): NEVER skip zero-unlogged-panics gate — it is always-required CI assertion
- Per obs-plan §11 (SLO): NEVER add retry-once or soft-budget policies for panic — masks real failures

## Contract bindings

- **obs ↔ tests harness** (per focus guide § obs §3): Tests consume structured JSONL log format + status endpoint from CI artifacts; gate must ensure conformance aligns to test-plan §3 binding schema (field presence, format, cardinality)
- **obs ↔ security** (per obs-plan §8 + scope: redaction boundary): Gate validates redaction boundary on **`agent-latest.jsonl`** (Conductor's own artifact, boundary applies) — no leaked absolute file paths, no internal struct names (minus allowlisted `target` module identity field)

## Acceptance criteria contributions

- (obs) `agent-latest.jsonl` produced in CI via agent-mode run (flag OR `CONDUCTOR_AGENT_MODE` env) without live Pulse; Blocked preflight still emits JSONL self-obs lines.
- (obs) Conformance gate FAILS on: missing required self-obs base field (timestamp_ms, level, target, service.{name,version,environment}, run_id); leaked absolute host path (drive-letter `X:\` / `/home` / `/Users` / `%APPDATA%` / `~/.cargo` / `.rustup`); internal struct-name leak (non-allowlisted Debug-dumped field). Allowlisted `target` module path (e.g. `conductor_core::obs`) PASSES, not flagged as leak.
- (obs) Zero-unlogged-panics gate FAILS on any `^thread.*panicked` line in stderr/JSONL; PASSES when only formatted `tracing::error!(panic=…)` JSON events present.
- (obs) `agent-latest.jsonl` uploaded as CI artifact with `if: always()` + 14-day retention per obs-plan §9 artifact table.

## Relevant amendment history

- **2026-06-15-structured-logging-stack** (§3 log format clarification): Established that obs-plan §3 schema block describes Run-report envelope only; self-obs base lines carry smaller set (`timestamp_ms`, `level`, `target`, service identity, `run_id`) — NOT verdict/latency/state/fingerprints. **Open question from scope (carry to P4):** gate must decide which schema to assert against `agent-latest.jsonl` (expected: self-obs base set); whether gate also checks `runs/<run_id>.jsonl` envelope separately.
- **2026-06-15-log-error-boundary-redaction** (§6/§11 redaction model reconciliation): Redaction masks absolute *file* paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>`, NOT blanket `::` token redaction. Internal struct names kept out by field-name allowlist + `Display`-not-`Debug` at `anyhow` edge. Allowlisted `target` module path explicitly preserved as documented identity field — scope confirms gate does NOT flag `module::` prefix as struct-name leak (obs-plan §9 line 532 conformance text amended).
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3 agent-mode flag reworded): Agent mode is read-only trigger: flag OR `CONDUCTOR_AGENT_MODE` env (Conductor never WRITES it). Forces JSON-only file sink `logs/agent-latest.jsonl`, no pretty-print. Scope notes `scripts/agent-run.{sh,ps1}` may invoke existing `run`/`logs` verb (confirm in implementation research).