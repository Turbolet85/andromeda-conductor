# tests extract

## Relevance
partial — the chunk is a supply-chain audit gate (config/tooling, no code), with test bindings to CI coverage gates and quality gates, not direct test implementation.

## Constraints
- per test-plan §1 Coverage triggers: supply-chain audit gate (`cargo-audit` + `cargo-deny`) is a mandatory Minimal-tier quality control (Vector 1 environment + toolchain version floor)
- per test-plan §3 5-command implementation (`run` stage): `cargo audit --deny warnings` and `cargo deny check` must execute green during the test harness `run` command before scenario invocation
- per test-plan §10 Quality Gates: dependency-security gate is one of the enforceable exit-code assertions (audit failure ⇒ non-zero exit, blocking the release gate)
- per test-plan §Bootstrap phases: coverage-tooling-install / quality-gate-config-emit phases wire the audit commands into the CI workflow config (this chunk provides the policy config and first green run; CI wiring is downstream)
- per test-plan §Security anti-patterns: committed `Cargo.lock` must remain un-drifted; `--locked` reproducibility check is a precondition for deterministic audit runs

## Patterns to follow
- per test-plan §Test-Relevant Conventions: deterministic audit runs require committed `Cargo.lock` as the source of truth (no lock-file drift between runs)
- per test-plan §3 Log format binding: any audit failures logged to stderr must be captured by the harness as sanitized output (no leakage of internal paths/system details)
- per test-plan §Test harness contract: audit tooling is wired into the `run` command execution path, not a separate daemon/listener — straightforward subprocess invocation with exit-code semantics

## Anti-patterns to avoid
- Do NOT run `cargo build --release` or merge without `cargo audit` and `cargo deny check` green (per test-plan §Minimal-tier residual-risk control)
- Do NOT allow `Cargo.lock` to drift; every audit run must use `--locked` to ensure reproducibility
- Do NOT skip toolchain/crate version floors (1.94.1 tar-rs CVE, `tauri` ≥ 2.10.3, garde 0.22.1 advisory status)

## Contract bindings
obs ↔ tests harness: audit stderr captured by assert_cmd harness; must not leak absolute paths or internal struct names (test-plan §Log format / §Run report envelope sanitization)

## Acceptance criteria contributions
- "(tests) `cargo audit --deny warnings` passes green (RustSec tree clean)."
- "(tests) `cargo deny check` passes green over committed `deny.toml` (advisories + bans + licenses + sources)."
- "(tests) `Cargo.lock` committed + `--locked` build/audit produces no drift."
- "(tests) Toolchain ≥ 1.94.1 and `tauri` ≥ 2.10.3 audit dependencies confirmed."

## Relevant amendment history
(none)