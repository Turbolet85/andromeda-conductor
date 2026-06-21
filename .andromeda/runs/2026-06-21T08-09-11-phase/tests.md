# tests extract

## Relevance
Partial — the chunk is infrastructure without a P-ID; integration + component testing applies; E2E paths + preflight gate deferred.

## Constraints
- Per test-plan §1 Coverage Summary: `conductor-verify` (MCP read-back client) is partially-testable; stub-server leg (rmcp in-process over stdio) is CI-runnable; real Pulse sidecar leg is local-gate-only (test-plan §1, entity `conductor-verify`)
- Per test-plan §2 Test Strategy: Unit covers isolated module logic + verdict mapping against rmcp stub; Integration covers rmcp stub stdio + error mapping (test-plan §2 Unit and Integration rows)
- Per test-plan §3 Test Harness Contract: `boot` command runs preflight via rmcp stub in-process (CI leg); live Pulse leg gated on `ANDROMEDA_PULSE_MCP_ENABLED` (test-plan §3 `boot` impl)
- Per test-plan §5 Integration Test Strategy: rmcp in-process/stdio stub server for MCP session testing (see Section 5 detail below)
- Per test-plan §1 Critical Path 3: MCP read-back negotiation + tool-surface availability asserted via the stub-server contract (protocol `2024-11-05`, required-tool set matching pinned manifest)
- Per test-plan §8 Coverage Trigger (contract-test): preflight must assert protocol negotiates DOWN to `2024-11-05` (not strict-newer) and required-tool set matches pinned `contracts/` manifest; mismatch ⇒ `blocked` (test-plan §1 Coverage Triggers)

## Patterns to follow
- thiserror typed `VerifyError` enum for rmcp/transport/spawn faults (no panics); `Result::Err` reserved for harness faults (scope §Verdict/error wall)
- Fixture: rstest `#[fixture]` for rmcp stub-server spawn over `TokioChildProcess` stdio (seeded deterministic, per test-plan §3 test-data-bootstrap)
- Contract test: assert negotiated protocol == `2024-11-05` (never strict-newer, per test-plan §1 Coverage Triggers, contract-test pattern); required-tool set vs. pinned manifest (agent-parseable contract assertion)
- Table-driven: rstest `#[case]` rows for valid/invalid metacharacter rejection on `ANDROMEDA_PULSE_DATA_DIR` (security Vector 4, negative-test pattern per test-plan §1 Coverage Triggers)

## Anti-patterns to avoid
- String concatenation / argument interpolation for sidecar spawn paths or `ANDROMEDA_PULSE_DATA_DIR`; only hard-coded paths + `.env(...)` builder (CVE-2026-30623, security Vector 4, per scope §Security invariants)
- Panics on malformed child stdout; bounded prost recursion + typed `VerifyError` mapping (test-plan §1 Coverage Triggers, Vector 4)
- Pinning rmcp client to a strict newer protocol default; always negotiate DOWN to `2024-11-05` (test-plan §1 Coverage Triggers, contract-test)

## Contract bindings
- **ipc-internal surface** (MCP read-back client) ↔ **test harness** (§3 Test Harness Contract `boot` command): rmcp stub must return negotiated protocol + required-tool manifest check; preflight gate (Epoch 5 chunk 2) consumes this surface
- **VerifyError wall** ↔ **verdict domain** (later chunks): typed errors become verdict state (e.g., `degraded_mode` ⇒ `KnownResidual`, empty canary ⇒ `Blocked`, transport error ⇒ harness fault) — verdict chunk maps these
- **Log format** (test-plan §3): if transport/spawn errors are logged, they must use the `tracing` JSONL format (per amendments 2026-06-15, self-obs stream distinct from emission journal)

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify` passes for rmcp stub spawn, protocol-negotiation, tool-list, and error-mapping unit tests (per test-plan §3)
- (tests) Metacharacter rejection on `ANDROMEDA_PULSE_DATA_DIR` via negative tests (rstest `#[case]` rows) (per test-plan §1 Coverage Triggers, negative-test)
- (tests) Contract test asserts negotiated protocol == `2024-11-05` and required-tool set matches pinned `contracts/` manifest (per test-plan §8 contract-test)
- (tests) VerifyError enum covers spawn, transport, protocol, and tool-call failure cases; no panics on malformed protobuf (per test-plan §1 Coverage Triggers, Vector 4)

## Relevant amendment history
- 2026-06-16-test-framework-fixtures-coverage-tooling: external-CLI tool versions (cargo-nextest 0.9.137, cargo-llvm-cov 0.8.7) are reference floors, not exact pins; dev-deps caret-resolved with `Cargo.lock` authoritative — applies to this chunk's fixture setup (rstest/insta dev-deps)
- 2026-06-17-raw-otlp-message-scaffold: OTLP-egress loopback gRPC stub (`tokio-stream` + tonic) registered in integration mechanisms — not directly this chunk's transport, but sets integration-test altitude pattern for later read-back verification chunks
