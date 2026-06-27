# Session Handoff

**Last Updated:** 2026-06-27T21:19:07Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-27-mcp-read-back-result-shape-adapter — feat: hand-rolled JSON-RPC read-back client (rmcp removed) — Pulse's tools/call is non-MCP-compliant

## Position
- Done: **2026-06-27-mcp-read-back-result-shape-adapter** — replaced conductor-verify's rmcp read-back client with a **hand-rolled line-delimited JSON-RPC stdio client** (raw `serde_json::Value` results); rebuilt `stub_pulse_mcp` + the in-process test stub (`tests/common/mod.rs`) to emit Pulse's RAW shapes; un-masked `run_preflight`'s call-error catch-all; **rmcp dropped** from conductor-verify. All gates green + live boot smoke confirmed read-back now parses against live Pulse.
- Next: **re-promote "Live-Pulse E2E proof"** (the next markerless working-route entry, now UNBLOCKED — read-back works live) via `/andromeda-phase`. Its remaining work is the canary EMISSION (emit→corpus→read-back→fingerprint-fidelity bridge that flips preflight `ready`) + the 5-family faithful emit/extract. Live env is up + proven (pulse-app :4317 · `andromeda-pulse-mcp.exe` at `andromeda-pulse/target/debug` · `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_DATA_DIR=%APPDATA%\andromeda-pulse`). Note `chunks/2026-06-27-live-pulse-e2e-proof/` holds prior planning — regenerate scope/plan on re-promote (its rmcp-typed-call assumption is now superseded).

## Work done
2 NEW (`conductor-verify/src/jsonrpc.rs`, `tests/common/mod.rs`) + rewrites (`client.rs`/`error.rs`/`bin/stub_pulse_mcp.rs`/3 tests) + edits (`preflight.rs`/`spawn.rs`/`lib.rs`/`Cargo.toml`); `Cargo.lock` drops rmcp + tree. `conductor-run/src/lib.rs` UNCHANGED (the `Ok(_)/Err(_)` match was source-compatible). Gates: `cargo nextest -p conductor-verify --features stub-server` **67/67** · workspace nextest **419/419** · doctest ok · workspace clippy clean · `cargo audit` exit 0 / `cargo deny check` ok. Live boot smoke ✓ (preflight → real `query_incident_list`, precondition "incident not found in corpus" = the call parses; `ready:false` by design — canary emission is the next chunk). Code-graph **1293n/5735e**.

## Drift resolved
**drift = 0.** 7 doc-agents · 6 proposals · **1 escalation resolved WITH the user** (the arch §Established Decisions [MCP Read-Back Client] REVERSAL — rmcp → hand-rolled JSON-RPC, because Pulse's `andromeda-pulse-mcp` is non-MCP-compliant for `tools/call`; user-confirmed at /andromeda-phase P4 + the wrap escalation). Applied: arch (§Stack · [MCP Read-Back Client] · §Read-Back Dependency Posture · §Conventions inbound-verification + error-handling · §Inherited Defaults · tree comment) + security-plan (§Threat Model · §Input Validation manifest+child-stdout rows · §Anti-Patterns STDIO/negotiate-down · §Dependency Security tree+update-policy · §Key decision) — all rmcp→hand-rolled, **every invariant preserved** (hardened spawn, negotiate-down, bounded decode). 2 sidecars appended; **playbook rule added** (SUT-contradicts-a-locked-decision → escalate-once-then-apply). Cascade: CLAUDE.md (Stack + §Modules) · stack.md · security-summary.md · security.md. Other 5 docs `proposals: []`.

## Notes
- **Curation:** T1 ×0 · **T2 ×1** (`verification-harness.md`: the read-back is now hand-rolled JSON-RPC; supersedes the 2026-06-21 rmcp-pinning Session Additions, which are now dead — rmcp removed). T3 ×0. Filters: the Pulse-incident-model reference scored < 0.6 confidence (kept in the `pulse-mcp-readback-result-shape` memory instead). 0 conflicts · 0 deferred.
- **Decisions:** P4 = **hand-rolled JSON-RPC client** (over fork-rmcp / two-sessions — rmcp 1.7 deserializes every result into the typed `ServerResult`, no raw escape). Wrap escalation = **apply the arch reversal + the playbook rule** (user-approved).
- **Last failed command:** none.
- **Follow-up — NEW:**
  - **Stale rmcp mentions in test-plan distillations NOT re-derived this wrap** (test-plan had no body drift, so its cascade didn't fire): `verification-harness.md` body (lines 18/33 "rmcp initialize handshake" / "rmcp stub over stdio") + `tests-summary.md` line 44 ("rmcp stub (MCP, CI)") still say "rmcp". The Tier-2 Session Addition flags the correct reality; full body reconcile defers to the next test-plan-touching chunk's cascade (or a manual edit).
  - `MAX_LINE_BYTES` on the read-back stdout is a SOFT post-read bound (16 MiB) — a hard pre-read byte-cap is a possible future hardening (trusted local sidecar; serde_json is recursion-limited).
- **Follow-up (carried):**
  - **Live-Pulse E2E proof** (re-promote next) — canary emission bridge + 5-family faithful emit/extract; live env proven Windows-runnable.
  - Operator-pause **live firing** + operator-checklist **live items** — Epoch-10 live-Pulse.
  - Coverage view's **live per-P-ID verdict lamps** (runs.db "latest RunRecord per P-ID") — Epoch-10.
  - `scenario.run` root obs span (the run driver) — Epoch-10.
  - `indicatif` 0.17→0.18 may drop `number_prefix`, retiring the RUSTSEC-2025-0119 deny.toml ignore.
  - `opentelemetry-proto default-features=false` trim (dormant transitive OTel SDK).
  - Desktop a11y verification + the GUI a11y CI gate await a Linux+xvfb env (separate polish — NOT live-Pulse-gated).

## Session End Status
Wrap in progress — commit pending the P7 gate.
