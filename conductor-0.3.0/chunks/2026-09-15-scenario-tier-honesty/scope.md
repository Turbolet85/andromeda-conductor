# Scope — Scenario tier honesty

**Marker:** `2026-09-15-scenario-tier-honesty`
**Working entry (verbatim):** Scenario tier honesty — every declared tier fits its phase duration inside the
closed tier set or states why, three situations kept distinct
**Annotations folded:** none — the entry carries no `PREREQ:` / `CARRY:` / `BLOCKED-ON:` / `CONTEXT:` /
`BLOCKING:` introducer at an annotation position (measured: the 143-byte line holds no ALL-CAPS introducer
after a separator).

## What this chunk builds

Every committed scenario's declared `slo_tier` becomes honest about that scenario's own summed phase
duration — either the tier holds the duration, or the file states why it does not — with the three
situations behind today's mismatches kept **separately identified** rather than collapsed into one defect
class. The closed tier set (`<5s` / `<20s` / `<90s`) is fixed: no scenario becomes honest by inventing a
new tier.

## The measured ground

Re-measured this session over all 36 committed scenarios (summed `gap_ms` per file against
`SloTier::deadline_ms()`); the partition reproduces `verification-matrix.json#v3-05`'s `observed_gap`
exactly — 9 + 2 + 6 = 17 over-tier, 19 fitting.

- **19 scenarios fit** their declared tier as written. Nothing owed: the acceptance requires a stated
  reason only for a tier that does NOT hold its duration.
- **Situation 1 — ceiling declared and exceeded (9):** `activity-floor` · `auto-resolve-idle-window` ·
  `error-baseline-spike` · `halo-hue-encoding` · `incident-auto-resolution` · `latency-regression` ·
  `pulse-run-contract` · `restart-suppression` · `service-went-silent`. Each declares `<90s`, the ceiling
  of the closed set, and runs past it. This is the operator-ratified honest-bucket posture — **not a
  defect** — and must stay recorded as posture.
- **Situation 2 — exceeds every tier while declaring a smaller one (2):** `ack-cooldown` (`<20s`,
  370 000 ms) · `severity-tier-autonomous` (`<5s`, 120 000 ms). No tier in the closed set holds these, so
  the honest declaration is the ceiling plus a stated reason — the same posture Situation 1 already
  occupies.
- **Situation 3 — a larger existing tier would hold it (6):** `constellation-severity-live-wiring`
  (`<20s`, 30 000 ms) · `findings-counter-refresh` (`<5s`, 6 000 ms) · `report-render-surface` (`<5s`,
  6 000 ms) · `service-constellation-discovery` (`<5s`, 6 000 ms) · `severity-tier-suggested` (`<20s`,
  82 000 ms) · `threshold-hot-reload` (`<5s`, 6 000 ms). These are the genuine defects: a tier already in
  the closed set holds the duration, so the scenario is simply mis-declared.
- **The five bare declarations are not a fourth situation.** `exception-event-capture` ·
  `fingerprint-distinct` · `fingerprint-storm` · `high-severity-log-capture` · `root-span-error-scope`
  declare a tier with no trailing comment, and all five **fit** — so the acceptance owes them nothing.
  The intersection of "over tier" and "no stated reason" is empty.

## Boundaries

- **No mechanical gate here.** The re-runnable check that would enforce this property belongs to the next
  working-route entry (`Scenario-assertion audit gate`, `verification-matrix.json#v3-06`), which owns both
  this outcome and the dead-assertion outcome. This chunk establishes the *state*; the sibling makes it
  *enforced*. [inferred — read from v3-06's acceptance, which explicitly claims the one-check outcome]
- **Tier declarations move, phase durations do not.** Honesty is reached by re-declaring the tier within
  the closed set or by stating the reason — never by editing `gap_ms` to fit a tier, which would change
  what the scenario drives. [inferred — the entry fixes the tier set, not the remedy]
- **No change to the tier ladder itself.** `SloTier`'s three variants and their `deadline_ms()` values are
  untouched.
- **Not a live-Pulse chunk.** Nothing here drives a run; the work is over committed scenario files and the
  tests that pin them.

## Surfaces and contracts touched

- `scenarios/*.toml` — the `slo_tier` line of the scenarios in Situations 2 and 3, and the stated-reason
  comment wherever a tier does not hold its duration.
- `crates/conductor-core/src/scenario.rs` — `SloTier` read-only reference (the closed set + `deadline_ms`).
- Tests pinning a specific scenario's tier or deadline. [inferred — a re-tier changes
  `deadline_ms()` for that scenario, so any test asserting the old tier or its derived deadline moves with
  it; P3 must enumerate them before the plan commits to a file list]
- `contracts/pulse-load-envelope.toml` — expected **untouched**: the envelope bounds scenario *duration*,
  and no duration changes. [inferred — to confirm at P3]

## Premises — closed at P3

- **VERIFIED — The SLO measures whole-run latency.** `execute.rs:68-69` stamps `journal_emitted_at`
  BEFORE `run_timeline_observed` (line 75) runs the whole timeline; lines 120-121 stamp
  `read_back_observed_at` AFTER the timeline and read-back complete; line 148 carries
  `observed_ms - emitted_ms`. The declared tier therefore genuinely must hold the summed phase duration.
  **Situation 3 is a re-tier, not a comment correction** — and the committed comments asserting the
  opposite ("the … window is phase timing, not the SLO budget", 3 files) are false at HEAD.
- **`[premise-corrected: at HEAD `constellation-severity-live-wiring` declares zero `[[expected]]` blocks,
  so the dead-assertion half was already discharged by the two preceding chunks; only its tier defect
  remains]`** — v3-05's `observed_gap` describes a state that has since moved. Its 30 212 ms measured
  figure still matches this scenario's 30 000 ms summed gaps against a 20 000 ms declared deadline.
- **VERIFIED, corrected basis — Situation 1's posture.** Evidenced by the
  `2026-08-18-error-baseline-spike-live-proof` amendment (that scenario re-tiered `<5s` → `<90s` on the
  "whole-run latency exceeds every tier by construction" rationale) and the committed comment at
  `scenarios/halo-hue-encoding.toml:62`. No master clause uses the words "operator-ratified"; that
  phrasing originates in v3-05's `observed_gap`. Cite the precedent, not a master ratification clause.

## Closed at P3 — boundary confirmations

- **VERIFIED** — no mechanical gate here; v3-06's acceptance explicitly claims the one-check outcome.
- **VERIFIED** — no test pins any of the 8 re-tiered scenarios' tiers. The six committed-scenario tier
  assertions in `scenario.rs` cover `error-baseline-spike`, `latency-regression`, `restart-suppression`,
  `fingerprint-storm`, `fingerprint-distinct` and `high-severity-log-capture` — all unchanged here.
- **VERIFIED** — `contracts/pulse-load-envelope.toml` is untouched and its verdicts unmoved (asserted
  terms are per-emitting-phase; `max_scenario_duration_ms` is recorded-not-asserted; no `gap_ms` changes).
- **VERIFIED** — no golden carries `slo_tier` or names any of the 8; `check_budgets` cannot break,
  because no committed scenario declares a `budget_ms` key at all.
- **NEW, scope did not name it** — the honesty property is currently unenforced for 34 of 36 scenarios:
  only the two with live `[[expected]]` blocks reach `evaluate_slo`, while every declare-only scenario
  takes `manual_record`, which records the tier and grades nothing.
- **NEW, scope did not name it** — seven scenarios state a reason resting on a retired premise (4 on the
  round-trip gloss, 3 on the phase-timing formula), so a tier fix that left the reason standing would
  leave the file arguing for the wrong tier.
