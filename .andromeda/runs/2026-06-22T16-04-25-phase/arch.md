# arch extract

## Relevance
Relevant — baseline convergence, ramp, and persistence windows for statistical-anomaly detection (P-009..P-012) map directly to timeline phase scheduling and verdict classification.

## Constraints
- Architecture mandates a single async runtime, tokio `current_thread` with deterministic scheduling via seeded `ChaCha8Rng` (per architecture.md §Design Philosophy + §Established Decisions [Async Runtime Flavor]); same scenario+seed must produce same emission-stream shape.
- Scenario config surface (`Scenario`/`ExpectedCheck` TOML) is declared-intent only; tolerance bounds (±10%, ±15%) and persistence-window timing as *evaluated logic* belong to Epoch-8 evaluator, not the TOML spec (architecture.md §Established Decisions [Scenario Config Format] + scope.md §Boundaries "Out").
- All four checks (P-009..P-012) carry `class="Hard"` — deterministic baseline-math and threshold-detection claims per architecture.md §Probabilistic-Assertion Policy; model-interpretive severity is P-020, not asserted here.
- Latency-shaping, error-spans, error-rate, and traffic-rate ramps all exist as Epoch-3 emit primitives (architecture.md §Stack and Technologies — `conductor-emit` seam); no new primitive required.
- Config validation is garde 0.22.1 with `#[garde(custom)]` field-level only; cross-field invariants (p50≤p95≤p99 ordering, severity-mix sums) use garde's `Context` pattern (architecture-amendments.md 2026-06-15-config-validation-surface).
- Module boundary: scenario TOMLs consume `conductor-core::Scenario`/`ExpectedCheck` via `from_toml_str`; no cross-crate emit-logic access in catalog layer (architecture.md §Established Decisions [Module Boundaries]).

## Patterns to follow
- Two-file structure (one per P-ID-pair, since detection baseline-depends on a shared timeline), mirroring the canonical `error-baseline-spike` example in architecture.md §Standard Contracts (run-report envelope shape names `error-baseline-spike` / `p_ids=["P-009","P-010"]`).
- Declarative phase timeline (`[[phases]]{name, gap_ms}` + `[[expected]]{kind, class, expected}`) — fixture round-trip tests through the existing scheduler prove deserialization + garde validation + `PhaseTimeline` construction, mirroring ch1/ch2 precedent (scope.md §Definition of Done).
- Hard/CalibrationRegion split: P-009/P-010/P-011/P-012 all `class="Hard"` (no tolerance slack); hard failures are typed `Verdict::Fail`, calibration-region routed to `ManualCheck` state (architecture-amendments.md 2026-06-21-run-report-envelope-serializer).

## Anti-patterns to avoid
- No new emit primitive; latency shaping and error-rate control already exist in `conductor-emit` (scope.md §Boundaries "No new emit primitive").
- No inbound listener; scenarios remain pure-egress (scope.md §Scope law "no new inbound listener").
- Do not embed tolerance evaluation in TOML; declare expected baselines as scalar markers only; Epoch-8 evaluator owns the ±N% comparison logic (scope.md §Open questions #2 "declare-only").

## Contract bindings
**Emit ↔ Scenarios:** `conductor-emit` (latency/error primitives, traffic-rate ramps — read-only consumption) / `conductor-core` (Scenario/ExpectedCheck deserialization from TOML, no new fields).
**Core ↔ Report:** `conductor-report` (per-scenario run metadata, verdict/latency_ms for `runs.db` rows; phase-dependent fingerprinting per exception, sourced from emit).
**Timeline ↔ Phases:** `conductor-timeline` (PhaseTimeline scheduling, baseline → ramp → hold-past-persistence phases execute deterministically under seeded RNG per the shape declared in TOML).

## Acceptance criteria contributions
- (arch) Scenario TOMLs (`error-baseline-spike.toml` formalized + new `latency-regression.toml`) deserialize + garde-validate via `Scenario::from_toml_str` with valid/invalid `#[case]` rows wherever touched; fixture round-trip tests prove `PhaseTimeline` construction (mirroring ch1/ch2).
- (arch) All P-009/P-010/P-011/P-012 expected checks declare `class="Hard"`; no `CalibrationRegion` exception here — deterministic baseline-math and threshold-detection per §Probabilistic-Assertion Policy.
- (arch) Phase timeline (baseline-converge → ramp → hold-past-persistence) uses existing `conductor-emit` latency-shaping / error-spans / error-rate / traffic-rate ramps; no new emit primitive.
- (arch) Tolerance bounds (±10%, ±15%) and persistence-window timing declared in TOML as expected markers; evaluation deferred to Epoch-8 evaluator (no model change to `Scenario`/`ExpectedCheck` unless a genuinely missing comparison kind surfaces in planning).

## Relevant amendment history
- **2026-06-16-scenario-config-model (toml 0.9 registered):** `Scenario::from_toml_str` contract locked; declarative TOML format over JSON (hand-author ergonomics); audit/deny-clean; cascaded to stack.md. (This chunk consumes it.)
- **2026-06-21-run-report-envelope-serializer (ManualCheck widened + Verdict→ReportState):** Hard-vs-calibration split formalized; P-009/P-012 all `class="Hard"` fall into Pass/Fail verdict, never CalibrationRegion (contrast with ch2's P-008). Verdict-first lamp precedence resolves a11y display.
- **2026-06-16-exception-events-fingerprint-control (fingerprint primitive in conductor-emit):** Fingerprint derivation for exception events placed in `conductor-emit` seam, not faults; phase-dependent fingerprinting will consume this primitive.
