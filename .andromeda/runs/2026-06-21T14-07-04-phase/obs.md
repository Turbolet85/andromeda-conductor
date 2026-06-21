# obs extract

## Relevance — relevant

This chunk implements the OTLP egress liveness check — a connectivity probe to `127.0.0.1:4317` that gates scenario emission. It is a boundary seam of `conductor-emit` and directly touches the telemetry surfaces contract (gRPC egress path instrumentation).

## Constraints — domain rules that apply

1. **No OTel SDK for self-obs** — per §3 OTel SDK init: the probe is a connectivity check only, not a self-observation signal. Any error capture is `tracing::error!(...)` JSON structured logs, never an OTel SDK initialization.

2. **Verdict/error wall preservation** — per §1 Scope & Boundaries: a refused/unreachable transport is `Result::Err` (harness fault), **never** a `Verdict`/`ReportState` value. This boundary is load-bearing.

3. **Wall-clock only for timing** — per §10 SLO Invariants & Telemetry Budgets: any connect-timeout or latency measurement MUST use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (preserves journal-relative SLO math).

4. **Bounded span name set** — per §11 Anti-Patterns (Spans / Traces): the probe instrumentation spans MUST use low-cardinality span names from the bounded set (e.g., `emit.liveness_check` or similar `emit.*` pattern). High-cardinality names (per-host, per-port-attempt) are banned.

5. **No unstructured stderr on transport errors** — per §11 Logs: agent cannot parse fields from unstructured text. Error messages MUST be structured JSON or sanitized with machine-parseable hints (the redaction layer applies: absolute host paths & internal struct names scrubbed).

6. **Star topology preservation** — per scope Boundaries: the probe primitive lives in `conductor-emit`; no reverse seam dependency from `conductor-verify` or `conductor-timeline` to this check is introduced. The star topology (all seams depend on `conductor-core` only) is verified via `Cargo.toml` edges.

## Patterns to follow — existing patterns relevant to implementation

1. **Module-boundary error typing (thiserror)** — per §3 Observability Harness Contract / §6 Log Coverage: errors at seams (MCP, rusqlite, tonic) are typed via thiserror enums (e.g., `EmitError`). The probe's refusal/unreachable result surfaces as a typed error (never a panic), sanitizable at the binary edge.

2. **Structured error logging at boundary calls** — per §6 Log Coverage Boundary-call wrappers: gRPC emit spans log method + latency_ms + error (if any). A liveness-check error MUST be logged as a structured `tracing::error!` event with fields: error reason (e.g., `connection_refused`), attempted endpoint (`127.0.0.1:4317`), and a sanitized error message (no absolute host paths).

3. **Panic-free transport handling** — per §11 Error Reporting anti-pattern: "NEVER let a panic on the MCP read-back / transport path go uncaptured." The same discipline applies to the OTLP egress transport: malformed/unreachable endpoints ⇒ typed `Ok(blocked)` or `Err(EmitError)`, never panic.

## Anti-patterns to avoid — domain bans that apply

1. **NEVER export self-observation OTLP anywhere** — per §11 Universal: "NEVER export any self-observation OTLP — not to `:4317` (the PRODUCT fault stream) and not to `:4318` (unused/dead per arch)." The probe itself is a connect check, not an OTLP export; any self-obs logging of the probe result is stdout/file/console JSON only.

2. **NEVER use high-cardinality span names** — per §11 Spans / Traces: the bounded span name set is explicit (§4, amended 2026-06-18 to include `emit.logs_batch`). A per-attempt span (e.g., `emit.liveness_check.attempt_1`) would be high-cardinality and banned.

3. **NEVER skip the run_id field in structured logs** — per §11 Logs: every JSON line MUST carry `run_id` (the correlation key). If the probe runs before a scenario `run_id` is assigned, the log MUST still carry the parent run context or be tagged with a bootstrap correlation field.

## Contract bindings — where your domain ties into another

- **conductor-emit tonic 0.14.6 channel + EmitError typed enum** — the probe uses the existing raw OTLP scaffold (per scope Surfaces / contracts). The error type MUST be compatible with the emit module's existing error envelope.
- **Test harness contract (tests §3 & obs §3)** — the probe's success/failure determines whether `scenario.run` span is attempted at all. A failed liveness check halts the run before any scenario telemetry is emitted. The status shape (readiness gate) binds to tests' preflight contract (upstream obs-scope §5 Standard Contracts).
- **Redaction layer (conductor-core::redact)** — per §11 PII Scrubbing & 2026-06-15-log-error-boundary-redaction amendment: error messages at the binary edge MUST pass through the redaction layer (absolute file paths → `<redacted>`, internal struct names dropped via allowlist).

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. **(obs) Liveness probe returns `Ok(())` when `:4317` is connectable; `Result::Err(EmitError)` when refused/unreachable.** A refused transport MUST NOT produce a verdict value or panic; it is a harness fault that halts execution.

2. **(obs) Probe error is typed (thiserror) and sanitizable — no absolute host path, internal struct name, or stack trace leaks into operator output or run artifacts.** Error display MUST pass through the redaction layer; `Display` impl (not `Debug`) is used at the CLI/Tauri binary edge.

3. **(obs) If the probe is instrumented with a span, it uses a bounded low-cardinality name** (e.g., `emit.liveness_check`) **from the §4 / §11 bounded set.** No per-attempt, per-host, or per-port span names are introduced.

4. **(obs) The probe runs before any scenario-result telemetry is emitted; its success/failure gates whether `scenario.run` span begins.** The run-report envelope (§3 Run-report envelope schema) is populated only if the probe succeeds and the scenario executes.

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why

- **2026-06-17-raw-otlp-message-scaffold** — clarified the "no OTel SDK for self-observation" invariant as BEHAVIORAL. Noted that opentelemetry-proto's default features transitively pull `opentelemetry` + `opentelemetry_sdk` (dormant, never initialized). The liveness probe MUST NOT initialize the SDK even though the dep is present.

- **2026-06-18-severity-logs** — added `emit.logs_batch` to the bounded span-name set. While this chunk precedes severity-logs in sequencing, the amendment reaffirms the bounded-set discipline: all emit instrumentation spans are low-cardinality and enumerated. The liveness probe span name MUST follow the same pattern.
