# arch extract

## Relevance
Relevant — severity-lifecycle scenarios (P-019..P-023, P-059, P-060) are the first mixed-class family, introducing `CalibrationRegion` legs; architecture governs workspace structure, standard contracts, cross-cutting patterns, and the split between hard-fail and calibration-region assertions.

## Constraints
- Per §Established Decisions [Scenario Config Format], severity-lifecycle scenarios MUST be declarative TOML deserialized via `Scenario::from_toml_str` with garde validation, no DSL (architecture.md).
- Per §Probabilistic-Assertion Policy, severity *choice* (P-019/P-020) routes to `CalibrationRegion → ManualCheck`; lifecycle *timing* (P-022 120 s auto-resolve, P-023 5-min cool-down) and tier→SLO routing (P-060) remain hard-fail (architecture.md).
- Per §Established Decisions [Module Boundaries], code and config live in declared workspace crates; this chunk contributes `scenarios/*.toml` files only, not new crates (architecture.md).
- Per §Cross-cutting Patterns [Config management], scenario config is files + environment variables only; no DSL, no cloud config (architecture.md).
- Per §Conventions [Data model], severity tokens (`Autonomous`/`Suggested`/`Curious`/`Resolved`) are declared as expected-check comparisons (`Contains`, `Absent`, `CountAtLeast`) — no new comparison-kind unless genuinely unexpressible (scope Q3, architecture.md).
- Per §Occupied Resources [Crate names], the `conductor-*` prefix is reserved; the `scenarios/` directory is the artifact location (architecture.md).
- Per §Standard Contracts [Run report envelope], each check classifies into `verdict ∈ {Pass, Fail, CalibrationRegion}` and `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}` with `slo_tier ∈ {<5s, <20s, <90s}` (P-060 tier routing) (architecture.md).

## Patterns to follow
- Mixed `class` split: `Hard` for deterministic lifecycle timing + tier routing; `CalibrationRegion` for model-interpretive severity choice — precedent is P-008 in chunk 2 (scope.md §What it builds, scope-anchor P-008 CalibrationRegion precedent).
- Scenario TOML shape (ch1–ch5 precedent): `name · p_ids · seed · slo_tier · jitter_ms · [[phases]]{name, gap_ms} · [[expected]]{kind, class, expected}` deserialized via `Scenario::from_toml_str` + garde `#[case]` validation (scope.md).
- Tier-stimuli mapping: error-span / error-rate / latency-shaping / retry-storm ramps are existing emit/fault primitives; severity tiers are produced by calibrated magnitude, not new primitives (scope.md §What it builds).
- Determinism under seed: same scenario + seed ⇒ same emission-stream shape via `current_thread` tokio + `ChaCha8Rng` seeded `seed_from_u64(scenario.seed)` (architecture.md §Design Philosophy, amendment 2026-06-16-seeded-phase-scheduler).

## Anti-patterns to avoid
- No new emit / fault primitive — all stimuli (error-spans, ramps, silence/gap, fingerprint-storm) exist from Epoch 3/4 (scope.md §What it builds).
- No new inbound listener — pure-egress (tier stimuli are starts/stops of existing egress; the port-occupier P-003 is ch1) (scope.md §Scope law).
- No new dependency — architecture pins toml 0.9, garde 0.22.1, serde 1.0.x; no graceful-degradation / model-off behavior (P-020's reduced mode is P-045 family, Epoch-7 ch8) (scope.md §Out).

## Contract bindings
- **Scenario TOML surface ↔ conductor-core shared types** — `Scenario`, `ExpectedCheck`, `Phase`, `ClaimClass`, `ComparisonKind`, `SloTier` in `conductor-core::{scenario.rs, expected.rs}` (scope.md §Surfaces / contracts).
- **CalibrationRegion verdict class ↔ Probabilistic-Assertion Policy** — `Verdict::CalibrationRegion → ReportState::ManualCheck` via `Verdict::default_report_state` (amendment 2026-06-21-run-report-envelope-serializer; architecture.md §Probabilistic-Assertion Policy).
- **Tier→SLO tier routing ↔ Timing-Tolerance Model** — `SloTier ∈ {<5s, <20s, <90s}` maps P-060 Tier-1/2/3 (architecture.md §Timing-Tolerance Model, §Established Decisions).
- **Severity/lifecycle tokens ↔ Read-back / detector-output** — `Autonomous` / `Suggested` / `Curious` / `Resolved` are declared in `[[expected]]` blocks, inferred from spec prose, asserted via `Contains` substring-tolerance (scope.md §Surfaces / contracts).

## Acceptance criteria contributions
- (arch) Severity-lifecycle scenario TOML file(s) live in `scenarios/` per workspace boundary rules (arch §Occupied Resources / §Inherited Defaults).
- (arch) Expected checks carry correct mixed `class`: `Hard` for P-022/P-023 lifecycle timing + P-060 tier routing; `CalibrationRegion` for P-019/P-020/P-059 model-interpretive legs (arch §Probabilistic-Assertion Policy).
- (arch) `slo_tier` assignment per scenario conforms to P-060's Tier-1/2/3 = `<5s`/`<20s`/`<90s` mapping; no new `SloTier` enum value (arch §Occupied Resources / §Conventions).
- (arch) Severity tokens declared via existing `ComparisonKind` (`Contains`, `Absent`, `CountAtLeast`) — no new comparison-kind unless Q3 lifecycle-status assertion is genuinely unexpressible (arch §Conventions).

## Relevant amendment history
- **2026-06-16-seeded-phase-scheduler** — `ChaCha8Rng` + rand_core 0.9 pinned; determinism-under-seed enforced via `seed_from_u64(scenario.seed)`. (Relevant: maintain seed-based determinism across the severity-lifecycle scenarios under `start_paused`.)
- **2026-06-16-scenario-config-model** — `toml 0.9` registered; declarative TOML over serde + garde at load. (Relevant: severity-lifecycle TOML follows the ch1–ch5 shape and deserializes via `Scenario::from_toml_str`.)
- **2026-06-21-run-report-envelope-serializer** — `ManualCheck` widened to include auto-measured calibration-region checks; `Verdict::default_report_state` maps `CalibrationRegion → ManualCheck`, verdict-first lamp. (Relevant: first mixed-class family; P-019/P-020/P-059 CalibrationRegion legs route here.)
- **2026-06-15-config-validation-surface** — `garde 0.22.1` pinned (field-level `#[garde(custom)]`; cross-field via `Context`). (Relevant: scenario validation may carry cross-field constraints; use the existing pattern.)
