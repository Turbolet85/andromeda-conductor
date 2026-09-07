# Scope — Halo hue budget re-driven

**Marker:** `2026-09-06-halo-hue-budget-re-driven`
**Working entry (verbatim title):** _Halo hue budget re-driven — an error stream sustained through incident
formation so the severity tier flips while the service is still emitting (P-025)_
**Version:** conductor-0.2.0 · Epoch 6b — Polish & ship

---

## 1. What this chunk builds

A **re-driven `halo-hue-encoding` scenario** whose own emission keeps running *through* Pulse's incident
formation, so that when the severity tier flips, the emitting service's `last_seen` is still fresh — and
Pulse's `metric.constellation.hue_update_ms` therefore measures the hue update rather than the staleness of
a service that had already gone quiet.

The prior chunk (`2026-08-21-delegated-timing-budgets-proven`) measured the ≤2s bound at 35581 ms and
36705 ms and proved *why*: the observable was sound and the **pairing** was wrong. This chunk changes the
pairing — it drives a stimulus the observable can honestly measure — and grades whatever that yields.

Two folded obligations ride with it (§3): the scenario's **declared tier is unreachable** and must be
re-tiered in the same pass, and the previous chunk's **`ReadBack::AutoResolved` observable must be
re-proven** on a fresh Pulse data dir under a stated bootstrap posture.

### 1.1 Measured starting state (basis stated per line; Conductor HEAD `3d69573`, SUT HEAD `83d4060`)

- **`scenarios/halo-hue-encoding.toml` declares ZERO `[phases.emission]` blocks.** Two phases,
  `healthy-baseline` and `error-pressure`, each carrying `gap_ms = 3000` and nothing else. Basis:
  whole-file read at P1. This is the CONTEXT's second cause, verified exactly as stated.
- **Its declared tier is already violated by its own phase data.** `slo_tier = "<5s"` (the header calls it
  "the fast tier") against 3000 + 3000 = 6000 ms of `gap_ms`. Basis: whole-file read. CARRY 1's premise
  verified.
- **It declares one `[[checklist]]` row and no `[[expected]]` check** — declare-only, so nothing grades it
  today (`verdict: null`). Basis: whole-file read. `Scenario::check_checklist` forbids a `[[checklist]]`
  beside a non-empty `expected`, so adding a graded check and keeping the checklist row is a load-path
  conflict this chunk must resolve deliberately, not incidentally. `[inferred]`
- **The harvest test pins the DISPROOF, not a pass.** `crates/conductor-run/tests/delegated_timing_harvest.rs:337`
  is `p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism`, and `:36` states it exists
  "never as a pass". Basis: grep + the file's own module doc. Re-driving the scenario puts this test's
  subject in play.
- **P-025 is classified `CoverageMode::DriveObserve`** (`crates/conductor-core/src/coverage.rs:239`), not
  `Auto` — so it does not participate in the `UNBACKED_AUTO` backing ledger
  (`crates/conductor-core/src/drift.rs:61`), and this chunk changes no coverage classification. Basis:
  direct read of both files. `[inferred]`
- **The load envelope's asserted terms are `max_sustained_storm_ms = 600000` and
  `max_sustained_rate_spans_per_s = 10000`**, with `max_scenario_duration_ms = 600000` recorded but not
  asserted, and the `[[exempt]]` ledger **empty** and held at exact-set equality in both directions. Basis:
  `contracts/pulse-load-envelope.toml:21-47`. A sustained emitting window is exactly what this chunk adds,
  so both asserted terms are live constraints on the new shape — and the rate term now counts **wire
  records** via `EmissionSpec::max_spans_per_dispatch()` (shipped by `2026-09-06-coverage-completeness-gate`),
  not dispatches. `[inferred]`
- **The service-identity split does NOT settle attribution for this observable.**
  `[premise-corrected: the hue leaf's allowlist is exactly `["duration_ms", "severity_tier"]` and the
  SUT's own comment at `pulse-app/src/observability.rs:986-990` reads "Aggregate-only: no service
  identifier" — so `conductor` vs `conductor-canary` cannot discriminate a hue sample.]** The
  `CANARY_SERVICE_NAME` / `DEFAULT_SERVICE_NAME` split is real (architecture §Occupied Resources) and
  still governs Pulse's per-service sample counts and EWMAs; it simply reaches no field on THIS line.
  Attribution must ride what the line carries — the `severity_tier` VALUE and sample order — or be
  designed out by ensuring the scenario's dot is the only one changing tier. Worse: the fire site emits
  the **slowest** changed dot's staleness and that dot's tier, so a same-pass canary change would
  *report the canary*. See `research.md` §THE HEADLINE FINDING and Open question 1.

---

## 2. What it does NOT build

- **No change to Pulse.** The hue render, the `hue_update_ms` fire site and the incident pipeline are the
  SUT's. This chunk changes only what Conductor drives and how it grades.
- **No `[[expected]]` check bound to a Pulse-internal duration.** `budget_ms` / `effective_deadline_ms`
  grade `read_back_observed_at − journal_emitted_at` — Conductor's own MCP round-trip — so binding a
  Pulse-internal render/hue duration to it is a category error, recorded as such in `v2-20`'s
  PREMISE-CORRECTION and in the scenario header. The delegated bound stays graded at the **harvest tier**.
- **No claim on the VISUAL hue.** P-025's rendered hue remains operator-checklist (`DriveObserve`); only
  the delegated timing observable is in play.
- **No new automation stack, no new dependency, no widening of the trust boundary.**
- **No retroactive un-verification of `v2-20`.** P-025 was excluded from that cap by operator ruling on
  2026-08-21 and is UNVERIFIED, never un-verified; the three budgets it did claim stay `verified`.

---

## 3. Folded freight (annotations on the working entry + the phase directive)

Per `promotion.md`, annotations fold as HYPOTHESES and each named coordinate was re-verified against the
artifact itself before it shaped this scope. Mechanism claims keep their original marker text verbatim;
their TRUTH is P3's scope-premise-closure job, not this fold's.

### 3.1 CONTEXT (from `2026-08-21-delegated-timing-budgets-proven`) — coordinates verified

> the <=2s bound measured 35581ms and 36705ms on two legs and the cause is structural, not slowness —
> `metric.constellation.hue_update_ms` computes `now - item.last_seen_unix_nano` at the tier change, i.e.
> STALENESS, so a service that goes quiet before the flip reports Pulse's own L2->L3(20-60s)->L4 formation
> latency. The identical formula passed at 702ms for `metric.constellation.discovery_ms`, which fires while
> `last_seen` is fresh — so the metric is sound and the pairing was wrong.

`[inferred — measured-marked claim, carried verbatim; the fire-site arithmetic is re-verified at P3]`

- Second cause carried verbatim: `halo-hue-encoding.toml` declares ZERO `[phases.emission]`, so it drives
  no tier change of its own and BOTH samples were the preflight canary's (`severity_tier: autonomous`,
  within a second of its incident). **Verified at P1** against the scenario file.
- The entry's own instruction: "Attaining the bound needs a sustained emission window spanning formation,
  which interacts with the load envelope's `max_sustained_storm_ms` term — size it against
  `contracts/pulse-load-envelope.toml`, never by eyeballing." **Term verified present at
  `pulse-load-envelope.toml:26` (600000).**
- `v2-20` refined to the three proven budgets at that chunk (operator-ratified); P-025 returned to the pool.
  **Verified**: `v2-20` is `verified` / chunk `2026-08-21-delegated-timing-budgets-proven`, its `acceptance`
  states the P-025 exclusion explicitly, and its `notes` carry the PREMISE-CORRECTION.
- Evidence pointer `crates/conductor-run/tests/delegated_timing_harvest.rs::p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism`
  — **verified present at `:337`**.

### 3.2 CARRY (from `2026-08-22-operator-pause-and-checklist-live-firing`) — verified exactly

> the scenario's DECLARED TIER is unreachable and must be re-tiered in the same pass.

`slo_tier = "<5s"` against 6000 ms of `gap_ms`; `latency_ms` measured 6081 and 6080 on two attended legs.
Nothing grades it today (declare-only, `verdict: null`), so it is **latent rather than broken** — it becomes
a real failure the moment the scenario gains an `[[expected]]` check. **All file-side coordinates verified at
P1.** The re-tier must be sized against the re-driven shape, alongside the load-envelope sizing above.

### 3.3 CARRY (from `2026-09-06-operator-gated-live-suite`, 0-pending adaptation) — coordinates verified

Re-prove that chunk's **observable 1** — the `ReadBack::AutoResolved` arm, which needs an EMPTY active set
at read-back — on a FRESH Pulse data dir and INSIDE the canary service's bootstrap hour. Two distinct
measured causes, which tune differently:

- **(a) PAST the hour the arm is unreachable at any window length**: `service_went_silent` cues raise NEW
  autonomous incidents during the silent phase itself, so a longer silence yields more of them.
  `[inferred — measured-marked claim, carried verbatim]`
- **(b) INSIDE the hour the window must clear 150 s** (120 s idle + up to a full 30 s observer tick)
  measured from the **LAST** canary incident, not the first — a leg's own preflight can form more than one.
  `[inferred — measured-marked claim, carried verbatim]`

Coordinate re-verification:
- `scenarios/auto-resolve-idle-window.toml` — the stale MARGIN model is at **`:28-32`** (the CARRY wrote
  `:29-32`; off by one line, same block, no material difference). It states the 150 s worst case, the
  165000 → 200000 widening and "NOT yet live-proven at this value". **Verified.**
- The two cited runs (`2026-09-06T10-58-18-536`, `2026-09-06T10-07-08-396`) appear in **committed**
  evidence — `chunks/2026-09-06-operator-gated-live-suite/evidence/live-suite-verdict.md` + `report.md` —
  so the measurements are re-readable from the repo, not only from a transient leg dir. **Verified.**
- `pulse-legs/a11y-20260906-110201/logs/agent-latest.jsonl.2026-09-06` resolves under
  **`%TEMP%/pulse-legs/`** (the operator convention pinned at `.claude/rules/verification-harness.md:52`),
  and that directory **still exists on this host**. **Verified, but transient** — the committed evidence
  above is the durable basis.
- SUT constants: `BOOTSTRAP_WINDOW_SECONDS = 3_600` at
  `andromeda-pulse crates/triage/src/baseline/activity_floor.rs:36` and the evaluator at
  `crates/triage/src/cue/emitter.rs:186` (`evaluate_service_went_silent`) — **both verified byte-exact at
  SUT HEAD `83d4060`, which is this host's checked-out Pulse HEAD.**

### 3.4 PHASE DIRECTIVE — the cross-project fact, verified at SUT HEAD `83d4060`

The bootstrap window bounding CARRY 3.3's re-proof is **operator-tunable at boot**. Every citation was
re-verified against the SUT source (T1: a dictated cross-project citation costs one pass and is never
optional). All verified, plus one reachability check the directive did not assert:

| Directive claim | Verified |
|---|---|
| Env `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` at `crates/triage/src/cue/thresholds.rs:12` | ✓ `ENV_BASELINE_BOOTSTRAP_SECONDS` const at `:12` |
| `resolve_bootstrap_window_seconds()` at `:26-49` | ✓ function spans that range |
| `Thresholds::from_env()` at `:229-237` | ✓ sets `bootstrap_window_seconds` from the resolver |
| Accepted range: integer strictly between 0 and `WINDOW_DURATION_SECONDS = 86_400` | ✓ guard is literally `Ok(v) if v > 0 && v < WINDOW_DURATION_SECONDS`; const at `activity_floor.rs:28` |
| Rejects fall back to 3600 s **and warn** | ✓ `env_rejected_out_of_range` / `env_rejected_unparseable`; accepted non-default logs `env_override` — all three reason strings exact |
| Conductor already recorded the knob at `chunks/2026-08-18-error-baseline-spike-live-proof/research.md:18` | ✓ verbatim: "silence-family only, irrelevant here" |
| **(not asserted by the directive) the knob is REACHED at boot** | ✓ `pulse-app/src/main.rs:472` — `let thresholds = Arc::new(Thresholds::from_env());`, and the value flows `Thresholds::bootstrap_window_seconds` → `set_bootstrap_window_seconds` → `bootstrap_state(now_nanos, …)` (`crates/triage/src/baseline/mod.rs:736`, `:473`). The knob is live, not inert. |

**`pulse-a11y.sh` is operator-owned and is NOT present in either repo** (absent from
`andromeda-pulse/scripts/`, `conductor/scripts/`, and both git indexes). Its stated behaviour — deterministic
L4 + MCP + a fresh data dir, and **not** this variable — is taken from the directive, not verified from a
file. Any bootstrap value this chunk relies on therefore reaches the leg through its **firing form**, per the
LIVE-LEG rule, never by assuming the launcher sets it. `[inferred]`

**The design call this chunk must make and STATE:** whether the re-runnable proof may lean on the knob (a
test-mode posture) or must hold under the SUT's default hour. The plan must say which, and the chosen value —
if any — goes into the leg's firing form.

- **VERIFIED at P3 — the knob's two useful directions serve opposite legs.** The AutoResolved arm wants
  the silence detector to stay **disarmed** (a *stretched* window, so `service_went_silent` never fires
  during the leg's own silent phase); the hue re-drive rides the storm / error-rate families, which
  consult **no** baseline at all (the `BootstrapState` gate sits only inside `evaluate_service_went_silent`,
  `crates/triage/src/cue/emitter.rs:186`). So the knob is **decisive for the CARRY 3.3 leg and irrelevant
  to the hue leg**. It is also genuinely live rather than declared: `pulse-app/src/main.rs:472` calls
  `Thresholds::from_env()` at boot and the value reaches `bootstrap_state` via
  `crates/triage/src/baseline/mod.rs:736`, `:473`.

---

## 4. Surfaces and contracts touched

| Surface | Expected treatment |
|---|---|
| `scenarios/halo-hue-encoding.toml` | The chunk's centre: gains emission that sustains through formation; `slo_tier` re-tiered; `[[checklist]]` vs `[[expected]]` resolved deliberately |
| `contracts/pulse-load-envelope.toml` | **Read, and sized against** — both asserted terms. An `[[exempt]]` entry is a last resort, not a first move (the ledger is empty and set-equality-held) |
| `crates/conductor-run/tests/delegated_timing_harvest.rs` | The P-025 harvest assertion re-aimed to grade the re-driven shape at its real measured value |
| `scenarios/auto-resolve-idle-window.toml` | CARRY 3.3 — its header's stale margin model, and whatever the re-proof measures |
| The live-leg firing form (`scripts/agent-run.{sh,ps1}` `run --live` + the operator launch) | Carries any bootstrap posture the plan states |
| `conductor-core` scenario model | **NOT touched — verified unnecessary.** `EmissionSpec::occurrences` are "paced evenly across its gap" (`phase_spec.rs:100-102`), so a sustained sub-2s-interval window is expressible with the shipped model |

**Verification matrix:** no `v2-NN` capability currently covers P-025 (`v2-20` explicitly excludes it and is
already `verified`). Whether this chunk claims anything is a P5 decision; the default is **claims nothing**.
`[inferred]`

---

## 5. Definition of done (scope-level; the plan sharpens these)

1. `halo-hue-encoding` drives an error-shaped stream that reaches a severity tier change **of its own** and
   is still emitting when that change fires — attribution provable, not assumed.
2. The delegated `hue_update_ms` observable is graded at its **real measured value** on a live leg, and the
   MECHANISM that governs it is pinned. **Amended at the P5 review (2026-09-07) — intent-incomplete:** the
   ≤2 s bound is **unmeasurable through this leaf**, because `ServiceRegistryEntry.last_seen_unix_nano` has
   no ingest-path writer and is refreshed only by the 15 s lifecycle tick (`registry.rs:336` gated on
   `current_quiet_duration_seconds == 0`, integer-truncated at `activity_floor.rs:142-149`;
   `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL = 15 s` at `lifecycle/mod.rs:52`, wired unchanged at
   `main.rs:1489`). So `hue_update_ms ≈ t_sample − t_last_refreshing_tick`, U(0, 15 s) and independent of
   dispatch rate — a sub-2 s sample is a tick coincidence (~13 % of flips), never attainment. The
   deliverable is therefore the mechanism pin plus the measured value, with **no pass arm** against 2 000 ms
   until the SUT changes the writer. That SUT-side fix is a Pulse intake item, not this chunk's work (§2).
3. The scenario's `slo_tier` is re-tiered to the re-driven shape, and the new shape passes the load
   envelope's asserted terms without an exemption.
4. The `AutoResolved` observable is re-proven on a fresh data dir under a **stated** bootstrap posture, or
   the attempt's measurement is recorded with which of causes (a)/(b) it hit.
5. The plan states, explicitly, whether the proof leans on `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`.
