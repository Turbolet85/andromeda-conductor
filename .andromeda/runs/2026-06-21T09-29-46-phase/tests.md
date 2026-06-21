# tests extract

## Relevance
Partial — the chunk implements the preflight readiness gate (a critical boundary before dependent scenarios); test coverage is bounded to the gate's 3 assertions + its integration boundary.

## Constraints
- per §3 5-command implementation: `boot` is a preflight readiness gate running `cargo run -p conductor-cli --bin conductor -- preflight --json`, asserting MCP `initialize` handshake + required-tool presence + canary round-trip, returning `{"ready": true|false}` JSON with distinct `Blocked` precondition strings on failure (never silent downgrade).
- per §5 Integration Test Strategy: the preflight gate runs against an rmcp 1.7.0 stub MCP server over stdio (in-process duplex for verdict logic; `TokioChildProcess` test-binary for the spawn/`.env()` injection-rejection path); CI leg fully runnable via stub; live read-back leg gated on `ANDROMEDA_PULSE_MCP_ENABLED` (local-gate-only, not CI).
- per §5 Cross-module patterns: contract test asserting the preflight gate negotiates DOWN to `2024-11-05` (NOT strict-newer), required-tool presence vs. pinned `contracts/` manifest, and canary round-trip; `ready:false` ⇒ `blocked` (test-scope Sec 5 contract-test trigger + security Vector 4).
- per §1 Test Scope: `conductor-verify` verdict logic + preflight state mapping (`ready:false` ⇒ `blocked`, `degraded_mode` ⇒ `KnownResidual`, empty canary ⇒ `blocked`) assertable against an rmcp in-process stub; the real read-back path needs the spawned `andromeda-pulse-mcp` sidecar + `mcp-server` feature (integration leg, local-gate-only).
- per §3 Test Harness Contract: preflight integrity — never silently downgrade a failed preflight; transport/MCP errors → typed `Blocked`/harness `Err`, never panic; fixed-path spawn + `.env()` data-dir (security Vector 4(a)).
- per §11 Test Anti-Patterns § Integration: use rmcp stub server over stdio (in-process duplex + `TokioChildProcess` test-binary); do NOT monkey-patch `andromeda-pulse-mcp`; never interpolate `ANDROMEDA_PULSE_DATA_DIR` into argv, pass strictly via `.env(...)` after rejecting injection metacharacters (CVE-2026-30623 rmcp STDIO injection).

## Patterns to follow
- per §5 Integration boundary mechanisms: rmcp 1.7.0 stub MCP server for unit/integration tests (in-process duplex for verdict logic; `TokioChildProcess` test-binary for the spawn path with `.env()` injection seam).
- per §4 Unit Test Strategy: conductor-verify verdict logic + preflight state mapping tested in isolation against an rmcp in-process stub.
- per §5 Cross-module patterns: contract test of required-tool-set match vs. pinned `contracts/` manifest + canary round-trip non-empty assertion (security Vector 4(c)).

## Anti-patterns to avoid
- NEVER fake Pulse's reaction as a CI verdict — the rmcp stub canary proves MCP wiring only; live Pulse behavior is local-gate-only.
- NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` into sidecar argv — pass strictly via `.env(...)` after rejecting injection metacharacters; spawn as a fixed hard-coded path.
- NEVER mock what you don't own without an abstraction layer — wrap the live Pulse behind the rmcp transport; do NOT monkey-patch `andromeda-pulse-mcp`.

## Contract bindings
obs ↔ tests harness (per §3 Log format amendment 2026-06-15: the `tracing` self-observation stream is SEPARATE from the per-run emission journal; both carry `run_id` identity fields but separate schemas — the harness owns the Run-report envelope JSONL format, obs derives its self-obs stream separately).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify` passes for the 3 preflight assertions (protocol version pin, required-tool presence, canary round-trip).
- (tests) Contract test asserts negotiated protocol == `2024-11-05` and required-tool set matches pinned `contracts/` manifest; any mismatch ⇒ `blocked` state (never silent downgrade).
- (tests) Canary round-trip non-empty assertion (empty/missing canary ⇒ `blocked`); security Vector 4(c) negative test.
- (tests) Security Vector 4(a) negative test: `ANDROMEDA_PULSE_DATA_DIR` with injection metacharacters is rejected, passed only via `.env(...)`; fixed sidecar path (not operator-chosen); bounded prost recursion does not panic on malformed child stdout.

## Relevant amendment history
(none) — no prior amendments touch the preflight-readiness-gate chunk's area.
