# arch extract

## Relevance
relevant

## Constraints
- Per §Design Philosophy: "Headless-drivable core, thin shells" — read-back transport is part of the runtime-agnostic library surface that both CLI and Tauri command edges will call identically; no Tauri-specific logic belongs here.
- Per §Stack and Technologies: MCP read-back MUST use rmcp 1.7.0 (`client` feature) — the official Rust MCP SDK with version negotiation support.
- Per §Established Decisions [MCP Read-Back Client]: negotiation MUST pin down to `2024-11-05` (Pulse's hand-rolled server version); strict newer-version defaults are a silent-mismatch risk; `ProtocolVersion` observable to downstream preflight gate.
- Per §Occupied Resources (Environment variables): `ANDROMEDA_PULSE_DATA_DIR` MUST be propagated to the spawned sidecar ONLY via `.env(...)` after metacharacter rejection; never interpolate into argv or shell. Resolve platform-default fallback when unset.
- Per §Occupied Resources (Process names): sidecar process is `andromeda-pulse-mcp`, launched via `TokioChildProcess` over stdio with fixed hard-coded path (no shell interpolation).
- Per §Conventions (Error handling): spawn/transport/RPC errors fan into typed `VerifyError` (thiserror) variants; verdict/error wall enforces `Result::Err` reserved for harness faults only.

## Patterns to follow
- Per §Established Decisions [Async Runtime Flavor] — use the core's hand-built `tokio::current_thread` runtime; do not spawn additional multi-thread runtimes or work-stealing schedulers.
- Per §Cross-cutting Patterns (Config management) — environment variable propagation follows the established `.env(...)` builder pattern from Pulse's prior art (`tests/sidecar_subprocess.rs`); hard-coded paths, no dynamic interpolation.
- Per §Conventions (Interface surfaces) — MCP read-back is an inbound verification surface; modeled as a long-lived session returned to callers (not yet wired to scenario runner — that is Epoch-5 chunk 2's job).

## Anti-patterns to avoid
- Do NOT use the opentelemetry-otlp exporter or hand-rolled JSON-RPC; rmcp 1.7.0 is the locked choice for version negotiation safety.
- Do NOT pin a strict newer rmcp `ProtocolVersion` default or bypass negotiation; Pulse's server is dated `2024-11-05` and the client MUST negotiate downward.
- Do NOT spawn the sidecar with a shell, interpolate the data-dir into argv, or accept unvalidated env-var values without metacharacter rejection (rmcp STDIO injection class, CVE-2026-30623).

## Contract bindings
- **Downstream: Epoch-5 chunk 2 (preflight readiness gate)** — consumes the `ProtocolVersion` observable from this chunk's session handle to assert `2024-11-05` and validate tool presence; passes typed errors to the gate's `Blocked` mapping logic.
- **Downstream: Epoch-5 chunks 3–5 (verdict / SLO timing)** — consume the four typed tool-call wrappers (`query_incident_list`, `retrieve_report` with `degraded_mode`, `retrieve_telemetry_slice`, `mark_incident_resolved`) to verify scenario outcomes against SLO windows.
- **Binds to security-plan:** subprocess-spawn hardening (fixed path, no shell, metacharacter rejection on `ANDROMEDA_PULSE_DATA_DIR`); bounded decode on read-back path; verified-safe argument passing per security-review checklist.

## Acceptance criteria contributions
- (arch) `conductor-verify` module lives per workspace boundary rules (§Inherited Defaults Crate-per-seam); rmcp 1.7.0 is the pinned dependency (§Stack and Technologies).
- (arch) Sidecar spawn hardening verified: fixed path (no shell), `.env(...)` data-dir propagation only, metacharacter rejection before spawn (§Occupied Resources Environment variables + §Cross-cutting Patterns Config management).
- (arch) Version negotiation observable: negotiated `ProtocolVersion` field present on session handle, pinned to `2024-11-05` on success (§Established Decisions [MCP Read-Back Client]).
- (arch) Typed error wall: rmcp client/transport/spawn errors mapped to `VerifyError` variants (thiserror); no unwrap/panic on read-back failures (§Conventions Error handling).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1:** This chunk's `TokioChildProcess` usage depends on the pinned Rust 2024 + tokio 1.48.x (§Established Decisions Async Runtime); MSRV 1.94.1 floor applies. This chunk likely builds on the workspace scaffold that raised the floor; verify `cargo` and `rust-version` match the 1.94.1 pin.
- **2026-06-16-test-framework-fixtures-coverage-tooling — test toolchain registered:** Acceptance intent §52 names "Tests cover what is verifiable without a live Pulse (metacharacter rejection, data-dir default resolution, error mapping; an MCP stub child where the test harness supports it)." This chunk's test strategy depends on the pinned rstest/proptest/insta/assert_cmd/assert_fs/predicates stack + nextest runner (zero-retry `ci` profile); use those fixtures for spawn-error mocking and env-var fallback simulation.
