# tests extract

## Relevance
Partial — this chunk (line-oriented output rendering) adds a presentation/render layer that reads existing test-typed outputs but does not add new test logic, timelines, or verification mechanics.

## Constraints
- per test-plan §1 (Scope Summary) — test tier is Minimal (0); coverage scope includes `conductor-cli` and reads `conductor-core`/`conductor-report`/`conductor-verify` as dependencies only
- per test-plan §3 (Test Harness Contract) — the `run` command already produces the Run-report envelope (verdict/state/latency_ms fields); this chunk renders those existing typed outputs without altering the envelope contract
- per test-plan §4 (Unit Test Strategy) — unit framework is cargo-nextest 0.9.137; the render module reads seam-exported types only (no internal struct testing); error-handling surface uses thiserror/anyhow and must not leak internal details
- per test-plan §6 (E2E Test Strategy) — cli surface signal already includes stdout labels `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` paired with text (color-only a11y rule); render module preserves this invariant
- per test-plan §11 Test Anti-Patterns (Unit) — NEVER test implementation details (private render fns); test only the public cli-verb output contracts
- per test-plan §11 Test Anti-Patterns (CI) — NEVER merge without `cargo audit --deny warnings` + `cargo deny check` green + committed un-drifted `Cargo.lock` (three new rendering crates)

## Patterns to follow
- per test-plan §4 (Conventions) — test file location: crate-local `#[cfg(test)] mod tests` inside `conductor-cli` render module; test function naming: snake_case `fn` under `#[test]`/`#[tokio::test]`
- per test-plan §7 (Fixtures) — use rstest 0.26.1 `#[fixture]` for seeded `conductor-timeline` generator + declarative scenario config one-per-P-ID; the `RunRecord` fixture (per test-plan §4 conductor-report entity) already exists as a golden-locked serialization contract
- per scope.md (Acceptance intent) — golden tests assert: status lines render with ASCII prefix ALWAYS present + color-strippable (non-tty / `NO_COLOR`); coverage table renders all 60 P-IDs (zero gaps) with mode + lamp columns; piped output contains no raw escape sequences

## Anti-patterns to avoid
- NEVER test that color codes are present in stdout — instead test the STRIPPED output is correct + verify color is only added when tty-gated (owo-colors auto-detection disables in pipes)
- NEVER duplicate the Markdown lamp-mapping logic (verdict→lamp precedence) in the render tests — the existing conductor-report golden tests already lock `verdict.rs`/`report_state.rs`/`Lamp` serialization; render tests mock the envelope, not re-verify the mapping
- NEVER add new verdict/report-state variants or emission logic — this is a render-only chunk; all test logic reads the existing Run-report envelope from conductor-core/conductor-report

## Contract bindings
- **render ↔ conductor-report envelope** — the render module reads the existing `RunRecord`, `Verdict`, `ReportState`, `Lamp` types; integration test asserts the render output matches the Markdown report's row layout (both render the same envelope)
- **render ↔ a11y (color-only rule)** — per scope.md + test-plan §6, the ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`) is ALWAYS present; unit test asserts stripping color via `NO_COLOR` env var leaves the prefix + text label intact

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-cli` passes for new render-module tests.
- (tests) Status-line unit tests assert: `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` prefix is present in both TTY and non-TTY output; color is only added when tty-gated.
- (tests) Coverage-table unit tests assert: all 60 P-IDs (P-001..P-060) render with zero gaps; table rows include mode (auto / drive+observe / static-only) + lamp columns.
- (tests) Render integration test asserts the cli output (via assert_cmd subprocess) matches the existing Markdown report row shape for the same `RunRecord` envelope.

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle:** conductor-tauri/ui (React frontend) carries no Rust/nextest unit tests; build-gated only (`tsc --noEmit` + `vite build`). Frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9. _Relevant:_ this chunk (line-oriented CLI render) is the Rust-side counterpart; no new frontend unit tests expected here, but ensure the CLI surface render tests (cargo-nextest) are comprehensive.