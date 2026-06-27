# tests extract

## Relevance
Relevant — directly addresses CI quality-gate infrastructure for coverage, flakiness, test reporting, and artifact upload.

## Constraints
1. Per test-plan §9: GitHub Actions platform required; `dorny/test-reporter` consumes JUnit XML into PR annotations
2. Per test-plan §9: nextest `ci` profile MUST emit JUnit XML (via `.config/nextest.toml` `[profile.ci.junit]`)
3. Per test-plan §10: Line coverage threshold ≥ 60% (Minimal tier), enforced by cargo-llvm-cov 0.8.7 `--fail-under-lines 60` over workspace
4. Per test-plan §10: Zero-flakiness budget — NO retries > 0; flakes surface as hard failures, never masked by retry policies
5. Per test-plan §9: Build fails if coverage falls below threshold, OR any test fails, OR lint/clippy/fmt fails, OR supply-chain audit non-green, OR `Cargo.lock` drifts
6. Per test-plan §10: Toolchain ≥ 1.94.1; `tauri` ≥ 2.10.3; committed `Cargo.lock` pinned
7. Per test-plan §10: Exclude generated code (tonic/prost), rmcp/tauri stubs, and rstest/insta fixtures from coverage roll-up

## Patterns to follow
1. Per test-plan §9: `cargo llvm-cov nextest --lcov --output-path lcov.info` + `--cobertura coverage.xml` for LCOV/Cobertura output
2. Per test-plan §9: Upload coverage report + JUnit XML as GitHub Actions build artifacts (`actions/upload-artifact`), establishing reusable pattern for downstream obs/a11y gates
3. Per test-plan §4: cargo-llvm-cov 0.8.7 is a reference floor (outside `Cargo.lock`; any green-running install satisfies the gate, per cargo-audit/deny precedent)

## Anti-patterns to avoid
1. Per test-plan §10: Do NOT set cargo-nextest `retries` > 0 — all failures are real; quarantine + fix, never retry-mask
2. Per test-plan §9: Do NOT allow CI to pass if coverage < 60% (line gate is binding)
3. Per test-plan §9: Verify zero-retry assertion at CI configuration level; do not rely on local `.config/nextest.toml` alone

## Contract bindings
Obs CI conformance gate (separate downstream chunk) reuses the artifact-upload pattern + JUnit consumption pattern established here; A11y CI gate similarly extends this scaffold.

## Acceptance criteria contributions
1. (quality) CI fails when line coverage < 60%; passes at/above it.
2. (quality) nextest `ci` profile emits JUnit XML and remains zero-retry; a flake surfaces as hard failure.
3. (quality) Coverage report (LCOV/Cobertura) + JUnit XML uploaded as GitHub Actions artifacts on every run.
4. (quality) No change to local `agent-run.{sh,ps1} run` green status; `cargo audit` + `cargo deny` remain green; zero engine/seam diff.

## Relevant amendment history
2026-06-16-test-framework-fixtures-coverage-tooling: cargo-llvm-cov 0.8.7 registered as reference floor (external-CLI tool, outside `Cargo.lock`, per cargo-audit/deny precedent); any green-running install satisfies the coverage gate, floors not exact pins.
