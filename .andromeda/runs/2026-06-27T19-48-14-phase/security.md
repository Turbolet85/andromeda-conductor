# security extract

## Relevance
Relevant — this chunk adapts the MCP read-back child-process boundary to parse raw JSON-RPC results instead of rmcp's typed `CallToolResult` envelope; the boundary itself is explicitly named in the threat model and input-validation rules.

## Constraints
1. Bound prost/protobuf decoding of MCP tool responses with no unbounded recursion on the read-back path (security-plan §Input Validation); empty canary round-trip must become `blocked`, never false pass (security-plan §Anti-Patterns § Universal).
2. MCP error responses and parsed JSON from child stdout are first-class typed verification inputs, never panic; route call/parse errors through the verdict/error wall as `Result::Err` (harness faults) or typed `Blocked` states (precondition failures), not silent downgrades (security-plan §Error Handling, §Anti-Patterns § Universal).
3. Preflight gate's canary assertion must distinguish genuine call/parse errors from empty corpus results — a failed `query_incident_list` call or parse failure surfaces as `blocked` with the true precondition string (e.g., "MCP call failed: …"), not masked as "incident not found in corpus" (security-plan §Threat Model Summary § Trust boundary, §Anti-Patterns § Universal).
4. `rmcp 1.7.0` pinning + `Cargo.lock` committed + `cargo-audit`/`cargo-deny` green must hold through the raw-JSON substitution; dependency audits pass before merge (security-plan §Dependency Security, §Bootstrap phases § dep-audit-tooling-install).
5. Run-report artifacts must not leak absolute paths or internal struct names; a `blocked` row from failed preflight populates identity fields + sanitized precondition string only; measurement fields JSON `null` (security-plan §Error Handling, §Anti-Patterns § Logging).

## Patterns to follow
1. **Typed error inputs on the read-back boundary** — JSON-RPC error responses and parse failures map to typed `VerifyError` enum variants (not raw panic), then route through verdict/error wall (security-plan §Error Handling).
2. **Canary round-trip as the trust gate** — a successful raw `query_incident_list` result containing the emitted incident proves child+corpus readiness; empty result or call error becomes `blocked` with the true precondition (security-plan §Threat Model Summary).

## Anti-patterns to avoid
1. NEVER decode raw JSON from child stdout without enforcing serde_json recursion limits; empty canary result must become `blocked`, not false pass (security-plan §Input Validation, §Anti-Patterns § Universal).
2. NEVER let a malformed or error-response JSON panic; treat JSON-RPC errors as typed precondition inputs routed through the verdict wall (security-plan §Anti-Patterns § Universal).
3. NEVER silently downgrade a failed canary (call error / parse failure / empty result) to "incident not found in corpus" — surface the true precondition as the distinct `blocked` state (security-plan §Anti-Patterns § Universal).

## Contract bindings
**Verdict/error wall ↔ error-handling cross-cut** — the canary error-masking fix must route genuine call failures through typed error states, never silent pass, preserving the verdict-state invariant (Design plan §Verdict States).

## Acceptance criteria contributions
1. (security) `cargo nextest run -p conductor-verify` includes a regression test asserting that a malformed/empty MCP response maps to `blocked` with a true precondition, not silent pass (guarantees bounded decoding + verdict-wall integrity).
2. (security) Stub emits Pulse's raw `tools/call` result shape (no `CallToolResult` envelope) so tests exercise the true wire contract (prevents shape-mismatch regressions).
3. (security) `run_preflight` canary leg distinguishes call/parse errors from empty-corpus results — precondition strings are sanitized (no paths, library details); measurement fields in `blocked` rows are JSON `null`.
4. (security) `cargo audit` + `cargo-deny` stay green before merge; `Cargo.lock` unchanged.

## Relevant amendment history
**2026-06-15-dependency-audit-gate** — `rmcp` stays ≥1.4.0 (on 1.7.0) and is noted as past GHSA-89vp-x53w-74fx (DNS-rebinding fix, not applicable to stdio clients). Amendment reframed cargo-audit/cargo-deny as minimum **floors** (installed 0.22.1 / 0.19.4) + committed `Cargo.lock` determinism; the chunk's raw-JSON substitution must not drift the lock or break audit green.
