# Codebase Research — 2026-06-21-connection-lifecycle-scenarios

## Scope
- **Depth:** moderate-deep · **Reads:** 11 (scenario.rs, phase_spec.rs, expected.rs, pause.rs, lib.rs, convert.rs, code-graph-views.sql, error-baseline-spike.toml, prior scope.md, pulse-capability-spec.md P-001..P-004, error.rs §test) · **Globs/Greps:** 7 · **code-graph queries:** 1

## Normative requirement (pulse-capability-spec.md §1 Connection & Health Awareness)
- **P-001 Receiver Lifecycle State** — a 5-state machine: **Listening** (receivers up, zero spans ever) → **Receiving** (span within last 10s; transition within **1s** of first valid span) → **Idle** (10–60s quiet) → **Stalled** (>60s quiet) ; **ReceiverFailed** (bind error/task panic) overrides any state. Conductor verification: *start without emit (Listening), start workload (Receiving), stop (Idle→Stalled by timing), pre-bind port conflict (ReceiverFailed).*
- **P-002 Last-Span-Ago Tracking** — monotonic since-last-span tracker, subsecond. Conductor verification: *emit at known intervals; query tracker; verify reported elapsed matches reality within **±1s**.*
- **P-003 Receiver Failure Surface** — surface receiver failures (port bind, panic, shutdown) within **2s**. Conductor verification: *sacrificial process holds :4317, then start Pulse; observe ReceiverFailed within 2s, tooltip "Port 4317 in use".*
- **P-004 Orthogonal Health Domains** — connection state ⟂ app health. Conductor verification: *emit high error rate then clean shutdown of emitter; verify halo indicates error severity while connection dot transitions Receiving → Idle without a false alert state.*

## Files inspected
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario {name, p_ids, seed, slo_tier, phases, jitter_ms}`; `from_toml_str` (toml→`CoreError::Config`, validate→`CoreError::Validation`); `SloTier {Tier5s/20s/90s}` serde-renamed `<5s>/<20s>/<90s>` + `deadline_ms()`; `PId` garde `P-NNN` 001..=060; `no_duplicate_pids` field-custom. **No `expected`/`holds` field.**
- `crates/conductor-core/src/phase_spec.rs` (full) — `PhaseSpec {name(1..=40), gap_ms(≤MAX_GAP_MS=3_600_000), emission}`; `emission` is `#[serde(default)]` (TOML-omittable) — the additive-field precedent. `MAX_JITTER_MS=60_000`. `EmissionSpec`/`Signal` `#[non_exhaustive]`.
- `crates/conductor-core/src/expected.rs` (full) — `ExpectedCheck {kind: ComparisonKind, class: ClaimClass, expected: String(≥1)}`; `ComparisonKind {Exact, Contains, Absent, CountAtLeast}`; `ClaimClass {Hard, CalibrationRegion}`. Wire spellings locked to canonical PascalCase by tests. **Exists, exported, but not referenced by `Scenario`.**
- `crates/conductor-core/src/pause.rs` (full) — `HoldPoint {scenario, p_id, step, prompt, allow_no_go}` + `resolve_hold`/`HeadlessResolver` (never blocks). Pause is wall-clock *outside* the seeded clock. **Exists, exported, not referenced by `Scenario`.**
- `crates/conductor-core/src/lib.rs` — exports already include `ExpectedCheck`, `ClaimClass`, `ComparisonKind`, `HoldPoint` (lib.rs:33,36) — no new export needed to use them.
- `crates/conductor-timeline/src/convert.rs` (full) — `impl From<&Scenario> for PhaseTimeline` reads **only** `phases` (name+gap_ms) and `jitter_ms`; emission/expected/holds are not consumed here. Additive `Scenario` fields are safe for the timeline bridge.
- `scenarios/error-baseline-spike.toml` — the precedent shape: top-level `name/p_ids/seed/slo_tier/jitter_ms` + `[[phases]] {name, gap_ms}` (emission omitted → defaults).
- `crates/conductor-faults/src/port_occupier.rs` (API grep) — `PortOccupier::occupy(port)` / `occupy_default()` (binds :4317) / `local_addr()` / `release()` + `Drop` (RAII); returns `Result<_, FaultError>`. The P-003 ReceiverFailed driver — already built.

## Graph impact (code-graph query → run_dir/tree-query-…json)
- **`Scenario` struct-literal sites = 3, all `#[cfg(test)]`:** `scenario.rs` (`scenario_with`), `convert.rs` (`scenario`), `error.rs:51` (invalid-scenario test). Adding a field forces updating these 3 (or `..Default::default()`). No non-test construction site exists yet (the CLI driver is Epoch 8).
- **Heavily-referenced core types** (additive-only, no signature change): `SloTier` 69 refs · `ClaimClass` 62 · `PId` 52 · `ComparisonKind` 47 — type annotations across verify/report/timeline tests, not struct-literals; safe under additive changes.
- **Wiring gap confirmed:** `ExpectedCheck` is consumed by `conductor-verify` (lib/record/slo/verdict) as the evaluator input, but nothing carries per-scenario checks from config into it — `Scenario` has no `expected`. This chunk closes the *config-carrier* half (follow-up d); the verify *consumption* end-to-end is Epoch 8.

## Patterns detected
- **TOML→model→timeline** (`scenario.rs:100`, `convert.rs:17`): `Scenario::from_toml_str` validates at load; `From<&Scenario>` is total/infallible (garde guarantees non-empty phases first).
- **Additive `#[serde(default)]` field** (`phase_spec.rs:34` `emission`): the established way to extend the config shape without breaking existing TOML or the `#[non_exhaustive]` forward-compat rule.
- **Committed-fixture test** (`scenario.rs:279` `committed_fixture_loads_and_validates`): loads a real `scenarios/*.toml` via `concat!(env!("CARGO_MANIFEST_DIR"), "/../../scenarios/<name>.toml")` and asserts deserialize+validate. The exact pattern to mirror per new TOML.
- **Determinism test** (`convert.rs:69–91`): `#[tokio::test(flavor="current_thread", start_paused=true)]` + `run_timeline` same-seed-identical / diff-seed-diverge (per testing.md Session Additions 2026-06-16).
- **Canonical-name serde golden** (`expected.rs` tests): exact-string `assert_eq!` locking enum wire spelling — apply to any new field's serialization.

## Conventions to follow
- Additive `#[serde(default)]` + `#[garde(dive)]` field(s) on `Scenario`; mirror `emission`'s default-omittable pattern (`phase_spec.rs:34`).
- Validation co-located with the struct (garde field rules); `dive` into a `Vec<ExpectedCheck>`/`Vec<HoldPoint>` (garde 0.22.1: no container-custom — field-level only, per arch amendment 2026-06-15).
- New scenario TOMLs under `scenarios/`, kebab-case, each carrying non-empty `p_ids` ("no scenario without a P-ID", security-plan §Anti-Patterns).
- Verdict/error wall: bad TOML → `CoreError::Config`/`Validation` (`Err`), never panic; outcomes stay values.
- SLO tiers from the closed set: P-001 Receiving (≤1s) + P-003 ReceiverFailed (≤2s) → `<5s`; P-001 Idle/Stalled long windows (10–60s / >60s) → `<90s` (`deadline_ms` 90_000 covers >60s); P-002 (±1s) → `<5s`. Final mapping fixed in P4.

## New files to create
- `scenarios/connection-lifecycle*.toml` — the catalog entries for P-001..P-004 (file count = P4 open question #1: 4 per-P-ID files vs 1 state-walk scenario).

## Files to modify
- `crates/conductor-core/src/scenario.rs` — add the `expected` (+ optional `holds`) field(s) to `Scenario` with `#[serde(default)]` + garde; update the `scenario_with` test helper literal; add round-trip + committed-fixture tests + valid/invalid `#[case]` rows.
- `crates/conductor-timeline/src/convert.rs` — add the new field to the `scenario()` test-helper literal (From impl unchanged).
- `crates/conductor-core/src/error.rs` — add the new field to the invalid-scenario test literal (`error.rs:51`).
- *(no `lib.rs` export change — `ExpectedCheck`/`HoldPoint` already public.)*

## Open questions
1. **File granularity** — 4 TOMLs (one per P-ID, each single `p_id`) vs 1 connection-lifecycle scenario whose `[[phases]]` walk Listening→Receiving→Idle→Stalled carrying `p_ids=[P-001..P-004]`. The spec gives each P-ID a *distinct* verification recipe (state walk / tracker / port-conflict / orthogonality), which leans toward per-P-ID files; P-003 (port-occupier) and P-004 (error-burst+clean-stop) are clearly their own scenarios. → **P4 AskUserQuestion.**
2. **Model-extension shape** — scenario-level `expected: Vec<ExpectedCheck>` (simple, forward-compatible) vs per-phase or P-ID-tagged checks. Note `ExpectedCheck`'s `ComparisonKind` has no native tolerance-window (the P-002 ±1s / the P-001 state-transition windows) — accept the current vocabulary and leave window interpretation to the Epoch-8 evaluator, or extend `ComparisonKind` now? Lean: keep additive + minimal, defer evaluator semantics. → P4.
3. **Holds** — do these drive+observe scenarios need an operator `holds` wiring at all? The visual lifecycle-badge claim routes to `ManualCheck` (a report-state), not necessarily a go/no-go `HoldPoint`. Lean: wire `expected` now; defer `holds` unless a scenario genuinely gates a non-Conductor step. → P4.
