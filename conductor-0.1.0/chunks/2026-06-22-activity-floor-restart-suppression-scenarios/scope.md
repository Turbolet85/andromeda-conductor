# Scope — Activity-floor + restart-suppression scenarios (P-013..P-016, P-057)

**Marker:** `2026-06-22-activity-floor-restart-suppression-scenarios`
**Version:** conductor-0.1.0 · **Epoch 7 (Scenario catalog) — chunk 4 of 8**
**Working-route intent:** _Activity-floor + restart-suppression scenarios — train/lunch/silence, restart gap, suppression/bypass triple (P-013..P-016, P-057)_

## What it builds

The **activity-floor + restart-suppression families of the `scenarios/` catalog** — declarative TOML scenario
config keyed to Pulse's activity-floor (P-013/P-014, `pulse-capability-spec.md` §3) and restart-suppression
(P-015/P-016 §4 + P-057) capabilities. This is the **first Epoch-7 chunk whose stimulus is fault helpers**
(conductor-faults, Epoch 4) rather than pure emit primitives: the bursty-train duty cycle (P-013), abrupt
permanent silence (P-014), and exact-length gap→resume (P-015) levers all already exist — this chunk authors
the scenarios that **compose** them and declare their expected outcomes.

Two scenario families (2 TOMLs, per the `input.md` coverage grouping + the ch3 per-family precedent):

- **`activity-floor.toml` (P-013, P-014)** — the activity-floor's false-positive guard + its death lever.
  - **P-013** (learning): a bursty-train baseline (active 5min / quiet 10min, repeating) trains the per-service
    histogram; then a 30-min developer-on-lunch quiet period must raise **no ServiceWentSilent** candidate (the
    learned-cadence tolerance). **Expected = ABSENCE.**
  - **P-014** (silence death): a *fresh* service emits 5min of consistent activity then stops abruptly → a
    **ServiceWentSilent attention cue surfaces at ~30s** (the 30s minimum, no learned quiet pattern yet).
    **Expected = PRESENCE.**
  - Both **`class="Hard"`** (activity-floor timing is deterministic per the Probabilistic-Assertion Policy).

- **`restart-suppression.toml` (P-015, P-016, P-057)** — restart detection → surgical suppression → the
  dual-condition bypass triple, one restart-anchored timeline.
  - **P-015** (restart): consistent traffic, then a **25s** gap + resume → a **RestartEvent within ≤2s** of resume.
  - **P-016** (surgical suppression): within the 60s post-restart window, a **15s error burst is SUPPRESSED**
    (<30s-persistence ErrorRateSpike) but a **60s sustained stream SURFACES** (exceeds the 30s persistence
    threshold). **Absence + presence.**
  - **P-057** (dual-condition bypass): the truth table `bypass = (relative ≥ 10×) OR (absolute ≥ 5%)` —
    **(a) 8×/3% → SUPPRESSED** (neither), **(b) 12×/4% → relative-bypass SURFACES**, **(c) 6×/7% →
    absolute-bypass SURFACES** (one cell of the OR each). **Absence + 2× presence.**
  - All **`class="Hard"`** (the Probabilistic-Assertion Policy names "suppression/bypass logic" explicitly as
    hard pass/fail).

This is **catalog wiring over levers that already exist**: conductor-faults `bursty-train-pattern` (P-013),
`abrupt-silence-fault` (P-014), `emission-gap-resume` (P-015), plus conductor-emit error-spans / error-rate
(the burst / sustained / bypass-magnitude stimuli). **No new fault primitive, no new emit primitive.** The
`Scenario`/`ExpectedCheck`/`Phase` TOML surface was wired in ch1 and consumed unchanged through ch3 — target:
zero further model change. **The one place that target is genuinely at risk** is the suppression *absence*
assertion (Open Q1): multiple core checks here ("suppressed" / "no false silent") assert an incident is **NOT**
surfaced, which the presence-only ch1 `ComparisonKind` set (`Contains`/`CountAtLeast`) cannot express. **No
live MCP run** happens here (Epoch-8 driver + Epoch-10 E2E).

## Requirement source of truth

- **`.andromeda/refs/pulse-capability-spec.md`** — THE normative source; each P-ID's "Conductor verification"
  clause is the concrete recipe:
  - **P-013** (L206-214): train bursty 5/10 → 30-min lunch → verify **no** ServiceWentSilent; cold-start = first
    60min/service universally suppressed while the histogram bootstraps.
  - **P-014** (L216-224): 5min active → abrupt stop → cue after **30s**; default severity hint Suggested, model
    retains discretion (P-020).
  - **P-015** (L232-240): consistent traffic → stop, wait **25s**, resume → RestartEvent within **2s**; bootstrap
    ≠ restart.
  - **P-016** (L242-250): restart → (a) 15s burst **suppressed** + (b) 60s sustained **surfaces** after 30s
    persistence; suppression is surgical (<30s ErrorRateSpike only; NOT hard signals / P-012 / P-018).
  - **P-057** (L706-714): override when **relative ≥10×** OR **absolute ≥5%**; triple (a) 8×/3% suppressed ·
    (b) 12×/4% relative-bypass · (c) 6×/7% absolute-bypass; surfaces with explicit Report annotation.
- **`.andromeda/architecture.md` §Probabilistic-Assertion Policy** — "suppression/bypass logic" + lifecycle
  timing are **Hard** pass/fail (not calibration-region); the model-interpretive severity that *consumes* these
  cues is P-020/P-021 (a later severity-lifecycle chunk), not asserted here. **§Timing-Tolerance Model** — the
  SLO tiers.
- **conductor-faults Epoch-4 levers** — `bursty-train-pattern` / `abrupt-silence-fault` / `emission-gap-resume`
  (the stimulus primitives; referenced, not modified).
- **Existing precedent** — ch1–ch3 `scenarios/*.toml` (the catalog shape `name · p_ids · seed · slo_tier ·
  jitter_ms · [[phases]]{name,gap_ms}` + `[[expected]]{kind,class,expected}`); `receiver-failed-port-conflict.toml`
  / `receiver-lifecycle-state.toml` (the prior fault-referencing scenarios — the model for how/whether a
  scenario names a fault).

## Boundaries

- **In:** the two scenario TOMLs (`activity-floor.toml` P-013/P-014; `restart-suppression.toml` P-015/P-016/P-057)
  under `scenarios/`; their `[[phases]]` (train/lunch / 5min-active-then-stop; gap→resume / suppression-window /
  bypass-triple) + `[[expected]]` checks (all `class="Hard"`); per-scenario SLO tier; coverage-mode header
  comments; fixture round-trip tests proving each deserializes + garde-validates + builds a valid `PhaseTimeline`.
- **Out:**
  - the live MCP read-back *run* (Epoch-8 driver / Epoch-10 E2E);
  - the suppression/bypass/floor **detection logic as evaluated runtime** — whether a burst was actually
    suppressed/surfaced is the Epoch-8 evaluator's comparison; the TOML only **declares** intent;
  - **P-013's "restart Pulse mid-training, verify histogram restoration" leg** — Conductor does **not** manage the
    Pulse process (explicit non-goal); the in-scope P-013 leg is the lunch-no-false-positive (histogram
    persistence-across-restart is a Pulse-internal property, observed only via an operator/Epoch-10 path, not
    driven here);
  - any **new fault or emit primitive** (all levers exist from Epoch 3/4);
  - the attention-cue → model-surfaced-incident interpretive layer (P-020/P-021 — severity-lifecycle, later);
  - the other Epoch-7 families; the CLI/desktop surfaces (Epoch 8/9).
- **Scope law:** every scenario carries its P-IDs; **no new inbound listener** — these are pure-egress
  (silence/gap/train are *stops/starts of egress*, NOT a port bind; the port-occupier P-003 is ch1, not here).

## Surfaces / contracts touched

- **Artifact:** `scenarios/activity-floor.toml` (NEW) + `scenarios/restart-suppression.toml` (NEW) — the primary
  deliverable.
- `conductor-core::{Scenario, ExpectedCheck, Phase}` (`scenario.rs`) — *consumed* via `from_toml_str`; **extended
  only if** the suppression *absence* check has no expressible `ComparisonKind` (Open Q1) — the one candidate
  model touch.
- `conductor-core::{ClaimClass, ComparisonKind, SloTier}` — class (all `Hard`); comparison kinds
  (`Contains`/`CountAtLeast` for the presence legs; the **absence** kind question for the suppression legs);
  per-scenario tier.
- **Fault/emit levers REFERENCED, not modified:** conductor-faults `bursty-train-pattern` /
  `abrupt-silence-fault` / `emission-gap-resume`; conductor-emit error-spans / error-rate (burst / sustained /
  bypass-magnitude stimuli).
- **Determinism goldens:** if either TOML feeds the **seed-named** replay goldens
  (`crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<N>.snap`), re-baseline — grep the new **seed**
  values (not just the scenario name), per the ch3 deviation.
- Requirement contract: `pulse-capability-spec.md` per-P-ID "Conductor verification" clauses.

## Open questions — resolve in planning (P4)

1. **Suppression "absence" comparison kind (THE headline decision).** P-013 (no false ServiceWentSilent),
   P-016(a) (15s burst suppressed), P-057(a) (8×/3% suppressed) each assert an incident is **NOT** surfaced — the
   defining outcome of a suppression capability. The ch1 `ComparisonKind` set is presence-only
   (`Contains`/`CountAtLeast`). Options: **(A)** add a minimal negative kind (`NotContains` / `CountAtMost` / an
   `expect: absent` modifier) so the suppression leg is a real `Hard` check in the TOML; **(B)** declare only the
   **positive** legs (P-014 cue, P-016(b) surfaces, P-057(b)/(c) bypass) and defer the absence legs to the
   Epoch-8 evaluator (the ch3 within-tolerance declare-only precedent). Unlike ch3 (a tolerance *band* on a
   present value), here the deferred bit would be **the core capability** — so the bias leans (A) a minimal
   `NotContains`, at the cost of the "zero model change" streak. **AskUserQuestion in P4.**
2. **File / timeline granularity.** 2 files (per the `input.md` family grouping + ch3 precedent) is the
   recommended shape — but each family carries **opposing outcomes**: activity-floor's P-013 (no alarm, trained
   service) vs P-014 (alarm, *fresh* service) plausibly need **different service identities / timelines**;
   restart-suppression's P-016 + P-057 are five sub-conditions (suppress / surface / 3× bypass). Does each family
   fit ONE seed/timeline (sequential phases, multiple `[[expected]]`), or does an opposing/independent-baseline
   outcome force a sub-scenario split (e.g. `activity-floor-train` + `activity-floor-silence`;
   `restart-suppression` + `bypass-triple`)? Recommend **2 files, multi-phase**; confirm.
3. **Fault-reference in the model.** Does `Scenario`/`Phase` express **which** fault/stimulus a phase runs
   (silence/gap/train/burst), or is it realized by phase-**name** semantics (`gap_ms` + a
   `"silence"`/`"resume"`/`"burst"` name) interpreted by the Epoch-8 driver? (Resolve against
   `receiver-failed-port-conflict.toml` / `receiver-lifecycle-state.toml` in P3 — the prior fault-referencing
   scenarios.) Sets whether the TOML *names* faults or only *times* phases.
4. **Bypass-magnitude + count stimulus.** P-057's conditions are (relative-magnitude × absolute-rate) pairs —
   8×/3%, 12×/4%, 6×/7% against 10×/5% thresholds. Confirm the emit/error-rate lever expresses **both** a relative
   multiplier over baseline AND an absolute error %, and how a phase encodes the pair (phase metadata vs
   declared-only in `[[expected]]`/comments).
5. **SLO tiers + the absence-SLO.** P-015 RestartEvent ≤2s → `<5s`; P-016/P-057 surfacing (after 30s persistence)
   → `<5s`/`<20s`; P-014 cue at ~30s → `<90s` (exceeds `<20s`). **The absence legs have no positive event to
   time** — how does an SLO tier apply to an expected *non-event* over a long window (30-min lunch, 60s
   suppression)? Confirm per the tier-scaling model (likely the tier governs only the positive/surfacing legs).
6. **Long-timeline realism.** P-013 needs convergence past a **60-min/service cold-start** + bursty training + a
   30-min lunch; `gap_ms` would reach ~1.8M ms (30min). Deterministic under `start_paused` (fine for the fixture
   test), but flag the **real-runtime** duration for Epoch-8/10 (a spec-faithful ~90-min P-013 run vs a compressed
   variant). Declare spec-faithful durations here; runtime compression is Epoch-8's.
7. **Zero-model-change confirmation.** Confirm the ch1 surface covers train/lunch/silence/gap/resume/
   suppression-window/bypass-triple with **no** further `Scenario`/`ExpectedCheck`/`Phase` change — or that the
   single minimal addition (the Q1 absence kind) is the only unavoidable one.

## Definition of done (chunk-level)

- `scenarios/activity-floor.toml` (P-013/P-014) + `scenarios/restart-suppression.toml` (P-015/P-016/P-057) exist
  under `scenarios/`, deserialize + garde-validate via `Scenario::from_toml_str`, and produce a valid
  `PhaseTimeline` through the existing scheduler — proven by fixture round-trip tests (mirrors ch1–ch3).
- All checks `class = "Hard"` (per the Probabilistic-Assertion Policy: "suppression/bypass logic" +
  activity-floor/lifecycle timing → hard), with valid/invalid garde `#[case]` rows wherever the model is touched.
- The suppression *absence* legs are expressed per the P4 Q1 decision (a minimal negative `ComparisonKind`, or
  declare-only-positive with the absence legs documented as Epoch-8-owned) — and the choice is recorded in the
  chunk notes for the Epoch-8 evaluator.
- **No new fault primitive, no new emit primitive, no new inbound bind** (the port-occupier is P-003/ch1).
- Determinism preserved: same scenario+seed ⇒ same stream shape (driven under `start_paused`); the **seed-named**
  goldens re-baselined **iff** either TOML feeds them (grep the seeds).
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · no new dependency.
