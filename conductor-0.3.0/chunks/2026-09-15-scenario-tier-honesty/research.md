# Codebase Research — 2026-09-15-scenario-tier-honesty

## Scope
- **Depth:** moderate · **Reads:** 7 (`execute.rs` 55-190 + 260-300 · `slo.rs` 1-70 · `scenario.rs` targeted regions · `drift.rs` 58-62 · `ack-cooldown.toml` full · 2 tauri fixtures) · **Globs/Greps:** 18
- **Harness rules consulted:** none owed — no live leg in this chunk. `.claude/rules/testing.md` and
  `.claude/rules/observability.md` auto-loaded on the Rust reads; two entries bear on the work and are
  applied below (the whole-`crates/` companion sweep, and the seed-named-golden rule).

## Files inspected
- `crates/conductor-run/src/execute.rs` (55-190, 260-300) — the scenario lifecycle. **Decides the chunk's
  central premise** (below) and holds the declare-only `manual_record` path.
- `crates/conductor-verify/src/slo.rs` (1-70) — `evaluate_slo`; `within_tolerance = (0..=deadline_ms).contains(&latency_ms)`.
- `crates/conductor-core/src/scenario.rs` — `SloTier` (52-76), `check_budgets` (189-215), and every
  committed-scenario tier assertion in the inline `#[cfg(test)]` module.
- `crates/conductor-core/src/drift.rs` (58-62) — the `constellation-severity-live-wiring` mention.
- `crates/conductor-tauri/tests/fixtures/scenarios/{fixture-alpha,fixture-beta}.toml` — the a11y routine arm's seeded fixtures.

## Graph impact (from the code-graph query)
- **`deadline_ms`** — 18 rows, rust plane, `db_state: regenerated`
  (`tree-query-2026-09-15-scenario-tier-honesty.json`). Callers are `conductor-verify/src/slo.rs` (the
  evaluator + its own tests) and `crates/conductor-verify/tests/expected_slo.rs:49`. No caller outside the
  verify seam, so the ladder's consumers are unaffected by a per-scenario re-declaration — the chunk changes
  DATA, not this function's callers.

## Patterns detected
- **The declare-only spine never grades an SLO** (`execute.rs:123-151`): when `scenario.expected.is_empty()`
  the run returns `manual_record(...)`, which sets `verdict: None`, `state: ManualCheck` and records
  `latency_ms` + `slo_tier` — `evaluate_check`/`evaluate_slo` are never reached. Its own doc comment states it:
  "a declare-only scenario asserts nothing".
- **Tier assertions live inline in `src/`, not under `tests/`** (`scenario.rs` `#[cfg(test)]` module) — the
  placement the testing rule's 2026-09-10 clause predicts, and the reason the sweep had to cover `crates/` whole.
- **Stated reasons ride the `slo_tier` line as a trailing comment** (31 of 36 scenarios), the affordance the
  TOML-over-JSON decision bought.

## Conventions to follow
- **Re-declare the tier, never the duration**: `elapsed_ms` is a deterministic cumulative sum of effective
  gaps, so editing `gap_ms` changes the seed-reproduced stream shape (`scenario.rs` model + arch §Determinism RNG).
- **Name the set, never a fresh literal** — the de-literalization rule that governs any doc/sample tier cell.
- **A wide name sweep over-returns**: `grep -rl` over `crates/` for the 8 names returned 13 file-hits; reading
  each shows only `scenario.rs` carries assertions, the rest being doc comments and inline test TOMLs.

## New files to create
- (none)

## Files to modify
- `scenarios/ack-cooldown.toml` — Situation 2: `<20s` → `<90s`; its stated reason rests on a retired premise (below).
- `scenarios/severity-tier-autonomous.toml` — Situation 2: `<5s` → `<90s` + stated reason.
- `scenarios/constellation-severity-live-wiring.toml` — Situation 3: `<20s` → `<90s`.
- `scenarios/severity-tier-suggested.toml` — Situation 3: `<20s` → `<90s`.
- `scenarios/findings-counter-refresh.toml` · `scenarios/report-render-surface.toml` ·
  `scenarios/service-constellation-discovery.toml` · `scenarios/threshold-hot-reload.toml` — Situation 3:
  `<5s` → `<20s` each (6 000 ms summed).
- `scenarios/auto-resolve-idle-window.toml` · `scenarios/incident-auto-resolution.toml` ·
  `scenarios/halo-hue-encoding.toml` — Situation 1, **tier unchanged**; only the stated reason is corrected
  where it asserts a retired premise.

**Caller threading / companion sweep (whole-`crates/` name sweep, every hit dispositioned):**
`grep -rl "<name>" crates/` over the 8 re-tiered scenarios → 13 file-hits across 5 distinct files.
- `crates/conductor-core/src/scenario.rs` — all 8 named. **No change**: the six committed-scenario tier
  assertions cover `error-baseline-spike` + `latency-regression` (`Tier90s`),
  `high-severity-log-capture` (`Tier5s`), `restart-suppression` (`Tier90s`), and
  `fingerprint-storm` + `fingerprint-distinct` (`Tier90s`) — **none of the 8**. The 8's own mentions are
  catalog/declare-only assertions that read `expected.is_empty()`, not the tier.
- `crates/conductor-run/tests/operator_pause_harvest.rs` (`ack-cooldown`) — **no change**: its three
  `slo_tier` occurrences are self-contained inline test TOMLs (`gap_ms = 100`, `<5s`), not the committed file.
- `crates/conductor-run/tests/severity_harvest.rs` (`ack-cooldown`, `severity-tier-autonomous`) — **no change**: zero tier references.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (`service-constellation-discovery`) — **no change**: zero tier references.
- `crates/conductor-core/src/drift.rs` (`constellation-severity-live-wiring`) — **no change**: a doc comment
  recording which chunk retired `P-079` from `UNBACKED_AUTO`; carries no tier.
- **Goldens: none.** `grep -rln 'slo_tier' crates/*/tests/snapshots/` → 0, and no golden names any of the 8.
  The seed-named-golden rule does not bite because no `gap_ms`/`seed`/`jitter_ms` changes.
- **`contracts/pulse-load-envelope.toml`: unchanged**, and its verdicts are unmoved — its asserted terms are
  per-emitting-phase and `max_scenario_duration_ms` is recorded-not-asserted; no duration changes at all.
- **`crates/conductor-tauri/tests/fixtures/scenarios/`: unchanged.** Both fixtures declare a tier
  (`fixture-alpha` `<5s`, `fixture-beta` `<20s`) and both are honest — one phase at `gap_ms = 10`. They are
  their own scenarios, not copies of the 36, so the a11y routine arm needs no re-run.

**Load-bearing mechanism, stated as the equality the design needs, and verified at HEAD:**
`latency_ms = read_back_observed_at − journal_emitted_at` must be ≥ the scenario's summed phase duration —
i.e. the tier genuinely has to hold that sum. Verified by reading the fire site: `execute.rs:68-69` stamps
`emitted_ms` / `journal_emitted_at` **before** `run_timeline_observed` (line 75) runs the whole timeline;
lines 120-121 stamp `observed_ms` / `read_back_observed_at` **after** the timeline completes and read-back
returns; line 148 (and `to_run_record`) carries `observed_ms - emitted_ms`. So the measured latency spans
every phase plus the read-back, and a scenario whose gaps sum to 370 000 ms can never meet a 20 000 ms tier.
Independently corroborated by the 2026-09-10 live measurement (declared 18 s / 35 s / 30 s → `latency_ms`
18 169 / 35 120 / 30 212, sidecar calls 0–22 ms), whose third pair is this chunk's
`constellation-severity-live-wiring` figure.

**`check_budgets` cannot break on a re-tier:** no committed scenario declares `budget_ms` at all
(`grep -rn 'budget_ms' scenarios/` → 4 hits, every one inside a comment, none a key). Both live
`[[expected]]` blocks (`root-span-error-scope`, `span-status-error-detection`) declare only
`kind`/`class`/`expected`, and neither scenario is being re-tiered.

## Scope premise closure
- **Premise 1 — VERIFIED** (was `[inferred]`): the SLO does measure whole-run latency, per the fire-site read
  above. The tension is resolved **against** the scenario comments: three files assert "the … window is phase
  timing, not the SLO budget", and that is false at HEAD. Situation 3 is therefore a genuine **re-tier**, not
  a comment correction — and the affected stated reasons are themselves wrong.
- **Premise 2 — `[premise-corrected: at HEAD `constellation-severity-live-wiring` declares zero `[[expected]]`
  blocks, so the dead-assertion half was already discharged by the two preceding chunks; only its tier defect
  remains]`.** v3-05's `observed_gap` describes a state that has since moved. Its 30 212 ms measured figure
  still matches this scenario's 30 000 ms summed gaps against a 20 000 ms declared deadline.
- **Premise 3 — VERIFIED with a corrected basis** (was `[inferred]`): the honest-bucket posture is evidenced
  by the `2026-08-18-error-baseline-spike-live-proof` amendment (that scenario re-tiered `<5s` → `<90s` on the
  "whole-run latency exceeds every tier by construction" rationale, swept across five sample sites) and by the
  committed comment at `scenarios/halo-hue-encoding.toml:62`. **No master clause uses the words
  "operator-ratified"** — that phrasing originates in v3-05's `observed_gap`. The plan should cite the
  amendment precedent and the committed comment, not a master ratification clause.

## Findings scope did not name
- **The property is currently unenforced for 34 of 36 scenarios.** Only the two with live `[[expected]]`
  blocks reach `evaluate_slo`; every declare-only scenario takes `manual_record`, which records the tier and
  grades nothing. This is why the honesty state can be wrong at all, and it is the obs §11 "no soft budgets"
  tension resolved: the budget is not soft, it is **absent** on that path. It is also exactly what the sibling
  entry (v3-06) exists to fix — so this chunk should state the state, not build the gate.
- **Seven scenarios carry a retired premise in their stated reason**, and fixing the tier without fixing the
  reason would leave the file arguing for the wrong tier. Two wordings, disjoint sets:
  - the retired "MCP round-trip" gloss of the latency formula (retired 2026-09-10) — **4** files:
    `findings-counter-refresh` · `halo-hue-encoding` · `report-render-surface` · `service-constellation-discovery`.
    Derivation: a comment-marker-stripped, wrap-tolerant sweep. A plain `grep -l 'MCP round-trip' scenarios/*.toml`
    returns **3** — `service-constellation-discovery` wraps the phrase across two comment lines, so the
    single-line form is a false negative.
  - "…is phase timing, not the SLO budget" — **3** files (`grep -n`): `ack-cooldown` ·
    `auto-resolve-idle-window` · `incident-auto-resolution`.
- **Four scenarios already state the correct reading** ("latency ≈ scenario duration"): `last-span-ago-tracking` ·
  `orthogonal-health-domains` · `receiver-failed-port-conflict` · `receiver-lifecycle-state` — the wording precedent to follow.

## Open questions
- Should the three Situation-1 files whose stated reason rests on a retired premise be corrected here, or is
  that out of scope for a chunk whose route entry speaks only of tiers "fitting or stating why"? → blocks:
  **plan-decision**. (A wrong stated reason is a dishonest declaration under the entry's own words, so the
  decisive lean is to correct them; P4 states the lean and its basis.)
