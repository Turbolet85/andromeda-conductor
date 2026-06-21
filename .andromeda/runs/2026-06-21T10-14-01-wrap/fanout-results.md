# Fan-out drift-detection results — 2026-06-21-preflight-readiness-gate wrap

7 doc-agents, one per spec source. Stripped proposal lists below.

## arch
```yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "Established Decisions — [MCP Read-Back Client]"
    change: "rmcp 1.7.0 (`client` feature for production; test fixtures may use `server` feature in feature-gated bins never compiled into default/release builds)."
    sidecar: "Clarified rmcp server-feature scope: test-only stub servers exempt from the non-server production posture via feature gating."
    rationale: "Report: `stub-server` feature-gated bin `stub_pulse_mcp` pulls `rmcp/server` for mocking Pulse's MCP server in tests; arch narrows rmcp to client-only (Pulse's server is hand-rolled JSON-RPC)."
```
(D-arch-resources: clean — `contracts/` artifact + `CONDUCTOR_CONTRACT_MANIFEST` + `conductor-verify` crate already registered in §Occupied Resources; new public API symbols are detail arch omits.)

## security-plan
```yaml
proposals: []
```
D-security-input clean (manifest bounds-checked; path-canon is the Epoch-8 cli edge), D-security-subprocess clean (no spawn added; chunk-1 hardening unchanged; negotiate-down asserted), D-security-deps clean (serde/toml already workspace deps, audit+deny green, lock un-drifted).

## design-system
```yaml
proposals: []
```
No UI rendered — backend-only chunk; all surfaces `tokens n/a`.

## layout-templates
```yaml
proposals: []
```
The preflight readiness line (`preflight  protocol 2024-11-05  tools 4/4  canary ok`) is ALREADY documented in layout-templates §Output structure for `conductor run` + `conductor suite` (ANSI 117, `[OK]` prefix). No undocumented surface.

## test-plan
```yaml
proposals: []
```
cargo-nextest + rstest on-spec; new paths tested at the mandated tier; harness/envelope unchanged (the `conductor preflight` verb is Epoch 8).

## obs-plan
```yaml
proposals: []
```
D-obs-instrumentation clean (bounded `verify.readback*` spans, run_id, std::time `checked_at`), D-obs-stack clean (no OTel SDK; tracing-JSON only), D-obs-redaction clean (`data_dir` redacted; precondition strings redacted; `blocked_precondition` via redacted message + allowlisted `state` — no leak).

## a11y-plan
```yaml
proposals: []
```
No interactive UI element; no a11y violation-schema change.
