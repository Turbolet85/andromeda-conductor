# Report — 2026-06-22-error-baseline-spike-latency-regression-scenarios

**Chunk:** Statistical-anomaly scenarios (P-009..P-012) — error-baseline-spike (formalized) + latency-regression catalog TOMLs; baseline→ramp persistence phases + all-Hard baseline-math/threshold-detection checks (conductor-core scenarios)
**Date:** 2026-06-22
**Commits:** (pending — this wrap authors the chunk commit; prior HEAD `b0717b7` = ch2 hard-signals)

## Changes (structured — detectors read this)
- **Files:**
  - `scenarios/error-baseline-spike.toml` (M) — formalized the pre-existing schema-exercise fixture: §3 header comment, phase gaps `2000/1000 → 90000/60000` (90s converge / 60s spike), +2 `[[expected]]` blocks.
  - `scenarios/latency-regression.toml` (NEW) — P-011/P-012; baseline→ramp phases (90s/90s); +2 `[[expected]]`.
  - `crates/conductor-core/src/scenario.rs` (M) — **test module only**: `statistical_anomaly_fixtures_load_and_validate` (slice-based rstest) + `statistical_anomaly_checks_are_hard_with_floor_and_candidate` guard (+4 test cases).
  - `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_424242.snap` (M) — re-baselined (see Deviations).
  - `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_7.snap` (M) — re-baselined (see Deviations).
- **Symbols / APIs:** none — no new/changed public fn · IPC method · endpoint · export · port/socket · env var. (Two `#[cfg(test)]` fns added in `scenario.rs`; not public surface.)
- **Crates / modules:** none added/removed/changed (declarative config + test artifacts only).
- **Dependencies:** none added/bumped. (Cargo.lock un-drifted.)
- **Schema / config:** 2 declarative scenario TOMLs keyed to P-009..P-012. **First catalog use of `ComparisonKind::CountAtLeast`** (`"10"` = P-009 10-span error-rate floor; `"50"` = P-011 50-sample latency floor) — the kind was already evaluator-wired (`conductor-verify/src/slo.rs:79`) + tested, never yet used in a TOML. **No model/schema change** to `Scenario`/`ExpectedCheck`/`PhaseSpec`/`SloTier`.
- **Coverage of new surfaces:**
  - `scenarios/error-baseline-spike.toml` (P-009/P-010, all `class=Hard`) → validation garde✓ (`from_toml_str`) · instrumentation n/a (no runtime; Epoch-8 driver) · PII n/a · tests unit✓ (fixture rstest + all-Hard/floor/candidate guard) · a11y n/a · tokens n/a
  - `scenarios/latency-regression.toml` (P-011/P-012, all `class=Hard`) → validation garde✓ · instrumentation n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a

## Deviations from intent
- **Two `conductor-timeline` determinism-replay snapshots re-baselined — NOT in the plan's "Files to modify."** The plan's intentional phase-gap retune (`90000/60000`, encoding the P-010 90s-converge / 60s-spike-over-30s-persistence recipe) changed the `error-baseline-spike` fixture's frozen `PhaseTransition` shape, which `replay.rs` freezes at two seeds (424242 + alternate 7) via insta. **Research's blast-radius grep (`grep "error-baseline-spike"`) missed them** — the snapshot files are *seed*-named (`replay__fixture_seed_424242.snap`), not scenario-named. Re-baselining is the correct, non-destructive consequence: the new values are insta's own computed output, **verified deterministic** — the seed-driven jitter deltas are byte-identical to the old goldens (+28/+47 @ seed 424242, +49/+11 @ seed 7); only the base gaps moved. The determinism invariant holds (same seed ⇒ same shape; the two seeds still diverge). In-scope iteration (a golden *of* a listed file).
- Otherwise: all plan acceptance criteria met as intended; the stated **zero-model-change / zero-new-dependency** goal held.

## Decisions & corrections
- **File granularity (P4 AskUserQuestion → user-confirmed):** 2 files per-P-ID-pair, NOT 4 per-P-ID (the ch1/ch2 convention). Rationale: detection (P-010/P-012) is baseline-dependent on a shared timeline; matches the architecture's canonical `error-baseline-spike` example (`p_ids=["P-009","P-010"]`). A deliberate, sanctioned divergence from the per-P-ID catalog convention for paired baseline→detection capabilities.
- **Baseline check = `CountAtLeast` floor; detection check = `Contains` candidate (research decision):** the model has no within-tolerance comparison kind, so the ±10%/±15% baseline-MATCH is the Epoch-8 evaluator's (declare-only, per ch2's SLO-deferral precedent); the declarable *hard* claims are the sample-count floor + candidate presence. All four checks `class=Hard`.
- **Candidate tokens:** `"ErrorRateSpike"` (P-010 — spec-confirmed, `pulse-capability-spec.md:244`) + `"LatencyRegression"` (P-012 — **inferred**; the spec uses lowercase prose, no PascalCase token; `Contains` is substring-tolerant). Flagged in-file as an Epoch-8 live-verify calibration point.
- **SLO tiers:** `error-baseline-spike` `<5s` (kept; P-010 detection <2s p99); `latency-regression` `<20s` (P-012 has no explicit p99 budget + a 60s-persistence signal → middle tier; a declared bucket the Epoch-8 journal-relative measurement may retune, potentially to `<90s`).
- **Correction (snapshot re-baseline):** research's scenario-name grep didn't surface the *seed*-named determinism goldens that consume the fixture — a reusable lesson for scenario-fixture chunks (grep blast-radius by seed too, not only scenario name).

## Outcome
- **All acceptance criteria met.** Both TOMLs deserialize + garde-validate via `Scenario::from_toml_str`; **zero** `Scenario`/`ExpectedCheck`/`PhaseSpec` model change; **zero** new dependency; every `[[expected]]` check `class=Hard` (no `CalibrationRegion` — the clean counterpart to ch2's P-008).
- **Gates green:** `cargo nextest run -p conductor-core` → **114/114** (110→114, +4) · `bash scripts/agent-run.sh run` → **exit 0**: workspace nextest **319** (315→319) · doctest 0 · `cargo clippy --workspace --all-targets -- -D warnings` clean.
- **Smoke:** `agent-run.sh run` (the release-gate path — the correct no-boot-path smoke per the verification-harness rule) → exit 0.
