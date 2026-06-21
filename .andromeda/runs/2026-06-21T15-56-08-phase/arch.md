# arch extract

## Relevance
Partial — SLO timing model aligns with core architecture; comparison kinds are new evaluation logic outside determinism/module seams.

## Constraints
- §Established Decisions (Timing-Tolerance Model): tier-scaled bounded tolerance `<5s`/`<20s`/`<90s`; latency is always `read_back_observed_at − journal_emitted_at` from `std::time`, never virtual clock (§Cross-cutting Patterns determinism discipline)
- §Occupied Resources: reserved `CONDUCTOR_*` env namespace; new surfaces must not overflow it
- §Conventions (Error handling): verification outcomes are typed values (`Verdict`/`ReportState`/`Assessment`); `Result::Err` reserved for harness faults only; verdict/error wall maintained
- §Standard Contracts (Run report envelope): the envelope already defines `slo_tier` ∈ `{<5s, <20s, <90s}` closed set and `verdict ∈ {Pass, Fail, CalibrationRegion}` (return value); the model produces these values
- §Established Decisions (Probabilistic-Assertion Policy): deterministic claims hard pass/fail; model-interpretive + sample-count-floor claims route to `CalibrationRegion`, never hard-fail
- §Module Boundaries (Crate-per-seam): the comparison evaluator and SLO timing logic live in `conductor-verify` (the verification seam) per star-topology (seam crates import only `conductor_core`); config data model placement (core vs. verify) is the open seam question

## Patterns to follow
- Declarative scenario config (serde + garde) — the `expected` block follows established discipline: deserialized at load, guard-validated per §Conventions, cross-field invariants via garde `Context` (precedent: scenario-config-model amendment, 2026-06-16)
- Infallible evaluator — comparison logic and SLO timing return typed values (e.g., `Assessment`/`SloOutcome`), not `Result`; malformed `expected` config is a load-time `ConfigError` / `VerifyError`, not a runtime evaluation fault
- Journal-relative timing — all clock reads via `std::time::{SystemTime, Instant}` as established; no tokio virtual-clock leakage into SLO math
- Type-safe tiers — `SloTier` enum over exactly the three closed values; no string/int casting that could diverge from the run-report envelope

## Anti-patterns to avoid
- No host wall-clock timestamps in latency math — `read_back_observed_at` and `journal_emitted_at` must use the same ground-truth (JSONL journal offsets), eliminating clock-skew faults
- No hard-fail on model-interpretive or sample-floor claims — `CalibrationRegion` is the prescribed route, distinguishing from deterministic claims that must be hard pass/fail
- No new env vars outside `CONDUCTOR_*` namespace — precedent: 2026-06-15 amendment adding `CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV` via the centralized registry

## Contract bindings
- **obs ↔ verify** — SLO tier outcome feeds the run-report serializer (Epoch 6) and `runs.db` writer; tracing spans at the evaluator level should log tier + latency outcome per §Self-observation
- **tests ↔ verify** — determinism acceptance tests verify same scenario + same observed + same timing ⇒ identical assessment; golden-fixture tests exercise all three comparison kinds and all three tiers
- **core ↔ verify** — config data model placement (open question): if `expected` block lives in core, verify depends on core types; if wholly in verify, core remains decoupled (the star-topology preference)

## Acceptance criteria contributions
- (arch) Comparison evaluator lives in `conductor-verify` per crate-per-seam (arch §Module Boundaries).
- (arch) `SloTier` enum over exactly `{<5s, <20s, <90s}` (arch §Standard Contracts); no overflow, no new string variants.
- (arch) Latency always `read_back_observed_at − journal_emitted_at` from `std::time`, stored/logged as integer milliseconds (arch §Established Decisions Timing-Tolerance Model).
- (arch) Deterministic comparison/deadline ⇒ hard `Pass`/`Fail`; model-interpretive / sample-floor miss ⇒ `CalibrationRegion`, never hard-fail (arch §Established Decisions Probabilistic-Assertion Policy).
- (arch) `expected` block deserializable + garde-validated at load, raising `ConfigError` / `VerifyError` on parse/bounds failure (arch §Conventions).

## Relevant amendment history
- **2026-06-16-scenario-config-model** — toml 0.9 + garde 0.22.1 registered in §Stack; `Scenario::from_toml_str` → `CoreError::Config`/`::Validation` precedent established. This chunk's `expected` block follows identical serde+garde discipline.
- **2026-06-15-config-validation-surface** — garde pinned 0.22.1 (was 0.23.0); cross-field invariants use `Context` pattern (field-level custom). The `expected` block's multi-field constraints (e.g. slo_tier consistency) follow this pattern.
