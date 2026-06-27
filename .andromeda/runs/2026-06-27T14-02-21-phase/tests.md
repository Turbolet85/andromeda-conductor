# tests extract

## Relevance
relevant — CI conformance gate directly implements test-plan §3 / §9 / §10 log-schema and zero-panic gate bindings.

## Constraints
1. Log format binding per test-plan §3: JSONL only, one JSON object per line, never multi-line (test-plan §3 "Never multi-line — `tracing` JSON flattening serializes everything onto one line").
2. Required self-obs base fields on every line per test-plan §3 amendment 2026-06-15: `timestamp_ms`, `level`, `target`, `service.name`/`service.version`/`deployment.environment`, `run_id` (distinct from the run-report envelope fields; self-obs stream schema).
3. Redaction boundary per test-plan §3: no absolute host paths (drive letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`) or internal struct names in any field; allowlisted `target` module paths preserved as identity fields (test-plan §3 "Artifacts MUST NOT leak").
4. CI gating policy per test-plan §10: machine-parseable output, non-zero exit on violation, zero silent passes on schema break — no human review.
5. Zero-unlogged-panics enforcement per obs-plan §10: only formatted `tracing::error!(panic=…)` JSON events acceptable; any unstructured `^thread.*panicked` backtrace fails the gate.
6. Agent-runnable invariant per test-plan §2: every gate layer produces exit codes (`jq -e` / `grep` return code) parseable by the CI harness; never dependent on visual inspection or manual step.

## Patterns to follow
1. Test-harness gate precedent from test-plan §3: the conformance check mirrors the `run` command's JUnit-XML machine-parsing pattern — serde_json/jq-parseable, CI-integrated, field-by-field assertion.
2. Append-mostly artifact collection per test-plan §3 Test data bootstrap: `logs/agent-latest.jsonl` accumulates across invocations (per-invocation append), conformance validates the accumulated file in one pass.
3. Negative-test discipline per test-plan §1 Coverage triggers (Vector 3 + artifact-sanitization discipline): the redaction boundary is proven by asserting leaked paths/struct-names are explicitly rejected, not merely absent.
4. Dual-sink awareness per test-plan §3 amendment 2026-06-24: both `logs/agent-latest.jsonl` (cli) and `logs/conductor-tauri.jsonl` (Tauri backend) carry the same base-field schema and fall under this gate's scope.

## Anti-patterns to avoid
1. Confusing self-obs stream schema with run-report envelope schema (test-plan §3 amendment 2026-06-15: "the two schemas must not be conflated" — self-obs carries base fields only; envelope fields live only in `runs/<run_id>.jsonl`).
2. Attempting to validate envelope fields (`verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) on every line of `agent-latest.jsonl` — those belong in `runs/<run_id>.jsonl` envelope, not self-obs stream, per test-plan §3.
3. Allowing silent schema passes or warnings; any missing required field, leaked path, or unlogged panic must exit non-zero (test-plan §10 "gating policy: all gates must gate the build, zero silent passes").

## Contract bindings
- **obs-plan §3** (self-obs stream sink location + base-field schema)
- **obs-plan §6** (log coverage / required field attestation on every line)
- **obs-plan §9** (CI log-conformance-gate spec + zero-panics-gate spec)
- **obs-plan §10** (zero-unlogged-panics SLO, always-required)
- **test-plan §3 (Log format, Test Harness Contract)** — owns the JSONL/field schema binding; obs derives from it (test-plan §3 bootstrap-phases note: "the harness greps the file. format break = harness break").
- **test-plan §9 (CI Integration)** — gate-pipeline conventions (§10 references this section as sibling to coverage/flakiness gates).
- **security-plan §Error Handling** — redaction boundary for Conductor's own artifacts (host paths + struct names).

## Acceptance criteria contributions
1. "(tests) CI conformance gate accepts `logs/agent-latest.jsonl` and validates every line carries required self-obs base fields (`timestamp_ms`, `level`, `target`, `service.name`/`service.version`/`deployment.environment`, `run_id`) per test-plan §3."
2. "(tests) Conformance gate FAILS on missing required field, leaked absolute host path, or internal struct-name leak; PASSES on allowlisted module-path `target` identity field."
3. "(tests) Zero-unlogged-panics gate FAILS on any `^thread.*panicked` line in logs or stderr; PASSES when only formatted `tracing` panic JSON present per obs-plan §10."
4. "(tests) `logs/agent-latest.jsonl` artifact uploaded as CI output via `actions/upload-artifact@v4` with `if: always()` on Windows runner; no live Pulse required."

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (test-plan §3 / Log format): Clarified that self-obs stream (`logs/agent-latest.jsonl` + `logs/conductor-tauri.jsonl`) and run-report emission journal (`runs/<run_id>.jsonl`) are SEPARATE artifacts with distinct field schemas. Self-obs carries base fields (timestamp_ms, level, target, service.*, run_id); envelope carries SLO fields (verdict, state, latency_ms, fingerprints). This amendment is critical for THIS chunk: the conformance gate asserts the self-obs base-field schema ONLY; the envelope schema is a separate test-plan §10 gate (not in scope for this chunk).
- **2026-06-24-frameless-window-shell** (test-plan §3 / Test Harness Contract): Registered `logs/conductor-tauri.jsonl` as a second self-obs sink (CLI and Tauri backend now both produce structured logs). Both sinks carry the same base-field schema. This chunk's conformance gate may validate both sinks or designate `agent-latest.jsonl` as primary per obs-plan §9 scope clarification.