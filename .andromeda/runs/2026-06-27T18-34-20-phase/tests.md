# tests extract

## Relevance
relevant — this chunk is the live-Pulse E2E proof, running CLI headless scenarios against a real Pulse instance with MCP read-back verification.

## Constraints
1. §1 Scope Summary: Minimal tier (0) with "production-grade verification rigor" + determinism hard-bar + assertion-policy split enforces all test work
2. §1 Critical Paths 1/2/3 + extension: error-baseline-spike / fingerprint-storm / restart-suppression / pii-scrub / connection-lifecycle must run headless + verify Pulse reactions within SLO via MCP read-back
3. §3 5-command discipline: boot=preflight gate with MCP canary round-trip (exit 0 iff ready:true); run=invoke scenarios (exit 0 iff all Pass); status=query runs.db; cleanup=idempotent teardown; logs=JSONL emission journal
4. §1 Surfaces: cli subprocess (exit-code + `[PASS]`/`[FAIL]`/`[BLOCKED]` labels) + ipc-internal MCP read-back (JSON tool responses) are verification signals
5. §1 Untestable zones: live Pulse corpus.db is untestable from Conductor (encrypted by Pulse) — verify via MCP read-back only, never direct corpus inspection
6. §1 Coverage triggers (contract-test): MCP manifest `2024-11-05` protocol + required-tool presence; mismatch ⇒ Blocked
7. §9 CI Integration: live-Pulse scenarios are **operator/local gate via `workflow_dispatch`, NOT a CI gate** — CI runs rmcp-stub legs only

## Patterns to follow
1. §3 5-command implementation: boot preflight with MCP `initialize` + canary round-trip (emit→ingest→query_incident_list→content-fidelity); run scenarios with deterministic `--seed`; status reads runs.db bound-parameter; cleanup removes artifacts + releases `:4317` port-occupier; logs tails JSONL
2. §6 E2E scenarios + §1 Test Harness Requirements: assert_cmd subprocess with stdin closed + exit code 0 = all Pass + runs.db row tracks verdict/state/latency_ms + JSONL written + MCP read-back confirms Pulse reaction; same seed ⇒ same stream shape re-run
3. §1 Test Data Strategy: deterministic seeded synthetic generation (conductor-timeline); no developer-seeded data
4. §3 Status Endpoint: Run-report envelope (run_id, seed, scenario, verdict ∈ {Pass,Fail,CalibrationRegion}, state ∈ {Pass,Fail,ManualCheck,KnownResidual,Blocked}, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints)

## Anti-patterns to avoid
1. §11 E2E: NEVER treat Blocked/ManualCheck/KnownResidual/CalibrationRegion as non-zero exit — those are envelope states, not hard Fail exits (Creator Brief assertion-policy split)
2. §11 Mocking: NEVER fake Pulse's reaction as a CI verdict (rmcp stub canary proves MCP wiring only); NEVER interpolate ANDROMEDA_PULSE_DATA_DIR into argv/shell — use `.env(...)` only; fixed sidecar path, no operator-chosen path
3. §11 Test Data: NEVER stamp emission journal from tokio's virtual clock — use `std::time::SystemTime`/`Instant`; paused-clock leak into SLO artifacts corrupts ground truth and must be caught by golden test

## Contract bindings
- obs §3 (log-format) ↔ tests §3: JSONL schema definition (per-line envelope); self-obs stream distinct from emission journal per amendment 2026-06-15-structured-logging-stack
- obs §3 ↔ tests §3: 5-command discipline gates the harness (boot/run/status/cleanup/logs)
- obs §Status endpoint ↔ tests §3: Run-report envelope shape binds readiness fields + verdict/state + latency + timestamps

## Acceptance criteria contributions
- (tests) `conductor preflight` against live Pulse → exit 0 with stdout JSON `{"ready": true}` + MCP canary round-trip completed (was Blocked; scope core deliverable)
- (tests) Each of error-baseline-spike / fingerprint-storm / restart-suppression / pii-scrub / connection-lifecycle runs against live Pulse with non-Blocked verdict state + JSONL + runs.db row
- (tests) Same scenario+seed ⇒ identical JSONL stream shape across re-runs (determinism invariant per seeded conductor-timeline)
- (tests) Evidence artifacts sanitized (no host paths / struct names); enforcement via golden test per §5 Integration golden pattern

## Relevant amendment history
- 2026-06-26-live-counter-channel-stream: deferred Tauri-integration + GUI tests to Epoch-9 tauri-driver chunk; this chunk is CLI headless only; run logic covered at conductor-run unit tier + conductor-cli E2E parity CLI leg — scope boundary clarity
- 2026-06-15-structured-logging-stack: self-obs stream (logs/agent-latest.jsonl) is distinct from emission journal (runs/<run_id>.jsonl) — two separate schemas not to be conflated, relevant to log-harness binding
- 2026-06-16-emission-journal-writer: unit serialization goldens use exact-assert; insta is the E2E journal golden mechanism with run_id/timestamp redaction — affects how this chunk's E2E golden-locks JSONL
