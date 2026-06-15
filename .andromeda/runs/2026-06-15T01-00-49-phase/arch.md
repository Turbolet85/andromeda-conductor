# arch extract

## Relevance
Relevant — config validation is a core seam with explicit surface/pattern rules in this chunk.

## Constraints
- Per arch §Established Decisions [Validation Library]: serde 1.0.x + garde 0.23.0 with `#[derive(Validate)]` on scenario config structs; `range` rules for simple bounds (error fraction ∈ [0,1], non-negative durations, sane ramp factors) + `#[garde(custom = …)]` for cross-field invariants.
- Per arch §Conventions "Config conventions": durations are non-negative; error fractions ∈ [0,1]; ramp factors validated sane; p50≤p95≤p99 ordering and severity-mix sums enforced.
- Per arch §Established Decisions [Error Handling] + §Conventions "Error handling": garde's validation `Report` becomes a `ConfigError` harness-failure class via `#[from]`; validation failure is `Result::Err`, never a `Verdict`/`ReportState`.
- Per arch §Occupied Resources "Environment variables": `CONDUCTOR_*` namespace (`CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`) reserved; precedence files-over-env, CLI-over-env.
- Per arch §Module Boundaries: validation rules co-located with the serde structs in their owning seam crate; path-validation guard lives at the edge (core/CLI boundary).
- Per arch §Cross-cutting Patterns "Config management": local files + env vars only; declarative scenario config files (serde + garde, no DSL); precedence is files over env defaults.
- Per security-plan §Input Validation: validate ALL scenario config at load with garde; `CONDUCTOR_*` path handles sit OUTSIDE garde (`std::fs::canonicalize` + bounds-check at the edge before any path use); the path-traversal guard class.

## Patterns to follow
- The prior chunk (conductor-core-shared-types) left `CoreError` `#[non_exhaustive]` with an explicit extension point for this chunk's garde `Report` `#[from]` variant — extend it as the ConfigError class.
- Workspace path-handle resolution pattern: `std::fs::canonicalize` + bounds-check (rejects traversal / out-of-bounds), unit-tested with a traversal attempt; guard sits outside garde as a reusable helper.
- Cross-field validator worked example: struct-level `#[garde(custom = …)]` custom-validator function over the current identity fields (establish the pattern for Epoch-2 emission-spec rules to extend).

## Anti-patterns to avoid
- No async validation (garde and path guards are sync).
- No scenario-file I/O beyond `canonicalize` (loader pipeline is later; this chunk validates in-memory deserialized struct only).
- No clap/CLI-arg parsing (CONDUCTOR_* are env handles; CLI-flag precedence is an Epoch-8 chunk).

## Contract bindings
- security-plan §Input Validation — scenario config validation at load + path-handle canonicalize/bounds-check; binds across validation scope.
- Prior chunk's CoreError extension point — this chunk fills the `#[non_exhaustive]` explicit extension with the garde Report `#[from]` variant.

## Acceptance criteria contributions
- (arch) `Scenario`/`PId` derive `garde::Validate`; non-empty `p_ids` enforced as `length(min=1)` rule; P-ID format/range rules (P-NNN, 001..060) enforced; `.validate(&())` rejects empty/malformed scenarios.
- (arch) Cross-field `#[garde(custom)]` struct-level validator pattern established over current identity fields; worked example on a valid invariant.
- (arch) `CoreError` gains `#[from] garde::Report` variant as ConfigError class; validation failure surfaces as `Result::Err`, never a Verdict/ReportState.
- (arch) `CONDUCTOR_*` path-handle resolver canonicalizes + bounds-checks; unit test includes traversal-attempt rejection; reusable guard pattern at core/CLI edge.

## Relevant amendment history
2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1 (security CVE-2026-33056 tar-rs); cascades to all occurrences in Stack, Infrastructure, Inherited Defaults sections.