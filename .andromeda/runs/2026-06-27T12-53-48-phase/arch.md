# arch extract

## Relevance
relevant — architecture.md §Infrastructure Patterns/§CI/CD and §Stack and Technologies pin the build/test/coverage toolchain this chunk gates into CI

## Constraints
1. Zero-retry nextest `ci` profile is LOCKED per architecture.md §Infrastructure Patterns (no retries mask flakes — determinism invariant enforced at CI level per §Cross-cutting Patterns)
2. GitHub Actions on dev-OS target runs `cargo build` / `nextest` / `clippy` per §CI/CD approach; dynamic proof stays a local gate (never CI-gated)
3. Line coverage requires `llvm-tools-preview` toolchain component per §Infrastructure Patterns Build system
4. Test suite includes `cargo test --doc` for doctests in addition to nextest per §Infrastructure Patterns
5. The `ensure_frontend` step couples any cargo compile of conductor-tauri to prior `npm run build` (affects CI wiring per §Infrastructure Patterns)
6. No live Pulse in CI per §CI/CD approach (CI has no Pulse instance)
7. Determinism discipline (seeded RNG + current_thread + no wall-clock assumptions) is preserved in all CI tests per §Cross-cutting Patterns

## Patterns to follow
1. Pinned zero-retry nextest `ci` profile in `.config/nextest.toml` (established per architecture.md §Infrastructure Patterns; CI gates enforce it)
2. Artifact-upload scaffold: JUnit XML + coverage report via GitHub Actions (reusable pattern for downstream obs-conformance + a11y gates)
3. Test discipline: no-retry determinism enforced at CI level — flake is a hard failure, never masked by retry

## Anti-patterns to avoid
1. No live Pulse / Linux+xvfb / webview dependency in CI (per scope.md + architecture.md §CI/CD)
2. No wall-clock timing in CI-driven tests (determinism invariant per §Cross-cutting Patterns)
3. No engine/seam code changes; CI YAML + nextest config only (per scope.md boundaries)

## Contract bindings
- §Infrastructure Patterns → Build system: `ensure_frontend` coupling in scripts/agent-run.{sh,ps1} + CI (frontend build-order constraint)
- Downstream obs-conformance gate reuses artifact-upload + gate-step pattern
- Downstream a11y gate reuses artifact-upload + gate-step pattern + CI-on-dev-OS pattern

## Acceptance criteria contributions
1. (arch) CI fails when line coverage drops below pinned floor; passes at/above (enforces test-plan §10 coverage-threshold decision)
2. (arch) nextest `ci` profile emits JUnit XML and remains zero-retry; flake surfaces as hard failure, never masked (enforces determinism invariant per §Cross-cutting Patterns)
3. (arch) Coverage + JUnit uploaded as GitHub Actions artifacts on every run (establishes reusable gate-step + artifact-upload scaffold)
4. (arch) No change to local `agent-run.{sh,ps1} run` green; `cargo audit` + `cargo deny` green; no engine/seam diff (preserves established build discipline)

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** — The test-framework chunk ESTABLISHED nextest zero-retry `ci` profile + cargo-llvm-cov line coverage in §Infrastructure Patterns; this chunk GATES those tools into CI workflow (spec→sound-impl: infrastructure machinery now CI-enforced via config + artifact collection).
