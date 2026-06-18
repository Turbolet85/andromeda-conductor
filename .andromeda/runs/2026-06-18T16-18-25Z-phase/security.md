# security extract

## Relevance
Relevant — this chunk adds a new log-emission primitive on the OTLP boundary, crossing the input-validation and error-handling trust boundaries.

## Constraints
1. All `ExportLogsServiceRequest` structs and `LogRecord` types MUST validate `SeverityNumber` ∈ [0,21] and assert matching `SeverityText` per OTel spec before gRPC egress (security plan §Input Validation, OTLP boundary discipline).
2. The `LogsEmitter` MUST treat `tonic::Status` responses and MCP-layer errors as typed verification inputs routed through the verdict/error wall (`Ok(...)` success vs `Result::Err` harness faults only); never panic on transport failure (security plan §Error Handling, typed error wall).
3. Wall-clock timestamps for log records MUST derive from `std::time::SystemTime` / `std::time::Instant`, never tokio's virtual clock, to preserve journal-relative SLO math determinism (security plan §Error Handling, Logging anti-patterns line 335).
4. `LogRecord.body` (`AnyValue`) MUST NOT leak absolute paths (`CONDUCTOR_*` directories, `ANDROMEDA_PULSE_DATA_DIR`) or internal seam-crate struct/field names in observable log output; bodies must remain benign placeholders or spec-controlled values (security plan §Error Handling run-report artifact sanitization, Logging anti-pattern line 334).
5. The logs feature on `opentelemetry-proto` dependency MUST be enabled via Cargo manifest feature change; no new crate introduction is acceptable (security plan §Dependency Security, Cargo.lock drift control).
6. No `LogRecord` emission pathway may spawn child processes, interpolate operator-supplied values into argv, or use shell evaluation — hard-coded fixed paths and `.env(...)` builders only (security plan §Input Validation line 114 + Anti-Patterns Code Patterns line 339, rmcp STDIO discipline applies to all subprocess spawn).

## Patterns to follow
1. Reuse existing `message.rs` helper functions (`service_resource`, `string_kv`, `unix_nanos`) for consistency with the prior trace/error-span emitters and to minimize new validation surface.
2. Mirror the `TraceEmitter` / `trace_request` builder shape and loopback egress to `127.0.0.1:4317` (no listener bind, no new port, only outbound gRPC client).
3. Test log emission via a loopback gRPC logs-service stub on ephemeral `127.0.0.1:0`; never bind the real `:4317` (reserved for Epoch-4 port-occupier fault scenario).

## Anti-patterns to avoid
1. NEVER deserialize `SeverityNumber` or construct `LogRecord` without bounds-checking the numeric value (0–21 per OTel spec); an out-of-range value silently passed to Pulse misses the 17-boundary hard-signal premise.
2. NEVER permit stack traces, absolute file paths, or internal struct names to surface in log `body` strings sent to Pulse (Logging anti-pattern line 334 — run-report artifact sanitization applies to emission).
3. NEVER use tokio virtual-clock timestamps for `time_unix_nano` / `observed_time_unix_nano`; journal-relative SLO verification depends on wall-clock determinism (Logging anti-pattern line 335).

## Contract bindings
Logs-emission ↔ obs (journal ground-truth): the emitted `SeverityNumber` and `body` must be journaled if the emit-side journal wiring is present (resolve from sibling chunks' altitude in research; default: standalone primitive, defer journal wiring per scope.md line 62).

## Acceptance criteria contributions
1. (security) `SeverityNumber` bounds validated on all constructed `LogRecord`s; test constructs boundary records (16/WARN and 17/ERROR) and verifies they remain distinguishable in the request.
2. (security) Log `body` strings contain no absolute paths or internal struct names; grep confirms no canonicalized `CONDUCTOR_*` or `ANDROMEDA_PULSE_DATA_DIR` leakage in test fixtures or emit code.
3. (security) `cargo audit` + `cargo deny check advisories` pass with `opentelemetry-proto` `logs` feature enabled; `Cargo.lock` drift check passes.
4. (security) `tonic::Status` responses and transport errors surface as `EmitError` variants (typed inputs), never panics; loopback stub test verifies error wall intact.

## Relevant amendment history
**2026-06-15-dependency-audit-gate:** `cargo-audit` 0.22.1 / `cargo-deny` 0.19.4 are minimum floors; RustSec DB fetched fresh each run. Toolchain confirmed at 1.95.0 channel, MSRV 1.94.1 (tar-rs symlink-chmod CVE-2026-33056 cleared). `tauri` ≥2.10.3 (origin-confusion CVE-2026-42184) is a forward required bump for the Tauri GUI chunk (Epoch 9), not this chunk — flagged but not yet active.