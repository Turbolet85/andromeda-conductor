# obs extract

## Relevance
Relevant — chunk 2 of Epoch 5 (Verification & read-back) adds the preflight readiness gate and `ReadyState` within conductor-verify, which surfaces telemetry boundaries and state tracking integral to the verification harness.

## Constraints
1. Per obs-plan §1: Conductor is Minimal tier (0); self-observation is **structured tracing logs only** (JSONL via `tracing-subscriber`), never OTel SDK (determinism constraint on `current_thread` runtime).
2. Per obs-plan §3: `service.identity` (name/version/environment) must be wired on every log line; `ReadyState` serialization is a value type, not yet persisted to `runs.db` (that is Epoch 6).
3. Per obs-plan §4 Critical Paths 1 & 5: the `verify.readback` span (MCP call) and `verify.readback_degraded_mode` span (with `degraded_mode=true` flag) are must-trace operations; required attributes are `mcp_method`, `latency_ms`, and for known-residual path, `degraded_mode_response`.
4. Per obs-plan §6: log levels — `info` for boundary-call summaries (readback completion, method name, latency, error if any), `warn` for recoverable errors (MCP timeout, degraded-mode response).
5. Per obs-plan §3 amendment 2026-06-15: redaction model (host-file-path scrub only, NOT `::`-token redaction; allowlisted `target` preserved); `ReadyState` carries no PII.
6. Per obs-plan §9 CI Integration: preflight logs must conform to binding schema (Section 6); `Blocked` precondition strings are logged as part of `state` field (enum: "Blocked").

## Patterns to follow
1. Per obs-plan §3 / §4: span nesting — `readback_*` spans are children of the gate-level operation (not `scenario.run` — preflight runs before any scenario); close on MCP response receipt.
2. Per obs-plan §6: JSONL log-line base set (every line carries `run_id`, `seed`, `scenario`, `journal_emitted_at`, service-identity); the preflight gate may emit `run_id = "preflight-<timestamp>"` or null (clarify in P4) — do not carry scenario-specific fields (verdict/state/latency) until scenario execution begins.
3. Per obs-plan §3 amendment 2026-06-16: envelope includes `read_back_observed_at` (ISO-8601 from `std::time`, null until verified); preflight logs NULL this field (no scenario latency yet).
4. Per obs-plan §1: all output is agent-readable (via `jq`); `ReadyState` JSON must be parseable as a struct with `ready` (boolean), `negotiated_protocol_version`, `expected_protocol_version`, `required_tools` map, `data_dir`, `canary_round_trip`, `blocked_precondition`, `checked_at` fields.

## Anti-patterns to avoid
1. Per obs-plan §11 (referenced by §6 CI gate): do NOT emit absolute host-file paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`), internal struct names via Debug-dumping, or `::` token redaction; use `Display` at error boundaries + allowlist field names only.
2. Per obs-plan §3: do NOT initialize OTel SDK (no `opentelemetry_sdk::trace::TracerProvider`, no batch tasks); `tracing` spans only — no `trace_id`/`traceparent` propagation.
3. Per obs-plan §4: do NOT use high-cardinality span attributes (e.g., per-tool-name or per-manifest-field variants); bounded span-name set only (`verify.readback`, `verify.readback_degraded_mode`).

## Contract bindings
**obs ↔ tests harness:** Per obs-plan §3 amendment 2026-06-15 (D-tests-obs-harness escalation), `ReadyState` JSON shape must align to tests' status-shape binding (Section §Standard Contracts); tests poll `ready` / `blocked_precondition` fields and assert gate success before scenario runs.

## Acceptance criteria contributions
1. (obs) Preflight-gate logs conform to base JSONL line schema (§6: `run_id`, `seed` [null if pre-scenario], `journal_emitted_at`, service.name/version/environment, `level`; no absolute paths / struct names).
2. (obs) MCP preflight spans (`verify.readback` x3: protocol check, tool presence, canary round-trip) instrumented with `#[tracing::instrument]` + `mcp_method`, `latency_ms`, error fields; close on response receipt.
3. (obs) `ReadyState` struct serializes to JSON (via `serde`) matching the architecture's readiness envelope (ready, negotiated_protocol_version, expected_protocol_version, required_tools map, data_dir, canary_round_trip, blocked_precondition, checked_at).
4. (obs) `Blocked` precondition strings (3: protocol mismatch, missing tool, failed canary) are logged at `info` level with `blocked_precondition` field + human-readable reason; CI gate asserts no unlogged `Blocked` exits.

## Relevant amendment history
1. **2026-06-15-structured-logging-stack:** self-obs base line clarified (timestamp_ms / level / target / service-identity / run_id on every line, envelope fields only on scenario-result). Preflight gate logs carry the base set only (not verdict/state/latency yet).
2. **2026-06-15-log-error-boundary-redaction:** redaction model anchored to host-file paths (not `::`-token redaction); `target` module field preserved. Preflight gate must scrub any MCP error messages of drive-letter paths / credential artifacts.
3. **2026-06-16-emission-journal-writer:** `read_back_observed_at` field added to envelope schema. Preflight logs NULL this field (no scenario timing); the `ReadyState` value type need not carry it (verification step precedes scenario execution — latency is not a readiness gate).
