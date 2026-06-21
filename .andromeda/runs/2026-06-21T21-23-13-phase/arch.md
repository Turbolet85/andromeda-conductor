# arch extract

## Relevance
Partial — the chunk defines the static coverage classification (mode per P-ID) and Markdown render; it does NOT implement scenarios, per-scenario verification, or live status integration, which remain domain constraints for later chunks.

## Constraints
1. Coverage model and artifact placement per §Inherited Defaults — crate-per-seam workspace; likely `conductor-core` (classification table) + `conductor-report` (Markdown render/write).
2. All 60 P-IDs (P-001..P-060) enumerated with zero gaps, each classified into exactly one of {auto, drive+observe, static-only} per §Established Decisions [Scenario Config Format] + scope §Classification source of truth (input.md §Coverage classification).
3. Artifact path `coverage-matrix.md` at project root per §Occupied Resources (On-disk artifacts) and §Infrastructure Patterns (directory structure crate comment).
4. Lamp precedence reuse (not re-derive) where verdict/report-state status is shown, per scope §Definition of done — binds to §Standard Contracts (Run report envelope) and §Established Decisions [Probabilistic-Assertion Policy] via `Verdict::default_report_state` (verdict-first rendering).
5. Render is pure/clock-free per scope §Definition of done (golden-testable), mirroring `RunReport::render` seam pattern — no tokio runtime / wall-clock dependency.
6. Module boundary: no cross-seam forbidden dependencies per §Compiler-enforced module seams; if classification lands in `conductor-core`, only serde + garde + thiserror (no emit/verify/report crates).

## Patterns to follow
1. Declarative model + serde de/serialization (P-ID → mode) colocated with seam ownership per §Conventions (Config conventions) — mirroring the scenario config model (toml 0.9 serde + garde validation).
2. Markdown artifact render as a pure function (`::render() → String`) testable via golden/insta per §Infrastructure Patterns (test-framework stack: rstest/insta/assert_cmd).
3. Verdict/error wall: classification correctness errors are harness-level typed returns (not panics) per §Conventions (Error handling).

## Anti-patterns to avoid
1. Do NOT include live per-P-ID status fetch or `runs.db` reads in the classification model — that belongs to later Epoch 8/9 CLI/desktop overlays, not the static matrix.
2. Do NOT re-derive lamp precedence logic — reuse `Lamp::for_record` per §Established Decisions [Probabilistic-Assertion Policy] and scope follow-up (c).

## Contract bindings
- **Verdict/ReportState**: binds to §Established Decisions [Probabilistic-Assertion Policy] + amendment 2026-06-21 (ManualCheck widened, default `Verdict::default_report_state` mapping) — Lamp renders MUST cite verdict-first precedence.
- **MCP standard envelope**: binds to §Standard Contracts (Readiness gate + Run report envelope) — blocked/pass/fail/manual-check/known-residual states must be documented as the rendering options.
- **Crate ownership**: binds to all domains — if `conductor-core` owns classification, all other crates depend on it; if `conductor-report` owns render, it depends on `conductor-core`.

## Acceptance criteria contributions
1. (arch) All 60 P-IDs (P-001..P-060) enumerated in the coverage model, zero gaps, each classified into exactly one of {auto, drive+observe, static-only} — asserted by unit test per scope §Definition of done.
2. (arch) Code lives in `conductor-core` (classification) + `conductor-report` (Markdown render + write) per workspace boundary rules (§Inherited Defaults § Crate-per-seam Cargo workspace).
3. (arch) Artifact written to project-root `coverage-matrix.md` per §Occupied Resources (On-disk artifacts) and §Infrastructure Patterns (directory structure).
4. (arch) Render is pure/clock-free (`fn render(&self) -> String`), testable via golden insta; no tokio runtime or wall-clock dependency per §Cross-cutting Patterns (Determinism discipline).
5. (arch) `Lamp::for_record` reused (not re-derived) for any status/verdict rendering; verdict-first precedence per amendment 2026-06-21.

## Relevant amendment history
- **2026-06-21-run-report-envelope-serializer** — ManualCheck broadened to include auto-measured calibration-region checks; `Verdict::default_report_state` default mapping (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`) recorded; verdict-first lamp precedence resolves a11y lamp conflict. Affects how coverage matrix (or later status columns) render Verdict/ReportState pairs.
