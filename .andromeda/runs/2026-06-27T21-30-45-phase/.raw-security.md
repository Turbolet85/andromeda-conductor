# security extract

## Relevance
Relevant — this chunk touches the MCP read-back child-process boundary (stdout validation), CLI inputs (env-var path handles), subprocess spawn hardening, and error/log emission boundaries (run-report artifacts).

## Constraints
1. MCP read-back child-process stdout MUST be bounded, non-panicking — the hand-rolled JSON-RPC client parses each line to `serde_json::Value` via serde_json (recursion-limited) + per-line soft size bound; JSON-RPC errors and empty canary route through the verdict/error wall to `Blocked` with named precondition, never panic (security plan §Threat Model § Attack surface — MCP read-back trusted-child boundary; §Input Validation § MCP-read-back-child-stdout row; §Anti-Patterns § Universal).

2. The `ANDROMEDA_PULSE_DATA_DIR` env handle MUST be validated before sidecar spawn — reject argument-injection metacharacters and pass strictly via `.env(...)` builder, NEVER interpolate into argv or shell (security plan §Threat Model § Attack surface; §Input Validation; §Anti-Patterns § Input; CVE-2026-30623).

3. The `andromeda-pulse-mcp` sidecar MUST be spawned only via fixed, hard-coded program path (no operator-chosen command) with `.env(...)` for `ANDROMEDA_PULSE_DATA_DIR` — no config-into-argv, no shell (security plan §Anti-Patterns § Code Patterns; MCP-sidecar STDIO design flaw).

4. Run-report artifacts (JSONL journals, `runs.db` rows, Markdown report) MUST NOT leak absolute host paths (canonicalized `CONDUCTOR_*` directories, `ANDROMEDA_PULSE_DATA_DIR`) or internal struct/field names — they are agent-parseable ground truth (security plan §Error Handling § Run-report artifact sanitization; §Anti-Patterns § Logging).

5. Preflight readiness-gate failure modes (version mismatch / missing tool / empty or non-matching canary / keychain fault) MUST surface as distinct `blocked` state with named precondition string, never silent downgrade to pass/fail/manual-check, never panic (security plan §Threat Model § preflight readiness gate; §Anti-Patterns § Universal).

6. CLI arguments and env-var overrides (`CONDUCTOR_SEED`, scenario key, `CONDUCTOR_*` path handles) MUST be validated at the `conductor-cli` edge — seed parses to integer type, scenario key maps to known P-ID, paths canonicalize + bounds-check before any write/read (security plan §Input Validation § CLI arguments & Env-var path handles).

7. JSONL journal and Markdown report timestamps MUST use `std::time::SystemTime`/`Instant`, never tokio's virtual clock, so journal-relative SLO math and determinism stay correct (security plan §Anti-Patterns § Logging).

## Patterns to follow
1. Hand-rolled JSON-RPC bounded-decode from `conductor-verify/src/jsonrpc.rs` — parse response lines to `serde_json::Value`, map JSON-RPC errors / decode faults to typed `VerifyError::{JsonRpc,Decode,Transport}` routed through the verdict/error wall (security plan § Threat Model — MCP read-back; § Input Validation).

2. Preflight readiness-gate typed outcomes per failure mode (version / tool / canary / keychain) → distinct `Blocked` with named precondition string (security plan § Established Decisions: MCP Read-Back Client; § Anti-Patterns § Universal).

3. Verdict/error wall discipline: harness faults (config parse, transport, MCP unreachable, canary fidelity mismatch) route to `Err`; verification outcomes (Pass / Fail / CalibrationRegion / ManualCheck / KnownResidual / Blocked) route to `Ok(Verdict)` / `Ok(ReportState)` — never panic on typed values (security plan § Error Handling).

4. Subprocess spawn hardening: fixed hard-coded path, `.env(...)` for paths, metacharacter rejection before spawn, no shell/eval (security plan § Threat Model; § Anti-Patterns § Input + Code Patterns).

## Anti-patterns to avoid
1. NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` (or operator-supplied values) into argv/shell — use `.env(...)` builder only (security plan § Anti-Patterns § Input; CVE-2026-30623).

2. NEVER uncanonicalize a `CONDUCTOR_*` path handle — canonicalize + bounds-check at CLI edge before any internal use (security plan § Anti-Patterns § Input; path traversal).

3. NEVER silently pass a failed preflight (version mismatch, missing tool, empty canary, keychain fault) as a non-`blocked` verdict — surface as distinct `blocked` with named precondition string (security plan § Anti-Patterns § Universal; Standard Contracts: Readiness gate).

## Contract bindings
- **Verdict/error wall** ↔ tests harness: run-report evidence (JSONL + `runs.db` + `.md`) is the CI-agnostic ground truth; verdicts are typed values, harness faults are typed errors routed at edges; live-run evidence populates the test-harness stub spine.
- **MCP read-back boundary** ↔ `conductor-emit` + `conductor-verify` composition: canary bridge emits via faithful emit, reads back via hand-rolled JSON-RPC, asserts fingerprint-fidelity; both seams share the typed error boundary (no panics cross the verdict wall).
- **Run-report sanitization** ↔ obs layer: JSONL journals + Markdown reports are self-observation emission surfaces; paths/struct-names are redacted at write time (not post-processed by obs after the fact).

## Acceptance criteria contributions
1. (security) Preflight readiness gate against live Pulse reaches `ready: true` with passing canary round-trip (was `Blocked`); verdict/error wall and blockage precondition naming invariants unregressed.

2. (security) Run-report artifacts (JSONL + `runs.db` + `.md`) contain zero absolute host paths or internal struct/field names (grep-verified); verdict/state/identity fields only for `Blocked` rows, measurement fields JSON `null`.

3. (security) MCP read-back decoding never panics on malformed child stdout — JSON-RPC errors / decode faults routed to typed `VerifyError` → `Blocked` or `Fail` via verdict/error wall.

4. (security) CLI `conductor preflight/run/suite` arguments and `CONDUCTOR_SEED` / `CONDUCTOR_*` path handles canonicalize + bounds-check before any write/read; scenario key maps to known P-ID.

5. (security) `ANDROMEDA_PULSE_DATA_DIR` passed to spawned sidecar via `.env(...)` builder only, never argv/shell; metacharacter rejection applied at spawn site.

## Relevant amendment history
- **2026-06-27-mcp-read-back-result-shape-adapter** (security-plan.md § Threat Model + § Input Validation + § Anti-Patterns + § Dependency Security): rmcp removed; read-back-child-stdout boundary reconciled to hand-rolled line-delimited JSON-RPC — bounded decoding (serde_json recursion-limited + per-line size bound), faults → `VerifyError::{JsonRpc,Decode,Transport}` via verdict/error wall; manifest assert reads `initialize` result's `protocolVersion` (was `peer_info()`); STDIO command/argument-injection class re-anchored to "MCP-sidecar STDIO"; rmcp ≥1.4.0 stdio-client note retired; audit tree "tonic/prost/rmcp" → "tonic/prost". **Why:** mirrors arch reversal; EVERY security invariant preserved (hardened spawn unchanged, read-back bounded + non-panicking, no new dependency, D-security-deps clean). Routine wording→sound-impl reconcile where invariant holds; decision reversal user-confirmed at wrap (2026-06-27).

- **2026-06-15-dependency-audit-gate** (security-plan.md § Dependency Security + § Bootstrap): cargo-audit/cargo-deny version pins reframed as minimum **floors** (installed 0.22.1 / 0.19.4); RustSec advisory DB fetched fresh each run — any tool ≥ floor running green satisfies gate. Toolchain "bump required" → "**done** (channel 1.95.0, rust-version = 1.94.1)". **Why:** audit gate landed green with floor versions; this chunk inherits clean dependency tree baseline (no new rustc/cargo deviations).
