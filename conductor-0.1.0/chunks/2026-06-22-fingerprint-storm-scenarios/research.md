# Codebase Research — 2026-06-22-fingerprint-storm-scenarios

## Scope
- **Depth:** moderate · **Reads:** 9 (model + 6 sibling TOMLs + Pulse spec + input.md) · **Greps:** 3 · **Code-graph:** 1 query (impact)

## Files inspected
- `crates/conductor-core/src/expected.rs` (full) — `ComparisonKind` = {Exact, Contains, Absent, CountAtLeast}; `ClaimClass` = {Hard, CalibrationRegion}; `ExpectedCheck { kind, class, expected }` (garde: `expected` non-empty). `Absent` = "observed does NOT contain the token". **No new variant needed.**
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario { name, p_ids, seed, slo_tier, phases, jitter_ms, expected }`; `expected` is `#[serde(default)]`. The **test module is the only thing I modify**: per-family loader `#[rstest]` (`#[case](stem, p_ids)` → load+validate, p_ids match, `!expected.is_empty()`), plus per-family guards: all-Hard (`activity_floor_and_restart_suppression_checks_are_all_hard`), Absent-usage (`activity_floor_asserts_..._via_absent`), Contains-presence (`presence_scenarios_assert_..._via_contains`). Pattern to extend, not change.
- `crates/conductor-emit/src/exception.rs` (grep) — **the emission primitive already exists**: `FingerprintVariant` = {Identical, PathVariant, LineVariant (→ SAME fp), TypeVariant, FrameVariant (→ DIFFERENT fp)} + `fingerprint(spec) -> String` (pure content fn, FNV/sha2-stable, path+line-insensitive — amended clause (c)). This chunk authors CONFIG that the Epoch-8 driver pairs with this primitive; it adds no emit code.
- Sibling TOMLs — `error-baseline-spike.toml` / `latency-regression.toml` (CountAtLeast floor + Contains candidate; **comment: "checks class='Hard' … the model-side severity that consumes the cue is P-020 (a later chunk), not asserted here"**; `LatencyRegression` is an explicitly **inferred** token), `restart-suppression.toml` (multi-phase, declare-only suppressed legs via comments, `EmissionSpec carries no magnitude — Epoch-8 driver realizes per-phase intent from phase names`), `high-severity-log-capture.toml` (P-007 `Contains "ERROR"` + `Absent "WARN"` two-sided), `activity-floor.toml` (P-013 `Absent "ServiceWentSilent"`, `<90s`), `exception-event-capture.toml` (P-006 `Contains "exception"`).
- `.andromeda/refs/pulse-capability-spec.md` §P-017 / §P-018 (lines 252–270) — authoritative; see "Patterns / spec facts" below.
- `.andromeda/input.md` (lines 24, 108) + `capability-verification-matrix.json` (P-017/P-018 → Pulse `crates/triage/src/pattern/storm.rs`).

## Graph impact (code-graph query → `tree-query-…json`)
- **ClaimClass** 69 refs · **ComparisonKind** 57 · **Scenario** 33 · **Scenario#expected** 23 · **ExpectedCheck** 18 · **Scenario::from_toml_str** 16 · **FingerprintVariant** (conductor-emit) present — all in `conductor-core`/`conductor-emit`.
- **Meaning:** every symbol this chunk references is a heavily-reused, already-shipped model type. The change adds 2 `scenarios/*.toml` + new test `#[case]` rows that call `from_toml_str`/match `ComparisonKind`/`ClaimClass` — **zero modification of any def, zero ripple** (touching these types would ripple across verify/report; reusing them ripples nothing — confirmed identical to the prior catalog chunks).

## Patterns detected (+ spec facts)
- **Detector-output token convention** (`scenario.rs` token set: ErrorRateSpike, LatencyRegression, RestartEvent, ServiceWentSilent, ReceiverFailed, Idle, Stalled, ERROR, WARN, exception): a PascalCase condition name asserted via `Contains`+`Hard`; the Epoch-8 evaluator substring-matches it; **model-side severity is NEVER hard-asserted in these catalog chunks — it is deferred to P-020 entirely** (declare-only in comments).
- **§P-017 (spec:252–260):** stable fingerprint = hash(`exception.type` + first 3 normalized frames; paths/lines/addresses stripped). **"Observable signal: NO direct user signal; identifier infrastructure for P-018."** → P-017 is verified *through* P-018 (does the same-fp triple aggregate into one storm? do type/frame variants stay distinct?).
- **§P-018 (spec:262–270):** same fingerprint ≥**5×**/30s → attention cue (default hint **Suggested**); ≥**10×** → escalates to **Autonomous**. **"Final severity is determined by model interpretation per P-020; the model retains discretion to dismiss/adjust."** Conductor verification injects **6** (→Suggested) then **12** (→Autonomous). Observable = the surfaced incident's storm characteristics; the attention cue itself has no user surface (Epoch-8 calibration, like the other cue tokens).
- **`Absent` for the no-false-positive / distinctness guard** (`activity-floor.toml`, `high-severity-log-capture.toml`): the negative side of a claim is a `Absent`+`Hard` check in its own outcome-coherent file.
- **`EmissionSpec` carries no count/magnitude field** — phases NAME the count+variant intent (e.g. `storm-same-fp-6x`); the Epoch-8 driver realizes the actual emission from the phase names + the `FingerprintVariant` primitive (exact restart-suppression precedent).

## Conventions to follow
- TOML header comment block: P-ID(s) + `pulse-capability-spec.md §…` citation + recipe + `Coverage mode: …` + Epoch-8 calibration points + any declare-only note (`scenario.rs` siblings).
- Seed family `43170NN`, distinct per file (next free: 4317017, 4317018). `jitter_ms = 50`.
- `slo_tier` is a declared bucket the Epoch-8 journal-relative run may retune (latency-regression precedent: "<20s" with that caveat). Storm detection has no explicit p99 budget → "<20s".

## New files to create
- `scenarios/fingerprint-storm.toml` — P-017 (same-fp triple) + P-018: storm the identical/path/line triple to 6 then 12 within the 30s window → they share ONE fingerprint → `Contains "RetryStorm"` (Hard). Severity 6→Suggested / 12→Autonomous declare-only (P-020/P-021). seed 4317017.
- `scenarios/fingerprint-distinct.toml` — P-017 (distinct-fp): base + type-variant + frame-variant each kept BELOW the 5× floor → THREE distinct fingerprints, none aggregates → `Absent "RetryStorm"` (Hard). The P-017 strictness/distinctness guard. seed 4317018.

## Files to modify
- `crates/conductor-core/src/scenario.rs` (tests only) — add: (a) a fingerprint-family loader `#[rstest]` (`fingerprint-storm`→[P-017,P-018], `fingerprint-distinct`→[P-017]); (b) an all-Hard guard `#[rstest]` over both stems; (c) a `Contains "RetryStorm"` presence assertion (fingerprint-storm) + an `Absent "RetryStorm"` assertion (fingerprint-distinct). No production code changes.

## Open questions (→ resolve at P4)
- **(1) Severity (Suggested/Autonomous) handling** — defer entirely to P-020 + assert only the deterministic `RetryStorm` detection token (all-Hard; matches every prior catalog chunk), OR add `CalibrationRegion` checks to keep the 6→Suggested/12→Autonomous escalation in-catalog. **P4 AskUserQuestion** (deviates from the input.md digest, which frames severity as the headline observable). Recommendation: defer (precedent + architecture "severity = model-interpretive").
- **(2) Token `RetryStorm`** — inferred (spec uses prose "retry storm"; no canonical PascalCase token). Mirrors the `LatencyRegression` inferred-token precedent; `Contains` is substring-tolerant; flag as an Epoch-8 calibration point. (Decide with Q1.)
- **(3) Two-file split** (storm + distinct) vs one — forced by outcome-coherence (`Contains "RetryStorm"` and `Absent "RetryStorm"` cannot co-exist in one read-back). (Folded into the P4 question.)
