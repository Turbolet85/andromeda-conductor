# arch extract

## Relevance
Relevant — touches workspace structure, module boundaries, standard contracts, occupied resources, infrastructure patterns, determinism discipline.

## Constraints
- Per §Design Philosophy: determinism under seed (`same scenario + seed ⇒ same emission-stream shape`; current_thread + ChaCha8Rng); headless-drivable core (emit + read-back paths both callable from CLI and Tauri); compiler-enforced module seams (crate-per-seam, forbidden cross-seam deps won't compile).
- Per §Established Decisions [OTLP Emission Strategy]: raw opentelemetry-proto types, tonic 0.14.6, byte-level fault control — no SDK exporter.
- Per §Established Decisions [MCP Read-Back Client]: hand-rolled line-delimited JSON-RPC (rmcp removed 2026-06-27); `initialize`/`tools/list`/`tools/call` returning raw `serde_json::Value`; version negotiation reads `initialize` result's `protocolVersion`, pinned `2024-11-05`; preflight gate asserts protocol + tool presence + canary round-trip; blocked state on mismatch.
- Per §Established Decisions [Verdict/Error Wall]: outcomes are typed values (`Verdict` / `ReportState`); `Result::Err` reserved for harness failures only; MCP/transport errors → typed verdict, never panics.
- Per §Established Decisions [Database] + §Inherited Defaults: rusqlite 0.38.0 + `bundled` SQLite 3.50.4, synchronous raw SQL, append-mostly `runs.db` off the async runtime.

## Patterns to follow
- Hand-rolled JSON-RPC client pattern: `initialize` preflight, pinned protocol version from the result, raw-value tool calls (established in conductor-verify per 2026-06-27-mcp-read-back-result-shape-adapter).
- Canary/readiness gate pattern: round-trip a known/unique marker through the corpus (emit → ingest → read-back → fidelity assertion), typed outcomes on failure modes (version mismatch / missing tool / stale-corpus false positive / keychain fault) → distinct `Blocked` precondition, never silent downgrade.
- Module composition: conductor-run as the run root library (preflight + readiness + execute_scenario + persist + live-counter drive_run) shared by both conductor-cli + conductor-tauri bins — composes conductor-emit + conductor-verify without adding new cross-seam edges.
- Spawn pattern with env propagation: hardened child process spawn (no argv injection), `.env(ANDROMEDA_PULSE_DATA_DIR)` passed to sidecar so it reads the live Pulse's corpus — the wiring canary validates.

## Anti-patterns to avoid
- Never panic on transport/MCP/canary transport/decode faults — return typed `Blocked` / `Fail` verdict via the verdict/error wall.
- No new inbound listener from Conductor (gRPC/MCP are read-back ingress, `:4317` is OTLP egress only; port-occupier fault is the sole deliberate exception, bound only for that scenario).
- No scenario without a P-ID (scope law applies — the 5 existing families [error-baseline-spike, fingerprint-storm, restart-suppression, pii-scrub, connection-lifecycle] may refine SLO blocks but add no new scenario).

## Contract bindings
- **Observation ↔ Tests harness** — self-obs JSON sink (`logs/agent-latest.jsonl`, sanitized stderr) + the readiness-gate checked_at stamp; test-harness captures preflight success as a blocking precondition (live Pulse feature gate).
- **Security ↔ Architecture** — sidecar spawn hardening (no argv injection), env-var .env(...) propagation rules, no secrets in the canary payload.
- **Design/A11y ↔ Architecture** — report lamp rendered verdict-first (per 2026-06-21 Verdict→ReportState mapping); Blocked rows populate identity fields only (null verdicts/latencies).

## Acceptance criteria contributions
- (arch) Code lives in conductor-verify / conductor-run / conductor-emit / conductor-report / conductor-cli per workspace crate-per-seam boundary rules (§Inherited Defaults).
- (arch) Readiness gate uses hand-rolled line-delimited JSON-RPC client, pinned protocol `2024-11-05` negotiated from `initialize` result (§Established Decisions [MCP Read-Back Client]).
- (arch) Canary round-trip uses fingerprint-storm path — emit via conductor-emit `fingerprint()` primitive, read-back via `retrieve_telemetry_slice` `fingerprint_refs` field (no title echo; titles are Pulse-scrubbed) — proving data-dir/corpus wiring end-to-end.
- (arch) Sidecar spawn propagates `ANDROMEDA_PULSE_DATA_DIR` via `.env(...)` hardened pattern; no new env-var beyond `CONDUCTOR_*` namespace (§Occupied Resources).
- (arch) Blocked precondition (version mismatch / missing tool / empty-canary / keychain failure) classified as ReportState::Blocked, never false pass or panic (§Verdict/Error Wall).
- (arch) Per-run JSONL journal + runs.db row + Markdown report persist with timestamp formats RFC-3339 text + integer-ms latency (§Standard Contracts timestamp formats, §Data model conventions).

## Relevant amendment history
- **2026-06-27-mcp-read-back-result-shape-adapter** — rmcp removed → hand-rolled JSON-RPC (§Established Decisions [MCP Read-Back Client] reversed). The prior decision was "rmcp 1.7.0 official SDK"; discovered at live implementation that Pulse's hand-rolled server is non-MCP-compliant for `tools/call` (returns raw payload, no `{content:[…]}` envelope) — rmcp rejects as `UnexpectedResponse` on every call. Hand-rolling is the faithful match; version negotiation reduces to reading `initialize` result's `protocolVersion`. Blocking prerequisite for this chunk; scope was rolled back on first attempt, closed by this adapter (2026-06-27).
- **2026-06-26-live-counter-channel-stream** — conductor-run (9th workspace member) registered as composition root library (preflight + readiness + execute_scenario + persist + drive_run), shared by conductor-cli + conductor-tauri. This chunk's preflight/readiness/execute/persist path sits in conductor-run.
- **2026-06-21-run-report-envelope-serializer** — ManualCheck broadened to include auto-measured model-interpretive (calibration-region) checks; Verdict→ReportState default mapping (`Pass→Pass`/`Fail→Fail`/`CalibrationRegion→ManualCheck`) + verdict-first lamp precedence. This chunk uses the verdict/state model to classify canary outcomes.
- **2026-06-21-runs-db-index** — runs.db instant columns stored as TEXT RFC-3339 (JSONL wire form); integer-ms SLO-math value lives in separate `latency_ms` INTEGER column. Blocked rows populate identity fields only (null verdict/latency/timestamps). This chunk persists canary outcomes into runs.db.
