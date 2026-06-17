# security extract

## Relevance
relevant — this chunk introduces a new IPC/RPC boundary (tonic/prost gRPC egress) and explicit untrusted input surface (raw OTLP protobuf structs from operator-supplied scenarios).

## Constraints

- All `ExportTraceServiceRequest` message assembly MUST use hand-constructed raw structs from opentelemetry-proto 0.32.0 with byte-level field control; no auto-marshalling that obscures fault-injection surface (per security-plan §Input Validation, four-boundary discipline for new RPC).
- `OTLP/gRPC egress to 127.0.0.1:4317` MUST be loopback-only; a transport refusal surfaces as a harness `Result::Err` and routes through the verdict/error wall, never panics (security-plan §Threat Model Summary § Attack surface — "trust boundary: loopback-only egress").
- Bound protobuf decoding of `tonic::Status` responses MUST prevent unbounded recursion; empty server responses MUST NOT false-pass — treat as transport verification input, not a verdict (security-plan §Input Validation, RUSTSEC-2020-0002 / RUSTSEC-2024-0437 DoS lineage).
- `EmitError` (thiserror enum) MUST collapse to type-erased `anyhow` at the `conductor-cli` edge only; internal error detail stays inside `conductor-emit` and is not exposed to operator stderr or run-report artifacts (security-plan §Error Handling, Error format).
- `Cargo.lock` MUST be regenerated + committed; `cargo-audit ≥ 0.22` + `cargo-deny ≥ 0.19` (recommended) MUST pass green on the new opentelemetry-proto / tonic / prost transitive tree before merge (security-plan §Dependency Security, CI integration + Critical CVE response SLA).

## Patterns to follow

- Tonic 0.14 codegen via `tonic-prost-build` (security-plan §Input Validation, "Inbound verification" conventions — codegen via pinned tool version).
- Typed `tonic::Status` codes are first-class verification inputs, never panics — route through the verdict/error wall alongside `Ok(...)` verdicts (security-plan §Error Handling, Error format).
- Version-pinned contract negotiation on the read-back child (MCP-adjacent pattern; OTLP egress is outbound only, but pinning discipline applies: tonic 0.14.6 fixed per chunk scope).

## Anti-patterns to avoid

- NEVER interpolate operator-supplied scenario config or severity/fingerprint fields into the gRPC message without garde validation at scenario load (security-plan §Input Validation, config-boundary row; bans: NEVER use Tauri `shell-open` if scenario data enters it, NEVER skip bounded protobuf decoding).
- NEVER expose `tonic::Status` details, internal struct names, or stack traces to operator stderr or run-report artifacts (security-plan §Error Handling, Logging bans).
- NEVER add unsafe code on the tonic/prost FFI boundary without review; keep raw protobuf decoding bounded (security-plan §Security Anti-Patterns § Universal, NEVER skip bounded protobuf).

## Contract bindings

- **obs ↔ tests:** loopback gRPC stubs only in tests; self-observation logging redacts absolute paths and internal field names (security-plan §Error Handling, run-report artifact sanitization).
- **scaffold ↔ later Epoch-3 chunks:** raw OTLP primitives here expose the byte-level fields P-005..P-060 scenarios will perturb (severity, status codes, fingerprints); fault injection follows later.

## Acceptance criteria contributions

- (security) `Cargo.lock` committed + `cargo audit` green on opentelemetry-proto / tonic / prost tree (security-plan §Dependency Security, CI integration).
- (security) All `ExportTraceServiceRequest` assembly avoids uncontrolled marshalling; field-by-field byte control via raw structs (security-plan §Input Validation, four-boundary discipline).
- (security) `EmitError` typed enum wired; transport failures route through verdict/error wall, not panics; `anyhow` collapse at CLI edge only (security-plan §Error Handling, Error format).
- (security) `tonic::Status` responses parsed with bounded recursion + empty canary assertion (security-plan §Input Validation, protobuf-decode DoS).

## Relevant amendment history

- **2026-06-15-dependency-audit-gate:** audit-tool versions reframed as minimum floors (cargo-audit ≥ 0.22, cargo-deny ≥ 0.19); toolchain ≥ 1.94.1 confirmed done. This chunk's gRPC/protobuf tree is a new high-risk-class transitive dependency; `Cargo.lock` commit + green audit is the residual-risk control for Minimal tier (security-plan §Threat Model Summary § tier justification, "dependency/supply-chain audit").