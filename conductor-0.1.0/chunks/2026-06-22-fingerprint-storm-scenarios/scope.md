# Scope — Fingerprint-storm scenarios (P-017, P-018)

**Marker:** `2026-06-22-fingerprint-storm-scenarios`
**Epoch:** 7 (Scenario catalog) — ch5/8
**Working entry:** _Fingerprint-storm scenarios — identity/path/line-variant fingerprints, storm cue thresholds (P-017, P-018)_

## What this builds

Declarative `scenarios/*.toml` catalog entries (serde + garde `Scenario` model) keyed to Pulse capabilities
**P-017** (fingerprint identity / grouping) and **P-018** (storm-cue thresholds). This is a **scenario-config
chunk** in the Epoch-7 catalog series — it authors TOML, it does **not** add Rust model types, emission
primitives, faults, or dependencies. The exception-event + seeded fingerprint **emission primitive** it relies
on already shipped in `2026-06-18-exception-events-fingerprint-control` (conductor-emit, P-006/P-017/P-018).

### P-017 — fingerprint identity (the "triple" + the distinct pair) — `input.md:57-58,108`
- **Same fingerprint:** the variant triple — **identical stack · path-variant · line-variant** — ALL THREE
  resolve to ONE identical fingerprint (the **amended clause (c)**: the line-variant case expects an IDENTICAL
  fingerprint — line-number / path insensitivity). The emission primitive already realizes this
  line-insensitive expected fingerprint over identical/path/line variants.
- **Different fingerprint:** **type-variant** + **frame-variant** → DIFFERENT fingerprints (the grouping must
  NOT collapse a different exception type or a changed frame).

### P-018 — storm cue thresholds — `input.md:24,108`
- Storm one fingerprint repeatedly inside a 30s window: **6 hits / 30s → "Suggested" cue**; **12 hits / 30s →
  "Autonomous" hint**. (Headline brief shape: "storm a fingerprint 12 times in 30s".) The threshold counting
  is deterministic, so these are hard checks, not model-interpretive.

## Boundaries (what this chunk does NOT do)

- **Zero model change / zero new dependency / no golden re-baseline** — mirrors every prior Epoch-7 catalog
  chunk. Reuse the pre-existing `ComparisonKind` / `ExpectedCheck` / `Scenario` types and the existing
  fingerprint emission primitive. Adding TOML files joins no determinism golden silently (each loader test is
  an explicit `#[case]` stem; goldens load only `error-baseline-spike.toml`).
- **All `class = "Hard"`** per architecture §Probabilistic-Assertion Policy — fingerprint identity is a
  deterministic hash and storm-cue thresholds are deterministic counts (suppression/threshold logic). No
  CalibrationRegion leg here (model-interpretive severity is the later severity-lifecycle family, P-019..P-023).
- **Live storm/dedup detection + cue evaluation are Pulse's behavior** — verified at Epoch-8 runtime against
  `scenario.expected` via MCP read-back; this chunk declares the expected outcomes only.
- **Outcome-coherence file split** — a scenario's `expected: Vec<ExpectedCheck>` evaluates against ONE
  read-back with no per-check window/service scoping, so the same token must not carry contradictory
  `Contains`/`Absent` (or contradictory threshold) outcomes in one file. The exact file granularity (e.g.
  same-fp grouping vs distinct-fp vs the 6/12 cue tiers) is a **P4 AskUserQuestion** decision; any
  declare-only legs (an outcome the single-read-back evaluator can't disambiguate) are recorded in TOML
  comments + chunk notes for the Epoch-8 evaluator.

## Surfaces / contracts touched

- **`scenarios/*.toml`** (NEW) — the catalog entries (name · p_ids · seed · slo_tier · jitter_ms · `[[phases]]`
  · `[[expected]]` with `kind`/`class`/`expected`). New seeds in the `43170NN` family, distinct per file.
- **`crates/conductor-core/src/scenario.rs`** (MOD, tests only) — extend the per-family loader `#[rstest]`
  `#[case]` rows + the all-Hard / kind-usage guards to cover the new files (the established Epoch-7 test
  pattern). No production code in `scenario.rs` changes.

## Acceptance (intent-level — refined into criteria at P4)

1. New `scenarios/*.toml` parse + garde-validate via `Scenario::from_toml_str`; loader rstest cases green.
2. P-017 same-fp (identical/path/line) and distinct-fp (type/frame) outcomes both asserted, outcome-coherently
   split, line-insensitivity (clause (c)) reflected.
3. P-018 storm detection asserted via `Contains "RetryStorm"` (Hard); the 6→Suggested/12→Autonomous
   severity escalation is **declare-only**, deferred to P-020/P-021 (the catalog precedent + the P4
   defer-to-P-020 decision — severity choice is model-interpretive). All `[[expected]]` checks stay `class="Hard"`.
4. Gates green: conductor-core + workspace nextest, clippy `-D warnings`, doctest, `agent-run.sh run` exit 0;
   determinism goldens UNCHANGED.
