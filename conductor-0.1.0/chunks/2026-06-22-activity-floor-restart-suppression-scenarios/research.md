# Codebase Research — 2026-06-22-activity-floor-restart-suppression-scenarios

## Scope
- **Depth:** moderate · **Reads:** 9 (model ×3, TOMLs ×4, timeline tests ×2) · **Globs/Greps:** 4
- **Headline:** the chunk is **zero-model-change** — the `Absent` comparison kind and the phase model already
  support every leg. Implementation = **3 (or 2) new `scenarios/*.toml` + `scenario.rs` `#[case]` test rows**.

## Files inspected
- `crates/conductor-core/src/expected.rs` (full) — **`ComparisonKind { Exact, Contains, Absent, CountAtLeast }`
  (line 35-45) — `Absent` = "observed does NOT contain the expected token" (line 41-42) ALREADY EXISTS** →
  **resolves the headline Open Q1** (suppression absence needs no model change). `ClaimClass { Hard,
  CalibrationRegion }` (line 21-26) — doc literally lists "suppression/bypass logic, lifecycle timing" as
  `Hard` (line 17-18). `ExpectedCheck { kind, class, expected: String (garde length min=1) }` (line 53-64) —
  `expected` is a token (or the decimal floor for `CountAtLeast`). Wire forms are canonical PascalCase
  (`"Absent"`, line 88).
- `crates/conductor-core/src/phase_spec.rs` (full) — **`PhaseSpec { name (1..=40 chars), gap_ms (≤ MAX_GAP_MS
  = 3_600_000 = 60min), emission }`** (line 22-37). `EmissionSpec { signal }` is `#[non_exhaustive]`, carries
  ONLY the `Signal { traces|metrics|logs }` class (line 43-70) — **no error-rate / magnitude / count / duration
  field** → the bypass-triple magnitudes (8×/3% …) and the fault kind (silence/gap/train) are NOT structured
  TOML data; they live in **phase NAMES + comments**, realized by the Epoch-8 driver. `MAX_JITTER_MS = 60_000`.
  The `MAX_GAP_MS` doc even cites "the Epoch-4 bursty-train ~10-minute quiet window" as the sizing rationale.
- `crates/conductor-core/src/scenario.rs` (full) — `Scenario { name, p_ids, seed, slo_tier, phases, jitter_ms,
  expected }`. **The fixture-loader test pattern I extend** (line 300-395): per-family `#[rstest] #[case(stem,
  p_id(s))]` asserting `name` + `p_ids` + `!expected.is_empty()` (connection-lifecycle 300-312, hard-signal
  314-326, statistical-anomaly 328-339 with multi-P-ID `&[&str]`); an **all-Hard + kind-presence guard**
  (`statistical_anomaly_checks_are_hard_with_floor_and_candidate` 341-363); and the **two-sided `Contains` +
  `Absent` assertion** (`p007_…asserts_both_sides_of_the_boundary` 379-395) — the exact template for the
  activity-floor Absent leg.
- `scenarios/high-severity-log-capture.toml` (P-007) — **the `Absent` precedent in a TOML**: `Contains "ERROR"`
  + `Absent "WARN"`, BOTH `class="Hard"`, present-token ≠ absent-token (no contradiction). `emission =
  { signal = "logs" }` per phase.
- `scenarios/receiver-failed-port-conflict.toml` (P-003) — **the fault-as-phase-name pattern**: the `:4317`
  bind "is driven at run time by the existing conductor-faults port-occupier (Epoch-8 driver) — this config
  does NOT bind the port itself" (comment, line 3-4); phase `name = "port-held"` + `gap_ms`. Confirms Q3:
  the TOML only *times+labels* phases; faults are Epoch-8-realized.
- `scenarios/receiver-lifecycle-state.toml` (P-001) — long-window precedent: a 4-phase state walk, `gap_ms`
  encodes durations (15000 idle / 50000 stalled), `slo_tier = "<90s"` because "the walk reaches Stalled (>60s
  of accumulated silence)".
- `scenarios/latency-regression.toml` (P-011/P-012) — the **persistence-tier + inferred-token precedent**:
  `slo_tier = "<20s"` for a 60s-persistence signal ("P-012 has no explicit p99 budget; the 60s-persistence
  signal warrants the middle tier"); `CountAtLeast` floor + `Contains` candidate; an inferred token with an
  "Epoch-8 calibration point" comment.
- `crates/conductor-timeline/tests/replay.rs` (full) — the **seed-named determinism golden loads ONLY
  `error-baseline-spike.toml`** (hardcoded path, line 21-24); the proptest properties use `arb_timeline()`
  (synthetic, in-code). My new TOMLs do **not** feed it.
- `crates/conductor-timeline/tests/determinism.rs` (grep) — **no scenario-TOML load at all** (0 matches for
  `scenarios/` / `from_toml_str` / `.toml`).

## Graph impact (code-graph query → `tree-query-{marker}.json`)
- **The consumed types are heavily-referenced shared symbols — consume, do NOT modify:** `ComparisonKind @
  expected.rs:34` (**55 refs**), `Scenario @ scenario.rs:68` (**29 refs**), `Scenario::expected @
  scenario.rs:95` (**19 refs**), `ExpectedCheck @ expected.rs:53` (**18 refs**), `ComparisonKind::Contains @
  expected.rs:39` (**13 refs**). A signature change to any would ripple across conductor-verify/report — which
  is exactly why **reusing the existing `ComparisonKind::Absent` variant** (zero new symbol) is the right call:
  the change is additive scenario *data* + test-only `#[case]` rows, **zero symbol-signature change**, so no
  new caller/impact edge is created. A `crates/` grep also confirmed **no `read_dir`/glob of `scenarios/`** —
  every loader is an explicit `#[case]` stem, so adding files joins no test silently (the `#[case]` rows are
  mandatory, not automatic).

## Patterns detected
- **Per-family fixture rstest** (`scenario.rs:300-339`): `#[case(stem, p_id(s))]` → assert `name == stem`,
  `p_ids`, and `!expected.is_empty()`. Multi-P-ID uses `&[&str]` (statistical-anomaly, 328-339).
- **All-Hard + kind-presence guard** (`scenario.rs:341-363`): `s.expected.iter().all(|c| c.class ==
  ClaimClass::Hard)` + `.any(|c| c.kind == ComparisonKind::X)`. The `p007` test (379-395) asserts BOTH a
  `Contains` and an `Absent` are present — the template for the activity-floor Absent guard.
- **Fault realized by phase-name, not bound in config** (`receiver-failed-port-conflict.toml:3-4`): silence /
  gap / train / burst / bypass become phase *names* + explanatory comments; the Epoch-8 driver maps name →
  fault helper (`abrupt-silence-fault` / `emission-gap-resume` / `bursty-train-pattern`).
- **`Absent` only when present-token ≠ absent-token** (`high-severity-log-capture.toml`): never assert
  `Contains X` and `Absent X` in the SAME scenario — they contradict on one read-back snapshot (the
  scenario-level `Vec<ExpectedCheck>` is evaluated against one read-back, with no per-check phase/service
  scoping). This is the forcing function behind the file-granularity question.

## Conventions to follow
- **TOML header comment block** (every prior TOML): P-ID(s) + `pulse-capability-spec.md` ref + the
  inject→observe recipe + `Coverage mode:` line + any Epoch-8 calibration points.
- **Seed convention `4317<pid>`**: receiver-lifecycle `4317001`, P-003 `4317003`, P-007 `4317007`,
  latency-regression `4317011` → activity-floor `4317013`, service-went-silent `4317014`, restart-suppression
  `4317015`. Distinct, and they feed no golden.
- **`jitter_ms = 50`** (family default; `0` only for the strict-timing port-conflict). `class = "Hard"` for
  every check (Probabilistic-Assertion Policy: suppression/bypass + lifecycle timing).
- **SLO tier per the asserted surfacing leg**: long-window / absence → `<90s` (receiver-lifecycle precedent);
  persistence-gated surfacing → `<20s` (latency-regression precedent, "60s-persistence warrants the middle
  tier"). RestartEvent ≤2s fits comfortably inside either.
- **Scenario name must match obs-plan §4** for the must-trace scenario: restart-suppression's `scenario` name
  is **`"restart-suppression"`** (obs-plan §4 fixes `timeline.execute_restart_suppression`, `path_type`
  canonical/bypass, `bypass_triggered` — all **Epoch-8 runtime** instrumentation, NOT this chunk's TOML).

## New files to create  (final shape pending the P4 granularity decision — 3-file recommended)
- `scenarios/activity-floor.toml` — **P-013**: bursty-train (5min active / 10min quiet ×N) → 30-min lunch →
  `Absent "ServiceWentSilent"` (`Hard`). `slo_tier="<90s"`. (Out: the spec's "restart Pulse, verify histogram
  restoration" leg — Conductor does not manage the Pulse process.)
- `scenarios/service-went-silent.toml` — **P-014**: fresh 5-min activity → abrupt stop → `Contains
  "ServiceWentSilent"` after 30s (`Hard`). `slo_tier="<90s"` (cue at ~30s > 20s).
- `scenarios/restart-suppression.toml` — **P-015/P-016/P-057**: consistent traffic → 25s gap+resume →
  `Contains "RestartEvent"` (P-015); within the 60s window a sustained 60s stream + the bypass pair (12×/4%,
  6×/7%) → `Contains "ErrorRateSpike"` (P-016 surgical-surface + P-057 bypass-surface). The **suppressed legs**
  (15s transient, 8×/3%) are **declare-only** (an `Absent "ErrorRateSpike"` would contradict the present
  sustained/bypass spikes on one read-back; Epoch-8 scopes suppression by window/magnitude). `slo_tier="<20s"`.

## Files to modify
- `crates/conductor-core/src/scenario.rs` — add (a) an `activity_floor_*` / `restart_suppression_*` per-family
  `#[rstest] #[case]` loader block (assert `name`/`p_ids`/`!expected.is_empty()`), (b) an all-`Hard` guard for
  the new scenarios, and (c) an `Absent`-usage assertion for `activity-floor` (mirrors `p007_…both_sides` +
  `statistical_anomaly_checks_are_hard…`). **No production code change.**

## Open questions
- **Q-granularity (P4 AskUserQuestion):** 3 files (split so both P-013 *and* P-014 get a real check — `Absent`
  vs `Contains` "ServiceWentSilent" can't coexist in one scenario) **[recommended]** vs 2 files (input.md
  family digest + the ch3 2-file precedent, but one of P-013/P-014 becomes declare-only). The same-token
  contradiction is the forcing function.
- **Q-tier:** restart-suppression `<20s` (latency-regression persistence precedent; lean) vs `<90s`. Lean
  `<20s`; the persistence-gated surface is an Epoch-8 calibration point.
- **Q-tokens (resolved):** "ServiceWentSilent" / "RestartEvent" / "ErrorRateSpike" are all spec-named (fewer
  inferred tokens than ch3); the P-057 bypass *annotation* ("Bypassed restart suppression due to…") is prose —
  the bypassed spikes still surface as `ErrorRateSpike`, so no inferred token is needed (note it for Epoch-8).
