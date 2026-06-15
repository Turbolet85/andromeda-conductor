# Playbook — Conductor

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: a Foundation-epoch chunk uses plain `cargo test` / `#[test]` before the "Test framework + fixtures" chunk installs cargo-nextest / rstest
  verdict: routine
  note: test-plan §2/§4 names cargo-nextest as the TARGET runner; interim `cargo test` is build-sequencing, not drift against the test strategy. Confirmed with the user on 2026-06-15 (config-validation-surface wrap); recurred from the prior chunk.
- pattern: an external audit/dev CLI tool (e.g. cargo-audit / cargo-deny) is installed a patch/minor behind the version the security-plan names, and the gate runs green
  verdict: routine
  note: dev CLI tools can't be pinned in `Cargo.lock`; the security-plan versions are minimum FLOORS and the real signal (the RustSec advisory DB) is fetched fresh at runtime, so a tool ≥ floor running green satisfies the gate. Confirmed with the user on 2026-06-15 (dependency-audit-gate wrap); security-plan §Dependency Security now states floors.
