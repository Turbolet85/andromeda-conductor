# security extract

## Relevance
Partial — this chunk (error spans emission) introduces OTLP trace construction and egress via tonic gRPC, touching input validation boundaries and dependency audit, but owns no sensitive data and is architecturally loopback-only.

## Constraints
1. **All new OTLP/prost structs MUST use deterministic ID generation.** Span/trace IDs are derived under the seeded discipline (no entropy/clock read per scope.md), never raw RNG, preserving reproducibility — per security-plan.md §Threat Model Summary (determinism discipline; Cross-cutting Patterns: Scope law + Determinism discipline) and scope.md Intent anchor.
2. **Bound parameters for ALL `runs.db` writes — never string-formatted SQL.** Even though content is self-generated synthetic data, use rusqlite 0.38.0 parameterized statements for `run_id`/`seed` writes (security-plan.md §Input Validation, rusqlite finding; §Security Anti-Patterns § Input ban). [NOTE: runs.db is Epoch 6 — forward-looking, not this chunk.]
3. **Tonic client egress remains loopback-only to `127.0.0.1:4317`.** Do not skip TLS verification or downgrade to plaintext; the threat model pins this as the sole deliberate egress surface (security-plan.md §Threat Model Summary § Attack surface: OTLP/gRPC egress; §Security Anti-Patterns § Data Protection ban on "disable TLS verification… on any surface ever promoted beyond loopback").
4. **Cargo.lock committed + `cargo-audit` (≥0.22.1 floor) + optional `cargo-deny` (≥0.19.4 floor) green.** Any new dependencies added (opentelemetry-proto already in tree; no new expected) must pass advisory audit before merge (security-plan.md §Dependency Security; §Security Anti-Patterns § Universal: "NEVER run cargo build --release… without cargo-audit green").
5. **Error handling: `tonic::Status` codes are typed verification inputs, never panics.** The verdict/error wall keeps `tonic::Status` as first-class typed input routed through `Ok(Verdict/ReportState)` vs `Result::Err` for harness faults only; a panic on the read-back path corrupts run classification (security-plan.md §Error Handling; §Threat Model Summary § Attack surface: MCP read-back, preflight gate).

## Patterns to follow
1. **Span/trace ID seeding discipline** — replicate the determinism pattern from existing scaffold (no global RNG; accept seed as input; derive IDs reproducibly under seeded randomness).
2. **Error type cascade** — use stack-native `thiserror` typed enums (`EmitError`) at the seam boundary, collapse to `anyhow` only at the `conductor-cli` / `#[tauri::command]` edges (security-plan.md §Error Handling; existing architecture Conventions: Error handling).
3. **Loopback egress reuse** — the assembled multi-span trace ships over the *existing* tonic `TraceServiceClient` to `127.0.0.1:4317`; no new transport or port surface (scope.md § Egress reuse; Threat Model Summary § Attack surface consistency).

## Anti-patterns to avoid
1. **NEVER use raw RNG / `SystemTime` for span/trace ID generation** — use seeded, deterministic derivation so "same scenario+seed ⇒ same trace shape" (security-plan.md §Error Handling § Run-report artifact sanitization: "use `std::time::SystemTime`/`Instant`" for clocks only, not IDs; scope.md § Deterministic ID generation).
2. **NEVER panic on `tonic::Status` / MCP read-back** — all child-process and transport errors are `Result` inputs to the verdict/error wall, not panics (security-plan.md §Threat Model Summary § MCP read-back attack surface; §Security Anti-Patterns § Universal: "NEVER let a malformed child/transport input panic").
3. **NEVER string-concat SQL for runs.db** — use bound parameters (security-plan.md §Input Validation § runs.db row; §Security Anti-Patterns § Input ban). [forward-looking]

## Contract bindings
**obs ↔ emission:** The error spans chunk introduces structured OTLP trace emission; downstream obs-plan (per cross-cutting read D26) must handle PII scrubbing and cardinality gates on the emitted span payloads. No hand-off needed for this chunk (emission construction only; verdict/PII scrubbing is later).

## Acceptance criteria contributions
- (security) `cargo audit` (≥0.22.1 floor) **green** — no advisory hits on new dependencies (opentelemetry-proto already in tree).
- (security) `Cargo.lock` committed and drift-free.
- (security) All span/trace ID generation is deterministic under seed, verified by scoped tests (loopback stub assertion on same scenario+seed ⇒ same trace ID shape).
- (security) `tonic::Status` codes from gRPC egress routed as typed `Result` inputs (no panics); `EmitError` collapse to `anyhow` at binary edge only.

## Relevant amendment history
**2026-06-15-dependency-audit-gate** (amendment #2) — cargo-audit/deny floor versions confirmed (`cargo-audit` ≥0.22.1 floor installed 0.22.1; `cargo-deny` ≥0.19.4 floor installed 0.19.4); toolchain bump `rust-version = 1.94.1` (≥1.94.1 floor) **done** (clears tar-rs symlink-chmod CVE-2026-33056). This chunk's audit gate must stay green; no tauri version bump needed yet (Tauri GUI is Epoch 9, dependency dormant).
