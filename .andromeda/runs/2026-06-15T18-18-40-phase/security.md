# security extract

## Relevance
Partial — log/error-boundary redaction is a direct artifact-hygiene realization, but secret/PII handling applies only to Conductor's self-observation logs and operator-facing errors (not the product OTLP PII payload that's Epoch 3).

## Constraints
1. Field-allowlist redaction: self-obs log lines MUST emit only explicit allowlisted field names (per obs-plan §3/§6); non-allowlisted fields and values matching host-path/internal-struct-name shapes are dropped or masked before write (security-plan §Error Handling, artifact-hygiene invariant).
2. anyhow-edge sanitization: `conductor-cli` stderr and `#[tauri::command]` returns MUST NOT expose stack traces, absolute file paths (canonicalized `CONDUCTOR_*` directories), or internal struct/field names — only operator-facing `error:`/`hint:` shape (security-plan §Error Handling).
3. Panic-hook redaction: `std::panic::set_hook` payloads MUST NOT leak host paths to the error stream — panic outputs route through the same sanitization boundary (security-plan §Error Handling, §Anti-Patterns/Logging).
4. Reusable redaction primitive: the shared scrubber MUST be callable by future run-report artifact writers (`<run_id>.md` / `runs.db` / JSONL journal, Epoch 6) — do not hard-code sanitization into single-use paths (scope.md §Definition of done).

## Patterns to follow
1. Per security-plan §Error Handling: typed `thiserror` enums inside seam crates (EmitError, VerifyError, ConfigError), collapse to type-erased `anyhow` only at the `conductor-cli` / `#[tauri::command]` boundaries — this chunk wires the collapse layer.
2. Per security-plan §Input Validation (amendment): `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are benign non-path string labels JSON-escaped in log values — no validation, distinct from `CONDUCTOR_*` path handles that canonicalize+bounds-check.
3. Field-allowlist source: obs-plan §3/§6 defines the log JSON schema field-set — use that as the explicit allowlist, not a separate implicit config.

## Anti-patterns to avoid
1. NEVER expose stack traces, absolute file paths, or internal struct/field names to the operator via cli stderr or Tauri returns (security-plan §Error Handling, §Anti-Patterns/Logging).
2. NEVER let panic payloads or unfiltered tracing spans leak host paths or internal state to the error stream — sanitize before any line reaches the operator (security-plan §Anti-Patterns/Logging).
3. NEVER silently downgrade a blocked/harness-fault state to pass/fail/manual-check — the verdict/error wall keeps `tonic::Status` and MCP errors as typed verification inputs (security-plan §Anti-Patterns/Universal).

## Contract bindings
- **security ↔ obs**: obs-plan §3/§6 field-allowlist source drives the tracing-subscriber configuration (this chunk).
- **security ↔ Epoch-6 artifact writers**: run-report writers reuse the shared scrubber primitive built here.

## Acceptance criteria contributions
1. (security) Log lines carrying non-allowlisted fields or host-path/internal-struct-name values are emitted redacted (dropped or masked), verified by grep over stderr/file output during nextest run.
2. (security) `conductor-cli` error messages contain no stack traces, absolute paths, or internal struct names — verified by acceptance tests at the error edges.
3. (security) Panic-hook output redacts host paths before the error stream surfaces — verified by triggering a panic in the test harness and grepping stderr.
4. (security) `cargo nextest run --workspace`, clippy, and `cargo audit`/`cargo deny` remain green with no new dependencies or lock drift.

## Relevant amendment history
**2026-06-15-structured-logging-stack** — obs identity env-handles (`CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`) noted as non-path string labels requiring no validation (distinct from `CONDUCTOR_*` path handles). Resolved D-security-input escalation: they are JSON-escaped label values in log bodies, not validation boundaries.
