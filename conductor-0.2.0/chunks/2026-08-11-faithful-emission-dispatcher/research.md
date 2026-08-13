# Codebase Research — 2026-08-11-faithful-emission-dispatcher

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs/Greps:** 8 · **Graph queries:** 4 (trace: `.andromeda/runs/2026-08-11T21-24-43-phase/tree-query-2026-08-11-faithful-emission-dispatcher.json`)

## Files inspected
- `crates/conductor-run/src/lib.rs` (278-300, 336-372) — `execute_scenario`'s emit segment and the whole of `coarse_emit`; the call ordering below is the chunk's central finding.
- `crates/conductor-core/src/phase_spec.rs` (35-115) — `EmissionSpec` (one field: `signal`), `Signal` (3 variants), `PhaseSpec` with `emission` under `#[garde(skip)]`, and the existing garde test trio.
- `crates/conductor-emit/src/lib.rs` (23-32) — the ten public primitive families the dispatcher consumes.
- `crates/conductor-emit/src/client.rs` (55-115) — `TraceEmitter::export` / `LogsEmitter::export` and their spans.
- `crates/conductor-timeline/src/scheduler.rs` (20-60) — `run_timeline` + the `timeline.execute` span.
- `crates/conductor-timeline/src/convert.rs` (full) — `PhaseTimeline::from(&Scenario)`; deliberately drops the emission descriptor.
- `crates/conductor-timeline/src/phase.rs` (35-55) — `PhaseTimeline` / `Phase` / `PhaseTransition` shapes.
- `crates/conductor-core/src/redact.rs` (30-60) — the field-name allowlist.
- `crates/conductor-core/src/obs.rs` (impl scan) — `JsonObsLayer` implements `on_event` / `on_new_span` / `on_close`; **no `on_record`** (confirms the CARRY).
- `crates/conductor-core/src/load_envelope.rs` (16-25, 268-290, 330-345) — the "declared, not derivable" rationale in prose and on the field.
- `crates/conductor-run/Cargo.toml`, `crates/conductor-timeline/Cargo.toml` — dependency edges.
- `scenarios/fingerprint-storm.toml` (1-40), `scenarios/restart-suppression.toml` (phase names) — the authored shape convention.
- `.andromeda/obs-plan.md` (285-315) — §4 CP1's must-trace chain + required span attributes.

## Graph impact
- **`coarse_emit`** — **1** caller: `execute_scenario` @ `crates/conductor-run/src/lib.rs:292`. Private to the crate; replacing it has zero cross-crate blast radius.
- **`execute_scenario`** — **3 production** callers + 4 test call sites: `conductor-cli::commands::run` @ `run.rs:23`, `conductor-cli::commands::suite` @ `suite.rs:30`, `conductor-run::drive_run` @ `lib.rs:498`. This is why its public signature must survive unchanged — three call sites across two crates and both bins.
- **`EmissionSpec`** — 21 references. Production consumers are few: `conductor-core::phase_spec` (definition), `scenario.rs:169/182/302/335` (model + validation), `conductor-run/src/lib.rs:346` (the `.signal` read inside `coarse_emit`), `conductor-timeline/src/convert.rs:31/45/50` (**test fixtures only** — the production `From` impl ignores it). `load_envelope.rs:280/337` are **test fixtures only**. So extending `EmissionSpec` has a genuinely small production blast radius; the volume is fixtures.
- **`crate_edges`** — `conductor-timeline → conductor-core` and `conductor-run → conductor-timeline`. **`conductor-timeline` has NO edge to `conductor-emit`**, and per crate-per-seam it must not gain one.

## Patterns detected
- **Emission happens AFTER the timeline completes, not during it** (`lib.rs:293-294`): `run_timeline(...).await` sleeps through *every* phase gap and returns the full `Vec<PhaseTransition>`; only then does `coarse_emit` iterate that slice and emit back-to-back with no delay. For `fingerprint-storm` this means ~24s of silence followed by 2 spans emitted instantly — the phase timing never reaches the wire. Pulse's detectors are windowed (L2 RetryStorm ≥5 same-fingerprint in a 30s rolling window; the ≥20s silence gap of P-015), so **emission timing is semantically load-bearing and is currently discarded**.
- **The `timeline.execute` span closes before any emission exists** (`scheduler.rs:34` + `lib.rs:293-294`): `run_timeline` owns the span and returns before `coarse_emit` is called, so `emit.batch` is a **sibling** of `timeline.execute` under `scenario.run`, not a child. `run_timeline`'s own doc says "Pure timing: nothing is emitted."
- **Count-before-open is the established span-attribute idiom** (`client.rs:66`, `:104`): `#[tracing::instrument(name="emit.batch", fields(emission_count = count_spans(&request)))]` computes the value from the argument in the attribute position — no `field::Empty`, no post-open `record()`. `timeline.execute` uses the same idiom for `phase_count = timeline.phases.len()` (`scheduler.rs:34`).
- **`PhaseTimeline` deliberately carries timing only** (`convert.rs:7-9`): "The per-phase emission descriptor is not consumed here (the timeline carries timing only); it rides in the config model for the Epoch-3 emission seam." So `run_timeline` cannot derive an emission count from its own argument today.
- **Shape is authored in phase NAMES** (`fingerprint-storm.toml`, `restart-suppression.toml`): `storm-same-fp-6x`, `storm-same-fp-12x`, `suppressed-15s-burst`, `bypass-relative-12x-4pct`. `fingerprint-storm.toml` states the convention explicitly — "EmissionSpec carries no occurrence-count field, so the Epoch-8 driver realizes the per-phase count + variant mix from the phase names (the restart-suppression precedent)."

## Conventions to follow
- **Span attributes are computed in the `fields(...)` position from arguments** (`client.rs:66`) — never `field::Empty` + a later `record()`, which `JsonObsLayer` would silently drop (`obs.rs:282-320`, no `on_record`).
- **The redact allowlist already admits every count this chunk needs** (`redact.rs`): `phase_count`, `emission_count`, `record_count`, `row_count`, `count`, `phase` are all present. A *new* attribute name (e.g. `batch_index`, `p_id_count`) would need adding there or it is dropped silently.
- **garde lives beside the serde struct with bounded consts** (`phase_spec.rs`): `MAX_GAP_MS`-style const + `#[garde(range(max = …))]`. Note `PhaseSpec.emission` is currently `#[garde(skip)]` — new nested rules will not execute until that becomes a dive.
- **Test fixtures construct `EmissionSpec::default()` in 4 files** (`convert.rs`, `load_envelope.rs`, `scenario.rs`, `phase_spec.rs`) — an added field with a `Default` impl keeps every existing fixture compiling.
- **`conductor-run` has no RNG dependency today** (`Cargo.toml`: core/timeline/emit/verify/report + anyhow/tracing/serde/tokio-time). Any seeded shaping in the dispatcher either derives from `conductor-timeline`'s existing `ChaCha8Rng` or adds `rand_chacha` to this crate — which is a dependency-surface change the standing `cargo audit` deferral's basis must then account for.

## Files to modify
- `crates/conductor-run/src/lib.rs` — replace `coarse_emit` (`:340`); `execute_scenario`'s public signature unchanged; its unit-test module at `:513` gains the dispatcher cases.
- `crates/conductor-core/src/phase_spec.rs` — `EmissionSpec` fields + garde rules; `PhaseSpec.emission` `#[garde(skip)]` → dive (fork-dependent).
- `crates/conductor-timeline/src/convert.rs` + `src/phase.rs` + `src/scheduler.rs` — only if the emission shape must reach the timeline (the `emission_count` resolution / per-phase-boundary dispatch).
- `scenarios/*.toml` — fork-dependent; 35 files, of which 2 currently declare emission at all.
- `crates/conductor-core/src/load_envelope.rs` — the `declared-not-derivable` prose (`:16-18`) and the field docs (`:48-52`) go stale the moment a rate/occurrence field exists; that is a deliberate re-statement, not a silent widen.
- **Caller threading:** `execute_scenario`'s 3 production callers (`conductor-cli/src/commands/run.rs:23`, `conductor-cli/src/commands/suite.rs:30`, `conductor-run/src/lib.rs:498`) need NO change while its signature holds — enumerated from the graph so /implement does not have to guess.

## New files to create
- `crates/conductor-run/src/dispatch.rs` (or equivalent) — the per-phase dispatcher, if the plan prefers a module over an in-`lib.rs` function. Decided at P4.

## Open questions
- **Where does the declared emission shape live — extended `EmissionSpec` fields, the authored phase-name convention, or a hybrid?** The cap's acceptance presumes declared fields that do not exist; the catalog presumes names. → blocks: **plan-decision**
- **Does the dispatcher emit DURING the timeline walk (making `emit.batch` a real child of `timeline.execute` and putting phase timing on the wire), or after it as today?** This decides whether `emission_count` can sit on `timeline.execute` at all, and whether windowed Pulse detectors can ever fire. → blocks: **plan-decision**
- **If emission stays after the walk, does `emission_count` move to `scenario.run` with a recorded premise correction on obs-plan §4 CP1?** Phase is read-only on specs, so the correction is a wrap-reconcile outcome, not a phase edit. → blocks: **plan-decision**
