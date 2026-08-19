# Codebase Research — 2026-08-19-connection-lifecycle-live-proof

## Scope
- **Depth:** deep · **Reads:** 16 files (targeted, both repos) · **Globs/Greps:** ~14 · **Graph queries:** 3 (trace: `.andromeda/runs/2026-08-19T21-35-28-phase/tree-query-2026-08-19-connection-lifecycle-live-proof.json`)

## Files inspected
**Conductor:**
- `scenarios/receiver-lifecycle-state.toml` (full) — P-001 walk: 4 phases, ALREADY carries `[phases.emission]` tables (occurrences 0/4/0/0), seed 4317001, tier `<90s`, ONE Hard `Contains "Stalled"`. Header states the split: "the timing walk is auto; the connection-dot hue is operator-checklist".
- `scenarios/receiver-failed-port-conflict.toml` (full) — P-003: ONE phase `port-held` (gap 2000ms, NO emission table → defaults to 1 plain trace), tier `<5s`, ONE Hard `Contains "ReceiverFailed"`. Header: the bind "is driven at run time by the existing conductor-faults port-occupier (Epoch-8 driver) — this config does NOT bind the port itself".
- `scenarios/last-span-ago-tracking.toml` (full) — P-002: 3 interval phases, no emission tables (each emits the default 1 plain span — which happens to be the right stimulus), tier `<5s`, ONE Hard `Contains "Receiving"`.
- `scenarios/orthogonal-health-domains.toml` (full) — P-004: `error-burst` + `clean-stop`, NO emission tables — the "high error rate" lives only in comments; the phase actually emits 1 plain span. Tier `<90s`, Hard `Contains "Idle"` + CalibrationRegion `Contains "error"`.
- `crates/conductor-faults/src/port_occupier.rs` (full) — RAII `occupy`/`release`/`Drop`; loopback hard-coded (only the port is chosen); the `fault.port_occupier` span opens created-not-entered AT `occupy` with exactly `fault_type` + `port`; `release` logs the realized hold on `message` at `debug`; a refused bind is typed `FaultError::Bind`. Already obs-plan-§4-conformant.
- `crates/conductor-faults/tests/port_occupier.rs` (full) — 7 tests incl. the span-line assertion; tests bind `:0` ephemeral, NEVER the real 4317.
- `crates/conductor-run/src/lib.rs` (:320–570 + structure scan) — `execute_scenario` order: ready-gate (blocked short-circuits BEFORE phases) → `probe_egress(:4317)` (refused ⇒ harness Err) → `Dispatcher::connect` → `run_timeline_observed(…, |window| fault_span(…), dispatch)` → `observe(client)` (EmptyCorpus/CallFailed ⇒ Blocked) → empty-`expected` ⇒ `manual_record` (ManualCheck; KnownResidual if degraded) / else graded. `FaultKind::{Silence, Ramp}` (:484); `classify_fault` (:505) deliberately excludes the occupier ("instrumented where it binds"); `fault_span` (:532) is the ONLY caller-side hook site.
- `crates/conductor-run/src/dispatch.rs` (full) — `EmissionShape` arms {Plain, Error, Exception, Severity, Latency, Pii, Ramp, Breathing, Topology}; a 0-occurrence phase never reaches `dispatch`; identity = `DEFAULT_SERVICE_NAME` (`conductor`).
- `crates/conductor-timeline/src/scheduler.rs` (:30–145) — **`run_timeline_observed` is generic over the phase-guard type `G`** (`O: FnMut(PhaseWindow<'t>) -> G`): "the scheduler keeps it alive until the phase's boundary and drops it there. That value is opaque here." The occupier's RAII window slot ALREADY EXISTS; no timeline change.
- `crates/conductor-core/src/phase_spec.rs` (:1–100) — `PhaseSpec { name, gap_ms, emission #[serde(default)] }`; `EmissionSpec::default()` = `occurrences: 1`, Plain, Traces (confirms the CARRY's "one plain span"); `MAX_GAP_MS = 3_600_000`; `EmissionSpec` is `#[non_exhaustive]`, `PhaseSpec` is NOT.
- `crates/conductor-core/src/coverage.rs` (:89–92) — **P-001..P-004 are ALL `CoverageMode::DriveObserve`** ("Connection & Health Awareness"). Not `Auto` ⇒ `UNBACKED_AUTO` is untouched by anything this chunk does.
- `crates/conductor-verify/src/extract.rs` (:1–100) — `Observation.text` = `query_incident_list` text + per-incident `retrieve_report` text; `observed_for(Contains)` grades against THAT text; EmptyCorpus/CallFailed ⇒ Blocked upstream.
- `crates/conductor-run/tests/pii_harvest.rs` (head) — the harvest mold: serde_json line parsers + verbatim doc-comment pins + hard predicates; TEST-ONLY, nothing wired into the run path; capture basis `{data_dir}/logs/agent-latest.jsonl.<date>` sliced by pre-leg line count.
- `Cargo.toml` (root) — `conductor-faults = { path = "crates/conductor-faults" }` already in `[workspace.dependencies]` (:26); the new edge is one `conductor-faults.workspace = true` line in conductor-run.
- `scripts/agent-run.sh` (cleanup verb) — no port probe in the script; comment: "the :4317 port-occupier is RAII-released when the conductor process exits". The bind-release probe is the test's job (test-plan §3), not the script's.

**SUT (andromeda-pulse, HEAD on disk):**
- `crates/ingest/src/state.rs` (full) — `IngestState.last_ingest_at_nanos` is **app-wide** (one atomic, not per-service); 0 = never ingested (initial Listening).
- `crates/ingest/src/connection.rs` (:36–345) — `IDLE_THRESHOLD_NANOS` = 10s, `STALLED_THRESHOLD_NANOS` = 60s, `POLLER_INTERVAL` = 1s. `compute_state`: bind-failed/panic ⇒ ReceiverFailed (overrides all); last==0 ⇒ Listening; age<10s ⇒ Receiving; 10–60s ⇒ Idle; ≥60s ⇒ Stalled. `start_poller` logs **on state change only**: target `connection.state.transition`, fields `from_state`/`to_state`/`last_span_ago_ms`/`trigger_reason`/`severity`, message "connection state transition" — `error` level for →ReceiverFailed, `info` otherwise. `state_label` is PascalCase ("Listening"/"Receiving"/"Idle"/"Stalled"/"ReceiverFailed"); reason precedence panic > bind > stale-heartbeat.
- `pulse-app/src/main.rs` (:1195–1285) — the gRPC receiver task: `try_bind` Ok ⇒ `BindStatus::Ok` + info `app.boot.otlp.grpc.bind` "OTLP gRPC receiver bound" (`bind_address`); Err ⇒ `BindStatus::Failed(reason)` + error "bind failed" (`reason`, `bind_address`). **No retry** — the task ends; pulse-app keeps running, so ReceiverFailed persists until a process restart rebinds.
- `pulse-app/src/connection_router.rs` (full) — the FSM's ONLY query surfaces are the TauRPC `connection.current_state` resolver (desktop UI) and the `pulse://stream/connection-state` broadcast; `HeartbeatBindStatus` adapts `HeartbeatState.otlp_grpc_bind()/otlp_http_bind()`.
- `pulse-app/src/heartbeat.rs` (:279–300) — `connection.tick` info line every heartbeat tick (15s cadence) with `state`/`last_span_ago_ms`/`severity`, message "heartbeat" — the periodic P-002 witness.
- `pulse-app/tests/e2e_p003_panic_hook_propagation.rs` (head) — Pulse's own P-003 e2e (panic→ReceiverFailed); confirms the log file path `<data-dir>/logs/agent-latest.jsonl`.
- `crates/mcp-server/src/tools.rs` (grep) — **NO MCP tool carries connection/receiver state** (every "Connection" hit is `duckdb::Connection`). The FSM state reaches no read-back surface Conductor consumes.

## Graph impact
- **crate_edges** — `conductor-run` → {core, emit, report, timeline, verify}; `conductor-faults` → core only; **no run→faults edge exists; conductor-faults has 0 inbound edges** (the island, re-confirmed at this HEAD).
- **PortOccupier** — every reference (37 rows) is inside `conductor-faults` (src + its own test). Wiring is purely additive outside the crate.
- **PhaseWindow / run_timeline_observed** — `conductor-timeline/src/scheduler.rs:44/:113`; **FaultKind** — `conductor-run/src/lib.rs:484–493`. `fault_span` has ONE call site (the `execute_scenario` closure), so re-threading its return type is a single-site change.
- **PhaseSpec literal constructors** (field-addition blast radius, from grep — struct literals are not calls): **13 sites across 6 files** — `conductor-core/src/{error,load_envelope,phase_spec,scenario}.rs`, `conductor-run/tests/dispatch_wire.rs`, `conductor-timeline/src/convert.rs`. Each gains `fault: None` (mechanical).

## Patterns detected
- **Guard-generic phase hook** (`scheduler.rs:113`): whatever `on_phase` returns is held to the phase boundary and dropped there — the exact RAII window the occupier needs; the driver is a caller-side guard struct, not a scheduler change.
- **Created-not-entered fault spans** (`lib.rs:532`, `port_occupier.rs::occupy`): keeps `emit.batch` parented under `timeline.execute`; the occupier's span already opens at the bind site per obs-plan §4.
- **Declare-only + harvest-tier proof** (`pii_harvest.rs` ← `restart_harvest` ← `baseline_harvest` ← `storm_harvest`): parsers over Pulse's own JSONL + verbatim pins + hard predicates; TEST-ONLY; leg window sliced by pre-leg line count.
- **Catalog guard tests per family** (`scenario.rs` :424/:511/:571/:793/:872/:885): each declare-only family has a membership guard; retiring this family's checks adds/extends one and re-bases any expected-count guard naming these four TOMLs.

## Conventions to follow
- Verdict/error wall: `FaultError::Bind` is already typed (`port_occupier.rs:47`); no panic path exists.
- Tests bind `:0` ephemeral, never the real 4317 (`tests/port_occupier.rs:2-3`).
- Emission declared explicitly per phase (`receiver-lifecycle-state.toml` is the in-family precedent; the other three predate the dispatcher and lean on the default).
- Live-leg mechanics: fresh data dirs under `%TEMP%/pulse-legs/`, capture Pulse's `agent-latest.jsonl.<date>`, pre/post line counts (the pii-scrub leg-verdict shape).

## New files to create
- `crates/conductor-run/tests/connection_harvest.rs` — the family's harvest test (pii_harvest mold): parsers for `connection.state.transition` / `app.boot.otlp.grpc.bind` / `connection.tick`; per-leg transition-ledger predicates; verbatim pins land at implement from the live legs.

## Files to modify
- `crates/conductor-core/src/phase_spec.rs` — `PhaseSpec` gains `#[serde(default)] #[garde(dive)] pub fault: Option<FaultSpec>` + a `FaultSpec { kind }` model (closed kind enum, `port_occupier` first member) + unit tests.
- The 13 `PhaseSpec` literal sites (6 files above) — `fault: None` threading.
- `crates/conductor-run/Cargo.toml` — `conductor-faults.workspace = true` (Cargo.lock gains the member-dep row; ZERO new external crates — the audit-deferral basis restated accordingly, per the security extract).
- `crates/conductor-run/src/lib.rs` — a `PhaseGuard { span: Option<Span>, occupier: Option<PortOccupier> }` returned from the `on_phase` closure (single site); occupy at phase open for a fault-declaring phase; failed occupy = typed warn + continue (plan decision #1).
- `scenarios/receiver-failed-port-conflict.toml` — re-base to the 3-phase choreography (pre-conflict window / port-held with `fault` + occurrences 0 / recovery window, total ≤ ~85s), check retired declare-only, tier `<5s` → `<90s`.
- `scenarios/receiver-lifecycle-state.toml` — check retired declare-only (phases already correct), tier `<90s` kept.
- `scenarios/last-span-ago-tracking.toml` — explicit `[phases.emission]` tables, check retired, tier re-calibrated (predicted `<20s`; the leg measures).
- `scenarios/orthogonal-health-domains.toml` — `error-burst` gains a real `kind = "error"` emission table; both checks retired; tier re-measured (predicted `<90s` kept).
- `crates/conductor-core/src/scenario.rs` — catalog-guard re-bases (new family declare-only guard + any expected-count/suite guard naming these four).
- `conductor-0.2.0/verification-matrix.json` — the v2-15 claim (P5's link step).

## Key run-path facts the plan must respect
- **The choreography constraint:** ready:true requires Pulse UP (canary round-trip) and Pulse holds `:4317`; the occupier can bind only while Pulse's receiver is down; a not-ready gate short-circuits BEFORE phases; Pulse never retries a failed bind (recovery = pulse-app restart). ⇒ The only single-run shape that lets Conductor's occupier drive a live ReceiverFailed: preflight green with Pulse up → a pre-conflict phase window in which the operator STOPS pulse-app → the port-held phase binds the occupier at phase open (one attempt) → the operator STARTS pulse-app into the conflict (bind fails → error transition line) → the phase boundary drops the guard (RAII release) → a recovery window in which the operator restarts pulse-app ("OTLP gRPC receiver bound"). All phases 0-occurrence, so nothing is emitted into the occupier or the down windows; `observe()` still works (the sidecar reads corpus.db regardless of pulse-app's state).
- **The FSM is app-wide and the preflight canary advances it** ⇒ a preflighted walk leg starts from Stalled (the canary poll outlasts 60s), so the witnessable live ledger for the walk is boot-time `Listening→Receiving` (during preflight) then `Stalled→Receiving→Idle→Stalled` across the phases — the harvest predicates assert THAT ledger, not the naive one.
- **Latency ≈ scenario duration** (the v2-11/v2-12 precedent, `scenario.rs:581`): tiers re-calibrate from measured leg values inside the closed set.

## Open questions
1. Failed occupy on a fault-declared phase: typed warn + continue (leg artifacts survive; the missing `fault.port_occupier` span and absent harvest witnesses make non-application visible) vs harness Err (kills the run on an operator-timing slip)? → blocks: **plan-decision** (P4 resolves; lean = warn + continue).
2. Exact port-conflict choreography window lengths (operator reaction budget vs the `<90s` tier ceiling; total phases ≤ ~85s) → blocks: **implementation-scope** (fixed at implement within the stated constraint).
