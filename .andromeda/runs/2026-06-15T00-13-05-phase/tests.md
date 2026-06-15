# tests extract

## Relevance
Relevant — the chunk is pure types with required serde unit tests + contract verification.

## Constraints
- §4 Unit Test Strategy: `cargo nextest run -p conductor-core` must pass for all new unit tests defining the type contracts (serde round-trip, enum variant shape, text-label accessors).
- §2 Test Strategy: Unit tests are the bulk of this chunk's surface; no integration/E2E for pure data types (those come with seams that use them).
- §1 Test Scope Summary: `conductor-core` is "pure Rust library, `cargo test -p conductor-core` per the per-seam test-isolation convention" — testable via direct `cargo nextest`.
- §7 Test Data & Fixtures: rstest `#[case]` table-driven variants over the enum variants (`Pass`/`Fail`/`CalibrationRegion` for `Verdict`; `Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked` for `ReportState`) + serde round-trip assertion fixtures.
- §10 Quality Gates: See coverage threshold (threshold value to be stated in Section 10 of the plan); new-code line coverage ≥ threshold is a hard gate.
- §11 Test Anti-Patterns: No deserializing scenario config without garde validation — garde is NEXT chunk, this chunk is serde-only (type shape contract only, no validation bounds).

## Patterns to follow
- **Serde round-trip pattern** (§7): rstest `#[case]` over the 3 `Verdict` + 5 `ReportState` variants, asserting `serde_json::to_string()` → `serde_json::from_str()` yields the same value + canonical PascalCase serialization match (e.g., `"Pass"`, `"CalibrationRegion"`).
- **Text-label accessor pattern** (scope): each enum variant exposes a `fn label(&self) -> &'static str` and `fn status_prefix(&self) -> &'static str` (e.g., `[PASS]`, `[BLOCKED]`); unit test asserts these are stable across all variants.
- **Error wall unit test** (scope §verdict/error wall): assert that `conductor-core` error enum exists, `Verdict`/`ReportState` are returned as `Ok` values (never error variants), and the wall holds by construction (type system enforces it).

## Anti-patterns to avoid
- Do NOT unit test scenario validation (garde rules are the next chunk); this chunk only asserts serde shape.
- Do NOT test async or I/O behavior (none exists in `conductor-core`).
- Do NOT import `thiserror` error types from other seams; the core error enum is `conductor-core`'s own.

## Contract bindings
- **Serde ↔ run-report envelope** (obs binding): Both enums must serialize to the exact canonical names the envelope specifies (`"verdict": "Pass"`, `"state": "Pass"`, etc.); this plan defines the serialized shape, obs derives the JSON envelope FROM this definition.
- **Text labels ↔ CLI design contract** (design-system binding): Text labels + ASCII prefixes (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) are the "status is never color-alone" invariant enforcement point; the cli surface depends on these accessors.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes for all new unit tests.
- (tests) Coverage: new-code line coverage of `conductor-core` ≥ {threshold from §10 Quality Gates}.
- (tests) Serde round-trip: `Verdict` and `ReportState` serialize to exact PascalCase names and deserialize identically.
- (tests) Text labels: each enum variant's `label()` and `status_prefix()` accessors return stable, non-empty values and are asserted in unit tests.

## Relevant amendment history
(none)
