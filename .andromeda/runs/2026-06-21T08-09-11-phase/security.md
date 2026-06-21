# security extract

## Relevance
Partial — the chunk introduces the rmcp client/transport layer and subprocess spawn surface, triggering subprocess-spawn hardening + dependency security + input validation constraints; preflight readiness gate + verdict logic are later chunks.

## Constraints
1. **Subprocess spawn hardening — fixed path, no shell, no arg interpolation** per security-plan §Threat Model Summary (Attack surface: MCP read-back vector), §Input Validation (env-var bounds row: `ANDROMEDA_PULSE_DATA_DIR` metacharacter rejection), §Anti-Patterns §Code Patterns: spawn `andromeda-pulse-mcp` from hard-coded path only; pass `ANDROMEDA_PULSE_DATA_DIR` strictly via `.env(...)` builder; reject argument-injection metacharacters before spawn; never interpolate into argv/shell (rmcp STDIO command/argument-injection class, CVE-2026-30623 / OX advisory).
2. **Version negotiation must land on `2024-11-05` (hand-rolled Pulse protocol), never strict-newer default** per security-plan §Anti-Patterns §Code Patterns: negotiate DOWN, not up; expose negotiated version via `peer_info()` for downstream preflight gate to assert.
3. **Bounded protobuf decoding on read-back path; reject unbounded recursion** per security-plan §Input Validation (MCP read-back child stdout row): no unbounded prost/protobuf decoding on the read-back path (protobuf-decode DoS lineage RUSTSEC-2020-0002 / RUSTSEC-2024-0437); empty canary round-trip ⇒ `blocked`, never false pass.
4. **Typed error wall: `VerifyError` thiserror enum; rmcp/transport errors fan into `VerifyError` variants, not panics** per security-plan §Error Handling (Internal logging + Error format) + §Anti-Patterns §Universal: `tonic::Status` codes and MCP error responses are first-class typed verification inputs routed through the verdict wall; never panic on read-back input.
5. **Dependency security: rmcp 1.7.0 (`client` feature) is the first rmcp use; `Cargo.lock` must stay un-drifted; `cargo-audit` ≥ 0.22 + optional `cargo-deny` ≥ 0.19 must pass** per security-plan §Dependency Security (Audit tool, Pinning, CI integration) + §Bootstrap phases (dep-security-ci-gate) + §Anti-Patterns §Universal: `Cargo.lock` committed, audit green before release; rmcp pinned ≥ 1.4.0 (on 1.7.0 — past Streamable-HTTP DNS-rebinding fix GHSA-89vp-x53w-74fx, not applicable since Conductor is stdio client).
6. **Typed tool-call surface wraps the four consumed tools** per scope.md (MCP tools consumed: `query_incident_list`, `retrieve_report` with `degraded_mode`, `retrieve_telemetry_slice`, `mark_incident_resolved`); errors surface as typed values the gate/verdict chunks later consume.

## Patterns to follow
1. **Scope-confined spawn** (security-plan §Code Patterns): spawn from `TokioChildProcess` as a fixed hard-coded program path; `ANDROMEDA_PULSE_DATA_DIR` resolved to platform default when Pulse leaves it unset (Windows `%APPDATA%\andromeda-pulse` · Linux `$XDG_CONFIG_HOME`/`~/.andromeda-pulse`), then passed strictly via `.env(...)` after metacharacter rejection.
2. **Versioning discipline** (security-plan §Anti-Patterns §Code Patterns): rmcp negotiation must tolerate minimal older versions (hand-rolled servers like Pulse's `2024-11-05`); strict-newer defaults are the silent-mismatch class the preflight gate exists to prevent.
3. **Verdicts as typed values, errors as first-class inputs** (security-plan §Error Handling, Threat Model Summary § Attack surface): `Result::Err` reserved for harness faults (spawn failure, transport down); protocol/tool issues become `VerifyError` values routed through the verdict/error wall, not panics.
4. **Bounded recursion discipline** (security-plan §Input Validation): rmcp client/tonic codegen are the FFI surface on the read-back path; bounded decode + the canary round-trip verify trustworthiness before downstream verdict logic touches the data.

## Anti-patterns to avoid
1. **NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` into argv or shell** (security-plan §Anti-Patterns §Input + §Code Patterns): use `.env(...)` only; reject metacharacters first; fixed-path spawn; unscoped/shell interpolation is CVE-2026-30623 / OX advisory class.
2. **NEVER spawn from operator-chosen command or use shell/eval** (security-plan §Anti-Patterns §Code Patterns): spawn `andromeda-pulse-mcp` as a hard-coded program path only; rmcp STDIO design flaw if operator can control the sidecar binary.
3. **NEVER panic on child stdout / MCP errors; never silently downgrade failed preflight** (security-plan §Anti-Patterns §Universal + §Code Patterns): treat `tonic::Status` / MCP error responses as typed verification inputs for later gate logic; an empty canary MUST surface as `blocked`, never a false pass-as-empty.

## Contract bindings
- **Preflight gate ↔ this chunk** — version negotiation output (via `peer_info()`) + typed `VerifyError` surface enable the downstream preflight chunk to assert version + tool presence + canary round-trip without re-spawning.
- **Verdict/error wall ↔ conductor-report seam** — `VerifyError` enum and the `Ok(Verdict/ReportState)` vs `Result::Err` discipline keep run classification consistent across seam crates.
- **Dependency security ↔ CI gate** — rmcp 1.7.0 first use; `cargo-audit`/`cargo-deny` + committed `Cargo.lock` enforce the supply-chain control for the OTLP/gRPC/MCP tree + `bundled` SQLite C.

## Acceptance criteria contributions
1. **(security) Sidecar spawn hardening verified** — test + grep confirm spawn path is hard-coded (not operator-derived), `ANDROMEDA_PULSE_DATA_DIR` rejection of `$`, `|`, `;`, `&`, backticks, and other shell metacharacters happens before `.env(...)`, and no argv/shell interpolation occurs.
2. **(security) Version negotiation + `peer_info()` observable** — integration test confirms negotiation lands on `2024-11-05` (or traces it as the active version), `peer_info()` returns the negotiated version, and mismatch-prone preflight gate has a typed input to assert against.
3. **(security) `VerifyError` typed surface + no panics on read-back path** — grep verifies no `unwrap()` / `expect()` on rmcp client / tonic responses; all `Result::Err` map to `VerifyError` variants; mcp-stub test harness (where available) confirms errors route to verdict wall, not panics.
4. **(security) `cargo audit` + `cargo-deny` clean on rmcp 1.7.0** — `Cargo.lock` committed and un-drifted; CI gate (`cargo audit` ≥ 0.22 + optionally `cargo deny check advisories bans sources licenses`) passes green before merge (no advisories on rmcp / tonic / prost / `bundled` SQLite tree).

## Relevant amendment history
1. **2026-06-15-dependency-audit-gate** — audit-tool versions (cargo-audit / cargo-deny) are minimum **floors** (0.22.1 / 0.19.4 confirmed green), not strict pins; toolchain bump to ≥ 1.94.1 (tar-rs symlink-chmod fix) is **done** (1.95.0). Applies here: rmcp 1.7.0 must audit green; `Cargo.lock` un-drifted is the control.
2. **2026-06-15-structured-logging-stack** — `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs identity) are non-path string labels (no validation); distinct from `CONDUCTOR_*` *path* handles. Does NOT apply to this chunk (read-back client only touches `ANDROMEDA_PULSE_DATA_DIR`, which IS a path handle requiring canonicalize + bounds-check per §Input Validation).
