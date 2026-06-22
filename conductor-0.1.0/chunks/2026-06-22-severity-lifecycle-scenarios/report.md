# Report — 2026-06-22-severity-lifecycle-scenarios

**Chunk:** Severity-lifecycle scenarios (P-019..P-023, P-059, P-060) — tiered-input severity (Autonomous/Suggested/Curious) + 120s auto-resolve→Resolved + 5-min ack cool-down + per-tier SLO; first MIXED Hard+CalibrationRegion family, zero model change (conductor-core scenarios)
**Date:** 2026-06-22
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)
- **Files:**
  - NEW (5): `scenarios/severity-tier-autonomous.toml` · `scenarios/severity-tier-suggested.toml` · `scenarios/severity-tier-curious.toml` · `scenarios/incident-auto-resolution.toml` · `scenarios/ack-cooldown.toml`
  - MOD (tests only): `crates/conductor-core/src/scenario.rs` (+5 test fns / +11 `#[case]`/`#[test]` instances in `#[cfg(test)] mod tests`)
- **Symbols / APIs:** NONE new or changed — no public fn, IPC method, endpoint, export, port/socket, or env var. `Scenario`/`ExpectedCheck`/`ComparisonKind`/`ClaimClass`/`SloTier` are CONSUMED unchanged (code-graph query: additive, zero cross-crate blast radius).
- **Crates / modules:** none added/removed/changed (only the `conductor-core` test module extended).
- **Dependencies:** none added/bumped.
- **Schema / config:** 5 new declarative scenario TOMLs under the already-registered `scenarios/` config-file category. Read-back/detector-output tokens declared in `[[expected]]`: `Autonomous` / `Suggested` / `Curious` (severity tiers, CalibrationRegion), `Resolved` (lifecycle status, Hard), `resolution summary` (P-059 continuity, CalibrationRegion) — all inferred from spec prose, asserted substring-tolerant via `Contains`. Comparison kinds used: `Contains`, `CountAtLeast` (NO new `ComparisonKind`). SLO tiers used: `<5s`/`<20s`/`<90s` (P-060 Tier-1/2/3; NO new `SloTier` value). Claim classes used: `Hard` (P-022/P-023/P-060 lifecycle timing + tier routing) + `CalibrationRegion` (P-019/P-020/P-059 model-interpretive) — the catalog's FIRST mixed-class family.
- **Coverage of new surfaces:**
  - `scenarios/severity-tier-autonomous.toml` (P-019/020/060) → validation garde✓ (`Scenario::from_toml_str`) · instrumentation n/a (declarative config; no runtime op — Epoch-8) · PII n/a (static config, no host paths/secrets) · tests unit✓ (rstest load+validate + tier-token + mixed-class) · a11y n/a (no UI) · tokens n/a (no UI)
  - `scenarios/severity-tier-suggested.toml` (P-019/020/021/060) → garde✓ · instr n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a
  - `scenarios/severity-tier-curious.toml` (P-019/020/021/060) → garde✓ · instr n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a
  - `scenarios/incident-auto-resolution.toml` (P-022/059) → garde✓ · instr n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a
  - `scenarios/ack-cooldown.toml` (P-023) → garde✓ · instr n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a

## Deviations from intent
- **Smoke command:** plan's Test Commands listed `agent-run.sh status`; I ran `agent-run.sh run` instead. Justification: `status` requires a `<run_id>` from a prior live run (it exited 2/usage); the `.claude/rules/verification-harness.md` rule (2026-06-21) + the script's own contract make `run` (the release-gate path: workspace nextest + doctest + clippy) the correct smoke for a no-boot-path config/fixture chunk. No code impact.
- **P-021 tag scoping (plan-internal refinement, not an acceptance deviation):** P-021 tagged on the suggested + curious tier files only (cue-driven Tier-2/3), NOT autonomous (hard-signal Tier-1) — grounding the P-IDs precisely per spec P-060 ("Tier-1 = hard signals", which bypass the P-021 cue layer). All 7 P-IDs still covered.

## Decisions & corrections
- **P4 Q1 (user) — per-tier split (5 files):** each severity-tier scenario carries a distinct `slo_tier`, so the family exercises all three P-019 severity tiers AND all three P-060 SLO tiers as live declarations. Driven by the research finding that `slo_tier` is **scenario-level** (one per `Scenario`), so P-060's three tiers cannot share one file.
- **P4 Q2 (user) — ack-cooldown assert-after / declare-only-within:** asserts only the drivable after-cooldown new-incident leg as Hard (`CountAtLeast "2"`); the within-cooldown suppression + the ack mechanism (`mark_incident_resolved` presumed) are declare-only Epoch-8 calibration points — because "acknowledge" has no clear tool in the pinned 4-tool MCP contract, and within/after contradict on the incident token under the single-read-back evaluator (the activity-floor "assert the provable side" precedent).
- **The inflection chunk:** first catalog family carrying `CalibrationRegion` as the dominant mode (severity choice = P-020, which every prior chunk DEFERRED to) mixed with a `Hard` lifecycle-timing spine. The suite mixed-class guard (`any(Hard)` && `any(CalibrationRegion)`) is the one new test shape (every prior family guard was `all(Hard)`).
- **Zero model change confirmed:** the existing `ComparisonKind` (`Contains`/`CountAtLeast`) + the P-008 `CalibrationRegion`-class precedent expressed every leg; no new kind/field/enum value. Lifecycle-status (P-022) = `Contains "Resolved"` + `CountAtLeast "2"` (new-not-reopen).
- **Epoch-8 calibration points (declare-only, recorded in each TOML's comment block):** the inferred read-back tokens (Autonomous/Suggested/Curious/Resolved/"resolution summary"); ack drivability (does `mark_incident_resolved` serve as ack?); the within-cooldown suppression + corpus-records leg; the spec-faithful long `gap_ms` (120s / 5min / 10min) vs a compressed runtime variant; the per-tier latency measurement (the TOML only declares the tier).

## Outcome
- **Acceptance criteria met:** all 5 TOMLs deserialize + garde-validate via `Scenario::from_toml_str`; mixed class correct (3 tier files all `CalibrationRegion`; `incident-auto-resolution` mixed Hard+CalReg; `ack-cooldown` Hard); each tier file's `slo_tier` realizes P-060 Tier-1/2/3; no new `ComparisonKind`/`SloTier`; all 7 P-IDs covered; suite mixed-class guard green.
- **Gates green:** `cargo nextest run -p conductor-core` **140/140** (129→140) · `cargo nextest run --workspace --profile ci` **345/345** (334→345) · `cargo clippy --workspace --all-targets -- -D warnings` exit 0 · `cargo test --doc` exit 0.
- **Determinism goldens UNCHANGED** — new seeds `4317019`–`4317023` feed no golden (replay.rs + determinism.rs load only `error-baseline-spike`; snapshots/ clean in git).
- **Smoke:** `bash scripts/agent-run.sh run` exit 0 (release-gate path).
