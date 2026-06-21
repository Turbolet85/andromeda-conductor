# security extract

## Relevance
Partial — OTLP egress liveness check is a transport-layer control within the Minimal threat model's egress boundary, but does not address input validation, data protection, or dependency supply-chain concerns.

## Constraints
1. **Loopback-only egress:** per security-plan §Threat Model Summary, OTLP/gRPC egress to `127.0.0.1:4317` is the sole network vector; the liveness probe must not introduce non-loopback or unauthenticated outbound routing (security-plan §Threat Model Summary — "loopback-only egress").
2. **Harness fault, not verdict:** per security-plan §Error Handling, a refused/unreachable transport must surface as `Result::Err` (harness fault), never as a `Verdict`/`ReportState` value; this boundary is load-bearing for the verdict/error wall (scope.md §Verdict/error wall).
3. **Typed error sanitization at the anyhow edge:** per security-plan §Error Handling, the refusal error must be a `thiserror`-typed enum (`EmitError`) that collapses to type-erased `anyhow` only at the `conductor-cli` / `#[tauri::command]` boundaries — no absolute host paths, internal struct names, or stack traces leaked to operator output or run artifacts (security-plan §Error Handling — "external responses sanitized").
4. **No panic on transport refusal:** per security-plan §Security Anti-Patterns § Universal, `tonic::Status` codes and transport errors are first-class typed verification inputs routed through the verdict/error wall; a panic on the read-back path corrupts run classification (security-plan §Security Anti-Patterns § Universal — "never let malformed child/transport input panic").
5. **Connect probe only — no state mutation:** per scope.md §What it builds, the probe establishes the channel and checks connectivity only; it does NOT export OTLP messages, mutate Pulse state, or run a scenario (scope.md — "Connect probe only").
6. **Wall-clock std::time, not tokio virtual clock:** per scope.md §Surfaces / contracts, any connect-timeout bound must use `std::time::SystemTime`/`Instant` (real time), never tokio's virtual clock; the probe is a connectivity gate, not a seeded timeline scheduling point (scope.md §Surfaces / contracts — "wall-clock from std::time").

## Patterns to follow
1. **Typed error enum in conductor-emit:** per security-plan §Input Validation (MCP read-back row) and Error Handling, use `thiserror` 2.0.18 typed `EmitError` enum with the refusal case named (e.g., `EmitError::EgressUnreachable`); derive no panics, collapse to `anyhow` only at binary edges (security-plan §Error Handling — "module-internal `thiserror` typed enums collapse to `anyhow` only at the `conductor-cli` / `#[tauri::command]` edges").
2. **Tonic preflight: bounded timeout + version negotiation:** per security-plan §Input Validation (MCP read-back row), the tonic `Endpoint::connect()` must be bounded by `std::time` (not tokio virtual clock) and must not downgrade protocol versions silently (security-plan §Security Anti-Patterns § Code Patterns — "NEVER pin the rmcp client to a strict newer protocol default").
3. **Refusal ⇒ Result::Err, never Blocked:** per scope.md §Verdict/error wall and security-plan §Error Handling, a refused transport is a harness fault (`Result::Err`), categorically distinct from `Blocked` (which is the MCP preflight state when readiness preconditions fail — security-plan §Security Anti-Patterns § Universal — "NEVER silently downgrade a failed preflight...to pass/fail/manual-check").

## Anti-patterns to avoid
1. **Do not silently promote refused egress to a Verdict::Blocked or Verdict::Fail:** per scope.md §Verdict/error wall, refusal is `Result::Err` only — it halts the run as a harness error before any scenario is measured; no verdict classification.
2. **Do not use tokio virtual clock for timeouts:** per scope.md §Surfaces / contracts, any connect-probe timeout is a real-time gate (std::time), never seeded-timeline scheduling; virtual clock corruption leads to false-positive/false-negative connectivity results.
3. **Do not expose absolute paths or internal struct names in the refusal message:** per security-plan §Error Handling, the `anyhow` edge must sanitize; e.g., never surface the canonicalized `CONDUCTOR_*` directory or tonic::Status struct fields directly to operator output.

## Contract bindings
- **architecture.md §Standard Contracts — Liveness equivalent (OTLP egress check):** "Before emission, the timeline engine confirms the gRPC channel to 127.0.0.1:4317 is connectable; a refused transport surfaces as a harness error (`Result::Err`), not a verification verdict." (scope.md — this chunk realizes that contract).
- **conductor-emit seam (star topology):** the probe lives in conductor-emit and surfaces refusal as a typed `EmitError`; no seam→seam dependency edge is introduced (scope.md §Boundaries — "no seam→seam dependency edge").

## Acceptance criteria contributions
1. **(security) Liveness probe returns `Ok(())` against a connectable `:4317`; against refused / down `:4317` it returns `Result::Err(EmitError)` (a typed `thiserror` enum), never a Verdict value, never a panic.** (scope.md §Acceptance intent — item 1).
2. **(security) The refusal error is typed and sanitizable at the binary edge** — `cargo clippy` + `gitleaks` pass; no absolute host path, internal struct name, or stack trace leaks into operator output or `runs.db` / JSONL journals (security-plan §Error Handling — "sanitized"; scope.md §Acceptance intent — item 2).
3. **(security) No new seam→seam dependency edge** — `Cargo.toml` edges + code-graph verification confirms conductor-emit's refusal chain does not reach conductor-verify or conductor-timeline (scope.md §Acceptance intent — item 3; security-plan §Security Anti-Patterns § Universal — "NEVER add...scenario without a Pulse P-ID, and NEVER...inbound network listener").

## Relevant amendment history
1. **2026-06-15-dependency-audit-gate** — audit-tool versions are minimum floors; toolchain bump confirmed done. **Section:** §Dependency Security / §Bootstrap phases. **Why:** cargo-audit 0.22.1 / cargo-deny 0.19.4 are floors (not rigid pins); RustSec advisory DB is runtime-fetched; toolchain already ≥ 1.94.1 (tar-rs fix applied). This chunk's tonic 0.14.6 / prost egress channel depends on the supply-chain gate staying green — no new dependencies introduced by this probe.
