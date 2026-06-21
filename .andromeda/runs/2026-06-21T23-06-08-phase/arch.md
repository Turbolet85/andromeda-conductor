# arch extract

## Relevance
Relevant

## Constraints
1. Per §Established Decisions [Determinism RNG]: seeded `ChaCha8Rng` via `seed_from_u64` is the sole non-determinism source; same scenario+seed reproduces identical stream shape (architecture §Determinism under a seed, §Cross-cutting Patterns · Determinism discipline).
2. Per §Established Decisions [Scenario Config Format]: declarative TOML scenario config (serde-deserialized + garde-validated via `Scenario::from_toml_str`); P-007 severity boundary and P-008 root-vs-deep must be expressible within the existing `Scenario`/`ExpectedCheck` TOML model (architecture §Established Decisions · §Conventions – Config conventions).
3. Per §Established Decisions [Validation Library]: garde 0.22.1 (field-level `#[garde(custom)]` only; cross-field invariants use `Context` pattern) for scenario bounds validation (architecture §Established Decisions [Validation Library] · §Stack and Technologies).
4. Per scope: hard-signal scenarios **reference, do not modify** the existing emit primitives (error-spans P-005/P-008, exception-events P-006 fingerprint, severity-logs P-007) — all exist from Epoch 3 and land in `conductor-emit` (per 2026-06-18-exception-events-fingerprint-control amendment, architecture §Infrastructure Patterns directory-tree).
5. Per §Occupied Resources (Crate names): code lands in `conductor-core` (`scenarios/` TOML files + `Scenario`/`ExpectedCheck` extensions if minimal, fixtures in `conductor-core/tests/`) or extending seam crates; the crate-per-seam Cargo workspace enforces module boundaries at compile time.
6. Per §Established Decisions [Verdict/error wall]: verification outcomes (`Verdict { Pass, Fail, CalibrationRegion }` / `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`) are typed values; `Result::Err` reserved for harness faults; P-008's calibration-region check routes to `ManualCheck` via the default `Verdict::default_report_state` mapping (architecture §Probabilistic-Assertion Policy · 2026-06-21-run-report-envelope-serializer amendment).
7. Per §Stack and Technologies: Rust 2024 (cargo 1.85, MSRV 1.94.1), tokio 1.48.x (`current_thread` flavor), toml 0.9, serde 1.0.x, garde 0.22.1 (architecture §Stack and Technologies, §Established Decisions).

## Patterns to follow
1. Hard-signals scenarios mirror ch1's precedent (e.g., `scenarios/error-baseline-spike.toml`, `scenarios/receiver-lifecycle-state.toml`): `name · p_ids · seed · slo_tier · jitter_ms · [[phases]] {name, gap_ms} · [[expected]] {kind, class, expected}` (scope §Requirement source of truth – Existing precedent).
2. P-005..P-007 carry `class = "Hard"` expected checks (deterministic); P-008 carries `class = "CalibrationRegion"` routed to `ManualCheck` — the one deviation (scope §What it builds – P-008 amendment; 2026-06-21-run-report-envelope-serializer amendment).
3. Fixture round-trip tests (TOML deserialize + garde-validate + `PhaseTimeline` builder) in `conductor-core/tests/fixtures/` mirroring ch1 pattern (scope §Definition of done – fixture round-trip tests).
4. SLO tier assignment per the verification budget: P-005 <500ms-p99 detection budget → `slo_tier = "<5s"` (architecture §Timing-Tolerance Model tier-scaled bounded slack).

## Anti-patterns to avoid
1. Do NOT introduce new emit primitives (P-005/P-008 error-spans, P-006 exception-event fingerprints, P-007 severity-logs all exist from Epoch 3); scenarios purely consume and drive them via TOML config (scope §Boundaries; §Requirement source of truth – scope law).
2. Do NOT land code in any crate outside the workspace members listed in §Occupied Resources; crate-per-seam module boundaries are compiler-enforced (architecture §Compiler-enforced module seams; scope law – no new inbound listener).
3. Do NOT hard-fail on model-interpretive/calibration-region assertions (P-008 root-vs-deep severity weighting); route to `CalibrationRegion` verdict → `ManualCheck` report state only (architecture §Probabilistic-Assertion Policy).

## Contract bindings
- **arch ↔ tests**: fixture round-trip tests (TOML → `Scenario` → `PhaseTimeline` + garde validation) prove each scenario deserializes, validates, and schedules deterministically (scope §Definition of done – mirrors ch1 + error-baseline-spike).
- **arch ↔ obs**: expected checks (`class = "Hard"` / `"CalibrationRegion"`) bind to tracing spans for SLO-measurement logging passed to the Epoch-8 CLI driver (architecture §Conventions – Error handling; obs §3).

## Acceptance criteria contributions
1. (arch) Scenario TOML file(s) live in `scenarios/` per workspace boundary rules (architecture §Occupied Resources · Crate names).
2. (arch) Hard-signal scenarios deserialize + garde-validate + produce a valid `PhaseTimeline` via the existing scheduler — proven by fixture round-trip tests (architecture §Inherited Defaults – Config conventions; scope §Definition of done).
3. (arch) P-005..P-007 expected checks carry `class = "Hard"`; P-008 carries `class = "CalibrationRegion"` (amendment 2026-06-21-run-report-envelope-serializer; scope §What it builds).
4. (arch) No new emit primitive, no new inbound listener, no new env var beyond `CONDUCTOR_*` namespace (architecture §Occupied Resources – Environment variables; scope §Boundaries).

## Relevant amendment history
- **2026-06-21-run-report-envelope-serializer** — ManualCheck widened to include auto-measured calibration-region checks; `Verdict::default_report_state` records `CalibrationRegion→ManualCheck`. Applies to P-008: the scenario's P-008 expected block must carry `class = "CalibrationRegion"`.
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive + exception-event builder placed in `conductor-emit`. Applies to P-006: the exception primitive already exists; no `conductor-faults` extension needed here.
- **2026-06-16-scenario-config-model** — toml 0.9 registered; declarative TOML scenario shape locked. Applies to all P-005..P-008: TOML must deserialize + garde-validate via the existing `Scenario::from_toml_str` (no model change assumed in scope Q5).
