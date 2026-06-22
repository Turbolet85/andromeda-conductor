# Scope — Severity-lifecycle scenarios (P-019..P-023, P-059, P-060)

**Marker:** `2026-06-22-severity-lifecycle-scenarios`
**Version:** conductor-0.1.0 · **Epoch 7 (Scenario catalog) — chunk 6 of 8**
**Working-route intent:** _Severity-lifecycle scenarios — tiered inputs, auto-resolve, ack-retrigger, per-tier SLO (P-019..P-023, P-059, P-060)_

## What it builds

The **severity-calibration + lifecycle family of the `scenarios/` catalog** — declarative TOML keyed to Pulse's
"Severity Calibration" category (`pulse-capability-spec.md` §5: P-019..P-023 + P-060) plus the pipeline-category
interpretation-continuity capability **P-059**.

**This is the inflection chunk of the catalog.** Every prior Epoch-7 family was all-`Hard` and explicitly
DEFERRED severity choice "to P-020". This chunk *is* P-020 — so it is the **first family whose headline
capability is model-interpretive**, and therefore the first to carry **`class = "CalibrationRegion"`** legs as
the *dominant* mode, mixed with a deterministic-lifecycle-timing `Hard` spine. The architecture's
Probabilistic-Assertion Policy draws the line exactly here: *severity choice* is calibration-region (never
hard-failed on exact values), while *lifecycle timing* (auto-resolve, cool-down) and *tier→SLO routing* stay hard.

The seven capabilities, by class:

- **P-019 — Three-Tier Severity Model** (Autonomous / Suggested / Curious). The tier *choice* is model-driven
  (P-020) → **CalibrationRegion**. The monotonicity invariant (escalation permitted; **de-escalation forbidden**
  once Autonomous) is a deterministic lifecycle rule — declare-only here (a multi-read-back temporal assertion the
  single-shot Epoch-8 evaluator owns).
- **P-020 — Model-Driven Severity Decision** (the LLM is the *sole* user-facing severity mechanism). Spec is
  explicit: "Pulse is not a deterministic classifier … Conductor scenarios test calibration regions, not exact
  values." → **CalibrationRegion** (routes to `ManualCheck`). Also defines graceful degradation (model-off ⇒ hard
  signals surface as Suggested, statistical/pattern go silent) — referenced, owned by the later P-045 model-off family.
- **P-021 — Algorithmic Attention Cues** (threshold detectors feed the model; "no user-facing surface of their
  own, never become incidents by themselves"). Observable signal: *None directly*. Verified **THROUGH P-020** — the
  model's surfaced incident references the cue in its Report evidence; a cue the model correctly dismisses yields no
  incident. No standalone hard assert (the underlying detectors are P-009..P-014 / P-017..P-018, already cataloged).
- **P-022 — Auto-Resolution and Lifecycle** (signal ceases ⇒ Active→**Resolved** after **120 s**; resolution
  summary per P-059; subsequent same-fingerprint detection creates a NEW incident, not a reopen). The **120 s
  transition + new-not-reopen** are deterministic lifecycle timing → **Hard**; resolution-summary *quality* is
  CalibrationRegion (declare-only).
- **P-023 — Acknowledge Cool-Down** (**5-min** cool-down after ack; retrigger within ⇒ no new active incident but
  corpus records the occurrence; retrigger after ⇒ new incident; per kind-and-scope, not global). Cool-down window
  logic is deterministic → **Hard**. **The open risk:** "acknowledge" has no obvious tool in the pinned four-tool
  MCP contract (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`)
  — whether the ack step is drivable (via `mark_incident_resolved`?) or is an operator-checklist action is a
  P4/Epoch-8 question.
- **P-059 — Active-Incident Interpretation Continuity** (active incident exempt from LWW; each digest yields its
  own interpretation; resolution summary on close; bounded at 5 concurrent). "Multiple interpretation updates over
  the lifecycle" is model-interpretive → **CalibrationRegion**; the resolution-summary-attaches fact is observable.
- **P-060 — Tiered Triggering Priority** (Tier-1 hard signals → **<5 s**; Tier-2 medium cues conf 0.7–0.85 →
  **<20 s**; Tier-3 baseline → **<90 s**; hardware-profile-dependent). The tier→budget routing maps **directly onto
  the existing `SloTier` enum** (`<5s`/`<20s`/`<90s`). Tier *assignment by trigger kind* is deterministic →
  **Hard**; the end-to-end latency *measurement* is Epoch-8's (it requires the model to surface).

This is **catalog wiring over stimuli that already exist** — no new emit or fault primitive. A severity tier is
produced by an *existing* signal at a calibrated magnitude (error span = Tier-1 candidate; error-rate / latency /
retry-storm cue = Tier-2/3); auto-resolution is "trigger then *cease*" (the Epoch-4 abrupt-silence / gap lever);
cool-down and continuity are re-emit + sustain (existing error-rate ramps). The
`Scenario`/`ExpectedCheck`/`Phase`/`ClaimClass`/`SloTier` TOML surface was wired in ch1, and the `CalibrationRegion`
class was first exercised by P-008 in ch2 — so this chunk *consumes* both. **Target: zero model change, zero
dependency, no golden re-baseline** (the Epoch-7 invariant). **No live MCP run here** (Epoch-8 driver / Epoch-10 E2E).

## Requirement source of truth

- **`.andromeda/refs/pulse-capability-spec.md` §5 (P-019..P-023, P-060) + P-059** — THE normative source; each
  P-ID's "Conductor verification" clause is the concrete recipe (tier injection; trigger→cease→120 s→Resolved;
  ack→retrigger within/after; per-tier latency; sustained-incident multiple-interpretation).
- **`.andromeda/architecture.md` §Probabilistic-Assertion Policy** — the two-state split that puts *severity
  choice* in CalibrationRegion and *lifecycle timing* in Hard; the default `Verdict → ReportState` mapping
  (`CalibrationRegion → ManualCheck`, verdict-first lamp). **§Timing-Tolerance Model** — the `<5s`/`<20s`/`<90s`
  tiers (= P-060's Tier-1/2/3) + model-interpretive timings routing to the calibration-region bucket.
- **P-008 CalibrationRegion precedent** (`chunks/2026-06-21-hard-signals-scenarios`) — the shipped template for a
  `class="CalibrationRegion"` expected block routed to `ManualCheck`.
- **Existing precedent** — ch1–ch5 `scenarios/*.toml` (catalog shape `name · p_ids · seed · slo_tier · jitter_ms ·
  [[phases]]{name,gap_ms}` + `[[expected]]{kind,class,expected}`); the fingerprint-storm / activity-floor two-file
  outcome-coherence split.

## Boundaries

- **In:** the severity-lifecycle scenario TOML(s) under `scenarios/` (P-019..P-023, P-059, P-060); their
  `[[phases]]` (tier-calibrated stimuli / trigger→cease→120 s / ack→retrigger-within-&-after / sustained / per-tier
  triggers) + `[[expected]]` checks (**mixed `Hard` + `CalibrationRegion`**); per-scenario SLO tier (P-060's
  Tier-1/2/3 = `<5s`/`<20s`/`<90s`); coverage-mode header comments; fixture round-trip tests proving each
  deserializes + garde-validates + builds a valid `PhaseTimeline`.
- **Out:**
  - the live MCP read-back *run* + the actual model severity decision / latency measurement (Epoch-8 driver /
    Epoch-10 E2E);
  - the **model itself** — Conductor injects stimuli + declares expected tiers/outcomes; it does NOT judge
    hypothesis quality (the model is Pulse's, the black box under test);
  - any **new emit / fault primitive** (all stimuli exist from Epoch 3/4: error spans, error-rate ramps, latency
    shaping, fingerprint storm, abrupt-silence / emission-gap-resume);
  - **graceful-degradation / model-off** behavior (P-020's reduced mode) — the later P-045 model-off family
    (Epoch-7 ch8), referenced not authored here;
  - the **visual** legs of P-019 (halo magnitude, counter increment, dropdown placement) beyond a declared
    operator-checklist note — desktop/operator surfaces (Epoch 9 + the operator checklist), not auto-MCP;
  - the constellation / context-grounding + scrub/pipeline families (later Epoch-7 chunks); the CLI/desktop surfaces
    (Epoch 8/9).
- **Scope law:** every scenario carries its P-IDs; **no new inbound listener** — pure-egress (tier stimuli + the
  cease/resume legs are starts/stops of egress; the port-occupier P-003 is ch1).

## Surfaces / contracts touched

- **Artifact:** new `scenarios/*.toml` severity-lifecycle file(s) — the primary deliverable. New distinct seeds in
  the established `43170NN` family, one per file.
- `conductor-core::{Scenario, ExpectedCheck, Phase}` (`scenario.rs`, `expected.rs`) — *consumed* via
  `from_toml_str`; extended **only if** a lifecycle-status assertion has no expressible `ComparisonKind` (Open Q3) —
  the one candidate model touch.
- `conductor-core::{ClaimClass, ComparisonKind, SloTier}` — **class is mixed here** (`Hard` for P-022/P-023 timing
  + P-060 routing; `CalibrationRegion` for P-019/P-020/P-059 model-interpretive legs); comparison kinds
  (`Contains`/`Absent`/`CountAtLeast` from ch1–ch5 — e.g. `Contains "Resolved"`, `Absent` new-incident within
  cool-down); per-scenario tier = P-060's mapping.
- Emit / fault levers **referenced, not modified:** error-spans / error-rate (tier stimuli + sustained incident),
  latency-shaping (Tier-2 regression cue), fingerprint-storm (retry-storm cue), abrupt-silence / emission-gap-resume
  (the "cease signal" for auto-resolution).
- Read-back/detector-output **tokens** declared in `[[expected]]`: `Autonomous` / `Suggested` / `Curious` (tiers),
  `Resolved` (lifecycle status) — inferred from spec prose, asserted substring-tolerantly via `Contains` (the
  `LatencyRegression` / `RetryStorm` precedent). Per the playbook rule-49 broadening (handoff 2026-06-22) these
  tokens do NOT re-fire the arch §Occupied-Resources drift.
- **Determinism goldens:** if any new TOML feeds the **seed-named** replay goldens
  (`crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<N>.snap`), re-baseline — grep the new **seed**
  values (the ch3 deviation). Goldens load only `error-baseline-spike.toml` today, so this should stay UNCHANGED.
- Requirement contract: `pulse-capability-spec.md` per-P-ID "Conductor verification" clauses.

## Open questions — resolve in planning (P4)

1. **File granularity (headline AskUserQuestion).** The working entry's own decomposition suggests **4 files**:
   `severity-tiers` (P-019/P-020/P-021), `incident-auto-resolution` (P-022/P-059), `ack-cooldown` (P-023),
   `tiered-slo-priority` (P-060). Confirm 4 vs fewer-combined vs a finer split (the P-019 three tiers or the P-023
   within/after may each force sub-files for outcome-coherence — see Q2).
2. **Outcome-coherence splits within the family.** (a) **P-023** asserts *Absent new incident within* the 5-min
   window AND *Contains new incident after* — contradictory on the "new incident" token under the single-read-back
   evaluator (the activity-floor / fingerprint precedent) → likely a within/after file or phase split. (b) **P-019**
   three tiers (Autonomous/Suggested/Curious) are mutually exclusive outcomes for one incident → likely three
   calibrated stimuli, possibly separate scenarios. Confirm the split granularity.
3. **Lifecycle-status comparison kind.** P-022's "status == Resolved" and "NEW incident, not reopen" — does the
   ch1–ch5 `ComparisonKind` set (`Contains`/`Absent`/`CountAtLeast`) express these (e.g. `Contains "Resolved"`,
   `CountAtLeast 2` distinct incidents), or is a minimal new kind/field needed? **Bias: reuse `Contains`/
   `CountAtLeast` — zero model change** (the inferred-token precedent).
4. **P-023 acknowledge drivability.** Is incident "acknowledgment" drivable through the pinned four-tool MCP
   contract (does `mark_incident_resolved` serve as ack?), or is the ack step an operator-checklist action (making
   P-023's auto leg partial)? Determines whether the cool-down is an auto `[[expected]]` check or a
   declare-only/operator leg. **Calibration point for Epoch-8** regardless.
5. **CalibrationRegion leg encoding.** How a tier expectation is carried — a `class="CalibrationRegion"` block with
   the *expected* tier token (e.g. `Contains "Suggested"`) that the Epoch-8 evaluator treats as a
   tendency / report-for-human (NOT a hard match), per the P-008 precedent. Confirm the `expected` shape carries the
   calibration target without implying a hard assert.
6. **Long-window timing realism.** P-022 needs a 120 s cease; P-023 a 5-min cool-down; P-059 a ~10-min sustained
   incident. Deterministic under `start_paused` (fine for the fixture), but flag spec-faithful `gap_ms`
   (~120 000 / ~300 000 / ~600 000 ms) for the Epoch-8/10 real-runtime duration vs a compressed variant. Declare
   spec-faithful here; runtime compression is Epoch-8's.
7. **Zero-model-change confirmation.** Confirm the ch1 surface + the ch2 `CalibrationRegion` class cover all seven
   recipes with **no** further `Scenario`/`ExpectedCheck`/`Phase` change — or that the single minimal addition (a Q3
   lifecycle-status kind) is the only unavoidable one.

## Definition of done (chunk-level)

- The severity-lifecycle scenario TOML(s) for P-019..P-023, P-059, P-060 exist under `scenarios/`, deserialize +
  garde-validate via `Scenario::from_toml_str`, and produce a valid `PhaseTimeline` through the existing scheduler —
  proven by fixture round-trip tests (mirrors ch1–ch5).
- Expected checks carry the **correct mixed class**: `Hard` for lifecycle timing (P-022 120 s, P-023 cool-down) +
  tier-SLO routing (P-060); `CalibrationRegion` for severity choice (P-019/P-020) + interpretation continuity
  (P-059) — with valid/invalid garde `#[case]` rows wherever the model is touched, and a class-usage guard asserting
  BOTH classes are present (the first mixed-class family).
- The model-interpretive legs are declared `CalibrationRegion` routed to `ManualCheck` (verdict-first lamp); the
  deterministic spine is `Hard`; any declare-only legs (P-019 monotonicity, P-021 cue-through-model, P-023 ack if
  undrivable) are recorded in TOML comments + chunk notes for the Epoch-8 evaluator.
- **No new emit / fault primitive, no new inbound bind, no new dependency**; the one possible model touch (a Q3
  lifecycle-status kind) is taken only if genuinely unexpressible.
- Determinism preserved: same scenario+seed ⇒ same stream shape (under `start_paused`); seed-named goldens
  re-baselined **iff** any new TOML feeds them (grep the seeds — expected UNCHANGED).
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · `agent-run.sh run` exit 0.
