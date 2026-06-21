# tests extract

## Relevance
partial — coverage-matrix is a static classification generator with a golden-tested artifact, not a scenario runner or coverage-test suite.

## Constraints
- Per §1 Scope Summary, the coverage-matrix is a "testable entity" (partially-testable: "completeness/zero-gaps and per-scenario P-ID keying are statically assertable; a missing P-XXX is a defect — Creator Brief") — zero gaps is a hard gate, not calibration (§1 critical paths: "the generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries; a missing P-XXX fails the gate").
- Per §3 Test Harness Contract / Log format, the artifact path is project-root `coverage-matrix.md` (kebab-case Markdown, consistent with run-report naming §1 test-scope, scope.md artifact-path note).
- Per §4 Unit Test Strategy, the render is pure/clock-free → exact-string golden test (per the canonical-line-shape pattern established in conductor-report's `verdict.rs`/`report_state.rs` serialization goldens; amendment 2026-06-16 clarifies exact-string `assert_eq!` at unit level for canonical schemas, insta reserved for E2E).
- Per §1 Coverage triggers, supply-chain audit gate applies — `cargo-audit` + `cargo-deny` green (Vector 1 threat model includes CLI input; matrix generation involves config loading).
- Per scope.md definition of done: "All 60 P-IDs emitted, zero unclassified/missing — completeness asserted by test"; "Pure/clock-free render → exact-string golden test; artifact write is loud-never-overwrite-safe consistent with the run-report seam."

## Patterns to follow
- Golden-test serialization via exact-string `assert_eq!` (matching conductor-report's canonical-line pattern from amendment 2026-06-16) — capture the fixed 60-row classification table as a committed snapshot, assert round-trip equality.
- Deterministic seeded synthetic fixture generation via rstest `#[fixture]` (per §3 test-data bootstrap and §4 unit-test conventions) — the classification data model (P-ID → mode enum) is code-native, not developer-seeded; validation fixture matrix covers mode enum completeness.
- Per-seam crate isolation: generator unit tests in `conductor-core` (if classification model lives there per scope.md crate-split note) or `conductor-report` (if render home); `cargo nextest run -p <crate>` per-seam gate.

## Anti-patterns to avoid
- No multi-line schema in the golden (JSON must flatten to one line per §3 / amendment 2026-06-15 JSONL clarification; Markdown fixture file golden is acceptable but assert its content, not render it).
- No missing P-ID or unclassified entry in the rendered artifact — a defect per Creator Brief (§1 critical paths, scope.md definition of done); assert count == 60 + assert all enums assigned (not None/default).
- No re-derived lamp precedence — scope.md open question (c) names this chunk a `Lamp::for_record` reuser (if a status column is included); if implemented, call the existing lamp logic, do not duplicate (per scope.md surface contracts).

## Contract bindings
obs ↔ tests harness: the artifact-sanitization discipline (§3 Log format, no absolute host paths or internal struct names) applies if a live-status column is rendered later (scope.md open question 1); the generator's role is classification table only at Epoch 6, status column is Epoch 8/9 concern.

## Acceptance criteria contributions
- "(tests) `cargo nextest run -p {crate}` passes for new tests."
- "(tests) Coverage-matrix completeness: all 60 P-IDs (P-001..P-060) present and classified into exactly one of {auto, drive+observe, static-only}, zero unclassified/missing; asserted by unit test."
- "(tests) Artifact golden: `coverage-matrix.md` render is exact-string golden-locked via `assert_eq!`; rebuild produces byte-identical output (deterministic, clock-free)."
- "(tests) Artifact write safety: `coverage-matrix.md` write is loud-never-overwrite-safe, consistent with run-report seam (no silent truncation; idempotent re-run produces same artifact)."

## Relevant amendment history
- **2026-06-16-emission-journal-writer** (§4 Unit Test Strategy) — conductor-report's golden-test pattern uses exact-string `assert_eq!` at unit level for canonical serialization, with insta reserved for E2E mechanisms. Applies: the coverage-matrix render golden should follow the same exact-string pattern (no insta adoption for this artifact).
- **2026-06-16-test-framework-fixtures-coverage-tooling** (§4 Unit Test Strategy) — external-CLI tool versions (cargo-nextest, cargo-llvm-cov) are reference floors, not exact pins; applies to the matrix generator's test gate (cargo nextest 0.9.137 is a floor, any green-running install satisfies). Cascaded to testing.md / tests-summary.md.
