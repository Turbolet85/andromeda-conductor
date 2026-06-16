# tests extract

## Relevance
partial

## Constraints
1. Test tier is Minimal (0) per test-plan §1 Scope Summary — augmented with Section 5 coverage triggers for security vectors + determinism property tests, no load/saturation testing
2. 5-command discipline (boot/run/status/cleanup/logs) is a binding harness contract per test-plan §3 Test Harness Contract — boot/run/status/cleanup are stubs here (seams not yet built); logs is a forward declaration of JSONL emission journal
3. Zero-flakiness mandate applies — no retry policies; flakes indicate real determinism breaks per test-plan §10 Quality Gates
4. Agent-driven invariant: all test layers must be runnable end-to-end by reading exit codes + structured output (jq/serde), no human-in-loop verification per test-plan §2 Agent-runnable invariants
5. Scenario catalog (P-001..P-060) enforcement: every scenario has a P-ID; coverage-matrix.md completeness is the "0.1.0 done" definition per test-plan §1 Critical paths
6. Minimal coverage thresholds enforced: ≥60% line / ≥50% branch / ≥70% function (cargo-llvm-cov `--fail-under-lines 60`) per test-plan §10 Quality Gates
7. Supply-chain audit green (cargo-audit + cargo-deny + committed Cargo.lock + toolchain ≥1.94.1 + tauri ≥2.10.3) is a binding build-failure condition per test-plan §9 CI Integration + §11 Anti-Patterns CI

## Patterns to follow
1. Per-seam crate-local `#[cfg(test)]` modules for unit tests + crate-local `tests/` for integration tests (arch Test-Relevant Conventions) per test-plan §2 Test directory + naming conventions
2. rstest 0.26.1 fixtures for seeded `conductor-timeline` generator + table-driven P-ID/garde-config matrices + `#[once]` for in-memory `runs.db` schema per test-plan §4 Unit Test Strategy + §7 Test Data & Fixtures
3. Run-report envelope as the canonical golden-tested shape (insta 1.46.1 in CI/assert mode, redacting run_id/timestamps) shared by Markdown report + runs.db row + JSONL journal per test-plan §3 Status endpoint shape + §7 Test Data & Fixtures
4. cargo-nextest 0.9.137 with ci-profile JUnit XML (parsed by dorny/test-reporter for PR annotations) + `cargo test --doc` for doctests + `cargo clippy --workspace --all-targets -- -D warnings` per test-plan §3 Test Harness Contract run command + §4 Unit Test Strategy

## Anti-patterns to avoid
1. NEVER set nextest retries > 0 or tolerate flaky tests — quarantine and fix immediately (retry policies mask real failures); determinism is enforced upstream per test-plan §10 Zero-flakiness budget + §11 Anti-Patterns Quality
2. NEVER fake Pulse's reaction as a CI verdict — rmcp stub canary proves MCP wiring only; live Pulse leg is local-gate-only via workflow_dispatch, never CI per test-plan §11 Anti-Patterns Test Strategy
3. NEVER interpolate ANDROMEDA_PULSE_DATA_DIR (or operator values) into sidecar argv — pass via `.env(...)` after rejecting injection metacharacters; spawn andromeda-pulse-mcp as fixed hard-coded path per test-plan §11 Anti-Patterns Mocking + Vector 4 negative test

## Contract bindings
- **Harness ↔ obs:** 5-command discipline (boot/run/status/cleanup/logs) contract §3 binds to obs §3; status returns Run-report envelope JSON matching obs-declared shape; logs tail runs/<run_id>.jsonl (emission journal ground truth); self-obs stream (stderr/logs/agent-latest.jsonl) is distinct artifact per amendment 2026-06-15-structured-logging-stack
- **Harness ↔ a11y:** CI a11y gate (color-only rule, text/role pairing, aria-live assertions) binds to a11y §Bootstrap per test-plan §10 Quality Gates + §3 Bootstrap phases
- **Harness ↔ security:** supply-chain gate (cargo-audit + cargo-deny + Cargo.lock + toolchain ≥1.94.1 + tauri ≥2.10.3) + injection-rejection test (ANDROMEDA_PULSE_DATA_DIR metacharacter rejection + hard-coded sidecar path) bind to security plan Vectors 1/2/3/4/6

## Acceptance criteria contributions
1. (tests) `agent-run.sh` / `agent-run.ps1` expose boot/run/status/cleanup/logs commands with consistent dispatch + usage; smoke-runnable headlessly (exit 0) on both shells per scope definition-of-done
2. (tests) GitHub Actions workflow wires build + nextest(ci) + clippy(-D warnings) + cargo test --doc + cargo-audit + cargo-deny + cargo-llvm-cov + frontend npm audit/vite build, green on current tree per scope definition-of-done
3. (tests) Coverage measured via cargo-llvm-cov; baseline ≥60% line coverage gating per test-plan §10 Quality Gates (threshold enforced in Epoch 10 quality-gate-config phase, measured here as forward declaration)
4. (tests) Zero nextest retries configured in ci profile; flaky-test quarantine mandate active per test-plan §10 Zero-flakiness budget

## Relevant amendment history
- 2026-06-15-structured-logging-stack (§3 Log format): self-obs stream (stderr/logs/agent-latest.jsonl; obs-plan §3 fields) is SEPARATE from per-run emission journal (runs/<run_id>.jsonl; this §3 envelope) — no field additions to the envelope, clarifying binding only
- 2026-06-15-design-token-typography-bundle (§4 Unit Test Strategy): conductor-tauri/ui frontend carries no Rust/nextest unit tests; build-gated (tsc + vite build + npm audit + vite preview); webview E2E coverage deferred to Epoch 9 — no harness change, routine Foundation-epoch deferral per playbook
- 2026-06-16-test-framework-fixtures-coverage-tooling (§4 Unit Test Strategy): external-CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors (outside Cargo.lock; any green-running install satisfies) per cargo-audit/deny precedent; crate dev-deps caret-resolved with Cargo.lock authoritative — routine D-tests-framework warning on version mismatch (chunk resolved different versions while gates green); cascaded to .claude/docs