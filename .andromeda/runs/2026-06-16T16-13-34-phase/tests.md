# tests extract

## Relevance
relevant — this chunk IS the test-framework / test-harness / fixtures bootstrap (all three foundation pillars).

## Constraints
- Test harness MUST be agent-driven headless per test-plan §3 Test Harness Contract (5-command discipline: boot/run/status/cleanup/logs; Conductor has no daemon, no inbound listener).
- Determinism invariant: `ci` profile MUST enforce `retries = 0` (per test-plan §2 agent-runnable invariants; "same scenario+seed ⇒ same stream shape" — retries mask flakes).
- Output MUST be machine-parseable: cargo-nextest `ci` profile JUnit XML (test-plan §3 run command + §9 CI integration), JSONL journal (test-plan §3 Log format), `serde_json` from envelope (test-plan §3 Status endpoint).
- Bootstrap phases (test-plan §3 Bootstrap phases) materialize in sequence: test-runner-install (nextest 0.9.137 + cargo-llvm-cov 0.8.7) → 5-command-discipline-wire → status-endpoint-implement (Run-report envelope, not HTTP endpoint) → log-format-bind-with-obs (JSONL per §3 Log format; distinct from obs self-obs stream per 2026-06-15-structured-logging-stack amendment) → test-data-bootstrap-wire (rstest fixtures) → coverage-tooling-install.
- Dev-dependencies (test-plan §4 Framework) must be: cargo-nextest 0.9.137, rstest 0.26.1, proptest 1.9.0, insta 1.x (`json` feature), assert_cmd 2, assert_fs 1, cargo-llvm-cov 0.8.7; no JS/TS unit runner adopted (test-plan §4 amendment 2026-06-15-design-token-typography-bundle).

## Patterns to follow
- Per-seam crate-local `#[cfg(test)] mod tests` + crate-local `tests/` for integration (test-plan §4 Conventions); `cargo nextest run -p conductor-<seam>` isolates per-seam.
- Fixture pattern: rstest `#[fixture]` for seeded `conductor-timeline` generator, rstest `#[case]` for table-driven P-ID catalog / garde-config matrices, `#[tokio::test(flavor="current_thread", start_paused=true)]` + `tokio::time::advance` (test-plan §4 Fixture pattern, §2 Agent-runnable invariants).
- Golden snapshots committed under `<crate>/tests/snapshots/` (insta default); proptest regressions under `proptest-regressions/` (test-plan §2 Deterministic invariant, §4 Framework).

## Anti-patterns to avoid
- No interactive `cargo insta review` in CI (test-plan §2 Agent-runnable invariants: insta runs in CI/assert mode only).
- No developer-seeded DB; only deterministic seeded synthetic generation via `conductor-timeline` (test-plan §3 Test data bootstrap).
- No retries on flaky tests (determinism invariant: retries=0 in `ci` profile).
- No frontend Rust unit tests; React/Tailwind is build-gated only (`tsc --noEmit`, `vite build`, `npm audit`); webview E2E deferred to Epoch 9 (test-plan §4 amendment 2026-06-15-design-token-typography-bundle).

## Contract bindings
- **5-command discipline** ↔ obs §3: boot/run/status/cleanup/logs contract; status returns Run-report envelope (arch Standard Contracts), logs read `runs/<run_id>.jsonl` per test-plan §3.
- **Log format** ↔ obs §Log format binding: per-run JSONL schema defined in test-plan §3 Log format is the source of truth; obs derives from here; the `tracing` self-obs stream (stderr / `logs/agent-latest.jsonl`) is a SEPARATE artifact (2026-06-15-structured-logging-stack amendment).
- **Status endpoint** ↔ obs §Status endpoint shape: no HTTP listener; polled by reading `runs.db` row / JSONL journal per test-plan §3 Status endpoint shape; envelope fields (`run_id`, `verdict`, `state`, `latency_ms`, etc.) shared across all artifacts.
- **CI gate** ↔ route §CI Integration: cargo-audit + cargo-deny green; `Cargo.lock` committed; toolchain ≥ 1.94.1; `tauri` ≥ 2.10.3 (test-plan §1 Coverage triggers, §10 Quality Gates).

## Acceptance criteria contributions
- (tests) `cargo nextest run --workspace --profile ci` runs green (no missing `ci` profile error) — scope def-of-done val-1 anchor (test-plan §3).
- (tests) Each tool exemplar passes green: rstest, proptest, insta, assert_cmd/assert_fs, cargo-llvm-cov — scope def-of-done (test-plan §4 Framework).
- (tests) Determinism invariant: `ci` profile enforces `retries = 0` — no flakiness masking (test-plan §2 Agent-runnable invariants).
- (tests) Machine-parseable output: cargo-nextest JUnit XML works; insta runs in assert mode (non-interactive) (test-plan §3 run command output format + §9 CI integration).
- (tests) `cargo-audit` + `cargo-deny` green; `Cargo.lock` committed + un-drifted; `cargo clippy -D warnings` clean (test-plan §1 supply-chain-audit + §10 Quality Gates).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** noted that `tracing` self-obs stream (stderr, base fields incl. service-identity + `run_id`) is SEPARATE from per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth + Run-report envelope); the two schemas must not be conflated. Affects: test-plan §3 Log format binding to obs — the envelope this chunk's test data carries does NOT include `service.*` fields (those belong to the self-obs stream, obs-plan §3).
- **2026-06-15-design-token-typography-bundle:** frontend (conductor-tauri/ui) tests are build-gated (`tsc --noEmit`, `vite build`, `npm audit`, `vite preview` render smoke), no Rust/nextest unit tests; webview E2E via tauri-driver deferred to Epoch 9. Affects: what exemplar tests this chunk writes — no React/TS/npm setup needed; the bundle is out-of-scope.