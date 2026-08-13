# Codebase Research — 2026-08-13-dispatcher-determinism-goldens

## Scope
- **Depth:** deep · **Reads:** 8 · **Globs/Greps:** 3 · **Graph queries:** 8 (all `db_state: fresh`)

## Files inspected
- `crates/conductor-timeline/src/scheduler.rs` (full) — `run_timeline_with` is the emitting form; `run_timeline` delegates to it with a no-op hook. One `ChaCha8Rng::seed_from_u64(seed)` draw **per phase** (`jittered_gap`), and `paced_slices` splits the jittered gap into one sleep per declared emission with the last slice absorbing the remainder. `EmissionPoint { phase_index, occurrence }` is all the timeline knows — it never sees a shape.
- `crates/conductor-timeline/src/phase.rs` (full) — `Phase { name, gap, emissions }`, `PhaseTimeline { phases, jitter }`, `total_emissions()`. `PhaseTransition { index, name, elapsed_ms }` — `elapsed_ms` is virtual-clock cumulative, no `std::time`.
- `crates/conductor-run/src/dispatch.rs` (full) — the dispatcher. `Dispatcher::dispatch(point)` looks up `scenario.phases[point.phase_index].emission`, derives `emission_seed(seed, phase_index, occurrence)` (a **pure** mix, no RNG state), and matches all 9 `EmissionShape` variants onto `conductor-emit` primitives. Requires a live `TraceEmitter::connect(endpoint)`.
- `crates/conductor-run/tests/dispatch_wire.rs` (full) — the existing wire harness: loopback tonic `TraceService`/`LogsService` stubs on ephemeral `127.0.0.1:0`, `drive_scenario!` macro driving the real `run_timeline_with` + `Dispatcher` under `start_paused`. 11 tests. **`the_same_seed_reproduces_the_same_stream` (line 345) already asserts same-seed-identical / different-seed-divergent on a `(trace_id, span_id)` projection.**
- `crates/conductor-timeline/tests/replay.rs` (1-45) — the existing insta goldens freeze `Vec<PhaseTransition>` only, at seeds `424242` + `7`, through `Scenario::from_toml_str → PhaseTimeline::from → run_timeline`.
- `crates/conductor-timeline/tests/pacing.rs` (1-40) — `stamped()` drives `run_timeline_with` and returns `Vec<(EmissionPoint, u128)>` stamped from `tokio::time::Instant` under the paused clock. This is the natural timeline-tier capture point.
- `crates/conductor-core/src/load_envelope.rs` (1-40, 100-280) — module doc already records the rate terms as *derivable but not yet asserted* and names the re-scope as separate work. `scenario_duration_ms` (line 114) = `fold` over **all** `p.gap_ms`.
- `contracts/pulse-load-envelope.toml` (full) + `scenarios/activity-floor.toml` (full) — the CARRY's subjects.

## Graph impact
- **`scenario_duration_ms`** — 6 call sites (`rows: 6`, after narrowing the descriptor to `load_envelope/scenario_duration_ms`; the broad `LIKE '%scenario_duration_ms%'` returned 16 because it also matches the field `EnvelopeTerms#max_scenario_duration_ms`). **Two production consumers**, not one: `LoadEnvelope::classify()` @ `load_envelope.rs:155` and `check_load_envelope()` @ `load_envelope.rs:227`. Rest: the `lib.rs:44` re-export + 3 in `duration_is_the_sum_of_phase_gaps()`.
- **`check_load_envelope`** — 7 callers (`rows: 7`): the `lib.rs:43` re-export + **6 test callers, all in-file**. Its only live invocation is `the_committed_catalog_matches_the_committed_envelope()` @ `load_envelope.rs:365` — the gate IS a committed test, mirroring `check_sut_drift`.
- **`LoadEnvelope::classify`** — 7 callers (`rows: 7`): 6 in-file tests + **one production site, `conductor-run::classify_run()` @ `crates/conductor-run/src/lib.rs:412`** — the path that drives the `[ENVIRONMENT-SUSPECT]` caption.
- **`run_timeline_with`** — 8 sites (`rows: 8`): `conductor-run::execute_scenario()` @ `lib.rs:296`, `conductor-run/tests/dispatch_wire.rs:16`, `scheduler.rs:58` (the `run_timeline` delegation), `pacing.rs:26` + `:107`, plus 3 `crate/` re-exports.
- **`EmissionPoint`** — 19 refs (`rows: 19`). Fields used: `phase_index`, `occurrence`. Consumed in `conductor-run/src/dispatch.rs` (20, 64, 65, 68, 85, 99, 106) and `conductor-timeline/tests/pacing.rs`.
- **crate edges** — 2 rows: `conductor-run → conductor-timeline`, `conductor-timeline → conductor-core`. `conductor-timeline` has exactly one inbound consumer; a test-only addition has zero cross-crate blast radius.

## Patterns detected
- **Shape-projection over byte-golden** (`dispatch_wire.rs:357`): where emitted spans carry `start/end_time_unix_nano` from `SystemTime::now()`, determinism is asserted on a projection (`(trace_id, span_id)`), never the whole payload. Matches the 2026-06-17 testing.md rule and obs's shape-projection pattern.
- **Two-seed golden + proptest generalization** (`replay.rs:31-45`): a golden at a fixed seed plus a contrasting seed pins seed-*sensitivity*, not just one shape.
- **Loopback stub, never `:4317`** (`dispatch_wire.rs:64-78`): ephemeral `127.0.0.1:0` tonic server; `:4317` is reserved for the port-occupier fault.
- **Insta names snapshots by test-file stem** — a golden added to `pacing.rs` lands as `pacing__*.snap`; `replay.rs` keeps `replay__*`.
- **Gate-as-committed-test** (`load_envelope.rs:365`, `check_sut_drift` precedent): the static catalog gate is a `#[test]` over the committed artifacts, not a runtime call.

## Conventions to follow
- **Virtual clock only** in timeline tests: `#[tokio::test(flavor = "current_thread", start_paused = true)]` (`replay.rs:31`, `pacing.rs:37`, every `dispatch_wire.rs` test).
- **Emission identity is seed-pure**: `emission_seed` (`dispatch.rs:195`) mixes seed × phase_index × occurrence with no RNG state, so the dispatch stream is a function of the seed alone.
- **`EnvelopeStatus` is a value on `Ok`** (`load_envelope.rs:152`) — a breach is an outcome, never `Err`.
- **Envelope messages carry identity only** — scenario names + release/date, never a path or type name (`envelope_message`, `load_envelope.rs:245`).

## New files to create
- `crates/conductor-timeline/tests/` or `crates/conductor-run/tests/` — one new golden test module (tier decided at P4) plus its committed `.snap` files.

## Files to modify
- `crates/conductor-core/src/load_envelope.rs` — the re-scoped term + its in-file unit tests + the module doc's "not yet asserted" paragraph.
- `contracts/pulse-load-envelope.toml` — the `[envelope]` comment block, the `:40-42` retire sentence, and the `[[exempt]]` ledger.
- `.andromeda/architecture.md` §Occupied Resources — the "derivable but deliberately not yet asserted" wording (the (c) reconcile).
- **Caller threading (from the graph, not memory):** re-scoping the duration basis touches **both** consumers or neither — `LoadEnvelope::classify` @ `load_envelope.rs:155` (→ `conductor-run::classify_run` @ `lib.rs:412` → the `[ENVIRONMENT-SUSPECT]` caption) and `check_load_envelope` @ `load_envelope.rs:227`. The CARRY names only the second.
- `crates/conductor-run/src/lib.rs` — only if `classify_run`'s test fixtures assume the old basis (`tests/test_envelope()` @ `lib.rs:523`).

## Open questions

**1. The CARRY's central premise is FALSIFIED — verified against the catalog, not predicted.** → blocks: **plan-decision**

`contracts/pulse-load-envelope.toml:40-42` and the CARRY both assert that under an emitting-phase-duration term "each of these passes on its own merits and needs no exemption". Computed over all 35 committed scenarios (ceiling 600 000 ms):

| scenario | total duration | **emitting-phase duration** | peak per-phase rate | over on total | **over on emitting** |
|---|---|---|---|---|---|
| `activity-floor` | 3 900 000 ms | **900 000 ms** | 0.20/s | YES | **YES** |
| `incident-auto-resolution` | 731 000 ms | **610 000 ms** | 1.60/s | YES | **YES** |

Every other scenario is inside on both bases. So the summed-emitting-phase re-scope is a **no-op on the gate's verdict** — the same two scenarios breach before and after — and retiring both `[[exempt]]` entries as (b) instructs turns the gate **red** with `unpinned: activity-floor, incident-auto-resolution`.

The reason the prediction failed is visible in `activity-floor` itself: its three `train-active-*` phases are 300 000 ms **each** and genuinely emit, so summing disjoint bursts yields 900 s of "storm" — but they are separated by 10-minute quiets, which is precisely what its exemption reason said made it idle. **Summing disjoint bursts is not "sustained".** The envelope's own prose bounds *sustained high-rate storm*, and `max_sustained_storm_ms = 600 000` reads naturally as a bound on **one continuous emitting window**, not a scenario-wide sum. On that basis: `activity-floor`'s longest emitting phase is 300 000 ms and `incident-auto-resolution`'s is 600 000 ms — **both inside**, and both exemptions genuinely retire.

Peak per-phase rate across the entire catalog is **4.00 spans/s** against a 10 000 spans/s term (2 500× headroom), so the rate term is assertable-and-true today but catches nothing on its own.

**2. Does `LoadEnvelope::classify` re-scope with the gate?** → blocks: **plan-decision**
Both consume `scenario_duration_ms`. If only `check_load_envelope` re-scopes, a scenario can pass the static gate while `classify_run` still captions it `[ENVIRONMENT-SUSPECT]` on total duration — gate and per-run judgment measuring different things. The CARRY names only the gate.

**3. Which tier does the emission-stream golden freeze?** → blocks: **plan-decision**
Timeline tier (`Vec<(EmissionPoint, virtual_ms)>` — pacing/ordering, no network) vs dispatch tier (a shape-projection of the captured OTLP over the existing loopback stub — what the dispatcher actually emits) vs both. Note `dispatch_wire.rs:345` already covers same-seed/different-seed **within a run**; what is missing is the committed on-disk golden (cross-commit drift tripwire) and a seed-space proptest.
