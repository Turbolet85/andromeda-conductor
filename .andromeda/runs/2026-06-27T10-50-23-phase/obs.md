# obs extract

## Relevance
Relevant — chunk lands tests exercising critical paths (Path 7 cross-surface parity); test harness itself uses observability stack (logging, no OTel SDK) and produces artifacts subject to obs schema conformance.

## Constraints
1. **§3 OTel SDK init:** No OTel SDK initialization anywhere in tauri-driver, tauri::test mock-runtime, or axe/Lighthouse harness — preserves `current_thread` determinism invariant
2. **§3 Logging stack:** Test code (especially Tauri command handlers) logs via `tracing` + `tracing-subscriber` JSON; dual sink (stderr pretty-print + file in agent mode)
3. **§4 Path 7 (cross-surface parity):** Tests must exercise Tauri mock-runtime vs CLI subprocess with identical seed → identical `runs.db` envelope (verdict/state) and 11-field JSONL schema
4. **§6 Log format schema:** Every log line carries `service.name`, `service.version`, `deployment.environment`; Run-report envelope carries 11 fields (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints)
5. **§9 CI gates:** `cargo nextest` JSON + `logs/agent-latest.jsonl` artifact upload; zero-unlogged-panics validation (no `^thread.*panicked` unstructured backtraces)
6. **§2 Agent-readable invariant:** All telemetry from tested code must be machine-parseable via jq/serde_json; no human-only dashboards; no absolute host paths or internal struct names in artifacts

## Patterns to follow
1. **Span naming (§4):** Code under test emits `{module}.{operation}` spans; test assertions on Path 7 must correlate by `run_id` (not W3C `traceparent`) — both surfaces tag logs with their respective run_id, matched by (scenario, seed)
2. **Test coverage for critical paths (§4):** Path 7 deferred-test batch exercises Tauri command handlers (`start_run`/`stop_run`/`run_report`/`resolve_operator_hold`) + `Channel` frames; assert runs.db envelope parity across surfaces
3. **Log schema binding (§6 + amendment 2026-06-16):** Run-report envelope now carries `read_back_observed_at` (11 fields); tests checking runs.db or journal must validate this schema (not 10-field)
4. **Redaction model (amendment 2026-06-15):** Test artifacts scrub absolute host-file paths (`X:\`, `/home`, `~/.cargo`) → `<redacted>`; internal struct names guarded by allowlist + Display-not-Debug at error boundaries

## Anti-patterns to avoid
1. **No OTel SDK (§3, amendment 2026-06-17):** Tests must never call `opentelemetry_sdk::trace::TracerProvider::new()` or similar; opentelemetry-proto may be a transitive dep (dormant is OK), but init is a violation
2. **No unbounded span names:** Test names/parameterization must not generate span names (e.g., per-test-ID spans); if tests emit spans, use bounded names following `{module}.{operation}` pattern
3. **No host paths in logs:** Tests that capture or replay logs must not output `%APPDATA%`, `~/.cargo`, `/Users`, absolute paths; CI conformance gate rejects these

## Contract bindings
- **Tests harness (obs ↔ tests plan):** Tests consume structured JSONL schema (§3) + `runs.db` envelope (§6); Path 7 parity assertion (test-plan §3, obs-plan §4) is a binding contract — identical (scenario, seed) → identical (verdict, state) across surfaces, both with 11-field envelope
- **A11y violation JSON (obs ↔ a11y plan):** axe/Lighthouse violations (Epoch 10, post-chunk) bind to obs envelope schema for CI gate; this chunk's harness setup must not break the logging surface a11y violations will use

## Acceptance criteria contributions
1. "(obs) Path 7 cross-surface-parity test exists, exercise Tauri mock-runtime ↔ CLI subprocess with identical seed, asserts identical `runs.db` envelope (verdict/state) — verifies 11-field schema (§6 + amendment 2026-06-16)."
2. "(obs) No unstructured panics in test runs; any panic captured via `std::panic::set_hook()` and logged as structured `tracing::error!(panic=…)` JSON (§7, §9 zero-unlogged-panics gate)."
3. "(obs) Test artifacts (`logs/agent-latest.jsonl` from test harness runs, if uploaded to CI) conform to log schema: no absolute host paths, no internal struct names; CI conformance gate validates presence of journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints (§6, §9)."
4. "(obs) Tauri test harness (tauri-driver, axe, Lighthouse, colorjs.io) does not initialize OTel SDK; uses `tracing`-only self-observation (§3, amendment 2026-06-17)."

## Relevant amendment history
- **2026-06-16-emission-journal-writer:** Run-report envelope schema updated to 11 fields; `read_back_observed_at` added (ISO-8601, null until read-back). Tests checking runs.db or emission journal (especially Path 7 parity) must account for this change; the envelope is now schema-aligned with test-plan owner.
- **2026-06-15-log-error-boundary-redaction:** Redaction model clarified — absolute host-file paths masked (drive letters, `/home`, `~/.cargo`) → `<redacted>`; internal struct names guarded by field-name allowlist + Display-not-Debug at error boundaries. Tests must not leak absolute paths; allowed `target` module paths are explicitly preserved.
- **2026-06-24-sanitized-stderr-agent-mode-logging:** `CONDUCTOR_AGENT_MODE` env var is a read-only trigger Conductor reads (`agent_mode = --agent-mode flag || env-set`); tests running in agent mode may see `CONDUCTOR_AGENT_MODE=1` set by the harness/operator.