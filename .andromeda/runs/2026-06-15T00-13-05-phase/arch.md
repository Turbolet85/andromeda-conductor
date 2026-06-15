# arch extract

## Relevance
relevant

## Constraints
- §Design Philosophy "Outcomes are values, errors are harness faults" — `Verdict`/`ReportState` are typed return values; `Result::Err` reserved for Conductor's own failures (arch §Error Handling)
- §Established Decisions [Error Handling] "typed per-seam error enums" + "verdict/error wall" — verification outcomes are `Ok(Verdict)` / `Ok(ReportState)`; harness errors are `Result::Err` (arch §Verdict/error wall)
- §Conventions "Naming patterns" — snake_case Rust identifiers, PascalCase serde serialization (arch §Serde representation)
- §Standard Contracts "Run report envelope" — `verdict ∈ {Pass, Fail, CalibrationRegion}`, `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`, serialized as canonical PascalCase (arch §Standard Contracts)
- §Conventions "Config conventions" — `SloTier` closed enum `<5s`/`<20s`/`<90s`, serde-renamed (arch §Data model conventions)
- §Established Decisions [Probabilistic-Assertion Policy] — deterministic claims hard pass/fail; model-interpretive claims → `CalibrationRegion` (arch §Probabilistic-Assertion Policy)
- §Established Decisions [Read-Back Dependency Posture] — five `ReportState` meanings: `Pass` / `Fail` / `ManualCheck` (operator-checklist terminal) / `KnownResidual` (pre-accepted residual) / `Blocked` (failed preflight precondition) (arch §Read-Back Dependency Posture)

## Patterns to follow
- Verdict/error wall applied uniformly: `enum Verdict { Pass, Fail, CalibrationRegion }` and `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` returned as `Ok(...)` (arch §Conventions "Error handling")
- Status-rendering contract: each enum state exposes text label + ASCII status-prefix accessor (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` for CLI) so status is never color-alone (scope §Status-rendering contract)
- serde round-trip with exact canonical names: `Verdict::Pass` → `"Pass"`, `ReportState::ManualCheck` → `"ManualCheck"` (arch §Standard Contracts)

## Anti-patterns to avoid
- DO NOT embed verification outcomes in `Result::Err` — `Verdict` and `ReportState` are not error variants (arch §Verdict/error wall)
- DO NOT make `SloTier` derive string-keyed — use serde `rename` to map `<5s`/`<20s`/`<90s` wire names to Rust identifiers (arch §Data model conventions)
- DO NOT add non-core error types or per-seam errors to `conductor-core` — seam errors (`EmitError`, `VerifyError`, …) arrive with their own seams; core owns harness faults only (scope §Boundaries)

## Contract bindings
- **tests harness** ↔ `Verdict`/`ReportState` enums: E2E fixtures serialize/deserialize run-report envelope; golden tests match canonical state names
- **obs (self-observation)** ↔ core error type: harness faults route to tracing 0.1.44 structured logs; error logging binds to `conductor-core::error::*` types (per obs-plan §3)
- **report seam** (Epoch 6) ↔ `Verdict`/`ReportState` + `Scenario` identity: run-report envelope embeds these enums; row serialization uses their serde representations

## Acceptance criteria contributions
- (arch) `conductor-core` defines `Verdict { Pass, Fail, CalibrationRegion }` and `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` with `Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize` (arch §Standard Contracts run-report envelope).
- (arch) Both enums serialize to canonical PascalCase names (`"Pass"`, `"Fail"`, `"CalibrationRegion"`, `"ManualCheck"`, `"KnownResidual"`, `"Blocked"`); serde round-trips verified in unit tests.
- (arch) Each enum exposes a text label accessor and ASCII status-prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) per CLAUDE.md "Status is never color-alone" contract (scope §Status-rendering contract).
- (arch) `Scenario` struct carries non-optional P-ID field, seed, name, `SloTier` (`<5s`/`<20s`/`<90s`, serde-renamed); derives `Serialize, Deserialize` only (scope §scenario model).
- (arch) Core error enum (`ConductorError` or similar) holds harness-fault variants; `Verdict`/`ReportState` are NOT error variants (arch §Verdict/error wall).
- (arch) `cargo build -p conductor-core` and `cargo test -p conductor-core` pass, including serde round-trip tests (scope §Acceptance hints).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold** — MSRV raised 1.88.0 → 1.94.1 (§Stack and Technologies); self-observation stack row (`tracing 0.1.44 + tracing-subscriber 0.3.23`) added (§Stack and Technologies). Both apply to `conductor-core` workspace seam and any dependencies this chunk establishes.
