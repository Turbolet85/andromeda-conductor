# arch extract

## Relevance
Partial — constellation and context-grounding scenarios are catalog entries (declarative TOML + test wiring) without new runtime engine code or emission primitives; architecture surfaces the workspace/module layout, Standard Contracts, and scenario-model conventions these must follow.

## Constraints
1. Scenarios must live in `scenarios/*.toml` declarative config (per §Stack and Technologies TOML + §Established Decisions [Scenario Config Format] — serde + garde validation, hand-author ergonomic format).
2. Each scenario carries a P-ID anchor (P-025, P-026, P-027, P-032, P-036); "no scenario without a P-ID" per §Conventions naming patterns + §Cross-cutting Patterns scope law.
3. Scenario loading/guard test wiring lives in `conductor-core` module (per §Inherited Defaults workspace crate-per-seam rule); forbidden cross-seam deps enforced by Cargo compiler.
4. Report states ManualCheck + KnownResidual must conform to §Standard Contracts run-report envelope shape (`verdict ∈ {Pass, Fail, CalibrationRegion}` / `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`), with P-032 KnownResidual pre-accepted and P-025/P-026/P-027 ManualCheck via operator-checklist path per §Read-Back Dependency Posture + §Probabilistic-Assertion Policy.
5. Emission primitives already exist from earlier epochs (rate.rs, topology.rs, fingerprint primitive in conductor-emit per amendment 2026-06-18); this chunk reuses, does not build.
6. Determinism: seed selection must not collide with severity-lifecycle golden seeds (4317019..023); JSONL journals + run reports remain unchanged ground truth per §Design Philosophy journal-relative.

## Patterns to follow
1. Scenario config struct with serde + garde `#[derive(Validate)]` bounds; cross-field invariants (p50≤p95≤p99 ordering, severity-mix sums) use garde's `Context` pattern (field-level `#[garde(custom)]` only per amendment 2026-06-15) — per §Established Decisions [Validation Library].
2. Per-scenario test wiring via rstest + per-file class/state purity guards; suite-level guard updated for ManualCheck/KnownResidual coverage per amendment 2026-06-21 (Verdict::default_report_state mapping and verdict-first lamp precedence).
3. P-032 declares intent to run inside real git workspace with known commits (operator/live-proof precondition, Epoch-10 downstream); P-036 cross-run fingerprint index in runs.db already exists (§Occupied Resources on-disk artifacts).

## Anti-patterns to avoid
1. No new ComparisonKind, emission primitive, or engine-path addition unless research proves the report-state expression requires minimal model addition — maintain "zero model change unless proven necessary" posture per scope boundary.
2. No new env var beyond the reserved `CONDUCTOR_*` namespace (CONDUCTOR_RUNS_DIR, CONDUCTOR_SCENARIOS_DIR, CONDUCTOR_CONTRACT_MANIFEST, CONDUCTOR_SEED, CONDUCTOR_SERVICE_NAME, CONDUCTOR_ENV per amendment 2026-06-15); no secrets/cloud credentials (local-only tool).

## Contract bindings
- **Report states** ↔ Obs (self-observation logs emit ReportState labels per tracing 0.1.44 + tracing-subscriber 0.3.23 stack per amendment 2026-06-14).
- **Scenario model** ↔ Tests (test-framework-fixtures loads + guards scenario TOML and state purity; nextest zero-retry ci profile per amendment 2026-06-16).
- **P-032 / P-036 read-back** ↔ Verify (MCP read-back via rmcp 1.7.0 against Pulse's 2024-11-05 hand-rolled server; preflight version-pinned manifest + canary gate per §Standard Contracts §Read-Back Dependency Posture).

## Acceptance criteria contributions
1. (arch) Five scenario TOMLs (P-025, P-026, P-027, P-032, P-036) exist in `scenarios/` directory, each carrying its P-ID, expected outcome (ManualCheck for P-025/026/027, KnownResidual for P-032, Auto recurrence for P-036), and SLO tier; loadable + garde-valid per §Established Decisions [Scenario Config Format].
2. (arch) Constellation trio (P-025/026/027) expresses operator-checklist/ManualCheck (DriveObserve) path; P-032 expresses KnownResidual via pre-accepted deviation; P-036 expresses Auto cross-run fingerprint recurrence check per §Probabilistic-Assertion Policy §Read-Back Dependency Posture.
3. (arch) Scenario test wiring in `conductor-core` loads and guards the new family (per-file class/state purity + suite-level guard updated for ManualCheck/KnownResidual coverage per amendment 2026-06-21).
4. (arch) No golden seed collision (severity-lifecycle seeds 4317019..023 unchanged); `agent-run.sh run` exit 0 per §Infrastructure Patterns CI/CD (build + test gating, nextest zero-retry).

## Relevant amendment history
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive (`fingerprint()` + exception-event builder) placed in `conductor-emit` (not conductor-faults); P-036's "Previously seen" cross-run check depends on this primitive, stored in runs.db fingerprint index per §Occupied Resources.
- **2026-06-21-run-report-envelope-serializer** — ManualCheck broadened to include auto-measured model-interpretive (calibration-region) checks; Verdict::default_report_state mapping (Pass→Pass / Fail→Fail / CalibrationRegion→ManualCheck); verdict/state independent, lamp chosen verdict-first per §Probabilistic-Assertion Policy.
- **2026-06-16-scenario-config-model** — toml 0.9 registered (declarative TOML over JSON for hand-author ergonomics); Scenario::from_toml_str → CoreError validation per §Established Decisions [Scenario Config Format].
- **2026-06-15-config-validation-surface** — garde 0.22.1 pinned (0.23.0 derive unavailable); cross-field invariants use garde's Context pattern (field-level custom only) per §Stack and Technologies Validation row.