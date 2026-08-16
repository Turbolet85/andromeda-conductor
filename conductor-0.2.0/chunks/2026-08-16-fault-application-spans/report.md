# Report — 2026-08-16-fault-application-spans

**Chunk:** Fault-application spans — silence, ramp and port-occupier phases observable beneath the timeline span, with duration and journal offset (conductor-faults/run/timeline)
**Date:** 2026-08-16
**Commits:** none yet — this wrap creates the chunk commit (HEAD at entry: `b7bb6f8`, the previous chunk)

## Changes (structured — detectors read this)

- **Files:** 15 changed, +436/−17. Source/config (8): `crates/conductor-core/src/redact.rs` (+6) · `crates/conductor-core/src/obs.rs` (+66, tests only) · `crates/conductor-timeline/src/scheduler.rs` (+51/−?) · `crates/conductor-timeline/src/lib.rs` (+5/−1) · `crates/conductor-timeline/tests/pacing.rs` (+89) · `crates/conductor-run/src/lib.rs` (+134) · `crates/conductor-faults/src/port_occupier.rs` (+21) · `crates/conductor-faults/tests/port_occupier.rs` (+41) · `crates/conductor-faults/Cargo.toml` (+5) · `Cargo.lock` (+2). Chunk folder (3 new): `scope.md` · `research.md` · `plan.md`. Bookkeeping: master-route, working-route, verification-matrix, friction-log, handoff.

- **Symbols / APIs:**
  - NEW public in `conductor-timeline`: `PhaseWindow<'a> { index, name, gap }` (struct) and `run_timeline_observed<'t, F, E, O, G>(timeline, seed, on_phase, on_emit)` (async fn) — both re-exported from `lib.rs`. The observer is called once as each phase opens and its returned value is held for that phase's window, then dropped at the boundary; the value is opaque to the scheduler, which names no `tracing` type.
  - CHANGED: `run_timeline_with` keeps its signature and becomes a thin delegator (no-op observer). The `#[tracing::instrument(name = "timeline.execute", …)]` attribute MOVED from it onto `run_timeline_observed`, so exactly one `timeline.execute` span is emitted on either path and a fault span created inside the loop takes it as parent.
  - NEW private in `conductor-run`: `FaultKind` (enum: `Silence` · `Ramp { factor }`), `classify_fault(&EmissionSpec) -> Option<FaultKind>`, `ramp_factor(u32, u32) -> f64`, `fault_span(&Scenario, &PhaseWindow, i64) -> Option<tracing::Span>`.
  - CHANGED private in `conductor-faults`: `PortOccupier` gains `span: Option<tracing::Span>` + `held_since: Instant`. **No public API change** — `occupy` / `occupy_default` / `local_addr` / `release` / `Drop` keep their signatures and behavior.
  - NEW span names emitted: `fault.silence` · `fault.ramp` · `fault.port_occupier`. All three were ALREADY reserved in obs-plan §11's bounded span-name set — **no new name is introduced**.
  - NEW allowlisted field names (5): `fault_type` · `fault_duration_ms` · `fault_start_offset_ms` · `ramp_factor` · `port`.
  - **No new** env var, port, socket, IPC method, endpoint or on-disk artifact path.

- **Crates / modules:** none added or removed. `conductor-faults` gains its first `tracing` dependency edge; `conductor-timeline` and `conductor-run` unchanged in membership.

- **Dependencies:** `tracing` (workspace pin 0.1.44) added as a NORMAL dep of `conductor-faults`; `serde_json` (workspace pin) added as its first `[dev-dependencies]` entry. **`Cargo.lock` moved exactly 2 lines — both dependency EDGES inside `conductor-faults`' existing `[[package]]` entry, ZERO new `[[package]]`** ⇒ the cargo-audit surface is unchanged this chunk. No bump, no removal.

- **Schema / config:** the self-obs JSONL line schema (owned by test-plan §3, mirrored in obs-plan §3) gains three span names in the span-lifecycle variant plus five allowlisted attributes on the `new` line. No migration, no config key, no violation-schema change; the Run-report envelope is untouched (eleven fields, unchanged).

- **Spec-master edits:** none in this chunk's implementation (specs are read-only to /implement). Four are expected at this wrap — see *Spec claims disproved by measurement*.

- **Counts / qualifiers moved:**
  - `conductor_core::redact::ALLOWLISTED_FIELDS` 31 → **36** entries. Verified at authoring: no spec master bakes this literal (obs-plan §4 `:347` names the constant, not its size) — so no count-drift is expected, but the detectors should confirm.
  - Workspace test count 588 → **597** (+9). No doc bakes a test count (grepped `.andromeda/*.md` + `.claude/docs/*.md`).

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** a `tracing-subscriber` **dev-dependency on `conductor-faults` was added and then reverted** before any gate ran. It was unnecessary: `conductor-faults` already depends on `conductor-core`, so the occupier's span is asserted through `conductor_core::init_observability(ObsSink::File(..))` and reading the JSONL back — the convention `conductor-timeline/tests/obs_span.rs` already established. `serde_json` (to parse those lines) is what landed instead.

- **Spec claims disproved by measurement:** four, all obs-plan, none yet dispositioned —
  1. **§4 placement.** "Each fault span wraps the fault application phase in `conductor-faults`" is unreachable: the code-graph returns **0 crate edges** for `conductor-faults` in either direction and **0 references** to `PortOccupier`/`AbruptSilence`/`EmissionGap`/`BurstyTrain` from outside that crate. Silence and ramp are declarative phase data (`phase_spec.rs:60-62` states this intent outright); the spans open from the run path's per-phase hook. Evidence: `research.md` §Graph impact · `.andromeda/runs/2026-08-16T11-10-20-phase/tree-query-2026-08-16-fault-application-spans.json`.
  2. **§4 `ramp_factor` range.** Specified as a float `0.0-1.0`; the shipped `EmissionShape::Ramp` carries `from_rate`/`to_rate`/`windows` integers and no existing value has that range. Shipped as the normalized signed slope `(to−from)/max(from,to)`, range **−1.0..1.0** (operator decision at phase P4), so a rise is distinguishable from a fall.
  3. **§4 port-occupier attributes + parentage.** `fault_duration_ms` / `fault_start_offset_ms` are not knowable at bind time and there is no journal basis at that site; the span carries `fault_type` + `port` at open and witnesses the realized hold on the allowlisted `message` field at `debug` on release. Its `timeline.execute` parentage is **conditional** — no run path applies the occupier, and matrix `v2-15` assigns that driver to the connection-lifecycle live-proof entry.
  4. **§6 log-level table.** It assigns `debug` to internal control flow including "fault application"; the shipped fault spans emit at **INFO**. Grounds: every sibling span (`scenario.run`, `timeline.execute`, `emit.batch`) is INFO via `#[tracing::instrument]`; at DEBUG the spans are invisible under the subscriber's default INFO filter, so this chunk's own acceptance (the spans appearing on emitted lines) could not hold without a `RUST_LOG` opt-in; and one span per phase is not the hot path §11's `no-info-in-hot-path` ban targets.

- **Coverage of new surfaces:**
  - `fault.silence` (self-obs span, run path) → validation n/a · instrumentation span✓ · PII n/a (bounded label + two integers) · tests unit✓ (`conductor-core obs::tests::the_fault_spans_carry_their_specified_attributes_past_the_allowlist`, `conductor-run tests::a_zero_occurrence_phase_classifies_as_silence_whatever_its_shape`) · a11y n/a · tokens n/a
  - `fault.ramp` (self-obs span, run path) → validation n/a · instrumentation span✓ · PII n/a · tests unit✓ (same line-shape test + `tests::the_ramp_factor_carries_direction_and_steepness`, `tests::only_the_two_run_path_faults_classify`) · a11y n/a · tokens n/a
  - `fault.port_occupier` (self-obs span, RAII site) → validation n/a · instrumentation span✓ · PII n/a (`port` is an integer) · tests integration✓ (`conductor-faults::port_occupier::the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`, asserted on the rendered JSONL) · a11y n/a · tokens n/a
  - `run_timeline_observed` (library API, timeline seam) → validation n/a · instrumentation span✓ (carries `timeline.execute`) · PII n/a · tests unit✓ (3 in `conductor-timeline::pacing`: observer fires once per phase incl. a silent one · value dropped at its own boundary · transition stream unchanged) · a11y n/a · tokens n/a

## Deviations from intent

1. **Edited `crates/conductor-timeline/src/lib.rs`, which the plan's Files-to-modify did not list.** Justification: a new public item is unusable unexported; judged the discipline's in-scope gray area (a helper the chunk's own module clearly spawns) rather than a soft-exit. One line changed, no behavior.
2. **Drove the smoke as `SCENARIO=service-went-silent agent-run.sh run` rather than the plan's bare `agent-run.sh status`.** Justification: `status` with no argument falls back to `latest_run_id()` and exits 0 on the newest journal on disk — before the smoke that was the PREVIOUS chunk's `2026-08-16T09-54-12-950` run, so the plan-listed form would have proven nothing about this chunk. The firing form minted a fresh run first, then `status` read it.
3. **Fault spans emit at INFO where obs-plan §6's table implies DEBUG** — surfaced by /implement, not authored. Grounds recorded above under *Spec claims disproved* #4; this wrap dispositions it.
4. **`serde_json` added as a `conductor-faults` dev-dependency, beyond the plan's `tracing` line.** Justification: it is what makes the occupier's span assertable on the RENDERED line rather than on internal struct fields (which `rules/testing.md` bans asserting). Lockfile impact measured: +2 edge lines, zero new packages.

## Decisions & corrections

- **Operator decision (phase P4):** `ramp_factor` = normalized signed slope, range −1.0..1.0, over the endpoint-ratio and carry-the-endpoints alternatives — direction belongs in the value.
- **Operator decision (phase P4):** instrument `fault.port_occupier` at its RAII site now and record the parentage limit, rather than deferring the span to the chunk that wires its driver.
- **Leaned decision (phase P4, artifact-decided, not put to the operator):** the span opens from a caller-supplied per-phase hook, not from a fault-kind field on `Phase` — `phase.rs:5-7` states the phase "never carries what those emissions contain", and the journal basis the offset needs lives at the call site (`lib.rs:338`), not in the scheduler. Per-emission spans from `Dispatcher::dispatch` were rejected on the facts: that path never fires for a silence phase.
- **Implementation decision:** the fault span is **created but never entered**. Entering it would re-parent every `emit.batch` raised during the phase onto the fault span; obs-plan §4 Critical Path 1 nests those beneath `timeline.execute`. `JsonObsLayer` emits `new` at creation and `close` at drop regardless of entering, so the bracket is intact.
- **Correction found while testing:** `EmissionSpec` is `#[non_exhaustive]`, so a cross-crate struct literal (even with `..Default::default()`) does not compile — the test helper uses `EmissionSpec::shaped(..)`.
- **Discipline applied unprompted (candidate for curation):** a bare `agent-run.sh status` exits 0 on the newest journal on disk and proves nothing about the current run; mint a fresh journal with `SCENARIO=… run` first, then read it.

## Outcome

**Acceptance criteria: met**, with one criterion class deliberately asserted at the test tier rather than on the run artifact (planned, recorded in the plan's Implementation notes).

Gates run — all green in 2 fix-loop iterations (one borrow-lifetime error in a newly written test):
- `cargo nextest run --workspace --profile ci` → **597/597, 0 skipped, zero retries** (588 before, +9)
- `cargo nextest run -p conductor-core -p conductor-timeline -p conductor-run -p conductor-faults --profile ci` → 345/345
- `cargo test --workspace --doc` → 0 failures
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `cargo deny check advisories bans licenses sources` → **true exit 0**, "advisories ok, bans ok, licenses ok, sources ok" (VERIFIED, not assumed — the sole supply-chain coverage while audit is red)
- `cargo audit` → **true exit 1**, byte-identical `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — the **23rd** consecutive advisory-DATABASE fault. Re-pinned per the operator-ratified bounded wait: no floor raise, no `deny.toml` ignore, no CI edit. Basis re-verified this chunk (lock +2 edge lines, zero new `[[package]]` ⇒ audit surface unchanged; deny green observed).
- Determinism: the committed `replay__*` / `pacing__*` / `dispatch_wire__*` insta goldens are byte-unchanged and no `.snap.new` was produced.

**Smoke ✓** (boot-path touched — `execute_scenario` is on the binary path): `SCENARIO=service-went-silent bash scripts/agent-run.sh run` minted a fresh run `2026-08-16T11-47-01-568`; `bash scripts/agent-run.sh status` returned exit 0 with `state: "Blocked"`, `verdict: null`, `slo_tier: "<90s"`; `logs/agent-latest.jsonl` was rewritten this run (4082 bytes, 13:47:01).

**One honest limit, predicted and confirmed.** That artifact contains **zero `fault.*` spans — and no `timeline.execute` either**. `execute_scenario` returns a Blocked record at `lib.rs:325` before the timeline runs whenever the preflight is not ready, so a Pulse-free run reaches no phase at all. The absent fault spans are that structural consequence, not an instrumentation defect — the missing `timeline.execute` (a span shipped two chunks ago and proven by its own test) is the independent corroboration. The artifact-level sighting belongs to the live-proof entries later in Epoch 3.

**Verification matrix:** 0 capabilities claimed. `v2-15` (connection-lifecycle family live-proven) was considered and declined at phase P5 — it needs a live-Pulse family leg driving the occupier to a `ReceiverFailed` read-back — and carries a `notes` line recording that no port-occupier driver exists anywhere, so its claiming chunk must wire one rather than merely run the scenario.
