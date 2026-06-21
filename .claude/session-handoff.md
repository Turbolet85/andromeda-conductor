# Session Handoff

**Last Updated:** 2026-06-21T10:24:11Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-21-preflight-readiness-gate — feat: preflight readiness gate (conductor-verify, Epoch 5)

## Position
- Done: **2026-06-21-preflight-readiness-gate** — the MCP `initialize` preflight readiness gate in `conductor-verify`: 3 assertions over the chunk-1 `ReadbackClient` (protocol pin `2024-11-05` · required-tool presence vs the pinned `contracts/mcp-contract.toml` manifest · data-dir canary read-back), each a distinct `ReportState::Blocked` precondition; `ReadyState` value (arch readiness-envelope shape, `data_dir` redacted); `run_preflight`/`preflight_boot` over the `connect_transport` seam; + the deferred live `TokioChildProcess` child-spawn test (feature-gated `stub_pulse_mcp` bin). **Epoch 5 (Verification & read-back) — chunk 2 of 6.**
- Next: **Epoch 5 chunk 3 — "OTLP egress liveness check"** (loopback `:4317` connectable; refused ⇒ harness `Err`, NOT a verdict) → `/andromeda-phase` to promote + plan.

## Work done
New `conductor-verify/src/{manifest,preflight}.rs` + `src/bin/stub_pulse_mcp.rs` + `tests/{preflight,preflight_spawn}.rs` + `contracts/mcp-contract.toml`; `error.rs` (+`VerifyError::Manifest`), `lib.rs` re-exports, `Cargo.toml` (`serde`/`toml` deps + `stub-server` feature + the stub `[[bin]]`). Gates green: conductor-verify 24/24 (default) · 25/25 (`--features stub-server`, incl. live child-spawn) · workspace 199/199 (+9) · clippy `-D` (+ feature) · doctest 0 · `cargo audit` + `cargo deny` clean · `Cargo.lock` un-drifted. **No seam→seam edge — `conductor-verify` stays `conductor-core`-only (star topology preserved).**

## Drift resolved
1 proposal from the 7-detector fan-out (6 returned `proposals: []`). **arch D-arch-decisions (warning)** — flagged the `stub-server` feature pulling `rmcp/server` as a departure from the rmcp-client-only posture. **Resolved → dismiss** (user-confirmed): test-only, feature-gated, never-shipped mock; chunk 1 already used rmcp `server` as a dev-dep with no arch amendment; production posture unchanged. Codified a new `playbook.md` rule (test-fixture feature use → routine dismiss) so it doesn't re-trigger. No spec bodies/sidecars edited.

## Notes
- **Key decisions:** P4 AskUserQuestion — canary = **read-back assertion only** (identity is a `CanaryMarker` parameter; emission is the Epoch-8 CLI's job), preserving the star topology. Open-questions resolved at implement: (q2) rmcp negotiates server-side + accepts the server's reported version, so a version mismatch routes through `run_preflight`, not a connect-`Err`; (q3) `data_dir` redacted via `redact_value`; (q4) feature-gated `[[bin]] stub_pulse_mcp` + `CARGO_BIN_EXE_*`.
- **Curation:** Tier 2 +2 — `verification-harness.md` (rmcp server-driven negotiate-down + test the mismatch from the manifest side) · `testing.md` (feature-gated stub-bin + `CARGO_BIN_EXE` real-child-spawn pattern). Filtered 1 (obs allowlist-drop — covered by `observability.md` body).
- **Follow-up (tracked, not route chunks):** (a) live child-spawn test — **DONE this chunk** (was chunk-1's deferral). (b) scenario-config garde wiring for the 4 fault helpers — Epoch 7. (c) `opentelemetry-proto default-features=false` — still open. (d) obs `fault.silence` sentinel — Epoch 7/8. (e) *new:* whether obs-plan §6 should add `blocked_precondition` to the log-field allowlist (today logged via redacted `message` + allowlisted `state`).
- **Last failed command:** none.
