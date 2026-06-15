# security extract

## Relevance
Partial — chunk defines data-type vocabulary with serde contracts; security applies to serde deserialization boundaries and error handling, not to behavioral intent.

## Constraints

1. All serde-derived structs in this chunk (Verdict, ReportState, Scenario) MUST support canonical PascalCase serialization names (`"Pass"`, `"Fail"`, `"CalibrationRegion"`, etc.) to match the run-report envelope contract — serde round-trip stable (security plan §Threat Model Summary data classification + §Standard Contracts; prevents misalignment downstream that could leak internal names).

2. Error handling: `Verdict` and `ReportState` are NEVER error variants — they route through `Ok(...)` only; the verdict/error wall reserves `Result::Err` strictly for conductor-core's own `thiserror` harness faults (security plan §Error Handling, §Established Decisions; prevents confusion between verification outcomes and failures).

3. `Scenario` struct MUST include a non-optional P-ID field — the "no scenario without a P-ID" architecture law becomes a compile-enforced invariant (security plan §Threat Model Summary attack surface + §Architecture constraint).

4. Status-rendering contract: both `Verdict` and `ReportState` expose a text label + ASCII status-prefix accessor (`[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]`) — "status is never color-alone" as a type property, not a call-site discipline (security plan §Error Handling surface; CLAUDE.md universal invariant).

5. The conductor-core `thiserror` error enum for harness faults (config-parse class and shared core error variants) must be defined — establishes the compile-enforced verdict/error wall pattern that every downstream seam crate matches on (security plan §Error Handling + §Established Decisions "Verdict/error wall").

## Patterns to follow

1. Serde derives on all type-vocabulary structs; canonicalize PascalCase names at the type level via `#[serde(rename)]` so downstream consumers inherit the serialization contract without repeating it.

2. Per security plan §Error Handling: bounded error detail — internal enum variants (`EmitError`, `VerifyError`, etc.) stay within seam crates as typed `thiserror` enums; conductor-core's own error type holds only harness-class faults; type-erased `anyhow` appears only at the CLI/Tauri edges.

3. Non-optional P-ID field on `Scenario` enforces the "no scenario without a P-ID" invariant at construction time, preventing downstream bugs that could silently widen the threat model (security plan §Threat Model Summary attack surface).

## Anti-patterns to avoid

1. NEVER serialize variant names in non-canonical form (snake_case, CamelCase) — stick to the serde contract's exact PascalCase names so run-report artifacts remain stable across tool versions and prevent internal struct names leaking to agent/operator surfaces (security plan §Error Handling sanitization + §Logging & Monitoring anti-pattern).

2. NEVER return `Verdict` or `ReportState` as error variants — the verdict/error wall is a type-level guarantee; conflating verification outcomes with harness failures corrupts run classification and obscures the root cause (security plan §Established Decisions "Verdict/error wall"; prevents panics on read-back).

## Contract bindings

- **verdict/error wall** ↔ downstream seam crates (`timeline`, `emit`, `faults`, `verify`, `report`, `cli`, `tauri`): each seam's typed error enum collapses only at the CLI/Tauri boundaries; verification outcomes stay as `Ok(...)` values.
- **run-report envelope** ↔ `conductor-report` (Epoch 6): the `Verdict` and `ReportState` enums serialized here become the canonical keys in the `<run_id>.md` / `runs.db` rows downstream.
- **Scenario identity** ↔ `conductor-timeline` (Epoch 2): this chunk defines the skeleton; Epoch 2 adds per-phase emission body + seeded phase scheduler.

## Acceptance criteria contributions

1. (security) `Verdict` and `ReportState` enums defined with exact variants (`Pass`, `Fail`, `CalibrationRegion` / `Pass`, `Fail`, `ManualCheck`, `KnownResidual`, `Blocked`) and serde-derive with canonical PascalCase names — grep verifies: `#[serde(rename = "Pass")]` pattern present.

2. (security) Both enums expose a text label + ASCII status-prefix accessor; `cargo test conductor_core::verdicts::test_display_*` green.

3. (security) `Scenario` struct contains a non-optional P-ID field (`p_id` or `pulse_id`); grep verifies `p_id: .*P-.*ID.*` pattern (no `Option<>`).

4. (security) conductor-core's own `thiserror` error enum defined; `Result<T>` bounds are enforced — verification outcomes never route through `Err`; `cargo clippy -p conductor-core` green with no verdict-as-error warnings.

## Relevant amendment history

(none) — This is a fresh plan with no prior amendments to this chunk's area.
