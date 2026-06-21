# security extract

## Relevance
Partial — MCP-specific; preflight is one of several verification pieces; some sections (Threat Model, Data Protection, Auth, Logging) have no chunk-specific content.

## Constraints
1. Protocol-version pin MUST assert negotiated version matches manifest-pinned `2024-11-05` exactly; mismatch ⇒ `Blocked` state (never silent downgrade) per security plan §Threat Model Summary, Attack surface: MCP read-back vector.
2. Required-tool presence assertions MUST validate all 4 pinned tools (`query_incident_list` · `retrieve_report` · `retrieve_telemetry_slice` · `mark_incident_resolved`) present before any read-back use; tool absence ⇒ `Blocked` per security plan §Input Validation, contract-manifest row.
3. Canary round-trip MUST emit a known incident, assert it reads back from corpus, prove end-to-end wiring; empty/missing canary ⇒ `Blocked`, never `pass` per security plan §Threat Model Summary: preflight readiness gate; scope: open design point notes encrypted-corpus/keychain leg (P-049) may be deferred.
4. Contract manifest file (`contracts/`) MUST be canonicalized via `std::fs::canonicalize` at the `conductor-verify` edge if `CONDUCTOR_CONTRACT_MANIFEST` override is supplied (path traversal defense, outside garde); format (TOML vs JSON) is an open design point per scope.
5. MCP errors and `tonic::Status` codes MUST be typed verification inputs routed through the verdict/error wall (`Ok(Blocked)` for precondition failures, `Result::Err` only for harness transport faults); never panic on child output per security plan §Error Handling + §Anti-Patterns: Universal.
6. Child-spawn subprocess hardening established in chunk 1 MUST hold: fixed `andromeda-pulse-mcp` program path, `ANDROMEDA_PULSE_DATA_DIR` passed strictly via `.env(...)` builder (never interpolated into argv), argument-injection metacharacters rejected before spawn per security plan §Anti-Patterns: Input + Code Patterns.

## Patterns to follow
1. Pinned contract manifest as single source of truth for negotiated protocol version + required tool list (security plan §Input Validation, contract-manifest row; scope: Artifacts; architecture Conventions: Inbound verification).
2. Typed `ReadyState` / `BlockedPrecondition` value types returned `Ok(...)` through the verdict/error wall, distinct from harness faults `Err(...)` (security plan §Error Handling; scope: Artifacts).
3. Stub MCP server in integration tests speaks `2024-11-05`, advertises the 4 required tools, round-trips canary (security plan §Threat Model Summary: preflight integrity; scope: Artifacts).
4. Manifest loader validates bounds (expected version present, required-tool names non-empty, canary incident identifiable) before use (security plan §Input Validation; scope: Artifacts — "parsed + bounds-checked").

## Anti-patterns to avoid
1. NEVER silently downgrade a failed preflight (version mismatch, missing tool, empty canary) to pass/fail/manual-check — must surface as distinct `Blocked` state with named precondition string (security plan §Anti-Patterns: Universal).
2. NEVER spawn `andromeda-pulse-mcp` as a configurable command or via shell — fixed hard-coded path, no config-into-argv (security plan §Anti-Patterns: Code Patterns, rmcp STDIO CVE-2026-30623).
3. NEVER panic on MCP error responses or `tonic::Status` codes — treat as typed verification inputs, first-class valued-routed through verdict wall (security plan §Anti-Patterns: Universal).

## Contract bindings
- **tests (CI integration gate):** chunk 1's deferred live-spawn integration test lands here; stub MCP server binary must speak `2024-11-05` + advertise required tools (scope: Artifacts; security plan §Dependency Security: CI integration).
- **conductor-emit (canary emission seam):** open design point (scope: Open design points) — dependency direction must respect crate-per-seam edges; stub integration covers read-back assertion independent of live-emit orchestration.
- **runs.db persistence (Epoch 6):** `ReadyState` need only be a serializable value here, not yet persisted; later chunks bind to error-sanitization run-report artifact rules (security plan §Error Handling: run-report sanitization).

## Acceptance criteria contributions
- (security) Preflight assertions never panic on malformed child output or `tonic::Status` codes — all error paths route through typed verdict wall, returning `Ok(BlockedPrecondition)` for failed assertions.
- (security) Contract manifest canonicalized at edge (if `CONDUCTOR_CONTRACT_MANIFEST` override supplied), format validated, required fields present.
- (security) Stub MCP server integration test speaks exactly `2024-11-05` and advertises all 4 required tools; mismatch ⇒ test verifies `Blocked` state is returned (never `pass`/`fail`).
- (security) Canary round-trip assertion rejects empty corpus response; integration test verifies empty ⇒ `Blocked` (never `pass`).

## Relevant amendment history
- **2026-06-15-dependency-audit-gate:** toolchain floor ≥1.94.1 confirmed done (channel 1.95.0, workspace MSRV 1.94.1), clearing tar-rs symlink-chmod CVE-2026-33056 in the `cargo build` extraction path; tauri ≥2.10.3 (origin-confusion CVE-2026-42184) remains a forward required bump (dormant until GUI crate, Epoch 9).
- **2026-06-15-structured-logging-stack:** obs identity env-handles `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path labels (JSON-escaped in log values), require no validation; distinct from path handles (`CONDUCTOR_CONTRACT_MANIFEST`) which canonicalize-and-bounds-check at edge.
